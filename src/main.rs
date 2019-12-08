use std::env;
use std::process::exit;
use std::collections::VecDeque;

mod tokenizer;

use crate::tokenizer::{Token, make_token_vector};

fn main() {
    let s = get_arg(1);
    let mut token_vec = VecDeque::from(make_token_vector(s));

    // print header
    println!(".intel_syntax noprefix");
    println!(".global _main");
    println!("_main:");

    // print main
    print_first_token(token_vec.pop_front().unwrap());
    for t in token_vec {
        print_by_token(t);
    }

    // print footer
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

fn print_first_token(t: Token) {
    match t {
        Token::Integer { val } => println!("  mov rax, {}", val),
        _ => {
            eprintln!("[ERROR] first token is not Integer. input Integer!");
            exit(1);
        }
    }
}

fn print_by_token(t: Token) {
    match t {
        Token::Operation { val } => if "+".to_string().eq(&val) {
            print!("  add rax, ");
        } else {
            print!("  sub rax, ");
        },
        Token::Integer { val } => {
            println!("{}", val);
        }
        _ => {}
    }
}