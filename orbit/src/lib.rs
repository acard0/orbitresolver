#![allow(dead_code)]

use std::{ffi::c_void, fs::OpenOptions, os::windows::io::AsRawHandle, time::Duration};

use il2cppinterop_core::{sys, MainParams};
use log::SetLoggerError;
use simplelog::{CombinedLogger, ConfigBuilder, SimpleLogger};
use windows::Win32::{Foundation::{CloseHandle, HANDLE, HMODULE}, System::{Console::{AllocConsole, SetStdHandle, STD_ERROR_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE}, LibraryLoader::DisableThreadLibraryCalls}};

mod app;
mod definitions;

#[no_mangle]
pub unsafe extern "system" fn DllMain(h_module: HMODULE, fdw_reason: u32, lp_reserved: *mut c_void) -> i32 {
    match fdw_reason {
        1 => { // DLL_PROCESS_ATTACH

            _= DisableThreadLibraryCalls(h_module);
            _= create_console();
            _= logger();

            let params = Box::into_raw(Box::new(MainParams { base: h_module.clone(), reserved: lp_reserved })) as usize;
            _= CloseHandle(sys::thread::spawn(move || {            
                let params = Box::from_raw(params as *mut MainParams);
                il2cppinterop_core::initialize(params.base, Some(Duration::MAX));

                app::main(*params)
            }));
        },
        0 => { // DLL_PROCESS_DEATTACH
           
        }
        _ => {}
    };

    1
}

fn create_console() -> std::io::Result<()> {
    unsafe {
        _= AllocConsole();
        
        let stdin = OpenOptions::new().read(true).open("CONIN$").unwrap();
        let stdin_handle = stdin.as_raw_handle();
        SetStdHandle(STD_INPUT_HANDLE, HANDLE(stdin_handle)).unwrap();
        std::mem::forget(stdin);

        let stdout = OpenOptions::new().write(true).open("CONOUT$").unwrap();
        let stdout_handle = stdout.as_raw_handle();
        SetStdHandle(STD_OUTPUT_HANDLE, HANDLE(stdout_handle)).unwrap();
        std::mem::forget(stdout);

        let stderr = OpenOptions::new().write(true).open("CONOUT$").unwrap();
        let stderr_handle = stderr.as_raw_handle();
        SetStdHandle(STD_ERROR_HANDLE, HANDLE(stderr_handle)).unwrap();
        std::mem::forget(stderr);

        Ok(()) 
    }
}

fn logger() -> Result<(), SetLoggerError> {
    std::env::set_var("RUST_BACKTRACE", "1");
    let config = ConfigBuilder::new().set_location_level(log::LevelFilter::Info).build();
    CombinedLogger::init(
        vec![
            SimpleLogger::new(log::LevelFilter::Info, config),
        ]
    )
}