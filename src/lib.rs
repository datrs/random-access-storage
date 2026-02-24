//! # Abstract interface to implement random-access instances
//!
//! This crate defines the shared [RandomAccess] trait that makes it possible to create
//! different backends for reading, writing and deleting bytes. With a shared interface,
//! implementations can easily be swapped, depending on the needs and the environment.
//!
//! ## Known Implementations
//!
//! Full implementations of [RandomAccess] include:
//!
//! * [random-access-memory](https://docs.rs/random-access-memory) for in-memory storage
//! * [random-access-disk](https://docs.rs/random-access-disk) for disk storage
//!
//! ## Examples
//!
//! Your own random-access backend can be implemented like this:
//!
//! ```
//! use random_access_storage::{RandomAccess, RandomAccessError, BoxFuture};
//!
//! struct MyRandomAccess {
//!   // Add fields here
//! }
//!
//! impl RandomAccess for MyRandomAccess {
//!   fn write(&self, _offset: u64, _data: &[u8]) -> BoxFuture<Result<(), RandomAccessError>> {
//!     unimplemented!();
//!   }
//!
//!   fn read(&self, _offset: u64, _length: u64) -> BoxFuture<Result<Vec<u8>, RandomAccessError>> {
//!     unimplemented!();
//!   }
//!
//!   fn del(&self, _offset: u64, _length: u64) -> BoxFuture<Result<(), RandomAccessError>> {
//!     unimplemented!();
//!   }
//!
//!   fn truncate(&self, _length: u64) -> BoxFuture<Result<(), RandomAccessError>> {
//!     unimplemented!();
//!   }
//!
//!   fn len(&self) -> u64 {
//!     unimplemented!();
//!   }
//! }
//! ```
use std::{future::Future, pin::Pin};

use thiserror::Error;

/// Convenience alias for the owned, sendable futures returned by [`RandomAccess::read`].
pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

/// Error type for the [RandomAccess] trait methods.
#[derive(Error, Debug)]
pub enum RandomAccessError {
  /// Given parameters are out of bounds.
  #[error("{} out of bounds. {} < {}{}",
          .end.as_ref().map_or_else(|| "Offset", |_| "Range"),
          .length,
          .offset,
          .end.as_ref().map_or_else(String::new, |end| format!("..{}", end)))]
  OutOfBounds {
    /// Offset that was out of bounds
    offset: u64,
    /// If it was a range that was out of bounds, the end of the range.
    end: Option<u64>,
    /// The length in the implementation that was exceeded.
    length: u64,
  },
  /// Unexpected [std::io::Error].
  #[error("Unrecoverable input/output error occured.{}{}",
          .return_code.as_ref().map_or_else(String::new, |rc| format!(" Return code: {}.", rc)),
          .context.as_ref().map_or_else(String::new, |ctx| format!(" Context: {}.", ctx)))]
  IO {
    /// Optional system return code that caused the error.
    return_code: Option<i32>,
    /// Optional context of the error.
    context: Option<String>,
    /// Source of the error.
    #[source]
    source: std::io::Error,
  },
}

impl From<std::io::Error> for RandomAccessError {
  fn from(err: std::io::Error) -> Self {
    Self::IO {
      return_code: None,
      context: None,
      source: err,
    }
  }
}

/// Interface for reading from, writing to and deleting from a
/// randomly accessible storage of bytes.
pub trait RandomAccess {
  /// Write bytes of `data` at an `offset` to the backend.
  ///
  /// # Errors
  ///
  /// * [RandomAccessError::OutOfBounds] if the backend has
  /// a maximum capacity that would be exceeded by the write.
  ///
  /// * [RandomAccessError::IO] if an unexpected IO error occurred.
  fn write(&self, offset: u64, data: &[u8]) -> BoxFuture<Result<(), RandomAccessError>>;

  /// Read a sequence of bytes at an `offset` from the backend.
  ///
  /// # Errors
  ///
  /// * [RandomAccessError::OutOfBounds] if
  /// [RandomAccess::len] > `offset` + `length`.
  ///
  /// * [RandomAccessError::IO] if an unexpected IO error occurred.
  fn read(
    &self,
    offset: u64,
    length: u64,
  ) -> BoxFuture<Result<Vec<u8>, RandomAccessError>>;

  /// Delete a sequence of bytes of given `length` at an `offset` from the backend.
  /// This either sets the bytes in the given slice to zeroes, or if
  /// `offset` + `length` >= [RandomAccess::len()] is the same as
  /// `truncate(offset)`.
  ///
  /// # Errors
  ///
  /// * [RandomAccessError::OutOfBounds] if [RandomAccess::len()] < `offset` + `length`.
  ///
  /// * [RandomAccessError::IO] if an unexpected IO error occurred.
  fn del(&self, offset: u64, length: u64) -> BoxFuture<Result<(), RandomAccessError>>;

  /// Resize the sequence of bytes so that [RandomAccess::len()] is set to
  /// `length`. If `length` < [RandomAccess::len()], the bytes are disregarded.
  /// If `length` > [RandomAccess::len()], the storage is zero-padded.
  ///
  /// # Errors
  ///
  /// * [RandomAccessError::OutOfBounds] if the backend has
  /// a maximum capacity smaller than `length`.
  ///
  /// * [RandomAccessError::IO] if an unexpected IO error occurred.
  fn truncate(&self, length: u64) -> BoxFuture<Result<(), RandomAccessError>>;

  /// Get the size of the storage in bytes.
  fn len(&self) -> u64;

  /// Whether the storage is empty.
  fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Flush buffered data on the underlying storage resource.
  ///
  /// # Errors
  ///
  /// * [RandomAccessError::IO] if an unexpected IO error occurred.
  fn sync_all(&self) -> BoxFuture<Result<(), RandomAccessError>> {
    Box::pin(std::future::ready(Ok(())))
  }
}
