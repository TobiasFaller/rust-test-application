extern crate zip;

use std::string::String;
use std::io::Read;

fn main() {
    let message_data = include_bytes!("message.zip");
    let message = extract_message(message_data).unwrap_or("Could not load data!".to_owned());
    println!("{}", message);
}

fn extract_message(data: &[u8]) -> Option<String> {
    let reader = std::io::Cursor::new(data);
    let mut zip = match zip::ZipArchive::new(reader) {
        Err(_) => return None,
        Ok(zip) => zip
    };

    zip.by_name("message.txt").ok().and_then(|mut file| {
        let mut vec = Vec::<u8>::new();
        match file.read_to_end(&mut vec) {
            Ok(_length) => String::from_utf8(vec).ok(),
            Err(_) => None
        }
    })
}
