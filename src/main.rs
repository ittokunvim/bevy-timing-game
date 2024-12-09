use bevy::prelude::*;
// use bevy_hanabi::prelude::*;

mod mainmenu;
mod ingame;
mod gameover;
mod gameclear;

const GAMETITLE: &str = "タイミングゲーム";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const CURSOR_RANGE: f32 = 10.0;
const PATH_IMAGE_CHARACTER: &str = "bevy-timing-game/character.png";
const PATH_IMAGE_BAR: &str = "bevy-timing-game/bar.png";
const PATH_IMAGE_CUE: &str = "bevy-timing-game/cue.png";
const PATH_IMAGE_INGAME: &str = "bevy-timing-game/ingame.png";
const PATH_IMAGE_MAINMENU: &str = "bevy-timing-game/mainmenu.png";
const PATH_IMAGE_TIMINGBUTTON: &str = "bevy-timing-game/timingbutton.png";
const PATH_SOUND_PERFECT: &str = "bevy-timing-game/perfect.ogg";
const PATH_SOUND_GOOD: &str = "bevy-timing-game/good.ogg";
const PATH_SOUND_OK: &str = "bevy-timing-game/ok.ogg";

const PATH_FONT: &str = "fonts/misaki_gothic.ttf";
const PATH_IMAGE_PAUSEBUTTON: &str = "images/pausebutton-light.png";
const PATH_SOUND_CLICK: &str = "sounds/click.ogg";
const PATH_SOUND_REVERSAL: &str = "sounds/reversal.ogg";

const BACKGROUND_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Mainmenu,
    Ingame,
    Pause,
    Gameover,
    Gameclear,
}

#[derive(Resource, Deref, DerefMut, Debug)]
struct Config {
    setup_ingame: bool,
}

#[derive(Resource, Deref)]
struct ClickSound(Handle<AudioSource>);

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
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .add_plugins(mainmenu::MainmenuPlugin)
        .add_plugins(ingame::IngamePlugin)
        .add_plugins(gameover::GameoverPlugin)
        .add_plugins(gameclear::GameclearPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    println!("main: setup");
    // camera
    commands.spawn(Camera2dBundle::default());
    // click sound
    let click_sound = asset_server.load(PATH_SOUND_CLICK);
    commands.insert_resource(ClickSound(click_sound));
}

fn update(
    mut commands: Commands,
    mouse_events: Res<ButtonInput<MouseButton>>,
    sound: Res<ClickSound>,
) {
    if !mouse_events.just_pressed(MouseButton::Left) { return }
    println!("main: play click sound");
    commands.spawn(AudioBundle {
        source: sound.clone(),
        settings: PlaybackSettings::DESPAWN
    });
}
