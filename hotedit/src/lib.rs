use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::Mutex;
use std::thread;

use bevy::prelude::*;
use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher};
use toml::{ self, Value, value::Table };

#[macro_use] extern crate lazy_static;

#[macro_use] extern crate rocket;
use rocket::form::{Form, Contextual, FromForm, FromFormField, Context};
use rocket_dyn_templates::{ Template, context };



pub use bevy_hotedit_macros::*;


pub struct HotEditPlugin {
    pub auto_open: bool,
}

impl Plugin for HotEditPlugin {
    fn build(&self, app: &mut App) {

        app.add_startup_system(setup);

        app.add_startup_system(|| {
            thread::spawn(move || { 
                rocket::async_main(async move {
                    let app = rocket::build()
                        .mount("/", routes![index, submit])
                        .attach(Template::fairing());
                    let _ = app.launch().await;
                });
            });
        });

        if self.auto_open { // open page in default browser
            open::that("http://localhost:8000").unwrap();
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








/*
<form action="/" method="post">
    <input type="text" placeholder="foo (string)"
        name="foo" id="foo" value="" />
    <input type="number" placeholder="bar (number)"
        name="bar" id="bar" value="" />
    <input type="checkbox" placeholder="baz (bool)"
        name="baz" id="baz" value="" />
</form>
*/

#[derive(Debug, FromForm)]
struct Submission<'v> {
    foo: &'v str,
    bar: i32,
    baz: bool,
}





#[get("/")]
fn index() -> Template {
    Template::render("base", context! {
        title: env!("CARGO_PKG_NAME"),
    })
}



#[post("/", data = "<form>")]
fn submit<'r>(form: Form<Contextual<'r, Submission<'r>>>) -> Template {
    dbg!(form);
    Template::render("base", context! {
        title: env!("CARGO_PKG_NAME"),
    })
}


