use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(DisplayFromFormat)]
pub fn derive_display_from_format(input: TokenStream) -> TokenStream {
    let input = syn::parse(input).unwrap();
    let DeriveInput { ident, .. } = input;

    let output = quote! {
        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
                let mut res = String::new();
                self.format(&mut Formatter::new(&mut res))?;
                write!(f, "{res}")
            }
        }
    };

    output.into()
}
