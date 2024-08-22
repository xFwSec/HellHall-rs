# HellHall-rs

HellHall-rs is a Rust based implementation of HellHall that allows for indirect syscalls to be performed on Windows systems. It is a rough PoC, designed to be a baseline, and can be fairly easily edited in order to add in more advanced methods, such as TartarusGate, HalosGate etc. It has some design decisions to make it easier to use than the already very easy C based version.

## Usage

HellHall-rs doesn't work the same way that the initial HellHall code does, in which structs are created, and then sent to assembly to move into the correct variables. Instead, HellHall-rs uses exported statics which can be accessed by the assembly in order to perform syscalls. These statics are referenced directly by the assembly code, eliminating the need for one of the original HellHall functions written in MASM.

This crate has been designed to allowed indirect syscalls in a single use of a macro. The current setup is using GetModuleHandle/GetProcAddress from the windows-sys crate to perform resolutions. To add your own in, you can modify the resolvers in resolvers.rs, just make sure you keep the the return types the same. You can add in API hashed versions fairly easily as the macro is just expected an expression to use with the proc resolver.

Below is an example using NtCreateThreadEx. The first variable passed to the macro is the idenitifer used by the procresolver function, and the others follow the syntax type: variable which allows to type used to pass to HellHall to by dynamically created and then populated with the correct functions.

```
use core::ptr::null;
use hellhall_rs::perform_syscall;
use windows::Win32::{Foundation::HANDLE, System::Threading::{WaitForSingleObject, INFINITE}};
use windows_sys::s;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let mut cthandle = HANDLE::default();
        perform_syscall!(
            s!("NtCreateThreadEx"), 
            *mut HANDLE: &mut cthandle, 
            i32: 0x1FFFFF, 
            *const u8: null(), 
            isize: -1, 
            *const (): threadedfunction as *const (), 
            *const u8: null(), 
            u64: 0, 
            usize: 0, 
            usize: 0, 
            usize: 0, 
            *const u8: null()
            );
        WaitForSingleObject(cthandle, INFINITE);
    }
    Ok(())
}

unsafe extern "system" fn threadedfunction(_lpthreadparameter: *mut u8) -> u32 {
    println!("Thread created successfully");
    0
}
```

## Opsec Considerations
This library uses a lot of exported functions and variables, which may be easy for reverse engineers to utilise. Therefore, it's highly recommended this isn't actually added as a crate, but the code used as a baseline and brought into your project. Primarily, the SSNNUMBER and JMPINSTRUCT variable names are fairly obvious to what the functions are doing and could do with renaming, locally in your main.rs file. The HellHall function in hellhall.s has also been exported through .globl in order for it to work out the box. It would be better to include that assembly file in your main code, and remove the globl, as that way exporting the function isn't required.

Finally, the problem with moving away from structs is that a reverse engineer can sit and watch the changes to SSNNUMBER in order to see the execution chain of functions in an easier way, which may lead you to want to return to the struct format of the original HellHall.
