use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::Mutex;
use std::thread;

use bevy::prelude::*;
use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher};
use toml::{ self, Value, value::Table };




#[macro_use] extern crate lazy_static;

pub use bevy_hotedit_macros::*;






pub struct HotEditPlugin {
    pub auto_open: bool,
}

impl Plugin for HotEditPlugin {
    fn build(&self, app: &mut App) {

        app.add_startup_system(setup);

        app.add_startup_system(|| {
            thread::spawn(move || { web_server(); });
        });

        if self.auto_open { // open page in default browser
            open::that("http://localhost:2022").unwrap();
        }

    }
}


lazy_static! {
    // the env! macro has some bugs. This works.
    pub static ref CONFIG_PATH: PathBuf = 
            PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("src/hotedit-values.toml");

    // a single table with all #[hot] values
    static ref CONFIG: Mutex<Table> = Mutex::new( load_config() );
}

fn load_config() -> Table {
    toml::from_str(
        &std::fs::read_to_string( CONFIG_PATH.as_path() ).unwrap()
    ).unwrap()
}

fn setup() {
    // There's probably a bevy-native way to do this, but this works.
    // Create another thread to spin-watch for writes on the config file,
    // re-reading it and setting the config mutex when updated.
    thread::spawn(move || {
        let (tx, rx) = channel();
        let mut watcher = raw_watcher(tx).unwrap();
        watcher.watch(CONFIG_PATH.as_path(), RecursiveMode::NonRecursive).unwrap();

        loop {
            match rx.recv() { 
                Ok(RawEvent { path: _, op: _, cookie: _ }) => {
                    thread::sleep(std::time::Duration::from_millis(100));
                    *CONFIG.lock().unwrap() = load_config();
                }
                Err(e) => eprintln!("watch error: {:?}", e),
            }
        }
    });
}

// lookup some value in the config table
pub fn lookup(ident: &str) -> Value {
    CONFIG.lock().unwrap().get(ident).unwrap().clone()
}




#[tokio::main]
async fn web_server() {
    let app = axum::Router::new().route("/", axum::routing::get(|| async { "Hello, World!" }));
    axum::Server::bind(&"0.0.0.0:2022".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();


}
