#![feature(proc_macro_span)]

use proc_macro::{ TokenStream, Span };
use proc_macro2::Literal;

use quote::quote;
use syn::{ self, parse_macro_input };

use toml::{ self, Value, value::Table };




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

    let mut file_t: Table = toml::from_str(&file).unwrap();

    
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


    let (value_debug, value_release, iden, ty) = if let Ok(s) = full_parse_r { 
        // const NAME: ty = value;
        
        // - insert the value into the toml, return the TokenStream unchanged

        file_t.insert(
            s.ident.to_string(),
            parse_value(&item.to_string())
        );
        
        std::fs::write(&path, toml::to_string_pretty(&file_t).unwrap()).unwrap();

        let value = s.expr;
        let iden = s.ident;
        let ty = s.ty;

        (quote!{#value}, quote!{#value}, quote!{#iden}, quote!{#ty})

    } else if let Ok(s) = trait_parse_r { // const NAME: ty;

        // get the value from the toml, return a modified TokenStream with the value inserted.

        let (iden, name, ty) = (&s.ident, s.ident.to_string(), s.ty);
        let mut ty = quote!(#ty);
        if ty.to_string() == "& str" { ty = quote!(String); }

        // give a half-answer if we don't have a value for this const
        // (maybe a better solution is possible, but this state shouldn't 
        // really happen unless the user just forgot, or in testing).
        if !file_t.contains_key(&name) {
            eprintln!("ERROR: key \"{}\" not found in toml file, omitting compiled const.", name);
            return quote!{
                #[inline]
                #[allow(non_snake_case)]
                fn #iden() -> #ty {
                    unimplemented!("{} not found in toml file", #name);
                }
            }.into();
        }

        // otherwise, get it from the toml and convert it into a literal
        let (value, conversion) = match &file_t[&name] {

            Value::Integer(i) => { 
                let l = Literal::i64_unsuffixed(*i);
                (quote!(#l), quote!(.as_integer().unwrap() as #ty))
            }

            Value::Float(f) => {
                let l = Literal::f64_unsuffixed(*f);
                (quote!(#l), quote!(.as_float().unwrap() as #ty))
            }

            Value::Boolean(b) => (quote!{#b}, quote!(.as_bool().unwrap() as #ty)),

            Value::String(s) => {
                (quote!{#s.to_string()}, quote!(.as_str().unwrap().to_string()))
            }

            _ => panic!("unsupported value \"{:?}\" for const \"{}\"", file_t[&name], name)
        };
        

        (quote!{lookup(#name) #conversion}, quote!{#value}, quote!{#iden}, quote!{#ty})
    } else {
        panic!("Could not parse {} as either syn::TraitItemConst or syn::ItemConst", item.to_string());
    };


    let new_item: TokenStream = quote! { 
        #[inline]
        #[allow(non_snake_case)]
        fn #iden() -> #ty {
            // either return the const value (release build) 
            // or look it up from the toml (debug build)
            if !cfg!(debug_assertions) { #value_release } else { #value_debug }
        }
    }.into();

    // println!("new_item: \"{}\"", new_item.to_string());

    return new_item;

}


// dumb trick, this won't work soon. No way to do enums or anything cool.
// Still it'll work for the moment.
fn parse_value(line: &str) -> Value {
    let value = line.split("=").skip(1).next().unwrap().split(";").next().unwrap().trim();
    format!("test = {}\n", value).parse::<Value>().unwrap()["test"].clone()
}

