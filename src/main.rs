extern crate bit_vec;

use std::env;
use std::fs::File; 
use std::io;
use std::path::Path;

mod mcu;

fn brick_check(path: &Path) {
    let f = File::open(path);
    match f {
        Ok(mut fv) => {
            let fc = mcu::K20::from_file(&mut fv);
            match fc {
                Ok(cv) => {
                    if !cv.meen() && cv.sec() {
                        println!("WARNING: This firmware will brick your K20 when flashed!!!");
                    } else {
                        println!("This firmware is as safe as a kitten.");
                    }
                },
                Err(e) => print_error(path, e)
            }
        },
        Err(e) => print_error(path, e)
    };
} 

fn print_usage() {
    println!("Usage: brickcheck FILE");
}

fn print_error(path: &Path, error: io::Error) {
    println!("brickcheck: {}: {}", path.display(), error)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => brick_check(Path::new(&args[1])),
        _ => print_usage(),
    }
}
