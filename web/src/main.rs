#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

mod service;
mod shell;
mod views;

#[cfg(target_arch = "wasm32")]
use shell::AppShell;
#[cfg(target_arch = "wasm32")]
use silex::prelude::*;

#[cfg(target_arch = "wasm32")]
fn main() {
    mount_to_body(|| AppShell());
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("rchronos web front end is intended for wasm32 via trunk.");
}
