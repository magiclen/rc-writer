//! A tiny implement for writing data to a reference counted instance.
//!
//! ## Examples
//!
//! ### RcWriter
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
//!
//! ### RcOptionWriter
//!
//! ```
//! extern crate rc_writer;
//!
//! use rc_writer::RcOptionWriter;
//!
//! use std::rc::Rc;
//!
//! use std::cell::RefCell;
//!
//! use std::io::Write;
//!
//! let data = RefCell::new(Some(Vec::new()));
//!
//! let data_rc = Rc::new(data);
//!
//! let mut writer = RcOptionWriter::new(data_rc.clone());
//!
//! writer.write(b"Hello world!").unwrap();
//!
//! writer.flush().unwrap();
//!
//! let data = data_rc.borrow_mut().take().unwrap(); // remove out the vec from rc
//!
//! assert_eq!(b"Hello world!".to_vec(), data);
//! ```

use std::rc::Rc;
use std::cell::RefCell;

use std::io::{self, Write, ErrorKind};

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

pub struct RcOptionWriter<W: Write> {
    inner: Rc<RefCell<Option<W>>>
}

impl<W: Write> RcOptionWriter<W> {
    pub fn new(writer: Rc<RefCell<Option<W>>>) -> RcOptionWriter<W> {
        RcOptionWriter {
            inner: writer
        }
    }
}

impl<W: Write> Write for RcOptionWriter<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        match self.inner.borrow_mut().as_mut() {
            Some(writer) => writer.write(buf),
            None => Err(io::Error::new(ErrorKind::BrokenPipe, "the writer has been removed out"))
        }
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        match self.inner.borrow_mut().as_mut() {
            Some(writer) => writer.flush(),
            None => Err(io::Error::new(ErrorKind::BrokenPipe, "the writer has been removed out"))
        }
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
}
