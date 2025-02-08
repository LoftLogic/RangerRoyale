use crate::model::player::Player;


///
/// Represents the player selection screen.
/// Stores players (up to three) in state.
/// Has the ability to select a player.
///
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PlayerSelector {
    players: Vec<Player>
}

impl PlayerSelector {

}