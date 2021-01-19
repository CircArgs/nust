use std::todo;
#[derive(Debug)]
pub struct CPU {
    register_a: u8,
    register_x: u8,
    status: u8,
    program_counter: usize,
}

// https://wiki.nesdev.com/w/index.php/Status_flags
// NVss DIZC
// |||| ||||
// |||| |||+- Carry i.e. 1
// |||| ||+-- Zero i.e. 2
// |||| |+--- Interrupt Disable i.e. 8
// |||| +---- Decimal i.e. 16
// ||++------ No CPU effect, see: the B flag
// |+-------- Overflow i.e. 128
// +--------- Negative i.e. 256
const CARRY: u8 = 0b0000_0001;
const BIT_ZERO: u8 = CARRY;
const ZERO: u8 = 0b0000_0010;
const BIT_ONE: u8 = ZERO;
const INTERRUPT: u8 = 0b0000_0100;
const BIT_THREE: u8 = INTERRUPT;
const DECIMAL: u8 = 0b0000_1000;
const BIT_FOUR: u8 = DECIMAL;
const OVERFLOW: u8 = 0b0100_0000;
const BIT_SIX: u8 = OVERFLOW;
const NEGATIVE: u8 = 0b1000_0000;
const BIT_SEVEN: u8 = NEGATIVE;

impl CPU {
    fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
        }
    }
    fn set_zero_bit(&mut self, value: u8) {
        if value == 0 {
            self.status |= ZERO;
        } else {
            self.status &= !ZERO;
        }
    }

    fn set_neg_bit(&mut self, value: u8) {
        if (value & BIT_SEVEN) != 0 {
            self.status |= NEGATIVE;
        } else {
            self.status &= !NEGATIVE;
        }
    }
    fn set_zero_and_neg_bits(&mut self, value: u8) {
        self.set_zero_bit(value);
        self.set_neg_bit(value);
    }

    fn interpret(&mut self, program: &[u8]) {
        let mut instruction = program[self.program_counter];
        while program.len() > 0 {
            match instruction {
                0xa9 => {
                    self.program_counter += 1;
                    instruction = program[self.program_counter];

                    self.register_a = instruction;
                    self.set_zero_and_neg_bits(self.register_a);
                }
                0x00 => break,
                0xaa => {
                    self.set_zero_and_neg_bits(self.register_a);
                    self.register_x = self.register_a;
                    self.register_a = 0;
                }
                0xe8 => {
                    if self.register_x == 0xff {
                        self.register_x = 0;
                    } else {
                        self.register_x += 1;
                    }

                    self.set_zero_and_neg_bits(self.register_x);
                }
                _ => todo!(),
            }
            self.program_counter += 1;
            instruction = program[self.program_counter];
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(&vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(&vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }
    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(&vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(&vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
}
