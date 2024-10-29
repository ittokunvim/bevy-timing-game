use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    WINDOW_SIZE,
    PATH_IMAGE_CHARACTER_IDLE,
    AppState,
};
use crate::ingame::bar::GRID_SIZE;

const CHARACTER_IMAGE_SIZE: u32 = 32;
const CHARACTER_SIZE: f32 = 64.0;

#[derive(Default, Component, Debug)]
struct Character {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ldtk_project_entities: Query<&Handle<LdtkProject>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Ldtk project
    if !ldtk_project_entities.is_empty() { return }

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(CHARACTER_IMAGE_SIZE), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = Character { first: 0, last: 3 };
    let pos = Vec3::new(
        WINDOW_SIZE.x / 2.0,
        WINDOW_SIZE.y / 2.0 + GRID_SIZE as f32 * 4.0,
        5.0
    );

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(CHARACTER_SIZE)),
                ..default()
            },
            texture: asset_server.load(PATH_IMAGE_CHARACTER_IDLE),
            transform: Transform::from_translation(pos),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
    ))
    .insert(Name::new("character"));
}

fn update(
    time: Res<Time>,
    mut query: Query<(&Character, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    let Ok((prop, mut timer, mut atlas)) = query.get_single_mut() else { return };

    timer.tick(time.delta());
    if timer.just_finished() {
        atlas.index = if atlas.index == prop.last { prop.first } else { atlas.index + 1 }
    }
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Ingame)));
    }
}
