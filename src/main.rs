use std::fs::File;
use std::env;
use regex::Regex;
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
    let file_stem = path.file_stem().unwrap().to_str().unwrap();
    let number_only_regex = Regex::new(r"^(\d+)_").unwrap();
    let number = number_only_regex.captures(file_stem).unwrap().get(1).unwrap().as_str();
    dbg!(number);


    let dir_as_ref = directory.to_str().expect("there was no string");
    // let current_dir = env::current_dir().unwrap();

    let dir_as_ref_with_point_at_start = format!("./{dir_as_ref}");
    let scenes_directory = Path::new(&dir_as_ref_with_point_at_start);
    dbg!(scenes_directory);
    let paths = fs::read_dir(scenes_directory).unwrap();
    // TODO: The below filter will work, just need to line it up
    // paths.filter(|x| x.unwrap().file_name());
    // for path in paths {
    //     let filename = path.unwrap().file_name();
    //     dbg!(filename);
    // }

}
