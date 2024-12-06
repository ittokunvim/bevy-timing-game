use bevy::prelude::*;
// use bevy_hanabi::prelude::*;

mod mainmenu;
mod ingame;
mod gameover;

pub const GAMETITLE: &str = "Timing Game";
pub const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
pub const CURSOR_RANGE: f32 = 10.0;
pub const PATH_IMAGE_CHARACTER: &str = "bevy-timing-game/character.png";
pub const PATH_IMAGE_BAR: &str = "bevy-timing-game/bar.png";
pub const PATH_IMAGE_CUE: &str = "bevy-timing-game/cue.png";
pub const PATH_IMAGE_INGAME: &str = "bevy-timing-game/ingame.png";
pub const PATH_IMAGE_MAINMENU: &str = "bevy-timing-game/mainmenu.png";
pub const PATH_IMAGE_TIMINGBUTTON: &str = "bevy-timing-game/timingbutton.png";
pub const PATH_SOUND_PERFECT: &str = "bevy-timing-game/perfect.ogg";
pub const PATH_SOUND_GOOD: &str = "bevy-timing-game/good.ogg";
pub const PATH_SOUND_OK: &str = "bevy-timing-game/ok.ogg";

pub const PATH_FONT_MEDIUM: &str = "fonts/FiraMono-Medium.ttf";
pub const PATH_FONT_BOLD: &str = "fonts/FiraSans-Bold.ttf";
pub const PATH_IMAGE_PAUSEBUTTON: &str = "images/pausebutton-light.png";
pub const PATH_SOUND_TIMING: &str = "sounds/click.ogg";
pub const PATH_SOUND_REVERSAL: &str = "sounds/reversal.ogg";

const BACKGROUND_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Mainmenu,
    Ingame,
    Pause,
    Gameover,
}

#[derive(Resource, Deref, DerefMut, Debug)]
struct Config {
    setup_ingame: bool,
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
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .insert_resource(Config { setup_ingame: true })
        .insert_resource(Score(0))
        // // Hanabi setup
        // .add_plugins(HanabiPlugin)
        // Plugins
        .add_systems(Startup, setup_camera)
        .add_plugins(mainmenu::MainmenuPlugin)
        .add_plugins(ingame::IngamePlugin)
        .add_plugins(gameover::GameoverPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    println!("main: setup");
    // camera
    commands.spawn(Camera2dBundle::default());
}
