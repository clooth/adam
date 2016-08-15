//! This crate provides a framework for building simple 2D games
//! using `SDL2`, `SDL2_image`, `SDL_mixer` and `SDL_ttf` behind the scenes.

#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unused_import_braces, unused_qualifications)]

#![feature(plugin)]
#![plugin(clippy)]

extern crate sdl2;
extern crate float_cmp;

pub mod data;
