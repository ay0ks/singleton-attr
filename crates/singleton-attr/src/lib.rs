#![allow(internal_features)]
#![feature(negative_impls, negative_bounds)]
pub mod traits;
extern crate singleton_attr_proc_macro;
pub use singleton_attr_proc_macro::singleton;
