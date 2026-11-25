
use std::io::{self, BufRead};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn getCost(
    g_nodes: i32,
    g_from: &[i32],
    g_to: &[i32],
    g_weight: &[i32],
) {
    let n = g_nodes as usize;

    // Build adjacency list
    let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; n];

    for i in 0..g_from.len() {
        let u = (g_from[i] - 1) as usize;
        let v = (g_to[i] - 1) as usize;
        let w = g_weight[i];

        graph[u].push((v, w));
        graph[v].push((u, w));
    }

    // best[i] = minimum possible maximum edge weight
    // needed to reach node i
    let mut best = vec![i32::MAX; n];

    // Start from node 0 (station 1)
    best[0] = 0;

    // Min-heap
    let mut heap = BinaryHeap::new();

    heap.push(Reverse((0, 0usize)));

    while let Some(Reverse((cost, node))) = heap.pop() {
        // Ignore outdated entries
        if cost > best[node] {
            continue;
        }

        // Destination reached
        if node == n - 1 {
            println!("{}", cost);
            return;
        }

        // Explore neighboring stations
        for &(neighbor, weight) in &graph[node] {
            let new_cost = cost.max(weight);

            if new_cost < best[neighbor] {
                best[neighbor] = new_cost;

                heap.push(Reverse((new_cost, neighbor)));
            }
        }
    }

    // Destination is unreachable
    println!("NO PATH EXISTS");
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let g_nodes_edges: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let g_nodes = g_nodes_edges[0].trim().parse::<i32>().unwrap();
    let g_edges = g_nodes_edges[1].trim().parse::<i32>().unwrap();

    let mut g_from: Vec<i32> = Vec::with_capacity(g_edges as usize);
    let mut g_to: Vec<i32> = Vec::with_capacity(g_edges as usize);
    let mut g_weight: Vec<i32> = Vec::with_capacity(g_edges as usize);

    for _ in 0..g_edges {
        let g_from_to: Vec<String> = stdin_iterator.next().unwrap().unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let g_from_temp = g_from_to[0].trim().parse::<i32>().unwrap();
        let g_to_temp = g_from_to[1].trim().parse::<i32>().unwrap();
        let g_weight_temp = g_from_to[2].trim().parse::<i32>().unwrap();

        g_from.push(g_from_temp);
        g_to.push(g_to_temp);
        g_weight.push(g_weight_temp);
    }

    getCost(g_nodes, &g_from, &g_to, &g_weight);
}