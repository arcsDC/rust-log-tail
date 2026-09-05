use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom};
use std::process;
use std::time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: rust-log-tail <file> [lines] [--follow]");
        process::exit(1);
    }

    let path = &args[1];
    let mut lines = 10;
    let mut follow = false;

    let mut i = 2;
    while i < args.len() {
        if args[i] == "--follow" {
            follow = true;
        } else if let Ok(n) = args[i].parse::<usize>() {
            lines = n;
        }
        i += 1;
    }

    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening {}: {}", path, e);
            process::exit(1);
        }
    };

    let mut reader = BufReader::new(file);
    let mut all_lines: Vec<String> = Vec::new();
    let mut buf = String::new();

    if let Err(e) = reader.read_to_string(&mut buf) {
        eprintln!("Error reading file: {}", e);
        process::exit(1);
    }

    for line in buf.lines() {
        all_lines.push(line.to_string());
    }

    let start = if all_lines.len() > lines {
        all_lines.len() - lines
    } else {
        0
    };

    for line in &all_lines[start..] {
        println!("{}", line);
    }

    if follow {
        let mut file = OpenOptions::new().read(true).open(path).unwrap();
        let mut reader = BufReader::new(file);
        let mut pos = reader.seek(SeekFrom::End(0)).unwrap();

        loop {
            std::thread::sleep(Duration::from_millis(200));
            let current = reader.seek(SeekFrom::Start(0)).unwrap();
            if current > pos {
                reader.seek(SeekFrom::Start(pos)).unwrap();
                let mut line = String::new();
                while reader.read_line(&mut line).unwrap() > 0 {
                    if !line.is_empty() {
                        println!("{}", line.trim_end());
                    }
                    line.clear();
                }
                pos = reader.seek(SeekFrom::Start(0)).unwrap();
            }
        }
    }
}
