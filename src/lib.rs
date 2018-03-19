// #![deny(warnings, missing_docs)]
// #![cfg_attr(test, feature(plugin))]
// #![cfg_attr(test, plugin(clippy))]

extern crate failure;

pub use failure::Error;

pub trait SyncMethods {
  fn open(&self) -> Result<(), Error>;
  fn write(&self, offset: u64, data: Vec<u8>) -> Result<(), Error>;
  fn read(&self, offset: u64, length: u64) -> Result<Vec<u8>, Error>;
  fn del(&self, offset: u64, length: u64) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct Sync<T> {
  pub handler: T,
  pub is_open: bool,
}

impl<T> Sync<T>
where
  T: SyncMethods,
{
  pub fn write(&self, offset: u64, data: Vec<u8>) -> Result<(), Error> {
    if !self.is_open {
      T::open(&self.handler)?;
    }
    T::write(&self.handler, offset, data)
  }

  pub fn read(&self, offset: u64, length: u64) -> Result<Vec<u8>, Error> {
    if !self.is_open {
      T::open(&self.handler)?;
    }
    T::read(&self.handler, offset, length)
  }

  pub fn del(&self, offset: u64, length: u64) -> Result<(), Error> {
    if !self.is_open {
      T::open(&self.handler)?;
    }
    T::del(&self.handler, offset, length)
  }
}
