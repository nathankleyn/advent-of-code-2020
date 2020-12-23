use std::convert::TryFrom;
use std::str::FromStr;

#[allow(dead_code)]
fn day_3(input: &str, right_shift: usize, down_shift: usize) -> Result<usize, MapParseError> {
    let map: Map = str::parse(input)?;

    let mut trees = 0;
    let mut col = right_shift; // Start at right by right_shift...
    for row in map.into_iter().skip(down_shift).step_by(down_shift) { // and down by down_shift
        match &row.into_iter().skip(col).next() {
            Some(MapItem::Tree) => {
                trees += 1;
            }
            _ => ()
        }

        col += right_shift;
    }

    Ok(trees)
}

#[derive(Debug, Eq, PartialEq)]
enum MapParseError {
    InvalidChar(char)
}

#[derive(Debug, Eq, PartialEq)]
struct Map {
    rows: Vec<MapRow>,
}

impl Map {
    fn len(&self) -> usize {
        self.rows.len()
    }
}

impl<'m> IntoIterator for &'m Map {
    // we will be counting with usize
    type Item = MapRowIter<'m>;
    type IntoIter = MapIter<'m>;

    fn into_iter(self) -> Self::IntoIter {
        MapIter {
            row_idx: 0,
            map: self
        }
    }
}

impl FromStr for Map {
    type Err = MapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s.lines()
            .filter(|line| !line.is_empty())
            .map(|s| str::parse(s))
            .collect::<Result<Vec<MapRow>, Self::Err>>()?;

        Ok(Map{
            rows
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct MapRow {
    items: Vec<MapItem>
}

impl MapRow {
    fn len(&self) -> usize {
        self.items.len()
    }
}

impl<'r> IntoIterator for &'r MapRow {
    type Item = &'r MapItem;
    type IntoIter = MapRowIter<'r>;

    fn into_iter(self) -> Self::IntoIter {
        MapRowIter {
            item_idx: 0,
            row: self
        }
    }
}

impl FromStr for MapRow {
    type Err = MapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<MapItem> = s.chars()
            .map(|c| MapItem::try_from(c))
            .collect::<Result<Vec<MapItem>, Self::Err>>()?;

            Ok(MapRow {
                items
            })
    }
}

#[derive(Debug, Eq, PartialEq)]
enum MapItem {
    OpenSquare,
    Tree
}

impl TryFrom<char> for MapItem {
    type Error = MapParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(MapItem::OpenSquare),
            '#' => Ok(MapItem::Tree),
            _ => Err(MapParseError::InvalidChar(value))
        }
    }
}

// Iterators

struct MapIter<'m> {
    row_idx: usize,
    map: &'m Map
}

impl<'m> Iterator for MapIter<'m> {
    type Item = MapRowIter<'m>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row_idx >= self.map.len() {
            return None;
        }

        self.row_idx += 1;
        self.map.rows.get(self.row_idx - 1)
            .map(|row| row.into_iter())
    }
}

struct MapRowIter<'r> {
    item_idx: usize,
    row: &'r MapRow
}

impl<'r> Iterator for MapRowIter<'r> {
    type Item = &'r MapItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.item_idx >= self.row.len() {
            self.item_idx = 0;
        }

        self.item_idx += 1;
        self.row.items.get(self.item_idx - 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::MapParseError;

    use super::day_3;

    #[test]
    fn day_3_part_1_examples() {
        assert_eq!(day_3(include_str!("example"), 3, 1), Ok(7));
    }

    #[test]
    fn day_3_part_1_test_input() {
        assert_eq!(day_3(include_str!("input"), 3, 1), Ok(167));
    }

    #[test]
    fn day_3_part_2_examples() {
        assert_eq!(day_3(include_str!("example"), 1, 1), Ok(2));
        assert_eq!(day_3(include_str!("example"), 3, 1), Ok(7));
        assert_eq!(day_3(include_str!("example"), 5, 1), Ok(3));
        assert_eq!(day_3(include_str!("example"), 7, 1), Ok(4));
        assert_eq!(day_3(include_str!("example"), 1, 2), Ok(2));
    }

    #[test]
    fn day_3_part_2_test_input() -> Result<(), MapParseError> {
        let a = day_3(include_str!("input"), 1, 1)?;
        let b = day_3(include_str!("input"), 3, 1)?;
        let c = day_3(include_str!("input"), 5, 1)?;
        let d = day_3(include_str!("input"), 7, 1)?;
        let e = day_3(include_str!("input"), 1, 2)?;

        assert_eq!(a * b * c * d * e, 736527114);

        Ok(())
    }
}
