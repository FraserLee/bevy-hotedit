use duplicate::duplicate_item;
use std::io::prelude::*;

pub const UTIL_PATH: &str = std::file!();

// this needs to have generic-enough conversions to make the procedural macro
// not be a nightmare, hence the rather ugly mess of boilerplate here.

// at least pain in this section lets the rest stay clean.

#[derive(Clone, Debug)]
pub enum Value {
    Float(f64),
    Int(i64),
    String(String),
    Boolean(bool),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Float(f) =>   format!("{}", f),
            Value::Int(i) =>     format!("{}", i),
            Value::String(s) =>  format!("\"{}\"", s),
            Value::Boolean(b) => format!("{}", b),
        }
    }
}


#[duplicate_item( 
    int_type; 
    [ i8 ]; [ i16 ]; [ i32 ]; [ i64 ]; [ isize ];
    [ u8 ]; [ u16 ]; [ u32 ]; [ u64 ]; [ usize ];
)]
impl From<Value> for int_type { fn from(v: Value) -> Self { match v { 
    Value::Int(i) => i as int_type,
    _ => panic!("Value::Int(i) expected, got {:?}", v),
} } }

#[duplicate_item( float_type ; [ f32 ]; [ f64 ]; )]
impl From<Value> for float_type { fn from(v: Value) -> Self { match v { 
    Value::Float(f) => f as float_type,
    _ => panic!("Value::Float(f) expected, got {:?}", v),
} } }

impl From<Value> for bool { fn from(v: Value) -> Self { match v { 
    Value::Boolean(b) => b, 
    _ => panic!("Value::Boolean(b) expected, got {:?}", v),
} } }

impl From<Value> for String { fn from(v: Value) -> Self { match v { 
    Value::String(s) => s, 
    _ => panic!("Value::String(s) expected, got {:?}", v),
} } }


#[duplicate_item( 
    int_type; 
    [ i8 ]; [ i16 ]; [ i32 ]; [ i64 ]; [ isize ];
    [ u8 ]; [ u16 ]; [ u32 ]; [ u64 ]; [ usize ];
)]
impl From<int_type> for Value { fn from(i: int_type) -> Self { Value::Int(i as i64) } }

#[duplicate_item( float_type ; [ f32 ]; [ f64 ]; )]
impl From<float_type> for Value { fn from(f: float_type) -> Self { Value::Float(f as f64) } }

impl From<bool> for Value { fn from(b: bool) -> Self { Value::Boolean(b) } }

impl From<String> for Value { fn from(s: String) -> Self { Value::String(s) } }
impl From<&str> for Value { fn from(s: &str) -> Self { Value::String(s.to_string()) } }

impl From<Value> for toml::Value { fn from(v: Value) -> Self { match v { 
    Value::Float(f) => toml::Value::Float(f),
    Value::Int(i) => toml::Value::Integer(i),
    Value::String(s) => toml::Value::String(s),
    Value::Boolean(b) => toml::Value::Boolean(b),
} } }

impl From<toml::Value> for Value { fn from(v: toml::Value) -> Self { match v { 
    toml::Value::Float(f) => Value::Float(f),
    toml::Value::Integer(i) => Value::Int(i),
    toml::Value::String(s) => Value::String(s),
    toml::Value::Boolean(b) => Value::Boolean(b),
    _ => panic!("toml::Value not fully supported, got {:?}", v),
} } }





// read a field from a toml file
pub fn lookup_from_file(ident: &str, path: &str) -> Option<Value> {
    let file_t = read_toml(path);

    if !file_t.contains_key(ident) { return None; }

    let value_t = file_t.get(ident);

    match value_t {
        None => None,
        Some(v) => Some(v.clone().into())
    }
}

// write a field to a toml file
pub fn write_to_file(ident: &str, value: &str, path: &str) {
    let mut file_t = read_toml(path);

    // file needs to be parsed twice, so arbitrary ident and value strings
    // (like a.b = [1,2,3]) can be written to the correct format.
    
    file_t.remove(ident);

    file_t = parse_toml(&format!("{} = {}\n\n\n\n{}", 
        ident,
        value,
        toml::to_string(&file_t).unwrap()
    ));

    std::fs::write(path, toml::to_string_pretty(&file_t).unwrap().as_bytes()).unwrap();
}

pub fn read_toml(path: &str) -> toml::value::Table {
    let mut file = std::fs::OpenOptions::new().read(true).write(true).create(true).open(path).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    parse_toml(&buffer)
}

fn parse_toml(toml: &str) -> toml::value::Table {
    let file_t: Result<toml::value::Table, _> = toml::from_str(toml);

    if file_t.is_err() { panic!("Error parsing, {} is not a valid toml file", toml); }

    file_t.unwrap()
}







