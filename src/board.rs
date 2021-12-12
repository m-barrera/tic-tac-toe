use crate::utils::{printer};
use crate::player::{Player};

/// BoardStruct holds the current Placed figures by every player
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Board {
    pub fields: [[Option<Player>;3];3]
}

/// Func Implmentation block of BoarStruct
impl Board {
    /// Creates a new Gameboard
    /// Each field can be filled by a Option<Player> which Player is an enum -> Player::X or Player::O
    /// Returns an instance of Board
    pub fn new() -> Board {
        return Board {
            fields: [
                [None, None, None],
                [None, None, None],
                [None, None, None],
            ]
        }
    }

    /// Draws the board onto the console 
    /// Referencing self to get the current ocupation of the single fields
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

    /// Checks if horizontal winning move
    /// self reference for fields
    /// player referense to check for individual player
    /// returns a Result<Player, String> -> if Ok(Player) -> Win condition was found
    pub fn check_horizontal(&self, player: Player) -> Result<Player, String>{

        for row in 0..2{
            if self.fields[row][0] == Some(player) && self.fields[row][1] == Some(player) && self.fields[row][2] == Some(player){
                return Ok(player);
            }
        }
        return Err(String::from("no match"));
    }

    /// Checks if vertical winning move
    /// self reference for fields
    /// player referense to check for individual player
    /// returns a Result<Player, String> -> if Ok(Player) -> Win condition was found
    pub fn check_vertical(&self, player: Player) -> Result<Player, String>{
        for column in 0..2 {
            if self.fields[0][column] == Some(player) &&  self.fields[1][column] == Some(player) && self.fields[2][column] == Some(player){
                return Ok(player);
            }
        }
        return Err(String::from("no match"));
    }

    /// Checks if diagonal winning move
    /// self reference for fields
    /// player referense to check for individual player
    /// returns a Result<Player, String> -> if Ok(Player) -> Win condition was found
    pub fn check_diagonal(&self, player: Player) -> Result<Player, String>{
        if  self.fields[0][0] == Some(player) &&  self.fields[1][1] == Some(player) &&  self.fields[2][2] == Some(player){
            return Ok(player)
        }else if  self.fields[0][2] == Some(player) &&  self.fields[1][1] == Some(player) &&  self.fields[2][0] == Some(player){
            return Ok(player)
        }
        return Err(String::from("no match"))
    }

    /// Checks if any move is left to be played or each field is occupied
    /// returns a boolean
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

    /// Check if move is valid and cell is not occupied
    /// Returns an Result<String,String> -> Ok(String) will proceed the mvoe
    fn check_move (&self, row: usize, column:usize) -> Result<String, String> {
        match &self.fields[row][column]{
            Some(Player::O) => return Err(String::from("allready occupied")),
            Some(Player::X) => return Err(String::from("allready occupied")),
            None => return Ok(String::from("valid move"))
        }
    }

    /// Fillls up a cell if move is valid
    /// return a Result<String,String> -> Ok valid move cell will be filled
    pub fn fill_cell(&mut self, row: usize, column: usize, player:Player ) -> Result<String, String> {
        match self.check_move(row, column) {
            Ok(_) => (),
            _ =>  return Err(String::from("invalid move"))
        };
        self.fields[row][column] = Some(player);
        return Ok(String::from("valid move"));
    }

}


#[cfg(test)]
mod test{
    #![allow(unused_results)]
    use crate::board::Board;
    use crate::player::Player;
     #[test]
    fn test_horizontal_win_by_X(){
        let mut b: Board = Board::new();
        b.fill_cell(0,0, Player::X);
        b.fill_cell(0,1, Player::X);
        b.fill_cell(0,2, Player::X);
        let expected = Ok(Player::X);
        assert_eq!(b.check_horizontal(Player::X), expected);
    }

    #[test]
    fn test_horizontal_win_by_O(){
        let mut b: Board = Board::new();
        b.fill_cell(0,0, Player::O);
        b.fill_cell(0,1, Player::O);
        b.fill_cell(0,2, Player::O);
        let expected = Ok(Player::O);
        assert_eq!(b.check_horizontal(Player::O), expected);
    }

    
    #[test]
    fn test_vertical_win_by_X(){
        let mut b: Board = Board::new();
        b.fill_cell(0,0, Player::X);
        b.fill_cell(1,0, Player::X);
        b.fill_cell(2,0, Player::X);
        let expected = Ok(Player::X);
        assert_eq!(b.check_vertical(Player::X), expected);
    }

    #[test]
    fn test_vertical_win_by_O(){
        let mut b: Board = Board::new();
        b.fill_cell(0,0, Player::O);
        b.fill_cell(1,0, Player::O);
        b.fill_cell(2,0, Player::O);
        let expected = Ok(Player::O);
        assert_eq!(b.check_vertical(Player::O), expected);
    }

    #[test]
    fn test_diagonal_win_by_X(){
        let mut b: Board = Board::new();
        b.fill_cell(0,0, Player::X);
        b.fill_cell(1,1, Player::X);
        b.fill_cell(2,2, Player::X);
        let expected = Ok(Player::X);
        assert_eq!(b.check_diagonal(Player::X), expected);
    }

    #[test]
    fn test_diagonal_win_by_O(){
        let mut b: Board = Board::new();
        b.fill_cell(0,0, Player::O);
        b.fill_cell(1,1, Player::O);
        b.fill_cell(2,2, Player::O);
        let expected = Ok(Player::O);
        assert_eq!(b.check_diagonal(Player::O), expected);
    }
    
}