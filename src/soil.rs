use std::{cmp::Ordering, collections::VecDeque, os::raw};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Range {
    start: u64,
    end: u64,
}

impl Range {
    pub fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    fn overlap(&self, source: Self) -> Option<(Self, Vec<Self>)> {
        if source.start >= self.start && source.end <= self.end {
            // self:    ----------|##########|----------
            // source:  -------------|######|-----------
            // overlap: -------------|######|-----------
            // remain:  --------------------------------
            return Some((Self::new(source.start, source.end), Vec::new()));
        }
        if source.start >= self.start && source.start < self.end && source.end >= self.end {
            // self:    ----------|##########|----------
            // source:  -------------|#########|--------
            // overlap: -------------|#######|----------
            // remain:  ---------------------|#|--------
            return Some((
                Self::new(source.start, self.end),
                vec![Self::new(self.end, source.end)],
            ));
        }
        if source.start < self.start && self.start < source.end && source.end <= self.end {
            // self:    ----------|##########|----------
            // source:  --------|###########|-----------
            // overlap: ----------|#########|-----------
            // remain:  --------|#|---------------------
            return Some((
                Self::new(self.start, source.end),
                vec![Self::new(source.start, self.start)],
            ));
        }
        if source.start < self.start && source.end > self.end {
            // self:    ----------|##########|----------
            // source:  --------|##############|--------
            // overlap: ----------|##########|----------
            // remain:  --------|#|----------|#|--------
            return Some((
                Self::new(self.start, self.end),
                vec![
                    Self::new(source.start, self.start),
                    Self::new(self.end, source.end),
                ],
            ));
        }

        // self:    ----------|##########|----------
        // source:  ---|###|------------------------
        // overlap: --------------------------------
        // remain:  ---|###|------------------------
        None
    }

    pub fn start(&self) -> u64 {
        self.start
    }

    pub fn end(&self) -> u64 {
        self.end
    }
}

#[derive(Debug)]
pub struct Map {
    dest: Range,
    source: Range,
}

impl Map {
    pub fn new(dest: u64, source: u64, len: u64) -> Self {
        Self {
            dest: Range::new(dest, dest + len),
            source: Range::new(source, source + len),
        }
    }

    fn convert(&self, input: Range) -> Option<(Range, Vec<Range>)> {
        if let Some((overlap, remaining)) = self.source.overlap(input) {
            match self.dest.start.cmp(&self.source.start) {
                Ordering::Less => {
                    let diff = self.source.start - self.dest.start;
                    // subtract diff to get to dest
                    Some((
                        Range::new(overlap.start - diff, overlap.end - diff),
                        remaining,
                    ))
                }
                Ordering::Greater => {
                    let diff = self.dest.start - self.source.start;
                    Some((
                        Range::new(overlap.start + diff, overlap.end + diff),
                        remaining,
                    ))
                }
                _ => unreachable!(),
            }
        } else {
            None
        }
    }
}

pub struct Almanac {
    mappings: Vec<Mappings>,
}

impl Almanac {
    pub fn new(mappings: Vec<Mappings>) -> Self {
        Self { mappings }
    }

    pub fn convert(&self, input: Range) -> Vec<Range> {
        let mut stack = vec![input];
        for i in 0..self.mappings.len() {
            let mapping = &self.mappings[i];
            let mut next_stack = Vec::new();
            while let Some(range) = stack.pop() {
                let mut output_ranges = mapping.convert(range);
                next_stack.append(&mut output_ranges);
            }
            stack = next_stack;
            if i == self.mappings.len() - 1 {
                // last mapping
                return stack;
            }
        }

        unreachable!()
    }
}

#[derive(Debug)]
pub struct Mappings {
    name: String,
    maps: Vec<Map>,
}

impl Mappings {
    pub fn new(name: String, maps: Vec<Map>) -> Self {
        Self { name, maps }
    }

