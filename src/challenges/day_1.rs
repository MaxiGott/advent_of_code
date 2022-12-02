extern crate core;

struct Elf {
    elf_id: usize,
    calories: i32
}

impl Elf {
    pub fn new(id: usize, calories: i32) -> Self {
        Elf {
            elf_id: id,
            calories
        }
    }
}

pub fn show_results(){
    // read file
    let input_res = crate::utils::helper::read_file_string("data/input_1.txt");
    let input = match input_res {
        Ok(value) => value,
        Err(err) => panic!("{}", err)
    };
    let stripped_input = input.trim(); // remove empty lines at beginning and end
    
    let mut elfs_matching = Vec::new();

    // iterate over all elf blocks
    for (idx, elf) in stripped_input.split("\n\n").enumerate() {
        let mut elfs_sum= 0;
        for cal in elf.split("\n"){
            let cal_as_int = cal.parse::<i32>().unwrap();  // string to integer
            elfs_sum += cal_as_int;
        }
        // create new elf with id and sum of calories
        let elf = Elf::new(idx, elfs_sum);
        elfs_matching.push(elf);
    }

    // sort list by calories (descending)
    elfs_matching.sort_by(|a, b| b.calories.cmp(&a.calories));

    // get elf with highest calories
    let fattest_elf = elfs_matching.first().unwrap();
    println!("Most calories: {}", fattest_elf.calories);

    // get the top three calories
    let best_three = elfs_matching.iter().take(3);
    let mut total_sum = 0;
    for elf in best_three{
        total_sum += elf.calories;
    }
    println!("Sum of the top three calories: {}", total_sum);
}
