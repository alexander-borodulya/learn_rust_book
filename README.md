# Introduction

The purpose of this repository is purely self-learning Rust language concepts by coding, using the official Rust book as a base.

The repository doesn’t contain any production-ready implementations nor pretends to be any complete project.

Meaning that the codebase structure doesn’t follow any design patterns and in most places as simple as possible (for instance, scope-based delimiters used).

# Internal Structure

The repository contains a bunch of workspaces:
- `rust_book_runner` - workspace entry point that's responsible for enumerating and running a particular chapter;
- `rust_book` - the workspace that contains implementations for each chapter;
- All the rest of the workspaces are chapter-specific implementations where the workspace is a more convenient way to represent a code sample, compared to a module or a single `rs-file`.

Each chapter has a corresponding `rs`-file (`rust_book/src/chapter_001.rs`) within the public function `pub fn run()` - a chapter entry point, which encapsulates sub-chapter calls.

Each subchapter corresponds to a function in the format: `fn chapter_m_n()`, where `m` is the chapter index and `n` is the subchapter index.
