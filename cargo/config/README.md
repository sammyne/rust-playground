# config

This crates demonstrates the local source replacement by the local `.cargo/config`.

## How
```bash
cd vendor/crates
cargo vendor

# do as guided, i.e., save the output to '.cargo/config' in the project root

# copy Cargo.lock to the project root

# build the project 
cd -
cargo build
```
