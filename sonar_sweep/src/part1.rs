use std::fs;

pub fn run(){
    let filename = "input.txt";

    let contents = fs::read_to_string(filename).expect("unable to read file");

    let mut curr_depth = 99999;
    let mut increase_count = 0;
    for depth in contents.split("\n"){
        if depth.len() > 0{
            let prev_depth = curr_depth;
            curr_depth = depth.parse::<i32>().expect("Invalid value"); 
            if curr_depth > prev_depth {
                increase_count += 1;
            }
        }
    } 
    println!("Increases: {}", increase_count);
}