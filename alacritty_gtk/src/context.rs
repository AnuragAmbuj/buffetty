use std::ffi::{CStr, CString};
use std::sync::OnceLock;

use alacritty::gl::types::GLenum;
use alacritty::renderer::RendererContext;
use glutin::context::ContextApi;
use gtk4::gdk;
use gtk4::prelude::*;

pub struct GtkContext {
    context: gdk::GLContext,
}

impl GtkContext {
    pub fn new(context: gdk::GLContext) -> Self {
        Self { context }
    }
}

// Helper to load GL symbols
fn load_symbol(symbol: &str) -> *const std::ffi::c_void {
    static LIBRARY: OnceLock<libloading::Library> = OnceLock::new();

    let library = LIBRARY.get_or_init(|| unsafe {
        // Try Common Linux libraries
        libloading::Library::new("libepoxy.so.0")
            .or_else(|_| libloading::Library::new("libGL.so.1"))
            .or_else(|_| libloading::Library::new("libGLESv2.so.2"))
            .expect("Failed to load OpenGL library")
    });

    unsafe {
        let func_name = CString::new(symbol).unwrap();
        // Try dlsym-style loading first
        if let Ok(symbol) = library.get::<*const std::ffi::c_void>(func_name.as_bytes()) {
            return *symbol;
        }
        // Fallback? usually dlsym is enough.
        std::ptr::null()
    }
}

impl RendererContext for GtkContext {
    fn make_current(&self) {
        self.context.make_current();
    }

    fn get_proc_address(&self, symbol: &CStr) -> *const std::ffi::c_void {
        load_symbol(symbol.to_str().unwrap())
    }

    fn renderer_api(&self) -> ContextApi {
        if self.context.uses_es() { ContextApi::Gles(None) } else { ContextApi::OpenGl(None) }
    }
}
