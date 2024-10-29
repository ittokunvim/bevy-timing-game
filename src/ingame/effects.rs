use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_hanabi::prelude::*;

use crate::AppState;

use crate::ingame::{
    TimingEvent,
    ReversalEvent,
};
use crate::ingame::cue::Cue;
use crate::ingame::bar::{
    BAR_SIZE,
    Bar,
};

const MAX_EFFECT_CAPACITY: u32 = 4096;

#[derive(Component)]
struct TimingEffect;

#[derive(Component)]
struct ReversalEffect;

fn setup(
    ldtk_project_entities: Query<&Handle<LdtkProject>>,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut commands: Commands,
) {
    if !ldtk_project_entities.is_empty() { return }

    // Timing effect
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.0, 0.7, 0.0, 1.0));
    gradient.add_key(1.0, Vec4::new(0.0, 0.7, 0.0, 0.0));
    let writer = ExprWriter::new();
    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(5.0).expr(),
        dimension: ShapeDimension::Surface,
    };
    let init_vel = SetVelocityCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        speed: writer.lit(10.0).expr(),
    };
    let age = writer.lit(0.0).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);
    let lifetime = writer.lit(1.0).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);
    let effect = EffectAsset::new(vec![MAX_EFFECT_CAPACITY], Spawner::once(1000.0.into(), true), writer.finish())
        .with_name("timing_effect")
        .init(init_pos)
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::constant(Vec2::splat(1.0)),
            screen_space_size: false,
        })
        .render(ColorOverLifetimeModifier { gradient });
    let effect_handle = effects.add(effect);

    commands.spawn((
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect_handle).with_z_layer_2d(Some(10.0)),
            ..default()
        },
        TimingEffect,
    ))
    .insert(Name::new("timing_effect"));
    // Reversal effect
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.7, 0.0, 0.0, 1.0));
    gradient.add_key(0.8, Vec4::new(0.7, 0.0, 0.0, 0.5));
    gradient.add_key(1.0, Vec4::new(0.7, 0.0, 0.0, 0.0));
    let writer = ExprWriter::new();
    let init_pos = SetPositionCone3dModifier {
        base_radius: writer.lit(2.0).expr(),
        top_radius: writer.lit(10.0).expr(),
        height: writer.lit(10.0).expr(),
        dimension: ShapeDimension::Volume,
    };
    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(20.0).expr(),
    };
    let age = writer.lit(0.0).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);
    let lifetime = writer.lit(1.0).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);
    let size = (writer.rand(ScalarType::Float) * writer.lit(1.5) + writer.lit(1.5)).expr();
    let init_size = SetAttributeModifier::new(Attribute::SIZE, size);
    let effect = EffectAsset::new(vec![MAX_EFFECT_CAPACITY], Spawner::once(50.0.into(), false), writer.finish())
        .with_name("reversal_effect")
        .init(init_pos)
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .init(init_size)
        .render(ColorOverLifetimeModifier { gradient });
    let effect_handle = effects.add(effect);

    commands.spawn((
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect_handle).with_z_layer_2d(Some(10.0)),
            ..default()
        },
        ReversalEffect,
    ))
    .insert(Name::new("reversal_effect"));
}

// Generates an effect at the cue position when the player sets the timing
fn update_timing_effect(
    mut timing_events: EventReader<TimingEvent>,
    mut effect: Query<(&mut EffectSpawner, &mut Transform), (With<TimingEffect>, Without<Cue>)>,
    cue_query: Query<&Transform, With<Cue>>,
) {
    if timing_events.is_empty() { return }
    timing_events.clear();

    let Ok((mut spawner, mut effect_transform)) = effect.get_single_mut() else { return; };
    let cue_transform = cue_query.single();
    // spawn timing effect
    effect_transform.translation = cue_transform.translation;
    spawner.reset();
}

// Causes an effect when the cue is flipped
fn update_reversal_effect(
    mut reversal_events: EventReader<ReversalEvent>,
    mut effect: Query<(&mut EffectSpawner, &mut Transform), (With<ReversalEffect>, Without<Bar>)>,
    bar_query: Query<&Transform, (With<Bar>, Without<ReversalEffect>)>,
    cue_query: Query<&Cue, With<Cue>>,
) {
    if reversal_events.is_empty() { return }
    reversal_events.clear();

    let Ok((mut effect_spawner, mut effect_transform)) = effect.get_single_mut() else { return; };
    let bar_transform = bar_query.single();
    let bar_xy = bar_transform.translation.xy();
    let cue_prop = cue_query.single();

    let effect_transform_x = match cue_prop.toggle_move {
        true => bar_xy.x - BAR_SIZE.x / 2.0,
        false => bar_xy.x + BAR_SIZE.x / 2.0,
    };
    let effect_rotation_z = if cue_prop.toggle_move { 1.5 } else { -1.5 };
    // spawn reversal effect
    effect_transform.translation = Vec3::new(effect_transform_x, bar_xy.y, 0.0);
    effect_transform.rotation = Quat::from_rotation_z(effect_rotation_z);
    effect_spawner.reset();
}

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                update_timing_effect,
                update_reversal_effect,
            ).run_if(in_state(AppState::Ingame)));
    }
}
