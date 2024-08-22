#[macro_export]
macro_rules! singleton_define {
    ($type:ident) => {
        static mut __INSTANCE: *mut $type = std::ptr::null_mut();
    };

    ($type:ident<$($gen:tt),+>) => {
        static mut __INSTANCE: *mut $type<$($gen),+> = std::ptr::null_mut();
    };
}

#[macro_export]
macro_rules! singleton_impl_methods {
    () => {
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
    };
}

#[macro_export]
macro_rules! singleton_impl {
    ($type:ident) => {
        impl singleton_attr::traits::Singleton for $type {
            singleton_impl_methods!();
        }
    };

    ($type:ident<$($gen:tt),+>) => {
        impl<$($gen),+> singleton_attr::traits::Singleton for $type<$($gen),+> {
            singleton_impl_methods!();
        }
    };

    ($type:ident<$($gen:tt),+> where $($bound:tt)+) => {
        impl<$($gen),+> singleton_attr::traits::Singleton for $type<$($gen),+> where $($bound)+ {
            singleton_impl_methods!();
        }
    };
}

#[macro_export]
macro_rules! singleton_manual {
    ($type:ident $(with $($known_gen:ident = $known_val:tt),+)?) => {
        const _: () = {
            singleton_define!($type$(<$($known_val),+>)?);
            singleton_impl!($type);
        };
    };

    ($type:ident<$($gen:tt),+> $(with $($known_gen:ident = $known_val:tt),+)?) => {
        const _: () = {
            singleton_define!($type$(<$($known_val),+>)?);
            singleton_impl!($type<$($gen),+>);
        };
    };

    ($type:ident<$($gen:tt),+> $(with $($known_gen:ident = $known_val:tt),+)? where $($bound:tt)+) => {
        const _: () = {
            singleton_define!($type$(<$($known_val),+>)?);
            singleton_impl!($type<$($gen),+> where $($bound)+);
        };
    };
}

#[macro_export]
macro_rules! singleton_safe_define {
    ($type:ident) => {
        static mut __INSTANCE: Option<std::sync::Arc<std::sync::Mutex<$type>>> = None;
    };

    ($type:ident<$($gen:tt),+>) => {
        static mut __INSTANCE: Option<std::sync::Arc<std::sync::Mutex<$type<$($gen),+>>>> = None;
    };
}

#[macro_export]
macro_rules! singleton_safe_impl_methods {
    () => {
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
    };
}

#[macro_export]
macro_rules! singleton_safe_impl {
    ($type:ident) => {
        impl singleton_attr::traits::SafeSingleton for $type {
            singleton_safe_impl_methods!();
        }
    };

    ($type:ident<$($gen:tt),+>) => {
        impl<$($gen),+> singleton_attr::traits::SafeSingleton for $type<$($gen),+> {
            singleton_safe_impl_methods!();
        }
    };

    ($type:ident<$($gen:tt),+> where $($bound:tt)+) => {
        impl<$($gen),+> singleton_attr::traits::SafeSingleton for $type<$($gen),+> where $($bound)+ {
            singleton_safe_impl_methods!();
        }
    };
}

#[macro_export]
macro_rules! singleton_safe_manual {
    ($type:ident $(with $($known_gen:ident = $known_val:tt),+)?) => {
        const _: () = {
            singleton_safe_define!($type$(<$($known_val),+>)?);
            singleton_safe_impl!($type);
        };
    };

    ($type:ident<$($gen:tt),+> $(with $($known_gen:ident = $known_val:tt),+)?) => {
        const _: () = {
            singleton_safe_define!($type$(<$($known_val),+>)?);
            singleton_safe_impl!($type<$($gen),+>);
        };
    };

    ($type:ident<$($gen:tt),+> $(with $($known_gen:ident = $known_val:tt),+)? where $($bound:tt)+) => {
        const _: () = {
            singleton_safe_define!($type$(<$($known_val),+>)?);
            singleton_safe_impl!($type<$($gen),+> where $($bound)+);
        };
    };
}
