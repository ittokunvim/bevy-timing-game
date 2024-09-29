use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod mainmenu;
mod ingame;

use crate::mainmenu::{
    mainmenu_setup,
    mainmenu_update,
};

use crate::ingame::{
    CueBundle,
    DecideEvent,
    Score,
    ingame_setup,
    ingame_update,
    cue_movement,
    decide_timing,
    play_decide_sound,
    update_scoreboard,
};

pub const GAMETITLE: &str = "Timing Game";
pub const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const BG_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);

pub const BG_IMAGE_PATH: &str = "images/mainmenu.png";
pub const FONT_MEDIUM_PATH: &str = "fonts/FiraMono-Medium.ttf";
pub const FONT_BOLD_PATH: &str = "fonts/FiraSans-Bold.ttf";
pub const LDTK_PROJECT_PATH: &str = "bevy-timing-game.ldtk";
pub const DECIDE_SOUND_PATH: &str = "sounds/timing.ogg";

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
        .insert_resource(Score(0))
        .add_event::<DecideEvent>()
        // Ldtk setup
        .add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_entity::<CueBundle>("Cue")
        // Setup
        .add_systems(Startup, setup_camera)
        // Mainmenu
        .add_systems(OnEnter(AppState::Mainmenu), mainmenu_setup)
        .add_systems(Update, mainmenu_update.run_if(in_state(AppState::Mainmenu)))
        // Ingame
        .add_systems(OnEnter(AppState::Ingame), ingame_setup)
        .add_systems(Update, (
            ingame_update,
            cue_movement,
            decide_timing,
            play_decide_sound,
            update_scoreboard,
        ).run_if(in_state(AppState::Ingame)))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
