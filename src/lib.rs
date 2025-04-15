#![allow(static_mut_refs)]

use std::cell::OnceCell;
use std::ffi::{CString, c_char, c_void};
use vtable_rs::{VPtr, vtable};

pub type ModEngineExtInitFn<T: ModEngine2ExtVmt> =
    unsafe extern "C" fn(connector: *const c_void, extension: *mut *mut T) -> bool;

#[cfg(feature = "default_ext")]
static mut EXTENSION: OnceCell<ModEngine2Extension> = OnceCell::new();

#[cfg(feature = "default_ext")]
pub fn init() {
    unsafe {
        EXTENSION.get_or_init(ModEngine2Extension::default);
    }
}

#[cfg(feature = "default_ext")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn modengine_ext_init(
    _connector: *const c_void,
    extension: *mut *mut ModEngine2Extension,
) -> bool {
    println!("modengine_ext_init");
    *extension = EXTENSION.get_mut().unwrap();

    true
}

#[vtable]
pub trait ModEngine2ExtVmt {
    fn destructor(&mut self) {
        println!("destructor!");
        // We can provide a default implementation too!
    }
    fn on_attach(&self) {
        println!("attached!");
    }
    fn on_detach(&self) {
        println!("detached!");
    }
    fn id(&self) -> *const c_char;
}

#[cfg(feature = "default_ext")]
#[repr(C)]
pub struct ModEngine2Extension {
    vftable: VPtr<dyn ModEngine2ExtVmt, Self>,
    id: CString,
}

#[cfg(feature = "default_ext")]
impl Default for ModEngine2Extension {
    fn default() -> Self {
        Self {
            vftable: VPtr::default(),
            id: CString::new("ermod").expect("Could not make C string from package name."),
        }
    }
}
#[cfg(feature = "default_ext")]
impl ModEngine2ExtVmt for ModEngine2Extension {
    extern "C" fn id(&self) -> *const c_char {
        self.id.as_ptr()
    }
}
