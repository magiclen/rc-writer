use std::{
    cell::RefCell,
    io::{ErrorKind, Write},
    rc::Rc,
};

use rc_writer::RcOptionWriter;

#[test]
fn write_to_option_vec() {
    let data = RefCell::new(Some(Vec::new()));

    let data_rc = Rc::new(data);

    let mut writer = RcOptionWriter::new(data_rc.clone());

    writer.write_all(b"Hello world!").unwrap();

    writer.flush().unwrap();

    let data = data_rc.borrow_mut().take().unwrap(); // remove out the vec from rc

    assert_eq!(b"Hello world!".to_vec(), data);
}

#[test]
fn write_fmt_to_option_vec() {
    let data_rc = Rc::new(RefCell::new(Some(Vec::new())));

    let mut writer = RcOptionWriter::new(data_rc.clone());

    let hello = "Hello";
    let world = "world!";

    write!(writer, "{hello} {world}").unwrap();

    let data = data_rc.borrow_mut().take().unwrap();

    assert_eq!(b"Hello world!".to_vec(), data);
}

#[test]
fn write_after_the_writer_is_removed_out() {
    let data_rc = Rc::new(RefCell::new(Some(Vec::new())));

    let mut writer = RcOptionWriter::new(data_rc.clone());

    data_rc.borrow_mut().take().unwrap();

    assert_eq!(ErrorKind::BrokenPipe, writer.write(b"Hello world!").unwrap_err().kind());

    assert_eq!(ErrorKind::BrokenPipe, writer.write_all(b"Hello world!").unwrap_err().kind());

    assert_eq!(ErrorKind::BrokenPipe, writer.flush().unwrap_err().kind());
}
