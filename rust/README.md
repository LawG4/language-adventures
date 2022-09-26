# Rust Adventures

This is where we'll place our attempts at exploring rust. We'll have one workspace, this is a set of packages which share the same output directory and lock file. Meaning that if you run the build from inside one of the example directories, the output will still be stored in this root rust directory.

```bash
# How to build
# Simply run the cargo build
cargo build

# Run an example
# Use example directory name without number prefix
cargo run --bin hello-world
```




