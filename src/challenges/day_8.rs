extern crate core;

use std::collections::{HashMap, VecDeque};

fn get_above(idx:usize, jdx: usize, grid:Vec<Vec<u32>>) -> Vec<u32>{
    let mut val_above: Vec<u32> = Vec::new();
    for (i, row) in grid.iter().enumerate(){
        if i < idx {
            let above: u32 = row[jdx];
            val_above.push(above);
        }
    }
    val_above.clone()
}

fn get_below(idx:usize, jdx: usize, grid:Vec<Vec<u32>>) -> Vec<u32>{
    let mut val_above: Vec<u32> = Vec::new();
    for (i, row) in grid.iter().enumerate(){
        if i > idx{
            let above: u32 = row[jdx];
            val_above.push(above);
        }
    }
    val_above.clone()
}

fn eval_direction(val: &u32, dir: &Vec<u32>) -> usize{
    let mut counter = 0;
    for elt in dir{
        counter += 1;
        if elt >= val{
            break;
        };
    }
    counter
}

fn eval_tree_val(curr: &u32, a: &Vec<u32>, b: &Vec<u32>, l: &Vec<u32>, r: &Vec<u32>) -> usize {
    let directions = vec![a, b, l, r];
    let mut erg = 1;
    for dir in directions{
        let value = eval_direction(curr, dir);
        erg *= value;
    }
    erg
}

pub fn show_results(){
    println!("Day 8: ");

    let input_res = crate::utils::helper::read_file_string("data/input_8.txt");
    let input = match input_res {
        Ok(value) => value,
        Err(err) => panic!("{}", err)
    };
    let str = input.trim(); // remove empty lines at beginning and end
    let mut visible = 0;
    let mut grid = Vec::new();

    for line in str.lines(){
        let line_as_int: Vec<u32> = line.trim().chars().map(|x| x.to_digit(10).unwrap()).collect();
        grid.push(line_as_int);
    }

    let mut tree_scores = Vec::new();
    for (idx, row) in grid.iter().enumerate(){
        if idx == 0 || idx == grid.len() - 1 { // erste reihe -> alle sind visible
            visible += row.len();
        }else{
            for (jdx, col) in row.clone().iter().enumerate(){
                if jdx == 0 || jdx == row.len() - 1{
                    visible += 1
                }else{ // idx, jdx are not on the edges
                    let mut above = get_above(idx, jdx, grid.clone());
                    let mut below = get_below(idx, jdx, grid.clone());
                    let mut left = grid.clone()[idx][..jdx].to_vec();
                    let mut right = grid.clone()[idx][jdx +1..].to_vec();
                    above.reverse();
                    left.reverse();

                    if above.clone().iter().max().unwrap() < col || // top
                        below.clone().iter().max().unwrap() < col || // below
                        left.clone().iter().max().unwrap() < col || // left
                        right.clone().iter().max().unwrap() < col { // right
                        visible += 1;
                    }

                    let tree_val = eval_tree_val(col, &above, &below, &left, &right);
                    tree_scores.push(tree_val);
                    println!("x:{} y:{}: {} = {}", idx, jdx, col, tree_val)
                }
            }
        }
    }
    println!("{}", visible);
    println!("{}", tree_scores.iter().max().unwrap());
}
