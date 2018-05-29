use std::fs::File;
use std::process;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let max_line_length_argv = args.get(1);
    let max_line_length: i32;
    match max_line_length_argv {
        Some(x) => max_line_length = x.parse().unwrap(),
        None => {
            println!("No line length provided");
            process::exit(1);
        },
    };

    let filename_argv = args.get(2);
    let filename: String;
    match filename_argv {
        Some(x) => filename = x.to_string(),
        None => {
            println!("No filename provided");
            process::exit(1);
        }
    }

    let mut file = File::open(filename).expect("unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("unable to read file and write string");

    let stripped_string: String = contents.replace("\n", " ");
    let words: Vec<&str> = stripped_string.split(" ").collect();
    println!("-----------");
    println!("words len - {}", words.len());

    let mut cost_array = vec![0i32; words.len()];
    let mut previous_break_indexes = vec![0; words.len()];

    let testvec = vec![1,2,3,4];
    let testvec_slice = &testvec[0..3];

    for i in 0..words.len() {
        let maxsize = contents.len();
        let mut minimum_index = 0;
        for j in 0..i+1 {
            let line_length = words[j..i+1].join(" ").len() as i32;
            if !(line_length > max_line_length) {
                let extra_space = max_line_length - line_length;
                let mut previous_cost: i32;
               println!("{}", j);
                if j == 0 {
                    previous_cost = 0;
                } else {
                    previous_cost = cost_array[j - 1];
                }
            }
        }
    }
}

