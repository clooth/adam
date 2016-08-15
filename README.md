# adam

[![Clippy Linting Result](https://clippy.bashy.io/github/clooth/adam/master/badge.svg)](https://clippy.bashy.io/github/clooth/adam/master/log)

This crate provides a framework for building simple 2D games using `SDL2`, `SDL2_image`, `SDL_mixer` and `SDL_ttf` behind the scenes.

## Feature Roadmap

- [ ] Renderer Management
- [ ] Generic Timer
- [ ] Sprites
- [ ] Input (Keyboard, Mouse)
- [ ] Sounds and Music

## Building

```
- git clone https://github.com/clooth/adam.git
- cd adam && cargo build --verbose
```

## Running tests

```
$ git clone https://github.com/clooth/adam.git
$ cd adam && cargo test --verbose
```

## Running examples

There are some examples in the `examples/` directory that you can run via:

```
- cargo run --example <example_name>
```

## Integration

```
[dependencies]
adam = "*"
```

```
$ cargo build
```

```
extern crate adam;

use adam::data::Rectangle;
```
