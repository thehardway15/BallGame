use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use systems::*;
use resources::*;
use crate::AppState;
use crate::game::SimulationState;

pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            // .add_startup_system(spawn_stars)
            // On Enter State
            .add_system(spawn_stars.in_schedule(OnEnter(AppState::Game)))
            // On Exit State
            .add_system(despawn_star.in_schedule(OnExit(AppState::Game)))
            // Systems
            // .add_system(tick_star_spawn_timer)
            // .add_system(spawn_stars_over_time);
            .add_systems((
                    tick_star_spawn_timer,
                    spawn_stars_over_time,
                )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running))
            );
    }
}