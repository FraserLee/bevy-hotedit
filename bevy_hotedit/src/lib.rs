#![feature(proc_macro_span)]

use proc_macro::{TokenStream, Span};
use proc_macro2::Literal;

use quote::quote;
use syn::{ self, parse_macro_input };

use toml::Value;







#[proc_macro_attribute]
pub fn hot(args: TokenStream, item: TokenStream) -> TokenStream {
    
    // read the file "hotedit-values.toml" from the "src/" directory of 
    // project using this macro, parse it into a toml from which to extract
    // values.
    let path = Span::call_site().source_file().path();
    let path = path.to_str().unwrap();
    let path = path.split("src/").next().unwrap();
    let path = format!("{}src/hotedit-values.toml", path);
    
    let file = std::fs::read_to_string(&path).unwrap();

    let toml = file.parse::<Value>().unwrap();
    let toml = toml.as_table().unwrap();
    

    // we don't need any macro arguments at the moment, but here's how to get them.
    let _args = parse_macro_input!(args as syn::AttributeArgs);

    // try to parse our line as either syn::TraitItemConst or syn::ItemConst
    // to extract stuff as
    //     const NAME: ty;
    // or
    //     const NAME: ty = value;
    //

    let trait_parse_r = syn::parse::<syn::TraitItemConst>(item.clone());
    let full_parse_r = syn::parse::<syn::ItemConst>(item.clone());

    let (name, ty, value) = if let Ok(tree) = full_parse_r {
        (tree.ident, *tree.ty, Some(tree.expr))
    } else if let Ok(tree) = trait_parse_r {
        (tree.ident, tree.ty, None)
    } else {
        panic!("Could not parse {} as either syn::TraitItemConst or syn::ItemConst", item.to_string());
    };



    
    // if the key isn't in the toml, either 
    // - A: insert whatever we find in the value field of the canst to initialize it.
    // - B: panic.
    if !toml.contains_key(&name.to_string()) {
        if let Some(_) = value {
            // There's probably a way to extract a string-version from value, 
            // but this seems good enough for the moment. Sorry.
            std::fs::write(&path, 
                format!("{}\n{} = {}", 
                    file, 
                    name, 
                    item.to_string().split("=").skip(1).next().unwrap().split(";").next().unwrap().trim()
                )).unwrap();
            return item;
        }

        panic!("key \"{}\" not found in toml file", name);
    }


    // at this point, we can just grab the key from the toml and use it to 
    // initialize the const.

    let new_item: TokenStream = match &toml[&name.to_string()] {
        Value::Integer(i) => { 
            let i_unsuffixed = Literal::i64_unsuffixed(*i);
            quote! { const #name : #ty = #i_unsuffixed; }.into()
        }
        Value::Float(f) => {
            let f_unsuffixed = Literal::f64_unsuffixed(*f);
            quote! { const #name : #ty = #f_unsuffixed; }.into()
        }
        _ => panic!("unsupported type")
    };

    println!("new_item: \"{}\"", new_item.to_string());

    new_item

}










