extern crate core;


pub fn show_results(){
    println!("Day 4: ");

    let input_res = crate::utils::helper::read_file_string("data/input_4.txt");
    let input = match input_res {
       Ok(value) => value,
       Err(err) => panic!("{}", err)
    };
    let sections = input.trim(); // remove empty lines at beginning and end
    // let sections = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
    // first part
    let mut counter = 0;
    let mut counter_ = 0;

    for sections_pair in sections.split("\n"){
        let elfs: Vec<&str> = sections_pair.split(",").collect();
        let range_1 = elfs[0].split("-").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let range_2 = elfs[1].split("-").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        if (range_1[0] <= range_2[0] && range_1[1] >= range_2[1]) ||
            (range_2[0] <= range_1[0] && range_2[1] >= range_1[1]){
            counter += 1;
        }

        if (range_1[0] <= range_2[1] && range_1[1] >= range_2[0]) ||
            (range_2[0] <= range_1[1] && range_2[1] >= range_1[0]) {
            counter_ += 1;
        }

    println!("Result 1: {}", counter);
    println!("Result 2: {}", counter_);

    }



}

