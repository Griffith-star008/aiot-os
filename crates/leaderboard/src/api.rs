/// API for leaderboard
use aiot_core::api::AiotError;

pub trait LeaderboardInterface {
    fn execute(&self) -> Result<(), AiotError>;
}
