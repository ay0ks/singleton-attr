/// Singleton
///
/// `Singleton::init_instance` - (re)initialize the singleton, prefer to use this if you want more control with the initial instance.
/// `Singleton::get_instance` - get the singleton instance, if the instance is not initialized, it will initialize it with the default value.
/// `Singleton::clone_instance` - get the singleton instance, use instead of `clone`, calls `Singleton::get_instance` under the hood.
pub trait Singleton: Default {
    fn init_instance(instance: Self);
    fn get_instance() -> &'static mut Self;
    fn clone_instance(&self) -> &'static mut Self;
}
