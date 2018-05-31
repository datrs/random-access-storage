#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

extern crate failure;

use failure::Error;

/// Methods that need to be implemented for the `Sync` struct.
pub trait RandomAccessMethods {
  /// Open the backend.
  fn open(&mut self) -> Result<(), Error>;

  /// Write bytes at an offset to the backend.
  fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), Error>;

  /// Read a sequence of bytes at an offset from the backend.
  fn read(&mut self, offset: usize, length: usize) -> Result<Vec<u8>, Error>;

  /// Delete a sequence of bytes at an offset from the backend.
  fn del(&mut self, offset: usize, length: usize) -> Result<(), Error>;
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

  /// Write bytes at an offset. Calls `SyncMethods::write` under the hood.
  pub fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), Error> {
    if !self.opened {
      T::open(&mut self.handler)?;
      self.opened = true;
    }
    T::write(&mut self.handler, offset, data)
  }

  /// Write bytes from an offset. Calls `SyncMethods::read` under the hood.
  pub fn read(
    &mut self,
    offset: usize,
    length: usize,
  ) -> Result<Vec<u8>, Error> {
    if !self.opened {
      T::open(&mut self.handler)?;
      self.opened = true;
    }
    T::read(&mut self.handler, offset, length)
  }

  /// Delete bytes from an offset. Calls `SyncMethods::del` under the hood.
  pub fn del(&mut self, offset: usize, length: usize) -> Result<(), Error> {
    if !self.opened {
      T::open(&mut self.handler)?;
      self.opened = true;
    }
    T::del(&mut self.handler, offset, length)
  }
}
