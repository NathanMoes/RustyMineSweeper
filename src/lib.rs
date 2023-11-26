use prompted::*;
use rand::Rng;
use std::fmt;
use std::marker::PhantomData;

#[derive(Clone, PartialEq)]
enum CellState {
    Hidden,
    Revealed,
    Flagged,
}
#[derive(Clone)]
pub struct BoardSquare {
    state: CellState,
    value: isize,
    is_mine: bool,
}

#[derive(Debug)]
/// Board object for any arbitrary type
/// Values for the board itself and width + height properties for it's limits
pub struct Board<BoardSquare> {
    board: Vec<Vec<BoardSquare>>,
    pub width: usize,
    pub height: usize,
    _marker: PhantomData<BoardSquare>,
}

/// Clone implementation for board of arbitrary type.
/// Essentially it just clones the vec and cop
impl<BoardSquare: Clone> Clone for Board<BoardSquare> {
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

impl<BoardSquare> Default for Board<BoardSquare> {
    fn default() -> Self {
        Board {
            board: Vec::new(),
            width: 5,
            height: 5,
            _marker: PhantomData,
        }
    }
}

/// Default implementation for the Board. Including basic features
impl<BoardSquare> Board<BoardSquare>
where
BoardSquare: Clone + Default + std::cmp::PartialEq,
{
    /// Creates a new Board object with given width and height dimensions with default values for the type
    /// # Examples
    ///
    /// ```
    /// // create i32 Board
    /// use rusty_mine_sweeper::Board;
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
    /// use rusty_mine_sweeper::Board;
    /// let width = 4;
    /// let height = 3;
    /// let board: Board<i32> = Board::new(width, height);
    ///
    /// assert_eq!(board.width, width);
    /// assert_eq!(board.height, height);
    /// ```
    pub fn new(width: usize, height: usize) -> Board<BoardSquare> {
        let mut board = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(BoardSquare::default());
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
    /// use rusty_mine_sweeper::Board;
    /// let width = 4;
    /// let height = 3;
    /// let board: Board<i32> = Board::new(width, height);

    /// assert!(board.get(width, height).is_none());
    /// assert!(board.get(width + 1, 0).is_none());
    /// assert!(board.get(0, height + 1).is_none());
    /// ```
    pub fn get(&self, x: usize, y: usize) -> Option<&BoardSquare> {
        self.board.get(y).and_then(|row| row.get(x))
    }

    /// Sets the value for a given index to the value passed in on the board
    /// # Examples
    /// ```
    /// use rusty_mine_sweeper::Board;
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
    pub fn set(&mut self, x: usize, y: usize, value: BoardSquare) {
        if y < self.height && x < self.width {
            self.board[y][x] = value;
        }
    }

    /// Creates a Iterator that gives a mutable reference to the values on the board
    /// # Examples
    /// ```
    /// use rusty_mine_sweeper::Board;
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
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Vec<BoardSquare>> {
        self.board.iter_mut()
    }
}

impl Board<BoardSquare> {
    /// Creates a Board of type isize with default values of -1
    /// # Examples
    /// ```
    /// use rusty_mine_sweeper::Board;
    /// use rusty_mine_sweeper::BoardSquare;
    /// let mut board: Board<BoardSquare> = Board::isize_board(5, 4);
    /// ```
    pub fn isize_board(width: usize, height: usize) -> Board<BoardSquare> {
        let mut board = Vec::with_capacity(height);
        for _ in 0..height {
            let row = vec![
                BoardSquare {
                    value: -1,
                    state: CellState::Hidden,
                    is_mine: false
                };
                width
            ];
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
                if !self.board[y][x].is_mine {
                    self.board[y][x].is_mine = true;
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
                if self.board[y_index][x_index].is_mine {
                    count += 1;
                }
            }
        }
        count
    }

    fn update_board(&mut self, x: usize, y: usize) {
        // First, update the clicked square itself
        self.board[y][x].value = self.check_square(x, y);
        self.board[y][x].state = CellState::Revealed;

        // Then, update each of the eight surrounding squares
        for y_index in y.saturating_sub(1)..=y + 1 {
            for x_index in x.saturating_sub(1)..=x + 1 {
                // Skip the clicked square itself, as it's already updated
                if x_index == x && y_index == y {
                    continue;
                }
                if x_index < self.width && y_index < self.height && x == x_index && y_index == y {
                    // Update each surrounding square
                    self.board[y_index][x_index].value = self.check_square(x_index, y_index);
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
                    if self.board[row_index][col_index].is_mine {
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
                    if self.board[row_index][col_index].state == CellState::Hidden {
                        self.board[row_index][col_index].state = CellState::Flagged;
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

    pub fn is_won(&self) -> Option<()> {
        for row in self.board.iter() {
            for square in row.iter() {
                if square.state != CellState::Flagged && square.is_mine {
                    return None;
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
impl fmt::Display for Board<BoardSquare> {
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

            for (j, square) in row.iter().enumerate() {
                if j > 0 {
                    write!(f, " | ")?;
                }

                // Display logic based on the state and value of BoardSquare
                match square.state {
                    CellState::Hidden => {
                        if square.value != -1 {
                            write!(f, "{}", square.value)?
                        } else {
                            write!(f, "{}", EMPTY_SQUARE)?
                        }
                    }
                    CellState::Revealed => {
                        if square.is_mine {
                            write!(f, "*")?;
                        } else {
                            write!(f, "{}", square.value)?;
                        }
                    }
                    CellState::Flagged => write!(f, "{}", MARKED_SQUARE)?,
                }
            }
            writeln!(f, " |")?;
        }
        Ok(())
    }
}

/// Helper function to handle input from the user to be used for making a move
fn handle_input(max_width: usize, max_height: usize) -> Result<(usize, usize), &'static str> {
    let row = input!("Enter row selection (must be char): ");
    let row_index = match row.trim().bytes().next() {
        Some(byte) if byte.is_ascii_lowercase() => (byte - b'a').into(),
        _ => return Err("Invalid row selection. Please enter a character from 'a' to 'z'."),
    };

    let col = input!("Enter column selection (must be num): ");
    let col_index = match col.trim().parse::<usize>() {
        Ok(num) if num > 0 => num - 1,
        _ => return Err("Invalid column selection. Please enter a positive number."),
    };

    if row_index >= max_height {
        Err("Row selected is out of bounds")
    } else if col_index >= max_width {
        Err("Column selected is out of bounds")
    } else {
        Ok((row_index, col_index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
