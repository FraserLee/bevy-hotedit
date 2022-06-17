use bevy::prelude::*;
use bevy_hotedit::*;

#[set_const_value]
const X:i32;

#[test]
fn int_const_value_test() {
    assert_eq!(X, 1);
}

#[set_const_value]
const Y:f32;

#[test]
fn float_const_value_test() {
    assert_eq!(Y, 1.0);
}




#[test]
fn bevy() {
    App::new()
        .add_startup_system( | | { assert_eq!(1, 1); })
        .run();
}



