use std::env;
use std::fs;
/*
fn encode(Vect<u8> input, &Vect<u8> output) {
    let mut count: u8 = 0;
    let mut cur = input_bytes[0];

    for item in input_bytes.iter().enumerate() {
        if item != cur {
            output.push(cur);
            output.push(count);

            count = 1;
            cur = item;
        }

        count += 1;
    }
    
    let output_str = String::from_utf8(output).unwrap();
    println!("{}", output_str);
}

fn decode() {

}
*/
fn main() {
    // Get file from commandline
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let flag = &args[2];

    // Read contents into a string
    let mut output: Vec<u8> = Vec::new();
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file.");
    let input_bytes = input.as_bytes();

    for (i, &item) in input_bytes.iter().enumerate() {
        println!("{}", item);
    }
}



