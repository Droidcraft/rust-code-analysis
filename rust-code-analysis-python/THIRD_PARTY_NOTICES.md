# Third Party Notices

This package provides Python bindings for third-party software.

## rust-code-analysis

**Source**: https://github.com/mozilla/rust-code-analysis
**License**: Mozilla Public License 2.0 (MPL-2.0)
**Copyright**: Mozilla Corporation and contributors

This package depends on and wraps the `rust-code-analysis` library developed by Mozilla.
The upstream library provides all code analysis and metrics computation functionality.
These Python bindings are a community contribution and are not officially maintained by Mozilla.

### MPL-2.0 License Summary

The Mozilla Public License 2.0 allows:
- Commercial use
- Distribution
- Modification
- Patent use
- Private use

Under the conditions:
- Disclose source (for MPL-licensed files)
- License and copyright notice
- Same license (for MPL-licensed files)

Full license text: https://www.mozilla.org/en-US/MPL/2.0/

---

## tree-sitter

The upstream library uses tree-sitter parsers for language support.

**Source**: https://github.com/tree-sitter/tree-sitter
**License**: MIT License
**Copyright**: tree-sitter contributors

### Language Grammars

| Language | Repository | License |
|----------|------------|---------|
| Python | tree-sitter/tree-sitter-python | MIT |
| Rust | tree-sitter/tree-sitter-rust | MIT |
| Java | tree-sitter/tree-sitter-java | MIT |
| JavaScript | tree-sitter/tree-sitter-javascript | MIT |
| TypeScript | tree-sitter/tree-sitter-typescript | MIT |
| Kotlin | AHarm/tree-sitter-kotlin-ng | MIT |
| C/C++ | Mozilla (tree-sitter-mozcpp) | MPL-2.0 |

---

## PyO3

**Source**: https://github.com/PyO3/pyo3
**License**: Apache-2.0 OR MIT
**Copyright**: PyO3 contributors

Used to create the Python bindings for this Rust library.
