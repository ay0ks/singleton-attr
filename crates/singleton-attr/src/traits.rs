pub trait Singleton {
    fn init_instance(instance: Self);
    fn get_instance() -> &'static mut Self;
    fn clone_instance(&self) -> &'static mut Self;
}
