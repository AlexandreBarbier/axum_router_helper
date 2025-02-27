use crate::models::{
    endpoint_attr::value_for_endpoint, EndpointAttributes, RouterAttributes, RouterConfiguration,
};
use log::debug;
use proc_macro::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, ItemImpl, ItemStruct};

pub fn router(router_attr: RouterAttributes, parsed_item: ItemImpl) -> TokenStream {
    let struct_name = parsed_item.self_ty.clone();

    let mut endpoints: Vec<proc_macro2::TokenStream> = Vec::new();
    let state = router_attr.state.unwrap_or(syn::parse_quote!(()));
    for it in parsed_item.items.iter() {
        let func = match it {
            syn::ImplItem::Fn(m) => m,
            _ => continue,
        };
        if func.attrs.len() == 0 {
            continue;
        }

        for attr in func.attrs.iter() {
            let method = attr
                .path()
                .get_ident()
                .expect(format!("method not found for {:?}", attr).as_str())
                .to_string()
                .as_str()
                .to_lowercase();

            let parsed_attr = attr.parse_args::<EndpointAttributes>();
            let parsed_attr = match parsed_attr {
                Ok(v) => v,
                Err(e) => {
                    println!("{:?} {:?}", e, attr.path().get_ident());
                    continue;
                }
            };
            debug!(
                "detected endpoint {}::{}",
                parsed_item
                    .self_ty
                    .as_ref()
                    .span()
                    .source_text()
                    .unwrap_or_default(),
                func.sig.ident.to_string()
            );

            endpoints.push(value_for_endpoint(
                router_attr.base_path.clone(),
                parsed_attr,
                func,
                method,
            ));
        }
    }
    let end_quote = quote! {
        vec![
            #(#endpoints),*
        ]
    };

    let k: proc_macro2::TokenStream = quote!(
        use axum_rh::router::models::Endpoint;
        use axum_rh::router::traits::ApiRouter;
        #parsed_item

        impl ApiRouter<#state> for #struct_name {
            fn endpoints() -> Vec<Endpoint<#state>> {
                #end_quote
            }
        }

    );
    TokenStream::from(k)
}

pub fn router_helper_derive(input: ItemStruct) -> TokenStream {
    let struct_name = &input.ident;
    let router_config = match input.attrs[0].meta.require_list() {
        Ok(v) => match v.parse_args::<RouterConfiguration>() {
            Ok(rc) => rc,
            Err(e) => {
                return e.to_compile_error().into();
            }
        },
        Err(e) => {
            return e.to_compile_error().into();
        }
    };

    let state: syn::Type = router_config.state_type;
    let routers = router_config.routers;
    let expanded = quote! {
        use axum_rh::router::traits::{ApiRouter, RouterHelper};

        impl RouterHelper<#state> for #struct_name {
            fn load_routers() -> axum::Router<#state> {
                load_routers!(#(#routers),*)
            }
        }
    };
    TokenStream::from(expanded)
}
