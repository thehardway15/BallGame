use bevy::prelude::*;

pub const STAR_SPAWN_TIME: f32 = 1.0;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>
}

impl Default for HighScores {
    fn default() -> Self {
        HighScores {
            scores: Vec::new()
        }
    }
}