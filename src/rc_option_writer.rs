use std::rc::Rc;
use std::cell::RefCell;
use std::io::{self, Write, ErrorKind};

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