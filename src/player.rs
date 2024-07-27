use crate::gameobj::GameObject;

pub struct Player {
    pub location_id: usize,
    pub inventory: Vec<GameObject>
}

impl Player {
    pub fn new(location_id: usize) -> Player {
        Player {
            location_id,
            inventory: Vec::new()
        }
    }
}