use games_ai_book::pathfinding::{self};

#[test]
fn dijkstra_search_test() {
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

    let path = pathfinding::dijkstra_search(&graph, &n1, &n7, Vec::new()).unwrap();
    let mut expected_path_1 = Vec::new();
    expected_path_1.push(connection_4);
    expected_path_1.push(connection_5);
    expected_path_1.push(connection_6);
    expected_path_1.push(connection_7);
    expected_path_1.push(connection_3);

    assert_eq!(path.cost(), 8.0);
    assert_eq!(path.connections, expected_path_1);

    let path2 = pathfinding::dijkstra_search(&graph, &n4, &n9, Vec::new()).unwrap();
    assert_eq!(path2.cost(), 3.0);
}

// Graph topology:
//
// 1 ---- 2 ---- 3\
// |             |  \
// |             |    \
// 4 ---- 5 ---- 6 ---- 7
//        |           /
//        |         /
//        8 ---- 9/
