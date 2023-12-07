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
        let game_id = get_game_id(&x);
        let arr: Vec<Game> = create_game(x.to_string());
        let result:bool = is_game_possible(arr, comparer.clone());

        if result {
            sum += game_id
        }

    }

    println!("{}", sum);
}

fn is_game_possible(game_arr: Vec<Game>, comparer: Game) -> bool {
    for game in game_arr{
        if comparer.red < game.red || comparer.green < game.green || comparer.blue < game.blue {
            return false;
        }
    }

    true
}

fn get_game_id(line_input: &str) -> u32 {
    let string_arr: Vec<&str> = line_input.split(":").map(|x| x.trim()).collect();
    string_arr[0].replace("Game ","").parse::<u32>().unwrap()
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

    println!("{:?}", result);

    result
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
