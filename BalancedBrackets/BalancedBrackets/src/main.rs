use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'isBalanced' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */
fn isBalanced(s: &str) -> String {
    let mut stack: Vec<char> = Vec::new();

    for ch in s.chars() {
        match ch {
            '(' | '{' | '[' => {
                stack.push(ch);
            }

            ')' | '}' | ']' => {
                let top = stack.pop();

                match (top, ch) {
                    (Some('('), ')') => {}
                    (Some('{'), '}') => {}
                    (Some('['), ']') => {}
                    _ => return String::from("NO"),
                }
            }

            _ => {}
        }
    }

    if stack.is_empty() {
        String::from("YES")
    } else {
        String::from("NO")
    }
}
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let s = stdin_iterator.next().unwrap().unwrap();

        let result = isBalanced(&s);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
