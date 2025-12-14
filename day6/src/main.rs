use core::num;
use std::{fs, vec};




fn main() {


    let data: String = fs::read_to_string("puzzle_input.txt").expect("Should be able to read puzzle_input file");
    

    let mut numbers: Vec<Vec<i64>> = Vec::new();
    let mut operations: Vec<&str> = Vec::new();

    for (count, line) in data.lines().enumerate() {
        println!("Line; {}", line);
       if count <= 3 {
        let row: Vec<i64> = line
            .split_whitespace()
            .map(|n| n.parse::<i64>().expect("Unable to parse number"))
            .collect();

        numbers.push(row);

       } else {
            operations.extend(line.split_whitespace());
        }        
    }

    
    
    let mut calculated_numbers: Vec<i64> = Vec::new();



    //for vector in numbers.len() {

    for i in 0..numbers[0].len() {
        println!("operations i {}", operations[i]);

        if operations[i] == "*" {

            calculated_numbers.push(numbers[0][i] * numbers[1][i] * numbers[2][i] * numbers[3][i]);

        } else if operations[i] == "+" {
            calculated_numbers.push(numbers[0][i] + numbers[1][i] + numbers[2][i] + numbers[3][i]);
        } 
    }

    
    println!("grand total : {}", calculated_numbers.iter().sum::<i64>());
    println!("opeorations : {:?}", operations ) 
                                                             






}
