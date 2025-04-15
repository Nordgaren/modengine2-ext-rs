# modengine2-rs
Minimal bindings for modengine2 extension.

## Usage
Set environment variable `MOD_NAME`, otherwise the mod will be named `CHANGEME`

implement `on_attach`
```rust
    fn on_attach(this: &ModEngine2Extension) {
        // Init mod here
    }
```
and optionally these.
```rust
    fn destructor(this: &mut ModEngine2Extension) {
        // Type destructor
    }
    fn on_detach(this: &ModEngine2Extension) {
        // Ran on detach.
    }
```
and pass them to the init function
```rust
extern "C" fn on_attach(this: &ModEngine2Extension) {
    // init logic
    std::thread::spawn(|| {
        eldenring_util::system::wait_for_system_init(&Program::current(), Duration::from_secs(60))
            .expect("Could not wait for system init");
    });
}

#[no_mangle]
#[allow(unused)]
pub extern "stdcall" fn DllMain(hinstDLL: usize, dwReason: u32, lpReserved: *mut usize) -> i32 {
    match dwReason {
        DLL_PROCESS_ATTACH => unsafe {
            modengine2_rs::init(None, Some(on_attach), None);
            1
        },
        DLL_PROCESS_DETACH => {
            1
        }
        _ => 1,
    }
}
```
everything else is handled for you

# Feature
`modengine_ext_init` - This feature removes the built in `modengine_ext_init` 
export, so you can write that function yourself. Otherwise you get a linker error.

This feature is just for those who want to change the code that runs before `on_attach`
```rust
#[unsafe(no_mangle)]
pub unsafe extern "C" fn modengine_ext_init(
    _connector: *const c_void,
    extension: *mut *mut ModEngine2Extension,
) -> bool {
    // your init function

    true
}
```
