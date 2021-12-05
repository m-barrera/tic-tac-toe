use crate::{player::Player, board::Board};

pub fn ownedBy(board: Board, player:Player, row: usize, column: usize) -> bool{
    match board.fields[row][column] {
        Some(player) => return true,
        _ => return false,
    }
}