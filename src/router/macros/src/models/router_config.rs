use syn::parse::{Parse, ParseStream};

#[derive(Debug)]
pub struct RouterConfiguration {
    pub state_type: syn::Type,
    pub routers: Vec<syn::Ident>,
}

impl Parse for RouterConfiguration {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let state_type: syn::Type = input.parse().expect("state is not a type");
        if input.lookahead1().peek(syn::Token![,]) {
            input.parse::<syn::Token![,]>()?;
        }

        let mut routers = Vec::new();
        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            routers.push(ident);
            if input.lookahead1().peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(RouterConfiguration {
            state_type,
            routers,
        })
    }
}
