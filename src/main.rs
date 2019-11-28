use std::env;
use std::process::exit;

fn main() {
    let first = match env::args().nth(1) {
        Some(s) => s,
        None => {
            eprint!("[Error] args.get(1) is missing.");
            exit(1);
        }
    };
    println!(".intel_syntax noprefix");
    println!(".global _main");
    println!("_main:");
    println!("  mov rax, {}", first);
    println!("  ret");
}
