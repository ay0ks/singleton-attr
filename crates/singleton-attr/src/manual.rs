#[macro_export]
macro_rules! singleton_manual {
    (
        $([$($trait_bound:tt)+])?
        $type:ident$(<$($inner_gen:ty),+>)?
        $([$($outer_gen:ty),+])?
        $(where [$($where_bound:tt)+])?
        $(with [$($param_gen:ty),+])?
    ) => {
        const _: () = {
            static mut __INSTANCE: *mut $type$(<$($param_gen),+>)? = None;

            impl$(<$($trait_bound)+>)? singleton_attr::traits::SafeSingleton for $type$(<$($outer_gen),+>)?
            $(where $($where_bound)+)? {
                #[inline]
                fn init_instance(instance: Self) {
                    unsafe {
                        __INSTANCE = std::alloc::alloc(std::alloc::Layout::new::<Self>()) as *mut Self;
                        std::ptr::write_volatile(__INSTANCE, instance);
                    }
                }

                #[inline]
                fn get_instance() -> std::sync::LockResult<std::sync::MutexGuard<'static, Self>> {
                    unsafe {
                        if __INSTANCE.is_null() {
                            Self::init_instance(Self::default());
                        }
                        &mut *__INSTANCE
                    }
                }
            }

            impl$(<$($trait_bound)+>)? Drop for $type$(<$($outer_gen),+>)?
            $(where $($where_bound)+)? {
                fn drop(&mut self) {
                    unsafe { std::alloc::dealloc(__INSTANCE as *mut u8, std::alloc::Layout::new::<Self>()); }
                }
            }
        };
    };
}

#[macro_export]
macro_rules! singleton_safe_manual {
    (
        $([$($trait_bound:tt)+])?
        $type:ident$(<$($inner_gen:ty),+>)?
        $([$($outer_gen:ty),+])?
        $(where [$($where_bound:tt)+])?
        $(with [$($param_gen:ty),+])?
    ) => {
        const _: () = {
            static mut __INSTANCE: Option<std::sync::Arc<std::sync::Mutex<$type$(<$($param_gen),+>)?>>> = None;

            impl$(<$($trait_bound)+>)? singleton_attr::traits::SafeSingleton for $type$(<$($outer_gen),+>)?
            $(where $($where_bound)+)? {
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
    };
}
