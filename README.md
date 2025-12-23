# Rust ResJS WASM

## What

This repo represents my attempt to port Dr Nederhof's `resjs` to Rust. I will only be working on
porting `res_concet.js` to Rust, and have no plans to port the other `*_concat.js` files (e.g. the editor).

## Why

I wanted to do this port, because...

- It's good experience working with Rust.
- I wanted to see if I could do it.
- I wanted to find out if the shorter load times and slightly faster execution of WASM in spite of increased IO latency
  would outweigh the performance of the original JS implementation.

Fair warning: My Rust skills are still very intro level, so don't judge my code too harshly. It'll get better. 

---

# Original README.md from https://github.com/nederhof/resjs

# RES in JavaScript

For more information, open `index.html` in a browser, or go to
[Hieroglyphic encoding (RES) in web pages using JavaScript](http://mjn.host.cs.st-andrews.ac.uk/egyptian/res/js/).

See also [L2/18-236](http://www.unicode.org/L2/L2018/18236-nederhof.pdf).
