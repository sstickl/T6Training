// LIBRARIES
//use std::env;
//use std::fs;

// MODS
mod boldface;

fn main() {
    let boldfaceops: Vec<Vec<String>> = boldface::init_boldface_db();

    for emergencyop in boldfaceops.iter() {
        println!("{}", emergencyop[0]);
        
        for step in emergencyop.iter().skip(1) {
            println!("\t{}", step);
        }
    }
}