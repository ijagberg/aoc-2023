use simple_grid::{Grid, GridIndex};
use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

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

    pub fn travel_loop(&self) -> Vec<(GridIndex, Direction)> {
        let mut pipe_loop = Vec::new();

        let mut queue: VecDeque<(GridIndex, Option<Direction>)> = VecDeque::new();
        queue.push_back((self.start, None));
        'travel: while let Some((curr, from_dir)) = queue.pop_front() {
            let current_tile = self.pipes[curr];
            for dir in nesw().filter(|d| from_dir.map(|from| from != d.opposite()).unwrap_or(true))
            {
                if let Some(neighbor) = self.get_neighbor(curr, dir) {
                    let neighbor_tile = self.pipes[neighbor];
                    if Tile::can_move_to(dir, neighbor_tile)
                        && Tile::can_move_to(dir.opposite(), current_tile)
                    {
                        queue.push_back((neighbor, Some(dir)));
                        pipe_loop.push((curr, dir));
                        if neighbor == self.start {
                            break 'travel;
                        }
                        break;
                    }
                }
            }
        }

        pipe_loop
    }

    pub fn loop_coverage(&self) -> HashSet<GridIndex> {
        let pipe_loop = self.travel_loop();
        let indices_in_loop: HashSet<_> = pipe_loop.iter().map(|(idx, _)| idx).copied().collect();
        let mut is_clockwise = Self::is_clockwise(&pipe_loop);

        let mut inside = HashSet::new();
        let mut prev_dir = pipe_loop[pipe_loop.len() - 1].1;
        for (idx, dir) in pipe_loop {
            match (dir, is_clockwise) {
                (Direction::North, true) => {
                    self.check_inside(self.pipes.right_index(idx), &indices_in_loop, &mut inside);
                    if prev_dir == Direction::East {
                        self.check_inside(
                            self.pipes.down_index(idx),
                            &indices_in_loop,
                            &mut inside,
                        );
                    }
                }
                (Direction::North, false) => {
                    self.check_inside(self.pipes.left_index(idx), &indices_in_loop, &mut inside);
                    if prev_dir == Direction::West {
                        self.check_inside(
                            self.pipes.down_index(idx),
                            &indices_in_loop,
                            &mut inside,
                        );
                    }
                }
                (Direction::East, true) => {
                    self.check_inside(self.pipes.down_index(idx), &indices_in_loop, &mut inside);
                    if prev_dir == Direction::South {
                        self.check_inside(
                            self.pipes.left_index(idx),
                            &indices_in_loop,
                            &mut inside,
                        );
                    }
                }
                (Direction::East, false) => {
                    self.check_inside(self.pipes.up_index(idx), &indices_in_loop, &mut inside);
                    if prev_dir == Direction::North {
                        self.check_inside(
                            self.pipes.left_index(idx),
                            &indices_in_loop,
                            &mut inside,
                        );
                    }
                }
                (Direction::South, true) => {
                    self.check_inside(self.pipes.left_index(idx), &indices_in_loop, &mut inside);
                    if prev_dir == Direction::West {
                        self.check_inside(self.pipes.up_index(idx), &indices_in_loop, &mut inside);
                    }
                }
                (Direction::South, false) => {
                    self.check_inside(self.pipes.right_index(idx), &indices_in_loop, &mut inside);
                    if prev_dir == Direction::East {
                        self.check_inside(self.pipes.up_index(idx), &indices_in_loop, &mut inside);
                    }
                }
                (Direction::West, true) => {
                    self.check_inside(self.pipes.up_index(idx), &indices_in_loop, &mut inside);
                    if prev_dir == Direction::North {
                        self.check_inside(
                            self.pipes.right_index(idx),
                            &indices_in_loop,
                            &mut inside,
                        );
                    }
                }
                (Direction::West, false) => {
                    self.check_inside(self.pipes.down_index(idx), &indices_in_loop, &mut inside);
                    if prev_dir == Direction::South {
                        self.check_inside(
                            self.pipes.right_index(idx),
                            &indices_in_loop,
                            &mut inside,
                        );
                    }
                }
            }
            prev_dir = dir;
        }

        let mut queue = VecDeque::new();
        for &idx in &inside {
            queue.push_back(idx);
        }

        let mut visited = HashSet::new();
        while let Some(curr) = queue.pop_front() {
            if visited.contains(&curr) {
                continue;
            }
            inside.insert(curr);
            visited.insert(curr);
            for n_dir in nesw() {
                if let Some(n_idx) = self.get_neighbor(curr, n_dir) {
                    if !indices_in_loop.contains(&n_idx) && !visited.contains(&n_idx) {
                        queue.push_back(n_idx);
                    }
                }
            }
        }

        inside
    }

    fn print_loop_coverage(&self, inside_loop: &HashSet<GridIndex>, in_loop: &HashSet<GridIndex>) {
        enum TileOrFilled {
            Loop,
            Tile(Tile),
            Filled,
        }

        impl Display for TileOrFilled {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let output = match self {
                    TileOrFilled::Loop => "O".to_string(),
                    TileOrFilled::Tile(tile) => {
                        tile.to_string();
                        ();
                        match tile {
                            Tile::Vert => "│",
                            Tile::Hori => "─",
                            Tile::NE => "└",
                            Tile::NW => "┘",
                            Tile::SW => "┐",
                            Tile::SE => "┌",
                            Tile::Ground => ".",
                            Tile::Start => "S",
                        }
                        .to_string()
                    }
                    TileOrFilled::Filled => "X".to_string(),
                };
                write!(f, "{}", output)
            }
        }

        let mut output = Grid::new(
            self.pipes.width(),
            self.pipes.height(),
            self.pipes
                .indices()
                .map(|c| {
                    if !in_loop.contains(&c) {
                        TileOrFilled::Tile(Tile::Ground)
                    } else {
                        TileOrFilled::Tile(self.pipes[c])
                    }
                })
                .collect(),
        );
        for &idx in inside_loop {
            output[idx] = TileOrFilled::Filled;
        }
        for &idx in in_loop {
            // output[idx] = TileOrFilled::Loop;
        }

        for row in output.rows() {
            let output: Vec<_> = output.row_iter(row).map(|c| c.to_string()).collect();
            println!("{}", output.join(""));
        }

        // println!("{}", output.to_pretty_string());
    }

    fn check_inside(
        &self,
        maybe_neighbor_idx: Option<GridIndex>,
        indices_in_loop: &HashSet<GridIndex>,
        inside: &mut HashSet<GridIndex>,
    ) {
        if let Some(neighbor_idx) = maybe_neighbor_idx {
            if !indices_in_loop.contains(&neighbor_idx) {
                inside.insert(neighbor_idx);
            }
        }
    }

    fn is_clockwise(pipe_loop: &[(GridIndex, Direction)]) -> bool {
        let mut sum = 0;
        for w in pipe_loop.windows(2) {
            let (_, from) = w[0];
            let (_, to) = w[1];

            if Self::is_left_turn(from, to) {
                sum -= 1;
            }
            if Self::is_right_turn(from, to) {
                sum += 1;
            }
        }

        sum > 0
    }

    fn is_left_turn(from: Direction, to: Direction) -> bool {
        use Direction::*;
        matches!(
            (from, to),
            (North, West) | (East, North) | (South, East) | (West, South)
        )
    }

    fn is_right_turn(from: Direction, to: Direction) -> bool {
        use Direction::*;
        matches!(
            (from, to),
            (East, South) | (South, West) | (West, North) | (North, East)
        )
    }

    fn get_neighbor(&self, idx: GridIndex, direction: Direction) -> Option<GridIndex> {
        match direction {
            Direction::North => self.pipes.up_index(idx),
            Direction::East => self.pipes.right_index(idx),
            Direction::South => self.pipes.down_index(idx),
            Direction::West => self.pipes.left_index(idx),
        }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Self::South,
            Direction::East => Self::West,
            Direction::South => Self::North,
            Direction::West => Self::East,
        }
    }
}

