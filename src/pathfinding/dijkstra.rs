use crate::pathfinding::Connection;
use crate::pathfinding::Graph;
use crate::pathfinding::Node;
use crate::pathfinding::Path;

pub fn dijkstra_search(
    graph: &Graph,
    start: &Node,
    end: &Node,
    trace: Vec<Connection>,
) -> Option<Path> {
    let pathes = traverse(graph, &start, end, trace);
    if pathes.len() > 0 {
        let cheapest = pathes
            .iter()
            .min_by(|a, b| a.cost().partial_cmp(&b.cost()).unwrap())
            .unwrap();

        Some(cheapest.clone())
    } else {
        None
    }
}

pub fn traverse(graph: &Graph, start: &Node, end: &Node, mut trace: Vec<Connection>) -> Vec<Path> {
    let mut traces = Vec::new();
    let connections = graph.get_connections(&start);

    for connection in connections.iter() {
        if (connection.b_node == *end) || connection.a_node == *end {
            trace.push(**connection);

            traces.push(Path {
                connections: trace.clone(),
            });
        } else {
            let mut new_trace = trace.clone();
            new_trace.push(**connection);

            let mut path_results = traverse(graph, &connection.b_node, end, new_trace);

            traces.append(&mut path_results);
        }
    }

    traces
}
