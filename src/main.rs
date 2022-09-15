use std::env;
use std::fs;

// Refer to https://esolangs.org/wiki/COW.
// These are ordered to match that page.
#[derive(Debug)] // needed for debug prints
#[derive(Clone)] // needed to call to_vec() on a slice of Commands.
enum Command {
    LoopEnd, // Loop end command.
    DecPtr, // Move current memory position backward one block.
    IncPtr, // Move current memory position forward one block.
    ExecVal, // Execute current memory block as if it were an instruction.
    RWCond, // If current memory block is 0, execute a Read; otherwise, execute a Write.
    DecVal, // Decrement current memory block.
    IncVal, // Increment current memory block.
    LoopStart, // Loop start command. If current memory block is 0, skips next block and continues until next LoopEnd.
    ZeroVal, // Set current memory block to 0.
    RegAccess, // If register does not have a value, copy the current memory block value into it; otherwise, copy its value into the current memory block and clear the register.
    Write, // Print the contents of the current memory block to STDOUT as an ASCII character.
    Read, // Read an ASCII character from STDIN and store it in the current memory block.
}

// Instruction set.
// This is the same as the Command enum, but with an added sub-instruction set for loops.
#[derive(Debug)] // needed for debug prints
enum Instruction {
    DecPtr,
    IncPtr,
    ExecVal,
    RWCond,
    DecVal,
    IncVal,
    ZeroVal,
    RegAccess, 
    Write,
    Read,
    Loop(Vec<Instruction>),
}

// Lexer to convert raw .cow source into a vector of COW opcodes.
fn lex(contents: String) -> Vec<Command> {
    let contents_split = contents.split_whitespace().collect::<Vec<&str>>();
    //println!("{:?}", contents_split);
    let mut lexed: Vec<Command> = Vec::new();
    for element in contents_split {
        let comm = match element {
            "moo" => Some(Command::LoopEnd),
            "mOo" => Some(Command::DecPtr),
            "moO" => Some(Command::IncPtr),
            "mOO" => Some(Command::ExecVal),
            "Moo" => Some(Command::RWCond),
            "MOo" => Some(Command::DecVal),
            "MoO" => Some(Command::IncVal),
            "MOO" => Some(Command::LoopStart),
            "OOO" => Some(Command::ZeroVal),
            "MMM" => Some(Command::RegAccess),
            "OOM" => Some(Command::Write),
            "oom" => Some(Command::Read),
            _ => None
        };

        // We don't want empty commands getting into the lexer.
        match comm {
            Some(comm) => lexed.push(comm),
            None => ()
        }
    }
    lexed
}

// Parse the commands into an instruction set to be run
fn parse(commands: Vec<Command>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    // Variables for tracking the number of levels of nested loops.
    let mut loop_level = 0;
    let mut loop_start = 0;

    // Hacky, but rust doesn't let you modify i within the loop
    let mut skip_next = false;

    for i in 0..commands.len() {
        // Skip to next iteration if needed
        if skip_next == true {
            skip_next = false;
            continue;
        }

        // At loop level of 0, we want to actually parse the commands. Otherwise, we just look for the end of the loop and recurse.
        if loop_level == 0 {
            let instr = match commands[i] {
                Command::LoopEnd => {
                    dbg!("hit this failure at i = {}", i);
                    panic!("Loop end with no loop start");
                },
                Command::DecPtr => Some(Instruction::DecPtr),
                Command::IncPtr => Some(Instruction::IncPtr),
                Command::ExecVal => Some(Instruction::ExecVal),
                Command::RWCond => Some(Instruction::RWCond),
                Command::DecVal => Some(Instruction::DecVal),
                Command::IncVal => Some(Instruction::IncVal),
                Command::LoopStart => {
                    loop_level += 1;
                    loop_start = i;
                    skip_next = true;
                    None
                },
                Command::ZeroVal => Some(Instruction::ZeroVal),
                Command::RegAccess => Some(Instruction::RegAccess),
                Command::Write => Some(Instruction::Write),
                Command::Read => Some(Instruction::Read),
            };

            match instr {
                Some(instr) => instructions.push(instr),
                None => ()
            };

        } else {
            match commands[i] {
                Command::LoopStart => {
                    loop_level += 1;
                    loop_start = i;
                    // Ignore next command.
                    skip_next = true;
                },
                Command::LoopEnd => {
                    loop_level -= 1;

                    // If this is the lowest loop level in this parse call, we want to add it as an instruction by parsing its contents.
                    // Avoid start/end of the loop
                    // TODO test this, idk how rust vector splitting stuff works
                    if loop_level == 0 {
                        instructions.push(Instruction::Loop(parse(commands[(loop_start + 1)..i].to_vec())));
                    }
                },
                _ => ()
            }
        }
    }

    instructions
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    dbg!(file_path);

    let file_contents = fs::read_to_string(file_path)
        .expect("Unable to read file");
    println!("{}", file_contents);

    let lexed = lex(file_contents);
    //dbg!("{}", lexed);

    let instructions = parse(lexed);

    dbg!("{}", instructions);
}
