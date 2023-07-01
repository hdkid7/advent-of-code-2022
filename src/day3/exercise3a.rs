use std::fs::File;
use std::io::BufReader;
use std::io::Lines;
use std::io::Result;
use std::path::Path;
use std::io::BufRead;

fn main() {
    let mut sum = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ref bag) = line {
                let arr = compartment_splitter(bag);
                let common_item = find_common_item(arr.clone());

                sum += get_score(&common_item);
                println!("{:?}", arr.clone());
            }
        }
    }

    println!("{sum}");

}

fn get_score(common_item: &char) -> u32 {
    let ascii_num = *common_item as u32;

    match  ascii_num as u32 {
        97..=122 => ascii_num - 96,
        65..=90 => (ascii_num - 64) + 26,
        _ => 0
    }
}

fn find_common_item(compartment: (String, String)) -> char {
    let compart_1 = compartment.0.chars();
    let compart_2 = compartment.1.chars().collect::<Vec<char>>();


    for char in compart_1 {
        if compart_2.contains(&char) {
            return char;
        }
    }

    return '9';
}

fn compartment_splitter(line : &str) -> (String,String) {
    let line_count = line.len();
    let line_count_half = line.len()/2;

   (line[..line_count_half].to_string(), line[line_count_half..line_count].to_string())
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}



/*
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ref cals) = line {
                let mut split = cals.split(' ');

                let my_tuple = new_code((split.next().unwrap().chars().next().unwrap(), split.next().unwrap().chars().next().unwrap()));

                sum += calculate_score(my_tuple);
            }
        }
    }
*/
