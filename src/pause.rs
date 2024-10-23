use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

use crate::{
    WINDOW_SIZE,
    PATH_IMAGE_PAUSE,
    PATH_IMAGE_PLAY,
    AppState,
};

const PAUSEBTN_SIZE: Vec2 = Vec2::splat(32.0);
const PAUSEBTN_PADDING: f32 = 5.0;

#[derive(Component)]
pub struct PauseButton;

pub fn update_pausebtn(
    mouse_event: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    pausebtn_query: Query<(&Transform, Entity), With<PauseButton>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<AppState>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if !mouse_event.just_pressed(MouseButton::Left) { return; }

    let window = window_query.single();
    let mut cursor_pos = window.cursor_position().unwrap();
    let (pausebtn_transform, pausebtn) = pausebtn_query.single();
    let pausebtn_pos = pausebtn_transform.translation.truncate();
    cursor_pos = Vec2::new(cursor_pos.x, -cursor_pos.y + WINDOW_SIZE.y);

    let distance = cursor_pos.distance(pausebtn_pos);

    if distance < PAUSEBTN_SIZE.x - 10.0 {
        commands.entity(pausebtn).despawn();
        match state.get() {
            AppState::Ingame => {
                commands.spawn((
                    get_pausebtn(asset_server, PATH_IMAGE_PLAY.to_string()),
                    PauseButton,
                ))
                .insert(Name::new("pausebtn"));
                app_state.set(AppState::Pause);
            },
            AppState::Pause => {
                commands.spawn((
                    get_pausebtn(asset_server, PATH_IMAGE_PAUSE.to_string()),
                    PauseButton,
                ))
                .insert(Name::new("pausebtn"));
                app_state.set(AppState::Ingame);
            },
            _ => ()
        }
    }
}

pub fn get_pausebtn(
    asset_server: Res<AssetServer>,
    image_path: String,
) -> SpriteBundle {
    let pause_pos = Vec3::new(
        WINDOW_SIZE.x - PAUSEBTN_SIZE.x / 2.0 - PAUSEBTN_PADDING, 
        0.0 + PAUSEBTN_SIZE.y / 2.0 + PAUSEBTN_PADDING, 
        10.0
    );

    SpriteBundle {
        sprite: Sprite {
            custom_size: Some(PAUSEBTN_SIZE),
            ..default()
        },
        texture: asset_server.load(image_path),
        transform: Transform::from_translation(pause_pos),
        ..default()
    }
}

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_pausebtn.run_if(in_state(AppState::Pause)));
    }
}
