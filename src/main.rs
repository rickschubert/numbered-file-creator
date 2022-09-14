use regex::Regex;
use std::fs::{self, DirEntry};
use std::fs::{File, ReadDir};
use std::path::Path;
use std::process::exit;
use lazy_static::lazy_static;
use std::{env};

lazy_static! {
    static ref NUMBER_ONLY_REGEX_IN_PATH: Regex = Regex::new(r"/?(\d+)_").unwrap();
    static ref NUMBLER_ONLY_REGEX_IN_FILE_NAME: Regex = Regex::new(r"^(\d+)_").unwrap();
}

fn get_leading_number_from_file(file_name: &str) -> &str {
    let number = NUMBER_ONLY_REGEX_IN_PATH
        .captures(file_name)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    return number;
}

fn get_names_from_dir_entry_results_vector(items_that_need_renaming: Vec<Result<DirEntry, std::io::Error>>) -> Vec<String> {
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

fn filter_for_files_to_be_renamed(inner: ReadDir, number: &str) -> Vec<String> {
    let to_be_renamed: Vec<Result<DirEntry, std::io::Error>> = inner
        .filter(|x| match x {
            Err(_) => false,
            Ok(dir_entry) => {
                let n = dir_entry.file_name();

                let captured = NUMBLER_ONLY_REGEX_IN_FILE_NAME.captures(&n.to_str().unwrap());
                match captured {
                    Some(_) => (),
                    None => return false,
                }

                let lead = get_leading_number_from_file(n.to_str().unwrap());
                let lead_i: &i32 = &lead.parse().unwrap();
                return lead.eq(number)
                    || lead_i > &number.parse::<i32>().unwrap();
            }
        })
        .into_iter()
        .collect();

    let mut names = get_names_from_dir_entry_results_vector(to_be_renamed);
    alphanumeric_sort::sort_str_slice_rev(&mut names);
    return names;
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
    let lead_of_new_file = get_leading_number_from_file(file_name);

    let dir = directory.to_str().expect("there was no string");
    let dir_relative = format!("./{dir}");
    let scenes_directory = Path::new(&dir_relative);
    let paths = fs::read_dir(scenes_directory);

    match paths {
        Ok(inner) => {
            let items_to_be_renamed = filter_for_files_to_be_renamed(inner, &lead_of_new_file);

            // For each item that needs renaming, increase the number indicator
            items_to_be_renamed.into_iter().for_each(|path| {
                let lead = get_leading_number_from_file(&path);
                let lead_as_int: &i32 = &lead.parse().unwrap();

                let new_lead = generate_new_lead(lead_as_int);
                let new_file_name = get_new_file_name(&path, &new_lead);

                rename_file(&path, &new_file_name);
            });

            create_new_file(&new_scene_path);
        }
        Err(error) => panic!("Problem reading the directory: {:?}", error),
    }
}

fn get_new_file_name(pathstring: &str, new_lead: &str) -> String {
    return NUMBER_ONLY_REGEX_IN_PATH.replace(pathstring, new_lead).to_string();
}

fn generate_new_lead(lead_as_int: &i32) -> String {
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
        Ok(_) => println!(
            "Renamed file from from {} to {}",
            pathstring, new_file_name
        ),
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
