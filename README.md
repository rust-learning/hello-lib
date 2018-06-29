###### hello-lib

this is a simple project which just shows how to create a rust library.

* create a library

  ```bash
  $ cargo new --vcs git --lib hello-lib
  ```

* execute a test

  ```bash
  $ cargo test
  ```

* build

  Compile a local package and all of its dependencies

  ```bash
  $ cargo build --release
  ```