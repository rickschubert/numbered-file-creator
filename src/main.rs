use std::fs::File;
use std::env;
use std::process::exit;
use std::path::Path;
use std::fs::{self, DirEntry};


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Error: You neeed to provide at least one argument, a dynamic path to the new scene file that should be created!");
        exit(0);
    }

    let ref new_scene_path = args[1];
    dbg!(new_scene_path);
    // let file = File::create(new_scene_path).expect("Error encountered while creating file!");
    // dbg!(file);

    let path = Path::new(new_scene_path);
    let directory = path.parent().unwrap();

    for entry in fs::read_dir(directory) {
        dbg!(entry);
    }

}
