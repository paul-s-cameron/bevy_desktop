#![allow(unused)]
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.init_state::<AssetState>();
    app.add_loading_state(
        LoadingState::new(AssetState::Loading)
            .continue_to_state(AssetState::Loaded)
            .load_collection::<Assets>()
    );
}

#[derive(States, Clone, PartialEq, Eq, Debug, Hash, Default)]
pub(crate) enum AssetState {
    #[default]
    Loading,
    Loaded,
}

#[derive(AssetCollection, Resource)]
pub(crate) struct Assets {
    #[asset(path = "embedded://bevy.png")]
    pub image: Handle<Image>,
}