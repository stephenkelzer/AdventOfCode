mod registry;
mod utils;

use proc_macro::TokenStream;
use quote::quote;
use registry::register;
use utils::{
    validate_and_extract_macro_attributes, validate_and_get_item_fn, validate_fn_output,
    validate_fn_params, validate_fn_visibility,
};

// NOTE: would be really cool to adjust this when [https://github.com/rust-lang/rust/issues/54725] is solved.
//       could utilize the file path to determine the year and day instead of requiring them as arguments.

/// Used to tag a function as a "solver" for a given Advent of Code puzzle.
#[proc_macro_attribute]
pub fn aoc_solver(args: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = validate_and_get_item_fn(item);

    let (puzzle, part) = validate_and_extract_macro_attributes(args);

    validate_fn_params(&item_fn);
    validate_fn_output(&item_fn);
    validate_fn_visibility(&item_fn);

    let fn_name = quote::format_ident!("{}", &puzzle.get_function_name(&part));
    let fn_params = &item_fn.sig.inputs;
    let fn_output = &item_fn.sig.output;
    let fn_body = &item_fn.block;

    let should_render_main = register(&puzzle, &part);

    let main_fn = match should_render_main {
        true => {
            let year = puzzle.get_year();
            let day = puzzle.get_day();
            let part_one_func_name =
                quote::format_ident!("{}", puzzle.get_function_name(&core::PuzzlePart::One));
            let part_two_func_name =
                quote::format_ident!("{}", puzzle.get_function_name(&core::PuzzlePart::Two));

            Some(quote! {
                fn main() {
                    core::runner::runner(#year, #day, #part_one_func_name, #part_two_func_name);
                }
            })
        }
        false => None,
    };

    quote! {
        #[allow(unused)]
        fn #fn_name(#fn_params) #fn_output {
            #fn_body
        }

        #main_fn
    }
    .into()
}
