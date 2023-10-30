pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
mod systems;

use bevy::app::App;
use bevy::prelude::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use crate::events::GameOver;
use systems::*;
use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<SimulationState>()
            // Events
            .add_event::<GameOver>()
            // On Enter System
            .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            // On Exit System
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)))
            // Plugins
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(StarPlugin)
            .add_plugin(ScorePlugin)
            // Systems
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)));
    }
}


#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}