use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Type};

#[proc_macro_derive(UnwrapFields)]
pub fn unwrap_fields_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let fields = match input.data {
        Data::Struct(s) => s.fields,
        _ => panic!("unwrap_fields can only be applied to structs"),
    };

    let unwrap_fields = fields
        .iter()
        .filter_map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_type = &field.ty;

            if let Type::Path(tp) = field_type {
                if tp.path.segments.len() == 1 && tp.path.segments[0].ident == "Option" {
                    let unwrapped_type = &tp.path.segments[0].arguments;

                    match unwrapped_type {
                        syn::PathArguments::AngleBracketed(args) => {
                            let inner_type = &args.args[0];
                            let return_type: syn::Type = syn::parse_quote! { &#inner_type };
                            Some(quote! {
                                pub fn #field_name(&self) -> #return_type {
                                    self.#field_name.as_ref().unwrap_or_else(||panic!("Field is None"))
                                }
                            })
                        }
                        _ => panic!("unwrap_fields can only be applied to Option<T> fields"),
                    }
                } else {
                    None
                }
            } else {
                panic!("unwrap_fields can only be applied to Option<T> fields");
            }
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        impl #struct_name {
            #(#unwrap_fields)*
        }
    };

    TokenStream::from(expanded)
}
