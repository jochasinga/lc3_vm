use super::sign_extend;
use super::update_flags;
use crate::hardware::register::Registers;
use crate::hardware::Memory;

pub fn ld(instr: u16, registers: &mut Registers, memory: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1ff, 9);
    registers.update(dr, memory.read(registers.r_pc + pc_offset));
    update_flags(dr, registers);
}