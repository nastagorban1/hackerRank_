use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plusMinus(arr: &[i32]) {
   let mut p_count = 0;
   let mut n_count = 0;
   let  mut z_count = 0;
    
    let n = arr.len() as f64 ;
    
    for &i in arr { 
        if i > 0 {
            p_count += 1;
        } else if i < 0 {
            n_count += 1; 
        } else {
            z_count += 1; 
        }
    }
        
    
    let counts = [p_count as f64, n_count as f64, z_count as f64]; 
    for &m in &counts {
        let result = m / n; 
          println!("{:.6}", result);
         }

}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}
