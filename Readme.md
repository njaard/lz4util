[![GitHub license](https://img.shields.io/badge/license-BSD-blue.svg)](https://raw.githubusercontent.com/njaard/wordnet-rs/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/wordnet.svg)](https://crates.io/crates/lz4util)

# Introduction

This is a small program for compressing and decompression with
[LZ4](http://www.lz4.org). LZ4 is a compression algorithm
that produces compression ratios somewhat worse than `gzip`, but
decompression is about 10 times faster!

The command line arguments are very similar to `gzip`.

This program is written in [Rust](http://rust-lang.org), a blazing
fast systems programming language.

# Installation

Assuming you have Rust's `cargo` installed, a simple `cargo install lz4util`
will install this program, by the command name `lz4`.

# Usage

Compresses `big_file`, produces `big_file.lz4`:

    lz4 big_file

Decompresses `big_file.lz4`, produces `big_file`:

    lz4 -d big_file.lz4

Compresses `big_file`, produces `big_file.lz4`, but doesn't delete `big_file`:

    lz4 -k big_file

Compresses `big_file` again, overwriting `big_file.lz4`, and still doesn't delete `big_file`:

    lz4 -fk big_file

Compress from a pipeline:

    find / | lz4 > all_files.lz4

Decompress from a pipeline:

    lz4 -d < all_files.lz4 | less

