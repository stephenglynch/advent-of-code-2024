#![feature(proc_macro_span)]
extern crate proc_macro;

use proc_macro::{Span, TokenStream};
use quote::quote;
use std::fs;

#[proc_macro]
pub fn count_lines(input: TokenStream) -> TokenStream {
    let file_path = input.to_string().replace("\"", "");
    let source = Span::call_site().source_file();
    let path = source.path();
    let source_dir_str = path.parent().unwrap().as_os_str().to_str().unwrap();
    let full_file_path = format!("{source_dir_str}/{file_path}");
    let file_content = fs::read_to_string(full_file_path).unwrap();

    let line_count = file_content.lines().count();
    let output = quote! {
        #line_count
    };

    output.into()
}
