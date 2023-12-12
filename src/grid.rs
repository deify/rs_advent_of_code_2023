use core::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn manhattan_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    pub columns: usize,
}
impl<T> Grid<T> {
    pub fn new(data: Vec<T>, columns: usize) -> Self {
        Grid { data, columns }
    }

    pub fn is_position_valid(&self, pos: &Position) -> bool {
        if pos.x >= self.columns {
            return false;
        }
        let offset = self.columns * pos.y + pos.x;

        offset <= self.data.len()
    }

    pub fn at(&self, pos: &Position) -> Option<&T> {
        if pos.x >= self.columns {
            return None;
        }
        let offset = self.columns * pos.y + pos.x;
        if offset > self.data.len() {
            None
        } else {
            Some(&self.data[offset])
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn find<P>(&self, predicate: P) -> Option<&T>
    where
        P: FnMut(&&T) -> bool,
    {
        self.data.iter().find(predicate)
    }

    fn index_to_position(&self, index: usize) -> Position {
        let (y, x) = num::Integer::div_mod_floor(&index, &self.columns);
        Position { x, y }
    }

    pub fn find_pos<P>(&self, mut predicate: P) -> Option<Position>
    where
        P: FnMut(&T) -> bool,
    {
        let i = self.data.iter().position(|x| predicate(x))?;
        Some(self.index_to_position(i))
    }

    pub fn find_positions<'a, P>(&'a self, mut predicate: P) -> impl Iterator<Item = Position> + 'a
    where
        P: FnMut(&'a T) -> bool + 'a,
    {
        self.data
            .iter()
            .enumerate()
            .filter_map(move |(i, g)| predicate(g).then_some(self.index_to_position(i)))
    }

    pub fn rows(&self) -> usize {
        self.data.len() / self.columns
    }

    pub fn iter_row(&self, row_index: usize) -> impl Iterator<Item = &T> {
        if row_index >= self.rows() {
            panic!("Out of bound row index");
        }
        let row_start = row_index * self.columns;
        let row_end = row_start + self.columns;
        self.data[row_start..row_end].iter()
    }

    pub fn iter_col(&self, col_index: usize) -> impl Iterator<Item = &T> {
        if col_index >= self.columns {
            panic!("Out of bound column index");
        }
        self.data.iter().skip(col_index).step_by(self.columns)
    }

    pub fn iter_row_mut(&mut self, row_index: usize) -> impl Iterator<Item = &mut T> {
        if row_index >= self.rows() {
            panic!("Out of bound row index");
        }
        let row_start = row_index * self.columns;
        let row_end = row_start + self.columns;
        self.data[row_start..row_end].iter_mut()
    }

    pub fn iter_col_mut(&mut self, col_index: usize) -> impl Iterator<Item = &mut T> {
        if col_index >= self.columns {
            panic!("Out of bound column index");
        }
        self.data.iter_mut().skip(col_index).step_by(self.columns)
    }

    pub fn insert_row(&mut self, row_index: usize, data: Vec<T>) {
        if row_index > self.rows() {
            panic!("Out of bound row index");
        }
        if data.len() != self.columns {
            panic!("Bad data length");
        }

        let mut insert_pos = row_index * self.columns;
        data.into_iter().for_each(|d| {
            self.data.insert(insert_pos, d);
            insert_pos += 1;
        });
    }

    pub fn insert_col(&mut self, col_index: usize, data: Vec<T>) {
        if col_index > self.columns {
            panic!("Out of bound row index");
        }
        if data.len() != self.rows() {
            panic!("Bad data length");
        }
        self.columns += 1;

        let mut insert_pos = col_index;
        data.into_iter().for_each(|d| {
            self.data.insert(insert_pos, d);
            insert_pos += self.columns;
        });
    }

    // TODO: fix -> returning a vec allows to chain operations impl Iter < impl Iter> did not work due to borrowing
    pub fn iter_rows(&self) -> impl Iterator<Item = Vec<&T>> {
        (0..self.rows()).map(|r| self.iter_row(r).collect())
    }

    pub fn iter_columns(&self) -> impl Iterator<Item = Vec<&T>> {
        (0..self.columns).map(|c| self.iter_col(c).collect())
    }

    pub fn drop_row(&mut self, row_index: usize) {
        let row_start = row_index * self.columns;
        let row_end = row_start + self.columns;

        (row_start..row_end).rev().for_each(|i| {
            self.data.remove(i);
        });
    }

    pub fn drop_column(&mut self, col_index: usize) {
        if col_index >= self.columns {
            panic!("Out of bound column index");
        }
        (col_index..self.data.len())
            .step_by(self.columns)
            .rev()
            .for_each(|i| {
                self.data.remove(i);
            });
        self.columns -= 1;
    }

    // pub fn iter_rows_mut(&mut self) -> impl Iterator<Item = impl Iterator<Item = &mut T>> {
    //     (0..self.rows()).map(|r| self.iter_row_mut(r))
    // }

    // pub fn iter_columns_mut(&mut self) -> impl Iterator<Item = impl Iterator<Item = &mut T>> {
    //     (0..self.columns).map(|c| self.iter_col_mut(c))
    // }
}

impl<T: Clone> Grid<T> {
    pub fn insert_col_with(&mut self, col_index: usize, value: &T) {
        self.insert_col(col_index, vec![value.clone(); self.rows()]);
    }
    pub fn insert_row_with(&mut self, row_index: usize, value: &T) {
        self.insert_row(row_index, vec![value.clone(); self.columns]);
    }
}
impl<T: Default + Clone> Grid<T> {
    pub fn insert_col_default(&mut self, col_index: usize) {
        self.insert_col_with(col_index, &T::default())
    }
    pub fn insert_row_default(&mut self, row_index: usize) {
        self.insert_row_with(row_index, &T::default())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidGrid,
}

impl<T: TryFrom<char>> FromStr for Grid<T> {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let columns = s
            .lines()
            .next()
            .map(|l| l.len())
            .ok_or(ParseError::InvalidGrid)?;
        let data = s
            .lines()
            .flat_map(|l| l.chars().map(T::try_from))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ParseError::InvalidGrid)?;
        Ok(Grid { data, columns })
    }
}

impl<T: Into<char> + Clone> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let self_str = self.iter_rows().fold(String::new(), |acc, r| {
            acc + r
                .into_iter()
                .cloned()
                .map(|c| std::convert::Into::<char>::into(c))
                .collect::<String>()
                .as_str()
                + "\n"
        });
        write!(f, "{}", &self_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_display() {
        let grid: Grid<char> = Grid::from_str("000\n000\n000\n").unwrap();

        assert_eq!(grid.columns, 3);
        assert_eq!(grid.rows(), 3);

        assert_eq!(grid.to_string(), "000\n000\n000\n");
    }

    #[test]
    fn test_insert_row() {
        let mut grid: Grid<char> = Grid::from_str("000\n000\n000\n").unwrap();
        grid.insert_row(1, vec!['1', '2', '3']);
        assert_eq!(grid.to_string(), "000\n123\n000\n000\n");
    }
    #[test]
    fn test_insert_col() {
        let mut grid: Grid<char> = Grid::from_str("000\n000\n000\n").unwrap();
        grid.insert_col(1, vec!['1', '2', '3']);
        assert_eq!(grid.to_string(), "0100\n0200\n0300\n");
        assert_eq!(grid.rows(), 3);
        assert_eq!(grid.columns, 4);

        grid.insert_col(4, "456".chars().collect());
        assert_eq!(grid.to_string(), "01004\n02005\n03006\n");
        assert_eq!(grid.rows(), 3);
        assert_eq!(grid.columns, 5);
    }

    #[test]
    fn test_iter_cols() {
        let grid: Grid<char> = Grid::from_str("123\n456\n789\n").unwrap();
        let mut iter = grid.iter_columns();

        assert_eq!(iter.next().unwrap().into_iter().collect::<String>(), "147");
        assert_eq!(iter.next().unwrap().into_iter().collect::<String>(), "258");
        assert_eq!(iter.next().unwrap().into_iter().collect::<String>(), "369");
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_iter_rows() {
        let grid: Grid<char> = Grid::from_str("123\n456\n789\n").unwrap();
        let mut iter = grid.iter_rows();

        assert_eq!(iter.next().unwrap().into_iter().collect::<String>(), "123");
        assert_eq!(iter.next().unwrap().into_iter().collect::<String>(), "456");
        assert_eq!(iter.next().unwrap().into_iter().collect::<String>(), "789");
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_iter_row_chaining() {
        let grid: Grid<char> = Grid::from_str("123\n456\n789\n").unwrap();
        let f = grid.iter_row(1).find(|e| **e == '5');
        assert_eq!(*f.unwrap(), '5');
    }

    #[test]
    fn test_iter_rows_chaining() {
        let grid: Grid<char> = Grid::from_str("123\n000\n789\n").unwrap();
        let f = grid
            .iter_rows()
            .enumerate()
            .find(|(i, v)| v.iter().all(|e| e == &&'0'));
        assert_eq!(f.unwrap().0, 1);
    }

    #[test]
    fn test_drop_row() {
        let mut grid: Grid<char> = Grid::from_str("123\n456\n789\n").unwrap();
        grid.drop_row(1);
        assert_eq!(grid.to_string(), "123\n789\n");
        assert_eq!(grid.rows(), 2);
        assert_eq!(grid.columns, 3);
    }

    #[test]
    fn test_drop_column() {
        let mut grid: Grid<char> = Grid::from_str("123\n456\n789\n").unwrap();
        grid.drop_column(1);
        assert_eq!(grid.to_string(), "13\n46\n79\n");
        assert_eq!(grid.rows(), 3);
        assert_eq!(grid.columns, 2);
    }
}
