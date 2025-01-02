use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};
use quote::quote;

#[proc_macro_attribute]
pub fn has_permit(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let func_vis = &func.vis;
    let func_block = &func.block;
    let func_decl = &func.sig;
    let func_name = &func_decl.ident;
    let func_asyncness = &func_decl.asyncness;
    let func_generics = &func_decl.generics;
    let func_inputs = &func_decl.inputs;
    let func_output = &func_decl.output;

    let s = attr.to_string();

    let expanded = quote! {
        #func_vis #func_asyncness fn #func_name #func_generics(req_in_permit:HttpRequest,#func_inputs) #func_output{
            match crate::token_auth::check_permit(req_in_permit, #s).await {
                 None =>  #func_block
                 Some(res) => { return res.resp_json(); }
            }
        }
    };

    expanded.into()
} 