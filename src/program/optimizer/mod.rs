use super::Instruction;
use std::hash::{DefaultHasher, Hash, Hasher};

mod clear_loop;
mod combine_instructions;
mod util;

pub fn optimize(instructions: &mut Vec<Instruction>) {
    const OPTIMIZATION_PASSES: usize = 32;

    for _current_pass in 0..OPTIMIZATION_PASSES {
        let changed = optimize_once(instructions);
        if !changed {
            // println!("Reached fixed point: {current_pass} pass(es)");
            return;
        }
    }
    // Failed to reach fixed point
}

fn optimize_once(instructions: &mut Vec<Instruction>) -> bool {
    // This is an elegant (?) alternative to cloning
    // the instructions, but it might be better to
    // simply do the clone anyway
    let initial_hash = calculate_hash(instructions);
    combine_instructions::optimize(instructions);
    clear_loop::optimize(instructions);
    calculate_hash(instructions) != initial_hash
}

fn calculate_hash(instructions: &[Instruction]) -> u64 {
    let mut hasher = DefaultHasher::new();
    instructions.hash(&mut hasher);
    hasher.finish()
}
