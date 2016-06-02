use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::Stdout;
use std::process::Command;
use std::fmt;

pub fn print_screen<T: fmt::Display>(state: &T, generation: u32) {
    let mut writer = BufWriter::new(io::stdout());
    clear_screen(&mut writer);
    writeln!(writer, "Generation: {}", generation).unwrap();
    write!(writer, "{}", state).unwrap();
}

fn clear_screen(writer: &mut BufWriter<Stdout>) {
    let output = Command::new("clear").output().unwrap();
    write!(writer, "{}", String::from_utf8_lossy(&output.stdout)).unwrap();
}

