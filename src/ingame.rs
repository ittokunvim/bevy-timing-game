use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    LDTK_PROJECT_PATH,
};
#[derive(Default, Component)]
pub struct Cue;

#[derive(Default, Bundle, LdtkEntity)]
pub struct CueBundle {
    cue: Cue,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
}

pub fn ingame_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    ldtk_project_entities: Query<&Handle<LdtkProject>>,
) {
    // Ldtk project
    if ldtk_project_entities.is_empty() {
        commands.spawn(LdtkWorldBundle {
            ldtk_handle: asset_server.load(LDTK_PROJECT_PATH),
            ..Default::default()
        });
    }
    // Camera
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = WINDOW_SIZE.x / 2.0;
    camera_transform.translation.y = WINDOW_SIZE.y / 2.0;
}
