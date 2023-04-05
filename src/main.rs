use std::env;
use std::io::Write;
use std::fmt;
use std::boxed::Box;


enum TokenKind {
    TK_RESERVRD,
    TK_NUM,
    TK_EOF,
}

struct Token {
    kind: TokenKind, // Token type
    next: Option<Box<Token>>, // Next input token by using Box(Smart pointer which indicates Heap allocated memory)
    val: i32, // Case, kind is number, that value.
    str: String, // Token string
}

let token: Option<Box<Token>> = None;

// Error Reporting
fn error(fmt: fmt::Arguments) -> ! { // "!" indicates this func never returns correct state.
    let mut stderr = std::io::stderr();
    writeln!(&mut stderr, "{}", fmt).expect("Error writing to stderr"); // macro writeln! which
                                                                        // implements
                                                                        // std::fmt::Write trait
    std::process::exit(1);
}

// When next token is expected symbol, read one token, else report error.
fn consume(op: char, token: &mut Token) -> bool {
    if token.kind != TokenKind::TK_RESERVRD || token.str.chars().next() != Some(op) {
        false;
    }
    token = &mut token.next;
    true;
}

// When next token is expected token, read on token, else report error
fn expect(op: char){
    if token.kind != TokenKind::TK_RESERVRD || token.str.chars().next() != Some(op){
        error(op);
    }
    token = &mut token.next;
}

fn expect_number() -> i32{
    if token.kind != TokenKind::TK_NUM {
        error("Not a number.");
    }
    let val = token.val;
    token = &mut token.next;
    val;
}






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

