use crate::pathfinding::Connection;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Path {
    pub connections: Vec<Connection>,
}

impl Path {
    pub fn cost(&self) -> f32 {
        let mut sum = 0.0;
        for connection in &self.connections {
            sum += connection.cost;
        }
        sum
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "------------------------\r\n").unwrap();
        println!("{:?}", self);
        write!(f, "\r\n").unwrap();
        for connection in &self.connections {
            write!(
                f,
                "{}--({})-->{}\r\n",
                connection.a_node.id, connection.cost, connection.b_node.id
            )
            .unwrap();
        }
        write!(f, "total: {}", self.cost()).unwrap();
        write!(f, "\r\n").unwrap();
        write!(f, "------------------------")
    }
}
