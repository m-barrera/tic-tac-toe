use crate::utils::{printer};
use crate::player::{Player};


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Board {
    pub fields: [[Option<Player>;3];3]
}

impl Board {
    pub fn new() -> Board {
        return Board {
            fields: [
                [None, None, None],
                [None, None, None],
                [None, None, None],
            ]
        }
    }


    pub fn draw_board(&self) {
            println!("*-----*-----*-----*");
            println!("|{}|{}|{}|", 
                printer::option_to_string(&self.fields[0][0]), 
                printer::option_to_string(&self.fields[0][1]), 
                printer::option_to_string(&self.fields[0][2]));
            println!("*-----*-----*-----*");
            println!("|{}|{}|{}|",
                printer::option_to_string(&self.fields[1][0]),
                printer::option_to_string(&self.fields[1][1]), 
                printer::option_to_string(&self.fields[1][2]));
            println!("*-----*-----*-----*");
            println!("|{}|{}|{}|",
                printer::option_to_string(&self.fields[2][0]),
                printer::option_to_string(&self.fields[2][1]), 
                printer::option_to_string(&self.fields[2][2]));
            println!("*-----*-----*-----*");
    }

    pub fn check_horizontal(&self, player: Player) -> Result<Player, String>{

        for row in 0..2{
            if self.fields[row][0] == Some(player) && self.fields[row][1] == Some(player) && self.fields[row][2] == Some(player){
                return Ok(player);
            }
        }
        return Err(String::from("no match"));
    }

    pub fn check_vertical(&self, player: Player) -> Result<Player, String>{
        for column in 0..2 {
            if self.fields[0][column] == Some(player) &&  self.fields[1][column] == Some(player) && self.fields[2][column] == Some(player){
                return Ok(player);
            }
        }
        return Err(String::from("no match"));
    }

    pub fn check_diagonal(&self, player: Player) -> Result<Player, String>{
        if  self.fields[0][0] == Some(player) &&  self.fields[1][1] == Some(player) &&  self.fields[2][2] == Some(player){
            return Ok(player)
        }else if  self.fields[0][2] == Some(player) &&  self.fields[1][1] == Some(player) &&  self.fields[2][0] == Some(player){
            return Ok(player)
        }
        return Err(String::from("no match"))
    }

    pub fn check_move_left(&self) -> bool {
        for row in self.fields{
            for cell in row {
                	match cell {
                        Some(Player::O) => continue,
                        Some(Player::X) => continue,
                        None => return true,
                    }
            }
        }
        return false
    }

    fn check_move (&self, row: usize, column:usize) -> Result<String, String> {
        match &self.fields[row][column]{
            Some(Player::O) => return Err(String::from("allready occupied")),
            Some(Player::X) => return Err(String::from("allready occupied")),
            None => return Ok(String::from("valid move"))
        }
    }

    pub fn fill_cell(&mut self, row: usize, column: usize, player:Player ) -> Result<String, String> {
        match self.check_move(row, column) {
            Ok(_) => (),
            _ =>  return Err(String::from("invalid move"))
        };
        self.fields[row][column] = Some(player);
        return Ok(String::from("valid move"));
    }

}