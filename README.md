# Dyarr

Dyarr is a Rust library that provides an "array" whose size and *dimensions* are determined at runtime. It is implemented using a 1D `Box<[T]>`, instead of a vector of vectors. So the size is fixed when being initialized.

The implementation is very simple, and you can easily extend it if you need more features.

## Usage

Currently, there is no formal documentation. However, you can find some examples in `tests/example.rs`.