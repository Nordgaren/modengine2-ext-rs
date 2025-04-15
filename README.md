# modengine2-ext-rs
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
everything else is handled for you, except the changes to the `.toml` file.

## toml file
You will need to add the dll to `external_dlls`. The path is relative to the modengine exe.
for example if it's in your `mod` folder for the ME2 mod, you would import it like so: `external_dlls = [ "mod/modname.dll" ]`

The name of your mod in the `MOD_NAME` environment variable is the name you want
to use for enabling the extension. Altogether it will look like this (Assume `MOD_NAME=modname`)

```toml
external_dlls = [ "mod/modname.dll" ]

[extension.modname]
enabled = true
```
Make sure you enable the extension before the `extension.mod_loader`, or else your file mods
will not get loaded. You can also enable it towards the end of the config file, after the `mods = []`
section.

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
