use std::fs;

pub fn run(){
   //retrieve data from file
    let filename = "input.txt";

    let contents = fs::read_to_string(filename).expect("unable to read file");

    //create two values: x, y
    let mut x = 0;
    let mut y = 0;

    //split by lines
    for movement in contents.split("\n"){
        //split line by space
        let line_split = movement.split(" ").collect::<Vec<&str>>();
        if line_split.len() < 2{ continue; }

        let command = line_split[0];
        let amount = line_split[1].parse::<i32>().unwrap();

        if command == "forward"{
            x += amount;
        }else if command == "up"{
            y -= amount;
        }else if command == "down"{
            y += amount;
        }
    }
        
    println!("result: {}", x * y);
}