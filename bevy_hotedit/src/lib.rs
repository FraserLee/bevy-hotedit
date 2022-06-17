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

    let mut toml = file.parse::<Value>().unwrap();
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

    if let Ok(s) = full_parse_r { // const NAME: ty = value;
        
        // - insert the value into the toml, return the TokenStream unchanged

        // There's probably a way to extract a string-version from value, 
        // but this seems good enough for the moment. Sorry.
        toml[&s.ident.to_string()] = Value::String(
            item.to_string().split("=").skip(1).next().unwrap().split(";").next().unwrap().trim().to_string()
        );
        
        std::fs::write(&path, toml::to_string_pretty(&toml).unwrap()).unwrap();


        return item;

    } else if let Ok(s) = trait_parse_r { // const NAME: ty;

        // get the value from the toml, return a modified TokenStream with the value inserted.

        // if the key isn't in the toml, either 
        let (name, ty) = (s.ident.to_string(), s.ty);
        if !toml.contains_key(&name) { panic!("key \"{}\" not found in toml file", name); }

        let new_item: TokenStream = match &toml[&name] {

            Value::Integer(i) => { 
                let i_unsuffixed = Literal::i64_unsuffixed(*i);
                quote! { const #name : #ty = #i_unsuffixed; }.into()
            }

            Value::Float(f) => {
                let f_unsuffixed = Literal::f64_unsuffixed(*f);
                quote! { const #name : #ty = #f_unsuffixed; }.into()
            }

            // todo: write tests
            Value::String(s) => { quote! { const #name : #ty = #s; }.into() },
            Value::Boolean(b) => { quote! { const #name : #ty = #b; }.into() },

            _ => panic!("unsupported type")
        };

        // println!("new_item: \"{}\"", new_item.to_string());

        return new_item;
    }

    panic!("Could not parse {} as either syn::TraitItemConst or syn::ItemConst", item.to_string());

}










