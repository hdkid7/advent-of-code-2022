use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::iter::FromIterator;
use std::fs::read_to_string;

//DAY 2 
//

// Datastructure
// id {"red":3, "green": 5, "blue": 7}


#[derive(Clone)]
#[derive(Debug)]
struct Game {
    red: u32,
    blue: u32,
    green: u32
}

fn main() {
    let lines: Vec<String> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let comparer: Game = Game {
        red: 12,
        green: 13,
        blue: 14
    };

    let mut sum = 0;

    for x in lines {
        let arr: Vec<Game> = create_game(x.to_string());

        sum += get_power_of_set(get_min_games_arr(arr));

    }

    println!("{}", sum);
}

fn get_min_games_arr(game_arr: Vec<Game>) -> Game {
    let mut base_game: Game = Game {red: 0, green: 0, blue: 0};

    for game in game_arr {
        if(game.green > base_game.green) {
            base_game = Game {green: game.green, ..base_game};
        }

        if(game.red > base_game.red) {
            base_game = Game {red: game.red, ..base_game};
        }

        if(game.blue > base_game.blue) {
            base_game = Game {blue: game.blue, ..base_game};
        }

    }

    base_game
}

fn get_power_of_set(game: Game) -> u32 {
    game.red * game.blue * game.green
}

fn create_game(line_input: String) -> Vec<Game>  {
    let string_arr: Vec<&str> = line_input.split(":").map(|x| x.trim()).collect();

    let games_arr: Vec<&str> = string_arr[1].split(";").map(|x| x.trim()).collect();
    
    create_game_arr(games_arr)
}

fn create_game_arr(string_input_arr: Vec<&str>) -> Vec<Game>  {

    let mut result: Vec<Game> = [].to_vec();

    for string_input in string_input_arr {
        let mut hash_map: HashMap<&str, u32> = HashMap::from_iter([("red", 0), ("green", 0), ("blue", 0)]);
        let color_str_arr: Vec<&str> = string_input.split(",").map(|x| x.trim()).collect();

        for color_str in color_str_arr {
            let arr: Vec<&str> = color_str.split(" ").collect();
            let number = arr[0].parse::<u32>().unwrap();
            let color = arr[1];

            *hash_map.entry(color).or_insert(number) += number;
        }

        let game = Game {
            red: *hash_map.get("red").unwrap(),
            green: *hash_map.get("green").unwrap(),
            blue: *hash_map.get("blue").unwrap()
        };

        result.push(game);
    }

    result
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
