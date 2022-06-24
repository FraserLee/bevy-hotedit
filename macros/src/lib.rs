#![feature(proc_macro_span)]

use proc_macro::{ TokenStream, Span };
use quote::{ quote, format_ident };
use regex::Regex;

use bevy_hotedit_util as util;

#[proc_macro_attribute]
pub fn hot(_args: TokenStream, item: TokenStream) -> TokenStream {

    // find the path to the file "hotedit-values.toml" from the "src/" 
    // directory of the project using this macro.

    let line_num = Span::call_site().start().line;

    let values_path = format!("{}/src/hotedit-values.toml", 
        std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let debug_path = format!("{}debug.toml", 
        std::file!().replace("macros/src/lib.rs", ""));
    

    // step 1: parse the line to pull out the const name and type

    
    // try to parse our line as either syn::TraitItemConst or syn::ItemConst
    // to extract stuff as
    //     const NAME: ty;
    // or
    //     const NAME: ty = value;
    //

    let trait_parse_r = syn::parse::<syn::TraitItemConst>(item.clone());
    let full_parse_r = syn::parse::<syn::ItemConst>(item.clone());


    let (ident, ty) = if let Ok(s) = full_parse_r { // const NAME: ty = value;

        // step 2: if the expression is specified with an =, parse
        // that to a variable and write it to the toml file.

        let ident = s.ident;
        let ty = *s.ty;

        let value = s.expr;
        if let syn::Expr::Lit(l) = *value {
            let value = match l.lit {
                syn::Lit::Str(s) => format!("\"{}\"", s.value()),
                syn::Lit::Int(i) => i.base10_digits().to_string(),
                syn::Lit::Float(f) => f.base10_digits().to_string(),
                syn::Lit::Bool(b) => b.value().to_string(),
                _ => panic!("unsupported literal type for const \"{}\"", ident)
            };
            util::write_to_file(&ident.to_string(), &value, &values_path);
        }

        (quote!{#ident}, quote!{#ty})

    } else if let Ok(s) = trait_parse_r {          // const NAME: ty;
        let ident = s.ident;
        let ty = s.ty;
        
        (quote!{#ident}, quote!{#ty})
    } else {
        panic!("Couldn't parse line below #[hot] macro. Make sure your syntax \
                looks like\
                \n\tconst NAME: ty;\
                \nor\
                \n\tconst NAME: ty = value;\n\n\
                (yours was: {})", item.to_string());
    };

    let ty = if ty.to_string() == "& str" { quote!(String) } else { ty };

    // step 3: generate a default value for the const

    let re_int_type = Regex::new(r"^[iu]([0-9]+|size)$").unwrap();
    let re_float_type = Regex::new(r"^f[0-9]$").unwrap();
    let re_bool_type = Regex::new(r"^bool$").unwrap();

    let (mut v_init, v_type) = if re_int_type.is_match(&ty.to_string()) {
        (quote!{ ::bevy_hotedit::Value::Int(0) }, "Int")
    } else if re_float_type.is_match(&ty.to_string()) {
        (quote!{ ::bevy_hotedit::Value::Float(0.0) }, "Float")
    } else if re_bool_type.is_match(&ty.to_string()) {
        (quote!{ ::bevy_hotedit::Value::Boolean(false) }, "Boolean")
    } else {
        (quote!{ ::bevy_hotedit::Value::String("".to_string()) }, "String")
    };


    // step 4: register the const (along with contextual info) in the 
    // hotedit-debug.toml file, so we can build the webpage.


    util::write_to_file(
        &format!("{}.type", ident.to_string()),
        &format!("\"{}\"", v_type),
        &debug_path
    );


    // step 5: lookup the value from the toml file, so we can auto-return that
    // if we're in release mode. Write a panic into the macro if it's not found
    // (but don't panic on compile).

    let ident_str = ident.to_string();

    let release_value = match util::lookup_from_file(&ident.to_string(), &values_path) {
        Some(v) => {
            let (v, conversion) = match v {
                util::Value::Int(i) => { (quote!{ #i }, quote!{ #i as #ty }) }
                util::Value::Float(f) => { (quote!{ #f }, quote!{ #f as #ty }) }
                util::Value::Boolean(b) => { (quote!{ #b }, quote!{ #b as #ty }) }
                util::Value::String(s) => { (quote!{ #s }, quote!{ #s.to_string() }) }
            };

            v_init = quote!{ ::bevy_hotedit::Value::from(#v) };

            conversion
        }
        None => quote!{
            panic!("{} not found in toml file", #ident_str);
        }
    };


    // step 6: return a function with the debug / release switch and the value.

    let registered_bool = format_ident!("{}_REGISTERED", ident.to_string());

    let new_item: TokenStream = quote! { 
        static mut #registered_bool: bool = false;
        #[inline]
        #[allow(non_snake_case)]
        fn #ident() -> #ty {
            // either return the const value (release build) 
            // or look it up from the toml (debug build)
            if cfg!(debug_assertions) { 
                unsafe { // maybe look into some way to avoid unsafe later?
                         // As far as they go, it's a pretty safe unsafe. Still,
                         // given how it's completely avoidable, might be nice
                         // to have the library completely safe.
                    if !#registered_bool {
                        #registered_bool = true;

                        ::bevy_hotedit::HotVariable {
                            name: #ident_str.to_string(),
                            line_num: #line_num,
                            value: #v_init,
                        }.register();
                    }
                }

                #ty::from( ::bevy_hotedit::lookup(#ident_str).unwrap() )
            } else {
                #release_value
            }
        }
    }.into();

    return new_item;
}

