extern crate core;

use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

fn has_unique_elements<T>(iter: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + Hash,
    {
        let mut uniq = HashSet::new();
        iter.into_iter().all(move |x| uniq.insert(x))
    }



fn find_start(str: &str) -> i32{
    let mut buffer = VecDeque::new();
    let mut counter = 0;

    for char in str.chars(){
        buffer.push_back(char);
        counter += 1;

        if buffer.len() > 14{
            buffer.pop_front();
            if has_unique_elements(buffer.iter()){
                break
            }
        }
    }

    counter
}


pub fn show_results(){
    println!("Day 6: ");

    let input_res = crate::utils::helper::read_file_string("data/input_6.txt");
    let input = match input_res {
        Ok(value) => value,
        Err(err) => panic!("{}", err)
    };
    let str = input.trim(); // remove empty lines at beginning and end

    let start = find_start(str);
    println!("{}", start)

}

