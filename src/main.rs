use std::env;
use std::fs;

// Refer to https://esolangs.org/wiki/COW.
// These are ordered to match that page.
#[derive(Debug)]
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
    Empty, // Does nothing.
}

// Lexer to convert raw .cow source into a vector of COW opcodes.
fn lex(contents: String) -> Vec<Command> {
    let contents_split = contents.split_whitespace().collect::<Vec<&str>>();
    //println!("{:?}", contents_split);
    let mut lexed: Vec<Command> = Vec::with_capacity(contents_split.len());
    for element in contents_split {
        let comm = match element {
            "moo" => Command::LoopEnd,
            "mOo" => Command::DecPtr,
            "moO" => Command::IncPtr,
            "mOO" => Command::ExecVal,
            "Moo" => Command::RWCond,
            "MOo" => Command::DecVal,
            "MoO" => Command::IncVal,
            "MOO" => Command::LoopStart,
            "OOO" => Command::ZeroVal,
            "MMM" => Command::RegAccess,
            "OOM" => Command::Write,
            "oom" => Command::Read,
            _ => Command::Empty,
        };
        lexed.push(comm);
    }
    lexed
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    dbg!(file_path);

    let file_contents = fs::read_to_string(file_path)
        .expect("Unable to read file");
    println!("{}", file_contents);

    let lexed = lex(file_contents);
    dbg!("{}", lexed);

}
