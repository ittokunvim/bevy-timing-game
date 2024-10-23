use bevy::prelude::*;

use crate::{
    GAMETITLE,
    WINDOW_SIZE,
    PATH_BG_IMAGE,
    PATH_FONT_MEDIUM,
    PATH_FONT_BOLD,
    AppState,
};

const GAMETITLE_FONT_SIZE: f32 = 40.0;
const GAMETITLE_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const CLICKSTART_TEXT: &str = "Click Start...";
const CLICKSTART_FONT_SIZE: f32 = 30.0;
const CLICKSTART_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

#[derive(Component)]
struct Mainmenu;

fn mainmenu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Game Title
    commands.spawn((
        TextBundle::from_section(
            GAMETITLE,
            TextStyle {
                font: asset_server.load(PATH_FONT_BOLD),
                font_size: GAMETITLE_FONT_SIZE,
                color: GAMETITLE_COLOR,
            }
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top: Val::Px(WINDOW_SIZE.y / 2.0 - GAMETITLE_FONT_SIZE / 2.0),
            ..default()
        }),
        Mainmenu,
    ))
    .insert(Name::new("game_title"));
    // Click Start
    commands.spawn((
        TextBundle::from_section(
            CLICKSTART_TEXT,
            TextStyle {
                font: asset_server.load(PATH_FONT_MEDIUM),
                font_size: CLICKSTART_FONT_SIZE,
                color: CLICKSTART_COLOR,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            right: Val::Px(16.0),
            bottom: Val::Px(16.0),
            ..default()
        }),
        Mainmenu,
    ))
    .insert(Name::new("click_start"));
    // Background image
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(PATH_BG_IMAGE),
            ..default()
        },
        Mainmenu,
    ))
    .insert(Name::new("bg_image"));
}

fn mainmenu_update(
    mouse_event: Res<ButtonInput<MouseButton>>,
    mainmenu_query: Query<Entity, With<Mainmenu>>,
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if mouse_event.just_pressed(MouseButton::Left) {
        // despawned mainmenu
        for mainmenu_entity in mainmenu_query.iter() {
            commands.entity(mainmenu_entity).despawn();
        }
        // changed app state
        app_state.set(AppState::Ingame);
    }
}

pub struct MainmenuPlugin;

impl Plugin for MainmenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Mainmenu), mainmenu_setup)
            .add_systems(Update, mainmenu_update.run_if(in_state(AppState::Mainmenu)));
    }
}