    pub fn convert(&self, input: Range) -> Vec<Range> {
        let mut output_ranges = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(input);
        let mut count = 0;
        while let Some(input_range) = queue.pop_front() {
            count += 1;
            let mut some_overlap = false;
            for map in &self.maps {
                if let Some((converted, remaining)) = map.convert(input_range) {
                    output_ranges.push(converted);
                    some_overlap = true;
                    for r in remaining {
                        queue.push_back(r);
                    }
                    break;
                }
            }

            if !some_overlap {
                output_ranges.push(input_range);
            }

            if count > 10 {
                break;
            }
        }

        output_ranges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_test() {
        let map = Map::new(52, 50, 48);

        assert_eq!(
            map.convert(Range::new(79, 79 + 14)),
            Some((Range::new(81, 95), Vec::new()))
        );

        assert_eq!(Map::new(81, 45, 19).convert(Range::new(74, 88)), None);

        let mappings = Mappings::new(
            "".to_string(),
            vec![
                Map::new(45, 77, 23),
                Map::new(81, 45, 19),
                Map::new(68, 64, 13),
            ],
        );

        assert_eq!(
            mappings.convert(Range::new(74, 88)),
            vec![Range::new(45, 56), Range::new(78, 81)]
        );
    }

    #[test]
    fn example1_test() {
        let almanac = Almanac::new(vec![
            Mappings::new(
                "seed-to-soil map:".to_string(),
                vec![Map::new(50, 98, 2), Map::new(52, 50, 48)],
            ),
            Mappings::new(
                "soil-to-fertilizer map:".to_string(),
                vec![
                    Map::new(0, 15, 37),
                    Map::new(37, 52, 2),
                    Map::new(39, 0, 15),
                ],
            ),
            Mappings::new(
                "fertilizer-to-water map:".to_string(),
                vec![
                    Map::new(49, 53, 8),
                    Map::new(0, 11, 42),
                    Map::new(42, 0, 7),
                    Map::new(57, 7, 4),
                ],
            ),
            Mappings::new(
                "water-to-light map:".to_string(),
                vec![Map::new(88, 18, 7), Map::new(18, 25, 70)],
            ),
            Mappings::new(
                "light-to-temperature map:".to_string(),
                vec![
                    Map::new(45, 77, 23),
                    Map::new(81, 45, 19),
                    Map::new(68, 64, 13),
                ],
            ),
            Mappings::new(
                "temperature-to-humidity map:".to_string(),
                vec![Map::new(0, 69, 1), Map::new(1, 0, 69)],
            ),
            Mappings::new(
                "humidity-to-location map:".to_string(),
                vec![Map::new(60, 56, 37), Map::new(56, 93, 4)],
            ),
        ]);

        assert_eq!(
            almanac.convert(Range::new(13, 13 + 1)),
            vec![Range::new(35, 36)]
        );
    }

    #[test]
    fn convert_test_2() {
        let mappings = vec![
            Mappings::new(
                "seed-to-soil map:".to_string(),
                vec![Map::new(50, 98, 2), Map::new(52, 50, 48)],
            ),
            Mappings::new(
                "soil-to-fertilizer map:".to_string(),
                vec![
                    Map::new(0, 15, 37),
                    Map::new(37, 52, 2),
                    Map::new(39, 0, 15),
                ],
            ),
            Mappings::new(
                "fertilizer-to-water map:".to_string(),
                vec![
                    Map::new(49, 53, 8),
                    Map::new(0, 11, 42),
                    Map::new(42, 0, 7),
                    Map::new(57, 7, 4),
                ],
            ),
            Mappings::new(
                "water-to-light map:".to_string(),
                vec![Map::new(88, 18, 7), Map::new(18, 25, 70)],
            ),
            Mappings::new(
                "light-to-temperature map:".to_string(),
                vec![
                    Map::new(45, 77, 23),
                    Map::new(81, 45, 19),
                    Map::new(68, 64, 13),
                ],
            ),
            Mappings::new(
                "temperature-to-humidity map:".to_string(),
                vec![Map::new(0, 69, 1), Map::new(1, 0, 69)],
            ),
            Mappings::new(
                "humidity-to-location map:".to_string(),
                vec![Map::new(60, 56, 37), Map::new(56, 93, 4)],
            ),
        ];

        assert_eq!(
            mappings[0].convert(Range::new(13, 14)),
            vec![Range::new(13, 14)]
        );
        assert_eq!(
            mappings[1].convert(Range::new(13, 14)),
            vec![Range::new(52, 53)]
        );
        assert_eq!(
            mappings[2].convert(Range::new(52, 53)),
            vec![Range::new(41, 42)]
        );
    }
}
