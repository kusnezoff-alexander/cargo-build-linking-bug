#[allow(non_camel_case_types)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    println!("Hello, world!");
    unsafe { my_custom_fn(); }
}
