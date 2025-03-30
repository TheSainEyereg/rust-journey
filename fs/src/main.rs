use std::{
    env, fs,
    io::{BufRead, BufReader, Read},
    path,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let file_path = path::Path::new(&args[1]);

    let bytes = fs::read(file_path).expect("Error reading file");
    println!("File size is {} bytes", bytes.len());

    match String::from_utf8(bytes) {
        Ok(content) => println!("Content is:\n{content}"),
        Err(_) => println!("File is not valid UTF-8"),
    }

    let mut stream = fs::File::open(file_path).expect("Error opening file");

    let mut buffer: [u8; 10] = [0; 10];
    stream.read(&mut buffer).expect("Error reading file");

    println!(
        "buffer.to_vec(): {}\nVec::from(buffer): {}",
        String::from_utf8(buffer.to_vec()).unwrap(),
        String::from_utf8(Vec::from(buffer)).unwrap()
    );

    let reader = BufReader::new(&stream);
    let mut string = reader.lines().next().unwrap().unwrap().to_lowercase();

    println!("String: {string}");

    let mut content = fs::read_to_string(file_path).expect("Error reading file");
    let slice = content.as_mut_str();
    slice.make_ascii_uppercase();

    string.push_str(slice);

    fs::write(file_path, string).expect("Error writing file");
}
