use bevy::prelude::*;

use crate::{
    PATH_IMAGE_BAR,
    AppState,
    Config,
};
use crate::ingame::GRID_SIZE;

pub const SIZE: Vec2 = Vec2::new(512.0, 32.0);

#[derive(Component)]
pub struct Bar;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
) {
    if !config.setup_ingame { return }

    println!("bar: setup");
    let y = GRID_SIZE * 10.0;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(SIZE),
                ..Default::default()
            },
            texture: asset_server.load(PATH_IMAGE_BAR),
            transform: Transform::from_xyz(0.0, y, 0.0),
            ..Default::default()
        },
        Bar,
    ))
    .insert(Name::new("bar"));
}

fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<Bar>>,
) {
    println!("bar: despawn");
    for entity in query.iter() { commands.entity(entity).despawn() }
}

pub struct BarPlugin;

impl Plugin for BarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(OnEnter(AppState::Gameover), despawn)
        ;
    }
}
