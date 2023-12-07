use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use io::BufReader;
use std::fs::read_to_string;
use std::collections::HashMap;


fn main() -> std::io::Result<()> {
    let mut sum = 0;
    
    let lines: Vec<String> = read_to_string("test.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    for x in &lines {
        let num_arr : Vec<u32> = extract_first_last_num(&convert_str_to_num_ver(x));
        let num = convert_arr_to_num(num_arr.clone());
        println!("INPUT: {} NUM: {}", x, num);
        sum += num;
    }

    println!("{}", sum);
    Ok(())
}

fn convert_str_to_num_ver(input_line : &str) -> String {
    let num_str_arr = [
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"
    ];

    let replacements = HashMap::from(
     [
        ("one","one1one"),
        ("two","two2two"),
        ("three","three3three"),
        ("four","four4four"),
        ("five","five5five"),
        ("six","six6six"),
        ("seven","seven7seven"),
        ("eight","eight8eight"),
        ("nine","nine9nine")
    ]);

    let mut str_buffer = String::from("");
    let mut result = input_line.to_string();

    let mut is_skip = false;

    let values: Vec<char> = input_line
        .chars()
        .collect();

        for num_str in num_str_arr {
             result = result.replace(num_str, replacements.get(num_str).unwrap());
        }

    result

    //example: seven234qwerone
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn convert_arr_to_num(arr : Vec<u32>) -> u32 {

    let string = arr.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");

    string.parse().unwrap()
}


fn extract_first_last_num(input_line : &str) -> Vec<u32> {
    let values: Vec<u32> = input_line
        .chars()
        .filter(|&x| x.is_numeric())
        .map(|x| x.to_digit(10).unwrap())
        .collect();


    [values[0] as u32, values[values.len() - 1] as u32].to_vec()
}
