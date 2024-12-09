use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

use crate::{
    WINDOW_SIZE,
    CURSOR_RANGE,
    PATH_IMAGE_PAUSEBUTTON,
    AppState,
    Config,
};

const IMAGE_SIZE: u32 = 64;
const IMAGE_COLUMN: u32 = 2;
const IMAGE_ROW: u32 = 1;
const SIZE: f32 = 32.0;
const PADDING: f32 = 5.0;

#[derive(Component)]
struct PauseButton {
    first: usize,
    last: usize,
}

fn setup(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
) {
    if !config.setup_ingame { return }

    println!("pausebutton: setup");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(IMAGE_SIZE),
        IMAGE_COLUMN,
        IMAGE_ROW,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = PauseButton { first: 0, last: 1 };
    let (x, y, z) = (
        WINDOW_SIZE.x / 2.0 - SIZE / 2.0 - PADDING, 
        -WINDOW_SIZE.y / 2.0 + SIZE / 2.0 + PADDING, 
        99.0
    );

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(SIZE)),
                ..Default::default()
            },
            texture: asset_server.load(PATH_IMAGE_PAUSEBUTTON),
            transform: Transform::from_xyz(x, y, z),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
    ))
    .insert(Name::new("pausebutton"));
}

fn update(
    mut query: Query<(&Transform, &PauseButton, &mut TextureAtlas), With<PauseButton>>,
    mut next_state: ResMut<NextState<AppState>>,
    mouse_events: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut config: ResMut<Config>,
) {
    if !mouse_events.just_pressed(MouseButton::Left) { return }

    let window = window_query.single();
    let mut cursor_pos = window.cursor_position().unwrap();
    let Ok((transform, prop, mut atlas)) = query.get_single_mut() else { return };
    let pausebutton_pos = transform.translation.truncate();
    cursor_pos = Vec2::new(
        cursor_pos.x - WINDOW_SIZE.x / 2.0,
        -cursor_pos.y + WINDOW_SIZE.y / 2.0
    );

    let distance = cursor_pos.distance(pausebutton_pos);

    if distance < SIZE - CURSOR_RANGE {
        if atlas.index == prop.first {
            println!("pausebutton: toggled");
            atlas.index = prop.last;
            println!("pausebutton: moved state to Pause from Ingame");
            next_state.set(AppState::Pause);
        } else {
            if config.setup_ingame {
                println!("pausebutton: change config.setup_ingame to false");
                config.setup_ingame = false;
            }
            println!("pausebutton: toggled");
            atlas.index = prop.first;
            println!("pausebutton: moved state to Ingame from Pause");
            next_state.set(AppState::Ingame);
        }
    }
}

fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<PauseButton>>,
) {
    println!("pausebutton: despawn");
    for entity in query.iter() { commands.entity(entity).despawn() }
}

pub struct PauseButtonPlugin;

impl Plugin for PauseButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Ingame)))
            .add_systems(Update, update.run_if(in_state(AppState::Pause)))
            .add_systems(OnEnter(AppState::Mainmenu), despawn)
        ;
    }
}
