# Pi Cake: Leptos WebAssembly Tutorial / Playground

A simple demonstration for using Rust with Leptos in a web application compiled to WebAssembly. Not for production, just to get into the topic, especially if you're new to Rust and Leptos.

![rusty screenshot](https://github.com/user-attachments/assets/6c703f14-7307-412f-ad04-bdfffbce7e51)


## Overview

This application builds Rust code into WebAssembly. It demonstrates:

- Rust + WebAssembly performance for mathematical calculations
- [Leptos](https://github.com/leptos-rs/leptos) framework for reactive web applications
- Internationalization with rust-i18n
- Tailwind v4 SASS/CSS styling

## Features

- Uses an expensive function to block the thread (calculate &pi; with adjustable precision)
- Measures the time it takes
- Reactive UI updates with Leptos
- i18n support with multiple languages
- Debug and release targets

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2021 edition or newer)
- [Trunk](https://trunkrs.dev/) (`cargo install trunk`)
- WebAssembly target: `rustup target add wasm32-unknown-unknown`
- Node.js and npm/yarn (for CSS processing)

## Getting Started

1. Clone the repository
2. Run the development server:
   ```sh
   trunk serve --open
   # or
   make server
   ```
   For optimal performance, run with the release flag:
    ```sh
    trunk serve --open --release
    ```

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

## License

MIT

## Acknowledgements

Thanks to [Steve Klabnik](https://steveklabnik.com/), [Carol Nichols](https://github.com/carols10cents), [Chris Krycho](https://github.com/chriskrycho) and the Rust Community for the [Rust Book](https://doc.rust-lang.org/book/).
