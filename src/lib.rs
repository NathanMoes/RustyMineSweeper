use prompted::*;
use std::fmt;
use std::marker::PhantomData;
use rand::Rng;

enum CellState {
    Hidden,
    Revealed,
    Flagged,
}

pub struct BoardSquare {
    state: CellState,
    value: isize,
}

#[derive(Debug)]
/// Board object for any arbitrary type
/// Values for the board itself and width + height properties for it's limits
pub struct Board<T> {
    board: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
    _marker: PhantomData<T>,
}

/// Clone implementation for board of arbitrary type.
/// Essentially it just clones the vec and cop
impl<T: Clone> Clone for Board<T> {
    fn clone(&self) -> Self {
        let mut board = Vec::with_capacity(self.height);
        for row in self.board.iter() {
            board.push(row.clone());
        }
        Board {
            board,
            width: self.width,
            height: self.height,
            _marker: PhantomData,
        }
    }
}

impl<T> Default for Board<T> {
    fn default() -> Self {
        Board {
            board: Vec::new(),
            width: 5,
            height: 4,
            _marker: PhantomData,
        }
    }
}

/// Default implementation for the Board. Including basic features
impl<T> Board<T>
where
    T: Clone + Default + std::cmp::PartialEq,
{
    /// Creates a new Board object with given width and height dimensions with default values for the type
    /// # Examples
    ///
    /// ```
    /// // create i32 Board
    /// use chomp::Board;
    /// let width = 4;
    /// let height = 3;
    /// let board: Board<i32> = Board::new(width, height);
    ///
    /// for y in 0..height {
    ///     for x in 0..width {
    ///         assert_eq!(*board.get(x, y).unwrap(), i32::default());
    ///     }
    /// }
    /// ```
    /// ```
    /// // creates board with correct dimensions
    /// use chomp::Board;
    /// let width = 4;
    /// let height = 3;
    /// let board: Board<i32> = Board::new(width, height);
    ///
    /// assert_eq!(board.width, width);
    /// assert_eq!(board.height, height);
    /// ```
    pub fn new(width: usize, height: usize) -> Board<T> {
        let mut board = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(T::default());
            }
            board.push(row);
        }
        Board {
            board,
            width,
            height,
            _marker: PhantomData,
        }
    }

    /// Gets the given element by x and y coordinates for the type of the board.
    /// Returns an option with Some(&) for valid value in bounds else None
    /// # Examples
    /// ```
    /// use chomp::Board;
    /// let width = 4;
    /// let height = 3;
    /// let board: Board<i32> = Board::new(width, height);

    /// assert!(board.get(width, height).is_none());
    /// assert!(board.get(width + 1, 0).is_none());
    /// assert!(board.get(0, height + 1).is_none());
    /// ```
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.board.get(y).and_then(|row| row.get(x))
    }

    /// Sets the value for a given index to the value passed in on the board
    /// # Examples
    /// ```
    /// use chomp::Board;
    /// let width = 4;
    /// let height = 3;
    /// let mut value = 1;
    /// let mut board: Board<usize> = Board::new(width, height);
    ///
    /// board.set(0, 0, value);
    /// assert!(board.get(0, 0) == Some(&value));
    /// value = 3;
    /// board.set(2, 2, value);
    /// assert!(board.get(2, 2) == Some(&value));
    /// value = 5;
    /// board.set(1, 1, value);
    /// assert!(board.get(1, 1) == Some(&value));
    /// ```
    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if y < self.height && x < self.width {
            self.board[y][x] = value;
        }
    }

    /// Creates a Iterator that gives a mutable reference to the values on the board
    /// # Examples
    /// ```
    /// use chomp::Board;
    /// let width = 4;
    /// let height = 3;
    /// let mut board: Board<isize> = Board::new(width, height);
    /// for mut row in board.iter_mut() {
    ///     for mut element in row.iter_mut() {
    ///         *element += 1;
    ///     }
    /// }
    /// for mut row in board.iter_mut(){
    ///     for element in row.iter(){
    ///         assert!(*element == 1)
    ///     }
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Vec<T>> {
        self.board.iter_mut()
    }
}

