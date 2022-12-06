extern crate core;


use std::collections::VecDeque;
use std::mem;

fn split_regularly(text: &str, idx: usize) -> Vec<&str>{
    let mut collection = Vec::new();
    let mut copy_string = text.clone();

    while copy_string.len() >= idx{
        let _s = copy_string.split_at(idx).0;
        copy_string = &copy_string[idx..];
        collection.push(_s.trim());
    }
    collection.push(copy_string.trim());
    collection
}

fn run(stacks: Vec<VecDeque<String>>, rearrangement: Vec<Vec<i32>>, old: bool){
    let mut stacks_copy = stacks.clone();

    for command in rearrangement{
        let n = command.get(0).unwrap().clone() as usize;
        let f = command.get(1).unwrap().clone() as usize - 1;
        let t = command.get(2).unwrap().clone() as usize - 1;

        let mut t_stack = stacks_copy.get(t).unwrap().clone();
        let mut f_stack = stacks_copy.get(f).unwrap().clone();

        if !old{
            for _ in 0..n{
                t_stack.push_front(f_stack.front().unwrap().clone());
                f_stack.pop_front();
            }
        }else {
            let mut buffer = Vec::new();
            for _ in 0..n{
                buffer.push(f_stack.front().unwrap().clone());
                f_stack.pop_front();
            }
            buffer.reverse();
            for elt in buffer{
                t_stack.push_front(elt);
            }
        }

        let _ = mem::replace(&mut stacks_copy[t], t_stack);
        let _ = mem::replace(&mut stacks_copy[f], f_stack);

    }
    let mut out = format!("");
    for stack in stacks_copy{
        let top = stack.front().unwrap();
        out = format!("{}{}", &out, top);
    }
    println!("{}", out.replace("[", ""). replace("]", ""))

}


pub fn show_results(){
    println!("Day 5: ");
    let input_res = crate::utils::helper::read_file_string("data/input_5.txt");
    let input = match input_res {
        Ok(value) => value,
        Err(err) => panic!("{}", err)
    };
    let text = input.trim(); // remove empty lines at beginning and end

    let mut crates = Vec::new();

    for line in text.lines(){
        if line == ""{
            break
        }else{
            crates.push(line);
        }
    }

    let mut ids = Vec::new();

    for id in crates.last().unwrap().split(" "){
        if id != ""{
            let id_int = id.parse::<i32>().unwrap();
            ids.push(id_int)
        }
    }

    let mut stacks: Vec<VecDeque<String>>= vec![];
    for _ in ids{
        stacks.push(VecDeque::new());
    }

    for h in &crates[..crates.len()-1]{
        let splitted_line = split_regularly(h, 4);
        for (id, split) in splitted_line.iter().enumerate(){
            let mut t_c = stacks.get(id).unwrap().clone(); // get old value
            // if new value must be added
            if split.clone() != ""{
                t_c.push_back(split.to_string());
                let _ = mem::replace(&mut stacks[id], t_c);
            }
        }
        println!("{}", h)
    }
    println!("Reading instructions...");

    let mut rearrangement = Vec::new();
    for line in text.lines(){
        if line.starts_with("move"){
            let splits = line.split(" ").collect::<Vec<&str>>();
            rearrangement.push(
                vec![
                    splits.get(1).unwrap().parse::<i32>().unwrap(),
                    splits.get(3).unwrap().parse::<i32>().unwrap(),
                    splits.get(5).unwrap().parse::<i32>().unwrap()
                ])
        }
    }
    // part 1
    let stacks_copy_1 = stacks.clone();
    let rearrangement_copy_1 = rearrangement.clone();

    run(stacks_copy_1, rearrangement_copy_1, false);

    // part 2
    let stacks_copy = stacks.clone();
    let rearrangement_copy = rearrangement.clone();

    run(stacks_copy, rearrangement_copy, true);
}

