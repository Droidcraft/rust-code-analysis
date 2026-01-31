use pyo3::prelude::*;
use ::rust_code_analysis::{self as rca, FuncSpace, SpaceKind};

/// Space kind enum - the type of code space being analyzed
#[pyclass(eq, eq_int)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PySpaceKind {
    Unknown = 0,
    Function = 1,
    Class = 2,
    Struct = 3,
    Trait = 4,
    Impl = 5,
    Unit = 6,
    Namespace = 7,
    Interface = 8,
}

impl From<SpaceKind> for PySpaceKind {
    fn from(kind: SpaceKind) -> Self {
        match kind {
            SpaceKind::Unknown => PySpaceKind::Unknown,
            SpaceKind::Function => PySpaceKind::Function,
            SpaceKind::Class => PySpaceKind::Class,
            SpaceKind::Struct => PySpaceKind::Struct,
            SpaceKind::Trait => PySpaceKind::Trait,
            SpaceKind::Impl => PySpaceKind::Impl,
            SpaceKind::Unit => PySpaceKind::Unit,
            SpaceKind::Namespace => PySpaceKind::Namespace,
            SpaceKind::Interface => PySpaceKind::Interface,
        }
    }
}

/// Cyclomatic complexity metrics
#[pyclass(get_all)]
#[derive(Clone, Debug)]
pub struct PyCyclomaticMetrics {
    pub sum: f64,
    pub average: f64,
    pub min: f64,
    pub max: f64,
}

impl From<&rca::cyclomatic::Stats> for PyCyclomaticMetrics {
    fn from(stats: &rca::cyclomatic::Stats) -> Self {
        PyCyclomaticMetrics {
            sum: stats.cyclomatic_sum(),
            average: stats.cyclomatic_average(),
            min: stats.cyclomatic_min(),
            max: stats.cyclomatic_max(),
        }
    }
}

#[pymethods]
impl PyCyclomaticMetrics {
    fn __repr__(&self) -> String {
        format!(
            "CyclomaticMetrics(sum={}, average={:.2}, min={}, max={})",
            self.sum, self.average, self.min, self.max
        )
    }
}

/// Cognitive complexity metrics
#[pyclass(get_all)]
#[derive(Clone, Debug)]
pub struct PyCognitiveMetrics {
    pub sum: f64,
    pub average: f64,
    pub min: f64,
    pub max: f64,
}

impl From<&rca::cognitive::Stats> for PyCognitiveMetrics {
    fn from(stats: &rca::cognitive::Stats) -> Self {
        PyCognitiveMetrics {
            sum: stats.cognitive_sum(),
            average: stats.cognitive_average(),
            min: stats.cognitive_min(),
            max: stats.cognitive_max(),
        }
    }
}

#[pymethods]
impl PyCognitiveMetrics {
    fn __repr__(&self) -> String {
        format!(
            "CognitiveMetrics(sum={}, average={:.2}, min={}, max={})",
            self.sum, self.average, self.min, self.max
        )
    }
}

/// Halstead software science metrics
#[pyclass(get_all)]
#[derive(Clone, Debug)]
pub struct PyHalsteadMetrics {
    /// η1 - number of distinct operators
    pub n1: f64,
    /// N1 - total number of operators
    pub big_n1: f64,
    /// η2 - number of distinct operands
    pub n2: f64,
    /// N2 - total number of operands
    pub big_n2: f64,
    /// Program length (N1 + N2)
    pub length: f64,
    /// Estimated program length
    pub estimated_program_length: f64,
    /// Purity ratio
    pub purity_ratio: f64,
    /// Program vocabulary (n1 + n2)
    pub vocabulary: f64,
    /// Program volume (bits)
    pub volume: f64,
    /// Difficulty to understand/write
    pub difficulty: f64,
    /// Program level (1/difficulty)
    pub level: f64,
    /// Effort to implement
    pub effort: f64,
    /// Time to implement (seconds)
    pub time: f64,
    /// Estimated number of bugs
    pub bugs: f64,
}

