![Rusty Pi](https://github.com/user-attachments/assets/64240465-e471-4637-a17e-6cbb64eaafe1)
# Pi Cake: Full Rust WebAssembly Stack Demonstration with a WASM Worker

> [!WARNING]
> How long does it take to measure &pi;?

A simple demo of a WebAssembly app together with a WebAssembly worker using Leptos, rust_i18n,
Tailwind 4 and web-sys, i.e. a full Rust WebAssembly stack.

It stupidly calculates Pi using the Leibniz formula and shows the result in a modal.
It does so by sending messages to the worker and receiving intermediate results and the final result.

## How it works

The app uses a WebAssembly worker to do the heavy lifting of calculating Pi.
After the worker is created it is passed a closure that handles messages from the worker.
The worker sends messages back to the main thread, the main thread receives these messages and updates the UI accordingly.
That's why the UI keeps responsive while the worker is doing the heavy lifting.
Without the worker the UI would be blocked.
The WASM worker itself is loaded using a separate JS file.

![rusty screenshot](https://github.com/user-attachments/assets/6c703f14-7307-412f-ad04-bdfffbce7e51)

## Overview

This application builds Rust code into WebAssembly. It demonstrates:

- Rust + WebAssembly performance for mathematical calculations
- [Leptos](https://github.com/leptos-rs/leptos) framework for reactive web applications
- Internationalization with rust-i18n
- Tailwind v4 SASS/CSS styling
- Web-sys for creating the WASM worker and communication between main thread and worker

## Features

- Uses an expensive function which would block the main thread (calculate &pi; with adjustable precision)
- Measures the time it takes
- Reactive UI updates with Leptos
- i18n support with multiple languages
- Debug (with logging) and release targets

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2021 edition or newer)
- [Trunk](https://trunkrs.dev/) (`cargo install trunk`)
- WebAssembly target: `rustup target add wasm32-unknown-unknown`
- Node.js and npm/yarn (for CSS processing)

## Getting Started

1. Clone the repository
2. Install
   ```sh
   cargo install trunk
   cargo install wasm-pack
   cargo install wasm-bindgen-cli
   ```` 
3. Run the development server:
   ```sh
   trunk serve --open
   # or
   make server
   ```

> [!TIP]
> For optimal performance, run with the release flag:
`sh trunk serve --open --release`

## Building for Production

```sh
trunk build --release
# or
make build
```

The compiled assets will be available in the `dist` directory.

## Commands

This project includes a Makefile with useful commands:

- `make server` - Start the development server
- `make build` - Build for production
- `make debug` - Build in debug mode
- `make docs` - Generate documentation
- `make test` - Run tests
- `make clean` - Clean all generated files

## Documentation

Generate documentation with:

```sh
cargo doc --document-private-items --open
# or
make docs
```

# What's Missing

## Using the API to Call Javascript Functions

- WASM is great but comes with large files
- especially when importing heavy libraries like RegEx
- call the API to use its built-ins and to compare against
- End2End testing

## License

MIT

## Acknowledgements

Thanks to [Steve Klabnik](https://steveklabnik.com/), [Carol Nichols](https://github.com/carols10cents), [Chris Krycho](https://github.com/chriskrycho) and the Rust Community for the [Rust Book](https://doc.rust-lang.org/book/).
