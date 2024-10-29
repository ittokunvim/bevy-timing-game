use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    PATH_LDTK_PROJECT,
    AppState,
};

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ldtk_project_entities: Query<&Handle<LdtkProject>>,
) {
    // Ldtk project
    if !ldtk_project_entities.is_empty() { return }

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load(PATH_LDTK_PROJECT),
        ..Default::default()
    })
    .insert(Name::new("ldtk"));
}

pub struct LdtkPlugin;

impl Plugin for LdtkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup);
    }
}
