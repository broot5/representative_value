use std::collections::HashMap;

fn main() {
    let v = vec![30, 92, 100, 53, 42, 60, 49];

    let average = get_average(&v);
    let median = get_median(&v);
    let mode = get_mode(&v);
    println!("{} {} {:?}", average, median, mode);
}

fn get_average(vec: &Vec<i32>) -> f64 {
    let sum: i32 = vec.iter().sum();
    sum as f64 / vec.len() as f64
}

fn get_median(vec: &Vec<i32>) -> f64 {
    let mut a = vec.clone();
    let length = a.len();
    a.sort();

    if length % 2 == 0 {
        (a[length / 2 - 1] + a[length / 2]) as f64 / 2.0
    } else {
        a[(length + 1) / 2 - 1] as f64
    }
}

fn get_mode(vec: &Vec<i32>) -> Vec<i32> {
    let mut score: HashMap<i32, i32> = HashMap::new();

    for numbers in vec {
        let count = score.entry(*numbers).or_insert(0);
        *count += 1;
    }

    let max_value = score.values().cloned().max().unwrap();

    score
        .into_iter()
        .filter(|&(_, v)| v == max_value)
        .map(|(k, _)| k)
        .collect()
}
