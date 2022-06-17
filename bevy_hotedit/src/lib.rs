use bevy::prelude::*;

pub use bevy_hotedit_macros::*;
pub use toml::{ self, value::Table };

pub struct HotEditPlugin;

impl Plugin for HotEditPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup() {
    println!("HotEditPlugin::setup");
}



