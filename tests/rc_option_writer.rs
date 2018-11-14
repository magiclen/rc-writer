extern crate rc_writer;

use std::cell::RefCell;
use std::rc::Rc;
use std::io::Write;

use rc_writer::RcOptionWriter;

#[test]
fn write_to_option_vec() {
    let data = RefCell::new(Some(Vec::new()));

    let data_rc = Rc::new(data);

    let mut writer = RcOptionWriter::new(data_rc.clone());

    writer.write(b"Hello world!").unwrap();

    writer.flush().unwrap();

    let data = data_rc.borrow_mut().take().unwrap(); // remove out the vec from rc

    assert_eq!(b"Hello world!".to_vec(), data);
}