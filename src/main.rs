use std::io::{self, BufRead};

/*
 * Complete the 'plus_minus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plus_minus(arr: &[i32]) {
    let mut pos: f32 = 0.0;
    let mut neg: f32 = 0.0;
    let mut zer: f32 = 0.0;
    let len: f32 = arr.len() as f32;
    for num in arr {
        if num > &0 {
            pos += 1.0;
        }
        if num == &0 {
            zer += 1.0;
        }
        if num < &0 {
            neg += 1.0;
        }
    }

    println!("{:.6}", pos / len);
    println!("{:.6}", neg / len);
    println!("{:.6}", zer / len);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plus_minus(&arr);
}
