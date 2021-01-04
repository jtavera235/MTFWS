use std::thread;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::path::Path;
use std::collections::HashMap;

fn read_file_contents(file: &mut File, buffer: &mut String) {
    file.read_to_string(buffer);
}

fn get_file_words(filename: &String) -> Vec<String> {
    let inp_file_path = Path::new(filename);
    let mut word_file = match File::open(&inp_file_path) {
        Ok(file) => file,
        Err(_e) => panic!("couldn't open the passed input file"),
    };
    let mut buffer = String::new();
    read_file_contents(&mut word_file, &mut buffer);
    let mut buffer_vector: Vec<&str> = buffer.split('\n').collect();
    buffer_vector.retain(|&x| x != "");
    let buffer_vector_as_string: Vec<String> = buffer_vector.iter().map(|x| x.to_string()).collect();
    buffer_vector_as_string
}

fn create_output_file(filename: &String) -> File {
    let inp_file_path = Path::new(filename);
    let word_file = match File::create(&inp_file_path) {
        Ok(file) => file,
        Err(_e) => panic!("couldn't open the passed input file"),
    };
    word_file
}

fn init_map(original_list: &Vec<String>) -> HashMap<String, i32> {
    let mut map: HashMap<String, i32> = HashMap::new();
    for i in 0..original_list.len() {
        let word = match original_list.get(i) {
            Some(w) => w,
            None => panic!("Index for word list is out of bounds"),
        };
        map.entry(word.clone()).or_insert(0);
    }
    map
}


fn pop_map(file_list: &Vec<Vec<String>>, map: &mut HashMap<String, i32>) {
    for i in 0..file_list.len() {
        let file_words = match file_list.get(i) {
            Some(w) => w,
            None => panic!("Index for word list is out of bounds"),
        };
        for n in 0..file_words.len() {
            let word = match file_words.get(n as usize) {
                Some(w) => w,
                None => panic!("Index for word is out of bounds"),
            };
            let mut value = match map.get(&word.to_string()) {
              Some(v) => *v,
              None => -1,
            };
            if value != -1 {
                value = value + 1;    
                map.insert(word.clone(), value);
            }
        }
    }
}


fn map_to_string(map: &HashMap<String, i32>) -> String {
    let mut map_string_rep = String::new();
    for n in map.keys() {
        map_string_rep.push_str(n.as_str());
        map_string_rep.push_str(": ");
        let value = match map.get(n) {
            Some(s) => *s,
            None => panic!("Something went wrong"),
        };
        let str_val = value.to_string();
        map_string_rep.push_str(str_val.as_str());
        map_string_rep.push_str("\n");
    }
    map_string_rep
}

fn write_to_output(string: &String, file: &mut File) {
    file.write_all(string.as_bytes()).expect("unable to write to file");
}

fn main() {

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        std::process::exit(1);
    }
    
    // gets the input file name
    let input_file = match args.get(1) {
        Some(x) => x,
        None => std::process::exit(1),
    };
    // creates a list of the input file words
    let origin_words = get_file_words(input_file);
    // declares the output file and whether it exists or not
    let output_file = String::new();
    let output_file_exists = false;

    // all the thread children
    let mut children = vec![];
    // the words contained in all the passed files which will be compared with origin_words
    let words = Vec::new();
    
    // init all the mutexes
    let out_file_mutex = Mutex::new(output_file.clone());
    let out_f_state_mutex = Mutex::new(output_file_exists);
    let out_file_arc = Arc::new(out_file_mutex);
    let out_f_state_arc = Arc::new(out_f_state_mutex);
    let args_mutex = Mutex::new(args.clone());
    let words_mutex = Mutex::new(words);
    let args_arc = Arc::new(args_mutex);
    let words_arc = Arc::new(words_mutex);

    // spawns new threads
    for n in 2..args.len() {
        let out_file_arc = Arc::clone(&out_file_arc);
        let out_f_state_arc = Arc::clone(&out_f_state_arc);
        let args = Arc::clone(&args_arc);
        let words = Arc::clone(&words_arc);
        children.push(thread::spawn(move || {
            let args = match args.lock() {
                Ok(e) => e,
                Err(_er) => panic!("Command line arguments is invalid."),
            };
            let mut words = words.lock().unwrap();
            let file = match args.get(n as usize) {
                Some(x) => x,
                None => panic!("file was matched to incorrect index"),
            };
            if !file.as_str().contains("-") {
                let file_words = get_file_words(file);
                words.push(file_words);
            } else if output_file_exists == false {
                let mut out_file_guard = out_file_arc.lock().unwrap();
                let mut out_file = file.to_string();
                out_file.retain(|c| c != '-');
                *out_file_guard = out_file;
                let mut out_f_state_guard = out_f_state_arc.lock().unwrap();
                *out_f_state_guard = true;
            } else {
                panic!("Output file already specified");
            }
        }));
    }
    // joins the threads together
    for child in children {
        let _ = child.join();
    }

    // iterate through words
    let mut output_file = create_output_file(&(*out_file_arc.lock().unwrap()));
    let mut map = init_map(&origin_words);
    pop_map(&(*words_arc.lock().unwrap()), &mut map);
    let string_map = map_to_string(&map);
    write_to_output(&string_map, &mut output_file);
    println!("Successfully mapped each iteration of the input.");
}


