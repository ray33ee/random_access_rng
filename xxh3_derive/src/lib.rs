use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(XXH3)]
pub fn derive_xxh3(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    let hash_expr = match input.data {
        Data::Struct(data) => {
            let field_hashes: Vec<_> = match data.fields {
                Fields::Named(ref fields) => {
                    fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote! {
                            h ^= XXH3::xxh3(&self.#name);
                        }
                    }).collect()
                }
                Fields::Unnamed(ref fields) => {
                    fields.unnamed.iter().enumerate().map(|(i, _)| {
                        let index = syn::Index::from(i);
                        quote! {
                            h ^= XXH3::xxh3(&self.#index);
                        }
                    }).collect()
                }
                Fields::Unit => {
                    return quote! {
                        impl XXH3 for #ident {
                            fn xxh3(&self) -> u128 {
                                0
                            }
                        }
                    }.into();
                }
            };

            quote! {
                let mut h = 0;
                #(#field_hashes)*
                h
            }
        }

        Data::Enum(data_enum) => {
            let match_arms = data_enum.variants.iter().enumerate().map(|(variant_index, v)| {
                let v_ident = &v.ident;
                let tag = variant_index;

                match &v.fields {
                    Fields::Named(fields) => {
                        let field_names: Vec<_> = fields.named.iter()
                            .map(|f| f.ident.as_ref().unwrap())
                            .collect();
                        let hash_lines: Vec<_> = field_names.iter().map(|name| {
                            quote! { h ^= XXH3::xxh3(#name); }
                        }).collect();

                        quote! {
                            Self::#v_ident { #(ref #field_names),* } => {
                                let mut h = XXH3::xxh3(&#tag);
                                #(#hash_lines)*
                                h
                            }
                        }
                    }

                    Fields::Unnamed(fields) => {
                        let bindings: Vec<syn::Ident> = (0..fields.unnamed.len())
                            .map(|i| syn::Ident::new(&format!("f{}", i), v_ident.span()))
                            .collect();
                        let hash_lines: Vec<_> = bindings.iter().map(|f| {
                            quote! { h ^= XXH3::xxh3(#f); }
                        }).collect();

                        quote! {
                            Self::#v_ident( #(ref #bindings),* ) => {
                                let mut h = XXH3::xxh3(&#tag);
                                #(#hash_lines)*
                                h
                            }
                        }
                    }

                    Fields::Unit => {
                        quote! {
                            Self::#v_ident => XXH3::xxh3(&#tag)
                        }
                    }
                }
            });

            quote! {
                match self {
                    #(#match_arms),*
                }
            }
        }

        Data::Union(_) => panic!("XXH3 cannot be derived for unions"),
    };

    let expanded = quote! {
        impl XXH3 for #ident {
            fn xxh3(&self) -> u128 {
                use xxh3_derive::XXH3;
                #hash_expr
            }
        }
    };

    TokenStream::from(expanded)
}
