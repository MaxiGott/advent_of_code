extern crate core;

use std::fs;


fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}

struct Elf {
    elf_id: i32,
    calories: i32
}

impl Elf {
    pub fn new(id: i32, calories: i32) -> Self {
        Elf {
            elf_id: id,
            calories
        }
    }
}


pub fn show_results_day_1(){
    let input_res = read_file_string("data/input_1.txt");
    let input = match input_res {
        Ok(value) => value,
        Err(err) => panic!("{}", err)
    };

    let mut elf_counter = 0;
    let mut elfs_matching = Vec::new();

    for elf in input.split("\n\n") {
        let mut cals_as_int = Vec::new();
        for cal in elf.split("\n"){
            if cal != "" {
                let cal_as_int = cal.parse::<i32>().unwrap();
                cals_as_int.push(cal_as_int);
            }
        }
        let sum: i32 = cals_as_int.iter().sum();
        let elf = Elf::new(elf_counter, sum);
        elfs_matching.push(elf);

        elf_counter += 1;

    }
    elfs_matching.sort_by(|a, b| b.calories.cmp(&a.calories));

    let fattest_elf = elfs_matching.first().unwrap();
    println!("Most calories: {}", fattest_elf.calories);

    let best_three = elfs_matching.iter().take(3);

    let mut total_sum = 0;
    for elf in best_three{
        total_sum += elf.calories;
    }
    println!("Sum of the top three calories: {}", total_sum);
}
