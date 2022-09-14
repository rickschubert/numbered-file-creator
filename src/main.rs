use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fs::{self, DirEntry};
use std::fs::{File, ReadDir};
use std::path::Path;
use std::process::exit;

lazy_static! {
    static ref NUMBER_ONLY_REGEX_IN_PATH: Regex = Regex::new(r"/?(\d+)_").unwrap();
    static ref NUMBLER_ONLY_REGEX_IN_FILE_NAME: Regex = Regex::new(r"^(\d+)_").unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Error: You neeed to provide at least one argument, a dynamic path to the new scene file that should be created!");
        exit(0);
    }

    let ref new_scene_path = args[1];

    let path = Path::new(new_scene_path);
    let directory = path.parent().unwrap();
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let lead_of_new_file = leading_number_from_file(file_name);
    let content = get_directory_content(directory);

    match content {
        Ok(content) => {
            rename_necessary_files_and_create_new_one(content, lead_of_new_file, new_scene_path)
        }
        Err(error) => panic!("Problem reading the directory: {:?}", error),
    }
}

fn leading_number_from_file(file_name: &str) -> &str {
    let number = NUMBER_ONLY_REGEX_IN_PATH
        .captures(file_name)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    return number;
}

fn content_paths(items_that_need_renaming: Vec<Result<DirEntry, std::io::Error>>) -> Vec<String> {
    let mut names = Vec::new();

    let mut paths = Vec::new();
    items_that_need_renaming.into_iter().for_each(|x| {
        let p = x.unwrap().path();
        paths.push(p);
    });

    paths.iter().for_each(|x| {
        names.push(x.to_str().unwrap().to_owned());
    });
    return names;
}

fn filter_dir_entries_for_files_to_rename(dir_entry: &DirEntry, number: &str) -> bool {
    let n = dir_entry.file_name();

    let captured = NUMBLER_ONLY_REGEX_IN_FILE_NAME.captures(&n.to_str().unwrap());
    match captured {
        Some(_) => (),
        None => return false,
    }

    let lead = leading_number_from_file(n.to_str().unwrap());
    let lead_i: &i32 = &lead.parse().unwrap();
    return lead.eq(number) || lead_i > &number.parse::<i32>().unwrap();
}

fn filter_content_for_files_to_rename(content: ReadDir, number: &str) -> Vec<String> {
    let to_be_renamed: Vec<Result<DirEntry, std::io::Error>> = content
        .filter(|x| match x {
            Err(_) => false,
            Ok(dir_entry) => filter_dir_entries_for_files_to_rename(dir_entry, number),
        })
        .into_iter()
        .collect();

    let mut names = content_paths(to_be_renamed);
    alphanumeric_sort::sort_str_slice_rev(&mut names);
    return names;
}

fn updated_file_name(path: &str) -> String {
    let lead = leading_number_from_file(&path);
    let lead_as_int: &i32 = &lead.parse().unwrap();
    let new_lead = new_lead(lead_as_int);
    let new_file_name = new_file_name(&path, &new_lead);
    return new_file_name;
}

fn get_directory_content(directory: &Path) -> Result<ReadDir, std::io::Error> {
    let dir = directory.to_str().expect("there was no string");
    let dir_relative = format!("./{dir}");
    let scenes_directory = Path::new(&dir_relative);
    let paths = fs::read_dir(scenes_directory);
    return paths;
}

fn rename_necessary_files_and_create_new_one(
    content: ReadDir,
    lead_of_new_file: &str,
    new_scene_path: &str,
) {
    let items_to_be_renamed = filter_content_for_files_to_rename(content, lead_of_new_file);
    items_to_be_renamed.into_iter().for_each(|path| {
        let new_file_name = updated_file_name(&path);
        rename_file(&path, &new_file_name);
    });
    create_new_file(new_scene_path);
}

fn new_file_name(pathstring: &str, new_lead: &str) -> String {
    return NUMBER_ONLY_REGEX_IN_PATH
        .replace(pathstring, new_lead)
        .to_string();
}

fn new_lead(lead_as_int: &i32) -> String {
    let mut new_lead = String::new();
    if lead_as_int < &9 {
        new_lead.push('0');
    }
    let new_number_as_int = lead_as_int + 1;
    new_lead = format!("/{new_lead}{new_number_as_int}_");
    return new_lead;
}

fn rename_file(pathstring: &str, new_file_name: &str) {
    let result = fs::rename(pathstring, new_file_name);
    match result {
        Ok(_) => println!("Renamed file from from {} to {}", pathstring, new_file_name),
        Err(error) => panic!("Unable to rename file: {}", error),
    }
}

fn create_new_file(new_scene_path: &str) {
    let result = File::create(new_scene_path);
    match result {
        Ok(_) => println!("Created new file {}", new_scene_path),
        Err(error) => panic!("Unable to create new file: {}", error),
    }
}
