use proc_macro::TokenStream;
use quote::quote;
use proc_macro2::{Ident, Span};
use syn::{parse_macro_input, ItemStruct, LitStr};
use std::sync::{LazyLock, Arc, Mutex};

struct ShaderInfo
{
    name: String,
}

static REGISTERED_SHADERS: LazyLock<Arc<Mutex<Vec<ShaderInfo>>>> = LazyLock::new(||{ Arc::new(Mutex::new(Vec::new())) });

#[proc_macro_attribute]
pub fn register_shader(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(item as ItemStruct);
    let args = parse_macro_input!(attr as LitStr);

    let struct_name = input_struct.ident.to_string();
    let ident_token = Ident::new(&struct_name, Span::call_site());
    let name = args.value();

    let mut shaders = REGISTERED_SHADERS.lock().unwrap();

    let id = shaders.len();
    shaders.push(ShaderInfo {name: struct_name.clone() });

    let output_tokens = quote! {
        #input_struct

        impl #ident_token {
            pub const NAME: &str  = #name;
            pub const ID  : usize = #id;
        }
    };

    output_tokens.into()
}

#[proc_macro_attribute]
pub fn shaders_registry(_attr: TokenStream, item: TokenStream) -> TokenStream {

    let input_struct = parse_macro_input!(item as ItemStruct);
    let struct_name = input_struct.ident.to_string();
    let ident_token = Ident::new(&struct_name, Span::call_site());

    let shape_names = REGISTERED_SHADERS.lock().unwrap();
    let num_shaders = shape_names.len();
    let generate_code = shape_names.iter().map(|shape_info| {
        let ident_token = Ident::new(&shape_info.name, Span::call_site());
        quote! {
            (#ident_token::NAME, #ident_token::ID, #ident_token::pipeline_descriptor as fn() -> PipelineDescriptor, #ident_token::GLOBAL_UNIFORMS),
        }
    });

    let output = quote! {
        #input_struct

        impl #ident_token {
            pub const SHADER_DETAILS: [(&str, usize, fn() -> PipelineDescriptor, bool); #num_shaders] = [ #(#generate_code)* ];
        }
    };

    println!("OUTPUT: {:}", output);

    output.into()
}


#[proc_macro]
pub fn shaders_generate_registry(_input: TokenStream) -> TokenStream {
    let shape_names = REGISTERED_SHADERS.lock().unwrap();

    let generated_code = shape_names.iter().map(|shape_info| {
        let ident_token = Ident::new(&shape_info.name, Span::call_site());
        quote! {
            let name = #ident_token::NAME;
            let id   = #ident_token::ID;
            let pipe = #ident_token::pipeline_descriptor();

            names.push(name);
            ids.push(id);
            pipes.push(pipe);
        }
    });

    let output = quote! {
        pub fn process_all_shaders() -> (Vec<&'static str>, Vec<usize>){
            let mut names = Vec::new();
            let mut ids   = Vec::new();
            let mut pipes = Vec::new();
            // loop over generated code and running a substitution
            #(#generated_code)* (names, ids)
        }
    };

    output.into()
}
