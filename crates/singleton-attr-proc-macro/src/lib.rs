#![feature(proc_macro_span)]
extern crate proc_macro;
use proc_macro::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, spanned::Spanned, ItemStruct, Token, Visibility};

#[proc_macro_derive(Singleton)]
pub fn singleton_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;

    TokenStream::from(quote! {
        const _: () = {
            static mut __INSTANCE: *mut #name = std::ptr::null_mut();

            impl singleton_attr::traits::Singleton for #name {
                #[inline]
                fn init_instance(instance: Self) {
                    unsafe {
                        __INSTANCE = std::alloc::alloc(std::alloc::Layout::new::<Self>()) as *mut Self;
                        std::ptr::write_volatile(__INSTANCE, instance);
                    }
                }

                #[inline]
                fn get_instance() -> &'static mut Self {
                    unsafe {
                        if __INSTANCE.is_null() {
                            Self::init_instance(Self::default());
                        }
                        &mut *__INSTANCE
                    }
                }
            }

            impl Drop for #name {
                fn drop(&mut self) {
                    unsafe { std::alloc::dealloc(__INSTANCE as *mut u8, std::alloc::Layout::new::<Self>()); }
                }
            }
        };
    })
}

#[proc_macro_derive(SafeSingleton)]
pub fn singleton_safe_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;

    TokenStream::from(quote! {
        const _: () = {
            static mut __INSTANCE: Option<std::sync::Arc<std::sync::Mutex<#name>>> = None;

            impl singleton_attr::traits::SafeSingleton for #name {
                #[inline]
                fn init_instance(instance: Self) {
                    unsafe {
                        __INSTANCE = Some(std::sync::Arc::new(std::sync::Mutex::new(instance)));
                    }
                }

                #[inline]
                fn get_instance() -> std::sync::LockResult<std::sync::MutexGuard<'static, Self>> {
                    unsafe {
                        if let None = __INSTANCE {
                            Self::init_instance(Self::default());
                        }

                        __INSTANCE.as_ref().unwrap().lock()
                    }
                }
            }
        };
    })
}
