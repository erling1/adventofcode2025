use std::{fs, vec, collections::HashSet};


fn detect_duplicates(ids: &[String]) -> i32 {
    let mut pattern_count = 0;

    for id in ids {
        let n = id.len();
        let max_len_p = n / 2;

        for len_p in 1..=max_len_p {
            let p1 = &id[0..len_p]; 
            let p2 = &id[len_p..2 * len_p];
            if p1 == p2 {
                pattern_count += 1;
                break; 
            }
        }
    }

    pattern_count
}
 
fn main() {
    let data: String =
        fs::read_to_string("puzzle_input.txt").expect("Not able to read puzzle input");
    let vec_ids: Vec<&str> = data.trim().split(',').collect();
 
    let vec_start_id: Vec<i64> = vec_ids
        .iter()
        .enumerate()
        .map(|(_, s)| {
            s.split('-')
                .next()
                .expect("Format error on first id ")
                .parse()
                .expect("Couldnt parse first number")
        })
        .collect();
    let vec_end_id: Vec<i64> = vec_ids
        .iter()
        .enumerate()
        .map(|(_, s)| {
            s.split("-")
                .nth(1)
                .expect("format error on second id")
                .parse()
                .expect("couldnt parse second number")
        })
        .collect();
 
    let mut range_array: Vec<Vec<String>> = Vec::new();
 
    for (&i, &j) in vec_start_id.iter().zip(vec_end_id.iter()) {
        range_array.push((i..j).collect::<Vec<i64>>().iter().map(|s| s.to_string()).collect());
    }

    for vec in &range_array {

    }
    
    let mut count_of_detected_duplicates: Vec<i32> = Vec::new();
 
    for vec in &range_array {

        count_of_detected_duplicates.push(detect_duplicates(vec));

    }

    println!("Total number of invalid IDs: {}", count_of_detected_duplicates.iter().sum::<i32>());
}