impl From<&rca::halstead::Stats> for PyHalsteadMetrics {
    fn from(stats: &rca::halstead::Stats) -> Self {
        PyHalsteadMetrics {
            n1: stats.u_operators(),
            big_n1: stats.operators(),
            n2: stats.u_operands(),
            big_n2: stats.operands(),
            length: stats.length(),
            estimated_program_length: stats.estimated_program_length(),
            purity_ratio: stats.purity_ratio(),
            vocabulary: stats.vocabulary(),
            volume: stats.volume(),
            difficulty: stats.difficulty(),
            level: stats.level(),
            effort: stats.effort(),
            time: stats.time(),
            bugs: stats.bugs(),
        }
    }
}

#[pymethods]
impl PyHalsteadMetrics {
    fn __repr__(&self) -> String {
        format!(
            "HalsteadMetrics(volume={:.2}, difficulty={:.2}, effort={:.2}, bugs={:.3})",
            self.volume, self.difficulty, self.effort, self.bugs
        )
    }
}

/// Lines of code metrics
#[pyclass(get_all)]
#[derive(Clone, Debug)]
pub struct PyLocMetrics {
    /// Source lines of code
    pub sloc: f64,
    /// Physical lines of code (instructions)
    pub ploc: f64,
    /// Logical lines of code (statements)
    pub lloc: f64,
    /// Comment lines of code
    pub cloc: f64,
    /// Blank lines
    pub blank: f64,
    pub sloc_average: f64,
    pub ploc_average: f64,
    pub lloc_average: f64,
    pub cloc_average: f64,
    pub blank_average: f64,
    pub sloc_min: f64,
    pub sloc_max: f64,
    pub ploc_min: f64,
    pub ploc_max: f64,
    pub lloc_min: f64,
    pub lloc_max: f64,
    pub cloc_min: f64,
    pub cloc_max: f64,
    pub blank_min: f64,
    pub blank_max: f64,
}

impl From<&rca::loc::Stats> for PyLocMetrics {
    fn from(stats: &rca::loc::Stats) -> Self {
        PyLocMetrics {
            sloc: stats.sloc(),
            ploc: stats.ploc(),
            lloc: stats.lloc(),
            cloc: stats.cloc(),
            blank: stats.blank(),
            sloc_average: stats.sloc_average(),
            ploc_average: stats.ploc_average(),
            lloc_average: stats.lloc_average(),
            cloc_average: stats.cloc_average(),
            blank_average: stats.blank_average(),
            sloc_min: stats.sloc_min(),
            sloc_max: stats.sloc_max(),
            ploc_min: stats.ploc_min(),
            ploc_max: stats.ploc_max(),
            lloc_min: stats.lloc_min(),
            lloc_max: stats.lloc_max(),
            cloc_min: stats.cloc_min(),
            cloc_max: stats.cloc_max(),
            blank_min: stats.blank_min(),
            blank_max: stats.blank_max(),
        }
    }
}

#[pymethods]
impl PyLocMetrics {
    fn __repr__(&self) -> String {
        format!(
            "LocMetrics(sloc={}, ploc={}, lloc={}, cloc={}, blank={})",
            self.sloc, self.ploc, self.lloc, self.cloc, self.blank
        )
    }
}

/// Maintainability Index metrics
#[pyclass(get_all)]
#[derive(Clone, Debug)]
pub struct PyMaintainabilityIndex {
    /// Original MI formula (can be negative)
    pub mi_original: f64,
    /// SEI (Software Engineering Institute) variant
    pub mi_sei: f64,
    /// Microsoft Visual Studio variant (0-100 scale)
    pub mi_visual_studio: f64,
}

impl From<&rca::mi::Stats> for PyMaintainabilityIndex {
    fn from(stats: &rca::mi::Stats) -> Self {
        PyMaintainabilityIndex {
            mi_original: stats.mi_original(),
            mi_sei: stats.mi_sei(),
            mi_visual_studio: stats.mi_visual_studio(),
        }
    }
}

#[pymethods]
impl PyMaintainabilityIndex {
    fn __repr__(&self) -> String {
        format!(
            "MaintainabilityIndex(original={:.2}, sei={:.2}, visual_studio={:.2})",
            self.mi_original, self.mi_sei, self.mi_visual_studio
        )
    }
}

