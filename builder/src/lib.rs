// use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};

use syn::{Data, DeriveInput, Fields, parse_macro_input};
use syn::spanned::Spanned;

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let builder_name = format_ident!("{}Builder", name);

    let mut fs = vec![];
    let mut defs = vec![];
    let mut ss = vec![];

    if let Data::Struct(ref data) = input.data {
        if let Fields::Named(ref fields) = data.fields {
            for f in fields.named.iter() {
                let name = &f.ident;
                let ty = &f.ty;
                fs.push(quote_spanned! {(f.span()=> #name: Option<#ty>)});
                defs.push(quote_spanned!(f.span()=> #name: None));
                ss.push(quote_spanned! {f.span()=>
                    pub fn #name(&mut self, #name: #ty) -> &mut self {
                        self.#name = Some(#name);
                        self
                    }
                });
            }
        }
    }

    let expanded = quote! {

        pub struct #builder_name {
            #( #fs ),*
        }

        impl #builder_name {
            #( #ss )*
        }

        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #( #defs ),*
                }
            }
        }

    };

    proc_macro::TokenStream::from(expanded)
}
