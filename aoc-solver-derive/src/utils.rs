use core::{Puzzle, PuzzlePart};

use proc_macro as pm;
use syn::{Item, ItemFn, Pat, Type, TypePath, Visibility};

pub fn validate_and_get_item_fn(item: pm::TokenStream) -> ItemFn {
    match syn::parse2::<Item>(item.into()) {
        Ok(item) => match item {
            Item::Fn(item_fn) => item_fn,
            _ => panic!("This macro can only be used on functions."),
        },
        Err(_) => panic!("This macro can only be used on functions."),
    }
}

pub fn validate_fn_params(item_fn: &ItemFn) {
    const ARGUMENT_NAME_FORMAT_ERROR: &str =
        "Argument must be named: `input`. Expecting: `input: &str`.";
    const ARGUMENT_TYPE_FORMAT_ERROR: &str =
        "Argument type must be `&str`. Expecting: `input: &str`.";

    if item_fn.sig.inputs.len() != 1 {
        panic!("Function may only have one argument. Expecting: `input: &str`.")
    }

    let argument = match &item_fn.sig.inputs[0] {
        syn::FnArg::Typed(arg) => arg,
        _ => panic!("Failed to detect function argument. Expecting: `input: &str`."),
    };

    let argument_name = match &*argument.pat {
        Pat::Ident(ident) => &ident.ident,
        _ => panic!("{}", ARGUMENT_NAME_FORMAT_ERROR),
    };

    if argument_name != "input" {
        panic!("{}", ARGUMENT_NAME_FORMAT_ERROR);
    }

    let reference = match &*argument.ty {
        Type::Reference(r) => r,
        _ => panic!("{}", ARGUMENT_TYPE_FORMAT_ERROR),
    };

    let path = match &*reference.elem {
        Type::Path(TypePath { path, .. }) => path,
        _ => panic!("{}", ARGUMENT_TYPE_FORMAT_ERROR),
    };

    let arg_type = match path.segments.first() {
        Some(segment) => &segment.ident,
        _ => panic!("{}", ARGUMENT_TYPE_FORMAT_ERROR),
    };

    if arg_type != "str" {
        panic!("{}", ARGUMENT_TYPE_FORMAT_ERROR);
    }
}

pub fn validate_fn_output(item_fn: &ItemFn) {
    const RETURN_TYPE_ERROR: &str = "Function must return type: `String`.";

    let output = match &item_fn.sig.output {
        syn::ReturnType::Type(_, ty) => ty,
        _ => panic!("{}", RETURN_TYPE_ERROR),
    };

    let path = match &**output {
        Type::Path(TypePath { path, .. }) => path,
        _ => panic!("{}", RETURN_TYPE_ERROR),
    };

    if path.segments.len() > 1 {
        panic!("{}", RETURN_TYPE_ERROR);
    }

    match path.segments.first() {
        Some(segment) if &segment.ident == "String" => {}
        _ => panic!("{}", RETURN_TYPE_ERROR),
    };
}

pub fn validate_fn_visibility(item_fn: &ItemFn) {
    if let Visibility::Public(_) = &item_fn.vis {
        panic!("Function must not be public.")
    }
}

pub fn validate_and_extract_macro_attributes(args: pm::TokenStream) -> (Puzzle, PuzzlePart) {
    let mut idents = args.into_iter().filter_map(|a| {
        if let pm::TokenTree::Literal(_) = a {
            Some(a.into())
        } else {
            None
        }
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

    let part: pm::TokenStream = idents.next().expect("Couldn't find part");
    let part: syn::LitInt = syn::parse(part).expect("failed to parse part");
    let part: u8 = part
        .base10_parse::<u8>()
        .expect("failed to parse part into u8");
    // if part < 1 || part > 2 {
    //     panic!("part can only be 1 or 2.")
    // }

    (Puzzle::new(&year, &day), PuzzlePart::new(part))
}
