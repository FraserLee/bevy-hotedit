#![allow(unused_imports)]
use std::path::PathBuf;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_hotedit::*;

fn main() { println!("run `cargo test -- --test-threads=1` to test"); }

#[hot] const INT_CONST1: usize;
#[hot] const INT_CONST2: i8;
#[hot] const INT_CONST3: u32;
#[hot] const FLOAT_CONST1: f32;
#[hot] const FLOAT_CONST2: f64;
#[hot] const STRING_CONST1: &str;
#[hot] const STRING_CONST2: String; // not sure whether the fact that this
                                    // works is a good idea or not.
#[hot] const BOOL_CONST: bool;

#[hot] const DYNAMIC: i32;

#[allow(dead_code)] struct ChangeTimer(Timer);
#[allow(dead_code)] struct ExitTimer(Timer);

// bevy acts weird if you try to have multiple doing file-related things at once
// (even with -- --test-threads=1 or other serial-enforcement options) so this
// rather inelegant workaround is used instead.
#[test]
fn mega_test() {

    // test all the constants are what we expect from the file

    assert_eq!(INT_CONST1(), 1);
    assert_eq!(INT_CONST2(), -15);
    assert_eq!(INT_CONST3(), 0x7fff_fff0);
    assert_eq!(FLOAT_CONST1(), -1.0);
    assert_eq!(FLOAT_CONST2(), 3.14);
    assert_eq!(STRING_CONST1(), "danger, snakes.");
    assert_eq!(STRING_CONST2(), "🐍🐍🐍");
    assert!(BOOL_CONST());

    // test to ensure changing the file at runtime updates the constant

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(HotEditPlugin)
        .insert_resource(ChangeTimer(Timer::from_seconds(0.25, false)))
        .insert_resource(ExitTimer(Timer::from_seconds(0.5, false)))
        .add_system( |
            mut ct: ResMut<ChangeTimer>, 
            mut et: ResMut<ExitTimer>,
            time: Res<Time>,
            mut exit: EventWriter<AppExit>,
            | {
            // after 0.25 seconds, change the constant to 2
            if ct.0.tick(time.delta()).just_finished() {
                std::fs::rename( // backup old toml
                    CONFIG_PATH.as_path(),
                    CONFIG_PATH.with_extension("backup.toml").as_path()
                ).unwrap();
                std::fs::write( // write new toml with just the constant
                    CONFIG_PATH.as_path(), b"DYNAMIC = 2\n"
                ).unwrap();
            }

            // after 0.5 seconds, panic with an error message if the constant is not 2
            if et.0.tick(time.delta()).just_finished() {
                std::fs::rename( // restore old toml
                    CONFIG_PATH.with_extension("backup.toml").as_path(),
                    CONFIG_PATH.as_path(),
                ).unwrap();
                if DYNAMIC() != 2 { panic!("dynamic constant did not change"); }
                exit.send(AppExit);
            }
        })
        .run();
}
