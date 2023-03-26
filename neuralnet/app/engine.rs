//black --> negatives
//red --> positives
//|1| --> man  |2| --> king  0 --> empty

// for the move function, return a true/false to signify if the move was valid or not
// - ensure only valid moves are executed on the board
// - handle automatic moves (if the user can take, then take, but if the user could take 2+ pieces then let the user make a move manually)
// - track the number of kills and do win detection
// we probably want more requirements but thats what i thought up real quick
// ah and i recommend storing the board as a 32-large vector instead of a 64-large because half of those spaces would be unreachable
// you may decide the interface as you wish and we'll work with it

//use rand::Rng;

#[derive(PartialEq, Eq)]
#[derive(Debug)]
pub enum Player {
    Red,
    Black,
}

#[derive(PartialEq, Eq)]
pub enum Winner {
    Red,
    Black,
    Draw
}

pub struct Board {
    pub cells: [[i32; 8]; 8],
    pub current_player: Player,
    pub is_game_over: bool,
    pub winner: Winner,
    pub red_kills: i32,
    pub black_kills: i32,
    pub invalid_message: String
}

impl Board {
    pub fn new() -> Self {

        // initialize the board with the starting position wwith black on top and red on bottom
        let mut cells = [ [ 0,-1, 0,-1, 0,-1, 0,-1],
                          [-1, 0,-1, 0,-1, 0,-1, 0],
                          [ 0,-1, 0,-1, 0,-1, 0,-1],
                          [ 0, 0 ,0, 0, 0, 0, 0, 0],
                          [ 0, 0 ,0, 0, 0, 0, 0, 0],
                          [ 1 ,0 ,1 ,0 ,1 ,0 ,1 ,0],
                          [ 0 ,1 ,0 ,1 ,0 ,1 ,0 ,1],
                          [ 1 ,0 ,1 ,0 ,1 ,0 ,1 ,0] ];

        //randomly choose starting player
        // let mut rng = rand::thread_rng();
        // let random_number = rng.gen_range(1..=2);
        // let mut startPlayer = Player::Red;
        // if(random_number == 2) {
        //     startPlayer = Player::Black;
        // }

        return Board {
            cells: cells,
            current_player: Player::Red,
            is_game_over: false,
            winner: Winner::Draw,
            red_kills: 0,
            black_kills: 0,
            invalid_message: String::from("")
        } 
    }

    pub fn is_move_valid(board: &mut Board, start: (usize, usize), end: (usize, usize)) -> bool {
        // check if the move from start to end is valid
        if start.0 >= 8 || start.1 >= 8 || end.0 >= 8 || end.1 >= 8 {
            print!("out of bounds ");
            board.invalid_message = String::from("Out of bounds");
            return false;
        }

        let startCell = board.cells[start.0][start.1] as i32;
        let endCell = board.cells[end.0][end.1] as i32;
        
        //check that the move is using the correct color peice and that there is a peice there
        if board.current_player == Player::Red {
            if startCell <= 0 {
                print!("red cant use black piece ");
                board.invalid_message = String::from("red cant use black piece");
                return false;
            }
        } else if board.current_player == Player::Black {
            if startCell >= 0 {
                print!("black cant use red piece ");
                board.invalid_message = String::from("black cant use red piece");
                return false;
            }
        }

        //check if the move is diagonal
        let di = start.0 as i32 - end.0 as i32;
        let dj = start.1 as i32 - end.1 as i32;
        if di.abs() != dj.abs() {
            print!("not a diagonal move ");
            board.invalid_message = String::from("not a diagonal move");
            return false;
        }

        //check the directions of the moves
        if startCell.abs() == 1 {
            if startCell == -1 && di > 0 {
                print!("move isnt in black direction ");
                board.invalid_message = String::from("move isnt in black direction");
                return false;
            } else if startCell == 1 && di < 0 {
                print!("move isnt in red direction ");
                board.invalid_message = String::from("move isnt in red direction");
                return false;
            }
        }

        //check that landing cell is empty
        if endCell != 0 {
            print!("cant land on another piece ");
            board.invalid_message = String::from("cant land on another piece");
            return false;
        }

        //if a jump, make sure jump is valid
        if di.abs() > 2 || dj.abs() > 2 {
            print!("too far of a jump ");
            board.invalid_message = String::from("too far of a jump");
            return false;
        } 
        //make sure jump is over the correct piece
        if di.abs() == 2 && dj.abs() == 2 {
            return Board::is_valid_jump(board, start, end);
        }

        return true;
    }
    
    //checks if the single jump is valid
    pub fn is_valid_jump(board: &mut Board, start: (usize, usize), end: (usize, usize)) -> bool {
        let mid_piece_coords = ((start.0 + end.0) / 2, (start.1 + end.1) / 2);
        let mid_piece = board.cells[mid_piece_coords.0][mid_piece_coords.1];

        if mid_piece == 0 {
            print!("Cannot jump over nothing ");
            board.invalid_message = String::from("Cannot jump over nothing");
            return false;
        } else if board.current_player == Player::Red && mid_piece > 0 {
            print!("Cannot jump friendly piece ");
            board.invalid_message = String::from("Cannot jump friendly piece");
            return false;
        } else if board.current_player == Player::Black && mid_piece < 0 {
            print!("Cannot jump friendly piece ");
            board.invalid_message = String::from("Cannot jump friendly piece");
            return false;
        } 
        return true;
    }

    //performs the specified move or defines the error message if the move is invalid
    pub fn make_move(board: &mut Board, start: (usize, usize), end: (usize, usize)) -> bool {
        // make the move from start to end
        if Board::is_move_valid(board, start, end) {
            let piece = board.cells[start.0][start.1];

            board.cells[start.0][start.1] = 0;
            board.cells[end.0][end.1] = piece;

            let di = start.0 as i32 - end.0 as i32;
            let dj = start.1 as i32 - end.1 as i32;

            //if single jump, then remove middle piece and add to score
            if di.abs() == 2 && dj.abs() == 2 {

                let mid_piece_coords = ((start.0 + end.0) / 2, (start.1 + end.1) / 2);
                let mid_piece = board.cells[mid_piece_coords.0][mid_piece_coords.1];

                if mid_piece < 0 {
                    board.red_kills = board.red_kills + 1;
                } else {
                    board.black_kills = board.black_kills + 1;
                }
                board.cells[mid_piece_coords.0][mid_piece_coords.1] = 0;
            }

            //turn piece into king if it reaches the end
            if piece == -1 && end.0 == 7 {
                board.cells[end.0][end.1] = -2;
            } else if piece == 1 && end.0 == 0 {
                board.cells[end.0][end.1] = 2;
            }

            // //change to the opposing player
            // if board.current_player == Player::Red {
            //     board.current_player = Player::Black;
            // } else {
            //     board.current_player = Player::Red;
            // }

            return true;

        } else {
            println!("{}", board.invalid_message);
            return false;
        }
    }

    //checks if the game is over
    pub fn is_game_over(board: &mut Board) -> bool {
        // check if the game is over
        if board.black_kills == 12 {
            board.winner = Winner::Black;
            return true;
        } else if board.red_kills == 12 {
            board.winner = Winner::Red;
            return true;
        }
        return false;
    }

    //returns the winner of the game
    pub fn get_winner(board: &Board) -> &Winner {
        // return the winner if there is one
        let winner = &board.winner;
        return winner;
    }

    //prints the current state of the board
    pub fn print_board(board: &Board) {
        for i in 0..8 {
            for j in 0..8 {
                print!("{} ",  board.cells[i][j]);
            }
            println!();
        }
    }
}