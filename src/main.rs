use std::env;
use std::process::exit;

fn main() {
    let s = get_arg(1);
    let mut chars = s.chars();

    // print header
    println!(".intel_syntax noprefix");
    println!(".global _main");
    println!("_main:");
    print!("  mov rax, ");
    loop {
        let c = match chars.next() {
            Some(c) => c,
            None => {
                println!();
                break;
            }
        };
        printByChar(c);
    }
    println!("  ret");
}

fn get_arg(n: usize) -> String {
    return match env::args().nth(n) {
        Some(s) => s,
        None => {
            eprintln!("[Error] args().nth({}) is missing.", n);
            exit(1);
        }
    };
}

fn printByChar(c: char) {
    match c {
        '+' => {
            println!();
            print!("  add rax, ");
        }
        '-' => {
            println!();
            print!("  sub rax, ");
        }
        c => {
            print!("{}", c);
        }
    }
}