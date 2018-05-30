use std::fs::File;
use std::process;
use std::io::prelude::*;

fn get_final_string(words: Vec<&str>, final_break_indexes: &mut Vec<usize>) -> String {
    let mut final_string = String::from("");

    let mut extra_char: char;

    for i in 0..words.len() {
        if final_break_indexes.contains(&(i+1)) {
            extra_char = '\n'
        } else {
            extra_char = ' '
        }

        final_string.push_str(words[i]);
        final_string.push(extra_char)
    }
    return final_string

}

fn get_break_indexes(previous_break_indexes: Vec<usize>, final_break_indexes: &mut Vec<usize>) -> &mut Vec<usize> {
  if previous_break_indexes.is_empty() {
    return final_break_indexes;
  } else {
      let last_break_index = *previous_break_indexes.last().expect("Previous break index is empty");
      final_break_indexes.push(last_break_index);

      return get_break_indexes(previous_break_indexes[0..(last_break_index as usize)].to_vec(), final_break_indexes)
  }
}


fn get_consistent_lines(max_line_length: i32, contents: String) -> String  {
    let stripped_string: String = contents.replace("\n", " ");
    let words: Vec<&str> = stripped_string.split(" ").collect();

    let mut cost_array = vec![0i32; words.len()];
    let mut previous_break_indexes = vec![0; words.len()];

    for i in 0..words.len() {
        let mut minimum_cost = 1000000000;
        let mut minimum_index = 0;
        for j in 0..i+1 {
            let line_length = words[j..i+1].join(" ").len() as i32;
            if !(line_length > max_line_length) {
              let extra_space = max_line_length - line_length;
              let mut previous_cost: i32;
              if j == 0 {
                  previous_cost = 0;
              } else {
                  previous_cost = cost_array[j - 1];
              }

              let total_cost = previous_cost + extra_space.pow(2);
              if total_cost < minimum_cost {
                  minimum_cost = total_cost;
                  minimum_index = j
              }

            }
        }
        previous_break_indexes[i] = minimum_index;
        cost_array[i] = minimum_cost
    }

    let mut initial_break_indexes = Vec::new();
    let final_break_indexes = get_break_indexes(previous_break_indexes, &mut initial_break_indexes);

    get_final_string(words, final_break_indexes)
}

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
    println!("{}", get_consistent_lines(max_line_length, contents))
}