/// ABC metric (Assignments, Branches, Conditions)
#[pyclass(get_all)]
#[derive(Clone, Debug)]
pub struct PyAbcMetrics {
    /// Number of assignments
    pub assignments: f64,
    /// Number of branches (function calls)
    pub branches: f64,
    /// Number of conditions
    pub conditions: f64,
    /// Magnitude: sqrt(A² + B² + C²)
    pub magnitude: f64,
    pub assignments_average: f64,
    pub branches_average: f64,
    pub conditions_average: f64,
    pub assignments_min: f64,
    pub assignments_max: f64,
    pub branches_min: f64,
    pub branches_max: f64,
    pub conditions_min: f64,
    pub conditions_max: f64,
}

impl From<&rca::abc::Stats> for PyAbcMetrics {
    fn from(stats: &rca::abc::Stats) -> Self {
        PyAbcMetrics {
            assignments: stats.assignments_sum(),
            branches: stats.branches_sum(),
            conditions: stats.conditions_sum(),
            magnitude: stats.magnitude_sum(),
            assignments_average: stats.assignments_average(),
            branches_average: stats.branches_average(),
            conditions_average: stats.conditions_average(),
            assignments_min: stats.assignments_min(),
            assignments_max: stats.assignments_max(),
            branches_min: stats.branches_min(),
            branches_max: stats.branches_max(),
            conditions_min: stats.conditions_min(),
            conditions_max: stats.conditions_max(),
        }
    }
}

#[pymethods]
impl PyAbcMetrics {
    fn __repr__(&self) -> String {
        format!(
            "AbcMetrics(A={}, B={}, C={}, magnitude={:.2})",
            self.assignments, self.branches, self.conditions, self.magnitude
        )
    }
}

/// Number of Methods metrics
#[pyclass(get_all)]
#[derive(Clone, Debug)]
pub struct PyNomMetrics {
    /// Number of functions
    pub functions: f64,
    /// Number of closures
    pub closures: f64,
    /// Total (functions + closures)
    pub total: f64,
    pub functions_average: f64,
    pub closures_average: f64,
    pub average: f64,
    pub functions_min: f64,
    pub functions_max: f64,
    pub closures_min: f64,
    pub closures_max: f64,
}

impl From<&rca::nom::Stats> for PyNomMetrics {
    fn from(stats: &rca::nom::Stats) -> Self {
        PyNomMetrics {
            functions: stats.functions_sum(),
            closures: stats.closures_sum(),
            total: stats.total(),
            functions_average: stats.functions_average(),
            closures_average: stats.closures_average(),
            average: stats.average(),
            functions_min: stats.functions_min(),
            functions_max: stats.functions_max(),
            closures_min: stats.closures_min(),
            closures_max: stats.closures_max(),
        }
    }
}

#[pymethods]
impl PyNomMetrics {
    fn __repr__(&self) -> String {
        format!(
            "NomMetrics(functions={}, closures={}, total={})",
            self.functions, self.closures, self.total
        )
    }
}

/// Number of Arguments metrics
#[pyclass(get_all)]
#[derive(Clone, Debug)]
pub struct PyNargsMetrics {
    /// Total function arguments
    pub total_functions: f64,
    /// Total closure arguments
    pub total_closures: f64,
    /// Average function arguments
    pub average_functions: f64,
    /// Average closure arguments
    pub average_closures: f64,
    /// Total arguments (functions + closures)
    pub total: f64,
    /// Overall average
    pub average: f64,
    pub functions_min: f64,
    pub functions_max: f64,
    pub closures_min: f64,
    pub closures_max: f64,
}

impl From<&rca::nargs::Stats> for PyNargsMetrics {
    fn from(stats: &rca::nargs::Stats) -> Self {
        PyNargsMetrics {
            total_functions: stats.fn_args_sum(),
            total_closures: stats.closure_args_sum(),
            average_functions: stats.fn_args_average(),
            average_closures: stats.closure_args_average(),
            total: stats.nargs_total(),
            average: stats.nargs_average(),
            functions_min: stats.fn_args_min(),
            functions_max: stats.fn_args_max(),
            closures_min: stats.closure_args_min(),
            closures_max: stats.closure_args_max(),
        }
    }
}

