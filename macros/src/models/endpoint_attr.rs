use quote::quote;
use syn::parse::{Parse, ParseStream};

#[derive(Debug, Clone, Default)]
pub struct EndpointAttributes {
    pub path: String,
    pub auth: bool,
}

impl Parse for EndpointAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ep_attr = Self::default();
        while !input.is_empty() {
            if input.lookahead1().peek(syn::LitStr) {
                let path: syn::LitStr = input.parse()?;
                ep_attr.path = path.value();
            } else {
                let ident: syn::Ident = input.parse()?;
                match ident.to_string().as_str() {
                    "path" => {
                        let _: syn::Token![=] = input.parse()?;
                        let path: syn::LitStr = input.parse()?;
                        ep_attr.path = path.value();
                    }
                    "auth" => {
                        let _: syn::Token![=] = input.parse()?;
                        let auth: syn::LitBool = input.parse()?;
                        ep_attr.auth = auth.value;
                    }
                    _ => {
                        println!("Unknown attribute: {}", ident.to_string());
                    }
                }
            }
            if input.lookahead1().peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(ep_attr)
    }
}

pub fn value_for_endpoint(
    base_path: Option<String>,
    attr: EndpointAttributes,
    func: &syn::ImplItemFn,
    method: String,
) -> proc_macro2::TokenStream {
    let path = base_path.unwrap_or_default() + &attr.path;
    let auth = attr.auth;
    let method = syn::Ident::new(method.as_str(), func.sig.ident.span());
    let func_name = &func.sig.ident;

    quote! {
        Endpoint {
            path: #path,
            authenticated: #auth,
            handler: axum::routing::#method(Self::#func_name),
        }
    }
}
