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
    let session_data = router_attr
        .session_type
        .unwrap_or(syn::parse_quote!(SessionData));
    for it in parsed_item.items.iter() {
        let func = match it {
            syn::ImplItem::Fn(m) => m,
            _ => continue,
        };
        if func.attrs.is_empty() {
            continue;
        }

        for attr in func.attrs.iter() {
            let method = attr
                .path()
                .get_ident()
                .unwrap_or_else(|| panic!("method not found for {attr:?}"))
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
                func.sig.ident
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
        use axum_rh::router::traits::ArhRouter;
        #parsed_item

        impl ArhRouter<#state, #session_data> for #struct_name {
            fn endpoints() -> Vec<Endpoint<#state>> {
                #end_quote
            }
        }

    );
    TokenStream::from(k)
}

pub fn router_helper_derive(input: ItemStruct) -> TokenStream {
    let struct_name = &input.ident;
    if input.attrs.is_empty() {
        return syn::Error::new(
            struct_name.span(),
            "RouterHelper derive macro requires at least one attribute",
        )
        .to_compile_error()
        .into();
    }
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
        use axum_rh::router::traits::{ArhRouter, RouterHelper, Routers};

        impl RouterHelper<#state> for #struct_name {
            fn load_routers() -> Routers<#state> {
                let open_router = load_routers!(#(#routers),*);
                let protected_router = load_auth_routers!(#(#routers),*);
                Routers { open_router, protected_router}
            }

            fn load_routers_with_auth() -> axum::Router<#state> {
                load_auth_routers!(#(#routers),*)
            }

        }
    };
    TokenStream::from(expanded)
}
