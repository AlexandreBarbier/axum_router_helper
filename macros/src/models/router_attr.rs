use syn::parse::{Parse, ParseStream};

#[derive(Debug, Clone, Default)]
pub struct RouterAttributes {
    pub state: Option<syn::Type>,
    pub session_type: Option<syn::Type>,
    pub base_path: Option<String>,
}

impl Parse for RouterAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut router_attr = Self::default();
        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            match ident.to_string().as_str() {
                "state" => {
                    let _: syn::Token![=] = input.parse()?;
                    let state: syn::Type = input.parse()?;
                    router_attr.state = Some(state);
                }
                "session_type" => {
                    let _: syn::Token![=] = input.parse()?;
                    let session_type: syn::Type = input.parse()?;
                    router_attr.session_type = Some(session_type);
                }
                "base_path" => {
                    let _: syn::Token![=] = input.parse()?;
                    let base_path: syn::LitStr = input.parse()?;
                    router_attr.base_path = Some(base_path.value());
                }
                _ => {
                    println!("Unknown attribute: {}", ident.to_string());
                }
            }
            if input.lookahead1().peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(router_attr)
    }
}
