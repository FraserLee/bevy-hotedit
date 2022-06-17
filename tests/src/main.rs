#![allow(unused_imports)]
use bevy::prelude::*;
use bevy_hotedit::*;




#[hot] const INT_CONST1: usize;
#[hot] const INT_CONST2: i8;
#[hot] const INT_CONST3: u32;
#[test] fn int_const_load_test() {
    assert_eq!(INT_CONST1, 1);
    assert_eq!(INT_CONST2, -15);
    assert_eq!(INT_CONST3, 0x7fff_fff0);
}




#[hot] const FLOAT_CONST1: f32;
#[hot] const FLOAT_CONST2: f64;
#[test] fn float_const_load_test() {
    assert_eq!(FLOAT_CONST1, -1.0);
    assert_eq!(FLOAT_CONST2, 3.14);
}



#[hot] const STRING_CONST1: &str;
#[hot] const STRING_CONST2: &str;
#[test] fn string_const_load_test() {
    assert_eq!(STRING_CONST1, "danger, snakes.");
    assert_eq!(STRING_CONST2, "🐍🐍🐍");
}


#[hot] const BOOL_CONST: bool;
#[test] fn bool_const_load_test() { assert_eq!(BOOL_CONST, true); }




#[test]
fn bevy_test() {
    App::new()
        .add_startup_system( | | { assert_eq!(1, 1); })
        .run();
}


fn main() { println!("run `cargo test` to test"); }

