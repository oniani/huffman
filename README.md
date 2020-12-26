# Huffman

A pure Rust implementation of the Huffman's algorithm for lossless data
compression.

## Dependencies

- [serde_json](https://docs.rs/serde_json/1.0.59/serde_json/index.html)

## Run

```sh
# Build the program
cargo build --release

# Compression.
# `to_encode` is a file to be encoded, `encoding.txt` is a Huffman compression
# After a successful run, there should be a file `metadata.json` which one
# could think of as a key for decoding/decompression the encoding/compression.
./target/release/huffman to_encode.txt encoding.txt

# Decompression.
# Make sure `metadata.json` is in the current directory.
# `encoding.txt` is the encoding that was generated in the previous step.
# `decoding.txt` should be a dcompression of `encoding.txt`.
# Notice that `decompress(compress(file)) = file` and hence, `decoding.txt`
# must have the same contents as  `to_encode.txt`.
# `--decompress` argument tells the program that the decompression mode is on.
./huffman encoding.txt decoding.txt --decompress
```

## Tests

Tests can be run as follows:

```
cargo test --release
```

## References

- [Huffman coding](https://en.wikipedia.org/wiki/Huffman_coding)

## License

[MIT License](LICENSE)
