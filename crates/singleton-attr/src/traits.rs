use std::sync::{LockResult, MutexGuard};

/// Unsafe singleton
///
/// `Singleton::init_instance` - (re)initialize the singleton, prefer to use this if you want more control with the initial instance.
/// `Singleton::get_instance` - get the singleton instance, if the instance is not initialized, it will initialize it with the default value.
pub trait Singleton: Default {
    fn init_instance(instance: Self);
    fn get_instance() -> &'static mut Self;
}

/// Safe singleton
///
/// `Singleton::init_instance` - (re)initialize the singleton, prefer to use this if you want more control with the initial instance.
/// `Singleton::get_instance` - get the singleton instance, if the instance is not initialized, it will initialize it with the default value.
pub trait SafeSingleton: Default {
    fn init_instance(instance: Self);
    fn get_instance() -> LockResult<MutexGuard<'static, Self>>;
}
