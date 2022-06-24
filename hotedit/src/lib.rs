use std::path::PathBuf;
use std::collections::HashMap;
use std::sync::Mutex;
use std::thread;

use bevy::prelude::*;

#[macro_use] extern crate lazy_static;

#[macro_use] extern crate rocket;
use rocket::form::{ Form, Contextual, FromForm, Context };
use rocket_dyn_templates::{ Template, /* context */ };



pub use bevy_hotedit_macros::*;
use bevy_hotedit_util as util;
pub use util::Value;




pub struct HotVariable {
    pub name: String,
    pub line_num: usize,
    pub value: Value,
}

impl HotVariable {
    pub fn register(self) { // consumes self, registering it in the global map
        let mut hot_vars = HOT_VARS.lock().unwrap();
        hot_vars.insert(self.name.clone(), self);
    }
}


lazy_static! {
    // the env! macro has some bugs. This works.
    pub static ref CONFIG_PATH: PathBuf = 
            PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("src/hotedit-values.toml");

    // a single table with all #[hot] values
    static ref HOT_VARS: Mutex<HashMap<String, HotVariable>> = Mutex::new(HashMap::new());
}


pub fn lookup(ident: &str) -> Option<Value> {
    // try to lookup the value in the global map, and if that fails, try to
    // parse it from the config file. Should both fail, return None.
    match HOT_VARS.lock().unwrap().get(ident) {
        Some(var) => Some(var.value.clone()),
        None => util::lookup_from_file(ident, CONFIG_PATH.to_str().unwrap()),
    }
}










pub struct HotEditPlugin {
    pub auto_open: bool,
}

impl Plugin for HotEditPlugin {
    fn build(&self, app: &mut App) {

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














#[derive(Debug, FromForm)]
struct Submission<'v> {
    foo: &'v str,
    bar: i32,
    baz: f32,
    qux: bool,
}





#[get("/")]
fn index() -> Template {
    Template::render("base", &Context::default())
    // context! {
        // title: env!("CARGO_PKG_NAME"),
    // })
}

#[post("/", data = "<form>")]
fn submit<'r>(form: Form<Contextual<'r, Submission<'r>>>) -> Template {
    if let Some(ref submission) = form.value {
        println!("SUBMISSION VALID, {:?}", submission);
    }
    dbg!(&form);

    Template::render("base", &form.context)
}


