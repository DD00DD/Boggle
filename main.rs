#![allow(non_snake_case,non_camel_case_types,dead_code)]

use std::{collections::HashMap, ops::Index};
use std::collections::HashSet;

/*
    Fill in the boggle function below. Use as many helpers as you want.
    Test your code by running 'cargo test' from the tester_rs_simple directory.
    
    To demonstrate how the HashMap can be used to store word/coord associations,
    the function stub below contains two sample words added from the 2x2 board.
*/
struct coord {
    nates: Vec<(u8,u8)>,
}

fn boggle(board: & [&str], words: & Vec<String>) -> HashMap<String, Vec<(u8, u8)>>
{
    let mut dict: HashMap<String, Vec<(u8, u8)>> = HashMap::new();
    let mut co = coord {
        nates: Vec::new(),
    };
    let row = board.len();
    let col = board[0].len();
    let w: HashSet<String> =  words.iter().cloned().collect();

    /*
    let word = String::from("east");
    let coords = Vec::from([ (0, 0), (0, 1), (1, 0), (1, 1) ]);
    found.insert(word, coords);

    let word = String::from("as");
    let coords = Vec::from([ (0, 1), (1, 0) ]);
    found.insert(word, coords);
    */

    for word in w {
        if word.len() <= (row*col){
            co.nates = Vec::new();
            let x = check(board, &word, &row, &col, &mut co);
            
            if x == true && co.nates.is_empty() == false{
                dict.insert(String::from(word), Vec::from(co.nates.clone()));
            }
        
        }
    }
    println!("{:?}", dict);
    dict
}
    
fn check(board: & [&str], word: &str, row: &usize, col: &usize, coor: &mut coord) -> bool{
    let mut visited: Vec<(u8, u8)> = Vec::new();

    for i in 0..*row {
        for j in 0..*col {
            if board[i].chars().nth(j) == word.chars().nth(0) {
                let test = matched(board, word, row, col, i as isize, j as isize, 1, &mut visited, coor);

                if test == true{
                    return true;
                }
            }
        }
    }
    false
}

fn matched (board: & [&str], word: &str, row: &usize, col: &usize, i: isize, j: isize, length: usize, visit: &mut Vec<(u8, u8)>, co: &mut coord) -> bool{
    if i < 0 || i  > (*row - 1) as isize {
        return false;
    }
    else if j < 0 || j > (*col - 1) as isize {
        return false;
    }
    else if board[i as usize].chars().nth(j as usize) != word.chars().nth(length - 1){
        return false;
    }
    else if length > word.len(){
        return false;
    }
    else if contains_duplicates(visit, (i as u8, j as u8)) == true{
        return false;
    }
    else if length == word.len(){  
        visit.push((i as u8, j as u8));
        co.nates = visit.to_vec();   
        return true;
    }
    else{
        visit.push((i as u8, j as u8));
        
        let a = (matched(board, word, row, col, i-1, j, length + 1, visit, co) ||
        matched(board, word, row, col, i+1, j, length+1, visit, co) ||
        matched(board, word, row, col, i, j-1, length+1, visit, co) || 
        matched(board, word, row, col, i, j+1, length+1, visit, co)||
        matched(board, word, row, col, i-1, j+1, length+1, visit, co) ||
        matched(board, word, row, col, i-1, j-1, length+1, visit, co) ||
        matched(board, word, row, col, i+1, j-1, length+1, visit, co) ||
        matched(board, word, row, col, i+1, j+1, length+1, visit, co));

        visit.pop();
        a
    }
}

fn contains_duplicates (vec: &Vec<(u8, u8)>, next: (u8, u8)) -> bool {
    for tuple in vec.iter(){
        if *tuple == next {
            return true;
        }
    }
    false // No duplicates found
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

