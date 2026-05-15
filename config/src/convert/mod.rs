#![cfg(feature = "toml")]

mod in_;
use in_::*;

use quote::{format_ident, quote};
use toml;
use crate::Config;

/**
* Convert TOML input string to Rust snippet that generates an 'esp_zb::Config' instance, when read in.
*/
// tbd. make it return _our_ error (no panics); one of which is Toml wrapper.
pub fn convert_toml(toml: &str) -> Result<String,toml::de::Error> {

    let c: RootConfig = toml::from_str(toml)?;

    // Flatten things

    //--- network
    // tbd. Secondary channel masks from the TOML. Should we? What options to give?
    //
    // [ChannelMask::from([13]), ChannelMask::ALL];
    //
    let q_channel_masks = {
        let primary_channels = c.network.primary_channels;
        quote! { [
            ChannelMask::from( [ #(#primary_channels),* ] ),
            ChannelMask::ALL
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
                #[cfg(not(feature = "coordinator"))]
                panic!("'node.type' \"coordinator\" but that feature is not enabled.");
                quote! {
                    NodeType::CoordinatorConfig {
                        install_code_policy: #install_code_policy,
                        max_children: #max_children,
                    }
                }
            },
            NodeSection::Router { install_code_policy, max_children} => {
                #[cfg(not(feature = "router"))]
                panic!("'node.type' \"router\" but that feature is not enabled.");
                quote! {
                    NodeType::RouterConfig {
                        install_code_policy: #install_code_policy,
                        max_children: #max_children,
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

    //--- endpoint
    // {
    //    let bc = BaseConfig{ manufacturer_name: "...", model_identifier: "..." }
    //    let ep_10 = EndpointConfig::ColorDimmableLightEPC;
    //    BTreeMap::from([(10, (ep_10, bc))])
    // }
    let q_endpoints = {
        let mut q_lets = quote!{};
        let mut q_arr_contents = quote!{};

        if c.endpoints.is_empty() {
            panic!("Need at least one end point. Please define an '[endpoint.{{id}}]' section.");
        }

        c.endpoint.instances.iter().for_each(|(k,v)| {
            if !Config::VALID_ENDPOINT_IDS.contains(k) {
                panic!("Invalid endpoint ID: {}", k);
            }

            let ident = format_ident!("ep_{}", k);

            // tbd. when this grows, detach to a function
            let value = match v {
                EndpointInstance::ColorDimmableLight {} => quote! {
                    EndpointConfig::ColorDimmableLightEPC
                },
                // exhaustive match
            };

            q_lets.extend(quote!{ let #ident = #value; });
            q_arr_contents.extend(quote!{ (#k, (#ident, bc)) })
        });

        let q_bc = {
            let mfn = c.endpoint.defaults.manufacturer_name;
            let mid = c.endpoint.defaults.model_identifier;
            quote! {
                BaseConfig{
                    manufacturer_name: #mfn,
                    model_identifier: #mid,
                }
            }
        };

        quote!{ {
            let bc = #q_bc;
            #q_lets
            BTreeMap::from([#q_arr_contents])
        } }
    };

    // All together now!
    //
    let q_all = quote!{
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
    };

    // We can now either:
    #[cfg(false)]
    let s = q_all.to_string();

    Ok({
        let syntax_tree = syn::parse2(q_all).unwrap();
        prettyplease::unparse(&syntax_tree)
    })
}
