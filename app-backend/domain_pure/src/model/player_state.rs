use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct PlayerState {
    player_id: i32, //aggregate id and foreign kez (once I have established a user)
    current_location_id: i32,
}

impl PlayerState {

    pub fn current_location_id(&self) -> i32 {
        self.current_location_id
    }
    pub fn set_current_location_id(&mut self, location_id: i32) {
        self.current_location_id = location_id;
    }
    pub fn player_id(&self) -> i32 {self.player_id}
}
