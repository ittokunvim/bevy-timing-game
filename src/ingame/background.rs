use bevy::prelude::*;

use crate::{
    PATH_IMAGE_INGAME,
    AppState,
    Config,
};

#[derive(Component)]
struct Background;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
) {
    if !config.setup_ingame { return }

    println!("background: setup");
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(PATH_IMAGE_INGAME),
            transform: Transform::from_xyz(0.0, 0.0, -99.0),
            ..Default::default()
        },
        Background,
    ));
}

fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<Background>>,
) {
    println!("background: despawn");
    for entity in query.iter() { commands.entity(entity).despawn() }
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(OnEnter(AppState::Mainmenu), despawn)
        ;
    }
}
