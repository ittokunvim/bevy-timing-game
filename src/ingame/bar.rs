use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    WINDOW_SIZE,
    AppState,
};

pub const GRID_SIZE: i32 = 16;
pub const BAR_SIZE: Vec2 = Vec2::new((GRID_SIZE * 32) as f32, (GRID_SIZE * 2) as f32);
const BAR_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);

#[derive(Default, Component)]
pub struct Bar;

fn setup(
    mut commands: Commands,
    ldtk_project_entities: Query<&Handle<LdtkProject>>,
) {
    // Ldtk project
    if !ldtk_project_entities.is_empty() { return }

    let bar_y = WINDOW_SIZE.y - (GRID_SIZE * 4) as f32 - BAR_SIZE.y / 2.0;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: BAR_COLOR,
                custom_size: Some(BAR_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(WINDOW_SIZE.x / 2.0, bar_y, 0.0),
            ..default()
        },
        Bar,
    ))
    .insert(Name::new("bar"));

}

pub struct BarPlugin;

impl Plugin for BarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup);
    }
}
