use crate::instruction::{Instruction, Inst, Reg};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum parseError {
    InvalidInstruction,
    InvalidDestination,
    InvalidRegister,
    InvalidRRMOVQ,
    InvalidMRMOVQ,
}

pub fn stringToInstruction (inst: String) -> Result<Instruction, parseError> {

    let inst = inst.trim()  // remove tabs and spaces in the beginning or end
        .replace(","," ");  // transfer comma to whitespace to easier spliting

    let opers: Vec<&str> = inst.split_whitespace().collect();

    let one_map = HashMap::from([
        ("halt", Inst::halt), ("nop", Inst::nop), ("ret", Inst::ret)
    ]);

    let two_map = HashMap::from([
        ("jmp", Inst::jmp), ("jle", Inst::jle), ("jl", Inst::jl), ("je", Inst::je), ("jge", Inst::jge), ("jg", Inst::jg),
        ("call", Inst::call),
        ("pushq", Inst::pushq), ("popq", Inst::popq),
    ]);

    let rA_rB_map = HashMap::from([
        ("rrmovq", Inst::rrmovq), ("cmovle", Inst::cmovle), ("cmovl", Inst::cmovl), ("cmove", Inst::cmove), ("cmovne", Inst::cmovne), ("comvg", Inst::cmovg),
        ("irmovq", Inst::irmovq), ("rmmovq", Inst::rmmovq), ("mrmovq", Inst::mrmovq),
        ("addq", Inst::addq), ("subq", Inst::subq), ("andq", Inst::andq), ("xorq", Inst::xorq),
    ]);

    let reg_map = HashMap::from([
        ("rax", Reg::rax), ("rcx", Reg::rcx), ("rdx", Reg::rdx), ("rbx", Reg::rbx), 
        ("rsp", Reg::rsp), ("rbp", Reg::rbp), 
        ("rsi", Reg::rsi), ("rdi", Reg::rdi), 
        ("r8", Reg::r8), ("r9", Reg::r9), ("r10", Reg::r10), ("r11", Reg::r11), ("r12", Reg::r12), ("r13", Reg::r13), ("r14", Reg::r14), 
    ]);


    if opers.len() == 1 {
        match one_map.get(opers[0]) {
            Some(inst) => return Ok(Instruction::new(*inst, None, None, None)),
            _ => return Err(parseError::InvalidInstruction)
        }
    } else if opers.len() == 2 {
        let reg: Reg;
        let dest: usize;

        if opers[1].starts_with('%') {
            reg = match reg_map.get(&opers[1][1..]) {
                Some(reg) => *reg,
                None => {println!("{}", opers[1]); return Err(parseError::InvalidRegister);} 
            };

            match two_map.get(opers[0]) {
                Some(inst) => return Ok(Instruction::new(*inst, Some(reg), None, None)),
                _ => return Err(parseError::InvalidInstruction)
            }
        } else {
            dest = match opers[1].parse::<usize>() {
                Ok(dest) => dest,
                Err(_) => return Err(parseError::InvalidDestination),
            };

            match two_map.get(opers[0]) {
                Some(inst) => return Ok(Instruction::new(*inst, None, None, Some(dest))),
                _ => return Err(parseError::InvalidInstruction)
            }
        }
    } 
    else if opers.len() == 3 {
        let rA: Reg;
        let rB: Reg;
        let dest: usize;
        let value: i64;

        if opers[1].starts_with('%') && opers[2].starts_with('%') {
            rA = match reg_map.get(&opers[1][1..]) {
                Some(reg) => *reg,
                None => {println!("{}", opers[1]); return Err(parseError::InvalidRegister);} 
            };
            rB = match reg_map.get(&opers[2][1..]) {
                Some(reg) => *reg,
                None => {println!("{}", opers[2]); return Err(parseError::InvalidRegister);} 
            };

            match rA_rB_map.get(opers[0]) {
                Some(inst) => return Ok(Instruction::new(*inst, Some(rA), Some(rB), None)),
                _ => return Err(parseError::InvalidInstruction)
            }
        } else if opers[2].contains('(') {
            
            let parts = opers[2].split('(').collect::<Vec<&str>>();
            dest = match parts[0].trim().parse::<usize>() {
                Ok(val) => val,
                Err(_) => return Err(parseError::InvalidDestination),
            };
            let register = &parts[1][..parts[1].len() - 1];

            rA = match reg_map.get(&opers[1][1..]) {
                Some(reg) => *reg,
                None => {println!("{}", opers[1]); return Err(parseError::InvalidRegister);} 
            };
            rB = match reg_map.get(register) {
                Some(reg) => *reg,
                None => {println!("{}", opers[2]); return Err(parseError::InvalidRegister);} 
            };

            return Ok(Instruction::new(Inst::rmmovq, Some(rA), Some(rB), Some(dest)));
        } else if opers[1].contains('(') {

            return Err(parseError::InvalidInstruction);
        } else if opers[2].contains('%') {
            return Err(parseError::InvalidInstruction);
        } else {
            return Err(parseError::InvalidInstruction);
        }
    }
    else {
        return Err(parseError::InvalidInstruction);
    }
}


#[cfg(test)]
mod test {
    use super::stringToInstruction;
    use super::parseError;
    use crate::instruction::{Instruction, Inst};

    #[test]
    fn spaceBefore() {
        let inst = stringToInstruction("\t xorq  %rax , %rax \t".to_string());
        // assert_eq!(inst, )
    }

    #[test]
    fn halt() {
        let inst = stringToInstruction("\t halt \t".to_string());
        assert_eq!(inst.unwrap(), Instruction::new(Inst::halt, None, None, None));
    
        let inst = stringToInstruction("\t halt %rax \t".to_string());
        assert_eq!(inst, Err(parseError::InvalidDestination));
    }

    #[test]
    fn jmp() {
        let inst = stringToInstruction("\t jmp \t".to_string());
        assert_eq!(inst.unwrap(), Instruction::new(Inst::jmp, None, None, None));
    
        let inst = stringToInstruction("\t jmp %rmx \t".to_string());
        assert_eq!(inst, Err(parseError::InvalidDestination));
    }

    
}
