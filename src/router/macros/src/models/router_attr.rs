use syn::parse::{Parse, ParseStream};

#[derive(Debug, Clone)]
pub struct RouterAttributes {
    pub state: Option<syn::Type>,
    pub base_path: Option<String>,
}

impl Default for RouterAttributes {
    fn default() -> Self {
        Self {
            state: None,
            base_path: None,
        }
    }
}
impl Parse for RouterAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut sc = Self::default();
        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            match ident.to_string().as_str() {
                "state" => {
                    let _: syn::Token![=] = input.parse()?;
                    let state: syn::Type = input.parse()?;
                    sc.state = Some(state);
                }
                "base_path" => {
                    let _: syn::Token![=] = input.parse()?;
                    let base_path: syn::LitStr = input.parse()?;
                    sc.base_path = Some(base_path.value());
                }
                _ => {
                    let state: syn::Type = syn::parse_quote!(#ident);
                    sc.state = Some(state);
                }
            }
            if input.lookahead1().peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(sc)
    }
}
