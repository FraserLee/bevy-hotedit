use std::path::PathBuf;
use std::collections::HashMap;
use std::sync::Mutex;
use std::thread;

use bevy::prelude::*;

#[macro_use] extern crate lazy_static;

#[macro_use] extern crate rocket;
use rocket::form::{ Form, Contextual, FromForm };
use rocket_dyn_templates::{ Template, context };



pub use bevy_hotedit_macros::*;
use bevy_hotedit_util as util;
pub use util::Value;




lazy_static! {
    // the env! macro has some bugs. This works.
    pub static ref CONFIG_PATH: PathBuf = 
            PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("src/hotedit-values.toml");

    // hashmap of all current #[hot] values
    static ref VALUES: Mutex<HashMap<String, Value>> = Mutex::new(HashMap::new());

    // hashmap with info about the declared #[hot] places
    static ref INFO: Mutex<HashMap<String, toml::Value>> = Mutex::new(HashMap::new());
}

use std::sync::Once;
static LOAD_VALUES: Once = Once::new();

// try to lookup the value in the global map.
pub fn lookup(ident: &str) -> Option<Value> {
    LOAD_VALUES.call_once(|| {

        let mut values = VALUES.lock().unwrap();
        util::read_toml(CONFIG_PATH.to_str().unwrap())
            .into_iter()
            .for_each(|(k, v)| {
            values.insert(k, v.into());
        });

    });




    match VALUES.lock().unwrap().get(ident) {
        Some(v) => Some(v.clone()),
        None => None,
    }
}


// try to lookup the value in the global map, or if not found look it up in the
// toml file (useful for populating initial load-values).
fn lookup_or_file(ident: &str) -> Option<Value> {
    match lookup(ident) {
        Some(v) => Some(v),
        None => util::lookup_from_file(ident, CONFIG_PATH.to_str().unwrap())
    }
}







pub struct HotEditPlugin {
    pub auto_open: bool,
}

impl Plugin for HotEditPlugin {
    fn build(&self, app: &mut App) {
        if !cfg!(debug_assertions) { return; }

        app.add_startup_system(|| {
            thread::spawn(move || { 

                // load debug.toml into INFO
                let mut info = INFO.lock().unwrap();
                let debug_path = util::UTIL_PATH.replace("util/src/lib.rs", "debug.toml");
                let debug_t = util::read_toml(&debug_path);

                for (k, v) in debug_t.into_iter() { info.insert(k, v); }

                drop(info);

                // create rocket app
                rocket::async_main(async move {
                    let app = rocket::build()
                        .mount("/", routes![
                            index,
                            post, 
                        ])
                        .attach(Template::fairing());
                    let _ = app.launch().await;
                });
            });
        });

        // open page in default browser
        if self.auto_open { 
            open::that("http://localhost:8000").unwrap();
        }

    }
}




#[get("/")]
fn index() -> Template {
    let info = INFO.lock().unwrap();
    let info = info.clone();

    let mut values = HashMap::<String, toml::Value>::new();

    for (k, field) in info.iter() {
        let v: toml::Value = lookup_or_file(k).unwrap().into();
        let v_arr = toml::Value::Array(vec![v]);




        values.insert(
            format!("{}.{}", field["type"].as_str().unwrap(), k.as_str()),
            v_arr,
        );
    }

    let f = context! {
        values,
        errors: HashMap::<String, String>::new(),
    };

    let c = context! {
        title: std::env::var("CARGO_PKG_NAME").unwrap(),
        f,
        fields: info,
    };
    Template::render("base", c)
}




#[derive(Debug, FromForm)]
struct Submission<'v> {
    int: HashMap<String, i64>,
    float: HashMap<String, f64>,
    bool: HashMap<String, bool>,
    string: HashMap<String, &'v str>,
}


#[post("/", data = "<form>")]
fn post<'r>(form: Form<Contextual<'r, Submission<'r>>>) -> Template {

    let info = INFO.lock().unwrap();
    let info = info.clone();

    if let Some(ref s) = form.value {
        let mut file_t = util::read_toml(CONFIG_PATH.to_str().unwrap());
        let mut values = VALUES.lock().unwrap();

        for (k, v) in s.int.iter() {
            file_t.insert(k.clone(), toml::Value::Integer(*v));
            values.insert(k.clone(), Value::Int(*v));
        }

        for (k, v) in s.float.iter() {
            file_t.insert(k.clone(), toml::Value::Float(*v));
            values.insert(k.clone(), Value::Float(*v));
        }

        for (k, v) in s.string.iter() {
            file_t.insert(k.clone(), toml::Value::String(v.to_string()));
            values.insert(k.clone(), Value::String(v.to_string()));
        }

        // false bools don't get sent, so we need to check for them another way
        for (k, v) in info.iter() {
            if v["type"].as_str().unwrap() == "bool" {
                let b = *s.bool.get(k).unwrap_or(&false);
                file_t.insert(k.clone(), toml::Value::Boolean(b));
                values.insert(k.clone(), Value::Boolean(b));
            }
        }


        std::fs::write(
            CONFIG_PATH.to_str().unwrap(), 
            toml::to_string_pretty(&file_t).unwrap()
        ).unwrap();
    }


    let c = context! {
        title: std::env::var("CARGO_PKG_NAME").unwrap(),
        f: &form.context,
        fields: info,
    };
    Template::render("base", c)
}


