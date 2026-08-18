#![cfg(feature = "toml")]

use std::string::String;
use quote::{format_ident, quote};
use toml;

use crate::{Endpoint};

mod in_;
use in_::{
    RootConfig,
    SecondaryChannels,
    NodeSection,
    EndpointDefaults,
    DeviceType,
    ServerCluster,
};

mod error;
use error::ConfigError::{
    self,
    FeatureConflict,
};

// tbd. This is SO LONG!!! Find ways to split it into a few.
/**
* Convert TOML input string to Rust snippet that generates an 'esp_zb::Config' instance, when read in.
*/
pub fn convert_toml(toml: &str) -> Result<String,ConfigError> {

    let c: RootConfig = toml::from_str(toml)?;  // may return a 'ParseError'

    //--- network
    //
    // ChannelMask::new(1 << 13)
    let q_primary_channels = {
        if c.network.primary_channels.is_empty() {
            Err("'network.primary_channels' is empty: please provide at least one channel to scan.")?;
        }

        let q = {
            let qs = c.network.primary_channels.into_iter().map(|v| {
                quote! { 1 << #v }
            });
            quote! { #(#qs)|* }    // 1 << 11u8 | 1 << 12u8 | ...
        };
        quote!{ ChannelMask::new( #q ) }
    };

    // ChannelMask::ALL
    let q_secondary_channels = {
        match c.network.secondary_channels {
            SecondaryChannels::Preferred => quote!{ ChannelMask::PREFERRED },
            SecondaryChannels::All =>       quote!{ ChannelMask::ALL },
        }
    };

    //--- platform
    // "{string}"
    let q_storage_partition_name = {
        let s = c.platform.storage_partition_name;
        quote! { #s }
    };

    //--- node

    // Note: We don't create '#[cfg]' barriers in the output; we assume that the application
    //      enables enough features.
    //
    // NodeType::CoordinatorConfig{ install_code_policy: false, max_children: 10 }
    //
    let q_node = {
        match c.node {
            NodeSection::Coordinator { install_code_policy, max_children} => {
                if cfg!(not(feature = "coordinator")) {
                    return Err(FeatureConflict("coordinator"));
                } else {
                    quote! {
                        NodeType::CoordinatorConfig {
                            install_code_policy: #install_code_policy,
                            max_children: #max_children,
                        }
                    }
                }
            },
            NodeSection::Router { install_code_policy, max_children} => {
                if cfg!(not(feature = "router")) {
                    return Err(FeatureConflict("router"));
                } else {
                    quote! {
                        NodeType::RouterConfig {
                            install_code_policy: #install_code_policy,
                            max_children: #max_children,
                        }
                    }
                }
            },
            #[cfg(false)]
            NodeSection::EndDevice { install_code_policy } => {
                #[cfg(not(feature = "end_device_UNTESTED"))]
                panic!("'node.type' \"end_device\" but that feature is not enabled.");

                unimplemented!();
                /*** just giving a taste; implement only if we *actually* do devices that are only EndDevice role.
                quote! {
                    NodeType::EndDeviceConfig {
                        install_code_policy,
                        ed_timeout: ezb_nwk_ed_timeout_e,
                        keep_alive: Duration,
                    }
                }***/
            }
        }
    };

    //--- endpoint.{id}
    //--- endpoint.defaults
    // {
    //    let ep_10 = Endpoint{ common_fields = CommonFields{ ... }, device_type = DeviceType::HAColorDimmableLight), additional_server_clusters = Vector::new() };
    //    BTreeMap::from([(10, ep_10)])
    // }
    let q_endpoints = {
        let mut q_lets = quote!{};
        let mut q_arr_contents = quote!{};

        if c.endpoint.instances.is_empty() {
            return Err(
                ConfigError::from("Need at least one end point. Please define '[endpoint.{id}]'.")
            );
        }

        let EndpointDefaults {
            manufacturer_name: manufacturer_name_def,
            model_identifier: model_identifier_def
        } = &c.endpoint.defaults;

        // '.as_ref()'s turn to 'Option<&String>' (important later)
        let manufacturer_name_def = manufacturer_name_def.as_ref();
        let model_identifier_def = model_identifier_def.as_ref();

        c.endpoint.instances.into_iter().try_for_each(|(k,v)| -> Result<(),ConfigError> {
            // Skip ".defaults", turn others to 'u8'
            let k = match k.as_str() {
                "defaults" => { return Ok(()) }, // skip
                id => {
                    let endpoint_id = id.parse::<u8>().map_err(|_| {
                        format!("Invalid endpoint ID (not 'u8'): {}", id)
                    })?;
                    if !Endpoint::is_valid_id(endpoint_id) {
                        // Rust note: no 'Display' on 'RangeInclusive'
                        let (a,b) = (Endpoint::VALID_RANGE.start(), Endpoint::VALID_RANGE.end());
                        return Err(
                            ConfigError::from(format!("Invalid endpoint ID (not in range '{a}..={b}'): {id}"))
                        );
                    }
                    endpoint_id
                }
            };  // k ∈ 1..=240

            // ep_10
            let ident = format_ident!("ep_{}", k);

            let q_common_fields = {
                // Note: Endpoints could override the common fields, but we haven't implemented (/needed) that.
                //
                let q_manufacturer_name = manufacturer_name_def
                    .map(|s| quote!{ Some(#s) })
                    .unwrap_or_else(|| quote!{ None });

                let q_model_identifier = model_identifier_def
                    .map(|s| quote!{ Some(#s) })
                    .unwrap_or_else(|| quote!{ None });

                quote!{
                    CommonFields{
                        manufacturer_name: #q_manufacturer_name,
                        model_identifier: #q_model_identifier,
                    }
                }
            };

            let q_device_type = match v.device_type {
                #[cfg(feature = "dt_color_dimmable_light")]
                DeviceType::HA_ColorDimmableLight {} => quote! {
                    Endpoint::HA_ColorDimmableLight
                },
                #[cfg(feature = "dt_color_dimmer_switch")]
                DeviceType::HA_ColorDimmerSwitch {} => quote! {
                    Endpoint::HA_ColorDimmerSwitch
                },
                #[cfg(feature = "dt_ias_cie")]
                DeviceType::HA_IasCie {} => quote! {
                    Endpoint::HA_IasCie
                },
                // exhaustive match
            };

            let q_additional_server_clusters = {
                let qs = v.additional_server_clusters.iter().map(|cluster| {
                    match cluster {
                        ServerCluster::HA_OnOff =>      quote! { ZclCluster::OnOff },
                        ServerCluster::HA_LevelControl => quote! { ZclCluster::LevelControl },
                        ServerCluster::HA_PowerConfig => quote! { ZclCluster::PowerConfig },
                        ServerCluster::HA_IasZone =>    quote! { ZclCluster::IasZone },
                    }
                });
                quote! { vec![ #(#qs),* ] }
            };

            //let q_discover = quote!{};  // tbd.

            q_lets.extend(quote!{
                let #ident = Endpoint{
                    common_fields = #q_common_fields,
                    device_type = #q_device_type,
                    additional_server_clusters: #q_additional_server_clusters,
                    // tbd. discover_remote_server_clusters
                }
            });
            q_arr_contents.extend(quote!{ (#k, #ident), });

            Ok(())  // next
        })?;

        quote!{ {
            #q_lets
            BTreeMap::from([#q_arr_contents])
        } }
    };

    // All together now!
    //
    let q_all = quote!{ {
        use alloc::collections::BTreeMap;
        use ezb_node::config::*; // Config, ChannelMask, NodeType, ...

        let primary_channels = #q_primary_channels;
        let secondary_channels = #q_secondary_channels;
        let storage_partition_name = #q_storage_partition_name;
        let node = #q_node;
        let endpoints = #q_endpoints;

        Config {
            primary_channels,
            secondary_channels,
            storage_partition_name,
            node,
            endpoints
        }
    } };

    pretty(q_all)
}

/**
* Check a Rust expression for syntactical correctness, and format it similar to 'rustfmt'.
*/
fn pretty(q: proc_macro2::TokenStream) -> Result<String,ConfigError> {

    // Note: 'prettyplease' is designed to handle full files. To handle an expression, we do some wrapping and unwrapping,
    //
    let q_file = quote! {
        fn dummy_wrapper() { #q }
    };

    let syntax_tree: syn::File = syn::parse2(q_file)?;
    let formatted = prettyplease::unparse(&syntax_tree);

    // Just the innards of the function (skip first and last line)
    let ls: Vec<&str> = formatted.lines().collect();
    assert!(ls.len() > 2);

    let inner_lines = & ls[1..ls.len() -1];

    let ret: String = inner_lines.into_iter()
        .map(|line| { line.strip_prefix("    ").unwrap_or(line).to_string() })
        .collect::<Vec<String>>()
        .join("\n");

    Ok(ret)
}
