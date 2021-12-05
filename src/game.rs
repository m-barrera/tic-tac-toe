use std::io;
use crate::{player::{Player}, board::Board};
pub enum GameResult {
    WIN,
    DRAW, 
    PROGRESS
}

pub struct Game {
    board: Board,
}

impl Game  {
    pub fn new() -> Game {
        return Game {
            board: Board::new(),
        }
    }


    fn get_fill_position(&self) -> Result<(usize,usize), String>{
        let mut row:String = String::new();
        let mut column: String = String::new();

        println!("Select your desired row (numeric 0-2)");
        io::stdin()
            .read_line(&mut row)
            .expect("Unable to read input");

        let row_to_int: usize = match row.trim().parse() {
            Ok(row_to_int) => row_to_int,
            Err(e) => {
                return Err(e.to_string());
            }
            };

        println!("Select your desired column (numeric 0-2)");
        io::stdin()
            .read_line(&mut column)
            .expect("Unable to read input");

        let column_to_int: usize = match column.trim().parse() {
            Ok(column_to_int) => column_to_int,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        return Ok((row_to_int, column_to_int));
    }


    fn check_game_status(&self)-> Result<(Player, GameResult), GameResult>{
        for player in [Player::X, Player::O] {
           match self.board.check_horizontal(player){ 
               Ok(_) => return Ok((player, GameResult::WIN)),
               Err(_) => (),
           }
           match self.board.check_vertical(player){ 
                Ok(_) => return Ok((player, GameResult::WIN)),
                Err(_) => (),
            }
            match self.board.check_diagonal(player){ 
                Ok(_) => return Ok((player, GameResult::WIN)),
                Err(_) => (),
            }
        }
        if self.board.check_move_left(){
            return Err(GameResult::PROGRESS);
        }

        return Err(GameResult::DRAW);
    }

    pub fn next_turn (&self,current_player: Player) -> Player {
        match current_player {
            Player::O => return Player::X,
            Player::X => return Player::O,
        }
    }

    pub fn start(mut self){
        println!("Welcome to Rust based tic tac toe!");
        println!("You are the player O");
        self.board.draw_board();
        let mut player = Player::X;
        loop {
            loop{
                println!("Please make your move O");
                match self.get_fill_position() {
                    Ok(res) => match self.board.fill_cell(res.0, res.1, player){
                        Ok(_) => break,
                        Err(e) => {
                            println!("{}", e);
                            continue;
                        }
                    },
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    },
                }
            }
            player = self.next_turn(player);
            
            self.board.draw_board();
            match self.check_game_status() {
                Ok(res) => println!("The player {:?} won", res.0),
                Err(GameResult::PROGRESS) => continue,
                Err(GameResult::DRAW) => println!("The game ended in a draw"),
                _ => continue,
            }

       }
    }
}