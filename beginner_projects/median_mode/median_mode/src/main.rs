fn main() {
    let v: Vec<i32> = vec![1, 1, 1, 0, 0, 0];

    let median_value = median(&v);

    let mode_value = mode(&v);

    println!("The median of the vector is: {}", median_value);
    println!("The mode of the vector is: {}", mode_value);
    
}

fn median(v: &Vec<i32>) -> f64 {
    let mut v = v.clone();
    v.sort();
    let len = v.len();
    if len % 2 == 0 {
        (v[len / 2 - 1] + v[len / 2]) as f64 / 2.0
    } else {
        v[len / 2] as f64
    }
}

fn mode(v: &Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut occurrences = HashMap::new();
    for &value in v {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences.into_iter().max_by_key(|&(_, count)| count).map_or(0, |(value, _)| value)
}