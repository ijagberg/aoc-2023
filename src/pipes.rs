use simple_grid::{Grid, GridIndex};
use std::collections::{HashSet, VecDeque};

pub struct Pipes {
    pipes: Grid<Tile>,
    start: GridIndex,
}

impl Pipes {
    pub fn new(pipes: Grid<Tile>) -> Self {
        let start = pipes
            .indices()
            .find(|&i| matches!(pipes[i], Tile::Start))
            .unwrap();
        Self { pipes, start }
    }

    pub fn find_farthest_point_from_start(&self) -> usize {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((0, self.start));

        let mut maximum = None;
        // bfs
        while let Some((dist, curr)) = queue.pop_front() {
            if !visited.contains(&curr) {
                Self::update_maximum(&mut maximum, dist);
                visited.insert(curr);
            } else {
                continue;
            }

            let current_tile = self.pipes[curr];
            if let Some(up_idx) = self.pipes.up_index(curr) {
                let up_tile = self.pipes[up_idx];
                if Tile::can_move_to(Direction::Down, current_tile)
                    && Tile::can_move_to(Direction::Up, up_tile)
                {
                    queue.push_back((dist + 1, up_idx));
                }
            }
            if let Some(right_idx) = self.pipes.right_index(curr) {
                let right_tile = self.pipes[right_idx];
                if Tile::can_move_to(Direction::Left, current_tile)
                    && Tile::can_move_to(Direction::Right, right_tile)
                {
                    queue.push_back((dist + 1, right_idx));
                }
            }
            if let Some(down_idx) = self.pipes.down_index(curr) {
                let down_tile = self.pipes[down_idx];
                if Tile::can_move_to(Direction::Up, current_tile)
                    && Tile::can_move_to(Direction::Down, down_tile)
                {
                    queue.push_back((dist + 1, down_idx));
                }
            }
            if let Some(left_idx) = self.pipes.left_index(curr) {
                let left_tile = self.pipes[left_idx];
                if Tile::can_move_to(Direction::Right, current_tile)
                    && Tile::can_move_to(Direction::Left, left_tile)
                {
                    queue.push_back((dist + 1, left_idx));
                }
            }
        }

        maximum.unwrap()
    }

    fn update_maximum(curr_max: &mut Option<usize>, new_value: usize) {
        if let Some(c) = curr_max {
            if new_value > *c {
                *c = new_value;
            }
        } else {
            *curr_max = Some(new_value);
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Vert,
    Hori,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl Tile {
    fn can_move_to(direction: Direction, tile: Self) -> bool {
        use Direction::*;
        use Tile::*;
        match direction {
            Up => matches!(tile, Vert | SW | SE | Start),
            Right => matches!(tile, Hori | SW | NW | Start),
            Down => matches!(tile, Vert | NW | NE | Start),
            Left => matches!(tile, Hori | SE | NE | Start),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '|' => Self::Vert,
            '-' => Self::Hori,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => return Err(()),
        })
    }
}
