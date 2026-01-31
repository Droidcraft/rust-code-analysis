# rust-code-analysis Python Bindings

Python bindings for [rust-code-analysis](https://github.com/mozilla/rust-code-analysis), a Mozilla library to compute code metrics using tree-sitter parsing.

> **Note**: This is a community-maintained binding. The upstream library is developed by Mozilla.
> See [THIRD_PARTY_NOTICES.md](./THIRD_PARTY_NOTICES.md) for license information.

## Installation

### From GitHub Releases

Pre-built wheels are available on [GitHub Releases](https://github.com/Droidcraft/rust-code-analysis/releases).

```bash
# Install with pip (replace YYYYMMDD-N with the latest release tag)
pip install rust-code-analysis --find-links https://github.com/Droidcraft/rust-code-analysis/releases/expanded_assets/python-20260131-2

# Or with uv
uv pip install rust-code-analysis --find-links https://github.com/Droidcraft/rust-code-analysis/releases/expanded_assets/python-20260131-2
```

### Adding as a Dependency

In your `pyproject.toml`:

```toml
[project]
dependencies = [
    "rust-code-analysis",
]

# For uv - configure the find-links source
[tool.uv]
find-links = [
    "https://github.com/Droidcraft/rust-code-analysis/releases/expanded_assets/python-20260131-2"
]

# For pip - use dependency-links or install with --find-links flag
```

Alternatively, reference a specific wheel directly:

```toml
[project]
dependencies = [
    # Linux x86_64
    "rust-code-analysis @ https://github.com/Droidcraft/rust-code-analysis/releases/download/python-20260131-2/rust_code_analysis-0.1.0-cp312-cp312-manylinux_2_17_x86_64.manylinux2014_x86_64.whl ; sys_platform == 'linux' and platform_machine == 'x86_64'",
]
```

### From Source

```bash
# Requires Rust toolchain and maturin
pip install maturin
git clone https://github.com/Droidcraft/rust-code-analysis
cd rust-code-analysis/rust-code-analysis-python
maturin build --release
pip install target/wheels/*.whl
```

## Usage

```python
import rust_code_analysis as rca

# Analyze source code
source = """
def factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"""

result = rca.analyze(source, "factorial.py")

# Access metrics
print(f"Cyclomatic Complexity: {result.metrics.cyclomatic.sum}")
print(f"Cognitive Complexity: {result.metrics.cognitive.sum}")
print(f"Lines of Code: {result.metrics.loc.sloc}")
print(f"Maintainability Index: {result.metrics.mi.mi_visual_studio}")

# Halstead metrics
h = result.metrics.halstead
print(f"Halstead Volume: {h.volume}")
print(f"Halstead Difficulty: {h.difficulty}")
print(f"Estimated Bugs: {h.bugs}")

# Function-level metrics
for func in result.get_functions():
    print(f"{func.name}: CC={func.metrics.cyclomatic.sum}")
```

## Supported Languages

- Python
- Rust
- Java
- JavaScript
- TypeScript
- Kotlin
- C/C++

## Metrics Available

- **Cyclomatic Complexity (CC)** - Control flow complexity
- **Cognitive Complexity** - Human-perceived complexity
- **Lines of Code (LOC)** - SLOC, PLOC, LLOC, CLOC, blank
- **Halstead Metrics** - Volume, difficulty, effort, bugs, time
- **Maintainability Index (MI)** - Three variants
- **ABC Metric** - Assignments, Branches, Conditions
- **NOM** - Number of Methods
- **NARGS** - Number of Arguments
- **NEXITS** - Number of Exit Points
- **WMC, NPM, NPA** - Object-oriented metrics

## License

MPL-2.0
