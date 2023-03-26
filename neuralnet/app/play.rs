mod engine;

//to run do "rustc play.rs" then "./play"

fn main() {

    let mut board = engine::Board::new();
    board.cells = [ [ 0,-1, 0,-1, 0,-1, 0,-1],
                    [-1, 0,-1, 0,-1, 0,-1, 0],
                    [ 0,-1, 0,-1, 0,-1, 0,-1],
                    [ 0, 0 ,0, 0, 0, 0, 1, 0],
                    [ 0,-1 ,0,-2, 0, 0, 0, 0],
                    [ 1 ,0 ,1 ,0 ,1 ,0 ,1 ,0],
                    [ 0 ,1 ,0 ,1 ,0 ,0 ,0,-1],
                    [ 1 ,0 ,1 ,0 ,1 ,0 ,0 ,0] ];

    //check regular diagonals
    // board.current_player = engine::Player::Red;
    // print!("{}", engine::Board::is_move_valid(&mut board, (5,6), (4,7))); //diag to empty
    // println!(" {}", true);
    // print!("{}", engine::Board::is_move_valid(&mut board, (4,0), (3,1)));// empty start
    // println!(" {}", false);
    // print!("{}", engine::Board::is_move_valid(&mut board, (5,0), (4,0)));// horizantal move
    // println!(" {}", false);
    // print!("{}", engine::Board::is_move_valid(&mut board, (5,0), (2,3)));// moves too far
    // println!(" {}", false);
    // print!("{}", engine::Board::is_move_valid(&mut board, (5,8), (2,8)));// out of bounds
    // println!(" {}", false);
    // print!("{}", engine::Board::is_move_valid(&mut board, (4,3), (3,4))); //red moving black king
    // println!(" {}", false);

    // //direction of moves
    // board.current_player = engine::Player::Black;
    // print!("{}", engine::Board::is_move_valid(&mut board, (2,5), (3,4))); //black diag forward
    // println!(" {}", true);
    // print!("{}", engine::Board::is_move_valid(&mut board, (4,1), (3,0))); //black diag back
    // println!(" {}", false);
    // print!("{}", engine::Board::is_move_valid(&mut board, (4,3), (3,4))); //king moving back
    // println!(" {}", true);
    // board.current_player = engine::Player::Red;
    // print!("{}", engine::Board::is_move_valid(&mut board, (5,6), (4,5))); //red diag forward
    // println!(" {}", true);
    // print!("{}", engine::Board::is_move_valid(&mut board, (3,6), (4,7))); //red diag back
    // println!(" {}", false);
    // print!("{}", engine::Board::is_move_valid(&mut board, (5,4), (4,3))); //lands on another piece
    // println!(" {}", false);

    // //test jumps
    // board.current_player = engine::Player::Red;
    // print!("{}", engine::Board::is_move_valid(&mut board, (5,0), (3,2))); //red jump
    // println!(" {}", true);
    // board.current_player = engine::Player::Black;
    // print!("{}", engine::Board::is_move_valid(&mut board, (4,3), (6,5))); //black king jump
    // println!(" {}", true);
    // print!("{}", engine::Board::is_move_valid(&mut board, (4,3), (2,5))); //jump into piece 
    // println!(" {}", false);
    // print!("{}", engine::Board::is_move_valid(&mut board, (1,6), (3,4))); //friendly fire
    // println!(" {}", false);

    board.current_player = engine::Player::Black;
    // print!("{}", engine::Board::make_move(&mut board, (4,3), (6,5))); //black king jump
    // println!(" {}", true);
    // println!(" {}", board.black_kills);

    println!("{:?}", board.current_player);
    print!("{}", engine::Board::make_move(&mut board, (6,7), (7,6))); //black promotion
    println!(" {}", true);
    println!("{:?}", board.current_player);

    engine::Board::print_board(&board);
}