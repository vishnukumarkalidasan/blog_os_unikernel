use crate::{println};
extern crate spin;
use lazy_static::lazy_static;
use hashbrown::HashMap;

lazy_static!{
    static ref DATABASE: spin::Mutex<HashMap<i32, char>> = spin::Mutex::new(HashMap::new());
}



pub fn set(idx: i32, val: char){
    DATABASE.lock().insert(idx, val);
    println!("set {} at key {}", val, idx);
}

pub fn get(idx:i32) -> char{
    let val: char = *DATABASE.lock().get(&idx).unwrap();
    println!("got {} from key {}", val, idx);
    return val;
}
