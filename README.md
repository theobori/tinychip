# chip8

*CHIP-8 is an interpreted programming language, developed by Joseph Weisbecker made on his 1802 Microprocessor. It was initially used on the COSMAC VIP and Telmac 1800 8-bit microcomputers in the mid-1970s.* - *[Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)*

It supports multiple graphical APIs

## How to build and run ?

1. Install the dependencies
    - `cargo`
2. Compile and install it
    - `cargo install --path .`
3. Run `chip8 --help`

## Tested programs

Name           | Status
-------------  |:-------------:
IBM logo | ✅
Pong | ❌
Space invaders | ❌
Tetris | ❌

## Todo

Name           | Status
-------------  |:-------------:
500Hz clock | ⚠️
Basic instructions | ⚠️
Handle CLI args **¹** | ⚠️
Debug features **²** | ⚠️
Create font | ⚠️

**¹** CLI args:
* Clock (optional)
* Window size (optional)
* Graphical API (optional)
* Interpreter (optional)

**²** Features:
* Pause the game
* Switch between a limited amount of stored emulation state
* Show interpreter values like registers, pc, keys, etc..
* Custom hotkeys

## Documentation

Run `cargo doc --open` to read the documentation in the browser.
