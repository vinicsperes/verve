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
appear, you type, and every character lights up as you go: clean text for the
ones you nailed, a red underline for the misses. It tracks your best run of the
session and throws a little confetti when you beat it.

## Features

- Real-time, per-character feedback as you type
- Live WPM and timer while typing; WPM, accuracy and time on finish
- Session best score, with a confetti celebration on a new record
- Random words from a curated word list, or your own custom text
- A single, self-contained binary: instant startup, no config files

<p align="center">
  <img src="demo/typing.gif" alt="typing in verve" width="720">
</p>

## Installation

`verve` is a single self-contained binary that runs on Linux, macOS and Windows.

### Download a prebuilt binary (easiest)

No Rust and no compiler needed. Just grab the binary for your system from the
[**Releases**](https://github.com/vinicsperes/verve/releases/latest) page.

**Linux / macOS**

```sh
# pick the file matching your system, e.g. verve-x86_64-unknown-linux-gnu.tar.gz
tar -xzf verve-*.tar.gz
sudo mv verve /usr/local/bin/      # somewhere on your PATH
verve
```

**Windows**

Download `verve-x86_64-pc-windows-msvc.zip` and unzip it. To run `verve` from
any terminal, put the executable in a folder on your `PATH`:

```powershell
# create a folder, move verve.exe into it, and add it to your PATH (permanent)
mkdir $HOME\bin
Move-Item verve.exe $HOME\bin\
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";$HOME\bin", "User")
```

Close and reopen the terminal, then run `verve` from anywhere.

### Install with Cargo

If you already have the [Rust toolchain](https://rustup.rs):

```sh
cargo install --git https://github.com/vinicsperes/verve
```

Building from source also needs a **C linker**, which Rust uses to compile some
dependencies. If you hit `error: linker 'cc' not found` (or `link.exe not found`
on Windows), install it for your system:

| OS              | Install the C toolchain                                          |
| --------------- | --------------------------------------------------------------- |
| Debian / Ubuntu | `sudo apt install build-essential`                              |
| Fedora          | `sudo dnf install gcc`                                          |
| Arch            | `sudo pacman -S base-devel`                                     |
| macOS           | `xcode-select --install`                                        |
| Windows         | Visual Studio **C++ Build Tools** (rustup offers to install it) |

`cargo install` places the binary in `~/.cargo/bin` (`%USERPROFILE%\.cargo\bin`
on Windows). If `verve` isn't found afterwards, add that folder to your `PATH`.
In fish: `fish_add_path ~/.cargo/bin`.

### Build from a local clone

```sh
git clone https://github.com/vinicsperes/verve.git
cd verve
cargo install --path .   # add `verve` to your PATH
# or just try it without installing:
cargo run --release
```

## Usage

```sh
verve                          # 25 random words
verve -w 50                    # 50 random words
verve -t "the quick brown fox" # type a fixed text
verve --help                   # all options
```

| Flag                | Description                                   |
| ------------------- | --------------------------------------------- |
| `-w, --words <n>`   | number of random words (default: 25)          |
| `-t, --text <text>` | type a fixed text instead of random words     |
| `-h, --help`        | show help                                      |

## Controls

| Key                | Action      |
| ------------------ | ----------- |
| any letter         | type        |
| `backspace`        | delete      |
| `ctrl`+`backspace` | delete word |
| `tab`              | restart     |
| `esc`              | quit        |

## How scoring works

- **WPM** is `(correct characters / 5) / minutes`, using the standard five-character
  word. Only correct characters count.
- **Accuracy** is correct characters over characters typed.
- The **best score** is saved per word count and persists across sessions. Custom-text
  runs (`--text`) are tracked only for the current session.

## License

[MIT](LICENSE)
