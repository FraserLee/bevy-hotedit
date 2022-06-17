use proc_macro::TokenStream;
use quote::quote;
use syn::{ self, parse_macro_input };

#[proc_macro_attribute]
pub fn set_const_value(args: TokenStream, item: TokenStream) -> TokenStream {
    let _args = parse_macro_input!(args as syn::AttributeArgs);
    let item  = parse_macro_input!(item as syn::TraitItemConst);

    let name = &item.ident;
    let ty = &item.ty;

    let new_item: TokenStream = quote! {
        const #name : #ty = 1 as #ty;
    }.into();

    // println!("new_item: \"{}\"", new_item.to_string());

    new_item
}









