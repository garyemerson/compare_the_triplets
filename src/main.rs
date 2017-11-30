use std::io;

fn main() {
    let nums_a: Vec<i32> = get_line()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let nums_b: Vec<i32> = get_line()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let mut score_a = 0;
    let mut score_b = 0;
    for i in 0..3 {
        if nums_a[i] > nums_b[i] {
            score_a += 1;
        } else if nums_a[i] < nums_b[i] {
            score_b += 1;
        }
    }
    println!("{} {}", score_a, score_b);
}

fn get_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s
}
