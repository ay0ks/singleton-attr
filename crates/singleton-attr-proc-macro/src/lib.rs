#![feature(proc_macro_span)]
extern crate proc_macro;
use proc_macro::{Span, TokenStream};
use quote::{format_ident, quote};
use rand::{self, distributions::Alphanumeric, Rng};
use syn::{parse_macro_input, spanned::Spanned, ItemStruct, Token, Visibility};

/// Unsafe singleton attribute macro
///
/// Mandates implementation of the `Default` trait, forbids implementation of `Clone` and `Drop` traits.
#[proc_macro_attribute]
pub fn singleton(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let span = Span::call_site();
    let source_path = span.source_file().path();
    let source_file = source_path.as_os_str().to_str().unwrap();

    let source_file = source_file
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect::<String>();

    let mut input = parse_macro_input!(item as ItemStruct);

    let prefix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    let name = &input.ident;
    let name_prefix = format!("__{}_S{}", source_file, prefix);
    let (mod_name, instance_name) = (
        format_ident!(
            "{}",
            format!("{}_{}_", name_prefix, name).to_ascii_lowercase()
        ),
        format_ident!(
            "{}",
            format!("{}_{}_INSTANCE_", name_prefix, name).to_ascii_uppercase()
        ),
    );
    let original_vis = input.vis.clone();
    input.vis = Visibility::Public(Token![pub](input.vis.span()));

    TokenStream::from(quote! {
        mod #mod_name {
            use std::{alloc::{self, Layout}, ptr};
            use singleton_attr::traits::Singleton;
            use super::*;

            static mut #instance_name: *mut #name = ptr::null_mut();
            #input

            impl Singleton for #name {
                #[inline]
                fn init_instance(instance: Self) {
                    unsafe {
                        #instance_name = alloc::alloc(Layout::new::<Self>()) as *mut Self;
                        ptr::write_volatile(#instance_name, instance);
                    }
                }

                #[inline]
                fn get_instance() -> &'static mut Self {
                    unsafe {
                        if #instance_name.is_null() {
                            Self::init_instance(Self::default());
                        }
                        &mut *#instance_name
                    }
                }
            }

            impl Drop for #name {
                fn drop(&mut self) {
                    unsafe { alloc::dealloc(#instance_name as *mut u8, Layout::new::<Self>()); }
                }
            }
        }
        #original_vis use #mod_name::#name;
    })
}

/// Safe singleton attribute macro
///
/// Mandates implementation of the `Default` trait, forbids implementation of `Clone` and `Drop` traits.
#[proc_macro_attribute]
pub fn singleton_safe(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let span = Span::call_site();
    let source_path = span.source_file().path();
    let source_file = source_path.as_os_str().to_str().unwrap();

    let source_file = source_file
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect::<String>();

    let mut input = parse_macro_input!(item as ItemStruct);

    let prefix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    let name = &input.ident;
    let name_prefix = format!("__{}_S{}", source_file, prefix);
    let (mod_name, instance_name) = (
        format_ident!(
            "{}",
            format!("{}_{}_", name_prefix, name).to_ascii_lowercase()
        ),
        format_ident!(
            "{}",
            format!("{}_{}_INSTANCE_", name_prefix, name).to_ascii_uppercase()
        ),
    );
    let original_vis = input.vis.clone();
    input.vis = Visibility::Public(Token![pub](input.vis.span()));

    TokenStream::from(quote! {
        mod #mod_name {
            use std::sync::{Arc, LockResult, Mutex, MutexGuard};
            use singleton_attr::traits::SafeSingleton;
            use super::*;

            static mut #instance_name: Option<Arc<Mutex<#name>>> = None;
            #input

            impl SafeSingleton for #name {
                #[inline]
                fn init_instance(instance: Self) {
                    unsafe {
                        #instance_name = Some(Arc::new(Mutex::new(instance)));
                    }
                }

                #[inline]
                fn get_instance() -> LockResult<MutexGuard<'static, Self>> {
                    unsafe {
                        if let None = #instance_name {
                            Self::init_instance(Self::default());
                        }

                        #instance_name.as_ref().unwrap().lock()
                    }
                }
            }
        }
        #original_vis use #mod_name::#name;
    })
}

/// Unsafe singleton derive macro
///
/// Mandates implementation of the `Default` trait, forbids implementation of `Clone` and `Drop` traits.
#[proc_macro_derive(Singleton)]
pub fn singleton_derive(item: TokenStream) -> TokenStream {
    let span = Span::call_site();
    let source_path = span.source_file().path();
    let source_file = source_path.as_os_str().to_str().unwrap();

    let source_file = source_file
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect::<String>();

    let input = parse_macro_input!(item as ItemStruct);

    let prefix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    let name = &input.ident;
    let name_prefix = format!("__{}_S{}", source_file, prefix);
    let instance_name = format_ident!(
        "{}",
        format!("{}_{}_INSTANCE_", name_prefix, name).to_ascii_uppercase()
    );

    TokenStream::from(quote! {
        static mut #instance_name: *mut #name = std::ptr::null_mut();

        impl singleton_attr::traits::Singleton for #name {
            #[inline]
            fn init_instance(instance: Self) {
                unsafe {
                    #instance_name = std::alloc::alloc(std::alloc::Layout::new::<Self>()) as *mut Self;
                    std::ptr::write_volatile(#instance_name, instance);
                }
            }

            #[inline]
            fn get_instance() -> &'static mut Self {
                unsafe {
                    if #instance_name.is_null() {
                        Self::init_instance(Self::default());
                    }
                    &mut *#instance_name
                }
            }
        }

        impl Drop for #name {
            fn drop(&mut self) {
                unsafe { std::alloc::dealloc(#instance_name as *mut u8, std::alloc::Layout::new::<Self>()); }
            }
        }
    })
}

/// Safe singleton attribute macro
///
/// Mandates implementation of the `Default` trait, forbids implementation of `Clone` and `Drop` traits.
#[proc_macro_derive(SafeSingleton)]
pub fn singleton_safe_derive(item: TokenStream) -> TokenStream {
    let span = Span::call_site();
    let source_path = span.source_file().path();
    let source_file = source_path.as_os_str().to_str().unwrap();

    let source_file = source_file
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect::<String>();

    let input = parse_macro_input!(item as ItemStruct);

    let prefix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    let name = &input.ident;
    let name_prefix = format!("__{}_S{}", source_file, prefix);
    let instance_name = format_ident!(
        "{}",
        format!("{}_{}_INSTANCE_", name_prefix, name).to_ascii_uppercase()
    );

    TokenStream::from(quote! {
        static mut #instance_name: Option<std::sync::Arc<std::sync::Mutex<#name>>> = None;

        impl singleton_attr::traits::SafeSingleton for #name {
            #[inline]
            fn init_instance(instance: Self) {
                unsafe {
                    #instance_name = Some(std::sync::Arc::new(std::sync::Mutex::new(instance)));
                }
            }

            #[inline]
            fn get_instance() -> std::sync::LockResult<std::sync::MutexGuard<'static, Self>> {
                unsafe {
                    if let None = #instance_name {
                        Self::init_instance(Self::default());
                    }

                    #instance_name.as_ref().unwrap().lock()
                }
            }
        }
    })
}
