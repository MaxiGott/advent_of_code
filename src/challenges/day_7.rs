extern crate core;

use std::collections::{HashMap, VecDeque};


pub fn show_results(){
    println!("Day 7: ");

    let input_res = crate::utils::helper::read_file_string("data/input_7.txt");
    let input = match input_res {
        Ok(value) => value,
        Err(err) => panic!("{}", err)
    };
    let str = input.trim(); // remove empty lines at beginning and end

    let mut curr_path: VecDeque<String> = VecDeque::new();
    let mut sizes:HashMap<String, i32> = HashMap::new();
    sizes.insert("".to_string(), 0);

    for line in str.lines().clone() {
        let splitted_line: Vec<String> = line.split(" ").map(
            |x| x.to_string()
        ).collect();

        if splitted_line[0] == "$" && splitted_line[1] == "cd" {
            if splitted_line[2] == "/" {
                curr_path = VecDeque::new();
            } else if splitted_line[2] == ".." {
                curr_path.pop_back();
            } else {
                curr_path.push_back(splitted_line[2].clone());
                sizes.insert(
                    Vec::from(curr_path.clone()).join("/"),
                    0
                );
            }

        } else if splitted_line[0] == "$" || splitted_line[0] == "dir" {
            continue;
        }else { // if anything with a size is given

            // add this size to all directories in the path
            for n in 0..curr_path.len()+1{
                let sliced_vec= Vec::from(curr_path.clone())[..n].to_vec();
                let size = splitted_line[0].parse::<i32>().unwrap();
                let key = sliced_vec.join("/");
                *sizes.get_mut(&*key).unwrap() += size;
            }
        }
    }

    let mut sum = 0;
    for (s_k, s_val) in sizes.clone(){
        if s_val <= 100000{
            sum += s_val
        }
    }
    println!("Result: {}", sum);

    let needed = sizes.clone()[""] - 40000000;
    let mut allowed_vals = Vec::new();
    for (s_k, s_val) in sizes.clone(){
        if s_val > needed{
            allowed_vals.push(s_val);
        }
    }
    let minimum = allowed_vals.iter().min().unwrap();
    println!("Result: {}", minimum);
}
