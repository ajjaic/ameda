//! A 2D cell grid represented as a linear Vector. It can be used in applications that require
//! manipulating specific sets of cells in the grid. For instance, you could get a set of the
//! indexes of all the right most cells, left most cells, middle cells of the grid or even the
//! neighbors of a specific cell. It is well suited for implementing different kinds of cellular
//! automatons.
//!
//! # Examples
//!
//! ```
//! use ameda::GridIndex;
//!
//! let grid = GridIndex::new(8, 8).unwrap();
//! assert_eq!(grid.right_column_indices(), &vec![7, 15, 23, 31, 39, 47, 55, 63]);
//! assert_eq!(grid.bottom_row_indices(), &vec![56, 57, 58, 59, 60, 61, 62, 63]);
//! ```

#[derive(Debug, PartialEq)]
pub struct GridIndex {
    grid_length: usize,
    grid_height: usize,
    total_indices: usize,
    top_left_corner: usize,
    top_right_corner: usize,
    bottom_left_corner: usize,
    bottom_right_corner: usize,
    left_column_indices: Vec<usize>,
    right_column_indices: Vec<usize>,
    top_row_indices: Vec<usize>,
    bottom_row_indices: Vec<usize>,
    middle_indices: Vec<usize>,
}

impl GridIndex {
    /// Constructs a new 2D grid of cells that are `grid_length` cells wide and `grid_height`
    /// cells high. The total number of cells in the grid would be a product of both the
    /// `grid_length` and `grid_height`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ameda::GridIndex;
    ///
    /// let grid = GridIndex::new(8, 8).unwrap();
    /// assert_eq!(grid.cell_count(), 64);
    /// let grid = GridIndex::new(5, 3).unwrap();
    /// assert_eq!(grid.cell_count(), 15);
    ///
    /// // The minimum grid size is 2x2. The maximum is 511, 511.
    /// assert_eq!(GridIndex::new(550, 440), None);
    /// assert_eq!(GridIndex::new(1, 10), None);
    /// ```
    pub fn new(grid_length: usize, grid_height: usize) -> Option<GridIndex> {
        match (grid_length, grid_height) {
            (x, y) if x > 1 && y > 1 && x < 512 && x < 512 => {
                let total_indices = grid_length * grid_height;

                let mut grid = GridIndex {
                    grid_length: grid_length,
                    grid_height: grid_height,
                    total_indices: total_indices,
                    top_left_corner: 0,
                    top_right_corner: (grid_length - 1),
                    bottom_left_corner: total_indices - grid_length,
                    bottom_right_corner: total_indices - 1,
                    left_column_indices: vec![],
                    right_column_indices: vec![],
                    top_row_indices: vec![],
                    bottom_row_indices: vec![],
                    middle_indices: vec![],
                };

                grid.top_row_indices = grid.row_indices(0);
                grid.bottom_row_indices = grid.row_indices(grid_height - 1);
                grid.left_column_indices = grid.column_indices(0);
                grid.right_column_indices = grid.column_indices(grid_length - 1);
                grid.middle_indices();
                Some(grid)
            }
            _ => None,
        }
    }

    /// Returns the number of cells in the grid
    ///
    /// # Example
    ///
    /// ```
    /// use ameda::GridIndex;
    ///
    /// let grid = GridIndex::new(8, 8).unwrap();
    /// assert_eq!(grid.cell_count(), 64);
    /// ```
    pub fn cell_count(&self) -> usize {
        self.total_indices
    }

    /// Returns the indices in any the rows in the grid. 0-indexed. The first row in the grid would
    /// be at the 0th index.
    pub fn row_cell_indexes(&self, row: usize) -> Option<Vec<usize>> {
        if row >= self.grid_height {
            None
        } else {
            Some(self.row_indices(row))
        }
    }

    /// Returns the indices in any the columns in the grid. 0-indexed. The first column in the grid
    /// would be at the 0th index.
    pub fn col_cell_indexes(&self, column: usize) -> Option<Vec<usize>> {
        if column >= self.grid_length {
            None
        } else {
            Some(self.column_indices(column))
        }
    }

    pub fn top_row_indices(&self) -> &Vec<usize> {
        &self.top_row_indices
    }

    pub fn left_column_indices(&self) -> &Vec<usize> {
        &self.left_column_indices
    }

    pub fn right_column_indices(&self) -> &Vec<usize> {
        &self.right_column_indices
    }

    pub fn bottom_row_indices(&self) -> &Vec<usize> {
        &self.bottom_row_indices
    }

    pub fn rt_i(&self, src_index: usize) -> Option<usize> {

        self.neighbor_index(src_index, "rt")
    }

    pub fn dr_i(&self, src_index: usize) -> Option<usize> {
        self.neighbor_index(src_index, "dr")
    }

    pub fn dn_i(&self, src_index: usize) -> Option<usize> {
        self.neighbor_index(src_index, "dn")
    }

