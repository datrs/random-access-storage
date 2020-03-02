#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

/// The `RandomAccess` trait allows for reading from and writing to a
/// randomly accessible storage of bytes.
#[async_trait::async_trait]
pub trait RandomAccess {
  /// An error.
  type Error;

  /// Write bytes at an offset to the backend.
  async fn write(
    &mut self,
    offset: u64,
    data: &[u8],
  ) -> Result<(), Self::Error>;

  /// Read a sequence of bytes at an offset from the backend.
  async fn read(
    &mut self,
    offset: u64,
    length: u64,
  ) -> Result<Vec<u8>, Self::Error>;

  /// Read a sequence of bytes at an offset from the backend.
  async fn read_to_writer(
    &mut self,
    offset: u64,
    length: u64,
    buf: &mut (impl futures_io::AsyncWrite + Send),
  ) -> Result<(), Self::Error>;

  /// Delete a sequence of bytes at an offset from the backend.
  async fn del(&mut self, offset: u64, length: u64) -> Result<(), Self::Error>;

  /// Resize the sequence of bytes, possibly discarding or zero-padding bytes
  /// from the end.
  async fn truncate(&mut self, length: u64) -> Result<(), Self::Error>;

  /// Get the size of the storage in bytes.
  async fn len(&self) -> Result<u64, Self::Error>;

  /// Whether the storage is empty.
  /// For some storage backends it may be cheaper to calculate whether the
  /// storage is empty than to calculate the length.
  async fn is_empty(&mut self) -> Result<bool, Self::Error>;

  /// Flush buffered data on the underlying storage resource.
  async fn sync_all(&mut self) -> Result<(), Self::Error>;
}
