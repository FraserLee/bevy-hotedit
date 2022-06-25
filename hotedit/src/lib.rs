use std::path::PathBuf;
use std::collections::HashMap;
use std::sync::Mutex;
use std::thread;

use bevy::prelude::*;

#[macro_use] extern crate lazy_static;

#[macro_use] extern crate rocket;
use rocket::form::{ Form, Contextual, FromForm, /* Context */ };
use rocket_dyn_templates::{ Template, context };



pub use bevy_hotedit_macros::*;
use bevy_hotedit_util as util;
pub use util::Value;




// struct used to init a hotvar, consumed with `.register()`
pub struct HotVar {
    pub name: String,
    pub init_value: Value,
    pub info: VarInfo,
}

pub struct VarInfo {
    pub line_num: usize,
    pub ty: String,
}

impl HotVar {
    pub fn register(self) { // consumes self, registering it in the global map
        let mut values = VALUES.lock().unwrap();
        values.insert(self.name.clone(), self.init_value);

        let mut info = INFO.lock().unwrap();
        info.insert(self.name, self.info);
    }
}


lazy_static! {
    // the env! macro has some bugs. This works.
    pub static ref CONFIG_PATH: PathBuf = 
            PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("src/hotedit-values.toml");

    // hashmap of all current #[hot] values
    static ref VALUES: Mutex<HashMap<String, Value>> = Mutex::new(HashMap::new());

    // hashmap with info about the declared #[hot] places
    static ref INFO: Mutex<HashMap<String, VarInfo>> = Mutex::new(HashMap::new());
}


pub fn lookup(ident: &str) -> Option<Value> {
    // try to lookup the value in the global map.
    match VALUES.lock().unwrap().get(ident) {
        Some(v) => Some(v.clone()),
        None => None,
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
                        .mount("/", routes![post, index])
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




#[get("/")]
fn index() -> Template {
    Template::render("base", context! {
        title: std::env::var("CARGO_PKG_NAME").unwrap(),
        f: context! {
            values: HashMap::<String, String>::new(),
            errors: HashMap::<String, String>::new(),
        },
    })
}



#[allow(dead_code)]
#[derive(Debug, FromForm)]
struct Submission<'v> {
    int: HashMap<String, i32>,
    float: HashMap<String, f32>,
    bool: HashMap<String, bool>,
    string: HashMap<String, &'v str>,
}

#[post("/", data = "<form>")]
fn post<'r>(form: Form<Contextual<'r, Submission<'r>>>) -> Template {
    if let Some(ref submission) = form.value {
        println!("SUBMISSION VALID, {:?}", submission);
    }
    // dbg!(&form);

    Template::render("base", context! {
        title: std::env::var("CARGO_PKG_NAME").unwrap(),
        f: &form.context,
    })
}



