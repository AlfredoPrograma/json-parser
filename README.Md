# JSON Parser (by a Rust Beginner)
This project is a simple JSON parser developed in Rust. It's designed as a beginner's project, making it perfect for those just starting out with Rust and looking to understand how to work with JSON data and the [Nom](https://docs.rs/nom/latest/nom/) parsing crate.

## Introduction
A few weeks ago, I was eager to learn, especially about an exciting and "low level" language, so I decided to start my **Rust learning journey**. I've been struggling a bit (*especially with lifetimes* 🫠), but nothing that can't be overcome with practice and research. As I became comfortable with my current knowledge, I decided to write my own **JSON Parser**.

The parser implementation is simple; it merely takes a raw **JSON** string and translates it into a tree-based structure in **Rust** comprised of nested `enums`. These `enums` encapsulate valid **JSON values** and aim to follow the rules provided by the [JSON specification](https://www.json.org/json-en.html).

## Features
- Parses a **raw JSON string** into a tree-based data structure composed of nested `enums`.
- Handles different **JSON data types**: `null, string, boolean, number, array, and object`.
- Catches invalid **JSON strings** and throws respective errors.
- Uses only one dependency ([Nom as a parser combinator library](https://docs.rs/nom/latest/nom/)).

## Installation
To install the **JSON parser**, you need to have **Rust** installed on your machine. If you don't have Rust installed, you can download it from the [official Rust website](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can clone this repository:
```bash
git clone https://github.com/AlfredoPrograma/json-parser.git
cd json-parser
```

## Example
Run the program to see an example:
```bash
cargo run
```
This will print the parsed structure built based on the `example.json` file. Feel free to modify and complicate the JSON file, then run the program again to see the parsed JSON in the output.

## To be continued...
I will continue my learning journey with Rust, as I find it to be a fun and complex language that I should use to solve niche and specific problems.

As for improving this project, I feel I've accomplished my goal, so I won't enhance it in the short term. But who knows...