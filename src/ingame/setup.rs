use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    WINDOW_SIZE,
    PATH_LDTK_PROJECT,
};

pub fn component(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    ldtk_project_entities: Query<&Handle<LdtkProject>>,
) {
    // Ldtk project
    if !ldtk_project_entities.is_empty() { return }

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load(PATH_LDTK_PROJECT),
        ..Default::default()
    })
    .insert(Name::new("ldtk"));
    // Camera
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = WINDOW_SIZE.x / 2.0;
    camera_transform.translation.y = WINDOW_SIZE.y / 2.0;
}
