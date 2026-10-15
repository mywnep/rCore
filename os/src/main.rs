// os/src/main.rs

// tell the rustc not to use std
#![no_std]
// `fn main` requires `std`, to bypass the Rust generated entry, use `#![no_main]`
// a self-declared entry is required, use `#[unsafe(no_mangle)]` to prevent this entry being mangled
#![no_main]
// handle some items required by rust
mod lang_items;

fn main() {}
