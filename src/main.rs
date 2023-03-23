use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Error: Incorrect number of arguments.");
        process::exit(1);
    }

    let num: i32 = args[1].parse().unwrap();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!(" mov rax, {}", num);
    println!(" ret");
}

