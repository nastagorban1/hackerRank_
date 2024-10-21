use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'lowestTriangle' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER trianglebase
 *  2. INTEGER area
 */

fn lowestTriangle(trianglebase: i32, area: i32) -> i32 {
let  height =(2 as f64 *area as f64/trianglebase as f64).ceil() as i32 ;
height
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let trianglebase = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let area = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let height = lowestTriangle(trianglebase, area);

    writeln!(&mut fptr, "{}", height).ok();
}
