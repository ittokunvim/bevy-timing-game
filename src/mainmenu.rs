use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    GAMETITLE,
    WINDOW_SIZE,
    PATH_FONT,
    PATH_IMAGE_MAINMENU,
    AppState,
    Config,
};

const GAMETITLE_SIZE: f32 = 32.0;
const GAMETITLE_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const CLICKSTART_TEXT: &str = "クリックしてスタート";
const CLICKSTART_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);
const BOARD_SIZE: Vec2 = Vec2::new(320.0, 240.0);
const BOARD_COLOR: Color = Color::srgba(0.9, 0.9, 0.9, 0.75);
const TEXT_SIZE: f32 = 20.0;

#[derive(Component)]
struct Mainmenu;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    println!("mainmenu: setup");
    // game title
    let top = Val::Px(WINDOW_SIZE.y / 2.0 - GAMETITLE_SIZE / 2.0 - BOARD_SIZE.y / 4.0);

    commands.spawn((
        TextBundle::from_section(
            GAMETITLE,
            TextStyle {
                font: asset_server.load(PATH_FONT),
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
    .insert(Name::new("gametitle"));
    // click start
    let top = Val::Px(WINDOW_SIZE.y / 2.0 - TEXT_SIZE / 2.0 + BOARD_SIZE.y / 4.0);

    commands.spawn((
        TextBundle::from_section(
            CLICKSTART_TEXT,
            TextStyle {
                font: asset_server.load(PATH_FONT),
                font_size: TEXT_SIZE,
                color: CLICKSTART_COLOR,
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top,
            ..Default::default()
        }),
        Mainmenu,
    ))
    .insert(Name::new("clickstart"));
    // board
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(BOARD_SIZE.x, BOARD_SIZE.y))),
            material: materials.add(BOARD_COLOR),
            ..Default::default()
        },
        Mainmenu,
    ))
    .insert(Name::new("board"));
    // image
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(PATH_IMAGE_MAINMENU),
            transform: Transform::from_xyz(0.0, 0.0, -99.0),
            ..Default::default()
        },
        Mainmenu,
    ))
    .insert(Name::new("image"));
}

fn update(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    mut config: ResMut<Config>, 
    mouse_events: Res<ButtonInput<MouseButton>>,
    query: Query<Entity, With<Mainmenu>>,
) {
    if !mouse_events.just_pressed(MouseButton::Left) { return }

    println!("mainmenu: config setup ingame is true");
    config.setup_ingame = true;
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
