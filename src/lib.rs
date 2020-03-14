use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, Fields, Attribute, parse_macro_input, DeriveInput};
use syn::spanned::Spanned;

#[proc_macro_derive(EnumTextAugmentation, attributes(enum_text_augmentation))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let tokens = match input.data {
        Data::Enum(DataEnum{ variants, .. }) => {
            let enum_ident = input.ident;
            let cases = variants
                .iter()
                .map(|variant| {
                    let variant_ident = &variant.ident;
                    variant.attrs
                        .iter()
                        .find(|attribute| is_enum_text_augmentation_attribute(attribute))
                        .map(|attribute| enum_text_augmentation_attribute_to_string(attribute))
                        .unwrap_or_else(|| Ok(ident_to_text(variant_ident)))
                        .map(|text| {
                            match variant.fields {
                                Fields::Named(..) => quote! { Self::#variant_ident{ .. } => #text },
                                Fields::Unnamed(..) => quote! { Self::#variant_ident(..) => #text },
                                Fields::Unit => quote! { Self::#variant_ident => #text },
                            }
                        })
                })
                .collect::<Vec<_>>();
            let (cases, errors): (Vec<_>, Vec<_>) = cases
                .into_iter()
                .partition(Result::is_ok);
            if errors.is_empty() {
                let cases = cases
                    .into_iter()
                    .map(Result::unwrap)
                    .collect::<Vec<_>>();
                quote! {
                    impl #enum_ident {
                        fn augmented_text(&self) -> &'static str {
                            match self {
                                #(#cases,)*
                            }
                        }
                    }
                }
            } else {
                let errors = errors
                    .into_iter()
                    .map(Result::unwrap_err)
                    .map(|error| error.to_compile_error())
                    .collect::<Vec<_>>();
                quote! { 
                    // Write out a dummy implementation for the method, so that the compiler only complains about the attribute and not also about the method that is used but not defined.
                    impl #enum_ident {
                        fn augmented_text(&self) -> &'static str {
                            unimplemented!()
                        }
                    }
                    // Write out the compile_error!(..) invocations.
                    #(#errors;)* 
                }
            }
        }
        _ => panic!("The EnumTextAugmentation attribute can only be assigned to an enum."),
    };
    tokens.into()
}

fn ident_to_text(ident: &syn::Ident) -> String {
    ident
        .to_string()
        .chars()
        .enumerate()
        .map(|(index, c)| if index > 0 && c.is_uppercase() { format!(" {}", c.to_lowercase()) } else { c.to_string() })
        .collect::<String>()
}

fn is_enum_text_augmentation_attribute(attribute: &Attribute) -> bool {
    attribute.path.segments
        .first()
        .map(|first| first.ident == "enum_text_augmentation")
        .unwrap_or(false)
}

fn enum_text_augmentation_attribute_to_string(attribute: &Attribute) -> syn::parse::Result<String> {
    let expected_string_message = "expected a String like \"#[enum_text_augmentation(\"My custom string.\")]\".";
    match attribute.parse_meta()? {
        syn::Meta::List(list) => {
            let syn::MetaList{ nested, .. } = &list;
            nested
                .into_iter()
                .next()
                .map(|lit| {
                    match lit {
                        syn::NestedMeta::Lit(syn::Lit::Str(lit)) => Ok(lit.value()),
                        _ => Err(syn::Error::new(lit.span(), expected_string_message))
                    }
                })
                .unwrap_or(Err(syn::Error::new(list.span(), expected_string_message)))
        }
        _ => Err(syn::Error::new(attribute.path.span(), expected_string_message))
    }
}
