# Chip Eight

My attempt at a Chip-8 emulator, in rust.

CHIP-8 is a simple, interpreted programming language that was developed in the mid-1970s by Joseph Weisbecker to make video game programming more accessible on microcomputers of that era. It's not actually a physical chip but rather a virtual machine designed to run games and simple graphics applications with minimal hardware requirements. More information [here](https://en.wikipedia.org/wiki/CHIP-8).

Features include:
- Full support for all instructions and quirks from the original Chip-8. As well as the modern Super-Chip, and XO-Chip extensions.
- Extensive configuration interface allowing fine-grained control over most aspects of the emulator. See usage section for more details.
- An abstraction layer that sits between the peripheral devices and the core emulator logic. Streamlining the implementation and configuration of alternate engines for the handling of keyboard input, and audio/display output.
- Ready to go SDL3 implementations of input/audio/display peripherals.



_Screenshots coming soon_



## Setup

Disclaimer: I work entirely on Linux, and have not tested this project on other operating systems. The following steps assume you are also on Linux, and as such, your success may vary.

### Prerequisites

This project requires SDL3. The `sdl3` crate (rust bindings for SDL3) exposes a number of configurable options to resolve this dependency, all of which are accessible via the corresponding `features` flags in `Cargo.toml`. I have it configured to build from source for personal convenience.

_For more details, please refer to the `sdl3` crate [documentation](https://github.com/maia-s/sdl3-sys-rs/tree/main/sdl3-sys#usage)_

### Installation

1. Clone the repo
    ```sh
    git clone https://github.com/brynmailer/chip-eight.git
    cd chip-eight
    ```
2. Build and install
    ```sh
    cargo install --path .
    ```
2. Run the emulator
    ```sh
    chip-eight roms/tests/CHIP8
    ```


## Usage

```sh
chip-eight [OPTIONS] <ROM_PATH>
```

_Pass the `--help` flag for the full list of options_


## Retrospective

_Coming soon_
