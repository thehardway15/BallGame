use bevy::prelude::*;

pub mod resources;
mod systems;

use systems::*;
use resources::*;
use crate::AppState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter state
            .add_system(insert_score.in_schedule(OnEnter(AppState::Game)))
            // Exit state
            .add_system(remove_score.in_schedule(OnExit(AppState::Game)))
            .init_resource::<HighScores>()
            .add_system(update_score.run_if(in_state(AppState::Game)))
            .add_system(update_high_scores)
            .add_system(high_score_updated);
    }
}