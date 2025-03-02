#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_template::AppPlugin;

fn main() {
    App::new()
        .add_plugins(AppPlugin)
        .run();
}
