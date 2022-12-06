extern crate core;

use std::collections::HashSet;
use std::ptr::null;
use regex::Regex;


pub fn intersection(nums: Vec<Vec<char>>) -> Vec<char> {
    let mut intersect_result: Vec<char> = nums[0].clone();

    for temp_vec in nums {
        let unique_a: HashSet<char> = temp_vec.into_iter().collect();
        intersect_result = unique_a
            .intersection(&intersect_result.into_iter().collect())
            .map(|i| *i)
            .collect::<Vec<_>>();
    }
    intersect_result
}

fn eval_char(common_char: char) -> u32{
    let mut result = 0;
    if common_char.is_ascii_lowercase(){
        result = common_char as u32 - 96;
    }
    else if common_char.is_ascii_uppercase() {
        result = common_char as u32 - 65 + 27;
    }else{
        panic!("Something went wrong here.");
    }
    result
}

pub fn show_results(){
    println!("Day 3: ");

    let input_res = crate::utils::helper::read_file_string("data/input_3.txt");
    let input = match input_res {
       Ok(value) => value,
       Err(err) => panic!("{}", err)
    };
    let rucksacks = input; // remove empty lines at beginning and end
    // first part
    let mut sum_for_rucksack = 0;
    for rucksack in rucksacks.split("\n"){
        if rucksack == ""{
            break
        }
        let (compartment_1, compartment_2)  = rucksack.split_at(rucksack.len()/2);
        let c1: Vec<char>= compartment_1.chars().collect();
        let c2: Vec<char> = compartment_2.chars().collect();
        let mut common_char:char = '\0';

        for s_c in c1{
            if c2.contains(&s_c){
                common_char = s_c;
                break;
            }
        }
        sum_for_rucksack += eval_char(common_char);
    }

    // second part
    let mut sum_for_rucksack_2 = 0;
    let re = Regex::new(r".*\n.*\n.*\n").unwrap();
    let groups: Vec<&str> = re.find_iter(&rucksacks).map(|m| m.as_str()).collect();

    for group in groups {
        let mut list_of_rucksacks = Vec::new();
        for g in group.split("\n"){
            let as_chars:Vec<char> = g.chars().collect();
            if as_chars.len() > 0{
                list_of_rucksacks.push(as_chars);
            }

        }
        let same_item = intersection(list_of_rucksacks);
        if same_item.len() > 1 || same_item.len() == 0{
            panic!("Something went wrong.")
        }else{
            sum_for_rucksack_2 += eval_char(same_item[0]);
        }

    }

    println!("{}", sum_for_rucksack);
    println!("{}", sum_for_rucksack_2);

}
