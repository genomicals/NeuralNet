use crate::ai::AI;


/// Owns and operates on a generation of AIs
pub struct Generation {
    pub gen_num: u32,
    pub ais: [AI; 1000],
}
impl Generation {
    pub fn new() -> Self {
        Generation {
            gen_num: 0,
            ais: [(); 1000].map(|_| AI::new())
        }
    }
}

