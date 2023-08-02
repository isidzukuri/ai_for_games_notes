use crate::pathfinding::Node;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Connection {
    pub cost: f32,
    pub a_node: Node,
    pub b_node: Node,
}
