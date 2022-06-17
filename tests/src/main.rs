#![allow(unused_imports)]
use bevy::prelude::*;
use bevy_hotedit::*;
use toml::value::Table;






#[hot] const INT_CONST1: usize;
#[hot] const INT_CONST2: i8;
#[hot] const INT_CONST3: u32;
#[test] fn int_const_load_test() {
    assert_eq!(INT_CONST1(), 1);
    assert_eq!(INT_CONST2(), -15);
    assert_eq!(INT_CONST3(), 0x7fff_fff0);
}




#[hot] const FLOAT_CONST1: f32;
#[hot] const FLOAT_CONST2: f64;
#[test] fn float_const_load_test() {
    assert_eq!(FLOAT_CONST1(), -1.0);
    assert_eq!(FLOAT_CONST2(), 3.14);
}



#[hot] const STRING_CONST1: &str;
#[hot] const STRING_CONST2: &str;

#[test] fn string_const_load_test() {
    assert_eq!(STRING_CONST1(), "danger, snakes.");
    assert_eq!(STRING_CONST2(), "🐍🐍🐍");
}


#[hot] const BOOL_CONST: bool;
#[test] fn bool_const_load_test() { assert_eq!(BOOL_CONST(), true); }




#[test]
fn bevy_test() {
    App::new()
        .add_startup_system( | | { assert_eq!(1, 1); })
        .run();
}

        
// fn main() { println!("run `cargo test` to test"); }

#[hot] const FOOBAR: i32 = 1;

fn main() { 
    App::new()
        .add_plugins(MinimalPlugins)
        .add_system(|| {
            println!("{}", FOOBAR);
        })
        .run();
}

/*

use bevy::app::ScheduleRunnerSettings;
use bevy::utils::Duration;
use std::sync::Mutex;
        
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SECONDS: Mutex<usize> = Mutex::new(0);
}
        
fn main() {        
    App::new()
        .add_plugins(MinimalPlugins)
        .add_startup_system( || {
            println!("run `cargo test` to test");
        }).add_system(|time: Res<Time>| {
        // set the seconds to the current time
        let mut seconds = SECONDS.lock().unwrap();
        *seconds = time.seconds_since_startup() as usize;

           
   
        }).add_system(|| {

            println!("{}", SECONDS.lock().unwrap());

        }).run();
}

*/



