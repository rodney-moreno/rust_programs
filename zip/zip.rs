use std::env;
use std::fs;
use std::str;

fn encode(input_bytes: &[u8]) {
    let mut count: u8 = 0;
    let mut cur = input_bytes[0];
    let mut output: Vec<u8> = Vec::new();

    for item in input_bytes.iter() {
        if *item != cur {
            output.push(cur);
            output.push(count);

            // println!("{} {}", cur, count);

            count = 1;
            cur = *item;
        }

        count += 1;
    }
    
    for i in output.iter().step_by(2) {
        println!("{}", i);
    }
}

fn main() {
    // Get file from commandline
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let flag = &args[2];

    // Read contents into a string
    let mut output: Vec<u8> = Vec::new();
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file.");

    // Convert input into 8-bit ints
    let input_bytes = input.as_bytes();

    if flag == "encode" {
        encode(input_bytes);
    }
}



