#[derive(Clone)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            grid: vec![T::default(); rows*cols],
        }
    }

    pub fn _from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            grid: grid.to_vec(),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.grid[row*self.rows + col]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        self.grid[row*self.rows + col] = value;
    }

    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let t = row < self.rows - 1;
        let b = 0 < row;
        let r = col < self.cols - 1;
        let l = 0 < col;

        let mut ret = vec![];

        if t == true { ret.push((row+1, col)) }
        if b == true { ret.push((row-1, col)) }
        if r == true { ret.push((row, col+1)) }
        if l == true { ret.push((row, col-1)) }

        if t == true && r == true { ret.push((row+1, col+1)) }
        if t == true && l == true { ret.push((row+1, col-1)) }
        if b == true && r == true { ret.push((row-1, col+1)) }
        if b == true && l == true { ret.push((row-1, col-1)) }

        ret
    }
}
