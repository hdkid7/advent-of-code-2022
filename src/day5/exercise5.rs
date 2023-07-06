use std::collections::HashMap;
use std::io::BufReader;
use std::io::Lines;
use std::io::Result;
use std::path::Path;
use std::io::BufRead;
use std::fs::File;

fn main() {
    get_empty_stack_map();
}


fn get_empty_stack_map() -> HashMap<u32, Vec<char>> {
    let mut stack_map: HashMap<u32, Vec<char>> = HashMap::new();

    let mut letter_arr: Vec<String> = Vec::new();
    let mut str_arr: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if line.as_ref().unwrap() == "" {
                break;
            }

            str_arr.push(line.unwrap());
        }
    }

    str_arr.reverse();

    let max_stack: char = str_arr[0].replace(" ", "").chars().last().unwrap(); 
    let max = max_stack.to_digit(10).unwrap();

    for i in 1..=max {
        stack_map.insert(i, Vec::new());
    };

    for i in 1..str_arr.len() {
        //TODO: use letter arr to populate stack map :)
        let letter_arr_1 = str_arr[i].replacen("[","",1).chars().step_by(4).collect::<Vec<char>>();
        println!("{:?}",letter_arr_1);
    
    }


    stack_map
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
