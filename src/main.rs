use prompted::input;
use rusty_mine_sweeper::*;

const MAX_HEIGHT: usize = 99;
const MAX_WIDTH: usize = 99;

enum UserAction {
    Mark,
    Select,
}

fn main() {
    println!("Welcome to rusty mine sweeper by Nathan Moes! Please note that all mines MUST be marked/flagged in order to win the game");
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
    let mut board: Board<MinesweeperSquare> = Board::isize_board(width, height);
    if let Ok(x) = get_user_difficulty_level() {
        for _ in 0..x {
            board.increase_difficulty();
        }
    }
    println!("{}", board);
    loop {
        if board.is_won().is_some() {
            println!("You won!");
            break;
        }

        match get_user_action() {
            Ok(action) => match action {
                UserAction::Mark => {
                    loop {
                        if board.mark_square().is_err() {
                            println!("Invalid square to mark/flag. Please try again.");
                            continue;
                        }
                        break;
                    }
                    println!("Board after your mark/flag:\n{}", board);
                }
                UserAction::Select => match board.make_move() {
                    Ok(_) => {
                        score += 1;
                        println!("Board after your move:\n{}", board);
                    }
                    Err(x) => {
                        if x == "You lose" {
                            println!("You lose");
                            break;
                        }
                        println!("Invalid move");
                    }
                },
            },
            Err(_) => {
                println!("Invalid choice. Please try again.");
                continue;
            }
        }
    }
    println!("Your score is {}", score);
}

fn get_user_action() -> Result<UserAction, &'static str> {
    loop {
        let action = input!("What would you like to do?\n1. Mark/Flag a spot\n2. Select a spot\n");
        match action.trim() {
            "1" => return Ok(UserAction::Mark),
            "2" => return Ok(UserAction::Select),
            _ => {
                println!("Invalid input. Please enter 1 or 2.");
                continue;
            }
        }
    }
}

fn get_user_difficulty_level() -> Result<usize, &'static str> {
    loop {
        let action =
            input!("Select a difficulty level, up to 9. Each level = 10% more mines on board\n");
        match action.trim() {
            "1" => return Ok(1),
            "2" => return Ok(2),
            "3" => return Ok(3),
            "4" => return Ok(4),
            "5" => return Ok(5),
            "6" => return Ok(6),
            "7" => return Ok(7),
            "8" => return Ok(8),
            "9" => return Ok(9),
            _ => {
                println!("Invalid input. Please enter 1..9.");
                continue;
            }
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
}
