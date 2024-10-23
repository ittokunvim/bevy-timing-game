use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod mainmenu;
mod ingame;
mod pause;
mod gameover;

pub const GAMETITLE: &str = "Timing Game";
pub const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
pub const PATH_BG_IMAGE: &str = "images/mainmenu.png";
pub const PATH_PAUSE_IMAGE: &str = "images/pause.png";
pub const PATH_PLAY_IMAGE: &str = "images/play.png";
pub const PATH_FONT_MEDIUM: &str = "fonts/FiraMono-Medium.ttf";
pub const PATH_FONT_BOLD: &str = "fonts/FiraSans-Bold.ttf";
pub const PATH_LDTK_PROJECT: &str = "ldtk/bevy-timing-game.ldtk";
pub const PATH_SOUND_REVERSAL: &str = "sounds/reversal.ogg";
pub const PATH_SOUND_TIMING: &str = "sounds/timing.ogg";

const BG_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Mainmenu,
    Ingame,
    Pause,
    Gameover,
}

#[derive(Resource, Deref, DerefMut)]
pub struct Score(pub usize);

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
        .insert_resource(Score(0))
        // Ldtk setup
        .add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::index(0))
        // Hanabi setup
        .add_plugins(HanabiPlugin)
        // Inspector setup
        .add_plugins(WorldInspectorPlugin::new())
        // Plugins
        .add_systems(Startup, setup_camera)
        .add_plugins(mainmenu::MainmenuPlugin)
        .add_plugins(ingame::IngamePlugin)
        .add_plugins(pause::PausePlugin)
        .add_plugins(gameover::GameoverPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