impl Board<isize> {
    /// Creates a Board of type isize with default values of -1
    /// # Examples
    /// ```
    /// use chomp::Board;
    /// let mut board: Board<isize> = Board::isize_board(5, 4);
    /// ```
    pub fn isize_board(width: usize, height: usize) -> Board<isize> {
        let mut board = Vec::with_capacity(height);
        for _ in 0..height {
            let row = vec![-1; width];
            board.push(row);
        }
        Board {
            board,
            width,
            height,
            _marker: PhantomData,
        }
    }

    /// Randomly places mines (~10% of the squares) on the board
    pub fn place_mines(&mut self) {
        let total_squares = self.width * self.height;
        let mines_count = total_squares / 10; // Approximately 10% of total squares

        let mut rng = rand::thread_rng();

        for _ in 0..mines_count {
            let mut placed = false;
            while !placed {
                let x = rng.gen_range(0..self.width);
                let y = rng.gen_range(0..self.height);

                // Place a mine if the cell is not already a mine
                if self.board[y][x] != -10 {
                    self.board[y][x] = -10;
                    placed = true;
                }
            }
        }
    }

    // Checks any given square for the number of bombs around it aka the number -10 and will assign itself a given number reflecting that
    fn check_square(&self, x: usize, y: usize) -> isize {
        let mut count = 0;
        for y_index in y.saturating_sub(1)..=y + 1 {
            for x_index in x.saturating_sub(1)..=x + 1 {
                if x_index >= self.width || y_index >= self.height {
                    continue;
                }
                if self.board[y_index][x_index] == -10 {
                    count += 1;
                }
            }
        }
        count
    }

    fn update_board(&mut self, x: usize, y: usize) {
        // First, update the clicked square itself
        self.board[y][x] = self.check_square(x, y);
    
        // Then, update each of the eight surrounding squares
        for y_index in y.saturating_sub(1)..=y + 1 {
            for x_index in x.saturating_sub(1)..=x + 1 {
                // Skip the clicked square itself, as it's already updated
                if x_index == x && y_index == y {
                    continue;
                }
                if x_index < self.width && y_index < self.height {
                    // Update each surrounding square
                    self.board[y_index][x_index] = self.check_square(x_index, y_index);
                }
            }
        }
    }
    

    /// Gets input from the user and makes the given move
    pub fn make_move(&mut self) -> Result<(usize, usize), &'static str> {
        let mut move_made = false;
        while !move_made {
            match handle_input(self.width, self.height) {
                Ok((row_index, col_index)) => {
                    if self.board[row_index][col_index] == -10 {
                        return Err("You lose");
                    }
                    self.update_board(col_index, row_index);
                    move_made = true;
                }
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            }
        }
        Ok((0, 0))
    }

    pub fn mark_square(&mut self) -> Result<(), &'static str> {
        let mut move_made = false;
        while !move_made {
            match handle_input(self.width, self.height) {
                Ok((row_index, col_index)) => {
                    if self.board[row_index][col_index] == -1 {
                        self.board[row_index][col_index] = 10;
                        move_made = true;
                    } else {
                        println!("Invalid position selection. Please select a non selected square to mark");
                        continue;
                    }
                }
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            }
        }
        Ok(())
    }
}

impl Board<bool> {
    /// Creates a Board of type bool with default values of true
    /// # Examples
    /// ```
    /// use chomp::Board;
    /// let mut board: Board<bool> = Board::boolean_board(5, 4);
    /// ```
    pub fn boolean_board(width: usize, height: usize) -> Board<bool> {
        let mut board = Vec::with_capacity(height);
        for _ in 0..height {
            let row = vec![true; width];
            board.push(row);
        }
        Board {
            board,
            width,
            height,
            _marker: PhantomData,
        }
    }

    /// Determines if the state of the board is lost for the game of chomp
    pub fn is_lost(&self) -> bool {
        for (y, row) in self.board.iter().enumerate() {
            for (x, &square) in row.iter().enumerate() {
                if square && (x > 0 || y > 0) {
                    return false;
                }
            }
        }
        self.board[0][0]
    }

