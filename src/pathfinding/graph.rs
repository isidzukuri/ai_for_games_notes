use colored::Colorize;
use rand::prelude::*;

use crate::pathfinding::Connection;
use crate::pathfinding::Node;

#[derive(Debug)]
pub struct Graph {
    pub store: Vec<Connection>,
}

// Directional, weighted graph
impl Graph {
    pub fn get_connections(&self, from_node: &Node) -> Vec<&Connection> {
        let mut result = Vec::new();

        for connection in self.store.iter().filter(|item| &item.a_node == from_node) {
            result.push(connection);
        }

        result
    }

    // TODO: refactor it into grnerator and printer for example
    pub fn generate_graph_grid(
        cols: usize,
        rows: usize,
        vertex_cost: Option<f32>,
        highlight: Option<Vec<Node>>,
    ) -> Graph {
        let mut rng = rand::thread_rng();
        let mut graph = Graph { store: Vec::new() };
        let mut nodes = vec![];
        let mut id = 1;
        let mut current_row = 0;

        let highlighted_nodes = match &highlight {
            Some(path) => path.iter().map(|item| item.id).collect(),
            _ => vec![],
        };

        println!("--------Graph ids-------");

        while current_row < rows {
            let mut current_col = 0;

            nodes.push(vec![]);
            while current_col < cols {
                nodes[current_row].push(Node { id: id });

                if current_col > 0 {
                    let cost: f32 = vertex_cost.unwrap_or(rng.gen());

                    graph.store.push(Connection {
                        cost: cost,
                        a_node: nodes[current_row][current_col - 1],
                        b_node: nodes[current_row][current_col],
                    });
                }

                if current_row > 0 {
                    graph.store.push(Connection {
                        cost: 1.0,
                        a_node: nodes[current_row - 1][current_col],
                        b_node: nodes[current_row][current_col],
                    });
                }

                current_col += 1;
                id += 1;
            }

            let ids: Vec<String> = nodes[current_row]
                .iter()
                .map(|item| match highlighted_nodes.contains(&item.id) {
                    true => item.id.to_string().green().to_string(),
                    false => item.id.to_string(),
                })
                .collect();

            println!("{}", ids.join("\t"));
            println!("");

            current_row += 1;
        }
        println!("--------------------");

        graph
    }
}
