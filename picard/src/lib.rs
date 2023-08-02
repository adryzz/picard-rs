#![feature(proc_macro_span)]

use std::path::PathBuf;

use proc_macro::{TokenStream};
use proc_macro::Span;
use quote::quote;
use syn::{parse_macro_input, LitStr, spanned::Spanned};


/// Compiles a shader file, and includes the binary as a byte array.
#[proc_macro]
pub fn shader(input: TokenStream) -> TokenStream {
    shader_macro(input)
}

fn shader_macro(input: TokenStream) -> TokenStream {
    let filename = parse_macro_input!(input as LitStr).value();

    let path = get_shader_path(&filename);

    let file = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            return syn::Error::new(filename.span(), format!("Error while opening shader file: {}.", e))
            .to_compile_error()
            .into();
        }
    };

    let mut assembler = picard_core::ShaderAssembler::new();

    let shader = match assembler.assemble_shader(&file) {
        Ok(s) => s.into_vec(),
        Err(e) => {
            return syn::Error::new(filename.span(), format!("Error while compiling shader: {}.", e))
            .to_compile_error()
            .into();
        }
    };

    let out = quote! {
        [#(#shader),*]
    };

    out.into()
}

fn get_shader_path(filename: &str) -> PathBuf {
    let file = Span::call_site().source_file();

    // This is to make it behave kinda like include_bytes! with the path
    file.path().with_file_name(filename)
}


/// Compiles an inline shader, and includes the binary as a byte array.
#[proc_macro]
pub fn pica(input: TokenStream) -> TokenStream {
    pica_macro(input)
}

fn pica_macro(input: TokenStream) -> TokenStream {
    let file = parse_macro_input!(input as LitStr).value();

    let mut assembler = picard_core::ShaderAssembler::new();
    let shader = match assembler.assemble_shader(&file) {
        Ok(s) => s.into_vec(),
        Err(e) => {
            return syn::Error::new(file.span(), format!("Error while compiling shader: {}.", e))
            .to_compile_error()
            .into();
        }
    };

    let out = quote! {
        [#(#shader),*]
    };

    out.into()
}