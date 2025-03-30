use notify::{Event, EventKind, RecursiveMode, Result, Watcher, recommended_watcher};
use std::{env, path, sync::mpsc::channel};

fn main() {
    let path_string = env::args().nth(1).expect("Please supply file path");

    let file_path = path::Path::new(&path_string);

    let (sender, receiver) = channel::<Result<Event>>();

    let mut watcher = recommended_watcher(sender).unwrap();

    watcher.watch(file_path, RecursiveMode::Recursive).unwrap();

    loop {
        match receiver.recv() {
            Ok(event) => {
                if let Ok(e) = event {
                    match e.kind {
                        EventKind::Create(_) => println!("File created"),
                        EventKind::Modify(_) => println!("File modified"),
                        EventKind::Remove(_) => println!("File removed"),
                        _ => (),
                    }
                }
            }
            Err(e) => println!("watch error: {}", e),
        }
    }
}
