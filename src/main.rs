use std::{io::{Seek, Write}, fs::File, thread, time::Duration};
use chrono::Local;

fn write_file(file: &mut File, text: &str) {
    match file.set_len(0) {
        Ok(_) => (),
        Err(_) => eprintln!("warning: failed to clear file"),
    }
    match file.seek(std::io::SeekFrom::Start(0)) {
        Ok(_) => (),
        Err(_) => eprintln!("warning: failed to seek file"),
    }
    match file.write_all(text.as_bytes()) {
        Ok(_) => (),
        Err(_) => eprintln!("warning: failed to write file"),
    }
}

fn main() {
    let mut file = match File::create("cnt.txt") {
        Ok(n) => n,
        Err(_) => panic!("failed to open ./cnt.txt"),
    };

    let wait_duration = Duration::from_millis(500);
    loop {
        let datetime = Local::now().format("%H:%M:%S").to_string();
        write_file(&mut file, &datetime);
        println!("{}", datetime);
        thread::sleep(wait_duration);
    }
}
