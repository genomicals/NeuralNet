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


/// The result of making a move on the board
pub enum CheckersResult {
    Ok(bool),       //turn is over, contains the current player's turn
    Win,            //win
}


/// The engine for a Checkers game
pub struct Engine {
    pub board_red: [i8; 32],
    pub board_black: [i8; 32],
    pub current_player: bool, //true = red, false = black
    pub red_pieces: u8,
    pub black_pieces: u8,
}
impl Engine {
    pub fn new() -> Self {
        let board = [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, -1,0, -1,0, -1,0, -1,-1,-1,-1,-1,-1,-1,-1,-1];
        //           0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31

        Engine {
            board_red: board.clone(),
            board_black: board,
            current_player: true,
            red_pieces: 12,
            black_pieces: 12,
        }
    }

    pub fn with_turn(player: bool) -> Self {
        let board = [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, -1,0, -1,0, -1,0, -1,-1,-1,-1,-1,-1,-1,-1,-1];
        //           0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31

        Engine {
            board_red: board.clone(),
            board_black: board,
            current_player: player,
            red_pieces: 12,
            black_pieces: 12,
        }

    }

    /// Returns the tile a piece would land on given a specific action, can return invalid tiles
    pub fn action_on_tile(tile: u8, action: &Action) -> u8 {
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

    /// Checks if the move can be completed for this player.
    pub fn is_move_valid(&self, tile: u8, action: &Action) -> bool {
        let board = if self.current_player {&self.board_red} else {&self.board_black};

        if board[tile as usize] <= 0 { //ensures there's a moveable piece on this tile for this player
            return false;
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
                if board[Engine::action_on_tile(tile, &Action::MoveNorthwest) as usize] >= 0 {
                    return false;
                }
            }
            Action::JumpNortheast => {
                if (tile < 8) || ((tile + 1) % 8 == 0) || ((tile + 2) % 8 == 0) {
                    //right 2 and top 2 edges
                    return false;
                }
                if board[Engine::action_on_tile(tile, &Action::MoveNortheast) as usize] >= 0 {
                    return false;
                }
            }
            Action::JumpSouthwest => {
                if (tile > 23) || (tile % 8 == 0) || ((tile - 1) % 8 == 0) {
                    //left 2 and bottom 2 edges
                    return false;
                }
                if board[Engine::action_on_tile(tile, &Action::MoveSouthwest) as usize] >= 0 {
                    return false;
                }
            }
            Action::JumpSoutheast => {
                if (tile > 23) || ((tile + 1) % 8 == 0) || ((tile + 2) % 8 == 0) {
                    //right 2 and bottom 2 edges
                    return false;
                }
                if board[Engine::action_on_tile(tile, &Action::MoveSoutheast) as usize] >= 0 {
                    return false;
                }
            }
        }

        board[Engine::action_on_tile(tile, action) as usize] == 0 //make sure spot is open
    }

    // TODO Performs the specified move or defines the error message if the move is invalid
    pub fn make_move(&mut self, tile: u8, action: Action) -> Result<CheckersResult, CheckersError> {
        //let board = if self.current_player {&self.board_red} else {&self.board_black}; //retrieve the board

        if !Engine::is_move_valid(&self, tile, &action) { //ensures coordinates are respected and spaces are open
            return Err(CheckersError::IllegalMove);
        }

        // we need to first perform the move, then take any automatic actions if necessary
        // question for implementing: if the enemy makes a move, and the player is forced to move,
        // does this mean their whole turn has completed without them having a say in what they moved?
        // more research is required

        // perform the move, NOTE: in move cases the turn will switch to the opponent for sure, not necessarily true for jumps
        let landing_tile = Engine::action_on_tile(tile, &action);
        match &action {
            Action::MoveNorthwest | Action::MoveNortheast | Action::MoveSouthwest | Action::MoveSoutheast => {
                self.update_board(landing_tile, self.get_board()[tile as usize]); //copy piece to new tile
                self.update_board(tile, 0); //remove piece from current tile

                // CASE I
                // Give control to the other team, and check the four adjacent spaces for any automatic moves
                // If only one such case then execute automatically, otherwise ask the ai
                self.current_player = !self.current_player;
                return self.handle_inward_jump(landing_tile);
            },
            _ => {
                // CASE II
                // 1. Check adjacent spaces for additional jumps 
                //let outward_result = self.handle_outward_jump(landing_tile);

                //    If any adjacent space has an enemy tile, check the space behind for automatic 
                //    If only one such open jump then execute automatically, if multiple then ask the ai for a move
                // 2. After checking for move chain, or move extension, then give control to the other team and essentially do case 1
                //return self.handle_inward_jump(landing_tile);
                
                return self.handle_outward_jump(landing_tile);
            }
        }

        //todo!();

        //Ok(CheckersResult::Ok(self.current_player))
    }


