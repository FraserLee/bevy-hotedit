use bevy::prelude::*;
use bevy_hotedit::*;

#[test]
fn test_pass() {
    App::new()
        .add_startup_system( | | {
            assert_eq!(one(), 1);
        })
        .run();
}

#[test]
fn test_fail() {
    App::new()
        .add_startup_system( | | {
            assert_eq!(one(), 2);
        })
        .run();
}



