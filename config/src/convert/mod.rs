#![cfg(feature = "toml")]

use std::string::String;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use toml;

use crate::{CommonFields, Config};

mod in_;
use in_::*;

mod error;
use error::ConfigError::{
    self,
    FeatureConflict,
    //ContentError
};

/**
* Convert TOML input string to Rust snippet that generates an 'esp_zb::Config' instance, when read in.
*/
// tbd. make it return _our_ error (no panics); one of which is Toml wrapper.
pub fn convert_toml(toml: &str) -> Result<String,ConfigError> {

    let c: RootConfig = toml::from_str(toml)?;  // may return a 'ParseError'

    //--- network
    // tbd. Secondary channel masks from the TOML. Should we? What options to give?
    //
    // [ChannelMask::new(1 << 13), ChannelMask::ALL];
    //
    let q_channel_masks = {
        if c.network.primary_channels.is_empty() {
            Err("'network.primary_channels' is empty: please provide at least one channel to scan.")?;
        }

        let q_primary_channels = {
            let qs = c.network.primary_channels.into_iter().map(|v| {
                quote!{ 1 << #v }
            });
            quote!{ #(#qs)|* }    // 1 << 11u8 | 1 << 12u8 | ...
        };

        let secondary_channels = match c.network.secondary_channels {
            SecondaryChannels::Preferred => {
                quote!{ ChannelMask::PREFERRED }
            },
            SecondaryChannels::All => {
                quote!{ ChannelMask::ALL }
            },
        };

        quote! { [
            ChannelMask::new( #q_primary_channels ),
            #secondary_channels
        ] }
    };

    //--- platform
    // "{string}"
    let q_storage_partition_name = {
        let s = c.platform.storage_partition_name;
        quote! { #s }
    };

    //--- node

    // Note: We don't need to create '#[cfg]' barriers in the output; we assume that the features
    //      given to us are the same the application carries.
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
    //    let ep_10 = (CommonFields{ ... }, Specific::ColorDimmableLightEPC);
    //    BTreeMap::from([(10, ep_10)])
    // }
    let q_endpoints = {
        let mut q_lets = quote!{};
        let mut q_arr_contents = quote!{};

        if c.endpoint.instances.is_empty() {
            Err("Need at least one end point. Please define an '[endpoint.{id}]' section.")?;
        }

        let EndpointDefaults {
            manufacturer_name: manufacturer_name_def,
            model_identifier: model_identifier_def
        } = &c.endpoint.defaults;

        // '.as_ref()'s turn to 'Option<&String>' (important later)
        let manufacturer_name_def = manufacturer_name_def.as_ref();
        let model_identifier_def = model_identifier_def.as_ref();

        c.endpoint.instances.iter().try_for_each(|(k,v)| -> Result<(),ConfigError> {
            // Skip ".defaults", turn others to 'u8'
            let k = match k.as_str() {
                "defaults" => { return Ok(()) }, // skip
                id => {
                    let endpoint_id = id.parse::<u8>().map_err(|_| {
                        format!("Invalid endpoint ID (not 'u8'): {}", id)
                    })?;
                    if !Config::is_valid_endpoint(endpoint_id) {
                        Err(format!("Invalid endpoint ID (not in valid range 1..=240): {}", id))?;
                    }
                    endpoint_id
                }
            };  // k ∈ 1..=240

            let ident = format_ident!("ep_{}", k);

            let q_cf = {
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

            // tbd. when this grows, detach to a function
            let q_specific = match v {
                EndpointInstance::ColorDimmableLight {} => quote! {
                    Specific::ColorDimmableLightEPC
                },
                // exhaustive match
            };

            q_lets.extend(quote!{ let #ident = EndpointConfig(#q_cf, #q_specific); });
            q_arr_contents.extend(quote!{ (#k, #ident) });

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

        let channel_masks = #q_channel_masks;
        let storage_partition_name = #q_storage_partition_name;
        let node = #q_node;
        let endpoints = #q_endpoints;

        Config {
            channel_masks,
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
