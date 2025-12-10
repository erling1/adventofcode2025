use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};




fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("File not Found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()

}


fn main() {

    let lines = lines_from_file("puzzle_input.txt");

    let start_idx: i32 = 50;
    let max_idx: i32 = 100;

    let mut idx_num: Vec<i32> = Vec::new();

    let mut number_of_full_rotations: i32 = 0;

    for line in lines {
        let letter = &line[0..1];

        let mut num: i32 = line[1..]
            .parse()
            .expect("Parsing on number failed");

        number_of_full_rotations += num/max_idx;

        

        num = num.rem_euclid(max_idx);


        if letter == "L" {
            num = -num
        } else if letter == "R" {

        } else {
            panic!("Unexpected letter: {}", letter);
        }

        
        idx_num.push(num);

        };


    let mut new_index: i32 = start_idx; 

    println!("idx_num after modulus: {:?}", idx_num);

    let mut new_idx_vec: Vec<i32> = Vec::new();

    let mut zero_count: i32 = 0;


    for number in &idx_num {


        let steps: i32 = *number;


        if steps < 0 && steps > new_index {
            number_of_full_rotations += 1;
        } else if steps > 0 &&  steps > max_idx - new_index {
            number_of_full_rotations += 1;
        } 
        




        new_index = ((new_index + steps).rem_euclid(max_idx)) as i32;



        if new_index == 0 {
            zero_count += 1
        }
        
        new_idx_vec.push(new_index);

    





        }

    
    println!("new_idx_vec: {:?}", new_idx_vec);
    println!("count of 0 values {}", zero_count);
    println!("count of 0 pluss full rotatons : {}", zero_count + number_of_full_rotations); 









}
