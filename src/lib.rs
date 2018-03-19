#![deny(warnings, missing_docs)]
// #![cfg_attr(test, feature(plugin))]
// #![cfg_attr(test, plugin(clippy))]

//! Abstract interface to implement `random-access-storage` instances.
//!
//! ## Why?
//! This module forms a shared interface for reading and writing bytes to
//! different backends. By having a shared interface, it means implementations
//! can easily be swapped, depending on the environment.
//!
//! ## Usage
//! ```rust,ignore
//! extern crate random_access_storage;
//! use random_access_storage::{Error, Sync, SyncMethods};
//!
//! struct S;
//! impl SyncMethods for S {
//!   fn open(&self) -> Result<(), Error> {
//!     // ...
//!   }
//!   fn write(&self, offset: u64, data: Vec<u8>) -> Result<(), Error> {
//!     // ...
//!   }
//!   fn read(&self, offset: u64, length: u64) -> Result<Vec<u8>, Error> {
//!     // ...
//!   }
//!   fn del(&self, offset: u64, length: u64) -> Result<(), Error> {
//!     // ...
//!   }
//! }
//!
//! let file = Sync::new(SyncMethods);
//! file.write(0, b"hello")?;
//! file.write(0, b" world")?;
//! let text = file.read(0, 11,)?;
//! assert!(text, b"hello world");
//! ```

pub extern crate failure;

pub use failure::Error;

/// Methods that need to be implemented for the `Sync` struct.
pub trait SyncMethods {
  /// Open the backend.
  fn open(&self) -> Result<(), Error>;

  /// Write bytes at an offset to the backend.
  fn write(&self, offset: u64, data: Vec<u8>) -> Result<(), Error>;

  /// Read a sequence of bytes at an offset from the backend.
  fn read(&self, offset: u64, length: u64) -> Result<Vec<u8>, Error>;

  /// Delete a sequence of bytes at an offset from the backend.
  fn del(&self, offset: u64, length: u64) -> Result<(), Error>;
}

/// A synchronous implementation of `random_access_storage`.
#[derive(Debug)]
pub struct Sync<T> {
  handler: T,
  is_open: bool,
}

impl<T> Sync<T>
where
  T: SyncMethods,
{
  /// Create a new `Sync` instance.
  pub fn new(handler: T) -> Sync<T> {
    Sync {
      handler: handler,
      is_open: false,
    }
  }

  /// Write bytes at an offset. Calls `SyncMethods::write` under the hood.
  pub fn write(&mut self, offset: u64, data: Vec<u8>) -> Result<(), Error> {
    if !self.is_open {
      T::open(&self.handler)?;
      self.is_open = true;
    }
    T::write(&self.handler, offset, data)
  }

  /// Write bytes from an offset. Calls `SyncMethods::read` under the hood.
  pub fn read(&mut self, offset: u64, length: u64) -> Result<Vec<u8>, Error> {
    if !self.is_open {
      T::open(&self.handler)?;
      self.is_open = true;
    }
    T::read(&self.handler, offset, length)
  }

  /// Delete bytes from an offset. Calls `SyncMethods::del` under the hood.
  pub fn del(&mut self, offset: u64, length: u64) -> Result<(), Error> {
    if !self.is_open {
      T::open(&self.handler)?;
      self.is_open = true;
    }
    T::del(&self.handler, offset, length)
  }
}
