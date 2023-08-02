use games_ai_book::pathfinding::{self};

// Graph topology:
//
// 1 ---- 2 ---- 3\
// |             |  \
// |             |    \
// 4 ---- 5 ---- 6 ---- 7
//        |           /
//        |         /
//        8 ---- 9/

#[test]
fn a_star_search_test() {
    let n1 = pathfinding::Node { id: 1 };
    let n2 = pathfinding::Node { id: 2 };
    let n3 = pathfinding::Node { id: 3 };
    let n4 = pathfinding::Node { id: 4 };
    let n5 = pathfinding::Node { id: 5 };
    let n6 = pathfinding::Node { id: 6 };
    let n7 = pathfinding::Node { id: 7 };
    let n8 = pathfinding::Node { id: 8 };
    let n9 = pathfinding::Node { id: 9 };

    let mut graph = pathfinding::Graph { store: Vec::new() };

    let connection_1 = pathfinding::Connection {
        cost: 3.0,
        a_node: n1,
        b_node: n2,
    };
    graph.store.push(connection_1);
    let connection_2 = pathfinding::Connection {
        cost: 10.0,
        a_node: n2,
        b_node: n3,
    };
    graph.store.push(connection_2);
    let connection_3 = pathfinding::Connection {
        cost: 2.0,
        a_node: n3,
        b_node: n7,
    };
    graph.store.push(connection_3);
    let connection_4 = pathfinding::Connection {
        cost: 1.0,
        a_node: n1,
        b_node: n4,
    };
    graph.store.push(connection_4);
    let connection_5 = pathfinding::Connection {
        cost: 1.0,
        a_node: n4,
        b_node: n5,
    };
    graph.store.push(connection_5);
    let connection_6 = pathfinding::Connection {
        cost: 2.0,
        a_node: n5,
        b_node: n6,
    };
    graph.store.push(connection_6);
    let connection_7 = pathfinding::Connection {
        cost: 2.0,
        a_node: n6,
        b_node: n3,
    };
    graph.store.push(connection_7);
    let connection_8 = pathfinding::Connection {
        cost: 1.0,
        a_node: n5,
        b_node: n8,
    };
    graph.store.push(connection_8);
    let connection_9 = pathfinding::Connection {
        cost: 1.0,
        a_node: n8,
        b_node: n9,
    };
    graph.store.push(connection_9);
    let connection_10 = pathfinding::Connection {
        cost: 50.0,
        a_node: n9,
        b_node: n7,
    };
    graph.store.push(connection_10);

    let Some(path) = pathfinding::astar_search(&graph, &n1, &n7, None) else { todo!() };

    assert_eq!(path.len(), 6);
    assert_eq!(path[0], n1);
    assert_eq!(path[1], n4);
    assert_eq!(path[2], n5);
    assert_eq!(path[3], n6);
    assert_eq!(path[4], n3);
    assert_eq!(path[5], n7);

    let Some(path) = pathfinding::astar_search(&graph, &n4, &n9, None) else { todo!() };

    assert_eq!(path.len(), 4);
    assert_eq!(path[0], n4);
    assert_eq!(path[1], n5);
    assert_eq!(path[2], n8);
    assert_eq!(path[3], n9);

    fn dummy_heuristic(node: &pathfinding::Node) -> f32 {
        if node.id == 4 {
            1000.0
        } else {
            0.0
        }
    }

    let Some(path) = pathfinding::astar_search(&graph, &n1, &n7, Some(dummy_heuristic)) else { todo!() };

    assert_eq!(path.len(), 4);
    assert_eq!(path[0], n1);
    assert_eq!(path[1], n2);
    assert_eq!(path[2], n3);
    assert_eq!(path[3], n7);
}

#[test]
fn a_star_grid_graph_search_display_visual_test() {
    let graph = pathfinding::Graph::generate_graph_grid(20, 20, None, None);

    let path = pathfinding::astar_search(
        &graph,
        &graph.store[0].a_node,
        &graph.store[759].b_node,
        None,
    );

    let _graph_with_path = pathfinding::Graph::generate_graph_grid(20, 20, None, path);
}
