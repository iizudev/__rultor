//! SPDX-License-Identifier: MIT OR Apache-2.0
#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// Derive macro implementation for `InputAction` trait
#[proc_macro_derive(InputAction)]
pub fn input_action_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        #[automatically_derived]
        impl #impl_generics ::bevy_actify::InputAction for #name #ty_generics #where_clause {}
    };

    TokenStream::from(expanded)
}
