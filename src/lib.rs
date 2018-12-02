use std::env;
use std::fs::File;
use std::io::BufReader;

pub fn get_input_file() -> BufReader<std::fs::File> {
    let mut args = env::args();
    let input_file_path = args.nth(1).expect("requires one argument: input file path");
    let input_file = File::open(input_file_path).expect("unable to open input file");
    let reader = BufReader::new(input_file);
    reader
}