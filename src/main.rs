use prompted::input;
use rusty_mine_sweeper::*;

const MAX_HEIGHT: usize = 99;
const MAX_WIDTH: usize = 99;

fn main() {
    println!("Welcome to rusty mine sweeper by Nathan Moes! Please note that all mine MUST be marked as flagged in order to win the game");
    let width: usize;
    let height: usize;
    loop {
        (width, height) = match get_params() {
            Ok((w, h)) => (w, h),
            Err(_) => {
                continue;
            }
        };
        break;
    }
    let mut score = 0;
    let mut board: Board<BoardSquare> = Board::isize_board(width, height);
    board.place_mines();
    println!("{}", board);
    loop {
        if board.is_won().is_some() {
            println!("You won!");
            break;
        }
        match get_mark_square() {
            Ok(x) => {
                if x == "y" {
                    println!("board after your mark\n{}", board);
                    continue;
                }
            }
            Err(_) => {
                println!("invalid move");
                continue;
            }
        }
        match board.make_move() {
            Ok(_) => {
                score += 1;
                println!("board after your move\n{}", board);
                continue;
            }
            Err(x) => {
                if x == "You lose" {
                    println!("You lose");
                    break;
                }
                println!("invalid move");
                continue;
            }
        }
    }
    println!("Your score is {}", score);
}

fn get_params() -> Result<(usize, usize), &'static str> {
    let width_input = input!("Enter the width you wish for the board\n");
    let width = match width_input.trim().parse::<usize>() {
        Ok(x) if x > MAX_WIDTH => {
            println!("Width must be less than {}", MAX_WIDTH);
            return Err("Width too large");
        }
        Ok(x) if x < 1 => {
            println!("Width must be greater than 0");
            return Err("Width too small");
        }
        Ok(x) => x,
        Err(_) => {
            println!("Invalid input for width. Please enter a valid number.");
            return Err("Invalid width input");
        }
    };

    let height_input = input!("Enter the height you wish for the board\n");
    let height = match height_input.trim().parse::<usize>() {
        Ok(x) if x > MAX_HEIGHT => {
            println!("Height must be less than {}", MAX_HEIGHT);
            return Err("Height too large");
        }
        Ok(x) if x < 1 => {
            println!("Height must be greater than 0");
            return Err("Height too small");
        }
        Ok(x) => x,
        Err(_) => {
            println!("Invalid input for height. Please enter a valid number.");
            return Err("Invalid height input");
        }
    };

    Ok((width, height))
}

fn get_mark_square() -> Result<&'static str, &'static str> {
    loop {
        let yes_or_no = input!("Would you like to mark a square?\n");
        match yes_or_no.trim().to_lowercase().as_str() {
            "yes" | "y" => {
                return Ok("y");
            }
            "no" | "n" => return Ok("n"),
            _ => {
                println!("Please enter yes or no");
                continue;
            }
        }
    }
}