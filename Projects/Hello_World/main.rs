use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    
    let mut array = Vec::new();
    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let pi = parse_input!(input_line, i32);
        array.push(pi);
    }
    
    // Sort the array
    array.sort();

    // Initialize r with a large value
    let mut r = i32::MAX;

    // Calculate differences between consecutive elements
    for j in 0..(n - 1) as usize {
        let d = array[j + 1] - array[j];
        if d < r {
            r = d;
        }
    }

    // Output the minimum difference
    println!("{}", r);
}
