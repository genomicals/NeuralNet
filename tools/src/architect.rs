use rand::{rngs::ThreadRng, Rng, seq::IteratorRandom};

use crate::{engine::{self, Engine, Action}, ai::{self, AI}};
use std::{sync::{Arc, Mutex}, thread, ops::Deref};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Architect {
    pub generation: Arc<Mutex<Vec<Arc<Mutex<AI>>>>>,
    pub fitness: Arc<Mutex<Vec<i32>>>,
    pub bracket: Arc<Mutex<Vec<usize>>>,
    //pub rng: Arc<Mutex<ThreadRng>>, //using the same engine for rng should increase performance slightly
}
impl Architect {
    pub fn new() -> Self {
        Architect {
            //generation: Arc::new(Mutex::new(Generation {
            //    gen_num: 0,
            //    ais: (0..1000).map(|_| AI::new()).collect(),
            //})),
            generation: Arc::new(Mutex::new(ai::gen_thousand())),
            fitness: Arc::new(Mutex::new(vec![0; 1000])),
            bracket: Arc::new(Mutex::new(vec![0; 1000])),
            //rng: rand::thread_rng(),
        }
    }

    /// Creates the tournament bracket, for now this is simple assignment.
    pub fn evolve_generation(&mut self) {
        let mut rng = thread_rng();
        self.bracket.lock().unwrap().shuffle(&mut rng); //shuffle the bracket
        self.run_games(); //generate fitnesses
        let min_val = *self.fitness.lock().unwrap().iter().min().unwrap(); //get the absolute minimum in the thing (for shifting all the values)
        let normalize_constant = *self.fitness.lock().unwrap().iter().max().unwrap() + min_val; //find what we need to normalize by
        let mut probabilities: Vec<f32> = self.fitness
            .lock()
            .unwrap()
            .iter()
            .map(|value| (value + min_val) as f32 / normalize_constant as f32)
            .collect(); //shift and normalize
        
        // kill 500 using a very smart algorithm
        let mut count = 0;
        while count < 500 {
            let p = probabilities.iter().enumerate().choose(&mut rng).unwrap();
            let realized_prob = rng.gen_range(0.0..1.05); //no ai is safe
            if *p.1 < realized_prob {
                count += 1;
                self.fitness.lock().unwrap().swap_remove(p.0);
                self.generation.lock().unwrap().swap_remove(p.0);
                probabilities.swap_remove(p.0);
            }
        }

        for _ in 0..500 {
            let mut gen = self.generation.lock().unwrap();

            let a0_index = rng.gen_range(0..500);
            let mut a1_index = rng.gen_range(0..500);
            while a0_index == a1_index { //ensure the two ais are unique, not just for genetic diversity but also to avoid deadlocks with the mutexes
                a1_index = rng.gen_range(0..500);
            }
            let a0 = gen[a0_index as usize].clone();
            let a1 = gen[a1_index as usize].clone();
            let new_genome = ai::reproduce(a0.lock().unwrap().deref(), a1.lock().unwrap().deref(), &mut rng).unwrap(); //we know our ais are the same size
            gen.push(Arc::new(Mutex::new(AI::with_genome(new_genome))));
            self.fitness.lock().unwrap().push(0);
            drop(a0);
            drop(a1);
            drop(gen);
        }
    }

