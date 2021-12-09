use std::fs;

pub fn run(){
    let filename = "input.txt";

    let contents = fs::read_to_string(filename).expect("unable to read file");

    let mut last_sum = 99999;
    let mut increase_count = 0;
    let depth_split = contents.split("\n").collect::<Vec<&str>>();
    for i in 0..depth_split.len()-3{
        let depth_one = depth_split[i].parse::<i32>().expect("Invalid value"); 
        let depth_two = depth_split[i+1].parse::<i32>().expect("Invalid value"); 
        let depth_three = depth_split[i+2].parse::<i32>().expect("Invalid value"); 
        let curr_sum = depth_one + depth_two + depth_three;
        if curr_sum > last_sum{
            increase_count += 1;
        }
        last_sum = curr_sum;
    } 
    println!("Increases: {}", increase_count);
}