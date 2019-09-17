extern crate rc_writer;

use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

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