    // Runs all the games in bracket order.
    pub fn run_games(&mut self) {
        let mut threads = Vec::with_capacity(25); //holds all threads
        for i in 0..25 { //25 threads, each handling 40 ais
            let n = i;
            let bracket_mtx = self.bracket.clone(); //each thread has access to the bracket
            let fitness_mtx = self.fitness.clone(); //access to the fitnesses
            let generation_mtx = self.generation.clone(); //access to the ais
            
            // push a new thread onto the list
            let handle = thread::spawn(move || {
                let mut rng = thread_rng();
                for j in 0..10 { //10 groups, each with 4 ais (round robin style tournament)
                    let k = n*40 + j*4; //convenient index calculation
                    let p0 = generation_mtx.lock().unwrap()[bracket_mtx.lock().unwrap()[k]].clone();
                    let p1 = generation_mtx.lock().unwrap()[bracket_mtx.lock().unwrap()[k+1]].clone();
                    let p2 = generation_mtx.lock().unwrap()[bracket_mtx.lock().unwrap()[k+2]].clone();
                    let p3 = generation_mtx.lock().unwrap()[bracket_mtx.lock().unwrap()[k+3]].clone();

                    // run all combinations of games for our four ais
                    let result = Architect::run_game(p0.clone(), p1.clone(), &mut rng);
                    fitness_mtx.lock().unwrap()[bracket_mtx.lock().unwrap()[k]] += result.0;
                    fitness_mtx.lock().unwrap()[bracket_mtx.lock().unwrap()[k+1]] += result.1;

                    let result = Architect::run_game(p0.clone(), p2.clone(), &mut rng);
                    fitness_mtx.lock().unwrap()[bracket_mtx.lock().unwrap()[k]] += result.0;
                    fitness_mtx.lock().unwrap()[bracket_mtx.lock().unwrap()[k+2]] += result.1;

                    let result = Architect::run_game(p0.clone(), p3.clone(), &mut rng);
                    fitness_mtx.lock().unwrap()[bracket_mtx.lock().unwrap()[k]] += result.0;
                    fitness_mtx.lock().unwrap()[bracket_mtx.lock().unwrap()[k+3]] += result.1;

                    let result = Architect::run_game(p1.clone(), p2.clone(), &mut rng);
                    fitness_mtx.lock().unwrap()[bracket_mtx.lock().unwrap()[k+1]] += result.0;
                    fitness_mtx.lock().unwrap()[bracket_mtx.lock().unwrap()[k+2]] += result.1;

                    let result = Architect::run_game(p1.clone(), p3.clone(), &mut rng);
                    fitness_mtx.lock().unwrap()[bracket_mtx.lock().unwrap()[k+1]] += result.0;
                    fitness_mtx.lock().unwrap()[bracket_mtx.lock().unwrap()[k+3]] += result.1;

                    let result = Architect::run_game(p2.clone(), p3.clone(), &mut rng);
                    fitness_mtx.lock().unwrap()[bracket_mtx.lock().unwrap()[k+2]] += result.0;
                    fitness_mtx.lock().unwrap()[bracket_mtx.lock().unwrap()[k+3]] += result.1;
                }
            });
            threads.push(handle);
        }

        // join each thread back into the main thread
        for t in threads {
            t.join().expect("joining thread");
        }
    }

    /// Runs a single game between to AI players and returns their fitness scores.
    pub fn run_game(player1: Arc<Mutex<AI>>, player2: Arc<Mutex<AI>>, rng: &mut ThreadRng) -> (i32, i32) {
        let player_decider: bool = rng.gen(); //decide if player1 is red or black
        let mut game = Engine::new(); //by default black will start
        let p1;
        let p2;
        if player_decider {
            p1 = player1;
            p2 = player2;
        } else {
            p1 = player2;
            p2 = player1;
        }

        let mut scores = [0, 0]; //red, black
        let mut stale_count = 0;
        let mut stale_history = [0, 0]; //red, black

        // main game loop
        let mut cur_turn = false;
        'turn_loop: loop {

            // retrieve output of neural network
            let moves = if cur_turn {
                p1.lock().unwrap().calculate(game.peek_red())
            } else {
                p2.lock().unwrap().calculate(game.peek_black())
            };

            let mut current_penalty = 0;

            // find the first valid move
            for cur in moves {
                let (tile, action) = Architect::index_to_move(cur);
                let result = game.make_move(tile, action);
                //println!("result : {:?}", result);
                if let Ok(game_result) = result { //if the move went smoothly
                    // want to check if any pieces have been captured
                    if stale_history[0] == game.red_pieces && stale_history[1] == game.black_pieces { //no pieces have been captured
                        stale_count += 1;
                    } else {
                        stale_history[0] = game.red_pieces;
                        stale_history[1] = game.black_pieces;
                    }

                    if stale_count >= 30 { //end the game
                        if game.red_pieces >= game.black_pieces { //black goes first but red gets the stalemate advantage
                            cur_turn = true;
                        } else {
                            cur_turn = false;
                        }
                        break 'turn_loop;
                    }


                    if cur_turn {scores[0] += current_penalty;} else {scores[1] += current_penalty;} //update penalties
                    match game_result {
                        engine::CheckersResult::Ok(turn) => {
                            cur_turn = turn;
                            continue 'turn_loop;
                        },
                        engine::CheckersResult::Win(winner) => {
                            cur_turn = winner;
                            break 'turn_loop;
                        }
                    }
                }

                // move did not go smoothly
                current_penalty -= 1;
            }
            // none of the moves worked, so other player must have won (either by getting trapped or literally having no pieces)
            cur_turn = !cur_turn;
            break 'turn_loop;
        }

        if cur_turn { //red won
            scores[0] += 10;
            scores[1] -= 10;
        } else { //black won
            scores[0] -= 10;
            scores[1] += 10;
        }

