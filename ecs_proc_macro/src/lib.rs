use syn::{parse_macro_input, Ident, DeriveInput};
use quote::quote;

#[proc_macro_derive(Component, attributes(component))]
pub fn component_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let gen = quote! {
        impl crate::storage::Component for #name {
            fn get(&self) -> &dyn std::any::Any {
                self as &dyn std::any::Any
            }

            fn get_mut(&mut self) -> &mut dyn std::any::Any {
                self as &mut dyn std::any::Any
            }

            fn get_type_id(&self) -> std::any::TypeId {
                std::any::TypeId::of::<#name>()
            }
        }
    };
    gen.into()
}


// Define a macro for systems. It should be used like this:
//
// ```
// #[macro_use]
// extern crate ecs_proc_macro;
//
// #[system]
// fn my_system(world: &mut World) {
//     // Do stuff
// }
// ```
//
// The macro will generate a struct with the name of the system and a function
// with the name of update
#[proc_macro_attribute]
pub fn system(args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _ = parse_macro_input!(args as syn::AttributeArgs);
    let input = parse_macro_input!(input as syn::ItemFn);

    let name = input.sig.ident;

    // Convert the name to camel case, and add the prefix "System"
    let system_name = Ident::new(&format!("System{}", string_to_camel_case(name.to_string())), name.span());

    // Extract the function body
    let body = match input.sig.output {
        syn::ReturnType::Default => {
            let body = input.block.clone();
            quote! {
                #body
            }
        },
        _ => {
            let body = input.block.clone();
            quote! {
                #body
                return;
            }
        }
    };


    let struct_name = quote! { 
        /// An automatically generated struct for the system.       
        pub struct #system_name;
    };

    // add the update function
    let update_func = quote! {
        impl crate::system::System for #system_name{
            fn update(&mut self, world: std::sync::Arc<std::sync::Mutex<&mut crate::world::World>>) {
                // Auto-generated code from #name
                #body
            }

            fn get_type_id(&self) -> std::any::TypeId {
                std::any::TypeId::of::<#system_name>()
            }
        }
    };

    // add the update function to the input
    let gen = quote! {
        #struct_name
        #update_func
    };


    // return the input
    gen.into()
}

fn string_to_camel_case(string: String) -> String{
    let mut result = String::new();
    let mut first = true;
    for c in string.chars(){
        if c == '_'{
            first = true;
        }else{
            if first{
                result.push(c.to_ascii_uppercase());
                first = false;
            }else{
                result.push(c.to_ascii_lowercase());
            }
        }
    }
    result
}