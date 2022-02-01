use std::io;
use std::io::prelude::*;
use rand::prelude::*;
use std::{thread, time};

fn main() {
    println!("Hello young person, are you into X(1) or O(2)?");
    game();
    println!("Goodbye!");
}

fn game() {
    let mut first_game = true;
    loop {
        if !first_game {
            println!("Choose your icon: X(1) or O(2)");
        }

        let choose_icon = read_line();

        let player_icon:char;
        let computer_icon:char;

        if choose_icon == "1" || choose_icon == "X" || choose_icon == "x" {
            println!("So you are X!");
            player_icon = 'X';
            computer_icon = 'O';
        } else if choose_icon == "2" || choose_icon == "O" || choose_icon == "o" {
            println!("So you are O!");
            player_icon = 'O';
            computer_icon = 'X';
        } else {
            println!("The input was: {}", choose_icon);
            println!("What? I do not understand....\nChoose between X(1) or O(2)");
            continue;
        }

        let mut positions:[char;9] = Default::default();
        update_board(positions);

        loop {
            //player phase
            println!("Your turn! - Place your marker between 1 - 9");
            let pos: usize;
            loop {
                match select_position() {
                    Ok(p) => {
                        if !validate_position(positions, p) {
                            println!("This space is already taken! Don't cheat!");
                            continue;
                        }
                        pos = p;
                        break;
                    },
                    Err(_) => {
                        println!("Invalid input");
                    }
                }
            }
            positions[pos] = player_icon;
            update_board(positions);            
            //player wins here
            if win_check(positions, player_icon) {
                println!("You won! Congratulations!");
                break;
            }

            //computer phase
            println!("Computer turn! It is thinking...");
            thread::sleep(time::Duration::from_millis(2000));
            
            loop {
                let pc_pos = computer_play();
                if !validate_position(positions, pc_pos) {
                    continue;                
                }
                positions[pc_pos] = computer_icon;
                break;
            }
            update_board(positions);

            //computer wins here
            if win_check(positions, computer_icon) {
                println!("You lost. Better luck next time.");
                break;
            }
        }

        first_game = false;

        if !play_again() {
            break;
        }
    }
}

fn play_again() -> bool {
    println!("Play again? (Y/N)");
    let ans = read_line();
    return ans == "Y" || ans == "y";
}

fn read_line() -> String {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let line_read = iterator.next().unwrap().unwrap();
    return line_read;
}

fn update_board(positions:[char;9]) {
    println!(" {} | {} | {} ",positions[0], positions[1], positions[2]);
    println!("---------");
    println!(" {} | {} | {} ",positions[3], positions[4], positions[5]);
    println!("---------");
    println!(" {} | {} | {} ",positions[6], positions[7], positions[8]);
}

fn select_position() -> Result<usize, usize> {    
    let line_read = read_line();

    if line_read.len() > 1 {
        return Err(1)
    } else if !line_read.chars().next().unwrap().is_numeric() {
        return Err(2)
    }

    Ok(line_read.parse::<usize>().unwrap() - 1)
}

fn validate_position(positions:[char;9], pos:usize) -> bool {
    positions[pos] != 'X' && positions[pos] != 'O'
}

fn computer_play() -> usize {
    let mut rng = rand::thread_rng();
    let n: i8 = rng.gen_range(0..9);
    n as usize
}

fn win_check(spaces:[char;9], icon: char) -> bool {

    return (spaces[0] == icon && spaces[1] == icon && spaces[2] == icon) //horizontal
    || (spaces[3] == icon && spaces[4] == icon && spaces[5] == icon) //horizontal
    || (spaces[6] == icon && spaces[7] == icon && spaces[8] == icon) //horizontal
    || (spaces[0] == icon && spaces[4] == icon && spaces[8] == icon) //diagonal
    || (spaces[6] == icon && spaces[4] == icon && spaces[2] == icon) //diagonal
    || (spaces[0] == icon && spaces[3] == icon && spaces[6] == icon) //vertical
    || (spaces[1] == icon && spaces[4] == icon && spaces[7] == icon) //vertical
    || (spaces[2] == icon && spaces[5] == icon && spaces[8] == icon); //vertical
}