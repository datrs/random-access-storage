#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RandomAccessError {
  #[error("{} out of bounds. {} < {}{}",
          .end.as_ref().map_or_else(|| "Offset", |_| "Range"),
          .length,
          .offset,
          .end.as_ref().map_or_else(String::new, |end| format!("..{}", end)))]
  OutOfBounds {
    offset: u64,
    end: Option<u64>,
    length: u64,
  },
  #[error("Unrecoverable input/output error occured.{}{}",
          .return_code.as_ref().map_or_else(String::new, |rc| format!(" Return code: {}.", rc)),
          .context.as_ref().map_or_else(String::new, |ctx| format!(" Context: {}.", ctx)))]
  IO {
    return_code: Option<i32>,
    context: Option<String>,
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

/// The `RandomAccess` trait allows for reading from and writing to a
/// randomly accessible storage of bytes.
#[async_trait::async_trait]
pub trait RandomAccess {
  /// Write bytes at an offset to the backend.
  async fn write(
    &mut self,
    offset: u64,
    data: &[u8],
  ) -> Result<(), RandomAccessError>;

  /// Read a sequence of bytes at an offset from the backend.
  async fn read(
    &mut self,
    offset: u64,
    length: u64,
  ) -> Result<Vec<u8>, RandomAccessError>;

  /// Delete a sequence of bytes at an offset from the backend.
  async fn del(
    &mut self,
    offset: u64,
    length: u64,
  ) -> Result<(), RandomAccessError>;

  /// Resize the sequence of bytes, possibly discarding or zero-padding bytes
  /// from the end.
  async fn truncate(&mut self, length: u64) -> Result<(), RandomAccessError>;

  /// Get the size of the storage in bytes.
  async fn len(&mut self) -> Result<u64, RandomAccessError>;

  /// Whether the storage is empty.
  /// For some storage backends it may be cheaper to calculate whether the
  /// storage is empty than to calculate the length.
  async fn is_empty(&mut self) -> Result<bool, RandomAccessError>;

  /// Flush buffered data on the underlying storage resource.
  async fn sync_all(&mut self) -> Result<(), RandomAccessError>;
}
