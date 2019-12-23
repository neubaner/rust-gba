mod arm_instructions;
use arm_instructions::*;

fn main() {
    decode_arm_instruction(2u32);
    println!("Hello, world!");
}
