#![allow(unsafe_code)]
#![no_std]
use core::arch::global_asm;
pub mod resolvers;

global_asm!(include_str!(".\\hellhall.s"));

#[no_mangle]
pub static mut SSNNUMBER: u32 = 0;
#[no_mangle]
pub static mut JMPINSTRUCT: *const u8 = core::ptr::null();
pub static mut NTDLLPTR: *mut u8 = core::ptr::null_mut();

extern "C" {
    pub fn HellHall();
}

#[macro_export]
macro_rules! perform_syscall {
    ($x:expr, $($y:ty:$z:expr),*) => {
        if $crate::NTDLLPTR.is_null() {
            $crate::NTDLLPTR = $crate::resolvers::ntdllresolver();
        };
        let addressptr = $crate::resolvers::procresolver($crate::NTDLLPTR, $x);
        $crate::find_ssn(core::mem::transmute(addressptr), core::ptr::addr_of_mut!($crate::SSNNUMBER), core::ptr::addr_of_mut!($crate::JMPINSTRUCT));
        let exec: unsafe extern "C" fn($($y),*) = core::mem::transmute($crate::HellHall as *const ());
        exec($($z),*)
    };
}

pub unsafe fn find_ssn(addressptr: *const u8, ssnnumber: *mut u32, jmpinstruct: *mut *const u8) -> u32 {
    // Create the iterate value
    let mut iterate: usize = 0;

    // Closure to find bytes depending on location.
    let findfromoffset = |offset: usize, n: usize, direction: &str| -> *const u8 {
        match direction {
            "up" => addressptr.byte_sub(offset.clone()).byte_add(n),
            "down" => addressptr.byte_add(offset.clone()).byte_add(n),
            _ => panic!()
        }
    };

    // Closure to find a jmp instruction
    let findjmp = |direction: &str, iter: usize| {
        let mut l: usize = 0;
        loop {
            match direction {
                "up" => {
                    if *(findfromoffset(l, iter*32, "up")) == 0x0f && *(findfromoffset(l+1, iter*32, "up")) == 0x05 {
                        *jmpinstruct = findfromoffset(l, iter*32, "up");
                        break;
                    } 
                    l += 1;
                },
                "down" => {
                    if *(findfromoffset(l, iter*32, "down")) == 0x0f && *(findfromoffset(l+1, iter*32, "down")) == 0x05 {
                        *jmpinstruct = findfromoffset(l, iter*32, "down");
                        break;
                    }
                    l += 1 ;
                },
                _ => panic!()
            }
        }
    };
    let mut ssn: u64 = 0;
    let mut matchunhooked = |direction: &str, loopnum: &usize| -> bool {
        if *(findfromoffset(0, (*loopnum)*32, direction)) == 0x4c 
            && *(findfromoffset(1, (*loopnum)*32, direction)) == 0x8b 
                && *(findfromoffset(2, (*loopnum)*32, direction)) == 0xd1 
                && *(findfromoffset(3, (*loopnum)*32, direction)) == 0xb8
                && *(findfromoffset(6, (*loopnum)*32, direction)) == 0x00 
                && *(findfromoffset(7, (*loopnum)*32, direction)) == 0x00 {
                    ssn = ((*(findfromoffset(5, (*loopnum)*32, direction)) as u64) << 8) | *(findfromoffset(4, (*loopnum)*32, direction)) as u64;
                    match direction {
                        "down" => {*ssnnumber = (ssn as u32) - ((*loopnum) as u32);},
                        "up" =>  {*ssnnumber = ssn as u32 + ((*loopnum) as u32);},
                        _ => panic!()
                    }
                    true
                } else {
                    false
                }
    };
    if matchunhooked("down", &mut 0) {
        findjmp("down", iterate.clone());
        0
    } else { 
        if *addressptr == 0xe9 || *(addressptr.byte_add(3)) == 0xe9 {
            loop {
                iterate = iterate + 1;
                if matchunhooked("down", &iterate) {
                    findjmp("down", iterate.clone());
                    return 0
                };
                if matchunhooked("up", &iterate) {
                    findjmp("up", iterate.clone());
                    return 0
                }
            } 
        };
        1
    }
}
