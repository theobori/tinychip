# tinychip

[![build](https://github.com/theobori/tinychip/actions/workflows/build.yml/badge.svg)](https://github.com/theobori/tinychip/actions/workflows/build.yml)

[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)

*CHIP-8 is an interpreted programming language, developed by Joseph Weisbecker made on his 1802 Microprocessor. It was initially used on the COSMAC VIP and Telmac 1800 8-bit microcomputers in the mid-1970s.* - *[Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)*

It is able to supports multiple graphical APIs and multiple interpreter implementations.
We consider an instruction ~= 1 cycle, so 500hz means it executes 500 instructions per second.

## 📖 How to build and run ?

1. Install the dependencies
    - `cargo`
    - `SDL2` (as library)
2. Compile and install it
    - `cargo install --path .`
3. Run `tinychip --help`

## 🤝 Contribute

If you want to help the project, you can follow the guidelines in [CONTRIBUTING.md](./CONTRIBUTING.md).

## 🖼️ Screenshots

<!-- #### Breakout (320x160 - 500hz) -->
![breakout](img/breakout_320_160.png)

<!-- #### Space Invaders (320x160 - 500hz) -->
![space_invaders](img/space_invaders_320_160.png)

<!-- #### IBM logo (640x320 - 500hz) -->
![ibm_logo](img/ibm_logo_640_320.png)

## ⌨️ Corresponding hotkeys

|   |   |   |   |
|---|---|---|---|
| **1** | **2** | **3** | **4** |
| **A** | **Z** | **E** | **R** |
| **Q** | **S** | **D** | **F** |
| **W** | **X** | **C** | **V** |

## 🔗 Compatibility

Some descriptions of the chip8 instructions differ depending on the machine. For example, the instructions `8xy6` and `8xye` do not do the same thing according to the documents.

In general throughout the documents there are two kinds of semantic for the load operations (`fx55`, `fx65`) and for the shift operations (`8xy6`, `8xye`).

#### Semantics

To use the original semantic, use the following flags:
- Load : `--original-load=true`
- Shift : `--original-shift=true`

Opcode | Default | Original
:-------------: | :---------: | :--------------:
**8xy6** | Vx = Vx >> 1, Vf = carry | Vx = Vy >> 1, Vf = carry
**8xye** | Vx = Vx << 1, Vf = carry | Vx = Vy << 1, Vf = carry
**fx55** | I = I + x + 1 | ❌
**fx65** | I = I + x + 1 | ❌

#### Games

Some games where we know the best compatibility settings, [Github issue](https://github.com/Diesel-Net/kiwi-8/issues/9).

## 🐋 Docker playground

#### 🔨 Build

```bash
docker buildx build . -t tinychip
```
#### 🎉 Run example

In the example below, we have:

- `X11` as graphic server
- `PulseAudio` as sound server

```bash
docker run -it --rm \
    -e DISPLAY=$DISPLAY \
    -v /tmp/.X11-unix:/tmp/.X11-unix \
    -v /dev/dri:/dev/dri \
    -v /dev/snd:/dev/snd \
    -v $PWD/roms:/roms \
    -v /run/user/$(id -u)/pulse/native:/run/user/$(id -u)/pulse/native \
    -e PULSE_SERVER=unix:/run/user/$(id -u)/pulse/native \
    -u $(id -u):$(id -u) \
    tinychip /roms/brick.ch8
```

## ℹ️ Documentation

Run `cargo doc --open` to read the documentation in the browser.
