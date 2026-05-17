#![cfg(feature = "toml")]

mod in_;
use in_::*;

mod error;
use error::ConfigError::{
    self,
    FeatureConflict,
    //ContentError
};

use std::string::String;

use quote::{format_ident, quote};
use toml;
use crate::Config;

/**
* Convert TOML input string to Rust snippet that generates an 'esp_zb::Config' instance, when read in.
*/
// tbd. make it return _our_ error (no panics); one of which is Toml wrapper.
pub fn convert_toml(toml: &str) -> Result<String,ConfigError> {

    let c: RootConfig = toml::from_str(toml)?;  // may return a 'ParseError'

    //--- network
    // tbd. Secondary channel masks from the TOML. Should we? What options to give?
    //
    // [ChannelMask::from([13]), ChannelMask::ALL];
    //
    let q_channel_masks = {
        if c.network.primary_channels.is_empty() {
            Err("'network.primary_channels' is empty: please provide at least one channel to scan.")?;
        }

        let primary_channels = c.network.primary_channels;
        let secondary_channels = match c.network.secondary_channels {
            SecondaryChannels::Preferred => {
                quote!{ ChannelMask::PREFERRED }
            },
            SecondaryChannels::All => {
                quote!{ ChannelMask::ALL }
            },
        };

        quote! { [
            ChannelMask::from( [ #(#primary_channels),* ] ),
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

    //--- endpoint.defaults
    //
    //    BaseConfig{ manufacturer_name: "...", model_identifier: "..." }
    //
    let q_bc = {
        let endpoint_defaults = c.endpoint.defaults;

        // 'quote!' normally unwraps on 'Option' so we need to do this, to have output "Some(...)|None".
        //
        let mfn = endpoint_defaults.manufacturer_name.as_ref()
            .map(|s| quote! { Some(#s) })
            .unwrap_or_else(|| quote! { None });

        let mid = endpoint_defaults.model_identifier.as_ref()
            .map(|s| quote! { Some(#s) })
            .unwrap_or_else(|| quote! { None });

        quote! {
            BaseConfig{
                manufacturer_name: #mfn,
                model_identifier: #mid,
            }
        }
    };

    //--- endpoint.{id}
    // {
    //    let ep_10 = EndpointConfig::ColorDimmableLightEPC;
    //    BTreeMap::from([(10, ep_10)])
    // }
    let q_endpoints = {
        let mut q_lets = quote!{};
        let mut q_arr_contents = quote!{};

        if c.endpoint.instances.is_empty() {
            Err("Need at least one end point. Please define an '[endpoint.{id}]' section.")?;
        }

        c.endpoint.instances.iter().try_for_each(|(k,v)| -> Result<(),ConfigError> {
            // Skip ".defaults", turn others to 'u8'
            let k = match k.as_str() {
                "defaults" => { return Ok(()) },    // bypass, we get it through its own struct
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

            // tbd. when this grows, detach to a function
            let value = match v {
                EndpointInstance::ColorDimmableLight {} => quote! {
                    EndpointConfig::ColorDimmableLightEPC
                },
                // exhaustive match
            };

            q_lets.extend(quote!{ let #ident = #value; });
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
        use ezb_node::config::*;    // Config, BaseConfig, ...

        let channel_masks = #q_channel_masks;
        let storage_partition_name = #q_storage_partition_name;
        let node = #q_node;
        let endpoint_defs = #q_bc;
        let endpoints = #q_endpoints;

        Config {
            channel_masks,
            storage_partition_name,
            node,
            endpoint_defs,
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
    //      suggested by google.ai.
    //
    let q_file = quote::quote! {
        fn dummy_wrapper() { #q }
    };

    let syntax_tree: syn::File = syn::parse2(q_file)?;
    let formatted = prettyplease::unparse(&syntax_tree);

    let cleaned = formatted
        .replace("fn dummy_wrapper() {", "")
        .trim_end() // remove newlines from the end
        .strip_suffix('}').unwrap() // the closing brace of the dummy
        .trim() // final clean, both ends
        .to_string();
    Ok(cleaned)
}
