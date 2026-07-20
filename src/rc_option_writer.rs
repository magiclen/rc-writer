use std::{
    cell::RefCell,
    fmt,
    io::{self, ErrorKind, IoSlice, Write},
    rc::Rc,
};

/// A writer which writes data into a reference counted writer that can be taken out.
///
/// Every write operation borrows the inner `RefCell` mutably, so it panics if the inner writer is already borrowed.
/// Unlike [`RcWriter`](crate::RcWriter), the generic type cannot be unsized because it is stored in an `Option`.
#[derive(Debug)]
pub struct RcOptionWriter<W> {
    inner: Rc<RefCell<Option<W>>>,
}

/// The error returned by every write operation after the writer has been taken out.
#[inline]
fn writer_removed() -> io::Error {
    io::Error::new(ErrorKind::BrokenPipe, "the writer has been removed out")
}

impl<W> Clone for RcOptionWriter<W> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone()
        }
    }
}

impl<W> From<Rc<RefCell<Option<W>>>> for RcOptionWriter<W> {
    #[inline]
    fn from(writer: Rc<RefCell<Option<W>>>) -> Self {
        Self::new(writer)
    }
}

impl<W> RcOptionWriter<W> {
    /// Create a new `RcOptionWriter` which writes data into the given reference counted writer.
    #[inline]
    pub const fn new(writer: Rc<RefCell<Option<W>>>) -> Self {
        Self {
            inner: writer
        }
    }

    /// Get a reference to the inner reference counted writer.
    #[inline]
    pub const fn get_ref(&self) -> &Rc<RefCell<Option<W>>> {
        &self.inner
    }

    /// Unwrap this `RcOptionWriter`, returning the inner reference counted writer.
    #[inline]
    pub fn into_inner(self) -> Rc<RefCell<Option<W>>> {
        self.inner
    }

    /// Take the writer out of the `Rc`, or return `None` if there are still other references to it or it has already been taken out.
    #[inline]
    pub fn try_into_writer(self) -> Option<W> {
        Rc::into_inner(self.inner).and_then(RefCell::into_inner)
    }
}

impl<W: Write> Write for RcOptionWriter<W> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.inner.borrow_mut().as_mut() {
            Some(writer) => writer.write(buf),
            None => Err(writer_removed()),
        }
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        match self.inner.borrow_mut().as_mut() {
            Some(writer) => writer.write_vectored(bufs),
            None => Err(writer_removed()),
        }
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        match self.inner.borrow_mut().as_mut() {
            Some(writer) => writer.flush(),
            None => Err(writer_removed()),
        }
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        match self.inner.borrow_mut().as_mut() {
            Some(writer) => writer.write_all(buf),
            None => Err(writer_removed()),
        }
    }

    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        match self.inner.borrow_mut().as_mut() {
            Some(writer) => writer.write_fmt(fmt),
            None => Err(writer_removed()),
        }
    }
}
