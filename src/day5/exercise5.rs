use std::collections::HashMap;
use std::io::BufReader;
use std::io::Lines;
use std::io::Result;
use std::path::Path;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let mut stack_map = get_populated_stack_map();

    // Loop thru txt file here
    // end loop

    let mut is_passed_line = false;

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ref command) = line {
                if command == "" {
                    is_passed_line = true;
                    continue;
                }
                if ! is_passed_line{
                    continue;
                }
                if is_passed_line {
                    println!("{command}");
                    println!("{:?}", stack_map);
                    let command2: MoveBox = parse_string(command);
                    move_box(&mut stack_map, command2);
                }
            }
        }
    }
    println!("{}", get_solution_str(stack_map));
}

#[derive(Debug)]
struct MoveBox {
    src: u32,
    dst: u32,
    amt: u32,
}

fn parse_string(command: &str) -> MoveBox  {
    let mut res = String::new();

    for x in command.chars() {
        if x.is_ascii_digit() {
            res.push(x);
        } else if x == ' ' {
            res.push(x);
        }
    }

    let arr : Vec<u32> = res.split(' ').filter(|x| x != &"").map(|x| x.parse::<u32>().unwrap()).collect();

    MoveBox{
        src: arr[1],
        amt: arr[0],
        dst: arr[2]
    }
}

fn get_solution_str(mut final_box: HashMap<u32, Vec<char>>) -> String {
    let mut res = String::new();

    for i in 1..final_box.len() + 1 {
        let stack = final_box.get_mut(&(i as u32)).unwrap();
        let char = stack.pop().unwrap();

        res.push(char);
    }

    res
}

fn move_box(boxes: &mut HashMap<u32, Vec<char>>, boxData: MoveBox) {
    println!("{}", boxData.src);
    let mut srcStack = boxes.get_mut(&boxData.src).unwrap();
    let mut buffer = Vec::new();

    for i in 1..=boxData.amt {
        let boxChar = srcStack.pop().unwrap();
        buffer.push(boxChar);
    }

    let mut dstStack = boxes.get_mut(&boxData.dst).unwrap();

    for i in buffer {
        dstStack.push(i);
    }
}

fn get_populated_stack_map() -> HashMap<u32, Vec<char>> {
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
    let mut vec = Vec::new();

    for i in 1..=max {
        stack_map.insert(i, vec.clone());
    };

    for i in 1..str_arr.len() {
        //TODO: use letter arr to populate stack map :)
        let letter_arr_1 = str_arr[i].replacen("[","",1).chars().step_by(4).collect::<Vec<char>>();

        for (idx, val) in letter_arr_1.iter().enumerate() {
            if val != &' ' {
                stack_map.get_mut(&(idx as u32 + 1)).unwrap().push(*val);
            }
        }
    }

    stack_map
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
