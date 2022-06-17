use proc_macro::TokenStream;
use quote::quote;
// use syn;

#[proc_macro_attribute]
pub fn set_const_value(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // replace whatever's below the macro to 'const #name: i32 = 1;'
    println!("item: \"{}\"", item.to_string());

    let new_item: TokenStream = quote! {
        const X: i32 = 1;
    }.into();

    println!("new_item: \"{}\"", new_item.to_string());

    new_item
}


