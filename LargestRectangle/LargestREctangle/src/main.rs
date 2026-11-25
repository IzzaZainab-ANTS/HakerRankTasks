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

    let mut stack: Vec<usize> = Vec::new();
    let mut max_area: i64 = 0;

    for i in 0..=n {
        let current_height = if i == n {
            0
        } else {
            h[i]
        };

        while let Some(&top_index) = stack.last() {
            if h[top_index] > current_height {
                stack.pop();

                let height = h[top_index] as i64;

                let width = if let Some(&previous_index) = stack.last() {
                    i - previous_index - 1
                } else {
                    i
                };

                let area = height * width as i64;

                max_area = max_area.max(area);
            } else {
                break;
            }
        }

        if i < n {
            stack.push(i);
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