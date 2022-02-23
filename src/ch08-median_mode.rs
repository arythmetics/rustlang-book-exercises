use std::io;
use std::collections::HashMap;

fn input_and_add(n_vec: &mut Vec<i32>) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    if n > 0 {
        n_vec.push(n);
        println!("'another one!' - DJ Khaled");
        input_and_add(n_vec);
    } else {
        n_vec.sort_unstable();
    } 
}

fn median_calc(n_vec:&mut Vec<i32>) -> i32 {
    let median_index = n_vec.len()/2;
    return n_vec[median_index]
}

fn mode_calc(n_vec:&mut Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    
    for n in n_vec.iter() {
        let count = map.entry(n).or_insert(0);
        *count += 1;
    };

    *map
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}

fn main() {
    let mut num_list: Vec<i32> = Vec::new();
    println!("Enter a number to add to the list");
    println!("When you're done adding, enter: -1");
    input_and_add(&mut num_list);

    let median: i32 = median_calc(&mut num_list);
    let mode: i32 = mode_calc(&mut num_list);

    println!("Le Mode: {}", mode);
    println!("Le Median: {}", median);
}
