use bevy::prelude::*;

#[test]
fn startup_test() {
    App::new()
        .add_startup_system( | | {
            assert_eq!(0, 1);
        })
        .run();
}

