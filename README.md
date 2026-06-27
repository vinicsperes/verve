<h1 align="center">verve</h1>

<p align="center">A minimalist typing speed test that lives in your terminal.</p>

<p align="center">
  <img src="https://img.shields.io/badge/license-MIT-d77757" alt="license">
  <img src="https://img.shields.io/badge/built%20with-Rust-d77757?logo=rust&logoColor=white" alt="built with Rust">
  <img src="https://img.shields.io/badge/platform-linux%20%C2%B7%20macOS%20%C2%B7%20windows-d77757" alt="platforms">
</p>

<p align="center">
  <img src="demo/record.gif" alt="verve" width="720">
</p>

`verve` measures how fast you type without ever leaving the terminal. Words
appear, you type, and every character lights up green or red as you go. It
remembers your best run of the session and throws a little confetti when you
beat it.

## Features

- Real-time, per-character feedback as you type
- Live and final words-per-minute and accuracy
- Session best score, with a confetti celebration on a new record
- Random words or your own custom text
- A single, self-contained binary — instant startup, no config

<p align="center">
  <img src="demo/typing.gif" alt="typing in verve" width="720">
</p>

## Getting started

`verve` is written in Rust and compiles to one self-contained binary that runs
on Linux, macOS and Windows. You'll need the [Rust toolchain](https://rustup.rs).

```sh
git clone https://github.com/vinicsperes/verve.git
cd verve
cargo run --release
```

## Usage

```sh
verve                # 25 random words
verve --words 50     # 50 random words
verve --text "..."   # type a fixed text
verve --help         # all options
```

## Controls

| Key                | Action      |
| ------------------ | ----------- |
| any letter         | type        |
| `backspace`        | delete      |
| `ctrl`+`backspace` | delete word |
| `tab`              | restart     |
| `esc`              | quit        |

## License

[MIT](LICENSE)
