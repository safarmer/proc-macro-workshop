use crate::lower::{FieldIr, Ir};
use quote::{quote, quote_spanned};

pub type Rust = proc_macro::TokenStream;

pub fn codegen(ir: Ir) -> Rust {
    let Ir {
        target,
        builder,
        fields,
    } = ir;

    let builder_fields = fields
        .iter()
        .map(|FieldIr { ty, name, span, .. }| quote_spanned!(*span=> #name: Option<#ty>));

    let builder_init = fields
        .iter()
        .map(|FieldIr { name, span, .. }| quote_spanned!(*span=> #name: None));

    let setters = fields.iter().map(|FieldIr { name, ty, span, .. }| {
        quote_spanned! {*span=>
            pub fn #name(&mut self, #name: #ty) -> &mut Self {
                self.#name = Some(#name);
                self
            }
        }
    });

    let build_calls = fields.iter().map(|f| {
        let FieldIr {
            name,
            span,
            required,
            ..
        } = f;

        if *required {
            let msg = format!("missing value for field '{}'", name.clone());
            quote_spanned! {*span=>
                #name: self.#name.take().ok_or(Box::<dyn Error>::from(#msg))?
            }
        } else {
            quote_spanned! {*span=>
                #name: self.#name.clone()
            }
        }
    });

    let expanded = quote! {
        use std::error::Error;

        pub struct #builder {
            #( #builder_fields ),*
        }

        impl #builder {

            #( #setters )*

            pub fn build(&mut self) -> Result<#target, Box<dyn Error>> {
                Ok(#target {
                    #( #build_calls ),*
                })
            }
        }

        impl #target {
            pub fn builder() -> #builder {
                #builder {
                    #( #builder_init ),*
                }
            }
        }

    };

    expanded.into()
}
