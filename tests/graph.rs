use claim::*;
use games_ai_book::pathfinding::{self};

#[test]
fn generate_graph_grid_test() {
    let g_2_2 = pathfinding::Graph::generate_graph_grid(2, 2, Some(1.0), None);

    assert_eq!(g_2_2.store.len(), 4);

    assert_eq!(g_2_2.store[0].cost, 1.0);
    assert_eq!(g_2_2.store[0].a_node, pathfinding::Node { id: 1 });
    assert_eq!(g_2_2.store[0].b_node, pathfinding::Node { id: 2 });

    assert_eq!(g_2_2.store[1].cost, 1.0);
    assert_eq!(g_2_2.store[1].a_node, pathfinding::Node { id: 1 });
    assert_eq!(g_2_2.store[1].b_node, pathfinding::Node { id: 3 });

    assert_eq!(g_2_2.store[2].cost, 1.0);
    assert_eq!(g_2_2.store[2].a_node, pathfinding::Node { id: 3 });
    assert_eq!(g_2_2.store[2].b_node, pathfinding::Node { id: 4 });

    assert_eq!(g_2_2.store[3].cost, 1.0);
    assert_eq!(g_2_2.store[3].a_node, pathfinding::Node { id: 2 });
    assert_eq!(g_2_2.store[3].b_node, pathfinding::Node { id: 4 });

    let g_2_2_random_traverse_cost = pathfinding::Graph::generate_graph_grid(2, 2, None, None);
    println!("{:?}", g_2_2_random_traverse_cost.store);

    assert_ge!(g_2_2.store[0].cost, 0.0);
    assert_le!(g_2_2.store[0].cost, 1.0);

    assert_ge!(g_2_2.store[1].cost, 0.0);
    assert_le!(g_2_2.store[1].cost, 1.0);

    assert_ge!(g_2_2.store[2].cost, 0.0);
    assert_le!(g_2_2.store[2].cost, 1.0);

    assert_ge!(g_2_2.store[3].cost, 0.0);
    assert_le!(g_2_2.store[3].cost, 1.0);
}
