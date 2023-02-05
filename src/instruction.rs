#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Inst {
    halt    = 0x0,
    nop     = 0x10,
    rrmovq  = 0x20, cmovle = 0x21, cmovl = 0x22, 
    cmove   = 0x23, cmovne = 0x24, cmovge = 0x25, cmovg = 0x26,
    irmovq  = 0x30,
    rmmovq  = 0x40,
    mrmovq  = 0x50,
    addq    = 0x60, subq = 0x61, andq = 0x62, xorq = 0x63,
    jmp     = 0x70, jle = 0x71, jl = 0x72,
    je      = 0x73, jne = 0x74, jge = 0x75, jg = 0x76,
    call    = 0x80,
    ret     = 0x90,
    pushq   = 0xa0,
    popq    = 0xb0,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Reg {
    rax = 0,
    rcx, rdx, rbx,
    rsp, rbp,
    rsi, rdi,
    r8, r9, r10, r11, r12, r13, r14,
    rIllegal = 0xF,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    instType: Inst,
    registerA: Reg,
    registerB: Reg,
    destination: usize,
}

impl Instruction {
    pub fn new(
        inst: Inst,
        rA: Option<Reg>, rB: Option<Reg>,
        dest: Option<usize>
    ) -> Instruction {
        Instruction {
            // position: pos,
            instType: inst,
            registerA: rA.unwrap_or(Reg::rIllegal),
            registerB: rB.unwrap_or(Reg::rIllegal),
            destination: dest.unwrap_or(0x0),
        }
    }
}

#[cfg(test)]
mod test{
    use super::Inst;

    // macro_rules! test_enum_value {
    //     ($func_name: ident, $variant:ident, $value: literal) => {
    //         #[test]
    //         fn $func_name() {
    //             let inst = Inst::$variant;
    //             println!("{:?}", inst);
    //             assert_eq!(inst as u8, $value);
    //         }
    //     }
    // }

    // test_enum_value!(test_enum_halt,  halt, 0);
    // test_enum_value!(test_enum_nop,   nop, 1);
    // test_enum_value!(test_enum_comvXX, cmovXX, 2);
    // test_enum_value!(test_enum_irmovq, irmovq, 3);
    // test_enum_value!(test_enum_rmmovq, rmmovq, 4);
    // test_enum_value!(test_enum_mrmovq, mrmovq, 5);
    // test_enum_value!(test_enum_OPq, OPq, 6);
    // test_enum_value!(test_enum_jXX, jXX, 7);
    // test_enum_value!(test_enum_call, call, 8);
    // test_enum_value!(test_enum_ret, ret, 9);
    // test_enum_value!(test_enum_pushq, pushq, 0xa);
    // test_enum_value!(test_enum_popq, popq, 0xb);
}
