//! Procedural macros for the Starterm project.
//!
//! NOTE: This file would live in its own crate (`starterm-macros`) within
//! the cargo workspace and be a `proc-macro = true` type crate.

// The following dependencies would be in the `starterm-macros/Cargo.toml`:
// proc-macro2 = "1.0"
// quote = "1.0"
// syn = "2.0"
// include_dir = "0.7"

// Placeholder for the actual macro implementation.
// The real implementation would parse arguments and use `include_dir`
// to generate code that embeds a directory's contents into the binary.

/*
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn embed_assets(input: TokenStream) -> TokenStream {
    let path = parse_macro_input!(input as LitStr);
    let dir_path = path.value();

    // Use a library like `include_dir` to handle the embedding.
    let expanded = quote! {
        {
            // This is a simplified representation.
            // `include_dir!` macro would generate a static Dir instance.
            static ASSETS: include_dir::Dir<'_> = include_dir::include_dir!($DIR_PATH);
            &ASSETS
        }
    };

    TokenStream::from(expanded)
}
*/ 