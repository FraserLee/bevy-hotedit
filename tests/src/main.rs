#![allow(unused_imports)]
use std::path::PathBuf;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_hotedit::*;

fn main() { println!("run `cargo test` to test"); }



// write all types of constants to a file, then read them back and compare

#[hot] const INT_CONST1: usize;
#[hot] const INT_CONST2: i8;
#[hot] const INT_CONST3: u32;
#[hot] const INT_CONST4: i32 = 42; // unspecified in the file, should assume given value
#[hot] const INT_CONST5: i32 = 42; // specified in the file, should be overwritten by given value
#[hot] const FLOAT_CONST1: f32;
#[hot] const FLOAT_CONST2: f64;
#[hot] const STRING_CONST1: &str;
#[hot] const STRING_CONST2: String; // not sure whether the fact that this
                                    // works is a good idea or not.
#[hot] const BOOL_CONST: bool;

#[test]
fn const_load_test() {
    *CONFIG_PATH.lock().unwrap() = 
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("src/const_load_test.toml");


    std::fs::write(
        CONFIG_PATH.lock().unwrap().as_path(),
        r#" INT_CONST1 = 1
            INT_CONST2 = -15
            INT_CONST3 = 2147483632

            INT_CONST5 = 1

            FLOAT_CONST1 = -1.0
            FLOAT_CONST2 = 3.14

            STRING_CONST1 = 'danger, snakes.'
            STRING_CONST2 = '🐍🐍🐍'

            BOOL_CONST = true
        "#
    ).unwrap();


    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(HotEditPlugin)
        .add_system( |mut exit: EventWriter<AppExit>| {
            assert_eq!(INT_CONST1(), 1);
            assert_eq!(INT_CONST2(), -15);
            assert_eq!(INT_CONST3(), 0x7fff_fff0);
            assert_eq!(INT_CONST4(), 42);
            assert_eq!(INT_CONST5(), 42);
            assert_eq!(FLOAT_CONST1(), -1.0);
            assert_eq!(FLOAT_CONST2(), 3.14);
            assert_eq!(STRING_CONST1(), "danger, snakes.");
            assert_eq!(STRING_CONST2(), "🐍🐍🐍");
            assert!(BOOL_CONST());
            // delete the test file
            std::fs::remove_file(CONFIG_PATH.lock().unwrap().as_path()).unwrap();
            // quit the app
            exit.send(AppExit);
        })
        .run();
}



