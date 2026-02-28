//! Alacritty - The GPU Enhanced Terminal.

#![warn(rust_2018_idioms, future_incompatible)]
#![deny(clippy::all, clippy::if_not_else, clippy::enum_glob_use)]
#![cfg_attr(clippy, deny(warnings))]
// With the default subsystem, 'console', windows creates an additional console
// window for the program.
// This is silently ignored on non-windows systems.
// See https://msdn.microsoft.com/en-us/library/4cc7ya5b.aspx for more details.
#![windows_subsystem = "windows"]

#[cfg(not(any(feature = "x11", feature = "wayland", target_os = "macos", windows)))]
compile_error!(r#"at least one of the "x11"/"wayland" features must be enabled"#);

pub mod cli;
pub mod clipboard;
pub mod config;
pub mod daemon;
pub mod display;
pub mod event;
pub mod input;
#[cfg(unix)]
pub mod ipc;
pub mod logging;
#[cfg(target_os = "macos")]
pub mod macos;
pub mod message_bar;
pub mod migrate;
#[cfg(windows)]
pub mod panic;
pub mod renderer;
pub mod scheduler;
pub mod string;
pub mod window_context;

pub mod gl {
    #![allow(clippy::all, unsafe_op_in_unsafe_fn)]
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}
