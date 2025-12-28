# dooc_embed_typst

A minimalist, self-contained wrapper for the [Typst](https://typst.app) compiler. 

This library is designed for building **Single Static Binary** tools. It allows
you to "bake" your Typst source code, fonts, images, and data directly into
your Rust executable, creating a PDF generator that runs offline with zero
external dependencies.

I originally started making a math worksheet generator for my homeschooled
daughter, but then I realized I had made a nice Typst wrapper to fit that
use-case, so I decided to publish it.

You can find the math worksheet generator here:

[doocMath](https://github.com/destroyerOfOfficeChairs/doocMath)

If this sentence is still included in the readme, then I have not yet rewritten
doocMath in Rust.

## Features

* **Single Static Binary:** Designed to "bake" all dependencies (fonts, images,
plugins, and source code) directly into your Rust executable using
`include_bytes!`.

* **Zero Runtime Dependencies:** The resulting binary runs completely offline.
It does not require a local Typst CLI installation, system fonts, or an
internet connection.

* **Virtual File System:** Implements a custom Typst World that serves files
from memory. This allows you to use `#image("graph.png")` or
`#plugin("tool.wasm")` within Typst, resolving them to bytes stored in your
Rust HashMap.

* **Native Data Injection:** Seamlessly pass Rust variables (Integers, Floats,
Strings, Booleans, etc.) into Typst via `sys.inputs`.

* **Pure Rust:** Built on top of `typst` and `typst-pdf` crates, ensuring
memory safety and easy integration into existing Rust workflows.

## Limitations

* **Single Source File:** The current `World` implementation only supports a
single entry point. You cannot use `#include` or `#import` to reference other
local .typ files. You must concatenate your source code into a single string
(or use a templating engine like `askama`) before passing it to the compiler.

* **No Package Manager:** This library is "offline-first." It does not support
Typst's package management system (e.g., `#import "@preview/..."`). If you need
external packages, you must download the source files manually and concatenate
them into your main string.

* **Manual Font Loading:** You must provide the font files for every font you
wish to use. The system defaults are not automatically loaded (which ensures
cross-platform consistency but requires initial setup).

* **Binary Size:** Because assets are embedded, your executable size will
increase by the size of the fonts and images you include.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dooc_embed_typst = { git = "https://github.com/destroyerOfOfficeChairs/dooc_embed_typst" }
```

## Usage

Take a look at `examples/demo.rs` to see a well-commented file on how to use
this library.

If you clone this repo and `cd` into it, you can run `demo.rs` by entering this
in your terminal:

```bash
cargo run -p dooc_embed_typst --example demo
```

You should then see an `output.pdf` file in your current directory.

## Version

0.1.0

## Changelog

* **0.1.0:** Initial commit.

## License

MIT
