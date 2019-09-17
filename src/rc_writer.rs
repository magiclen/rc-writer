use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

pub struct RcWriter<W: Write> {
    inner: Rc<RefCell<W>>,
}

impl<W: Write> RcWriter<W> {
    #[inline]
    pub fn new(writer: Rc<RefCell<W>>) -> RcWriter<W> {
        RcWriter {
            inner: writer,
        }
    }
}

impl<W: Write> Write for RcWriter<W> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        self.inner.borrow_mut().write(buf)
    }

    #[inline]
    fn flush(&mut self) -> Result<(), io::Error> {
        self.inner.borrow_mut().flush()
    }
}
