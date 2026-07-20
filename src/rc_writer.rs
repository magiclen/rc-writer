use std::{
    cell::RefCell,
    fmt,
    io::{self, IoSlice, Write},
    rc::Rc,
};

/// A writer which writes data into a reference counted writer.
///
/// Every write operation borrows the inner `RefCell` mutably, so it panics if the inner writer is already borrowed.
#[derive(Debug)]
pub struct RcWriter<W: ?Sized> {
    inner: Rc<RefCell<W>>,
}

impl<W: ?Sized> Clone for RcWriter<W> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone()
        }
    }
}

impl<W: ?Sized> From<Rc<RefCell<W>>> for RcWriter<W> {
    #[inline]
    fn from(writer: Rc<RefCell<W>>) -> Self {
        Self::new(writer)
    }
}

impl<W: ?Sized> RcWriter<W> {
    /// Create a new `RcWriter` which writes data into the given reference counted writer.
    #[inline]
    pub const fn new(writer: Rc<RefCell<W>>) -> Self {
        Self {
            inner: writer
        }
    }

    /// Get a reference to the inner reference counted writer.
    #[inline]
    pub const fn get_ref(&self) -> &Rc<RefCell<W>> {
        &self.inner
    }

    /// Unwrap this `RcWriter`, returning the inner reference counted writer.
    #[inline]
    pub fn into_inner(self) -> Rc<RefCell<W>> {
        self.inner
    }
}

impl<W> RcWriter<W> {
    /// Take the writer out of the `Rc`, or return `None` if there are still other references to it.
    #[inline]
    pub fn try_into_writer(self) -> Option<W> {
        Rc::into_inner(self.inner).map(RefCell::into_inner)
    }
}

impl<W: Write + ?Sized> Write for RcWriter<W> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.borrow_mut().write(buf)
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        self.inner.borrow_mut().write_vectored(bufs)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.inner.borrow_mut().flush()
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.inner.borrow_mut().write_all(buf)
    }

    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        self.inner.borrow_mut().write_fmt(fmt)
    }
}
