use proc_macro as pm;
use quote::quote;

#[proc_macro]
pub fn aoc_solver(args: pm::TokenStream) -> pm::TokenStream {
    let mut idents = args.into_iter().filter_map(|a| match a {
        pm::TokenTree::Literal(_) => Some(a.into()),
        pm::TokenTree::Ident(_) => Some(a.into()),
        _ => None,
    });

    let year: pm::TokenStream = idents.next().expect("Couldn't find year");
    let year: syn::LitInt = syn::parse(year).expect("failed to parse year");
    let year: u16 = year
        .base10_parse::<u16>()
        .expect("failed to parse year into u16");

    let day: pm::TokenStream = idents.next().expect("Couldn't find day");
    let day: syn::LitInt = syn::parse(day).expect("failed to parse day");
    let day: u8 = day
        .base10_parse::<u8>()
        .expect("failed to parse day into u8");

    let part_one_func: pm::TokenStream = idents
        .next()
        .expect("Couldn't find function name for part one solver.");
    let part_one_func: syn::Ident =
        syn::parse(part_one_func).expect("Couldn't parse function name into syn::Ident.");

    let part_two_func: pm::TokenStream = idents
        .next()
        .expect("Couldn't find function name for part two solver.");
    let part_two_func: syn::Ident =
        syn::parse(part_two_func).expect("Couldn't parse function name into syn::Ident.");

    if let Some(_) = idents.next() {
        panic!("Too many arguments passed");
    }

    quote! {
        fn main() {
            core::runner::runner(#year, #day, #part_one_func, #part_two_func);
        }
    }
    .into()
}
