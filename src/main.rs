use bevy::prelude::*;

mod mainmenu;

use crate::mainmenu::{
    mainmenu_setup,
    mainmenu_update,
};

pub const GAMETITLE: &str = "Timing Game";
pub const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const BG_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);

pub const BG_IMAGE_PATH: &str = "images/mainmenu.png";
pub const FONT_MEDIUM_PATH: &str = "fonts/FiraMono-Medium.ttf";
pub const FONT_BOLD_PATH: &str = "fonts/FiraSans-Bold.ttf";

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Mainmenu,
    Ingame,
    Gameover
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WINDOW_SIZE.into(),
                    title: GAMETITLE.to_string(),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
        )
        .init_state::<AppState>()
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(AppState::Mainmenu), mainmenu_setup)
        .add_systems(Update, mainmenu_update.run_if(in_state(AppState::Mainmenu)))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
