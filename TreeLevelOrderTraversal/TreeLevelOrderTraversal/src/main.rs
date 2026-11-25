use std::io::{self, Read};

struct Node {
    data: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn insert(root: &mut Option<Box<Node>>, value: i32) {
    match root {
        Some(node) => {
            if value < node.data {
                insert(&mut node.left, value);
            } else if value > node.data {
                insert(&mut node.right, value);
            }
        }

        None => {
            *root = Some(Box::new(Node {
                data: value,
                left: None,
                right: None,
            }));
        }
    }
}

// Perform level-order traversal using BFS.
fn level_order(root: &Node) {
    let mut queue: Vec<&Node> = Vec::new();

    queue.push(root);

    let mut front = 0;
    let mut first = true;

    while front < queue.len() {
        let current = queue[front];
        front += 1;

        if !first {
            print!(" ");
        }

        print!("{}", current.data);
        first = false;

        if let Some(left_child) = &current.left {
            queue.push(left_child.as_ref());
        }

        if let Some(right_child) = &current.right {
            queue.push(right_child.as_ref());
        }
    }

    println!();
}

fn main() {
    
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut numbers = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap());

    
    let n = numbers.next().unwrap() as usize;

    
    let mut root: Option<Box<Node>> = None;

    for _ in 0..n {
        let value = numbers.next().unwrap();
        insert(&mut root, value);
    }

    if let Some(root_node) = root.as_deref() {
        level_order(root_node);
    }
}
