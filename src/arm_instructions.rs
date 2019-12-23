type Register = u8;

pub enum Cond {
    EQ,
    NE,
    CS,
    CC,
    MI,
    PL,
    VS,
    VC,
    HI,
    LS,
    GE,
    LT,
    GT,
    LE,
    AL,
}

enum Operand2 {
    Register { register: Register, shift: u8 },
    Immediate { value: u8, rotate: u8 },
}

struct DataProcessingOperation {
    destination: Register,
    setFlag: bool,
    operand1: Register,
    operand2: Operand2,
}

pub enum Operation {
    ADC(DataProcessingOperation),
    ADD(DataProcessingOperation),
    AND(DataProcessingOperation),
    B { offset: i32 },
    BIC(DataProcessingOperation),
    BL { offset: i32 },
    BX { jump: Register },
    CP,
    CMN(DataProcessingOperation),
    CMP(DataProcessingOperation),
    EOR(DataProcessingOperation),
    LDC,
    LDM,
    LDR,
    MCR,
    MLA,
    MOV(DataProcessingOperation),
    MRC,
    MRS,
    MSR,
    MUL,
    MVN(DataProcessingOperation),
    ORR(DataProcessingOperation),
    RSB(DataProcessingOperation),
    RSC(DataProcessingOperation),
    SBC(DataProcessingOperation),
    STC,
    STM,
    STR,
    SUB(DataProcessingOperation),
    SWI,
    SWP,
    TEQ(DataProcessingOperation),
    TST(DataProcessingOperation),
}

pub struct ArmInstruction {
    cond: Cond,
    op: Operation,
}

fn decode_cond(instruction: &u32) -> Cond {
    let cond = (instruction >> 28) as u8;
    match cond {
        0b0000 => Cond::EQ,
        0b0001 => Cond::NE,
        0b0010 => Cond::CS,
        0b0011 => Cond::CC,
        0b0100 => Cond::MI,
        0b0101 => Cond::PL,
        0b0110 => Cond::VS,
        0b0111 => Cond::VC,
        0b1000 => Cond::HI,
        0b1001 => Cond::LS,
        0b1010 => Cond::GE,
        0b1011 => Cond::LT,
        0b1100 => Cond::GT,
        0b1101 => Cond::LE,
        0b1110 => Cond::AL,
        _ => unreachable!(),
    }
}

fn decode_int24(value: &u32) -> i32 {
    const MODULO: i32 = 1 << 24;
    const MAX_VALUE: u32 = (1 << 23) - 1;

    if *value > MAX_VALUE {
        return (*value as i32) - MODULO;
    }

    *value as i32
}

// From https://iitd-plos.github.io/col718/ref/arm-instructionset.pdf
fn decode_operation(instruction: &u32) -> Operation {
    // Branch and Exchange
    if instruction & 0b0000_0001_0010_1111_1111_1111_0001_0000 != 0 {
        return Operation::BX {
            jump: (instruction & 0x0f) as u8,
        };
    // Branch or branch and link
    } else if instruction & (0b0000_101 << 24) != 0 {
        let link = (instruction >> 24) & 1u32;
        let offset = decode_int24(&(instruction & 0x00ffffff));

        if link == 0u32 {
            return Operation::B { offset };
        } else {
            return Operation::BL { offset };
        }
    // Data processing
    } else if instruction & 0b0000_00_1_0000_0_0000_0000_000000000000 != 0 {
        let setFlag = (instruction >> 19) & 1u32;
        let opcode = (instruction >> 20) & 0xfu32;
        let immediateFlag = (instruction >> 24) & 1u32;
        let operand1 = (instruction >> 16) & 0xfu32;
        let destination = (instruction >> 12) & 0xfu32;
        let operand2 = match immediateFlag {
            0 => Operand2::Register {
                register: (instruction & 0xf) as Register,
                shift: ((instruction >> 4) & 0xff) as u8,
            },
            1 => Operand2::Immediate {
                value: (instruction & 0xff) as u8,
                rotate: ((instruction >> 8) & 0xf) as u8,
            },
            _ => unreachable!(),
        };
    }

    ArmInstruction::LDC
}

pub fn decode_arm_instruction(instruction: &u32) -> ArmInstruction {
    ArmInstruction {
        cond: decode_cond(&instruction),
        op: decode_operation(instruction),
    }
}
