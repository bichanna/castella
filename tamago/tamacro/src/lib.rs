use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Expr, Lit, Meta, Variant};

#[proc_macro_derive(DisplayFromFormat)]
pub fn derive_display_from_format(input: TokenStream) -> TokenStream {
    let input = syn::parse(input).unwrap_or_else(|_| {
        panic!("Failed to parse");
    });
    let DeriveInput { ident, .. } = input;

    let output = quote! {
        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut res = String::new();
                self.format(&mut Formatter::new(&mut res))?;
                write!(f, "{res}")
            }
        }
    };

    output.into()
}

#[proc_macro_derive(DisplayFromConstSymbol, attributes(symbol))]
pub fn derive_display_from_const_symbol(input: TokenStream) -> TokenStream {
    let input = syn::parse(input).unwrap_or_else(|_| {
        panic!("Failed to parse");
    });
    let DeriveInput {
        ident: enum_name,
        data,
        ..
    } = input;

    let variants = match data {
        Data::Enum(data_enum) => data_enum.variants,
        _ => panic!("DisplayFromConstSymbol derive macro can only be used on enums"),
    };

    let match_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let symbol = get_symbol_attr(variant).unwrap_or_else(|| {
            panic!("Each variant of {enum_name} must have a #[symbol = \"...\"] attribute");
        });

        quote! {
            #enum_name::#variant_name => write!(fmt, #symbol),
        }
    });

    let output = quote! {
        impl std::fmt::Display for #enum_name {
            fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use #enum_name::*;
                match self {
                    #(#match_arms)*
                }
            }
        }
    };

    output.into()
}

#[proc_macro_derive(FormatFromConstSymbol, attributes(symbol))]
pub fn derive_format_from_const_symbol(input: TokenStream) -> TokenStream {
    let input = syn::parse(input).unwrap_or_else(|_| {
        panic!("Failed to parse");
    });
    let DeriveInput {
        ident: enum_name,
        data,
        ..
    } = input;

    let variants = match data {
        Data::Enum(data_enum) => data_enum.variants,
        _ => panic!("FormatFromConstSymbol derive macro can only be used on enums"),
    };

    let match_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let symbol = get_symbol_attr(variant).unwrap_or_else(|| {
            panic!("Each variant of {enum_name} must have a #[symbol = \"...\"] attribute");
        });

        quote! {
            #enum_name::#variant_name => write!(fmt, #symbol),
        }
    });

    let output = quote! {
        impl crate::formatter::Format for #enum_name {
            fn format(&self, fmt: &mut crate::formatter::Formatter<'_>) -> fmt::Result {
                use #enum_name::*;
                match self {
                    #(#match_arms)*
                }
            }
        }
    };

    output.into()
}

fn get_symbol_attr(variant: &Variant) -> Option<String> {
    for attr in &variant.attrs {
        if attr.path().is_ident("symbol") {
            if let Meta::NameValue(name_value) = &attr.meta {
                if let Expr::Lit(lit) = &name_value.value {
                    if let Lit::Str(lit_str) = &lit.lit {
                        return Some(lit_str.value());
                    }
                }
            }
        }
    }
    None
}
