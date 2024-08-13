# HellHall-rs

HellHall-rs is a Rust based implementation of HellHall that allows for indirect syscalls to be performed on Windows systems. It is a rough PoC, designed to be a baseline, and can be fairly easily edited in order to add in more advanced methods, such as TartarusGate, HalosGate etc. It has some design decisions to make it easier to use than the already very easy C based version.

## Usage

HellHall-rs doesn't work the same way that the initial HellHall code does, in which structs are created, and then sent to assembly to move into the correct variables. Instead, HellHall-rs uses exported statics which can be accessed by the assembly in order to perform syscalls. These statics are referenced directly by the assembly code, eliminating the need for one of the original HellHall functions written in MASM.

These functions to setup the syscalls and perform transmutations of the HellHall assembly function have been added as macros to allow ease-of-use. These macros are called syscall_setup and convert_to_type, and these macros are the only items that need to be imported for the execution to work properly.

```
use core::ptr::null;
use hellhall_rs::{convert_to_type, syscall_setup};
use windows::{core::PCSTR, Win32::{Foundation::HANDLE, System::{LibraryLoader::{GetProcAddress, LoadLibraryA}, Threading::{WaitForSingleObject, INFINITE}}}};

type NtCreateThreadExType = extern "C" fn(handle: *mut HANDLE, accessmask: i32, objectattributes: *const u8, processhandle: isize, lpstartaddress: *const u8, lpstartparameters: *const u8, flags: u64, stackzerobits: usize, sizeofstackcommit: usize, sizeofstackreserve: usize, lpbytesbuffer: *const u8);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let ntdllptr = LoadLibraryA(PCSTR("ntdll.dll\0".as_ptr()))?;
        let ntctexptr = GetProcAddress(ntdllptr, PCSTR("NtCreateThreadEx\0".as_ptr()));
        syscall_setup!(ntctexptr);
        let ntct: NtCreateThreadExType = convert_to_type!();
        let mut cthandle = HANDLE::default();
        ntct(&mut cthandle, 0x1FFFFF, null(), -1, (threadedfunction as *const ()).cast(), null(), 0, 0, 0, 0, null());
        WaitForSingleObject(cthandle, INFINITE);
    }
    Ok(())
}

unsafe fn threadedfunction() {
    println!("Testing");
}
```

## Opsec Considerations
This library uses a lot of exported functions and variables, which may be easy for reverse engineers to utilise. Therefore, it's highly recommended this isn't actually added as a crate, but the code used as a baseline and brought into your project. Primarily, the SSNNUMBER and JMPINSTRUCT variable names are fairly obvious to what the functions are doing and could do with renaming, locally in your main.rs file. The HellHall function in hellhall.s has also been exported through .globl in order for it to work out the box. It would be better to include that assembly file in your main code, and remove the globl, as that way exporting the function isn't required.

Finally, the problem with moving away from structs is that a reverse engineer can sit and watch the changes to SSNNUMBER in order to see the execution chain of functions in an easier way, which may lead you to want to return to the struct format of the original HellHall.
