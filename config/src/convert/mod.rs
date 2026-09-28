#![cfg(feature = "toml")]

use std::string::String;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
//r use syn::parse::Parser;
use toml;

mod in_;
use in_::{
    RootConfig,
    SecondaryChannels,
    NodeSection,
    EndpointDefaults,
    DeviceType,
    Cluster,
};

mod error;
use error::ConfigError::{
    self,
    FeatureConflict,
};

#[cfg(feature = "coordinator")]
const ROLE: &'static str = "coordinator";
#[cfg(feature = "router")]
const ROLE: &'static str = "router";
//#[cfg(feature = "end_device")]
//const ROLE: &'static str = "end device";

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
    // "None" | "Option({string})"
    let q_storage_partition_name = {
        let opt = c.platform.storage_partition_name;
        quote! { #opt }
    };

    //--- node

    // Feature notes:
    //      - Both the 'build.rs' and the application _will_ use features for node type.
    //      - by parsing all possible node types, we allow the user to get better error messages
    //        (wrong-featured but otherwise valid TOML will always parse).
    //      - We don't create '#[cfg]' barriers in the generated output (avoiding complexity), just
    //          expect the user to bring in enough features.
    //
    let q_node = {
        match c.node {
            NodeSection::Coordinator { allow_open_pairing, max_children, pre_configured_devices} => {
                #[cfg(not(feature = "coordinator"))]
                return Err(FeatureConflict("coordinator"));

                #[cfg(feature = "coordinator")]
                {
                    let install_code_policy = !allow_open_pairing;

                    // 'PreConfiguredDevice' still needs some parsing.
                    //
                    let q_pre_configured_devices: Vec<TokenStream> = pre_configured_devices .into_iter()
                        .map(|o| {
                            let (ieee_address_bytes, install_code_bytes): ([u8;8], [u8;18]) = o.parse()?;

                            let ieee_address = proc_macro2::Literal::byte_string(&ieee_address_bytes);
                            let install_code = proc_macro2::Literal::byte_string(&install_code_bytes);

                            let ieee_address_comment = ieee_address_bytes .into_iter()
                                .map(|b| format!("{:02X}", b))
                                .collect ::<Vec<String>>()  // note: force iteration to finish; '.join()' doesn't work on iterators.
                                .join(":");

                            let install_code_comment = install_code_bytes
                                .chunks(2)
                                .map(|chunk| {
                                    assert!(chunk.len() == 2);
                                    format!("{:02X}{:02X}", chunk[0], chunk[1])
                                })
                                .collect ::<Vec<String>>()
                                .join(" ");

                            Ok(quote! {
                                PreConfiguredDevice {{
                                    #[doc = #ieee_address_comment]
                                    ieee_address: *#ieee_address,
                                    #[doc = #install_code_comment]
                                    install_code: *#install_code,
                                }}
                            })
                        }).collect ::<Result<_,ConfigError>>()?;

                    quote! {
                        NodeType::CoordinatorConfig {
                            install_code_policy: #install_code_policy,
                            max_children: #max_children,
                            pre_configured_devices: #( #q_pre_configured_devices ),*,
                        }
                    }
                }
            },
            #[cfg_attr(not(feature = "router"), allow(unused_variables))]
            NodeSection::Router { allow_open_pairing, max_children} => {
                #[cfg(not(feature = "router"))]
                return Err(FeatureConflict("router"));

                #[cfg(feature = "router")]
                {
                    let install_code_policy = !allow_open_pairing;
                    quote! {
                        NodeType::RouterConfig {
                            install_code_policy: #install_code_policy,
                            max_children: #max_children,
                        }
                    }
                }
            },
            NodeSection::EndDevice { allow_open_pairing: _, .. } => {
                unimplemented!();
                /*** just giving a taste; implement only if we *actually* do devices that are only EndDevice role.
                let install_code_policy = !allow_open_pairing;
                quote! {
                    NodeType::EndDeviceConfig {
                        install_code_policy: #install_code_policy,
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

        let (defaults, instances) = c.endpoint.parse()?;

        let EndpointDefaults {
            manufacturer_name: manufacturer_name_def,
            model_identifier: model_identifier_def
        } = defaults;

        // Coordinator and end device are meaningless without an end point.
        //
        #[cfg(not(feature = "router"))]
        if instances.is_empty() {
            return Err(ConfigError::from(
                format!("Need at least one end point for {ROLE}. Please define '[endpoint.{{id}}]'.")
            ));
        }

        // '.as_ref()'s turn to 'Option<&String>' (important later)
        let manufacturer_name_def = manufacturer_name_def.as_ref();
        let model_identifier_def = model_identifier_def.as_ref();

        instances.into_iter().for_each(|(k,v)| { // k ∈ 1..=240
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
                    DeviceType::ColorDimmableLight
                },
                #[cfg(feature = "dt_color_dimmer_switch")]
                DeviceType::HA_ColorDimmerSwitch {} => quote! {
                    DeviceType::ColorDimmerSwitch
                },
                #[cfg(feature = "dt_ias_cie")]
                DeviceType::HA_IasCie {} => quote! {
                    DeviceType::IasCie
                },
                // exhaustive match
            };

            let q_additional_client_clusters = {
                let qs = v.additional_client_clusters.iter().map(|cluster| {
                    match cluster {
                        Cluster::HA_OnOff =>        quote! { Cluster::OnOff },
                        Cluster::HA_LevelControl => quote! { Cluster::LevelControl },
                        Cluster::HA_PowerConfig =>  quote! { Cluster::PowerConfig },
                        Cluster::HA_IasZone =>      quote! { Cluster::IasZone },
                    }
                });
                quote! { vec![ #(#qs),* ] }
            };

            q_lets.extend(quote!{
                let #ident = Endpoint {
                    common_fields: #q_common_fields,
                    device_type: #q_device_type,
                    additional_client_clusters: #q_additional_client_clusters,
                    // tbd. discover_remote_server_clusters
                };
            });
            q_arr_contents.extend(quote!{ (#k, #ident), });
        });

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

    let s= pretty(q_all)?;
    Ok(s)
}

/**
* Check a Rust expression for syntactical correctness, and format it similar to 'rustfmt'.
*/
fn pretty(q: proc_macro2::TokenStream) -> Result<String,ConfigError> {

    // DEBUG: enable this if there are internal problems with code generation.
    //panic!("CHECK: {:?}", q);

    // Note: 'prettyplease' is designed to handle full files. To handle an expression, we do some wrapping and unwrapping,
    //
    let q_file = quote! {
        fn dummy_wrapper() { #q }
    };

    // If there's something wrong with the 'quote!' macros, 'syn::parse2' fails.
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
