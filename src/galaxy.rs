use simple_grid::{Grid, GridIndex};
use std::fmt::Display;

pub struct GalaxyMap {
    map: Grid<GalaxyOrEmpty>,
    age: usize,
    rows_up: Vec<usize>,
    columns_left: Vec<usize>,
}

impl GalaxyMap {
    pub fn new(map: Grid<GalaxyOrEmpty>, age: usize) -> Self {
        let mut rows_total = 0;
        let mut rows_up = vec![0; map.height()];
        for row in map.rows() {
            rows_up[row] = rows_total;
            if map.row_iter(row).all(|c| c == &GalaxyOrEmpty::Empty) {
                rows_total += age;
            }
            rows_total += 1;
        }

        let mut columns_total = 0;
        let mut columns_left = vec![0; map.width()];
        for column in map.columns() {
            columns_left[column] = columns_total;
            if map.column_iter(column).all(|c| c == &GalaxyOrEmpty::Empty) {
                columns_total += age;
            }
            columns_total += 1;
        }

        Self {
            map,
            age,
            columns_left,
            rows_up,
        }
    }

    pub fn galaxies(&self) -> Vec<GridIndex> {
        self.map
            .indices()
            .filter(|idx| self.map[*idx] == GalaxyOrEmpty::Galaxy)
            .collect()
    }

    pub fn distance_between(&self, a: GridIndex, b: GridIndex) -> usize {
        let column_diff = self.columns_left[a.column()].abs_diff(self.columns_left[b.column()]);
        let row_diff = self.rows_up[a.row()].abs_diff(self.rows_up[b.row()]);
        column_diff + row_diff
    }

    pub fn to_pretty_string(&self) -> String {
        self.map.to_pretty_string()
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum GalaxyOrEmpty {
    Galaxy,
    Empty,
}

impl TryFrom<char> for GalaxyOrEmpty {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Galaxy),
            _ => Err(()),
        }
    }
}

impl Display for GalaxyOrEmpty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            GalaxyOrEmpty::Galaxy => "#",
            GalaxyOrEmpty::Empty => ".",
        };

        write!(f, "{}", output)
    }
}
