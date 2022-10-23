#![allow(unused)]
use std::arch::asm;

fn main() {

    let mut a: u64 =4;
    let b: u64 =4;
    let c: u64 =4;
    let mut x: u64 =4;
    //let i: u64 = 3;
    //let o: u64;
    let cmd = 0xd1;
    let mut name_buf = [0_u8; 12];

    fn mul(d: u64, e: u64)  -> u128 {
        let lo:u64;
        let hi:u64;
        unsafe{asm!(
                "mul {}",
        in(reg) d,
        inlateout("rax") e => lo,
        lateout("rdx") hi
        );}
        ((hi as u128) << 64 ) + lo as u128
    }

    extern "C" fn foo(arg: i32) -> i32 {
        println!("arg = {}", arg);
        arg * 2}

    fn call_foo(arg: i32) -> i32 {
        unsafe {
            let result;
            asm!(
                    "call {}",
            in(reg) foo,
            in("rdi") arg,
            out("rax") result,
            clobber_abi("C"),
            );
            result
        }
    }

    fn load_fpu_control_word(control: u16) {
        unsafe {
            asm!("fldcw[{}]", in(reg) &control, options(nostack));
        }
    }

    unsafe {
        asm!(
                "nop",
            // "mov {0}, {1}",
            "add {0}, {1}",
            "add {0}, {2}",
            inout(reg) a,
            in(reg) b,
            in(reg) c,
            );
            asm!("out 0x64, eax", in("eax") cmd
        );
        asm!(
            "push rbx",
        "cpuid",
        "mov [rdi], ebx",
        "mov [rdi + 4], edx",
        "mov [rdi + 8], ecx",
        "pop rbx",
        in("rdi") name_buf.as_mut_ptr(),
        inout("eax") 0 => _,
        out("ecx") _,
        out("edx") _,
                );
        asm!(
        "mov {tmp}, {x}",
        "shl {tmp}, 1",
        "shl {x}, 2",
        "add {x}, {tmp}",
        x = inout(reg) x,
        tmp = out(reg) _,
        );
    }

    assert_eq!(a, 12);
    assert_eq!(x, 4*6);
    let name = core::str::from_utf8(&name_buf).unwrap();
    println!("CPU Manufacturer ID: {}", name);
    // CLOSING BRACKETS
    }



