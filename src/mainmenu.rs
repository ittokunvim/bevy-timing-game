use bevy::prelude::*;

use crate::{
    GAMETITLE,
    WINDOW_SIZE,
    PATH_FONT_MEDIUM,
    PATH_FONT_BOLD,
    PATH_IMAGE_MAINMENU,
    AppState,
};

const GAMETITLE_SIZE: f32 = 40.0;
const GAMETITLE_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const CLICKSTART_TEXT: &str = "Click Start...";
const CLICKSTART_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const TEXT_SIZE: f32 = 30.0;
const TEXT_PADDING: Val = Val::Px(16.0);

#[derive(Component)]
struct Mainmenu;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    println!("mainmenu: setup");
    // game title
    let top = Val::Px(WINDOW_SIZE.y / 2.0 - GAMETITLE_SIZE / 2.0);

    commands.spawn((
        TextBundle::from_section(
            GAMETITLE,
            TextStyle {
                font: asset_server.load(PATH_FONT_BOLD),
                font_size: GAMETITLE_SIZE,
                color: GAMETITLE_COLOR,
            }
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top,
            ..Default::default()
        }),
        Mainmenu,
    ))
    .insert(Name::new("game_title"));
    // click start
    commands.spawn((
        TextBundle::from_section(
            CLICKSTART_TEXT,
            TextStyle {
                font: asset_server.load(PATH_FONT_MEDIUM),
                font_size: TEXT_SIZE,
                color: CLICKSTART_COLOR,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            right: TEXT_PADDING,
            bottom: TEXT_PADDING,
            ..Default::default()
        }),
        Mainmenu,
    ))
    .insert(Name::new("click_start"));
    // image
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(PATH_IMAGE_MAINMENU),
            transform: Transform::from_xyz(0.0, 0.0, -10.0),
            ..Default::default()
        },
        Mainmenu,
    ))
    .insert(Name::new("image"));
}

fn update(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    mouse_events: Res<ButtonInput<MouseButton>>,
    query: Query<Entity, With<Mainmenu>>,
) {
    if !mouse_events.just_pressed(MouseButton::Left) { return }

    println!("mainmenu: despawn");
    for entity in query.iter() { commands.entity(entity).despawn() }
    println!("mainmenu: moved state to Ingame from Mainmeu");
    next_state.set(AppState::Ingame);
}

pub struct MainmenuPlugin;

impl Plugin for MainmenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Mainmenu), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Mainmenu)))
        ;
    }
}
