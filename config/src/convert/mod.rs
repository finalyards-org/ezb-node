#![cfg(feature = "toml")]

use std::any::Any;
use std::str::FromStr;
use std::string::String;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::token::Token;
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
};
use crate::convert::in_::PreConfiguredDevice;
//R #[cfg(feature = "coordinator")]
//R const ROLE: &'static str = "coordinator";
//R #[cfg(feature = "router")]
//R const ROLE: &'static str = "router";
//R //#[cfg(feature = "end_device")]
//R //const ROLE: &'static str = "end device";

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
    // "None" | "Some("...")"
    let q_storage_partition_name = {
        let opt = c.platform.storage_partition_name;
        q_opt(opt)  // retains 'None'
    };

    //--- node

    // Take 'pre_configured_devices' first. It belongs to 'NodeSection::Coordinator' though is
    // presented stand-alone in the TOML.
    //
    let pre_configured_devices = c.pre_configured_devices;

    //r assert!(pre_configured_devices.len() > 0);  // DEBUG

    #[allow(non_snake_case)]
    let ROLE: &'static str;

    // Feature notes:
    //      - The 'build.rs' side (feature = "toml") DOES NOT use coordinator/router/end_device features.
    //      - by parsing all possible node types, we may allow the user to get better error messages
    //      - We don't create '#[cfg]' barriers in the generated output (avoiding complexity), just
    //          expect the user to bring in enough features.
    //
    let q_node = {
        match c.node {
            NodeSection::Coordinator { allow_open_pairing, max_children/*, pre_configured_devices*/} => {
                let install_code_policy = !allow_open_pairing;
                ROLE = "Coordinator";

                // 'PreConfiguredDevice' still needs some parsing.
                //
                let q_pre_configured_devices: Vec<TokenStream> = pre_configured_devices .into_iter()
                    .map(|o| {
                        let PreConfiguredDevice{ mac_address, install_code, _name: _ } = o;

                        let q_mac_address = proc_macro2::Literal::byte_string(&mac_address);
                        let q_install_code = proc_macro2::Literal::byte_string(&install_code);

                        let q_doc = {
                            fn f(bytes: &[u8]) -> String {
                                let s = bytes
                                    .chunks(2)
                                    .map(|chunk| {
                                        assert_eq!(chunk.len(), 2);
                                        format!("{:02X}{:02X}", chunk[0], chunk[1])
                                    })
                                    .collect ::<Vec<String>>()  // note: '.join()' doesn't work on iterators; must collect
                                    .join(" ");
                                s
                            }
                            format!("MAC address: {}", f(&mac_address))
                        };

                        Ok(quote! {
                            #[doc = #q_doc]
                            PreConfiguredDevice {
                                mac_address: *#q_mac_address,
                                install_code: *#q_install_code,
                            }
                        })
                    }).collect ::<Result<_,ConfigError>>()?;

                quote! {
                    NodeType::CoordinatorConfig {
                        install_code_policy: #install_code_policy,
                        max_children: #max_children,
                        pre_configured_devices: vec![ #( #q_pre_configured_devices ),* ],
                    }
                }
            },

            NodeSection::Router { allow_open_pairing, max_children} => {
                let install_code_policy = !allow_open_pairing;
                ROLE = "Router";

                if !pre_configured_devices.is_empty() {
                    Err(format!("'.pre_configured_devices' is Coordinator only; should not be provided for: {}", ROLE))?;
                }

                quote! {
                    NodeType::RouterConfig {
                        install_code_policy: #install_code_policy,
                        max_children: #max_children,
                    }
                }
            },

            NodeSection::EndDevice { allow_open_pairing: _, .. } => {
                unimplemented!();
                ROLE = "End device";

                /*** just giving a taste; implement only if we *actually* do devices that are only EndDevice role.
                let install_code_policy = !allow_open_pairing;

                if !pre_configured_devices.is_empty() {
                    Err("'.pre_configured_devices' is Coordinator only; should not be provided for: {}", ROLE)?;
                }

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
        if !c.node.is_router() && instances.is_empty() {
            Err(ConfigError::from(
                format!("Need at least one end point for {ROLE}. Please define '[endpoint.{{id}}]'.")
            ))?;
        }

        // '.as_ref()'s turn to 'Option<&String>' (important later)
        let manufacturer_name_def = manufacturer_name_def.as_ref();
        let model_identifier_def = model_identifier_def.as_ref();

        instances.into_iter().try_for_each(|(k,v)| { // k ∈ 1..=240
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

            fn q_cluster(cluster: Cluster) -> TokenStream {
                match cluster {
                    Cluster::HA_OnOff =>        quote! { Cluster::OnOff },
                    Cluster::HA_LevelControl => quote! { Cluster::LevelControl },
                    Cluster::HA_PowerConfig =>  quote! { Cluster::PowerConfig },
                    Cluster::HA_IasZone =>      quote! { Cluster::IasZone },
                }
            }

            fn q_clusters<I: IntoIterator<Item=Cluster>>(xs: I) -> TokenStream {
                let xs = xs.into_iter();

                let qs = xs.map(q_cluster);
                quote! { vec![ #(#qs),* ] }
            }

            let q_additional_client_clusters = {
                q_clusters( v.additional_client_clusters )
            };

            /// Inject the 'discover_server_clusters: ...' for router and end device, but keep
            /// it empty for a coordinator.
            ///
            let q_discover_server_clusters: TokenStream = {

                if c.node.is_coordinator() {
                    match v.discover_server_clusters {
                        None => quote!(None),   // not applicable on a coordinator
                        Some(x) =>
                            return Err(ConfigError::from(
                                format!("A coordinator does not use '.discover_server_clusters', but has {}", x.len())
                            )),
                    }
                } else { // router, end device
                    q_opt( v.discover_server_clusters.map(q_clusters) )
                }
            };

            q_lets.extend(quote!{
                let #ident = Endpoint {
                    common_fields: #q_common_fields,
                    device_type: #q_device_type,
                    additional_client_clusters: #q_additional_client_clusters,

                    discover_server_clusters: #q_discover_server_clusters,
                };
            });

            q_arr_contents.extend(quote!{ (#k, #ident), });

            Ok(())  // continue
        })?;

        quote!{ {
            #q_lets
            BTreeMap::from([#q_arr_contents])
        } }
    };

    // All together now!
    //
    // Note: Can use 'std::' since any ESP-IDF + Embassy application will need it.
    //
    let q_all = quote!{ {
        use std::collections::BTreeMap;
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

    // Note: 'prettyplease' is designed to handle full files. To handle an expression, we do some wrapping and unwrapping,
    //
    let q_all = quote! {
        fn dummy_wrapper() { #q }
    };

    // If there's something wrong with the 'quote!' macros,
    //let syntax_tree: syn::File = syn::parse2(q_all)?;

    // If the generated tokens are not compliant Rust, help by showing the tokens. THIS IS ENOUGH:
    // there's no reason for a fancier mechanism.
    //
    let syntax_tree: syn::File = syn::parse2(q_all.clone()).map_err(|e| {
        panic!(
         "INTERNAL: Code generation failed.\n\
         Message: {}\n\
         Tokens: {}",
            e, q_all.to_string()
        );
        e
    })?;

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

/// Turn 'Option' into a 'TokenStream', _keeping_ the Option abstraction.
///
/// @note Mere 'quote!(opt)' creates nothing on 'None' and the contents on 'Some'.
///
fn q_opt<T: quote::ToTokens>(opt: Option<T>) -> TokenStream {
    match opt {
        None => quote! { None },
        Some(x) => quote! { Some(#x) },
    }
}
