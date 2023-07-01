use std::fs::File;
use std::io::BufReader;
use std::io::Lines;
use std::io::Result;
use std::path::Path;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let mut sum = 0;
    let mut sack_group = Vec::with_capacity(3);

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ref bag) = line {
                if sack_group.len() == 2 {
                    sack_group.push(bag.to_string());
                    let common_item = find_common_item(&sack_group);
                    sum += get_score(&common_item);
                    sack_group = Vec::with_capacity(3);
                } else {
                    sack_group.push(bag.to_string());
                }

            }
        }
    }

    println!("{}", sum);
}

fn get_score(common_item: &char) -> u32 {
    let ascii_num = *common_item as u32;

    match  ascii_num as u32 {
        97..=122 => ascii_num - 96,
        65..=90 => (ascii_num - 64) + 26,
        _ => 0
    }
}

fn find_common_item(compartment: &Vec<String>) -> char {
    let compart_1 = &compartment[0];
    let char_arr = compart_1.chars();

    let compart_2 = create_present_map(&compartment[1]);
    let compart_3 = create_present_map(&compartment[2]);

    for char in char_arr {
        if let (Some(_), Some(_)) = (compart_2.get(&char), compart_3.get(&char)) {
            return char;
        }
    }

    return '0';
}

fn create_present_map(compartment: &str) -> HashMap<char, u16> {
    let char_arr = compartment.chars();
    let mut res = HashMap::new();

    for char in char_arr {
        res.insert(char,1);
    }

    res
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
