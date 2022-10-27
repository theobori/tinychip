# toychip

*CHIP-8 is an interpreted programming language, developed by Joseph Weisbecker made on his 1802 Microprocessor. It was initially used on the COSMAC VIP and Telmac 1800 8-bit microcomputers in the mid-1970s.* - *[Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)*

It supports multiple graphical APIs

## How to build and run ?

1. Install the dependencies
    - `cargo`
2. Compile and install it
    - `cargo install --path .`
3. Run `toychip --help`

## Tested programs

Name           | Status
-------------  | :-------------:
IBM logo | ✅
Pong | ❌
Space invaders | ❌
Tetris | ❌

## Compatibility

Some descriptions of the chip8 instructions differ depending on the machine. For example, the instructions `8xy6` and `8xye` do not do the same thing according to the documents.

In general throughout the documents there are two kinds of semantic for the load operations (`fx55`, `fx65`) and for the shift operations (`8xy6`, `8xye`).

#### Semantics

To use the original semantic, use the following flags:
- Load : `--original-load-semantic=true`
- Shift : `--original-shift-semantic=true`

Opcode | Default | Original
:-------------: | :---------: | :--------------:
**8xy6** | Vx = Vx >> 1, Vf = carry | Vx = Vy >> 1, Vf = carry
**8xye** | Vx = Vx << 1, Vf = carry | Vx = Vy << 1, Vf = carry
**fx55** | I = I + x + 1 | ❌
**fx65** | I = I + x + 1 | ❌

#### Games

Some games where we know the best compatibility settings, [Github issue](https://github.com/Diesel-Net/kiwi-8/issues/9).

## Todo

Name           | Status
-------------  | :-------------:
500Hz clock | ⚠️
Basic instructions | ✅
Handle every CLI args **¹** | ✅
Debug features **²** | ⚠️
Create font | ✅

**¹** CLI args:
* Clock (optional)
* Window size (optional)
* Graphical API (optional)
* Interpreter (optional)
* quirks (shift and load)

**²** Features:
* Pause the game
* Switch between a limited amount of stored emulation state
* Show interpreter values like registers, pc, keys, etc..
* Custom hotkeys

## Documentation

Run `cargo doc --open` to read the documentation in the browser.