    pub fn dl_i(&self, src_index: usize) -> Option<usize> {
        self.neighbor_index(src_index, "dl")
    }

    pub fn lt_i(&self, src_index: usize) -> Option<usize> {
        self.neighbor_index(src_index, "lt")
    }

    pub fn ul_i(&self, src_index: usize) -> Option<usize> {
        self.neighbor_index(src_index, "ul")
    }

    pub fn up_i(&self, src_index: usize) -> Option<usize> {
        self.neighbor_index(src_index, "up")
    }

    pub fn ur_i(&self, src_index: usize) -> Option<usize> {
        self.neighbor_index(src_index, "ur")
    }


    fn row_indices(&self, row: usize) -> Vec<usize> {
        let start_index = self.grid_length * row;
        let end_index = (self.grid_length * (row + 1)) - 1;

        let mut v = Vec::with_capacity(self.grid_length);
        for i in start_index..(end_index + 1) {
            v.push(i);
        }
        v
    }

    fn column_indices(&self, column: usize) -> Vec<usize> {
        let mut v = Vec::with_capacity(self.grid_height);
        for i in 0..self.grid_height {
            v.push((self.grid_length * i) + column)
        }
        v
    }

    fn middle_indices(&mut self) {
        for i in 0..self.total_indices {
            if !self.left_column_indices.contains(&i) && !self.right_column_indices.contains(&i) &&
               !self.top_row_indices.contains(&i) &&
               !self.bottom_row_indices.contains(&i) {
                self.middle_indices.push(i);
            }

        }
    }

    fn neighbor_index(&self, src_index: usize, neighbor: &str) -> Option<usize> {

        let indices_to_check = match neighbor {
            "rt" => (vec![&self.right_column_indices], Some(src_index + 1)),
            "dr" => {
                (vec![&self.right_column_indices, &self.bottom_row_indices],
                 Some(src_index + self.grid_length + 1))
            }
            "dn" => (vec![&self.bottom_row_indices], Some(src_index + self.grid_length)),
            "dl" => {
                (vec![&self.left_column_indices, &self.bottom_row_indices],
                 Some(src_index + self.grid_length - 1))
            }
            "lt" => {
                (vec![&self.left_column_indices],
                 {
                     if src_index != 0 {
                         Some(src_index - 1)
                     } else {
                         None
                     }
                 })
            }
            "ul" => {
                (vec![&self.left_column_indices, &self.top_row_indices],
                 {
                     if src_index < (self.grid_length + 1) {
                         None
                     } else {
                         Some(src_index - self.grid_length - 1)
                     }
                 })
            }
            "up" => {
                (vec![&self.top_row_indices],
                 {
                     if src_index < self.grid_length {
                         None
                     } else {
                         Some(src_index - self.grid_length)
                     }
                 })
            }
            "ur" => {
                (vec![&self.right_column_indices, &self.top_row_indices],
                 {
                     if src_index < self.grid_length {
                         None
                     } else {
                         Some(src_index - self.grid_length + 1)
                     }
                 })
            }
            _ => (vec![], None),
        };

        if src_index < self.total_indices &&
           !indices_to_check.0.iter().any(|v| v.contains(&src_index)) {
            indices_to_check.1
        } else {
            None
        }
    }


    // fn position(&self, index: usize) -> Pos {
    //     match index {
    //         0 => Pos::TopLeftCorner,
    //         _ => Pos::TopRightCorner, // incorrect
    //     }
    // }
}

