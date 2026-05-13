
mod in_;
use in_::*;

use quote::{format_ident, quote};
use toml;

/**
* Convert TOML input string to Rust snippet that generates an 'esp_zb::Config' instance, when read in.
*/
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

    // #[cfg(not(feature = "coordinator"))]
    // compile_error!("Use of Coordinator in TOML: please enable the 'coordinator' feature for your app crate.");
    // NodeType::CoordinatorConfig{ install_code_policy: false, max_children: 10 }
    //
    let q_node = {
        let q_feature_test = {
            let (node_type, node_feature) = match c.node {
                NodeSection::Coordinator {..} => ("coordinator", "coordinator"),
                NodeSection::Router {..} => ("router", "router"),
                //NodeSection::EndDevice {..} => ("end_device_UNTESTED", "end_device")
            };

            let msg = format!("TOML has: 'node.type = {node_type}' but feature \"{node_feature}\" is not enabled.");
            quote! {
            #[cfg(not(feature = #node_feature))]
            compile_error!(#msg);
        }
        };

        let q2 = match c.node {
            NodeSection::Coordinator { install_code_policy, max_children} => {
                quote! {
                    NodeType::CoordinatorConfig {
                        install_code_policy: #install_code_policy,
                        max_children: #max_children,
                    }
                }
            },
            NodeSection::Router { install_code_policy, max_children} => {
                quote! {
                    NodeType::RouterConfig {
                        install_code_policy: #install_code_policy,
                        max_children: #max_children,
                    }
                }
            },
            #[cfg(false)]
            NodeSection::EndDevice { install_code_policy } => {
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
        };

        quote!{ { #q_feature_test #q2 } }
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

        c.endpoint.instances.iter().for_each(|(k,v)| {
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
        let file = syn::parse2(q_all).unwrap();
        prettyplease::unparse(&file)
    })
}


// Would _love_ ❤️ to have tests, but we're generating text, not code. Need to:
//  - add 'esp_zb' to dev-dependencies
//  - generate an actual snippet (from a reference file; heck we can just point to 'apps/bin/*/app.toml' themselves)
//  - bring in the _generated_ snippet and
//  - run the tests at the 'Config' (struct) level
// Easy piecy!!!
//
#[cfg(false)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_toml() {
        let input = r#"
[network]
primary_channels = [13]

[platform]
storage_partition_name = "zb_storage"

[node]
type = "coordinator"
install_code_policy = false
max_children = 10

[endpoint.defaults]
manufacturer_name = "Your name"

[endpoint.10]
device_type = "color_dimmable_light"
"#;

        let out = parse_config(input).unwrap();

        assert_eq!(o.network.primary_channels, vec![13]);
        assert_eq!(o.node.r#type, "coordinator");
        assert_eq!(o.platform.storage_partition_name, "zb_storage");

        // Tarkistetaan endpoint 10
        let ep10 = o.endpoint.instances.get("10").unwrap();
        assert_eq!(ep10.device_type, "color_dimmable_light");

        // Tarkistetaan oletusarvot
        assert_eq!(o.endpoint.defaults.get("manufacturer_name").unwrap(), "Your name");
    }
}
