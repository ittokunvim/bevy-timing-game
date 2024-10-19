use bevy::prelude::*;

use crate::AppState;

fn setup() {}

fn update() {}

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Pause), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Pause)));
    }
}
