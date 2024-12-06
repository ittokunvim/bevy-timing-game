use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    AppState,
};

pub const GRID_SIZE: f32 = 16.0;
pub const SIZE: Vec2 = Vec2::new(GRID_SIZE * 32.0, GRID_SIZE * 2.0);
const COLOR: Color = Color::srgb(0.25, 0.25, 0.25);

#[derive(Component)]
pub struct Bar;

fn setup(
    mut commands: Commands,
) {
    println!("bar: setup");
    let bar_y = WINDOW_SIZE.y - GRID_SIZE * 4.0 - SIZE.y / 2.0;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: COLOR,
                custom_size: Some(SIZE),
                ..Default::default()
            },
            transform: Transform::from_xyz(WINDOW_SIZE.x / 2.0, bar_y, 0.0),
            ..Default::default()
        },
        Bar,
    ))
    .insert(Name::new("bar"));
}

pub struct BarPlugin;

impl Plugin for BarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
        ;
    }
}
