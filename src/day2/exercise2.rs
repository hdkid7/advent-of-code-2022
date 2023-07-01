use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ref cals) = line {
                let mut split = cals.split(' ');

                let my_tuple = new_code((split.next().unwrap().chars().next().unwrap(), split.next().unwrap().chars().next().unwrap()));

                sum += calculate_score(my_tuple);
            }
        }
    }

    println!("{}", sum);
}

fn new_code(round: (char, char)) -> (char, char) {
    let (elf_action, player_action) = round;

    let mut new_action = '?';

    if player_action == 'X' {
        match elf_action {
            'A' => new_action = 'Z',
            'B' => new_action = 'X',
            'C' => new_action = 'Y',
             _ => new_action = panic!("IMPOSSIBLE VALUE")
        };
    } else if player_action == 'Y' {
        match elf_action {
            'A' => new_action = 'X',
            'B' => new_action = 'Y',
            'C' => new_action = 'Z',
             _ => new_action = panic!("IMPOSSIBLE VALUE")
        };
    }else if player_action == 'Z' {
        match elf_action {
            'A' => new_action = 'Y',
            'B' => new_action = 'Z',
            'C' => new_action = 'X',
            _ => new_action = panic!("IMPOSSIBLE VALUE")
        };
    }

    (round.0, new_action)
}

fn calculate_score(round: (char, char)) -> i32 {
    let (_elf_choice, player_choice) = round;

    let shape_chosen_map = HashMap::from([
        ('X', 1),
        ('Y', 2),
        ('Z', 3)
    ]);

    let shape_chosen_score = shape_chosen_map.get(&player_choice).unwrap();

     shape_chosen_score + calculate_round_score(round)
}

fn calculate_round_score(round: (char, char)) -> i32 {
    let elf_choice = rock_paper_conversion(round.0);
    let player_choice = rock_paper_conversion(round.1);

    if elf_choice == 1 {
        return match player_choice {
            1 => 3,
            2 => 6,
            3 => 0,
            _ => 0
        };
    }else if elf_choice == 2 {
        return match player_choice {
            1 => 0,
            2 => 3,
            3 => 6,
            _ => 0
        };
    }else if elf_choice == 3 {
        return match player_choice {
            1 => 6,
            2 => 0,
            3 => 3,
            _ => 0
        };
    }

    0
}

fn rock_paper_conversion(choice: char) -> i32 {
    let shape_chosen_map = HashMap::from([
        ('X', 1),
        ('Y', 2),
        ('Z', 3),
        ('A', 1),
        ('B', 2),
        ('C', 3)
    ]);

    *shape_chosen_map.get(&choice).unwrap()
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
