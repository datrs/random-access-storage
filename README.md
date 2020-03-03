# random-access-storage
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Abstract interface to implement random-access instances.

- [Documentation][8]
- [Crate][2]

## Why?
This module forms a shared interface for reading and writing bytes to
different backends. By having a shared interface, it means implementations
can easily be swapped, depending on the environment.

## Usage
```rust
use random_access_storage::{RandomAccessMethods, RandomAccess};
use async_trait::async_trait;

struct S;
#[async_trait]
impl RandomAccessMethods for S {
  type Error = std::io::Error;

  async fn open(&mut self) -> Result<(), Self::Error> {
    unimplemented!();
  }

  async fn write(&mut self, offset: u64, data: &[u8]) -> Result<(), Self::Error> {
    unimplemented!();
  }

  async fn read(&mut self, offset: u64, length: u64) -> Result<Vec<u8>, Self::Error> {
    unimplemented!();
  }

  async fn read_to_writer(
    &mut self,
    offset: u64,
    length: u64,
    writer: &mut (impl futures::io::AsyncWriter + Send)
  ) -> Result<(), Self::Error> {
    unimplemented!();
  }

  async fn del(&mut self, offset: u64, length: u64) -> Result<(), Self::Error> {
    unimplemented!();
  }

  async fn truncate(&mut self, length: u64) -> Result<(), Self::Error> {
    unimplemented!();
  }

  async fn len(&mut self) -> Result<u64, Self::Error> {
    unimplemented!();
  }

  async fn is_empty(&mut self) -> Result<bool, Self::Error> {
    unimplemented!();
  }

  async fn sync_all(&mut self) -> Result<(), Self::Error> {
    unimplemented!();
  }
}

let _file = RandomAccess::new(S);
```

## Installation
```sh
$ cargo add random-access-storage
```

## See Also
- [random-access-storage/random-access-storage](https://github.com/random-access-storage/random-access-storage)

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/random-access-storage.svg?style=flat-square
[2]: https://crates.io/crate/random-access-storage
[3]: https://img.shields.io/travis/datrs/random-access-storage.svg?style=flat-square
[4]: https://travis-ci.org/datrs/random-access-storage
[5]: https://img.shields.io/crates/d/random-access-storage.svg?style=flat-square
[6]: https://crates.io/crates/random-access-storage
[7]: https://docs.rs/random-access-storage/badge.svg
[8]: https://docs.rs/random-access-storage
