mod registry;
mod utils;

use proc_macro as pm;
use proc_macro::{self, TokenStream};
use quote::quote;
use registry::{get_registery_count, register_if_unique};
use utils::{
    extract_macro_attributes, update_fn_name, validate_and_get_item_fn, validate_fn_output,
    validate_fn_params, validate_fn_visibility,
};

/// Used to tag a function as a "solver" for a given Advent of Code puzzle.
#[proc_macro_attribute]
pub fn aoc_solver(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_fn = validate_and_get_item_fn(item.into());

    let (year, day, part) = extract_macro_attributes(args);

    register_if_unique((year, day, part));

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

/// get the count of aoc_solver macros registered in this module
#[proc_macro]
pub fn get_count(_item: pm::TokenStream) -> pm::TokenStream {
    let val = get_registery_count();
    quote! {#val}.into()
}
