extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::Item::{self, Fn};
use syn::Ident;

#[proc_macro_attribute]
pub fn handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);
    match item {
        Fn(ast) => {
            let name = &ast.ident;
            let vis = &ast.vis;
            let handler_name = syn::Ident::new(&format!("__handler_for_{}", name), proc_macro2::Span::call_site());
            let gen = quote! {
                #ast

                #vis fn #handler_name(ctx: &mut Context, data: &ParseData) -> Result<Action> {
                    match data {
                        MessagePack(data) => #name(ctx, &rmp_serde::decode::from_slice(&data[..]).unwrap()),
                        JSON(string) => #name(ctx, &serde_json::from_str(&string).unwrap()),
                    }
                }
            };
            gen.into()
        },
        _ => {
            panic!("Attribute target should be function");
        }
    }
}

#[proc_macro]
pub fn handler_fn(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);
    let handler_fn = Ident::new(&format!("__handler_for_{}", ident), ident.span());
    let gen = quote! {
        #handler_fn
    };
    gen.into()
}