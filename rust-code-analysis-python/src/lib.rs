//! Python bindings for rust-code-analysis
//!
//! This module provides Python access to the rust-code-analysis library,
//! enabling computation of code metrics for Python, Rust, and other languages.

use pyo3::prelude::*;
use ::rust_code_analysis as rca;
use std::path::Path;

mod types;

use types::*;

/// Analyze source code and compute all metrics.
///
/// Args:
///     source: Source code as a string
///     path: File path (used for language detection and naming)
///     language: Optional language override ("python", "rust", "java", etc.)
///
/// Returns:
///     FuncSpace containing all metrics for the code
///
/// Raises:
///     ValueError: If the language cannot be determined or is unsupported
///
/// Example:
///     >>> import rust_code_analysis as rca
///     >>> result = rca.analyze("def foo(): pass", "example.py")
///     >>> print(result.metrics.cyclomatic.sum)
#[pyfunction]
#[pyo3(signature = (source, path, language=None))]
fn analyze(source: &str, path: &str, language: Option<&str>) -> PyResult<PyFuncSpace> {
    let path = Path::new(path);
    let source_bytes = source.as_bytes().to_vec();

    let lang = match language {
        Some(lang_str) => rca::get_from_ext(lang_str).ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Unsupported language: '{}'. Use supported_languages() to see available options.",
                lang_str
            ))
        })?,
        None => rca::guess_language(&source_bytes, path)
            .0
            .ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Could not determine language from file extension: '{}'",
                    path.display()
                ))
            })?,
    };

    let space = rca::get_function_spaces(&lang, source_bytes, path, None)
        .ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Failed to parse source code")
        })?;

    Ok(convert_func_space(&space))
}

/// Analyze a file from disk.
///
/// Args:
///     path: Path to the file to analyze
///     language: Optional language override
///
/// Returns:
///     FuncSpace containing all metrics
///
/// Raises:
///     IOError: If the file cannot be read
///     ValueError: If the language cannot be determined
///
/// Example:
///     >>> import rust_code_analysis as rca
///     >>> result = rca.analyze_file("src/main.py")
#[pyfunction]
#[pyo3(signature = (path, language=None))]
fn analyze_file(path: &str, language: Option<&str>) -> PyResult<PyFuncSpace> {
    let source = std::fs::read_to_string(path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to read file '{}': {}", path, e))
    })?;
    analyze(&source, path, language)
}

/// Get list of supported languages.
///
/// Returns:
///     List of supported language identifiers that can be passed to analyze()
///
/// Example:
///     >>> import rust_code_analysis as rca
///     >>> print(rca.supported_languages())
///     ['python', 'rust', 'java', 'javascript', ...]
#[pyfunction]
fn supported_languages() -> Vec<&'static str> {
    vec![
        "python",
        "rust",
        "java",
        "javascript",
        "typescript",
        "tsx",
        "kotlin",
        "cpp",
        "c",
    ]
}

/// Get language identifier from file extension.
///
/// Args:
///     extension: File extension (without dot), e.g., "py", "rs"
///
/// Returns:
///     Language name if recognized, None otherwise
///
/// Example:
///     >>> import rust_code_analysis as rca
///     >>> rca.language_from_extension("py")
///     'python'
///     >>> rca.language_from_extension("rs")
///     'rust'
#[pyfunction]
fn language_from_extension(extension: &str) -> Option<&'static str> {
    match extension {
        "py" => Some("python"),
        "rs" => Some("rust"),
        "java" => Some("java"),
        "js" | "jsx" | "mjs" => Some("javascript"),
        "ts" => Some("typescript"),
        "tsx" => Some("tsx"),
        "kt" | "kts" => Some("kotlin"),
        "cpp" | "cxx" | "cc" | "c" | "h" | "hpp" | "hxx" => Some("cpp"),
        _ => None,
    }
}

/// The rust_code_analysis Python module.
///
/// Provides code metrics computation using tree-sitter parsing.
/// Supports Python, Rust, Java, JavaScript, TypeScript, Kotlin, and C/C++.
///
/// Main functions:
///     - analyze(source, path, language=None): Analyze source code string
///     - analyze_file(path, language=None): Analyze a file from disk
///     - supported_languages(): List supported language identifiers
///     - language_from_extension(ext): Get language from file extension
///
/// Example:
///     >>> import rust_code_analysis as rca
///     >>>
///     >>> # Analyze Python code
///     >>> result = rca.analyze('''
///     ... def factorial(n):
///     ...     if n <= 1:
///     ...         return 1
///     ...     return n * factorial(n - 1)
///     ... ''', "factorial.py")
///     >>>
///     >>> # Access metrics
///     >>> print(f"Cyclomatic Complexity: {result.metrics.cyclomatic.sum}")
///     >>> print(f"Lines of Code: {result.metrics.loc.sloc}")
///     >>> print(f"Maintainability Index: {result.metrics.mi.mi_visual_studio}")
///     >>>
///     >>> # Get function-level metrics
///     >>> for func in result.get_functions():
///     ...     print(f"{func.name}: CC={func.metrics.cyclomatic.sum}")
#[pymodule]
fn rust_code_analysis(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(analyze, m)?)?;
    m.add_function(wrap_pyfunction!(analyze_file, m)?)?;
    m.add_function(wrap_pyfunction!(supported_languages, m)?)?;
    m.add_function(wrap_pyfunction!(language_from_extension, m)?)?;

    // Register all classes
    m.add_class::<PySpaceKind>()?;
    m.add_class::<PyFuncSpace>()?;
    m.add_class::<PyCodeMetrics>()?;
    m.add_class::<PyCyclomaticMetrics>()?;
    m.add_class::<PyCognitiveMetrics>()?;
    m.add_class::<PyHalsteadMetrics>()?;
    m.add_class::<PyLocMetrics>()?;
    m.add_class::<PyMaintainabilityIndex>()?;
    m.add_class::<PyAbcMetrics>()?;
    m.add_class::<PyNomMetrics>()?;
    m.add_class::<PyNargsMetrics>()?;
    m.add_class::<PyNexitsMetrics>()?;
    m.add_class::<PyWmcMetrics>()?;
    m.add_class::<PyNpmMetrics>()?;
    m.add_class::<PyNpaMetrics>()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_python() {
        let source = "def foo():\n    pass";
        let result = analyze(source, "test.py", None).unwrap();
        assert!(result.metrics.nom.functions >= 1.0);
    }

    #[test]
    fn test_analyze_rust() {
        let source = "fn main() { }";
        let result = analyze(source, "test.rs", None).unwrap();
        assert!(result.metrics.nom.functions >= 1.0);
    }

    #[test]
    fn test_language_detection() {
        assert_eq!(language_from_extension("py"), Some("python"));
        assert_eq!(language_from_extension("rs"), Some("rust"));
        assert_eq!(language_from_extension("unknown"), None);
    }
}
