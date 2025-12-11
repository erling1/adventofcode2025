use core::num;
use std::{fs, vec};
 
fn main() {
    let data: String =
        fs::read_to_string("puzzle_input.txt").expect("Should be able to read puzzle_input file");
 
    let mut vec_of_numbers: Vec<Vec<(usize, u32)>> = data
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .enumerate()
                .map(|(i, c)| (i, c.to_digit(10).expect("not a digit")))
                .collect()
        })
        .collect();
 
    let mut fin_vec: Vec<i64> = Vec::new();
    for number_vec in &mut vec_of_numbers {
        let mut first_value: u32 = 0;
        let mut second_value: u32 = 0;
        let mut idx: usize = 0;
 
        for i in 1..=2 {
            if i == 1 {
                for (ind, value) in number_vec.iter() {
                    if *value > first_value && *ind != number_vec.len() - 1 {
                        first_value = *value;
                        idx = *ind;
                    }
                }
            } else if i == 2 {
                if idx + 1 < number_vec.len() {
                    for (_ind, value) in &number_vec[(idx + 1)..] {
                        if *value > second_value {
                            second_value = *value;
                        }
                    }
                }
            }
        }
 
        let result = (first_value as i64 * 10) + second_value as i64;
        fin_vec.push(result);
    }
 
    println!("total joltage: {}", fin_vec.iter().sum::<i64>());
}
