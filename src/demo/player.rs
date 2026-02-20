use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use avian2d::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    demo::{
        animation::{Direction, PlayerAnimation},
        level::PlayerMarker,
        movement::MovementController,
    },
};

pub const PLAYER_Z_TRANSLATION: f32 = 100.;
pub const PLAYER_COLLIDER_RADIUS: f32 = 10.0;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();

    // Record directional input as movement controls.
    app.add_systems(
        FixedUpdate,
        record_player_directional_input
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

/// The player character.
pub fn player(
    max_speed: f32,
    player_assets: &PlayerAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    // A texture atlas is a way to split a single image into a grid of related images.
    // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    //let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, UVec2::splat(1)), None);
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(20), 5, 15, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();
    (
        Name::new("Player"),
        Player,
        Sprite::from_atlas_image(
            player_assets.char.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: player_animation.get_atlas_index(),
            },
        ),
        MovementController {
            max_speed,
            ..default()
        },
        player_animation,
        PlayerMarker,
        LockedAxes::new().lock_rotation(),
        Transform::from_xyz(0., 0., PLAYER_Z_TRANSLATION),
        RigidBody::Dynamic,
        GravityScale(0.0),
        Collider::circle(PLAYER_COLLIDER_RADIUS),
    )
}

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    // Collect directional input.
    let mut _intent = Vec2::ZERO;

    if input.pressed(KeyCode::KeyW) {
        _intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) {
        _intent.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) {
        _intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) {
        _intent.x += 1.0;
    }

    use Direction::*;
    let dir = match _intent {
        Vec2 { x: 0.0, y: 0.0 } => Nothing,
        Vec2 { x: 0.0, y: 1.0 } => Up,
        Vec2 { x: 0.0, y: -1.0 } => Down,
        Vec2 { x: 1.0, y: 0.0 } => Right,
        Vec2 { x: -1.0, y: 0.0 } => Left,
        Vec2 { x: 1.0, y: 1.0 } => UpRight,
        Vec2 { x: -1.0, y: 1.0 } => UpLeft,
        Vec2 { x: 1.0, y: -1.0 } => DownRight,
        Vec2 { x: -1.0, y: -1.0 } => DownLeft,
        _ => panic!("Unknown intent"),
    };

    for mut controller in &mut controller_query {
        controller.intent = dir.clone();
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    char: Handle<Image>,
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            char: assets.load_with_settings(
                "textures/chars/eris_esra-template-20x20.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            steps: vec![
                assets.load("audio/sound_effects/step1.ogg"),
                assets.load("audio/sound_effects/step2.ogg"),
                assets.load("audio/sound_effects/step3.ogg"),
                assets.load("audio/sound_effects/step4.ogg"),
            ],
        }
    }
}
