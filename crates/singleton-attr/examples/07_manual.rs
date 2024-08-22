#[macro_use]
extern crate singleton_attr;

singleton_manual!([T:Default] Config<T>[T] where [T:Default] with [Box<String>]);
