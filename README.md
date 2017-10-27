# boxxy-rs

"_If you implement boundaries and nobody is around to push them, do they even
exist?_". Have you ever wondered how your sandbox looks like from the inside?
Tempted to test if you can escape it, if only you had a shell to give it a try?
boxxy is a library that can be linked into a debug build of an existing program
and drop you into an interactive shell. From there you can step through various
stages of you sandbox and verify it actually contains™.

## Development

    cargo run --example boxxy

## Linking with rust

Just put a dev-dependencies in your Cargo.toml and copy `examples/boxxy.rs` to
your `examples/` folder. Modify to include your sandbox.

    [dev-dependencies]
    boxxy = "*"

## Linking with C

There is an example program, check the Makefile to see how it's built.

    make cboxxy

## Warning

The shell is a basic interface for human input, do not write actual scripts,
there be dragons.

**Do not include boxxy in production builds.**

## License

This project is free software released under the LGPL3+ license. The example
code in `example/boxxy.rs` is released under the MIT license.