    /// Checks and executes inward jumps towards the specified tile
    #[inline]
    fn handle_inward_jump(&mut self, landing_tile: u8) -> Result<CheckersResult, CheckersError> {
        let mut possible_moves = [false; 4];
        let directions = if landing_tile % 2 == 0 { //indices of the spaces around the landing tile
            [landing_tile - 9, landing_tile - 7, landing_tile - 1, landing_tile + 1]
        } else {
            [landing_tile - 1, landing_tile + 1, landing_tile + 7, landing_tile + 9]
        };

        possible_moves[0] = self.is_move_valid(directions[0], &Action::JumpSoutheast);
        possible_moves[1] = self.is_move_valid(directions[1], &Action::JumpSouthwest);
        possible_moves[2] = self.is_move_valid(directions[2], &Action::JumpNortheast);
        possible_moves[3] = self.is_move_valid(directions[3], &Action::JumpNorthwest);

        // get indices of trues
        let valid_bools: Vec<usize> = possible_moves.iter().enumerate().filter_map(|(i,v)| v.then_some(i)).collect();
        
        if valid_bools.len() != 1 {
            return Ok(CheckersResult::Ok(self.current_player)); //ask the AI to make the next move
        }

        // we know we need to execute an automatic move
        match valid_bools[0] {
            0 => return self.make_move(directions[0], Action::JumpSoutheast), //moving from northwest to southeast
            1 => return self.make_move(directions[1], Action::JumpSouthwest), //moving from northeast to southwest
            2 => return self.make_move(directions[2], Action::JumpNortheast), //moving from southwest to northeast
            3 => return self.make_move(directions[3], Action::JumpNorthwest), //moving from southeast to northwest
            _ => unreachable!()
        }
    }

    // Checks and executes outward jumps for the specified tile. I don't think we need the return here as we always call handle_inward_jump() after.
    fn handle_outward_jump(&mut self, landing_tile: u8) -> Result<CheckersResult, CheckersError>{
        let mut possible_moves = [false; 4];
        possible_moves[3] = self.is_move_valid(landing_tile, &Action::JumpNorthwest);
        possible_moves[0] = self.is_move_valid(landing_tile, &Action::JumpSoutheast);
        possible_moves[1] = self.is_move_valid(landing_tile, &Action::JumpSouthwest);
        possible_moves[2] = self.is_move_valid(landing_tile, &Action::JumpNortheast);

        let valid_bools: Vec<usize> = possible_moves.iter().enumerate().filter_map(|(i,v)| v.then_some(i)).collect();

        if valid_bools.len() == 0 {
            // turn is over, check automatic moves for enemy
            self.current_player = !self.current_player; //
            return self.handle_inward_jump(landing_tile); //handle inward jumping
        }
        
        if valid_bools.len() > 1 {
            return Ok(CheckersResult::Ok(self.current_player)); //ask the AI to make the next move
        }
        
        // we know we need to execute an automatic move
        match valid_bools[0] {
            2 => return self.make_move(landing_tile, Action::JumpNortheast), //moving from southwest to northeast
            3 => return self.make_move(landing_tile, Action::JumpNorthwest), //moving from southeast to northwest
            0 => return self.make_move(landing_tile, Action::JumpSoutheast), //moving from northwest to southeast
            1 => return self.make_move(landing_tile, Action::JumpSouthwest), //moving from northeast to southwest
            _ => unreachable!()
        }
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

    /// Get the current board as immutable
    #[inline]
    pub fn get_board(&self) -> &[i8; 32] {
        if self.current_player {&self.board_red} else {&self.board_black} //retrieve the board
    }

    /// Get the current board as mutable
    #[inline]
    pub fn get_board_mut(&mut self) -> &mut [i8; 32] {
        if self.current_player {&mut self.board_red} else {&mut self.board_black} //retrieve the board
    }

    /// Updates both boards at the same time
    pub fn update_board(&mut self, tile: u8, value: i8) {
        let mut board_main;
        let mut board_secondary;
        if self.current_player {
            board_main = self.board_red;
            board_secondary = self.board_black;
        } else {
            board_main = self.board_black;
            board_secondary = self.board_red;
        }
        board_main[tile as usize] = value;
        board_secondary[31 - tile as usize] = 1 - value;
    }
}

