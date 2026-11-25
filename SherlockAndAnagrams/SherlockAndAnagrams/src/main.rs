use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;
/*
 * Complete the 'sherlockAndAnagrams' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING s as parameter.
 *         Plan for doing this 

            Extract the substring and create an owned copy. 
             Sort its characters.
             Find its previous frequency.
            Every previous occurrence creates one new pair.
             Record the current occurrence.
  */

fn sherlockAndAnagrams(s: &str) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();

    let mut frequencies: HashMap<Vec<char>, i32> = HashMap::new();
    let mut pairs = 0;

    for start in 0..n {
        for end in start + 1..=n {
            let mut key = chars[start..end].to_vec();

            key.sort();

            let count = frequencies.entry(key).or_insert(0);

            pairs += *count;
            *count += 1;
        }
    }

    pairs
}
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let q = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..q {
        let s = stdin_iterator.next().unwrap().unwrap();

        let result = sherlockAndAnagrams(&s);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
