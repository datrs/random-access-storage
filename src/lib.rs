#![deny(missing_docs)]
#![feature(external_doc)]
#![doc(include = "../README.md")]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

extern crate failure;

pub use failure::Error;

/// Methods that need to be implemented for the `Sync` struct.
pub trait SyncMethods {
  /// Open the backend.
  fn open(&mut self) -> Result<(), Error>;

  /// Write bytes at an offset to the backend.
  fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), Error>;

  /// Read a sequence of bytes at an offset from the backend.
  fn read(&mut self, offset: usize, length: usize) -> Result<Vec<u8>, Error>;

  /// Delete a sequence of bytes at an offset from the backend.
  fn del(&mut self, offset: usize, length: usize) -> Result<(), Error>;

  // fn close ();
  // fn destroy ();
  // fn stat ();
}

/// Create a synchronous instance.
#[derive(Debug)]
pub struct Sync<T> {
  /// Check whether or not the file has been opened.
  pub opened: bool,
  handler: T,
}

impl<T> Sync<T>
where
  T: SyncMethods,
{
  /// Create a new `Sync` instance.
  pub fn new(handler: T) -> Sync<T> {
    Sync {
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

  // pub fn close () {}
  // pub fn destroy () {}
  // pub fn stat () {}
}
