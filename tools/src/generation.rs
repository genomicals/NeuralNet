use crate::ai::AI;

/// Owns and operates on a generation of AIs
pub struct Generation1 {
    pub gen_num: u32,
    pub ais: Vec<AI>,
}
impl Generation1 {
    pub fn new() -> Self {
        Generation1 {
            gen_num: 0,
            ais: (0..1000).map(|_| AI::new()).collect(),
        }
    }
}
