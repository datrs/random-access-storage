## 2020-03-03, Version 4.0.0
### Commits
- [[`064eb1d6c9`](https://github.com/datrs/random-access-storage/commit/064eb1d6c9c1110f7cb01bfbaa6eb43d52330cd4)] (cargo-release) version 4.0.0 (Bruno Tavares)
- [[`0e7ab518e5`](https://github.com/datrs/random-access-storage/commit/0e7ab518e5d529160073e5aa295ff6a711472944)] Merge pull request #22 from bltavares/async-trait (Bruno Tavares)
- [[`2f5f4a0567`](https://github.com/datrs/random-access-storage/commit/2f5f4a0567db834f72afc6b75c512f309c9f4b13)] Github feedback: use futures-io to help with compile times (Bruno Tavares)
- [[`2bf3131775`](https://github.com/datrs/random-access-storage/commit/2bf31317753b242bedec37a0a6a576f5c26533c7)] Implement async API for random-access-storage (Bruno Tavares)
- [[`0bed19ef50`](https://github.com/datrs/random-access-storage/commit/0bed19ef50d0df238d0457e8af8890289ff385ed)] Merge pull request #20 from bltavares/master (Szabolcs Berecz)
- [[`831b9aa23e`](https://github.com/datrs/random-access-storage/commit/831b9aa23e72fbe38cf19c3c7c794d8acfe9099a)] Adjust Travis to run clippy and rustfmt on stable only (Bruno Tavares)
- [[`9c228739cc`](https://github.com/datrs/random-access-storage/commit/9c228739cc504d778b5c840d3d339dfbb54cae33)] Removes unused dependency (Bruno Tavares)
- [[`e0a1ea9880`](https://github.com/datrs/random-access-storage/commit/e0a1ea9880a42f48459763d30724badcbe274ac2)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 .travis.yml  |  8 ++++----
 CHANGELOG.md | 18 ++++++++++++++++++
 Cargo.toml   |  6 ++++--
 README.md    | 24 ++++++++++++------------
 src/lib.rs   | 29 ++++++++++++++++++-----------
 5 files changed, 56 insertions(+), 29 deletions(-)
```


## 2019-07-27, Version 3.0.0
### Commits
- [[`49c25778a0`](https://github.com/datrs/random-access-storage/commit/49c25778a0f80db733028c059958303e374b5965)] (cargo-release) version 3.0.0 (Yoshua Wuyts)
- [[`553af611fd`](https://github.com/datrs/random-access-storage/commit/553af611fde9a22e92b17c9b52cc1379cd4dc57d)] u64 file offsets (#17) (James Halliday)
- [[`b73e7fb619`](https://github.com/datrs/random-access-storage/commit/b73e7fb619ebf07de6f5f409bee4f843cb0f0967)] sync_all method (#18) (James Halliday)
- [[`02e700d41c`](https://github.com/datrs/random-access-storage/commit/02e700d41ce119325d09c762b697b957af16697a)] Changed signature of len() to immutably borrow self (#19) (Freddie Ridell)
- [[`36c5b96d63`](https://github.com/datrs/random-access-storage/commit/36c5b96d63159317c6ec756ff579174e07856b10)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 CHANGELOG.md | 16 ++++++++++++++++
 Cargo.toml   |  4 ++--
 README.md    |  4 ++++
 src/lib.rs   | 21 ++++++++++-----------
 4 files changed, 32 insertions(+), 13 deletions(-)
```


## 2018-12-18, Version 2.0.0
### Commits
- [[`55ab88f0fd`](https://github.com/datrs/random-access-storage/commit/55ab88f0fd5114f8911442bc665fcca6949516ac)] (cargo-release) version 2.0.0 (Yoshua Wuyts)
- [[`81319ec100`](https://github.com/datrs/random-access-storage/commit/81319ec100e196c0ad79528d966d7717005c38f1)] len() and is_empty() (#16) (James Halliday)
- [[`fb18d9f5d3`](https://github.com/datrs/random-access-storage/commit/fb18d9f5d3a4b950f457d941acd69f6c57a11bf7)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 CHANGELOG.md | 18 ++++++++++++++++++
 Cargo.toml   |  2 +-
 README.md    |  8 ++++++++
 src/lib.rs   |  8 ++++++++
 4 files changed, 35 insertions(+), 1 deletion(-)
```


## 2018-11-20, Version 1.0.0
### Commits
- [[`2e9f4090d7`](https://github.com/datrs/random-access-storage/commit/2e9f4090d766a8fbaf6301e789b7db1439e383ad)] (cargo-release) version 1.0.0 (Yoshua Wuyts)
- [[`5e423e35ff`](https://github.com/datrs/random-access-storage/commit/5e423e35ff60ad7ed918a883c776c287a4b9b3fe)] truncate method (#15) (James Halliday)
- [[`0b242b6234`](https://github.com/datrs/random-access-storage/commit/0b242b6234586ceb1b23a2ac5a7ba5b021084be5)] Keep up with modern times in clippy invocation (#12) (Szabolcs Berecz)
- [[`622cf79c92`](https://github.com/datrs/random-access-storage/commit/622cf79c92194ea234c92bd090cd061425331ff0)] update changelog (Yoshua Wuyts)

### Stats
```diff
 .travis.yml  |  2 +-
 CHANGELOG.md | 19 +++++++++++++++++++
 Cargo.toml   |  2 +-
 README.md    |  4 ++++
 src/lib.rs   |  4 ++++
 5 files changed, 29 insertions(+), 2 deletions(-)
```


## 2018-08-29, Version 0.6.0
### Commits
- [[`23e48f8e29`](https://github.com/datrs/random-access-storage/commits/23e48f8e29fc5cb0eaf7a0e77485cd6d23884771)] (cargo-release) version 0.6.0 (Yoshua Wuyts)
- [[`f1fc4982aa`](https://github.com/datrs/random-access-storage/commits/f1fc4982aa1f1a4e6b919344810f87ec36be36c7)] Fixes #7: Make RandomAccess always open (#11) (Szabolcs Berecz)
- [[`3855bef02a`](https://github.com/datrs/random-access-storage/commits/3855bef02a392bff362a297b610cdf9180f00ded)] Update README.md (#10) (周汉成)
- [[`97ebff543c`](https://github.com/datrs/random-access-storage/commits/97ebff543c466582f68c7dd192361dc41a9646dc)] add read_to_writer method (#9) (Yoshua Wuyts)
- [[`50b947e688`](https://github.com/datrs/random-access-storage/commits/50b947e688ef3ea5addd81991386ae389a66c1f6)] Fix typo and switch to clippy-preview (#5) (Szabolcs Berecz)
- [[`61a3d42a56`](https://github.com/datrs/random-access-storage/commits/61a3d42a56cfba8c0803854394f9488e68525085)] (cargo-release) start next development iteration 0.5.1-alpha.0 (Yoshua Wuyts)

### Stats
```diff
 .travis.yml |  2 +-
 Cargo.toml  |  2 +-
 README.md   | 23 +++++++++++++++-------
 src/lib.rs  | 66 ++++++++++----------------------------------------------------
 4 files changed, 29 insertions(+), 64 deletions(-)
```


