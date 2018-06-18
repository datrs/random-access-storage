#![cfg_attr(nightly, deny(missing_docs))]
#![cfg_attr(nightly, feature(external_doc))]
#![cfg_attr(nightly, doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

/// Methods that need to be implemented for the `RandomAccess` struct.
pub trait RandomAccessMethods {
  /// An error.
  type Error;

  /// Open the backend.
  fn open(&mut self) -> Result<(), Self::Error>;

  /// Write bytes at an offset to the backend.
  fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), Self::Error>;

  /// Read a sequence of bytes at an offset from the backend.
  fn read(
    &mut self,
    offset: usize,
    length: usize,
  ) -> Result<Vec<u8>, Self::Error>;

  /// Delete a sequence of bytes at an offset from the backend.
  fn del(&mut self, offset: usize, length: usize) -> Result<(), Self::Error>;
}

/// Create a random access instance.
#[derive(Debug)]
pub struct RandomAccess<T> {
  /// Check whether or not the file has been opened.
  pub opened: bool,
  handler: T,
}

impl<T> RandomAccess<T>
where
  T: RandomAccessMethods,
{
  /// Create a new `RandomAccess` instance.
  pub fn new(handler: T) -> RandomAccess<T> {
    Self {
      handler,
      opened: false,
    }
  }

  /// Write bytes at an offset. Calls out to `RandomAccessMethods::write`.
  pub fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), T::Error> {
    if !self.opened {
      T::open(&mut self.handler)?;
      self.opened = true;
    }
    T::write(&mut self.handler, offset, data)
  }

  /// Write bytes from an offset. Calls out to `RandomAccessMethods::read`.
  pub fn read(
    &mut self,
    offset: usize,
    length: usize,
  ) -> Result<Vec<u8>, T::Error> {
    if !self.opened {
      T::open(&mut self.handler)?;
      self.opened = true;
    }
    T::read(&mut self.handler, offset, length)
  }

  /// Delete bytes from an offset. Calls out to `RandomAccessMethods::del`.
  pub fn del(&mut self, offset: usize, length: usize) -> Result<(), T::Error> {
    if !self.opened {
      T::open(&mut self.handler)?;
      self.opened = true;
    }
    T::del(&mut self.handler, offset, length)
  }
}
