// The supporting derive (and attribute-like) macros
// are placed in a crate named as the trait plus a trailing `_derive`.
// It's good practice to structure our derive macro packages like this,
// I decided to include also the attribute-like macro in the same
// crate since they should be used in pair.
pub use game_actor_derive;

// A `GameActor` is something that can self-declare its own
// type name, has some `health` and `mana` and lets us
// manipulate those values
pub trait GameActor {
    // Get the actor type name as a &str
    fn get_game_actor_type(&self) -> &str;
    // Get the current health
    fn get_health(&self) -> &u64;
    // Get the current mana
    fn get_mana(&self) -> &u64;
    // Modify the current `health` by the specified amount
    fn set_health(&mut self, amount: u64) -> ();
    // Modify the current `mana` by the specified amount
    fn set_mana(&mut self, amount: u64) -> ();
}
