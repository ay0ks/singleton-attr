extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, spanned::Spanned, ItemStruct, Visibility, Token};
use rand::{self, distributions::Alphanumeric, Rng};

#[proc_macro_attribute]
pub fn singleton(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemStruct);

    let prefix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    let name = &input.ident;
    let name_prefix = format!("___S{}", prefix);
    let (mod_name, init_name, instance_name) = (
        format_ident!("{}_{}_", name_prefix, name),
        format_ident!("{}", format!("{}_{}_INIT_", name_prefix, name).to_ascii_uppercase()),
        format_ident!("{}", format!("{}_{}_INSTANCE_", name_prefix, name).to_ascii_uppercase()),
    );
    let original_vis = input.vis.clone();
    input.vis = Visibility::Public(Token![pub](input.vis.span()));

    TokenStream::from(quote! {
        mod #mod_name {
            use super::*;
            use std::sync::{Arc, LockResult, Mutex, MutexGuard, Once};

            static #init_name: Once = Once::new();
            static mut #instance_name: Option<Arc<Mutex<#name>>> = None;

            #input

            impl #name {
                #[inline]
                pub unsafe fn initialize(value: Self) {
                    #instance_name = Some(Arc::new(Mutex::new(value)));
                }

                #[inline]
                pub unsafe fn deinitialize() {
                    #instance_name = None;
                }

                #[inline]
                pub fn get_instance() -> Arc<Mutex<Self>> {
                    #init_name.call_once(|| unsafe { Self::initialize(Self::default()); });
                    unsafe { #instance_name.clone().unwrap() }
                }

                #[inline]
                fn clone(&self) -> Arc<Mutex<Self>> {
                    unsafe { #instance_name.clone().unwrap() }
                }
            }

            impl Drop for #name {
                fn drop(&self) {}
            }
        }
        #original_vis use #mod_name::#name;
    })
}