#[pymethods]
impl PyNargsMetrics {
    fn __repr__(&self) -> String {
        format!(
            "NargsMetrics(total={}, average={:.2})",
            self.total, self.average
        )
    }
}

/// Number of Exit Points metrics
#[pyclass(get_all)]
#[derive(Clone, Debug)]
pub struct PyNexitsMetrics {
    pub sum: f64,
    pub average: f64,
    pub min: f64,
    pub max: f64,
}

impl From<&rca::exit::Stats> for PyNexitsMetrics {
    fn from(stats: &rca::exit::Stats) -> Self {
        PyNexitsMetrics {
            sum: stats.exit_sum(),
            average: stats.exit_average(),
            min: stats.exit_min(),
            max: stats.exit_max(),
        }
    }
}

#[pymethods]
impl PyNexitsMetrics {
    fn __repr__(&self) -> String {
        format!(
            "NexitsMetrics(sum={}, average={:.2})",
            self.sum, self.average
        )
    }
}

/// Weighted Methods per Class metrics (OO-specific)
#[pyclass(get_all)]
#[derive(Clone, Debug)]
pub struct PyWmcMetrics {
    /// Sum of CC for all methods in classes
    pub classes: f64,
    /// Sum of CC for all methods in interfaces
    pub interfaces: f64,
    /// Total WMC
    pub total: f64,
}

impl From<&rca::wmc::Stats> for PyWmcMetrics {
    fn from(stats: &rca::wmc::Stats) -> Self {
        PyWmcMetrics {
            classes: stats.class_wmc_sum(),
            interfaces: stats.interface_wmc_sum(),
            total: stats.total_wmc(),
        }
    }
}

#[pymethods]
impl PyWmcMetrics {
    fn __repr__(&self) -> String {
        format!(
            "WmcMetrics(classes={}, interfaces={}, total={})",
            self.classes, self.interfaces, self.total
        )
    }
}

/// Number of Public Methods metrics (OO-specific)
#[pyclass(get_all)]
#[derive(Clone, Debug)]
pub struct PyNpmMetrics {
    pub classes: f64,
    pub interfaces: f64,
    pub total: f64,
}

impl From<&rca::npm::Stats> for PyNpmMetrics {
    fn from(stats: &rca::npm::Stats) -> Self {
        PyNpmMetrics {
            classes: stats.class_npm_sum(),
            interfaces: stats.interface_npm_sum(),
            total: stats.total_npm(),
        }
    }
}

#[pymethods]
impl PyNpmMetrics {
    fn __repr__(&self) -> String {
        format!(
            "NpmMetrics(classes={}, interfaces={}, total={})",
            self.classes, self.interfaces, self.total
        )
    }
}

/// Number of Public Attributes metrics (OO-specific)
#[pyclass(get_all)]
#[derive(Clone, Debug)]
pub struct PyNpaMetrics {
    pub classes: f64,
    pub interfaces: f64,
    pub total: f64,
}

impl From<&rca::npa::Stats> for PyNpaMetrics {
    fn from(stats: &rca::npa::Stats) -> Self {
        PyNpaMetrics {
            classes: stats.class_npa_sum(),
            interfaces: stats.interface_npa_sum(),
            total: stats.total_npa(),
        }
    }
}

#[pymethods]
impl PyNpaMetrics {
    fn __repr__(&self) -> String {
        format!(
            "NpaMetrics(classes={}, interfaces={}, total={})",
            self.classes, self.interfaces, self.total
        )
    }
}

