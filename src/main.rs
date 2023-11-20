use prompted::input;
use rusty_mine_sweeper::*;

const MAX_HEIGHT: usize = 99;
const MAX_WIDTH: usize = 99;

fn main() {
    let (width, height) = match get_params() {
        Ok((w, h)) => (w, h),
        Err(_) => {
            println!(
                "incorrect params for the board. Please try ensure proper sizing with restart"
            );
            return;
        }
    };
    // let mut board: Board<bool> = Board::boolean_board(width, height);
    // println!("{}", board);
    // loop {
    //     if board.is_lost() {
    //         break;
    //     }
    //     board.make_move();
    //     println!("board after your move\n{}", board);
    //     match board.ai_make_move() {
    //         Some(()) => {
    //             println!("board after ai move\n{}", board);
    //             continue;
    //         }
    //         None => {
    //             break;
    //         }
    //     }
    // }
    let mut board: Board<isize> = Board::isize_board(width, height);
    board.place_mines();
    println!("{}", board);
    loop {
        match get_mark_square() {
            Ok(x) => {
                if x == "y" {
                    match board.mark_square() {
                        Ok(_) => {
                        }
                        Err(_) => {
                        }
                    }
                    println!("board after your mark\n{}", board);
                }
            }
            Err(_) => {
                println!("invalid move");
                continue;
            }
        }
        match board.make_move() {
            Ok(_) => {
                println!("board after your move\n{}", board);
                continue;
            },
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
}

fn get_params() -> Result<(usize, usize), &'static str> {
    let width_input = input!("Enter the width you wish for the board\n");
    let width: usize = match width_input.trim().parse::<usize>().unwrap() {
        x if x > MAX_WIDTH => {
            println!("Width must be less than {}", MAX_WIDTH);
            return Err("Width too large");
        }
        x if x < 1 => {
            println!("Width must be greater than 0");
            return Err("Width too small");
        }
        x => x,
    };

    let height_input = input!("Enter the height you wish for the board\n");
    let height: usize = match height_input.trim().parse::<usize>().unwrap() {
        x if x > MAX_HEIGHT => {
            println!("Height must be less than {}", MAX_HEIGHT);
            return Err("Height too large");
        }
        x if x < 1 => {
            println!("Height must be greater than 0");
            return Err("Height too small");
        }
        x => x,
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