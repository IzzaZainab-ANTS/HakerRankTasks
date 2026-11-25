use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'minimumDistances' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY a as parameter.
 */

use std::collections::HashMap;

fn minimumDistances(a: &[i32]) -> i32 {
    let mut last_seen: HashMap<i32, usize> = HashMap::new();
    let mut min_distance = i32::MAX;

    for index in 0..a.len() {
        let value = a[index];

        if let Some(&previous_index) = last_seen.get(&value) {
            let distance = index - previous_index;

            if distance < min_distance as usize {
                min_distance = distance as i32;
            }
        }

        last_seen.insert(value, index);
    }

    if min_distance == i32::MAX {
        -1
    } else {
        min_distance
    }
}
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = minimumDistances(&a);

    writeln!(&mut fptr, "{}", result).ok();
}