/// Aggregate of all code metrics for a space
#[pyclass(get_all)]
#[derive(Clone, Debug)]
pub struct PyCodeMetrics {
    pub cyclomatic: PyCyclomaticMetrics,
    pub cognitive: PyCognitiveMetrics,
    pub halstead: PyHalsteadMetrics,
    pub loc: PyLocMetrics,
    pub mi: PyMaintainabilityIndex,
    pub abc: PyAbcMetrics,
    pub nom: PyNomMetrics,
    pub nargs: PyNargsMetrics,
    pub nexits: PyNexitsMetrics,
    pub wmc: PyWmcMetrics,
    pub npm: PyNpmMetrics,
    pub npa: PyNpaMetrics,
}

impl From<&rca::CodeMetrics> for PyCodeMetrics {
    fn from(metrics: &rca::CodeMetrics) -> Self {
        PyCodeMetrics {
            cyclomatic: (&metrics.cyclomatic).into(),
            cognitive: (&metrics.cognitive).into(),
            halstead: (&metrics.halstead).into(),
            loc: (&metrics.loc).into(),
            mi: (&metrics.mi).into(),
            abc: (&metrics.abc).into(),
            nom: (&metrics.nom).into(),
            nargs: (&metrics.nargs).into(),
            nexits: (&metrics.nexits).into(),
            wmc: (&metrics.wmc).into(),
            npm: (&metrics.npm).into(),
            npa: (&metrics.npa).into(),
        }
    }
}

#[pymethods]
impl PyCodeMetrics {
    fn __repr__(&self) -> String {
        format!(
            "CodeMetrics(cc={}, cognitive={}, sloc={}, mi={:.2})",
            self.cyclomatic.sum, self.cognitive.sum, self.loc.sloc, self.mi.mi_visual_studio
        )
    }
}

/// A function space containing metrics and nested spaces
#[pyclass]
#[derive(Clone, Debug)]
pub struct PyFuncSpace {
    #[pyo3(get)]
    pub name: Option<String>,
    #[pyo3(get)]
    pub start_line: usize,
    #[pyo3(get)]
    pub end_line: usize,
    #[pyo3(get)]
    pub kind: PySpaceKind,
    #[pyo3(get)]
    pub metrics: PyCodeMetrics,
    spaces: Vec<PyFuncSpace>,
}

#[pymethods]
impl PyFuncSpace {
    /// Get nested spaces (functions, classes, etc.)
    #[getter]
    fn spaces(&self) -> Vec<PyFuncSpace> {
        self.spaces.clone()
    }

    /// Recursively collect all function spaces
    fn get_functions(&self) -> Vec<PyFuncSpace> {
        let mut result = Vec::new();
        self.collect_by_kind(&mut result, PySpaceKind::Function);
        result
    }

    /// Recursively collect all class spaces
    fn get_classes(&self) -> Vec<PyFuncSpace> {
        let mut result = Vec::new();
        self.collect_by_kind(&mut result, PySpaceKind::Class);
        result
    }

    /// Recursively collect all spaces of any kind
    fn get_all_spaces(&self) -> Vec<PyFuncSpace> {
        let mut result = Vec::new();
        self.collect_all(&mut result);
        result
    }

    fn __repr__(&self) -> String {
        format!(
            "FuncSpace(name={:?}, kind={:?}, lines={}-{}, cc={})",
            self.name,
            self.kind,
            self.start_line,
            self.end_line,
            self.metrics.cyclomatic.sum
        )
    }
}

impl PyFuncSpace {
    fn collect_by_kind(&self, result: &mut Vec<PyFuncSpace>, kind: PySpaceKind) {
        if self.kind == kind {
            result.push(self.clone());
        }
        for space in &self.spaces {
            space.collect_by_kind(result, kind);
        }
    }

    fn collect_all(&self, result: &mut Vec<PyFuncSpace>) {
        result.push(self.clone());
        for space in &self.spaces {
            space.collect_all(result);
        }
    }
}

/// Convert from rust-code-analysis FuncSpace to PyFuncSpace
pub fn convert_func_space(space: &FuncSpace) -> PyFuncSpace {
    PyFuncSpace {
        name: space.name.clone(),
        start_line: space.start_line,
        end_line: space.end_line,
        kind: space.kind.into(),
        metrics: (&space.metrics).into(),
        spaces: space.spaces.iter().map(convert_func_space).collect(),
    }
}
