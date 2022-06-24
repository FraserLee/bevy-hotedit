#![allow(unused_imports)]
use std::path::PathBuf;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_hotedit::*;

// fn main() { println!("run `cargo test -- --test-threads=1` to test"); }





#[hot] const INT_CONST1: usize;
#[hot] const INT_CONST2: i8;
#[hot] const INT_CONST3: u32;
#[test] fn int_const_load() {
    assert_eq!(INT_CONST1(), 1);
    assert_eq!(INT_CONST2(), -15);
    assert_eq!(INT_CONST3(), 0x7fff_fff0);
}

#[hot] const FLOAT_CONST1: f32;
#[hot] const FLOAT_CONST2: f64;
#[test] fn float_const_load() {
    assert_eq!(FLOAT_CONST1(), -1.0);
    assert_eq!(FLOAT_CONST2(), 3.14);
}

#[hot] const STRING_CONST1: &str;
#[hot] const STRING_CONST2: String; // not sure whether the fact that this
                                    // // works is a good idea or not.
#[test] fn string_const_load() {
    assert_eq!(STRING_CONST1(), "danger, snakes.");
    assert_eq!(STRING_CONST2(), "🐍🐍🐍");
}

#[hot] const BOOL_CONST: bool;
#[test] fn bool_const_load() {
    assert!(BOOL_CONST());
}



// quick bevy program that inits the hotedit plugin and hides the main window
// (for testing)
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // hide main window
        .add_startup_system(|windows: NonSend<WinitWindows>| {
            windows.get_window(WindowId::primary()).unwrap().set_visible(false);
        })
        .add_plugin(HotEditPlugin { auto_open: true })
        .run();
}
