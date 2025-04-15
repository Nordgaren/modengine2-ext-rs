#![allow(static_mut_refs)]

use std::{
    arch::x86_64::__cpuid,
    cell::OnceCell,
    ffi::{CString, c_char, c_void},
};
use vtable_rs::{VPtr, vmt_instance, vtable};

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

static mut EXTENSION: OnceCell<ModEngine2Extension> = OnceCell::new();

/// init function that takes `on_attach`, and two optional functions. Must be called by the user
/// in `DLLMain`.
pub fn init(
    on_attach: fn(&ModEngine2Extension),
    destructor: Option<fn(&mut ModEngine2Extension)>,
    on_detach: Option<fn(&ModEngine2Extension)>,
) {
    unsafe {
        EXTENSION.get_or_init(|| ModEngine2Extension::new(on_attach, destructor, on_detach));
    }
}

/// Exported function that gets called by modengine 2.
#[unsafe(no_mangle)]
#[cfg(not(feature = "modengine_ext_init"))]
pub unsafe extern "C" fn modengine_ext_init(
    _connector: *const c_void,
    extension: *mut *mut ModEngine2Extension,
) -> bool {
    println!("modengine_ext_init");
    *extension = EXTENSION
        .get_mut()
        .expect("EXTENSION is not initialized! Call `modengine2_rs::init` in DLLMain!");

    true
}

#[unsafe(no_mangle)]
#[cfg(feature = "modengine_ext_init")]
pub unsafe extern "C" fn modengine_ext_init_builtin(
    _connector: *const c_void,
    extension: *mut *mut ModEngine2Extension,
) -> bool {
    println!("modengine_ext_init_builtin");
    *extension = EXTENSION
        .get_mut()
        .expect("EXTENSION is not initialized! Call `modengine2_rs::init` in DLLMain!");

    true
}

#[repr(C)]
pub struct ModEngine2Extension {
    vftable: VPtr<dyn ModEngine2ExtVmt, Self>,
    id: CString,
    destructor: fn(&mut ModEngine2Extension),
    on_attach: fn(&ModEngine2Extension),
    on_detach: fn(&ModEngine2Extension),
}

impl ModEngine2Extension {

    /// Creates new `ModEngine2Extension`. Requires an `on_attach`. Other functions optional.
    ///
    /// # Arguments
    ///
    /// * `on_attach`: Required function. Initialize your mod here.
    /// * `destructor`: Optional function. Destructor for your mod.
    /// * `on_detach`: Optional function. Ran on detach.
    ///
    /// returns: ModEngine2Extension
    ///
    /// # Examples
    ///
    /// ```rust
    /// use modengine2_rs::ModEngine2Extension;
    ///
    /// fn on_attach(this: &ModEngine2Extension) {
    ///     // initialize mod here
    /// }
    ///
    /// fn main() {
    ///    modengine2_rs::init(on_attach, None, None);
    /// }
    /// ```
    fn new(
        on_attach: fn(&ModEngine2Extension),
        destructor: Option<fn(&mut ModEngine2Extension)>,
        on_detach: Option<fn(&ModEngine2Extension)>,
    ) -> Self {
        Self {
            vftable: VPtr::new(),
            id: Self::get_name(),
            on_attach,
            destructor: destructor.unwrap_or_else(|| |_| {}),
            on_detach: on_detach.unwrap_or_else(|| |_| {}),
        }
    }

    fn get_name() -> CString {
        CString::new(option_env!("MOD_NAME").unwrap_or_else(|| "CHANGEME"))
            .expect("Could not make C string from package name.")
    }
}

impl ModEngine2ExtVmt for ModEngine2Extension {
    extern "C" fn destructor(&mut self) {
        let destructor = self.destructor;
        destructor(self);
        self.id = CString::default();
    }
    extern "C" fn on_attach(&self) {
        let on_attach = self.on_attach;
        on_attach(self);
    }
    extern "C" fn on_detach(&self) {
        let on_detach = self.on_detach;
        on_detach(self);
    }
    extern "C" fn id(&self) -> *const c_char {
        self.id.as_ptr()
    }
}
