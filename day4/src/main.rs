use core::num;
use std::{fs, vec};




fn main() {


    let data: String = fs::read_to_string("puzzle_input.txt").expect("Should be able to read puzzle_input file");
    
    let grid_data: Vec<Vec<i32>> = data.lines().map(|l| l.chars().map(|c| match c {
                                                                                '@' => 1,
                                                                                _ => 0, 
                                                                            }).collect::<Vec<i32>>()).collect();
    let rows = grid_data.len() as isize;
    let cols = if rows > 0 { grid_data[0].len() as isize } else { 0 };

    let mut number_of_accessiable_rolls: i32 = 0;


    for i in 0..rows {
        for j in 0..cols {

            if grid_data[i as usize][j as usize]==0 {
                continue
            }

            let mut number_of_paper_rolls: i32 = 0;

            
            let current_i = i;
            let current_j = j;

            let mut neighbor_count: i32 = 0;
            
            for di in -1..=1 {
                for dj in -1..=1 {
                    
                    if di == 0 && dj == 0 {
                        continue; 
                    }

                    let ni = current_i + di;
                    let nj = current_j + dj;

                    if ni >= 0 && ni < rows && nj >= 0 && nj < cols {
                        
                        let ni_usize = ni as usize;
                        let nj_usize = nj as usize;
                        
                        let value = grid_data[ni_usize][nj_usize];
                        number_of_paper_rolls += value;

                    neighbor_count += 1;

                    

            

                    }
                }
            }

            if number_of_paper_rolls < 4 {
                number_of_accessiable_rolls += 1;
            
            }

        }
    }    
    println!("vec of vec of char {:?}", grid_data);
    println!("row length: {}", grid_data.len());
    println!("col length: {}", grid_data[0].len());
    println!("Number of accessiable paper rolls: {}", number_of_accessiable_rolls);


}
