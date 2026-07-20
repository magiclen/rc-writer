use std::{cell::RefCell, io::Write, rc::Rc};

use rc_writer::RcWriter;

#[test]
fn to_vec() {
    let data = RefCell::new(Vec::new());

    let data_rc = Rc::new(data);

    let mut writer = RcWriter::new(data_rc.clone());

    writer.write_all(b"Hello world!").unwrap();

    writer.flush().unwrap();

    assert_eq!(b"Hello world!".to_vec(), *data_rc.borrow());
}

#[test]
fn write_fmt_to_vec() {
    let data_rc = Rc::new(RefCell::new(Vec::new()));

    let mut writer = RcWriter::new(data_rc.clone());

    let hello = "Hello";
    let world = "world!";

    write!(writer, "{hello} {world}").unwrap();

    assert_eq!(b"Hello world!".to_vec(), *data_rc.borrow());
}

#[test]
fn to_unsized_vec() {
    let data_rc: Rc<RefCell<dyn Write>> = Rc::new(RefCell::new(Vec::new()));

    let mut writer = RcWriter::new(data_rc);

    writer.write_all(b"Hello world!").unwrap();

    writer.flush().unwrap();
}

#[test]
fn try_into_writer() {
    let data_rc = Rc::new(RefCell::new(Vec::new()));

    let mut writer = RcWriter::new(data_rc.clone());

    writer.write_all(b"Hello world!").unwrap();

    drop(data_rc);

    assert_eq!(b"Hello world!".to_vec(), writer.try_into_writer().unwrap());
}
