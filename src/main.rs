fn main() {
    let bytes = 
    include_str!("lol.txt").split("\n\n").map(|x| x.lines());

    print!("{:?}", bytes);
}

