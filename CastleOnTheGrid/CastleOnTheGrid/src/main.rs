use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'minimumMoves' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. STRING_ARRAY grid
 *  2. INTEGER startX
 *  3. INTEGER startY
 *  4. INTEGER goalX
 *  5. INTEGER goalY
 */

 fn minimumMoves(
    grid: &[String],
    startX: i32,
    startY: i32,
    goalX: i32,
    goalY: i32,
) -> i32 {
    let n = grid.len();

    // Handle the case where start and destination are the same.
    if startX == goalX && startY == goalY {
        return 0;
    }

    // Track visited cells.
    let mut visited = vec![vec![false; n]; n];

    // Store the minimum number of moves to reach each cell.
    let mut distance = vec![vec![-1; n]; n];

    // BFS queue using Vec and a front index.
    let mut queue: Vec<(usize, usize)> = Vec::new();
    let mut front = 0;

    let start_x = startX as usize;
    let start_y = startY as usize;

    // Initialize the starting position.
    queue.push((start_x, start_y));
    visited[start_x][start_y] = true;
    distance[start_x][start_y] = 0;

    // Four possible directions.
    let directions: [(i32, i32); 4] = [
        (-1, 0), // Up
        (1, 0),  // Down
        (0, -1), // Left
        (0, 1),  // Right
    ];

    // Process the queue.
    while front < queue.len() {
        let (x, y) = queue[front];
        front += 1;

        // Check whether we have reached the destination.
        if x == goalX as usize && y == goalY as usize {
            return distance[x][y];
        }

        // Explore all four directions.
        for (dx, dy) in directions {
            let mut nx = x as i32 + dx;
            let mut ny = y as i32 + dy;

            // Continue moving in the selected direction.
            while nx >= 0 && nx < n as i32
                && ny >= 0 && ny < n as i32
            {
                let row = nx as usize;
                let col = ny as usize;

                // Stop if we encounter an obstacle.
                if grid[row].as_bytes()[col] == b'X' {
                    break;
                }

                // Process only cells we haven't visited.
                if !visited[row][col] {
                    visited[row][col] = true;

                    distance[row][col] = distance[x][y] + 1;

                    queue.push((row, col));
                }

                // Move to the next cell in the same direction.
                nx += dx;
                ny += dy;
            }
        }
    }
    -1
} 

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut grid: Vec<String> = Vec::with_capacity(n as usize);

    for _ in 0..n {
        let grid_item = stdin_iterator.next().unwrap().unwrap();
        grid.push(grid_item);
    }

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let startX = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let startY = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let goalX = first_multiple_input[2].trim().parse::<i32>().unwrap();

    let goalY = first_multiple_input[3].trim().parse::<i32>().unwrap();

    let result = minimumMoves(&grid, startX, startY, goalX, goalY);

    writeln!(&mut fptr, "{}", result).ok();
}
