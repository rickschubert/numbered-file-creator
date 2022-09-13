use filters::filter::Filter;
use regex::Regex;
use std::env;
use std::fs::{self, DirEntry};
use std::fs::{File, ReadDir};
use std::path::Path;
use std::process::exit;

fn get_leading_number_from_file(file_name: &str) -> &str {
    // TODO: It would be nice if this wouldn't have to be calculated every time
    let number_only_regex = Regex::new(r"^(\d+)_").unwrap();
    let number = number_only_regex
        .captures(file_name)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    return number;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Error: You neeed to provide at least one argument, a dynamic path to the new scene file that should be created!");
        exit(0);
    }

    let ref new_scene_path = args[1];
    dbg!(new_scene_path);

    let path = Path::new(new_scene_path);
    let directory = path.parent().unwrap();
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let number = get_leading_number_from_file(file_name);
    dbg!(number);

    let dir_as_ref = directory.to_str().expect("there was no string");

    let dir_as_ref_with_point_at_start = format!("./{dir_as_ref}");
    let scenes_directory = Path::new(&dir_as_ref_with_point_at_start);
    dbg!(scenes_directory);
    let paths = fs::read_dir(scenes_directory);

    match paths {
        Ok(inner) => {
            let items_that_need_renaming: Vec<Result<DirEntry, std::io::Error>> = inner
                .filter(|x| match x {
                    Err(_) => false,
                    Ok(dirEntry) => {
                        let n = dirEntry.file_name();
                        let leading_number_from_file = get_leading_number_from_file(n.to_str().unwrap());
                        println!("{}", leading_number_from_file);
                        // return true;
                        let leading_number_as_int: &i32 = &leading_number_from_file.parse().unwrap();
                        return leading_number_from_file != number && leading_number_as_int >= &number.parse::<i32>().unwrap();
                    }
                })
                .into_iter()
                .collect();
            dbg!(items_that_need_renaming);
        }
        Err(error) => panic!("Problem reading the directory: {:?}", error),
    }
}
