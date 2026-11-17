use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */
 
fn miniMaxSum(arr: &[i32]) {
    let mut total: i64 = 0;

    let mut minimum = i64::from(arr[0]);
    let mut maximum = i64::from(arr[0]);

    for element in arr {
        let value = i64::from(*element);

        total += value;

        if value < minimum {
            minimum = value;
        }

        if value > maximum {
            maximum = value;
        }
    }

    let minimum_sum = total - maximum;
    let maximum_sum = total - minimum;

    println!("{} {}", minimum_sum, maximum_sum);
}
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
