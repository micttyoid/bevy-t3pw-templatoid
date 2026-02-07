use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::Aseprite;
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{LoadingState, LoadingStateAppExt, config::ConfigureLoadingState},
};

use crate::{screens::Screen, utils::tiled::TiledMap};

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(LoadingState::new(Screen::Splash).load_collection::<TitleAssets>())
        .add_loading_state(
            LoadingState::new(Screen::Loading)
                .continue_to_state(Screen::Gameplay)
                .load_collection::<GameplayAssets>(),
        );
}

// Assets for gameplay loaded during load screen
#[derive(AssetCollection, Resource)]
pub struct GameplayAssets {
    #[asset(path = "textures/chars/player.aseprite")]
    pub character: Handle<Aseprite>,
    #[asset(path = "audio/music/Fluffing A Duck.ogg")]
    pub gameplay_bg_music: Handle<AudioSource>,
    #[asset(path = "map_tile-16x16.tmx")]
    pub map: Handle<TiledMap>,
}

// Assets for title screen loaded during splash screen
#[derive(AssetCollection, Resource)]
pub struct TitleAssets {
    #[asset(path = "audio/sound_effects/button_click.ogg")]
    pub button_click_sfx: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/button_hover.ogg")]
    pub button_hover_sfx: Handle<AudioSource>,
}
