#![cfg(feature = "gen")]

use std::{
    string::String,
};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
//use syn::token::Token;
use toml;

// We'll need it once end device joins in.
use humantime_serde as _;

mod in_;
use in_::{
    RootConfig,
    SecondaryChannels,
    NodeSection,
    EndpointDefaults,
    EndpointInstance,
    DeviceType,
    Cluster,
};
#[cfg(feature = "coordinator")]
use in_::PreConfiguredDevice;

mod error;
use error::ConfigError::{
    self,
};

// tbd. This is SO LONG!!! Find ways to split it into a few.
/**
* Convert TOML input string to Rust module that can provide an 'esp_zb::Config' instance.
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

    //--- node

    #[allow(non_snake_case)]
    let ROLE: &'static str;

    let mut is_router = false;
    let mut is_coordinator = false;

    // Helper functions; by default renders to nothing
    let mut q_helpers = quote!();

    // Feature notes:
    //      - The 'build.rs' side (feature = "gen") DOES NOT use coordinator/router/end_device features.
    //      - by parsing all possible node types, we may allow the user to get better error messages
    //      - We also don't create '#[cfg]' barriers in the generated output (we could!), avoiding complexity.
    //        Just expect the user to bring in enough features.
    //
    let q_node = {
        match c.node {
            #[cfg(feature = "coordinator")]
            NodeSection::Coordinator { /*r allow_open_pairing,*/ max_children, pre_configured_devices } => {
                ROLE = "Coordinator";
                is_coordinator = true;

                let q_pre_configured_devices: Vec<TokenStream> = pre_configured_devices .iter()
                    .map(|o| {
                        let PreConfiguredDevice{ mac_address, install_code, _name } = o;

                        // Convert the hex strings (back to) the format they were on the Zigbee device packaging.
                        // This is to keep the generated code human-friendly.
                        fn f(bytes: &[u8]) -> TokenStream {
                            let s = bytes
                                .chunks(2)
                                .map(|chunk| {
                                    assert_eq!(chunk.len(), 2);
                                    format!("{:02X}{:02X}", chunk[0], chunk[1])
                                })
                                .collect ::<Vec<String>>()  // note: '.join()' doesn't work on iterators; must collect
                                .join(" ");
                            quote!(#s)
                        }
                        let q_mac_hex = f(mac_address);
                        let q_install_code_hex = f(install_code);

                        let q_name = quote!(#_name);

                        // quote! note: When we want to include a comment with dynamic contents, '///'
                        //      will not cut it. Use of '#[doc = ...]' does.
                        //
                        Ok(quote! {
                            #[doc = #q_name]
                            PreConfiguredDevice {
                                mac_address: hex_bytes( #q_mac_hex ),
                                install_code: hex_bytes( #q_install_code_hex ),
                            }
                        })
                    }).collect ::<Result<_,ConfigError>>()?;

                // Note: Let's output helpers even when there are no pre-configured devices.
                //      This helps catch possible generative errors, since those will show up
                //      just having a Coordinator, not being "behind the corner" of whether we
                //      have pre-configured-devices or not.
                //
                q_helpers = quote!(
                    // Convert human-readable hex string.
                    #[allow(dead_code)]
                    const fn hex_bytes<const N: usize>(s: &str) -> [u8; N] {
                        let s = s.as_bytes();   // so it's easy to iterate (it's ASCII)

                        let mut buf = [0u8; N];
                        let mut j = 0;  // position within 's' (jumps over spaces)

                        // Rust note: cannot use 'for .. 0..N' in 'const fn'. Revert to 'while'
                        //for i in 0..N {

                        let mut i = 0;
                        while i < N {
                            while s[j] == b' ' { j += 1; }; // step over space(s)

                            // Rust note: '&s[x..y]' does not work within 'const fn'
                            //buf[i] = bite(&s[j..j+2]);
                            buf[i] = bite(&[s[j],s[j+1]]);
                            j += 2;
                            i += 1;
                        }
                        assert!(j == s.len()); // all processed (note: 'assert_eq' not supported in 'const fn')
                        buf
                    }

                    // Helper: two hex chars -> full byte
                    #[allow(dead_code)]
                    const fn bite(sl: &[u8;2]) -> u8 {
                        const fn nibble(c: u8) -> u8 {
                            match c as u8 {
                                b'0'..=b'9' => c - b'0',
                                b'A'..=b'F' => c - b'A' + 10,
                                _ => unreachable!(),
                            }
                        }
                        let hi = nibble(sl[0]);
                        let lo = nibble(sl[1]);
                        (hi << 4) | lo
                    }
                );

                quote!(
                    NodeType::CoordinatorConfig {
                        max_children: #max_children,
                        pre_configured_devices: vec![ #( #q_pre_configured_devices ),* ],
                    }
                )
            },

            #[cfg(feature = "router")]
            NodeSection::Router { allow_open_pairing, max_children} => {
                let install_code_policy = !allow_open_pairing;
                ROLE = "Router";
                is_router = true;

                quote!(
                    NodeType::RouterConfig {
                        install_code_policy: #install_code_policy,
                        max_children: #max_children,
                    }
                )
            },

            #[cfg(false)] //#[cfg(feature = "end_device_UNUSED")]
            NodeSection::EndDevice { allow_open_pairing: _, .. } => {
                unimplemented!();
                ROLE = "End device";

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
        if instances.is_empty() && !is_router {
            Err(
                format!("Need at least one end point for {ROLE}. Please define '[endpoint.{{id}}]'.")
            )?;
        }

        // '.as_ref()'s turn to 'Option<&String>' (important later)
        let manufacturer_name_def = manufacturer_name_def.as_ref();
        let model_identifier_def = model_identifier_def.as_ref();

        instances.into_iter().try_for_each(|(k,v): (u8,EndpointInstance)| { // k ∈ 1..=240
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

            // Inject the 'discover_server_clusters: ...' for router and end device, not for
            // coordinator.
            //
            let q_discover_server_clusters_field: TokenStream = {

                if cfg!(feature = "coordinator") && is_coordinator {
                    // If the TOML does have the field, we only realise it here.
                    if ! v.discover_server_clusters.is_none() {
                        Err( format!("Coordinator should not carry '.discover_server_clusters'."))?;
                    }
                    quote!()    // gen nothing
                } else {
                    // router, end device has the field
                    let q = q_opt( v.discover_server_clusters.map(q_clusters) );
                    quote!( discover_server_clusters: #q )
                }
            };

            q_lets.extend(quote!{
                let #ident = Endpoint {
                    common_fields: #q_common_fields,
                    device_type: #q_device_type,
                    additional_client_clusters: #q_additional_client_clusters,
                    #q_discover_server_clusters_field
                };
            });

            q_arr_contents.extend(quote!{ (#k, #ident), });

            Ok::<_,ConfigError>(())  // continue
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
    // 'quote' notes:
    //      - Regular comments '//' get eaten out, but '///', '//!' (doc comments) remain.
    //
    let q_all = quote!(
        //! Generated code.
        #![allow(unused_doc_comments)]

        use std::{
            collections::BTreeMap,
            sync::OnceLock,
        };

        use ezb_node::config::*; // Config, ChannelMask, NodeType, ...
        pub use ezb_node::config::Config;

        #q_helpers

        static CFG: OnceLock<Config> = OnceLock::new();

        pub fn get() -> &'static Config {
            CFG.get_or_init(|| {
                let primary_channels = #q_primary_channels;
                let secondary_channels = #q_secondary_channels;
                let node = #q_node;
                let endpoints = #q_endpoints;

                Config {
                    primary_channels,
                    secondary_channels,
                    node,
                    endpoints
                }
            })
        }
    );

    let s= final_(q_all)?;
    Ok(s)
}

/**
* Check a Rust expression for syntactical correctness, and format it similar to 'rustfmt'.
*/
fn final_(q_all: TokenStream) -> Result<String,ConfigError> {

    // If there's something wrong with the 'quote!' macros,
    //let syntax_tree: syn::File = syn::parse2(q_all)?;

    // If the generated tokens are not compliant Rust, help by showing the tokens. THIS IS ENOUGH:
    // there's no reason for a fancier mechanism.
    //
    let syntax_tree: syn::File = syn::parse2(q_all.clone()).map_err(|e| {
        format!(
             "INTERNAL: Code generation failed.\n\
             Message: {}\n\
             Tokens: {}",
                e, q_all.to_string()
        )
    })?;

    let formatted = prettyplease::unparse(&syntax_tree);
    Ok(formatted)
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
