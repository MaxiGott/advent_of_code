extern crate core;


fn decide_game(opp: &str, my: &str) -> i32{
    let mut points = 0;
    if opp == "A"{
        if my == "X"{
            points = 1 + 3; // Stone vs, Stone
        }
        else if my == "Y" {
            points = 2 + 6; // Stone vs. Paper
        }
        else{
            points = 3 + 0 // Stone vs. Scissor
        }
    }
    else if opp == "B" {
        if my == "X"{
            points = 1 + 0; // Paper vs. Stone
        }
        else if my == "Y" {
            points = 2 + 3; // Paper vs. Paper
        }
        else{
            points = 3 + 6 // Paper vs. Scissor
        }
    }
    else {
        if my == "X"{
            points = 1 + 6; // Scissor vs Stone
        }
        else if my == "Y" {
            points = 2 + 0; // Scissor vs Paper
        }
        else{
            points = 3 + 3; // Scissor vs. Scissor
        }
    }
    points
}

fn decide_game_2(opp: &str, my: &str) -> i32{
    let mut points = 0;

    if opp == "A"{
        if my == "X"{
            points = 3 + 0; // loose with scissor
        }
        else if my == "Y" {
            points = 1 + 3; // Draw with Stone
        }
        else{
            points = 2 + 6 // Win with Paper
        }
    }
    else if opp == "B" {
        if my == "X"{
            points = 1 + 0; // loose with rock
        }
        else if my == "Y" {
            points = 2 + 3; // Draw with Paper
        }
        else{
            points = 3 + 6 // Win with Scissor
        }
    }
    else {
        if my == "X"{
            points = 2 + 0; // loose with paper
        }
        else if my == "Y" {
            points = 3 + 3; // Draw with scissor
        }
        else{
            points = 1 + 6 // Win with stone
        }
    }
    points
}


pub fn show_results(){
    println!("Day 2: ");

    let input_res = crate::utils::helper::read_file_string("data/input_2.txt");
    let input = match input_res {
       Ok(value) => value,
       Err(err) => panic!("{}", err)
    };
    let stripped_input = input.trim(); // remove empty lines at beginning and end

    let games = stripped_input.split("\n");
    let mut total_points_first = 0;
    let mut total_points_second = 0;
    for game in games{
        let choices: Vec<&str>= game.split(" ").collect();
        let opponent_choice = choices[0];
        let my_choice = choices[1];
        total_points_first += decide_game(opponent_choice, my_choice);
        total_points_second += decide_game_2(opponent_choice, my_choice);
    }

    println!("Total Points first Round: {}", total_points_first);
    println!("Total Points second Round: {}", total_points_second);
}
