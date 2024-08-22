pub use windows_sys::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};

pub unsafe fn ntdllresolver() -> *mut u8 {
    GetModuleHandleA(windows_sys::s!("ntdll.dll")).cast()
}

pub unsafe fn procresolver(hmodule: *mut u8, lpprocname: *const u8) -> *const u8 {
    match GetProcAddress(hmodule as *mut core::ffi::c_void, lpprocname) {
        Some(a) => return core::mem::transmute(a),
        _ => return core::ptr::null()
    }
}
