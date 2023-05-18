use std::io::BufReader;
use std::io::BufRead;
use std::io::prelude::*;
use std::fs::File;
use std::env;

#[macro_use]
extern crate bitflags;

bitflags! {
    struct Flags: u32 {
        const Z = 0b00000000;   // do nothing
        const N = 0b00000001;   // give line number on the beginning of each line
        const H = 0b00000010;   // print number of lines specified in the args[4] (from the top) (head)
        const T = 0b00000100;   // print number of lines specified in the args[4] (from the bottom) (tail)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut flags = Flags::Z;
    let mut n: u32 = 0;
    
    if (args.len() == 3) || (args.len() == 4) {
        if args[2].contains('n') { flags |= Flags::N }
        if args[2].contains('h') { flags |= Flags::H; n = args[3].parse().unwrap(); }
        else if args[2].contains('t') { flags |= Flags::T; n = args[3].parse().unwrap(); }
    }
    else if args.len() != 2 {
        println!("Usage: {} <file name> -<nth> <num>", args[0]);
        return;
    }

    if (flags.contains(Flags::H) || flags.contains(Flags::T)) && n == 0 { n = 6; }

    // use BufReader to read file line by line
    let file = match File::open(&args[1]) {
        Err(reason) => panic!("Couldn't open file {}. Reason: {}", args[1], reason),
        Ok(file) => file,
    };

    let mut line_num = 1;
    let mut reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        if flags.contains(Flags::N) && !(flags.contains(Flags::H) && line_num > n) && !(flags.contains(Flags::T) && line_num <= n) {
            print!("{:4} ", line_num);
        }
        if flags.contains(Flags::H) {
            if line_num <= n {
                println!("{}", line);
            }
        }
        else if flags.contains(Flags::T) {
            if line_num > (n) {
                println!("{}", line);
            }
        }
        line_num += 1;
    }
}