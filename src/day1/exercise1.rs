use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts.txt must exist in the current path
    let mut sum = 0;
    let mut top_3_arr: [i32; 3] = [0; 3];

    if let Ok(lines) = read_lines("./hosts.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {

            if let Ok(ref cals) = line {
                if cals == "" {
                    if sum > top_3_arr[0] {
                        top_3_arr[0] = sum;
                        top_3_arr.sort();
                    }
                    sum = 0;
                } else {
                    let num_of_cals = cals;
                    let num_of_cals = num_of_cals.parse::<i32>().unwrap();

                    sum = sum + num_of_cals;
                }
            }
        }
    }

    println!("MOST CALS {:?}", top_3_arr);
    println!("MOST CALS {}", top_3_arr.iter().sum::<i32>());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
