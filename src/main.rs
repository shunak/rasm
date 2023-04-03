use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        std::process::exit(1);
    }

    let mut p = args[1].as_bytes().as_ptr();
    let end = unsafe { p.add(args[1].len()) };

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let mut num = parse_int(&mut p);
    println!("  mov rax, {}", num);

    while p < end {
        if unsafe { *p } == b'+' {
            p = unsafe { p.add(1) };
            num = parse_int(&mut p);
            println!("  add rax, {}", num);
            continue;
        }

        if unsafe { *p } == b'-' {
            p = unsafe { p.add(1) };
            num = parse_int(&mut p);
            println!("  sub rax, {}", num);
            continue;
        }

        eprintln!("予期しない文字です: '{}'", unsafe { *p } as char);
        std::process::exit(1);
    }

    println!("  ret");
}

fn parse_int(p: &mut *const u8) -> i64 {
    let mut num = 0;
    while unsafe { (**p as char).is_digit(10) } {
        num = num * 10 + (unsafe { **p } - b'0') as i64;
        *p = unsafe { p.add(1) };
    }
    num
}

