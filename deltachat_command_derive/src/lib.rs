#![recursion_limit = "128"]
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn;

#[proc_macro]
pub fn get_args_struct(input: TokenStream) -> TokenStream {
    let fn_name: Ident = syn::parse(input).unwrap();
    let new_ident = Ident::new(&format!("cmd_{}_args", fn_name), Span::call_site());
    let r = proc_macro2::Group::new(
        proc_macro2::Delimiter::None,
        proc_macro2::TokenStream::from(quote! {#new_ident}),
    );
    TokenStream::from(quote! {#r})
}

#[proc_macro]
pub fn api_function(input: TokenStream) -> TokenStream {
    let ast: syn::ItemFn = syn::parse(input).unwrap();
    let fn_name = &ast.sig.ident;

    let fn_inputs = &ast.sig.inputs;
    let arguments_ident = Ident::new(&format!("cmd_{}_args", fn_name), Span::call_site());
    let result_ident = Ident::new(&format!("cmd_{}_res", fn_name), Span::call_site());
    let fn_output = &ast.sig.output;
    let fn_block = &ast.block;

    let fn_generics = &ast.sig.generics; // the lifetimes

    let mut argument_idents: Vec<proc_macro2::TokenStream> = Vec::new();

    for input in fn_inputs {
        match input {
            syn::FnArg::Typed(pat_type) => match &*pat_type.pat {
                syn::Pat::Ident(p) => {
                    let ident = p.ident.clone();
                    argument_idents.push(quote! {
                        let #ident = args.#ident;
                    })
                }
                _ => {}
            },
            _ => {}
        }
    }

    let argument_assigning = argument_idents
        .into_iter()
        .collect::<proc_macro2::TokenStream>();

    let result_struct = match fn_output {
        syn::ReturnType::Default => quote! {
            struct #result_ident #fn_generics{
                invocation_id: u32,
            }
        },
        syn::ReturnType::Type(_, type_box) => {
            quote! {
                struct #result_ident #fn_generics{
                    result: #type_box,
                    invocation_id: u32,
                }
            }
        }
    };

    let result = quote! {
        #[derive(Deserialize, Debug)]
        struct #arguments_ident #fn_generics{
            #fn_inputs
        }
        #[derive(Serialize, Debug)]
        #result_struct

        fn #fn_name(args: #arguments_ident, invocation_id: u32) -> #result_ident {
            #result_ident {
                result: {
                    #argument_assigning
                    #fn_block
                },
                invocation_id: invocation_id,
            }
        }
    };

    TokenStream::from(result)
}

#[proc_macro]
pub fn api_function2(input: TokenStream) -> TokenStream {
    let ast: syn::ItemFn = syn::parse(input).unwrap();
    let fn_name = &ast.sig.ident;

    let fn_inputs = &ast.sig.inputs;
    let arguments_ident = Ident::new(&format!("cmd_{}_args", fn_name), Span::call_site());
    let result_ident = Ident::new(&format!("cmd_{}_res", fn_name), Span::call_site());
    let fn_output = &ast.sig.output;
    let fn_block = &ast.block;

    let fn_generics = &ast.sig.generics; // the lifetimes

    let mut argument_idents: Vec<proc_macro2::TokenStream> = Vec::new();

    for input in fn_inputs {
        match input {
            syn::FnArg::Typed(pat_type) => match &*pat_type.pat {
                syn::Pat::Ident(p) => {
                    let ident = p.ident.clone();
                    argument_idents.push(quote! {
                        let #ident = args.#ident;
                    })
                }
                _ => {}
            },
            _ => {}
        }
    }

    let argument_assigning = argument_idents
        .into_iter()
        .collect::<proc_macro2::TokenStream>();

    let result_struct = match fn_output {
        syn::ReturnType::Default => quote! {
            struct #result_ident #fn_generics{
                invocation_id: u32,
            }
        },
        syn::ReturnType::Type(_, type_box) => {
            quote! {
                struct #result_ident #fn_generics{
                    result: #type_box,
                    invocation_id: u32,
                }
            }
        }
    };

    let result = quote! {
        #[derive(Deserialize, Debug)]
        struct #arguments_ident #fn_generics{
            #fn_inputs
        }
        #[derive(Serialize, Debug)]
        #result_struct

        fn #fn_name(args: #arguments_ident, invocation_id: u32, account: &Account) -> #result_ident {
            #result_ident {
                result: {
                    #argument_assigning
                    #fn_block
                },
                invocation_id: invocation_id,
            }
        }
    };

    TokenStream::from(result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
