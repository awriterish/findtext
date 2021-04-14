use std::env;
use std::fs;

fn findInFile(lookFor:String, files:Vec<String>) {
    for file in files {
        let data = fs::read_to_string(file).expect("Unable to read file");
        let split = data.split("\n");
        for s in split {
            if s.contains(&lookFor) {
                println!("{}", s);
            }
        }
    }
}

fn main() {
    let arg = env::args()
        .skip(1)
        .collect::<Vec<String>>();
    if arg.len() > 1 {
        findInFile(arg[0].to_owned(), arg[1..arg.len()].to_owned());
    } else {
        println!("Not enough arguments passed.");
    }
}
