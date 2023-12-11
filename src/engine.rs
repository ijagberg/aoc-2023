use simple_grid::{Grid, GridIndex};
use std::collections::{HashMap, HashSet};

type SchematicGrid = Grid<SchematicSymbol>;

#[derive(Debug)]
pub struct Schematic {
    inner: SchematicGrid,
    parts: Vec<Part>,
    index_to_part: HashMap<GridIndex, usize>,
}

impl Schematic {
    pub fn new(inner: SchematicGrid) -> Self {
        let (parts, index_to_part) = Self::fill_parts(&inner);
        Self {
            inner,
            parts,
            index_to_part,
        }
    }

    pub fn parts(&self) -> &Vec<Part> {
        &self.parts
    }

    pub fn part_numbers(&self) -> Vec<u32> {
        let mut set = HashSet::new();
        for (idx, part) in &self.index_to_part {
            for neighbor in idx.neighbors() {
                if let Some(SchematicSymbol::Symbol(_)) = self.inner.get(neighbor) {
                    set.insert(self.parts[*part]);
                }
            }
        }
        set.iter().map(|v| v.value).collect()
    }

    pub fn gear_ratios(&self) -> Vec<u32> {
        let mut gears = Vec::new();
        for idx in self.inner.indices() {
            if let Some(SchematicSymbol::Symbol('*')) = self.inner.get(idx) {
                let mut adjacent_parts = HashSet::new();
                for neighbor in idx.neighbors() {
                    if let Some(part) = self.index_to_part.get(&neighbor) {
                        adjacent_parts.insert(self.parts[*part]);
                    }
                }

                if adjacent_parts.len() == 2 {
                    gears.push(adjacent_parts.iter().map(|p| p.value).product());
                }
            }
        }
        gears
    }

    fn fill_parts(grid: &SchematicGrid) -> (Vec<Part>, HashMap<GridIndex, usize>) {
        let mut parts = Vec::new();
        let mut indices_to_parts = HashMap::new();
        for end_idx in grid.indices() {
            // check if a part ends at this index
            if let SchematicSymbol::Number(num) = grid[end_idx] {
                if Self::is_end_of_number(grid, end_idx) {
                    let mut value = 0;
                    for l in 0.. {
                        let part_index = GridIndex::new(end_idx.column() - l, end_idx.row());
                        indices_to_parts.insert(part_index, parts.len()); // this is where the part will go when finished parsing
                        value += grid[part_index].unwrap_number() * 10_u32.pow(l as u32);
                        if Self::is_start_of_number(grid, part_index) {
                            parts.push(Part::new(part_index, value, l + 1));
                            break;
                        }
                    }
                }
            }
        }

        (parts, indices_to_parts)
    }

    fn is_part_adjacent_to_symbol(grid: &SchematicGrid, part: &Part) -> bool {
        for idx in part.indices() {
            for neighbor in idx.neighbors() {
                if Self::is_symbol(grid, neighbor) {
                    return true;
                }
            }
        }

        false
    }

    fn is_symbol(grid: &SchematicGrid, idx: GridIndex) -> bool {
        matches!(grid.get(idx), Some(SchematicSymbol::Symbol(_)))
    }

    fn is_start_of_number(grid: &SchematicGrid, idx: GridIndex) -> bool {
        if let Some(left) = idx.left() {
            !Self::is_number(grid, left)
        } else {
            true
        }
    }

    fn is_end_of_number(grid: &SchematicGrid, idx: GridIndex) -> bool {
        if let Some(right) = idx.right() {
            !Self::is_number(grid, right)
        } else {
            true
        }
    }

    fn is_number(grid: &SchematicGrid, idx: GridIndex) -> bool {
        matches!(grid.get(idx), Some(SchematicSymbol::Number(_)))
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Part {
    start: GridIndex,
    value: u32,
    len: usize,
}

impl Part {
    fn new(start: GridIndex, value: u32, len: usize) -> Self {
        Self { start, value, len }
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    fn indices(&self) -> impl Iterator<Item = GridIndex> + '_ {
        (0..self.len).map(|c| GridIndex::new(self.start.column() + c, self.start.row()))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SchematicSymbol {
    Number(u32),
    Symbol(char),
    Period,
}

impl SchematicSymbol {
    fn unwrap_number(self) -> u32 {
        if let Self::Number(v) = self {
            v
        } else {
            panic!()
        }
    }
}
