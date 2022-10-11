use std::collections::HashMap;

use game_actor::GameActor;
use game_actor_derive::{add_game_actor_attributes, GameActor};

#[add_game_actor_attributes]
#[derive(GameActor, Debug)]
pub struct Adventurer {
    pub items: HashMap<String, String>,
}
