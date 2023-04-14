//black --> negatives
//red --> positives
//|1| --> man  |2| --> king  0 --> empty

use crate::errors::CheckersError;

/// Moves that can be taken on a tile
pub enum Action {
    MoveNorthwest,
    MoveNortheast,
    MoveSouthwest,
    MoveSoutheast,
    JumpNorthwest,
    JumpNortheast,
    JumpSouthwest,
    JumpSoutheast,
}

pub enum CheckersResult {
    Ok,            //your turn is over with no captures
    Capture,       //your turn is over with captures
    Win,           //win
    Return,        //your turn requires more action, with no captures
    ReturnCapture, //your turn requires more action, with captures
}

pub struct Engine {
    pub board_red: [i8; 32],
    pub board_black: [i8; 32],
    pub current_player: bool, //true = red, false = black
    pub red_pieces: u8,
    pub black_pieces: u8,
}
impl Engine {
    pub fn new() -> Self {
        let mut board = [0; 32];

        // set the red pieces
        board[0] = 1;
        board[1] = 1;
        board[2] = 1;
        board[3] = 1;
        board[4] = 1;
        board[5] = 1;
        board[6] = 1;
        board[7] = 1;
        board[8] = 1;
        board[10] = 1;
        board[12] = 1;
        board[14] = 1;

        // set the black pieces
        board[17] = -1;
        board[19] = -1;
        board[21] = -1;
        board[23] = -1;
        board[24] = -1;
        board[25] = -1;
        board[26] = -1;
        board[27] = -1;
        board[28] = -1;
        board[29] = -1;
        board[30] = -1;
        board[31] = -1;

        Engine {
            board_red: board.clone(),
            board_black: board,
            current_player: true,
            red_pieces: 12,
            black_pieces: 12,
        }
    }

    /// Returns the tile a piece would land on given a specific action, can return invalid tiles
    pub fn action_on_tile(tile: u8, action: Action) -> u8 {
        match action {
            Action::MoveNorthwest => {
                if tile % 2 == 0 {
                    tile - 9
                } else {
                    tile - 1
                }
            }
            Action::MoveNortheast => {
                if tile % 2 == 0 {
                    tile - 7
                } else {
                    tile + 1
                }
            }
            Action::MoveSouthwest => {
                if tile % 2 == 0 {
                    tile - 1
                } else {
                    tile + 7
                }
            }
            Action::MoveSoutheast => {
                if tile % 2 == 0 {
                    tile + 1
                } else {
                    tile + 9
                }
            }
            Action::JumpNorthwest => tile - 10,
            Action::JumpNortheast => tile - 6,
            Action::JumpSouthwest => tile + 6,
            Action::JumpSoutheast => tile + 10,
        }
    }

    /// Checks if the move can be completed for this player
    pub fn is_move_valid(&self, tile: u8, action: Action) -> bool {
        let board: &[i8; 32];
        if self.current_player {
            //red's turn
            board = &self.board_red;
        } else {
            //black's turn
            board = &self.board_black;
        }

        match action {
            Action::MoveNorthwest => {
                if (tile % 2 == 0) && (tile % 8 == 0 || tile < 8) {
                    return false;
                } //left and top edge
            }
            Action::MoveNortheast => {
                if tile % 2 == 0 {
                    if tile < 8 {
                        return false;
                    }
                }
                //top edge
                else {
                    if ((tile + 1) % 8) == 0 {
                        return false;
                    }
                } //right edge
            }
            Action::MoveSouthwest => {
                if tile % 2 == 0 {
                    if tile % 8 == 0 {
                        return false;
                    }
                }
                //left edge
                else {
                    if tile > 23 {
                        return false;
                    }
                } //bottom edge
            }
            Action::MoveSoutheast => {
                if (tile % 2 == 1) && (tile > 23 || ((tile + 1) % 8 == 0)) {
                    return false;
                } //right and bottom edge
            }
            Action::JumpNorthwest => {
                if (tile < 8) || (tile % 8 == 0) || (tile - 1) % 8 == 0 {
                    //left 2 and top 2 edges
                    return false;
                }
                if board[Engine::action_on_tile(tile, Action::MoveNorthwest) as usize] >= 0 {
                    return false;
                }
            }
            Action::JumpNortheast => {
                if (tile < 8) || ((tile + 1) % 8 == 0) || ((tile + 2) % 8 == 0) {
                    //right 2 and top 2 edges
                    return false;
                }
                if board[Engine::action_on_tile(tile, Action::MoveNortheast) as usize] >= 0 {
                    return false;
                }
            }
            Action::JumpSouthwest => {
                if (tile > 23) || (tile % 8 == 0) || ((tile - 1) % 8 == 0) {
                    //left 2 and bottom 2 edges
                    return false;
                }
                if board[Engine::action_on_tile(tile, Action::MoveSouthwest) as usize] >= 0 {
                    return false;
                }
            }
            Action::JumpSoutheast => {
                if (tile > 23) || ((tile + 1) % 8 == 0) || ((tile + 2) % 8 == 0) {
                    //right 2 and bottom 2 edges
                    return false;
                }
                if board[Engine::action_on_tile(tile, Action::MoveSoutheast) as usize] >= 0 {
                    return false;
                }
            }
        }

        board[Engine::action_on_tile(tile, action) as usize] == 0 //make sure spot is open
    }

    //performs the specified move or defines the error message if the move is invalid
    pub fn make_move(&mut self, tile: u8, action: Action) -> Result<CheckersResult, CheckersError> {
        if !Engine::is_move_valid(&self, tile, action) {
            return Err(CheckersError::IllegalMove);
        }
        Ok(CheckersResult::Ok)
    }

    /// Get a reference to the board for red
    pub fn peak_red(&self) -> &[i8; 32] {
        &self.board_red
    }
    /// Get a reference to the board for black
    pub fn peak_black(&self) -> &[i8; 32] {
        &self.board_black
    }

    /// Get a copy of the board for red
    pub fn peak_red_python(&self) -> [i8; 32] {
        self.board_red.clone()
    }
    /// Get a copy of the board for black
    pub fn peak_black_python(&self) -> [i8; 32] {
        self.board_black.clone()
    }

    /// Prints the current state of the board, for debugging
    pub fn print_board(&self) {
        //println!("{:?}", self.board);
    }
}
