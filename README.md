# 🔍 Rust Grep

A simple CLI tool built in Rust that mimics basic functionality of `grep`. It supports interactive usage, regular expression pattern matching, and highlights results in Green.

## 🚀 Features

- Interactive command-line interface with `~ >` prompt
- Regex-based search using the `regex` crate
- Colored match highlighting using `termcolor`
- Graceful error handling for invalid input or missing files

---

## 🧱 Project Structure

src/
├── main.rs # Entry point — handles command-line interaction
└── grep.rs # Contains Grep struct and search logic

---

## ⚙️ Dependencies

```toml
[dependencies]
regex = "1"
termcolor = "1"