        if player_decider {(scores[0], scores[1])} else {(scores[1], scores[0])}
    }

    /// Converts a number to an action
    pub fn num_to_action(num: u8) -> Action {
        match num {
            0 => Action::MoveNorthwest,
            1 => Action::MoveNortheast,
            2 => Action::MoveSouthwest,
            3 => Action::MoveSoutheast,
            4 => Action::JumpNorthwest,
            5 => Action::JumpNortheast,
            6 => Action::JumpSouthwest,
            7 => Action::JumpSoutheast,
            _ => unreachable!()
        }
    }

    /// Converts an index (0 through 169) into a tile and action
    #[inline]
    pub fn index_to_move(index: u8) -> (u8, Action) {
        match index {
            0 => (0, Action::MoveSoutheast),
            1 => (0, Action::JumpSoutheast),

            2 => (1, Action::MoveNorthwest),
            3 => (1, Action::MoveNortheast),
            4 => (1, Action::MoveSouthwest),
            5 => (1, Action::MoveSoutheast),
            6 => (1, Action::JumpSoutheast),

            7 => (2, Action::MoveSouthwest),
            8 => (2, Action::MoveSoutheast),
            9 => (2, Action::JumpSouthwest),
            10 => (2, Action::JumpSoutheast),

            11 => (3, Action::MoveNorthwest),
            12 => (3, Action::MoveNortheast),
            13 => (3, Action::MoveSouthwest),
            14 => (3, Action::MoveSoutheast),
            15 => (3, Action::JumpSouthwest),
            16 => (3, Action::JumpSoutheast),
            
            17 => (4, Action::MoveSouthwest),
            18 => (4, Action::MoveSoutheast),
            19 => (4, Action::JumpSouthwest),
            20 => (4, Action::JumpSoutheast),

            21 => (5, Action::MoveNorthwest),
            22 => (5, Action::MoveNortheast),
            23 => (5, Action::MoveSouthwest),
            24 => (5, Action::MoveSoutheast),
            25 => (5, Action::JumpSouthwest),
            26 => (5, Action::JumpSoutheast),

            27 => (6, Action::MoveSouthwest),
            28 => (6, Action::MoveSoutheast),
            29 => (6, Action::JumpSouthwest),

            30 => (7, Action::MoveNorthwest),
            31 => (7, Action::MoveSouthwest),
            32 => (7, Action::JumpSouthwest),

            33 => (8, Action::MoveNortheast),
            34 => (8, Action::MoveSoutheast),
            35 => (8, Action::JumpNortheast),
            36 => (8, Action::JumpSoutheast),

            37 => (9, Action::MoveNorthwest),
            38 => (9, Action::MoveNortheast),
            39 => (9, Action::MoveSouthwest),
            40 => (9, Action::MoveSoutheast),
            41 => (9, Action::JumpNortheast),
            42 => (9, Action::JumpSoutheast),

            43 => (10, Action::MoveNorthwest),
            44 => (10, Action::MoveNortheast),
            45 => (10, Action::MoveSouthwest),
            46 => (10, Action::MoveSoutheast),
            47 => (10, Action::JumpNorthwest),
            48 => (10, Action::JumpNortheast),
            49 => (10, Action::JumpSouthwest),
            50 => (10, Action::JumpSoutheast),

            51 => (11, Action::MoveNorthwest),
            52 => (11, Action::MoveNortheast),
            53 => (11, Action::MoveSouthwest),
            54 => (11, Action::MoveSoutheast),
            55 => (11, Action::JumpNorthwest),
            56 => (11, Action::JumpNortheast),
            57 => (11, Action::JumpSouthwest),
            58 => (11, Action::JumpSoutheast),

            59 => (12, Action::MoveNorthwest),
            60 => (12, Action::MoveNortheast),
            61 => (12, Action::MoveSouthwest),
            62 => (12, Action::MoveSoutheast),
            63 => (12, Action::JumpNorthwest),
            64 => (12, Action::JumpNortheast),
            65 => (12, Action::JumpSouthwest),
            66 => (12, Action::JumpSoutheast),

            67 => (13, Action::MoveNorthwest),
            68 => (13, Action::MoveNortheast),
            69 => (13, Action::MoveSouthwest),
            70 => (13, Action::MoveSoutheast),
            71 => (13, Action::JumpNorthwest),
            72 => (13, Action::JumpNortheast),
            73 => (13, Action::JumpSouthwest),
            74 => (13, Action::JumpSoutheast),

            75 => (14, Action::MoveNorthwest),
            76 => (14, Action::MoveNortheast),
            77 => (14, Action::MoveSouthwest),
            78 => (14, Action::MoveSoutheast),
            79 => (14, Action::JumpNorthwest),
            80 => (14, Action::JumpSouthwest),

            81 => (15, Action::MoveNorthwest),
            82 => (15, Action::MoveSouthwest),
            83 => (15, Action::JumpNorthwest),
            84 => (15, Action::JumpSouthwest),

            85 => (16, Action::MoveNortheast),
            86 => (16, Action::MoveSoutheast),
            87 => (16, Action::JumpNortheast),
            88 => (16, Action::JumpSoutheast),
            
            89 => (17, Action::MoveNorthwest),
            90 => (17, Action::MoveNortheast),
            91 => (17, Action::MoveSouthwest),
            92 => (17, Action::MoveSoutheast),
            93 => (17, Action::JumpNortheast),
            94 => (17, Action::JumpSoutheast),
            
            95 => (18, Action::MoveNorthwest),
            96 => (18, Action::MoveNortheast),
            97 => (18, Action::MoveSouthwest),
            98 => (18, Action::MoveSoutheast),
            99 => (18, Action::JumpNorthwest),
            100 => (18, Action::JumpNortheast),
            101 => (18, Action::JumpSouthwest),
            102 => (18, Action::JumpSoutheast),
            
            103 => (19, Action::MoveNorthwest),
            104 => (19, Action::MoveNortheast),
            105 => (19, Action::MoveSouthwest),
            106 => (19, Action::MoveSoutheast),
            107 => (19, Action::JumpNorthwest),
            108 => (19, Action::JumpNortheast),
            109 => (19, Action::JumpSouthwest),
            110 => (19, Action::JumpSoutheast),
            
            111 => (20, Action::MoveNorthwest),
            112 => (20, Action::MoveNortheast),
            113 => (20, Action::MoveSouthwest),
            114 => (20, Action::MoveSoutheast),
            115 => (20, Action::JumpNorthwest),
            116 => (20, Action::JumpNortheast),
            117 => (20, Action::JumpSouthwest),
            118 => (20, Action::JumpSoutheast),
            
            119 => (21, Action::MoveNorthwest),
            120 => (21, Action::MoveNortheast),
            121 => (21, Action::MoveSouthwest),
            122 => (21, Action::MoveSoutheast),
            123 => (21, Action::JumpNorthwest),
            124 => (21, Action::JumpNortheast),
            125 => (21, Action::JumpSouthwest),
            126 => (21, Action::JumpSoutheast),
            
            127 => (22, Action::MoveNorthwest),
            128 => (22, Action::MoveNortheast),
            129 => (22, Action::MoveSouthwest),
            130 => (22, Action::MoveSoutheast),
            131 => (22, Action::JumpNorthwest),
            132 => (22, Action::JumpSouthwest),
            
            133 => (23, Action::MoveNorthwest),
            134 => (23, Action::MoveSouthwest),
            135 => (23, Action::JumpNorthwest),
            136 => (23, Action::JumpSouthwest),
            
            137 => (24, Action::MoveNortheast),
            138 => (24, Action::MoveSoutheast),
            139 => (24, Action::JumpNortheast),
            
            140 => (25, Action::MoveNorthwest),
            141 => (25, Action::MoveNortheast),
            142 => (25, Action::JumpNortheast),
            
            143 => (26, Action::MoveNorthwest),
            144 => (26, Action::MoveNortheast),
            145 => (26, Action::MoveSouthwest),
            146 => (26, Action::MoveSoutheast),
            147 => (26, Action::JumpNorthwest),
            148 => (26, Action::JumpNortheast),
            
            149 => (27, Action::MoveNorthwest),
            150 => (27, Action::MoveNortheast),
            151 => (27, Action::JumpNorthwest),
            152 => (27, Action::JumpNortheast),
            
            153 => (28, Action::MoveNorthwest),
            154 => (28, Action::MoveNortheast),
            155 => (28, Action::MoveSouthwest),
            156 => (28, Action::MoveSoutheast),
            157 => (28, Action::JumpNorthwest),
            158 => (28, Action::JumpNortheast),
            
            159 => (29, Action::MoveNorthwest),
            160 => (29, Action::MoveNortheast),
            161 => (29, Action::JumpNorthwest),
            162 => (29, Action::JumpNortheast),
            
            163 => (30, Action::MoveNorthwest),
            164 => (30, Action::MoveNortheast),
            165 => (30, Action::MoveSouthwest),
            166 => (30, Action::MoveSoutheast),
            167 => (30, Action::JumpNorthwest),
            
            168 => (31, Action::MoveNorthwest),
            169 => (31, Action::JumpNorthwest),

            _ => unreachable!(),
        }
    }
}