    /// Determines if there is a winning move for the AI to take.
    /// If there is, returns Some(x,y). Else None
    pub fn winning_move(posn: Board<bool>) -> Option<(usize, usize)> {
        if posn.is_lost() {
            return None;
        }
        for row in 0..posn.height {
            for col in 0..posn.width {
                if row == 0 && col == 0 || !posn.board[row][col] {
                    continue;
                }
                let mut p = posn.clone();
                p.chomp(col, row);
                let the_move = Self::winning_move(p);
                if the_move.is_none() {
                    return Some((col, row));
                }
            }
        }
        None
    }

    /// "Chomps" the board with the given cords given. Returns true if successful, else false.
    /// Chomp means to mark false elements to the right and below the given cord, including the cord
    /// # Examples
    /// ```
    /// // Chomp top left
    /// use chomp::Board;
    /// let mut board: Board<bool> = Board::boolean_board(5, 5);
    ///
    /// board.chomp(0, 0);
    ///
    /// for y in 0..5 {
    ///    assert_eq!(
    ///        *board.get(0, y).unwrap(),
    ///        false,
    ///        "Board value at ({}, {}) should be false after chomp",
    ///        2,
    ///         y
    ///    );
    /// }
    ///
    /// for x in 0..5 {
    ///    assert_eq!(
    ///        *board.get(x, 0).unwrap(),
    ///        false,
    ///        "Board value at ({}, {}) should be false after chomp",
    ///        x,
    ///        0
    ///    );
    /// }
    ///
    /// for x in 1..5 {
    ///    assert_eq!(
    ///        *board.get(x, 1).unwrap(),
    ///        true,
    ///        "Board value at ({}, {}) should be true after chomp",
    ///        x,
    ///        1
    ///    );
    /// }
    /// ```
    /// ```
    /// use chomp::Board;
    /// // Chomp bottom right
    /// let mut board: Board<bool> = Board::boolean_board(5, 5);
    ///
    /// board.chomp(4, 4);
    ///
    /// assert_eq!(
    ///     *board.get(4, 4).unwrap(),
    ///     false,
    ///     "Board at (4, 4) should be false after chomp"
    /// );
    ///
    /// for x in 0..4 {
    ///     assert_eq!(
    ///         *board.get(x, 0).unwrap(),
    ///         true,
    ///         "Board value at ({}, {}) should be false after chomp",
    ///         x,
    ///         0
    ///     );
    /// }
    ///
    /// for y in 0..4 {
    ///     assert_eq!(
    ///         *board.get(0, y).unwrap(),
    ///         true,
    ///         "Board value at ({}, {}) should be false after chomp ",
    ///         0,
    ///         y
    ///     );
    /// }
    /// ```
    pub fn chomp(&mut self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        for (y_index, row) in self.board.iter_mut().enumerate() {
            for (x_index, element) in row.iter_mut().enumerate() {
                if (y_index >= y && x_index == x) || (y_index == y && x_index >= x) {
                    *element = false;
                }
            }
        }
        true
    }

    /// Gets input from the user and makes the given move
    pub fn make_move(&mut self) {
        let mut move_made = false;
        while !move_made {
            match handle_input(self.width, self.height) {
                Ok((row_index, col_index)) => {
                    if !self.board[row_index][col_index] {
                        println!("Invalid position selection. Position is not playable");
                        continue;
                    }
                    self.chomp(col_index, row_index);
                    move_made = true;
                }
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            }
        }
    }

    /// finds the smallest chompable square and returns if it there is Some value, else None
    fn find_smallest_chomp(&self) -> Option<(usize, usize)> {
        let mut best_chomp = None;
        let mut min_affected = usize::MAX;

        for (y, row) in self.board.iter().enumerate() {
            for (x, &square) in row.iter().enumerate() {
                if square {
                    let affected = self.count_affected_squares(x, y);
                    if affected < min_affected {
                        min_affected = affected;
                        best_chomp = Some((x, y));
                    }
                }
            }
        }
        best_chomp
    }

    /// counts the number of affected squares if the cords were chomped
    fn count_affected_squares(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for (y_index, row) in self.board.iter().skip(y).enumerate() {
            for (x_index, &square) in row.iter().enumerate() {
                if square && ((x_index >= x && y_index == y) || (y_index >= y && x_index == x)) {
                    count += 1;
                }
            }
        }
        count
    }

