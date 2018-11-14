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