fn nesw() -> impl Iterator<Item = Direction> {
    [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
    .into_iter()
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
            North => matches!(tile, Vert | SW | SE | Start),
            East => matches!(tile, Hori | SW | NW | Start),
            South => matches!(tile, Vert | NW | NE | Start),
            West => matches!(tile, Hori | SE | NE | Start),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Tile::Vert => "|",
            Tile::Hori => "-",
            Tile::NE => "L",
            Tile::NW => "J",
            Tile::SW => "7",
            Tile::SE => "F",
            Tile::Ground => ".",
            Tile::Start => "S",
        };
        write!(f, "{}", output)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn example_pipes() -> Pipes {
        let grid = "........S---7..|...|..|...|..|...|..L---J........";

        Pipes::new(Grid::new(
            7,
            7,
            grid.chars().map(|c| Tile::try_from(c).unwrap()).collect(),
        ))
    }

    fn big_example() -> Pipes {
        let grid = "FF7FSF7F7F7F7F7F---7L|LJ||||||||||||F--JFL-7LJLJ||||||LJL-77F--JF--7||LJLJ7F7FJ-L---JF-JLJ.||-FJLJJ7|F|F-JF---7F7-L7L|7||FFJF7L7F-JF7|JL---77-L-JL7||F7|L7F-7F7|L.L7LFJ|||||FJL7||LJL7JLJL-JLJLJL--JLJ.L";
        Pipes::new(Grid::new(
            20,
            10,
            grid.chars().map(|c| Tile::try_from(c).unwrap()).collect(),
        ))
    }

    #[test]
    fn is_clockwise_test() {
        let pipes = example_pipes();
        let path = pipes.travel_loop();

        dbg!(&path);

        assert!(Pipes::is_clockwise(&path));
    }
}
