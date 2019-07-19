#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

use std::io;

/// The `RandomAccess` trait allows for reading from and writing to a
/// randomly accessible storage of bytes.
pub trait RandomAccess {
  /// An error.
  type Error;

  /// Write bytes at an offset to the backend.
  fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), Self::Error>;

  /// Read a sequence of bytes at an offset from the backend.
  fn read(
    &mut self,
    offset: usize,
    length: usize,
  ) -> Result<Vec<u8>, Self::Error>;

  /// Read a sequence of bytes at an offset from the backend.
  fn read_to_writer(
    &mut self,
    offset: usize,
    length: usize,
    buf: &mut impl io::Write,
  ) -> Result<(), Self::Error>;

  /// Delete a sequence of bytes at an offset from the backend.
  fn del(&mut self, offset: usize, length: usize) -> Result<(), Self::Error>;

  /// Resize the sequence of bytes, possibly discarding or zero-padding bytes
  /// from the end.
  fn truncate(&mut self, length: usize) -> Result<(), Self::Error>;

  /// Get the size of the storage in bytes.
  fn len(&self) -> Result<usize, Self::Error>;

  /// Whether the storage is empty.
  /// For some storage backends it may be cheaper to calculate whether the
  /// storage is empty than to calculate the length.
  fn is_empty(&mut self) -> Result<bool, Self::Error>;
}
