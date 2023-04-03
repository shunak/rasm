use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        std::process::exit(1);
    }

    let mut p = args[1].chars().peekable();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let mut num = parse_int(&mut p);
    println!("  mov rax, {}", num);

    while let Some(op) = p.next() {
        match op {
            '+' => {
                let num = parse_int(&mut p);
                println!("  add rax, {}", num);
            }
            '-' => {
                let num = parse_int(&mut p);
                println!("  sub rax, {}", num);
            }
            _ => {
                eprintln!("予期しない文字です: '{}'", op);
                std::process::exit(1);
            }
        }
    }

    println!("  ret");
}

fn parse_int(p: &mut std::iter::Peekable<std::str::Chars>) -> i64 {
    let mut num = 0;
    while let Some(c) = p.peek().cloned() {
        if c.is_digit(10) {
            num = num * 10 + c.to_digit(10).unwrap() as i64;
            p.next();
        } else {
            break;
        }
    }
    num
}

