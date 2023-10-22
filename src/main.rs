mod setup;
mod menu;
mod common;

use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use crate::common::*;
use crate::setup::BootPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window{
                title: strings::APP_NAME.to_string(),
                resolution: (BASE_WINDOW_WIDTH,BASE_WINDOW_HEIGHT).into(),
               ..default()
            }),
            ..default()
        }))
        .add_plugins(AudioPlugin)
        .add_plugins(CommonPlugin)
        .add_plugins(BootPlugin)
        .run();
}