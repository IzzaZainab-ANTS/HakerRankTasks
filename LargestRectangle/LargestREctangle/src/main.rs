use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'largestRectangle' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts INTEGER_ARRAY h as parameter.
 */

fn largestRectangle(h: &[i32]) -> i64 {
    let n = h.len();
    let mut max_area: i64 = 0;

    for i in 0..n {
        let mut min_height = h[i];

        for j in i..n {
            if h[j] < min_height {
                min_height = h[j];
            }

            let width = (j - i + 1) as i64;
            let area = min_height as i64 * width;

            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}


 fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let h: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = largestRectangle(&h);

    writeln!(&mut fptr, "{}", result).ok();
}