// enum Pos {
//     TopLeftCorner,
//     TopRightCorner,
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<GridIndex> {
        let create_grid = |x, y| match GridIndex::new(x, y) {
            Some(a) => a,
            None => panic!(),
        };
        vec![create_grid(8, 8),
             create_grid(8, 4),
             create_grid(2, 2),
             create_grid(8, 7),
             create_grid(5, 3),
             create_grid(12, 10),
             create_grid(10, 5),
             create_grid(20, 20),
             create_grid(123, 115)]
    }

    #[test]
    fn test_grid_extremes() {
        for g in test_data() {
            assert_eq!(*g.top_row_indices(),
                       (0..(g.top_right_corner + 1)).collect::<Vec<usize>>());

            assert_eq!(*g.bottom_row_indices(),
                       (g.bottom_left_corner..(g.bottom_right_corner + 1)).collect::<Vec<usize>>());

            assert_eq!(*g.left_column_indices(),
                       (0..(g.grid_height))
                           .map(|x| g.grid_length * x)
                           .collect::<Vec<usize>>());

            assert_eq!(*g.right_column_indices(),
                       (0..(g.grid_height))
                           .map(|x| (g.grid_length * (x + 1)) - 1)
                           .collect::<Vec<usize>>());
        }
    }

    #[test]
    fn test_neighbors() {
        for g in test_data() {
            assert_eq!(Some(g.top_left_corner + 1), g.rt_i(g.top_left_corner));
            assert_eq!(Some(g.top_left_corner + g.grid_length + 1),
                       g.dr_i(g.top_left_corner));
            assert_eq!(Some(g.top_left_corner + g.grid_length),
                       g.dn_i(g.top_left_corner));
            assert_eq!(None, g.dl_i(g.top_left_corner));
            assert_eq!(None, g.lt_i(g.top_left_corner));
            assert_eq!(None, g.ul_i(g.top_left_corner));
            assert_eq!(None, g.up_i(g.top_left_corner));
            assert_eq!(None, g.ur_i(g.top_left_corner));

            assert_eq!(None, g.rt_i(g.top_right_corner));
            assert_eq!(None, g.dr_i(g.top_right_corner));
            assert_eq!(Some(g.top_right_corner + g.grid_length),
                       g.dn_i(g.top_right_corner));
            assert_eq!(Some(g.top_right_corner + g.grid_length - 1),
                       g.dl_i(g.top_right_corner));
            assert_eq!(Some(g.top_right_corner - 1), g.lt_i(g.top_right_corner));
            assert_eq!(None, g.ul_i(g.top_right_corner));
            assert_eq!(None, g.up_i(g.top_right_corner));
            assert_eq!(None, g.ur_i(g.top_right_corner));

            assert_eq!(Some(g.bottom_left_corner + 1), g.rt_i(g.bottom_left_corner));
            assert_eq!(None, g.dr_i(g.bottom_left_corner));
            assert_eq!(None, g.dn_i(g.bottom_left_corner));
            assert_eq!(None, g.dl_i(g.bottom_left_corner));
            assert_eq!(None, g.lt_i(g.bottom_left_corner));
            assert_eq!(None, g.ul_i(g.bottom_left_corner));
            assert_eq!(Some(g.bottom_left_corner - g.grid_length),
                       g.up_i(g.bottom_left_corner));
            assert_eq!(Some(g.bottom_left_corner - g.grid_length + 1),
                       g.ur_i(g.bottom_left_corner));

            assert_eq!(None, g.rt_i(g.bottom_right_corner));
            assert_eq!(None, g.dr_i(g.bottom_right_corner));
            assert_eq!(None, g.dn_i(g.bottom_right_corner));
            assert_eq!(None, g.dl_i(g.bottom_right_corner));
            assert_eq!(Some(g.bottom_right_corner - 1),
                       g.lt_i(g.bottom_right_corner));
            assert_eq!(Some(g.bottom_right_corner - g.grid_length - 1),
                       g.ul_i(g.bottom_right_corner));
            assert_eq!(Some(g.bottom_right_corner - g.grid_length),
                       g.up_i(g.bottom_right_corner));
            assert_eq!(None, g.ur_i(g.bottom_right_corner));

            for rnd_i in &g.middle_indices {
                assert_eq!(Some(*rnd_i + 1), g.rt_i(*rnd_i));
                assert_eq!(Some(*rnd_i + g.grid_length + 1), g.dr_i(*rnd_i));
                assert_eq!(Some(*rnd_i + g.grid_length), g.dn_i(*rnd_i));
                assert_eq!(Some(*rnd_i + g.grid_length - 1), g.dl_i(*rnd_i));
                assert_eq!(Some(*rnd_i - 1), g.lt_i(*rnd_i));
                assert_eq!(Some(*rnd_i - g.grid_length - 1), g.ul_i(*rnd_i));
                assert_eq!(Some(*rnd_i - g.grid_length), g.up_i(*rnd_i));
                assert_eq!(Some(*rnd_i - g.grid_length + 1), g.ur_i(*rnd_i));
            }
        }
    }


    // type Index = usize;
    // type Size = usize;

    // struct Board {
    //     size: Size,
    //     cells: Vec<Cell>,
    // }

    // struct Right(Index);
    // struct DownRight(Index);
    // struct Down(Index);

    // enum Cell {
    //     TopLeftCorner(CellState, Right, DownRight, Down),
    // }

    // enum CellState {
    //     Alive,
    //     Dead,
    // }

    // enum Neighbor {
    //     Right(Index),
    //     DownRight(Index),
    //     Down(Index),
    // }

    // #[test]
    // fn game_of_life() {
    //     let mut b = Board {
    //         size: 8,
    //         cells: vec![],
    //     };
    //     let grid_index_processor = GridIndex::new((8, 8));

    //     for i in 0..64 {
    //         match grid_index_processor.position(i) {

    //             Pos::TopLeftCorner => {
    //                 b.cells.push(Cell::TopLeftCorner(CellState::Dead,
    //                                                  Right(grid_index_processor.rt_index(i)
    //                                                      .unwrap()),
    //                                                  DownRight(grid_index_processor.dr_index(i)
    //                                                      .unwrap()),
    //                                                  Down(grid_index_processor.dn_index(i)
    //                                                      .unwrap())));
    //             }
    //             _ => (),

    //         }

    //     }
    // }
}
