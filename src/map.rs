use std::collections::HashMap;

type Id = [char; 3];

pub struct Map {
    nodes: HashMap<Id, Node>,
}

impl Map {
    pub fn new(nodes: HashMap<Id, Node>) -> Self {
        Self { nodes }
    }

    pub fn path_length(&self, from: Id, to: Id, path: &[Direction]) -> u64 {
        use Direction::*;
        let mut current = from;
        for (count, dir) in path.iter().cycle().enumerate() {
            if current == to {
                return count as u64;
            }
            let node = &self.nodes[&current];
            current = match dir {
                Left => node.0,
                Right => node.1,
            };
        }

        unreachable!()
    }

    pub fn step_once(&self, from: Id, step: Direction) -> Id {
        use Direction::*;
        match step {
            Left => self.nodes[&from].0,
            Right => self.nodes[&from].1,
        }
    }

    pub fn nodes(&self) -> &HashMap<Id, Node> {
        &self.nodes
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub struct Node(pub Id, pub Id);
