//! A tiny implement for writing data to a reference counted instance.
//!
//! ## Example
//!
//! ```
//! extern crate rc_writer;
//!
//! use rc_writer::RcWriter;
//!
//! use std::rc::Rc;
//!
//! use std::cell::RefCell;
//!
//! use std::io::Write;
//!
//! let data = RefCell::new(Vec::new());
//!
//! let data_rc = Rc::new(data);
//!
//! let mut writer = RcWriter::new(data_rc.clone());
//!
//! writer.write(b"Hello world!").unwrap();
//!
//! writer.flush().unwrap();
//!
//! assert_eq!(b"Hello world!".to_vec(), *data_rc.borrow());
//! ```

use std::rc::Rc;
use std::cell::RefCell;

use std::io::{self, Write};

pub struct RcWriter<W: Write> {
    inner: Rc<RefCell<W>>
}

impl<W: Write> RcWriter<W> {
    pub fn new(writer: Rc<RefCell<W>>) -> RcWriter<W> {
        RcWriter {
            inner: writer
        }
    }
}

impl<W: Write> Write for RcWriter<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        self.inner.borrow_mut().write(buf)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.inner.borrow_mut().flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_to_vec() {
        let data = RefCell::new(Vec::new());

        let data_rc = Rc::new(data);

        let mut writer = RcWriter::new(data_rc.clone());

        writer.write(b"Hello world!").unwrap();

        writer.flush().unwrap();

        assert_eq!(b"Hello world!".to_vec(), *data_rc.borrow());
    }
}
