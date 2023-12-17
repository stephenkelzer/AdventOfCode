mod utils;

use once_cell::sync::Lazy;
use proc_macro as pm;
use quote::quote;
use std::{collections::HashSet, sync::Mutex};
use syn::{parse_macro_input, Item};
use utils::{
    extract_macro_attributes, update_fn_name, validate_fn_output, validate_fn_params,
    validate_fn_visibility,
};

static USED_VALUES: Lazy<Mutex<HashSet<(u16, u8, u8)>>> = Lazy::new(|| Mutex::new(HashSet::new()));

/// Used to tag a function as a "solver" for a given Advent of Code puzzle.
#[proc_macro_attribute]
pub fn aoc_solver(args: pm::TokenStream, item: pm::TokenStream) -> pm::TokenStream {
    let mut item_fn = match parse_macro_input!(item) {
        Item::Fn(func_input) => func_input,
        _ => panic!("This macro can only be used on functions."),
    };

    let (year, day, part) = extract_macro_attributes(args);

    match USED_VALUES.lock() {
        Ok(values) if values.contains(&(year, day, part)) => {
            // This error message isn't the best.. This indicates that the lock on the mutex cannot be acquired, because it is
            // being block by another request. For now this error will be fine, but finding a better way to do this would
            // be nice.
            panic!(
                "Value '{:?}' has already been used in a previous invocation of the macro",
                (year, day, part)
            );
        }
        Ok(mut values) => {
            values.insert((year, day, part).clone());
        }
        Err(e) => {
            println!("{:?}", e);
            panic!(
                "Value '{:?}' has already been used in a previous invocation of the macro",
                (year, day, part)
            )
        }
    }

    validate_fn_params(&item_fn);
    validate_fn_output(&item_fn);
    validate_fn_visibility(&item_fn);

    update_fn_name(&mut item_fn, year, day, part);

    let fn_name = &item_fn.sig.ident;
    let fn_params = &item_fn.sig.inputs;
    let fn_output = &item_fn.sig.output;
    let fn_body = &item_fn.block;

    quote! {
        #[allow(unused)]
        fn #fn_name(#fn_params) #fn_output {
            println!("Executing func: {}", stringify!(#fn_name));
            #fn_body
        }
    }
    .into()
}
