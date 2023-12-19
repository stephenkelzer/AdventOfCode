mod main_processor;
mod registry;
mod utils;

use crate::main_processor::processor;
use proc_macro as pm;
use proc_macro::{self, TokenStream};
use quote::quote;
use registry::{get_registery_count, register_if_unique};
use utils::{
    update_fn_name, validate_and_extract_macro_attributes, validate_and_get_item_fn,
    validate_fn_output, validate_fn_params, validate_fn_visibility,
};

// NOTE: would be really cool to adjust this when [https://github.com/rust-lang/rust/issues/54725] is solved.
//       could utilize the file path to determine the year and day instead of requiring them as arguments.

/// Used to tag a function as a "solver" for a given Advent of Code puzzle.
#[proc_macro_attribute]
pub fn aoc_solver(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_fn = validate_and_get_item_fn(item.into());

    let (year, day, part) = validate_and_extract_macro_attributes(args);

    let should_render_main = register_if_unique((year, day, part));

    validate_fn_params(&item_fn);
    validate_fn_output(&item_fn);
    validate_fn_visibility(&item_fn);

    update_fn_name(&mut item_fn, year, day, part);

    let fn_name = &item_fn.sig.ident;
    let fn_params = &item_fn.sig.inputs;
    let fn_output = &item_fn.sig.output;
    let fn_body = &item_fn.block;

    let main_fn_name = quote::format_ident!("main");

    let main_fn = match should_render_main {
        true => Some(quote! {
            fn main() {
                let args: Vec<String> = std::env::args().collect();
                dbg!(args);

                // crate::main_processor::processor();

                // let handler = std::thread::spawn(|| async {
                //     println!("wut")
                // });

                // handler.join.unwrap();

                // (async || { println!("wut") })();

                // let test = || {
                //     println!("yo")
                // };
                // test();

                println!("main function!")
            }
        }),
        false => None,
    };

    quote! {

        #[allow(unused)]
        fn #fn_name(#fn_params) #fn_output {
            println!("Executing func: {}", stringify!(#fn_name));
            #fn_body
        }

        #main_fn
    }
    .into()
}

/// get the count of aoc_solver macros registered in this module
#[proc_macro]
pub fn get_count(_item: pm::TokenStream) -> pm::TokenStream {
    let val = get_registery_count();
    quote! {#val}.into()
}