    /// Makes the AI calculate and make a move
    /// Returns an option if it was successful or None if it has lost
    pub fn ai_make_move(&mut self) -> Option<()> {
        let mut move_made = false;
        while !move_made {
            match Self::winning_move(self.clone()) {
                Some((row_index, col_index)) => {
                    if !self.chomp(col_index, row_index) {
                        continue;
                    }
                    let row_label = (b'a' + row_index as u8) as char;
                    move_made = true;
                    println!("AI made move at (row:{}, col:{})", row_label, col_index + 1);
                }
                None => {
                    println!("No winning move present");
                    if self.is_lost() {
                        println!("AI lost, game is lost");
                        return None;
                    } else {
                        match self.find_smallest_chomp() {
                            Some((x, y)) => {
                                self.chomp(x, y);
                                let row_label = (b'a' + y as u8) as char;
                                println!("AI made move at (y:{}, x:{})", row_label, x + 1);
                                move_made = true;
                            }
                            None => {
                                println!("AI lost, AI is dummy dummy");
                                return None;
                            }
                        };
                    }
                }
            }
        }
        Some(())
    }
}

const EMPTY_SQUARE: char = '\u{25FB}';
const MARKED_SQUARE: char = '\u{1F6A9}';

/// Implementation for fmt::Display for the board
/// displays the given value for the item in each cord with 0..width and 0..height numbers and letters respectively
impl<T> fmt::Display for Board<T>
where
    T: fmt::Display + Clone + Default + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " ")?;
        for i in 0..self.board[0].len() {
            if i >= 10 {
                write!(f, "{}  ", i + 1)?;
            } else {
                write!(f, " {}  ", i + 1)?;
            }
        }
        writeln!(f)?;

        for (i, row) in self.board.iter().enumerate() {
            let row_label = (b'a' + i as u8) as char;
            write!(f, "{} ", row_label)?;

            for (j, val) in row.iter().enumerate() {
                if j > 0 {
                    write!(f, " | ")?;
                }

                if std::any::TypeId::of::<T>() == std::any::TypeId::of::<isize>() {
                    // Safely cast val to isize
                    let isize_val = unsafe { *(val as *const T as *const isize) };
                    if isize_val == -1 || isize_val == -10 {
                        write!(f, "{}", EMPTY_SQUARE)?;
                    } else if isize_val == 10 {
                        write!(f, "{}", MARKED_SQUARE)?;
                    } else {
                        write!(f, "{}", isize_val)?;
                    }
                } else {
                    write!(f, "{}", val)?;
                }
            }
            writeln!(f, " |")?;
        }
        Ok(())
    }
}

/// helper function to handle input from the user to be used for making a move
fn handle_input(max_width: usize, max_height: usize) -> Result<(usize, usize), &'static str> {
    let row = input!("Enter row selection (must be char): ");
    let row_index: usize = (row.trim().bytes().next().unwrap_or(b'a') - b'a').into();

    let col = input!("Enter column selection (must be num): ");
    let col_index = col.trim().parse::<usize>().unwrap_or(1) - 1;

    if row_index >= max_height {
        Err("Row selected is out of bounds")
    } else if col_index >= max_width {
        return Err("Column selected is out of bounds");
    } else {
        return Ok((row_index, col_index));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_can_be_set_with_all_bool_correct() {
        let width = 4;
        let height = 3;
        let mut board: Board<bool> = Board::boolean_board(width, height);

        assert!(board.get(0, 0) == Some(&true));
        assert!(board.get(1, 1) == Some(&true));
        assert!(board.get(2, 2) == Some(&true));
    }

    #[test]
    fn test_chomp_out_of_bounds() {
        let mut board: Board<bool> = Board::boolean_board(5, 5);

        assert!(board.chomp(5, 5) == false);

        for x in 0..5 {
            for y in 0..5 {
                assert_eq!(
                    *board.get(x, y).unwrap(),
                    true,
                    "Board value at ({}, {}) should be true after chomp",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn test_get_some_move_for_winning_move() {
        let board: Board<bool> = Board::boolean_board(5, 5);

        let posn = board.clone();

        let winning_move = Board::winning_move(posn);

        assert!(winning_move.is_some());
    }
}
