use std::collections::HashMap;

use crate::pathfinding::Graph;
use crate::pathfinding::Node;

// A* selects the path that minimizes f(n) = g(n) + h(n)
// g(n) is the cost of the path from the start node to n
// h(n) is a heuristic function that estimates the cost of the cheapest path from n to the goal

// https://doc.rust-lang.org/book/ch19-05-advanced-functions-and-closures.html
pub fn astar_search(
    graph: &Graph,
    start: &Node,
    goal: &Node,
    heuristic_f: Option<fn(&Node) -> f32>,
) -> Option<Vec<Node>> {
    let mut open_set: Vec<Node> = vec![];
    open_set.push(*start);

    let mut came_from: HashMap<u32, Node> = HashMap::new();

    // For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
    let mut g_score: HashMap<u32, f32> = HashMap::new();
    g_score.insert(start.id, 0.0);

    // For node n, fScore[n] := gScore[n] + h(n). fScore[n] represents our current best guess as to
    // how cheap a path could be from start to finish if it goes through n.
    let mut f_score: HashMap<u32, f32> = HashMap::new();
    f_score.insert(start.id, heuristic(heuristic_f, start));

    while open_set.len() > 0 {
        let mut current = open_set[0];

        for node in open_set.iter() {
            if let Some(score) = f_score.get(&node.id) {
                if score < f_score.get(&current.id).unwrap() {
                    current = *node;
                }
            }
        }

        if &current == goal {
            println!(
                "------- A* score: {} --------",
                f_score.get(&current.id).unwrap()
            );

            return Some(reconstruct_path(came_from, goal));
        }

        let remove_index = open_set.iter().position(|item| *item == current).unwrap();
        open_set.remove(remove_index);

        for connection in graph.get_connections(&current) {
            let neighbor = connection.b_node;
            // d(current,neighbor) is the weight of the edge from current to neighbor
            // tentative_gScore is the distance from start to the neighbor through current
            let tentative_g_score = g_score.get(&current.id).unwrap() + connection.cost;
            if &tentative_g_score < g_score.get(&neighbor.id).unwrap_or(&f32::INFINITY) {
                // This path to neighbor is better than any previous one. Record it!
                came_from.insert(neighbor.id, current.clone());
                g_score.insert(neighbor.id, tentative_g_score);

                let f_score_neighbor = tentative_g_score + heuristic(heuristic_f, &neighbor);
                f_score.insert(neighbor.id, f_score_neighbor);

                if !open_set.contains(&&neighbor) {
                    open_set.push(neighbor);
                }
            }
        }
    }

    None
}

// use given function if present or return default value
pub fn heuristic(heuristic_f: Option<fn(&Node) -> f32>, node: &Node) -> f32 {
    match heuristic_f {
        Some(fnc) => return fnc(node),
        _ => return 0.0,
    }
}

pub fn reconstruct_path(came_from: HashMap<u32, Node>, end: &Node) -> Vec<Node> {
    let mut total_path = vec![];

    total_path.push(*end);

    let mut current = end;

    while came_from.contains_key(&current.id) {
        current = came_from.get(&current.id).unwrap();

        total_path.insert(0, *current);
    }

    total_path
}
