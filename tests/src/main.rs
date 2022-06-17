use bevy::prelude::*;
use bevy_hotedit::*;

#[test]
fn test_pass() {
    App::new()
        .add_startup_system( | | { assert_eq!(X, 1); })
        .run();
}

#[test]
fn test_fail() {
    App::new()
        .add_startup_system( | | { assert_eq!(X, 2); })
        .run();
}

#[set_const_value]
const X:i32;


