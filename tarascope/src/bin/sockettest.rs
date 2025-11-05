use std::{io::pipe, os::fd::AsRawFd};

fn main() {
    let (reader, writer) = pipe().unwrap();
    let writer_fd = writer.as_raw_fd();
    println!("{}", writer_fd);
}