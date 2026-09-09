//! Per-node evaluation results: `Evaluation`, `Value`, `Body`.
//!
//! # The ValuePayload exposure inventory (a reported FORK)
//!
//! `ValuePayload` has seven variants. The bindings project them as:
//!
//! | variant        | exposure |
//! |----------------|----------|
//! | `Body`         | full — opaque handle + `mass_properties` / `validate` doors |
//! | `Boolean`      | full — unwraps to a `Body`, or `None` when empty |
//! | `Split`        | full — `above` / `below` as optional bodies |
//! | `Instances`    | full — a list of bodies |
//! | `Datum`        | full — typed plane / axis / point with `Length` coordinates |
//! | `Profile`      | KIND ONLY — sketch geometry does not ship to Python before the v2 switch |
//! | `Declarations` | KIND ONLY — the naming projection is deferred, not blocked |
//!
//! The two kind-only rows are SCOPE decisions, not capability limits.
//! Being precise about which, because the distinction is load-bearing:
//!
//! * `ValidatedProfile::plane()`/`loops()` DO exist and `profile` is
//!   wholesale re-exported — this very module's sibling uses
//!   `pncad::profile` to build sketches. Projecting a profile back to
//!   Python is therefore perfectly possible; it is **ruled out**:
//!   Python never ships the opaque-profile intermediate state.
//!   Sketch read-back belongs with the v2 program representation.
//! * `StableName` is likewise prelude-curated with public fields, so
//!   Declarations is reachable too. It is deferred because the
//!   naming/selection projection is a design subject of its own, and
//!   binding a provisional shape here would fork it.

use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::errors::ErrorClass;
use crate::py::quantity::Length;
use crate::py::{doc::NodeId, typed_err};
use crate::tags::{
    NODE_NOT_EVALUATED, export_error_tag, node_error_tag, node_inner_kind_tag,
    normalization_kind_tag, promoted_curve_kind_tag, promoted_kind_tag, step_import_error_tag,
};
use crate::validation;
use pncad::document as d;
use pncad::tolerance::Tol;
use pncad::topo;

/// The refusing ARM's word, as the exception carries it: the kernel
/// refusal's own discriminant beside the carrier's, `None` where the
/// refusal has no arms.
fn inner_kind(py: Python<'_>, kind: &d::NodeErrorKind) -> Py<PyAny> {
    match node_inner_kind_tag(kind) {
        Some(tag) => PyString::new(py, tag).unbind().into_any(),
        None => py.None().into_any(),
    }
}

/// Raise `EvaluationError` with a stable `reason` tag.
///
/// `kind`, `inner_kind`, `through` and `finding` are ALWAYS present on
/// the exception — `None` where the reason has no failing kind, no
/// arm under that kind, no poisoning ancestor, or no refusal-menu
/// payload — so stub-guided code can read them without an
/// `AttributeError` trap — a stub that over-promises is worse than one
/// that says `None`.
fn eval_err(py: Python<'_>, message: impl Into<String>, reason: &str, node: NodeId) -> PyErr {
    let node = match node.into_pyobject(py) {
        Ok(bound) => bound.unbind().into_any(),
        // A `#[pyclass]` conversion fails as a `PyErr` already —
        // surface it as the raise rather than losing it.
        Err(failed) => return failed,
    };
    typed_err(
        py,
        ErrorClass::Evaluation,
        message,
        &[
            ("reason", PyString::new(py, reason).unbind().into_any()),
            ("node", node),
            ("kind", py.None().into_any()),
            ("inner_kind", py.None().into_any()),
            ("through", py.None().into_any()),
            ("finding", py.None().into_any()),
        ],
    )
}

/// Raise `EvaluationError` for a node that ITSELF failed: the payload
/// is the `NodeErrorKind`'s stable tag plus the node id; the message
/// is the kernel error's own `Display` prose — never a `Debug` dump.
///
/// `kind` is the CARRIER's word — which door refused — and `inner_kind`
/// is the arm of the kernel refusal that door holds, `None` where that
/// refusal has no arms. Two enums, two discriminants, each projected
/// where it lives.
fn node_failure(py: Python<'_>, node: NodeId, error: &d::NodeError) -> PyErr {
    let node_obj = match node.into_pyobject(py) {
        Ok(bound) => bound.unbind().into_any(),
        Err(failed) => return failed,
    };
    // The refusal MENU: an undeclared-contact
    // refusal carries its candidate declaration as a typed
    // `FlushFinding` on the exception — the same value shape
    // `Evaluation.find_flush_candidates` answers with, ready for
    // `Node.declare`/`Doc.declare`. `None` on every other kind.
    let finding = match &error.kind {
        d::NodeErrorKind::UndeclaredContact { finding, .. } => {
            match super::flush::FlushFinding((**finding).clone()).into_pyobject(py) {
                Ok(bound) => bound.unbind().into_any(),
                Err(failed) => return failed,
            }
        }
        _ => py.None().into_any(),
    };
    typed_err(
        py,
        ErrorClass::Evaluation,
        error.to_string(),
        &[
            (
                "reason",
                PyString::new(py, "node_failed").unbind().into_any(),
            ),
            ("node", node_obj),
            (
                "kind",
                PyString::new(py, node_error_tag(&error.kind))
                    .unbind()
                    .into_any(),
            ),
            ("inner_kind", inner_kind(py, &error.kind)),
            ("through", py.None().into_any()),
            ("finding", finding),
        ],
    )
}

/// Raise `EvaluationError` for a POISONED node: `through` names the
/// nearest failed ancestor, `kind` tags its root cause (present
/// whenever the evaluation's own invariant holds — fail-honest, so a
/// broken hop yields no `kind` rather than a wrong one).
fn poisoning(py: Python<'_>, node: NodeId, through: NodeId, root: Option<&d::NodeError>) -> PyErr {
    let objs = (node.into_pyobject(py), through.into_pyobject(py));
    let (node_obj, through_obj) = match objs {
        (Ok(n), Ok(t)) => (n.unbind().into_any(), t.unbind().into_any()),
        (Err(failed), _) | (_, Err(failed)) => return failed,
    };
    let mut fields: Vec<(&str, Py<PyAny>)> = vec![
        ("reason", PyString::new(py, "poisoned").unbind().into_any()),
        ("node", node_obj),
        ("through", through_obj),
        ("finding", py.None().into_any()),
    ];
    // The message is the root cause's `Display` prose: the node
    // never ran, so the honest sentence names the ancestor's problem.
    let message = match root {
        Some(error) => {
            fields.push((
                "kind",
                PyString::new(py, node_error_tag(&error.kind))
                    .unbind()
                    .into_any(),
            ));
            fields.push(("inner_kind", inner_kind(py, &error.kind)));
            format!("never ran — poisoned by failed ancestor: {error}")
        }
        None => {
            fields.push(("kind", py.None().into_any()));
            fields.push(("inner_kind", py.None().into_any()));
            format!("never ran — poisoned through node {}", through.0.0)
        }
    };
    typed_err(py, ErrorClass::Evaluation, message, &fields)
}

/// Bulk mass properties of a body, in canonical units.
///
/// Volume and area are `m³` and `m²` — dimensions OUTSIDE D6's closed
/// `{Length, Angle, Count}` set, so they cross as plain floats in
/// canonical units rather than as invented quantity types.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct MassProperties {
    /// Signed enclosed volume, m³.
    #[pyo3(get)]
    volume: f64,
    /// Total surface area, m².
    #[pyo3(get)]
    surface_area: f64,
    /// Certified half-width of the volume enclosure.
    #[pyo3(get)]
    volume_pad: f64,
    /// Certified half-width of the area enclosure.
    #[pyo3(get)]
    area_pad: f64,
}

#[pymethods]
impl MassProperties {
    fn __repr__(&self) -> String {
        format!(
            "MassProperties(volume={} m^3, surface_area={} m^2)",
            self.volume, self.surface_area
        )
    }
}

/// A solid body — an OPAQUE handle.
///
/// Arena keys never cross; a body crosses as a handle whose
/// interior is reachable only through curated doors.
///
/// # The declared contacts ride WITH the body
///
/// `contacts` is the tier-3′ second argument (M3 PR 6a): the
/// declarations the op that produced this body minted for it. It is
/// CAPTURED here rather than crossing as a value, which is the
/// carrier-projection rule at a door whose Rust signature takes two
/// things a Python caller can only hold one of. `ContactRecords` has
/// no Python constructor and is curated `INTERIOR` — so a
/// `validate_pseudomanifold(contacts)` door would be uncallable — and
/// handing one body ANOTHER body's declarations is exactly the
/// mis-pairing the F1 contract exists to refuse (the validator never
/// blesses discovered contacts). The demo tour states the same rule
/// from the other side: its `SceneBody` carries `contacts` beside the
/// body and runs 3′ "with the op's OWN declared contacts".
///
/// Empty is the honest default, not a hole. With no declarations the
/// kernel's own contract is that 3′ ≡ tier 3 plus the census actually
/// run, which is the STRICTEST rung of the ladder — so a door that
/// mints a body without records can only make this gate refuse
/// (`UndeclaredContact`), never falsely pass.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct Body {
    pub(crate) inner: Arc<topo::Body<f64>>,
    contacts: Arc<topo::ContactRecords>,
}

impl Body {
    /// A body with no declared contacts — every door but the two that
    /// have records to carry (an evaluated value, and `assemble`).
    pub(crate) fn plain(inner: Arc<topo::Body<f64>>) -> Self {
        Self {
            inner,
            contacts: Arc::new(topo::ContactRecords::default()),
        }
    }

    /// A body with the declarations its producer minted for it.
    pub(crate) fn declared(
        inner: Arc<topo::Body<f64>>,
        contacts: Arc<topo::ContactRecords>,
    ) -> Self {
        Self { inner, contacts }
    }
}

#[pymethods]
impl Body {
    /// Volume, area, and their certified pads.
    fn mass_properties(&self, py: Python<'_>) -> PyResult<MassProperties> {
        let tol = Tol::witness();
        let props = topo::mass_properties(&self.inner, tol).map_err(|err| {
            typed_err(
                py,
                ErrorClass::Validation,
                err.to_string(),
                &[(
                    "reason",
                    PyString::new(py, "mass_properties_failed")
                        .unbind()
                        .into_any(),
                )],
            )
        })?;
        Ok(MassProperties {
            volume: props.volume,
            surface_area: props.surface_area,
            volume_pad: props.volume_pad,
            area_pad: props.area_pad,
        })
    }

    /// Full validation. Raises `ValidationError` listing the failures.
    fn validate(&self, py: Python<'_>) -> PyResult<()> {
        self.run_validator(py, "validate", topo::validate(&self.inner))
    }

    /// Closure validation only.
    fn validate_closed(&self, py: Python<'_>) -> PyResult<()> {
        self.run_validator(py, "validate_closed", topo::validate_closed(&self.inner))
    }

    /// Tessellate at a chordal budget — the ladder's step 4.
    ///
    /// `chordal` is a DISTANCE: the maximum the piecewise-linear mesh
    /// may sag from the exact surface. It is deliberately not the
    /// kernel's ε, which `DocEdit.set_tolerance` sets and which
    /// decides what the model IS; this decides how coarsely a view of
    /// it may approximate it. Two budgets see the same body.
    fn tessellate(&self, py: Python<'_>, chordal: &Length) -> PyResult<super::mesh::Mesh> {
        super::mesh::tessellate(py, &self.inner, chordal)
    }

    /// Geometric validation only.
    fn validate_geometric(&self, py: Python<'_>) -> PyResult<()> {
        let tol = Tol::witness();
        self.run_validator(
            py,
            "validate_geometric",
            topo::validate_geometric(&self.inner, tol),
        )
    }

    /// **Tier 3′** — the ladder's fourth rung: tier 3's whole local
    /// battery PLUS the global coincidence census tier 3 defers,
    /// certified against THIS body's own declared contacts.
    ///
    /// The census is a diff in both directions. Every coincidence it
    /// finds must be backed by a declaration (`UndeclaredContact`
    /// otherwise — no scan-to-bless) and every declaration must be
    /// geometrically confirmed (`StaleContactDeclaration` otherwise).
    /// There is no contacts argument because a body already carries
    /// its own; see the type's docs for why that is the only pairing
    /// this door can honestly offer.
    ///
    /// So the verdict depends on which door minted the body, and that
    /// is the point rather than a wrinkle: a boolean result and an
    /// assembled product arrive with their declarations and pass over
    /// their seams; the SAME solids gathered by `product`, which
    /// declares nothing, arrive without and this gate reports the
    /// seam it finds. Raises `ValidationError` listing the failures,
    /// with `door`, `failure_count` and `findings` as on the other
    /// three rungs.
    fn validate_pseudomanifold(&self, py: Python<'_>) -> PyResult<()> {
        let tol = Tol::witness();
        self.run_validator(
            py,
            "validate_pseudomanifold",
            topo::validate_pseudomanifold(&self.inner, &self.contacts, tol),
        )
    }
}

impl Body {
    /// Shared shape for the validator doors, which all return
    /// `Result<(), Vec<ValidationError>>`.
    ///
    /// The exception carries `door`, the failure COUNT, and one
    /// [`ValidationFinding`] per failure, in the kernel's own
    /// deterministic report order. The human message is unchanged: each
    /// finding through the enum's own `Display`, one prose sentence with
    /// recourse, joined because a `Vec` has no rendering of its own —
    /// so the words are a branch a caller can take and the sentence is
    /// still the diagnosis they read.
    ///
    /// ONE raise per call, whatever the count. `failure_count` is what
    /// says the door found several, and splitting the raise would
    /// report one of N failures where the join reports all of them.
    ///
    /// It raises through [`typed_err`] like every other door: the
    /// kernel words each tier-3′ census finding through `Display`, so
    /// the joined message is prose the assertion accepts.
    fn run_validator(
        &self,
        py: Python<'_>,
        door: &str,
        outcome: Result<(), Vec<topo::ValidationError>>,
    ) -> PyResult<()> {
        let Err(failures) = outcome else {
            return Ok(());
        };
        let count = failures.len().into_pyobject(py)?.unbind().into_any();
        let findings: Vec<ValidationFinding> = failures
            .iter()
            .map(|failure| ValidationFinding(validation::project(failure)))
            .collect();
        let findings = findings.into_pyobject(py)?.unbind().into_any();
        Err(typed_err(
            py,
            ErrorClass::Validation,
            format!(
                "{door} reported {} failure(s): {}",
                failures.len(),
                failures
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
            &[
                ("door", PyString::new(py, door).unbind().into_any()),
                ("failure_count", count),
                ("findings", findings),
            ],
        ))
    }
}

/// **ONE failure a validator found**, as words a caller branches on.
///
/// The value class behind `ValidationError.findings`, and **the single
/// place on this surface where a refusal's discriminant crosses in a
/// SEQUENCE rather than as a scalar attribute**. That is argued by this
/// door's own shape and by nothing else: `validate*` is the one door
/// that reports MANY refusals at once — `failure_count` has said so
/// since it was bound — so one `variant` string could only name one of
/// them. Everywhere else a refusal reports a single fault and its word
/// is a plain attribute; read this as the exception it is, not as a
/// second convention.
///
/// Frozen, constructed only by the binding, and compared by value: two
/// findings that say the same thing are `==`.
///
/// `variant` is which `ValidationError` arm refused. The other five
/// are its payload, `None` on an arm that carries none, so `getattr`
/// never raises and a caller never has to branch on `variant` first:
///
/// * `subject_kind` — what a census refusal is ABOUT: `"entity"` (one
///   carrier outside the certifiable inventory) or `"face_pair"` (a
///   candidate contact). The two are two different repairs.
/// * `entity_kind` — that entity's kind (`"face"`, `"edge"`,
///   `"vertex"`, …); `None` for a pair, whose sides are faces.
/// * `contact_kind` — which coincidence the tier-3′ census found
///   (`"vertex_on_face"`, `"edge_edge_cross"`, …). The branch that
///   matters: an `"edge_face_pierce"` is interpenetration and cannot
///   be declared, while an `"edge_edge_overlap"` can be.
/// * `stale_kind` — which declared record the census could not
///   confirm (`"vertex_vertex"`, `"vertex_on_face"`, `"curve_locus"`,
///   `"patch"`). The granularity is which record to withdraw or
///   re-seat; withdrawing another one leaves the refusal standing.
/// * `ring_contact_kind` — how a ring meets its face's own outer loop
///   (`"vertex_vertex"`, `"vertex_on_edge"`, `"edge_along_edge"`).
///   The word says where the ring has to move: a shared position one
///   vertex clears, or a shared arc no single move separates.
///
/// **No arena key crosses**, here as everywhere: a `Body` is an opaque
/// handle, so WHICH face or vertex a finding names stays in the
/// kernel's own prose on the message, and these words are what a
/// caller acts on.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct ValidationFinding(validation::Finding);

#[pymethods]
impl ValidationFinding {
    /// Which `ValidationError` arm refused.
    #[getter]
    fn variant(&self) -> &'static str {
        self.0.variant
    }

    /// What a census refusal is about: `"entity"` or `"face_pair"`.
    #[getter]
    fn subject_kind(&self) -> Option<&'static str> {
        self.0.subject_kind
    }

    /// The entity kind of an `"entity"` subject.
    #[getter]
    fn entity_kind(&self) -> Option<&'static str> {
        self.0.entity_kind
    }

    /// Which coincidence the census found.
    #[getter]
    fn contact_kind(&self) -> Option<&'static str> {
        self.0.contact_kind
    }

    /// Which declared record lost its witness.
    #[getter]
    fn stale_kind(&self) -> Option<&'static str> {
        self.0.stale_kind
    }

    /// How a ring meets its face's own outer loop.
    #[getter]
    fn ring_contact_kind(&self) -> Option<&'static str> {
        self.0.ring_contact_kind
    }

    fn __repr__(&self) -> String {
        // Python's own spelling of an absent word, not Rust's: a repr
        // a reader can paste back is the whole point of one.
        fn word(value: Option<&str>) -> String {
            value.map_or_else(|| "None".to_owned(), |word| format!("'{word}'"))
        }
        format!(
            "ValidationFinding(variant='{}', subject_kind={}, \
             entity_kind={}, contact_kind={}, stale_kind={}, \
             ring_contact_kind={})",
            self.0.variant,
            word(self.0.subject_kind),
            word(self.0.entity_kind),
            word(self.0.contact_kind),
            word(self.0.stale_kind),
            word(self.0.ring_contact_kind)
        )
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    /// Consistent with [`Self::__eq__`]: the six words ARE the value,
    /// so hashing them hashes exactly what equality compares.
    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::hash::DefaultHasher::new();
        (
            self.0.variant,
            self.0.subject_kind,
            self.0.entity_kind,
            self.0.contact_kind,
            self.0.stale_kind,
            self.0.ring_contact_kind,
        )
            .hash(&mut hasher);
        hasher.finish()
    }
}

/// A datum: a construction plane, frame, axis, or point.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct Datum {
    /// `"plane"`, `"frame"`, `"axis"`, `"axis_in_plane"`, or
    /// `"point"`.
    #[pyo3(get)]
    kind: &'static str,
    /// Plane/frame/axis origin, or the point's position, as metres.
    #[pyo3(get)]
    origin: (Length, Length, Length),
    /// Plane normal, frame normal (x̂ × ŷ), or axis direction; `None`
    /// for a point.
    #[pyo3(get)]
    direction: Option<(f64, f64, f64)>,
    /// An in-plane axis in its frame's own 2-D coordinates — the
    /// origin then the direction, as authored. `None` for every other
    /// kind, whose numbers are all world numbers.
    ///
    /// The two halves cross differently because they ARE different
    /// things. The origin is a POSITION — a distance from the frame's
    /// own origin, measured in metres — so it crosses dimensioned, as
    /// `origin` does and as `Node.datum_axis_in_plane` takes it. The
    /// direction is dimensionless and crosses bare, which is the
    /// placement vocabulary's rule: a bare float appears only where
    /// the Rust side is itself a direction or a matrix entry. Being
    /// written in a frame's coordinates rather than the world's
    /// changes the DATUM a position is measured from, never its
    /// dimension.
    #[pyo3(get)]
    in_plane: Option<((Length, Length), (f64, f64))>,
    /// A frame's sketch +x and +y axes, unit and perpendicular; `None`
    /// for every other kind.
    ///
    /// The pair rides ALONGSIDE `direction` rather than replacing it:
    /// a reader asking which way a datum faces gets the same answer
    /// for a frame as for a plane, and one asking how the frame is
    /// TURNED — the datum a plane does not carry — reads the axes.
    //
    // The tuple is spelled out rather than hidden behind a
    // `type SketchAxes = ((f64, f64, f64), (f64, f64, f64))`, and the
    // reason is that this is a `#[pyo3(get)]` projection: the written
    // shape IS the object Python receives, and `pncad.pyi` states it
    // literally as
    // `Optional[tuple[tuple[float, float, float], tuple[float, float, float]]]`.
    // An alias would name the pair on the Rust side while the thing the
    // stub records stayed a bare nested tuple on the Python side, so a
    // reader checking the binding against the stub would have to chase
    // the alias to learn nothing new. The two fields above it —
    // `direction`'s triple and `in_plane`'s pair of pairs — are literal
    // for the same reason and stay under clippy's threshold; naming
    // only the third would make three projections of one kind read as
    // two.
    #[allow(clippy::type_complexity)] // the tuple IS the Python-side contract; see above
    #[pyo3(get)]
    axes: Option<((f64, f64, f64), (f64, f64, f64))>,
}

#[pymethods]
impl Datum {
    fn __repr__(&self) -> String {
        format!("Datum({})", self.kind)
    }
}

/// **A measured quantity** (ERROR-DESIGN E3): the value a `Measure`
/// node evaluated to, with the F1 dimension it was measured in.
///
/// The dimension rides the measure rather than the reader inferring it
/// — that is E3's claim, and a Python consumer gets it as data. The
/// value is reported in canonical kernel units (metres for a `Length`,
/// radians for an `Angle`); a `Length` measure additionally answers
/// through the typed quantity, which is the spelling the rest of this
/// surface uses.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct Measurement {
    /// `"Length"`, `"Angle"`, `"Count"` or `"Scalar"`.
    #[pyo3(get)]
    dimension: &'static str,
    /// The measured value in canonical kernel units.
    #[pyo3(get)]
    value: f64,
    /// The value as a typed `Length`; `None` for every other
    /// dimension, because a `Length` is what a length is.
    #[pyo3(get)]
    length: Option<Length>,
}

#[pymethods]
impl Measurement {
    fn __repr__(&self) -> String {
        format!("Measurement({} {})", self.value, self.dimension)
    }
}

/// **An assertion's verdict** (ERROR-DESIGN E10), REPORT-ONLY.
///
/// Three states, kept three: `holds` is `True`, `False`, or `None`
/// where the run's tolerance could not separate the measurement from
/// the bound. Nothing collapses the third into a silent pass — that is
/// the whole reason the state exists.
///
/// `measured` and `bound` are present for a decided verdict and `None`
/// for an undecided one. Reading a verdict changes nothing: a failing
/// assertion gates no build and moves no product (E10 v1).
#[pyclass(frozen, module = "pncad")]
pub(crate) struct Verdict {
    /// `"Holds"`, `"Violated"` or `"Unevaluated"`.
    #[pyo3(get)]
    status: &'static str,
    /// `True`/`False` for a decided verdict, `None` for an undecided
    /// one.
    #[pyo3(get)]
    holds: Option<bool>,
    /// What the measure evaluated to, in canonical kernel units.
    #[pyo3(get)]
    measured: Option<f64>,
    /// What the bound evaluated to, in canonical kernel units.
    #[pyo3(get)]
    bound: Option<f64>,
    /// Why there is no verdict; `None` when there is one.
    #[pyo3(get)]
    reason: Option<String>,
}

#[pymethods]
impl Verdict {
    fn __repr__(&self) -> String {
        match (self.measured, self.bound) {
            (Some(m), Some(b)) => format!("Verdict({}: {m} vs {b})", self.status),
            _ => format!("Verdict({})", self.status),
        }
    }
}

/// The F1 dimension as the one spelling this surface uses.
///
/// Capitalized on purpose: this is the Python-facing type name a
/// `Measurement` repr reads back as, not prose. The other two
/// spellings of the same word list are the kernel's prose rendering
/// (`Dimension`'s `Display`, lowercase) and `errors::dimension_tag`
/// (the lowercase FFI tag, pinned equal to that rendering).
fn dimension_name(dim: d::Dimension) -> &'static str {
    match dim {
        d::Dimension::Length => "Length",
        d::Dimension::Angle => "Angle",
        d::Dimension::Count => "Count",
        d::Dimension::Scalar => "Scalar",
    }
}

/// Project a canonical-metre point into typed `Length`s.
pub(crate) fn lengths(p: pncad::geom_core::Point3<f64>) -> (Length, Length, Length) {
    (
        Length(pncad::quantity::Length::from_meters(p.x)),
        Length(pncad::quantity::Length::from_meters(p.y)),
        Length(pncad::quantity::Length::from_meters(p.z)),
    )
}

/// Project a materializer's names into their canonical text — the one
/// alphabet `Node.fillet` reads (see `doc::name_text`).
fn names(py: Python<'_>, found: Vec<pncad::prelude::StableName>) -> PyResult<Vec<String>> {
    found.iter().map(|n| super::doc::name_text(py, n)).collect()
}

/// A node's successful value.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct Value {
    payload: d::ValuePayload<f64>,
    /// The value's OWN declared-contact channel (ASM-R2b D-1): what
    /// `instantiate` carried in from a part. A boolean's records ride
    /// its payload instead, and the two homes reconcile at
    /// [`Value::declared_body`] — deliberately the same reconciliation
    /// `editor_core::product::sources_of` makes, because a Python
    /// caller reading a body off a value and the gather reading the
    /// same body must not disagree about what was declared over it.
    contacts: Arc<topo::ContactRecords>,
    node: NodeId,
}

impl Value {
    /// One of this value's bodies with the declarations that body
    /// carries — `sources_of`'s rule, restated at the boundary.
    ///
    /// Multi-output payloads carry no records (the `OpOut` invariant:
    /// "output body 0" names nothing there), so `Instances` and
    /// `Split` halves are plain — no home to read from, and none is
    /// invented.
    fn declared_body(&self, body: &Arc<topo::Body<f64>>) -> Body {
        match &self.payload {
            d::ValuePayload::Body(_) => {
                Body::declared(Arc::clone(body), Arc::clone(&self.contacts))
            }
            d::ValuePayload::Boolean(d::BooleanValue::Body { contacts, .. }) => {
                Body::declared(Arc::clone(body), Arc::clone(contacts))
            }
            _ => Body::plain(Arc::clone(body)),
        }
    }
}

#[pymethods]
impl Value {
    /// The payload's kind tag, straight from the kernel's own
    /// `ValuePayload::kind_name` — so the Python tag set cannot drift
    /// from the document layer's.
    #[getter]
    fn kind(&self) -> &'static str {
        self.payload.kind_name()
    }

    /// The single body this value denotes.
    ///
    /// Accepts `Body` and a non-empty `Boolean`; raises for every
    /// other kind, and for an empty Boolean.
    fn body(&self, py: Python<'_>) -> PyResult<Body> {
        match &self.payload {
            d::ValuePayload::Body(body) => Ok(self.declared_body(body)),
            d::ValuePayload::Boolean(d::BooleanValue::Body { body, .. }) => {
                Ok(self.declared_body(body))
            }
            d::ValuePayload::Boolean(d::BooleanValue::Empty) => Err(eval_err(
                py,
                "the Boolean produced an empty result",
                "empty_boolean",
                self.node,
            )),
            other => Err(eval_err(
                py,
                format!("a `{}` value is not a body", other.kind_name()),
                "wrong_kind",
                self.node,
            )),
        }
    }

    /// Every body this value denotes: one for `Body`/`Boolean`, the
    /// whole list for `Instances`, both sides for a `Split`.
    fn bodies(&self) -> Vec<Body> {
        let wrap = |body: &Arc<topo::Body<f64>>| self.declared_body(body);
        let side = |s: &d::SplitSide<f64>| match s {
            d::SplitSide::Body(body) => Some(wrap(body)),
            d::SplitSide::Empty => None,
        };
        match &self.payload {
            d::ValuePayload::Body(body) => vec![wrap(body)],
            d::ValuePayload::Boolean(d::BooleanValue::Body { body, .. }) => vec![wrap(body)],
            d::ValuePayload::Instances(bodies) => bodies.iter().map(wrap).collect(),
            d::ValuePayload::Split { above, below } => {
                [side(above), side(below)].into_iter().flatten().collect()
            }
            _ => Vec::new(),
        }
    }

    /// A split's two sides, `(above, below)`; `None` where empty.
    fn split(&self, py: Python<'_>) -> PyResult<(Option<Body>, Option<Body>)> {
        let side = |s: &d::SplitSide<f64>| match s {
            d::SplitSide::Body(body) => Some(Body::plain(Arc::clone(body))),
            d::SplitSide::Empty => None,
        };
        match &self.payload {
            d::ValuePayload::Split { above, below } => Ok((side(above), side(below))),
            other => Err(eval_err(
                py,
                format!("a `{}` value is not a split", other.kind_name()),
                "wrong_kind",
                self.node,
            )),
        }
    }

    /// The datum this value denotes.
    fn datum(&self, py: Python<'_>) -> PyResult<Datum> {
        match &self.payload {
            d::ValuePayload::Datum(d::DatumValue::Plane { origin, normal }) => {
                let n = normal.get();
                Ok(Datum {
                    kind: "plane",
                    origin: lengths(*origin),
                    direction: Some((n.x, n.y, n.z)),
                    in_plane: None,
                    axes: None,
                })
            }
            d::ValuePayload::Datum(d::DatumValue::Axis { origin, dir }) => {
                let v = dir.get();
                Ok(Datum {
                    kind: "axis",
                    origin: lengths(*origin),
                    direction: Some((v.x, v.y, v.z)),
                    in_plane: None,
                    axes: None,
                })
            }
            d::ValuePayload::Datum(d::DatumValue::Point { position }) => Ok(Datum {
                kind: "point",
                origin: lengths(*position),
                direction: None,
                in_plane: None,
                axes: None,
            }),
            d::ValuePayload::Datum(d::DatumValue::Frame { origin, u, v }) => {
                let (x, y) = (u.get(), v.get());
                let n = d::DatumValue::frame_normal(*u, *v);
                Ok(Datum {
                    kind: "frame",
                    origin: lengths(*origin),
                    direction: Some((n.x, n.y, n.z)),
                    in_plane: None,
                    axes: Some(((x.x, x.y, x.z), (y.x, y.y, y.z))),
                })
            }
            // BOTH spellings reach Python: `origin`/`direction` are the
            // world line, so a reader that only wants to know where the
            // axis IS treats it like any other axis, and `in_plane`
            // carries the sketch numbers a revolve consumes.
            d::ValuePayload::Datum(d::DatumValue::AxisInPlane {
                plane_origin,
                plane_dir,
                origin,
                dir,
            }) => {
                let v = dir.get();
                Ok(Datum {
                    kind: "axis_in_plane",
                    origin: lengths(*origin),
                    direction: Some((v.x, v.y, v.z)),
                    in_plane: Some((
                        (
                            Length(pncad::quantity::Length::from_meters(plane_origin.x)),
                            Length(pncad::quantity::Length::from_meters(plane_origin.y)),
                        ),
                        (plane_dir.x, plane_dir.y),
                    )),
                    axes: None,
                })
            }
            other => Err(eval_err(
                py,
                format!("a `{}` value is not a datum", other.kind_name()),
                "wrong_kind",
                self.node,
            )),
        }
    }

    /// The quantity a `Measure` node evaluated to (E3).
    ///
    /// **A measure with no value at this scalar refuses by NAME**
    /// (M10-6, R2's MINOR-4). `min_clearance` is answered by an engine
    /// that needs an enclosure, so at the `f64` scalar a Python caller
    /// drives it has no value — a typed ABSENCE, not a failure, which
    /// is why the node evaluates at all. That absence shares
    /// `kind_name()` with a taken measurement ("measure"), so the
    /// generic arm below produced the nonsense "a `measure` value is
    /// not a measure" and told a caller nothing about what to do.
    /// `Value.assertion` already carried its reason through; this door
    /// now does the same.
    ///
    /// The absence raises `MeasureUnavailableAt` and not
    /// `EvaluationError`: it is the kernel's own typed reason, and it
    /// carries the verb, the scalar this build ran at and the DOOR
    /// that can answer, so the recourse is in the refusal rather than
    /// in a reader's memory. `EvaluationError` is for a node that
    /// FAILED, and this one did not.
    fn measure(&self, py: Python<'_>) -> PyResult<Measurement> {
        match &self.payload {
            d::ValuePayload::Measure { value, dim } => Ok(Measurement {
                dimension: dimension_name(*dim),
                value: *value,
                length: (*dim == d::Dimension::Length)
                    .then(|| Length(pncad::quantity::Length::from_meters(*value))),
            }),
            d::ValuePayload::MeasureUnavailable { reason, .. } => {
                Err(super::measure::measure_unavailable_at_err(py, reason))
            }
            other => Err(eval_err(
                py,
                format!("a `{}` value is not a measure", other.kind_name()),
                "wrong_kind",
                self.node,
            )),
        }
    }

    /// The verdict an `Assertion` node evaluated to (E10) — report
    /// only: reading it changes nothing about the document.
    fn assertion(&self, py: Python<'_>) -> PyResult<Verdict> {
        let d::ValuePayload::Assertion(verdict) = &self.payload else {
            return Err(eval_err(
                py,
                format!("a `{}` value is not an assertion", self.payload.kind_name()),
                "wrong_kind",
                self.node,
            ));
        };
        let (measured, bound, reason) = match verdict {
            d::AssertionVerdict::Holds { measured, bound }
            | d::AssertionVerdict::Violated { measured, bound } => {
                (Some(*measured), Some(*bound), None)
            }
            d::AssertionVerdict::Unevaluated { reason } => (None, None, Some(reason.to_string())),
        };
        Ok(Verdict {
            status: verdict.label(),
            holds: verdict.holds(),
            measured,
            bound,
            reason,
        })
    }

    fn __repr__(&self) -> String {
        format!("Value({})", self.payload.kind_name())
    }
}

/// The result of evaluating a document: the per-node result DAG.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct Evaluation {
    pub(crate) inner: d::Evaluation<f64>,
    /// The evaluated document's parameter bindings, captured at
    /// `evaluate` — `select_where`'s decided atoms state their value
    /// as an `Expr`, which cannot be evaluated without them. Captured
    /// HERE because the answer must be as of the same document the
    /// evaluation is of; threading the doc back in per query would
    /// let the two drift.
    params: d::ParamEnv<f64>,
    /// The document the evaluation ran on, captured at `evaluate` for
    /// the same reason [`Self::params`] is — and this is the whole of
    /// what the kernel's `RunCtx` is: a run is a (document,
    /// evaluation) PAIR, and resolution needs both halves (the
    /// document decides whether a stored name's minting node is still
    /// there at all, which no evaluation can answer).
    ///
    /// **Captured rather than taken per call**, which is the same
    /// argument `NodePick` makes about its `(node, body)` pairing:
    /// Python's `Doc` is mutable and `accept` swaps the document
    /// under the handle, so a `resolve(doc, name)` door would let a
    /// caller ask this evaluation about a document it is not of, and
    /// answer confidently against the wrong recipe. Pairing the two
    /// here makes that unspellable.
    doc: d::ProfileDoc,
    /// The document's gathered product, materialized on the first ask
    /// and kept for every later one
    /// ([`crate::product_memo`], which holds the whole of the reasoning).
    ///
    /// It lives HERE because a product is a pure function of the pair
    /// above and the run's tolerance, and this object is that pair: it
    /// is frozen, so nothing can move under the memo, and the doors
    /// that want a product — `run_checks`, `assemble`, `product`,
    /// `product_named` — keep their signatures and share one gather.
    product: crate::product_memo::ProductMemo,
}

impl Evaluation {
    /// The (document, evaluation) pair and the memo over it, as the
    /// arguments [`crate::product_memo`]'s doors take.
    pub(crate) fn gathered<T>(
        &self,
        f: impl FnOnce(&crate::product_memo::ProductMemo, &d::ProfileDoc, &d::Evaluation<f64>) -> T,
    ) -> T {
        f(&self.product, &self.doc, &self.inner)
    }

    /// The DI3 pairing gate: `doc` must be the document this
    /// evaluation is of. `Err` carries the two ids the caller's own
    /// refusal arm names.
    pub(crate) fn paired_with(&self, doc: &super::doc::Doc) -> Result<(), d::Mispaired> {
        crate::product_memo::ProductMemo::paired(&self.inner, doc.inner.id())
    }
}

#[pymethods]
impl Evaluation {
    /// The node's successful value.
    ///
    /// A node that produced NO value raises with the REAL typed cause
    /// — never a placeholder:
    /// `reason` is `"node_failed"` or `"poisoned"`, `kind` is the
    /// `NodeErrorKind`'s stable tag, a poisoning carries `through`,
    /// and the message renders the kernel's own `NodeError`.
    ///
    /// A node with no ENTRY at all is two different states and they
    /// are kept apart: `unknown_node` for an id this document does
    /// not have, and `node_not_evaluated` — the standing ladder's own
    /// spelling, shared with `ReadbackError` and `HitTestError` — for
    /// a live node that this run never reached. The second arm exists
    /// because [`super::value::evaluate`]'s `cancel=` made it
    /// reachable: a canceled run holds the completed PREFIX, and every
    /// node past it is in [`Self::order`] with no result. Before that
    /// keyword the arm was unreachable and the door said "no such
    /// node" for both, which was true only because the false case
    /// could not arise.
    fn value(&self, py: Python<'_>, node: &NodeId) -> PyResult<Value> {
        match self.inner.result(node.0) {
            Some(d::NodeResult::Ok(node_value)) => Ok(Value {
                payload: node_value.payload.clone(),
                contacts: Arc::clone(&node_value.contacts),
                node: *node,
            }),
            Some(d::NodeResult::Failed(error)) => Err(node_failure(py, *node, error)),
            Some(d::NodeResult::Poisoned { through }) => {
                let root = self.inner.node_error(node.0);
                Err(poisoning(py, *node, NodeId(*through), root))
            }
            None if self.inner.order.contains(&node.0) => Err(eval_err(
                py,
                "this evaluation never reached the node: it was canceled first, \
                 and holds the completed prefix only",
                NODE_NOT_EVALUATED,
                *node,
            )),
            None => Err(eval_err(
                py,
                "no such node in the evaluated document",
                "unknown_node",
                *node,
            )),
        }
    }

    /// **Whether this run was CANCELED** — the Python shape of the
    /// kernel's `EvalOutcome`, whose two variants are a yes/no and
    /// cross as one.
    ///
    /// `True` means the token passed as [`super::value::evaluate`]'s
    /// `cancel=` was observed set at a yield point, so
    /// [`Self::order`] is still the FULL deterministic order (order is
    /// data, not schedule) while the results hold the completed
    /// PREFIX only. Every node past that prefix answers `False` from
    /// [`Self::succeeded`] and raises `node_not_evaluated` from
    /// [`Self::value`] — a canceled run is a PARTIAL ANSWER, never a
    /// failed one, and no node in it is marked failed by the
    /// cancelation.
    ///
    /// `False` without a `cancel=` token is not a claim worth
    /// doubting: `evaluate` mints a fresh token nobody can reach, so
    /// a run with no token cannot be canceled.
    #[getter]
    fn canceled(&self) -> bool {
        self.inner.outcome == d::EvalOutcome::Canceled
    }

    /// Whether the node produced a value.
    fn succeeded(&self, node: &NodeId) -> bool {
        self.inner.value(node.0).is_some()
    }

    /// The evaluation order.
    fn order(&self) -> Vec<NodeId> {
        self.inner.order.iter().copied().map(NodeId).collect()
    }

    /// **Every edge name of `node`'s output body, as of THIS
    /// evaluation** — the name materializer, crossing as text.
    ///
    /// This is the door a `Node.fillet` selection comes through, and
    /// it MATERIALIZES rather than queries: it answers for the
    /// evaluation in hand, the caller stores the answer, and from that
    /// moment the selection is frozen like any other. A recipe holds
    /// no live "all edges", because a stored one would silently grow
    /// under an upstream edit — the staleness the freeze prevents.
    ///
    /// The answer is the WHOLE kind, and each string is an OPAQUE
    /// identifier: its internal structure is not API (see
    /// `doc::name_text`), so narrowing the set is a SELECTOR's job —
    /// [`Self::select`] and [`Self::select_where`], which
    /// answer in the same alphabet.
    ///
    /// Empty when the node has no value, no name table, or no edges.
    /// The fillet node is what refuses an EMPTY selection, so the
    /// emptiness surfaces there rather than here.
    fn all_edges(&self, py: Python<'_>, node: &NodeId) -> PyResult<Vec<String>> {
        names(py, pncad::select::all_edges(&self.inner, node.0))
    }

    /// Every FACE name of `node`'s output, [`Self::all_edges`]'s
    /// sibling — same contract, same alphabet.
    fn all_faces(&self, py: Python<'_>, node: &NodeId) -> PyResult<Vec<String>> {
        names(py, pncad::select::all_faces(&self.inner, node.0))
    }

    /// Every VERTEX name of `node`'s output, same contract.
    fn all_vertices(&self, py: Python<'_>, node: &NodeId) -> PyResult<Vec<String>> {
        names(py, pncad::select::all_vertices(&self.inner, node.0))
    }

    /// Every BODY name `node`'s evaluation carries, same contract.
    /// Usually one row; a split's two halves are the plural case.
    fn all_bodies(&self, py: Python<'_>, node: &NodeId) -> PyResult<Vec<String>> {
        names(py, pncad::select::all_bodies(&self.inner, node.0))
    }

    /// **Materialize a STRUCTURAL selection**: every name of `node`'s
    /// output matching `selector`'s role-path shape, as of THIS
    /// evaluation — Rust's `select`, same contract as `all_edges`
    /// (canonical order, the caller stores it, frozen thereafter).
    ///
    /// The answer is the same opaque texts the whole-kind
    /// materializers speak, ready for `Node.fillet` unread: narrowing
    /// happens through this door, never by parsing a name (name text
    /// is an identifier, not a value).
    ///
    /// Infallible like `select`: empty when `node` has no value, no
    /// name table, or nothing matches.
    fn select(
        &self,
        py: Python<'_>,
        node: &NodeId,
        selector: &super::select::Selector,
    ) -> PyResult<Vec<String>> {
        names(py, pncad::select::select(&self.inner, node.0, &selector.0))
    }

    /// **Materialize a selection narrowed by GEOMETRY**: `selector`
    /// narrows by name shape, then each survivor is resolved to its
    /// entity in THIS evaluation and tested against the CONJUNCTION
    /// `geom` — Rust's `select_where`, verb for verb. An empty `geom`
    /// makes this exactly [`Self::select`]; run it twice and
    /// concatenate for a geometric union.
    ///
    /// Same materializer contract and same opaque-text alphabet as
    /// [`Self::select`]: the binding narrows, your code never reads
    /// inside a name.
    ///
    /// Raises `SelectRefusal`, typed, where the Rust door refuses:
    /// exact atoms are total and cannot refuse, but a DECIDED atom's
    /// in-band margin (`reason="in_band"`), a tied name whose
    /// candidates disagree (`"tied_disagrees"`), an unreadable
    /// candidate (`"unreadable"`), a non-datum reference
    /// (`"not_a_datum"`) all refuse rather than silently including or
    /// dropping a candidate.
    fn select_where(
        &self,
        py: Python<'_>,
        node: &NodeId,
        selector: &super::select::Selector,
        geom: Vec<super::select::GeomPred>,
    ) -> PyResult<Vec<String>> {
        let tol = Tol::witness();
        let atoms: Vec<pncad::select::GeomPred> = geom.into_iter().map(|g| g.0).collect();
        match pncad::select::select_where(
            &self.inner,
            node.0,
            &selector.0,
            &atoms,
            &self.params,
            tol,
        ) {
            Ok(found) => names(py, found),
            Err(refusal) => Err(super::select::select_refusal(py, &refusal)),
        }
    }

    /// **Where is the face I selected?** — the named face's carrier
    /// frame, as of THIS evaluation.
    ///
    /// The forward twin of the materializers: they hand out names,
    /// and this asks one where it SITS. `name` is one of the opaque
    /// texts `all_faces` / `select` / `select_where` answered with,
    /// handed back unread.
    ///
    /// The answer is the CARRIER's frame, copied out of stored
    /// geometry — a definitional re-read, so no pad and no
    /// measurement. It is not a verdict: no door here says whether a
    /// face is planar or where it is relative to anything, which is
    /// `select_where`'s decided half.
    ///
    /// Raises `ReadbackError`, typed: `no_such_name` for a stale
    /// selection, `ambiguous` for an N2 tie (ask
    /// [`Self::denotation`] first), `wrong_kind` for an edge or
    /// vertex name, `no_canonical_frame` for a NURBS carrier, and the
    /// node ladder for a node this evaluation did not produce.
    fn face_frame(
        &self,
        py: Python<'_>,
        node: &NodeId,
        name: &str,
    ) -> PyResult<super::readback::Pose> {
        let name = super::doc::name_from_text(name)?;
        pncad::select::face_frame(&self.inner, node.0, &name)
            .map(super::readback::Pose)
            .map_err(|err| super::readback::readback_err(py, &err))
    }

    /// **Where is the edge I selected?** — the named edge's certified
    /// carrier frame, [`Self::face_frame`]'s sibling.
    ///
    /// A straight edge answers with `u_ref is None`: a line has a
    /// direction and no distinguished perpendicular, and the door
    /// says so rather than inventing one.
    ///
    /// Raises `ReadbackError` as [`Self::face_frame`] does, with
    /// `wrong_kind` for a non-edge name and `no_carrier` for an edge
    /// still carrying null-edge scaffolding.
    fn edge_frame(
        &self,
        py: Python<'_>,
        node: &NodeId,
        name: &str,
    ) -> PyResult<super::readback::Pose> {
        let name = super::doc::name_from_text(name)?;
        pncad::select::edge_frame(&self.inner, node.0, &name)
            .map(super::readback::Pose)
            .map_err(|err| super::readback::readback_err(py, &err))
    }

    /// **Where is the vertex I selected?** — the named vertex's
    /// stored position, dimensioned.
    ///
    /// Raises `ReadbackError` as [`Self::face_frame`] does, with
    /// `wrong_kind` for a non-vertex name.
    fn vertex_position(
        &self,
        py: Python<'_>,
        node: &NodeId,
        name: &str,
    ) -> PyResult<(Length, Length, Length)> {
        let name = super::doc::name_from_text(name)?;
        pncad::select::vertex_position(&self.inner, node.0, &name)
            .map(lengths)
            .map_err(|err| super::readback::readback_err(py, &err))
    }

    /// **What KIND of surface carries the face I selected?** — the
    /// face's stored carrier tag, copied out.
    ///
    /// A tag READ, never a verdict: `SurfaceKind.Plane` comes back
    /// because the body RECORDS a plane there, and "is this face
    /// planar" is a comparison the caller makes against the answer.
    /// No tolerance enters and nothing is decided — the same split
    /// [`Self::face_frame`] keeps between a value and a predicate.
    ///
    /// It is the door [`Self::face_frame`] cannot be: a NURBS carrier
    /// has no canonical frame and the frame door refuses it
    /// (`no_canonical_frame`), while its kind is still readable here.
    /// It is also how a caller checks a face BEFORE building a
    /// `Node.datum_face_frame` on it, which refuses a non-planar
    /// carrier at `evaluate`.
    ///
    /// Raises `ReadbackError` as [`Self::face_frame`] does, with
    /// `wrong_kind` for an edge or vertex name.
    fn face_carrier_kind(
        &self,
        py: Python<'_>,
        node: &NodeId,
        name: &str,
    ) -> PyResult<super::select::SurfaceKind> {
        let name = super::doc::name_from_text(name)?;
        pncad::select::face_carrier_kind(&self.inner, node.0, &name)
            .map(super::select::surface_kind)
            .map_err(|err| super::readback::readback_err(py, &err))
    }

    /// **How does this name resolve — uniquely, or as a tie?** The
    /// referencing question, answered without exposing what it
    /// resolves to.
    ///
    /// This is the door to ask BEFORE a frame: the three frame doors
    /// refuse an N2 tie (`ambiguous`) rather than picking a
    /// candidate, and this says whether one is coming. It answers a
    /// COUNT, never the candidates — those are arena keys, which do
    /// not cross.
    ///
    /// Raises `ReadbackError` for the node ladder and `no_such_name`.
    fn denotation(
        &self,
        py: Python<'_>,
        node: &NodeId,
        name: &str,
    ) -> PyResult<super::readback::Denotation> {
        let name = super::doc::name_from_text(name)?;
        pncad::select::denotation(&self.inner, node.0, &name)
            .map(super::readback::Denotation)
            .map_err(|err| super::readback::readback_err(py, &err))
    }

    /// **Does this STORED name still denote, in THIS evaluation?** —
    /// the question every consumer that keeps names must ask on every
    /// run, and the one Python's whole store-then-reuse story runs on.
    ///
    /// Answers a `Resolution`, never a raise: "this name is gone" is
    /// a verdict, not an error, and the three states are kept three
    /// because their repairs differ. `resolved` carries `node`, `body`
    /// and `kind`; `failed` means rebind (with `offers` as the
    /// kernel's suggestions, never its substitutions); `indeterminate`
    /// means the name is fine and the RUN is not — the minting node
    /// failed, was poisoned or never evaluated, so the repair is
    /// upstream and the name resolves again when that node does.
    ///
    /// **Evaluation-wide**, where `denotation` is node-scoped: this
    /// searches every table in evaluation order and answers WHICH node
    /// carries the name, so it resolves for a name `denotation` would
    /// refuse `no_such_name` for at the node you happened to ask.
    ///
    /// The only raise here is the boundary one every name-taking door
    /// shares: text that is not a name at all is a `ValueError`. A
    /// well-formed name that denotes nothing is the `failed` verdict,
    /// which is the whole point of the door.
    fn resolve(&self, py: Python<'_>, name: &str) -> PyResult<super::resolve::Resolution> {
        let name = super::doc::name_from_text(name)?;
        super::resolve::resolution(
            py,
            &pncad::select::resolve(
                pncad::select::RunCtx {
                    doc: &self.doc,
                    eval: &self.inner,
                },
                &name,
            ),
        )
    }

    /// **What is under this ray?** — the nearest face hit across
    /// `targets`, resolved to a stable name, as of THIS evaluation.
    ///
    /// The fourth door onto a name, and it answers in the same opaque
    /// alphabet the other three speak: `PickHit.name` is a text
    /// `Node.fillet` takes unread, exactly as `select`'s answers are.
    ///
    /// `targets` are `NodePick`s — build them with `NodePick.build` or
    /// `NodePick.build_all`. There is no other spelling of a pick
    /// target here, and that is deliberate: a target whose
    /// `(node, body)` is not the pair its mesh was tessellated from
    /// answers a plausible, confidently WRONG name rather than an
    /// error, and a `NodePick` cannot be built that way.
    ///
    /// **A miss is `None`, and it is typed.** The ray hitting no
    /// offered triangle is not a failure, and a failure is never
    /// flattened into it. Ties are broken totally and documented
    /// kernel-side: the winner minimizes `(t, position in targets,
    /// triangle position)`, so a ray down a shared edge answers the
    /// same face every time.
    ///
    /// Raises `HitTestError`, typed: the standing ladder up front for
    /// a target whose node this evaluation has no value for
    /// (`node_not_evaluated`, `node_failed`, `node_poisoned`), and the
    /// loud `unnamed` bug arm if the winning face inverts to no name.
    fn pick_face(
        &self,
        py: Python<'_>,
        targets: Vec<PyRef<'_, super::pick::NodePick>>,
        ray: &super::pick::Ray,
    ) -> PyResult<Option<super::pick::PickHit>> {
        super::pick::pick_face(py, self, targets, ray)
    }

    /// **The cross-body flush candidates between `a`'s and
    /// `b`'s outputs, as of THIS evaluation** — the detect arm of the
    /// detect/declare protocol: the verifier run in
    /// candidate-generation mode, so a
    /// finding can never disagree with the boolean's own
    /// verify-at-use.
    ///
    /// Findings come back in canonical order and are only ever
    /// DEFINITE values — inspect them, then `Node.declare` /
    /// `Doc.declare` / `Doc.declare_all` turn the inspected findings
    /// into the `Declare` node `Node.boolean`'s `declare=` consumes.
    /// Detection and declaration are separate doors ON PURPOSE (the
    /// ruled no-fusion boundary). Like `select`, the query answers
    /// EMPTY if either node has no value in this evaluation.
    ///
    /// Raises `SelectRefusal`, typed, exactly where the Rust door
    /// refuses: a pair whose verify-door margin is inside the
    /// ambiguity band (`reason="pair_in_band"` — neither reported nor
    /// silently dropped), a tied name whose candidates disagree
    /// (`"tied_disagrees"`), an unreadable name-table entry
    /// (`"unreadable"`), a broken ambient tolerance (`"band"`).
    fn find_flush_candidates(
        &self,
        py: Python<'_>,
        a: &NodeId,
        b: &NodeId,
    ) -> PyResult<Vec<super::flush::FlushFinding>> {
        let tol = Tol::witness();
        match pncad::select::find_flush_candidates(&self.inner, a.0, b.0, tol) {
            Ok(findings) => Ok(findings
                .into_iter()
                .map(super::flush::FlushFinding)
                .collect()),
            Err(refusal) => Err(super::select::select_refusal(py, &refusal)),
        }
    }

    /// How many nodes ran their op this evaluation.
    ///
    /// With no `prior=`, this is every node that ran. With one, it is
    /// the changed cone — and `recomputed + reused` is what makes the
    /// two numbers EVIDENCE of reuse rather than a hint about it.
    ///
    /// The sum is the nodes that RAN OR WERE REUSED, which is the live
    /// node count only when every node produced a result. A POISONED
    /// node — one that never ran because an ancestor failed — is
    /// counted by neither, so on any refusal path the sum undershoots
    /// `len(order())` by exactly the number of poisonings. A node that
    /// ran and FAILED is counted here, in `recomputed`: it ran.
    #[getter]
    fn recomputed(&self) -> usize {
        self.inner.recomputed
    }

    /// How many nodes were served from `evaluate`'s `prior=` memo
    /// without re-running their op — zero when no prior was passed.
    #[getter]
    fn reused(&self) -> usize {
        self.inner.reused
    }

    /// How many REFERENCED documents this evaluation actually crossed
    /// the seam to evaluate.
    ///
    /// The sharing evidence for `evaluate`'s `resolver=`: N instances
    /// of one part count 1, because the part is evaluated once and its
    /// body reused; a part that instantiates a part counts here too, so
    /// the number is the whole run's seam traffic and not one level's.
    /// Zero without a resolver — nothing crosses.
    #[getter]
    fn part_evaluations(&self) -> usize {
        self.inner.part_evaluations
    }

    /// Export the single body `node` denotes as a STEP (AP214 Part 21)
    /// exchange-file string (the document-layer export door,
    /// `pncad::export::step_for_node` — one construction site
    /// for "which body does this node denote", shared with Rust).
    ///
    /// Accepts a `body` or non-empty `boolean` value; everything else
    /// raises a typed `ExportError`.
    ///
    /// Every `StepOptions` field is a keyword here, and each defaults
    /// to `None` meaning the Rust default — so the door narrows
    /// nothing and an omitted keyword is the same file a Rust caller
    /// gets from `StepOptions::default()`. The options struct is built
    /// by a literal that names every field, so a field the kernel
    /// gains does not compile until this door decides about it; that
    /// decision is recorded, either way, in the surface census.
    ///
    /// `uncertainty` is the exported
    /// `UNCERTAINTY_MEASURE_WITH_UNIT` length; omitted, the writer
    /// reads the run's ambient tolerance, which is the ε the body was
    /// built under. A value that is not finite and strictly positive
    /// is the writer's refusal, not a check restated here.
    #[pyo3(signature = (
        node,
        product_name = None,
        timestamp = None,
        author = None,
        organization = None,
        originating_system = None,
        uncertainty = None,
    ))]
    #[allow(clippy::too_many_arguments)]
    fn step_string(
        &self,
        py: Python<'_>,
        node: &NodeId,
        product_name: Option<String>,
        timestamp: Option<String>,
        author: Option<String>,
        organization: Option<String>,
        originating_system: Option<String>,
        uncertainty: Option<Length>,
    ) -> PyResult<String> {
        let tol = Tol::witness();
        let defaults = pncad::step_export::StepOptions::default();
        let options = pncad::step_export::StepOptions {
            product_name: product_name.unwrap_or(defaults.product_name),
            timestamp: timestamp.unwrap_or(defaults.timestamp),
            author: author.unwrap_or(defaults.author),
            organization: organization.unwrap_or(defaults.organization),
            originating_system: originating_system.unwrap_or(defaults.originating_system),
            uncertainty_m: uncertainty.map(|u| u.0.meters()).or(defaults.uncertainty_m),
        };
        pncad::export::step_for_node(&self.inner, node.0, &options, tol)
            .map_err(|err| export_err(py, *node, &err))
    }

    fn __repr__(&self) -> String {
        format!("Evaluation({} nodes)", self.inner.order.len())
    }
}

/// Raise `ExportError` mirroring the Rust door's refusal: `variant`
/// is the arm's stable tag, `node` rides along, a poisoning adds
/// `through` and a wrong-kind value adds `kind`. The message is the
/// door's own `Display`.
fn export_err(py: Python<'_>, node: NodeId, err: &pncad::export::ExportError) -> PyErr {
    use pncad::export::ExportError as E;
    let node_obj = match node.into_pyobject(py) {
        Ok(bound) => bound.unbind().into_any(),
        Err(failed) => return failed,
    };
    // `through`/`kind` are ALWAYS present (`None` where inapplicable)
    // so stub-guided reads cannot `AttributeError`.
    let mut fields: Vec<(&str, Py<PyAny>)> = vec![
        (
            "variant",
            PyString::new(py, export_error_tag(err)).unbind().into_any(),
        ),
        ("node", node_obj),
        ("through", py.None().into_any()),
        ("kind", py.None().into_any()),
    ];
    match err {
        E::Poisoned { through, .. } => match NodeId(*through).into_pyobject(py) {
            Ok(bound) => fields[2] = ("through", bound.unbind().into_any()),
            Err(failed) => return failed,
        },
        E::NotABody { kind, .. } => {
            fields[3] = ("kind", PyString::new(py, kind).unbind().into_any());
        }
        // `Product` is the WHOLE-DOCUMENT door's refusal: it names
        // product roots, not this call's node, so it adds no field
        // here. The arm is spelled out because the match
        // is exhaustive on purpose — the tripwire, not a wildcard.
        E::UnknownNode { .. }
        | E::NodeFailed { .. }
        | E::EmptyBoolean { .. }
        | E::Step(_)
        | E::Product(_) => {}
    }
    typed_err(py, ErrorClass::Export, err.to_string(), &fields)
}

/// **A boundary-graph census**: what one region contributes to the
/// body, in faces, edges and vertices.
///
/// Two of these ride every [`StructureNormalization`] — the counts the
/// file states for the region, and the counts the mint left it with —
/// and their difference is the mapping a reader needs to reconcile the
/// file's numbers with the imported body's.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct FaceCensus {
    /// Faces.
    #[pyo3(get)]
    faces: usize,
    /// Edges.
    #[pyo3(get)]
    edges: usize,
    /// Vertices.
    #[pyo3(get)]
    vertices: usize,
}

#[pymethods]
impl FaceCensus {
    fn __eq__(&self, other: &Self) -> bool {
        (self.faces, self.edges, self.vertices) == (other.faces, other.edges, other.vertices)
    }

    fn __repr__(&self) -> String {
        format!(
            "FaceCensus(faces={}, edges={}, vertices={})",
            self.faces, self.edges, self.vertices
        )
    }
}

/// **One re-minted boundary graph**, reported as data: the file's
/// locus was fully explained and adopted, but its tessellation is not
/// representable, so the kernel cut the same surface into its own
/// faces, edges and vertices and says which.
///
/// Volume and validity are exact either way; what changed is only the
/// cut. `file_census` and `kernel_census` are that change, counted.
///
/// `kind` is the stable word for what was re-minted, and the payload
/// of the one arm that carries one arrives beside it, `None` on the
/// other four: `promoted_to` is which analytic surface kind certified
/// and `residual` is the certified deviation that let it. Present on
/// every row, so a read never raises and a caller never has to branch
/// on `kind` first.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct StructureNormalization(pncad::step_import::StructureNormalization);

#[pymethods]
impl StructureNormalization {
    /// The `ADVANCED_FACE` entity instance the file states.
    #[getter]
    fn face(&self) -> u64 {
        self.0.face
    }

    /// What was re-minted, as a stable word.
    #[getter]
    fn kind(&self) -> &'static str {
        normalization_kind_tag(&self.0.kind)
    }

    /// Which analytic surface kind certified — on `surface_promotion`
    /// alone.
    #[getter]
    fn promoted_to(&self) -> Option<&'static str> {
        match &self.0.kind {
            pncad::step_import::NormalizationKind::SurfacePromotion { to, .. } => {
                Some(promoted_kind_tag(to))
            }
            _ => None,
        }
    }

    /// The certified residual sup (metres): the patch's worst
    /// deviation from the surface it was promoted to. On
    /// `surface_promotion` alone.
    #[getter]
    fn residual(&self) -> Option<f64> {
        match &self.0.kind {
            pncad::step_import::NormalizationKind::SurfacePromotion { residual, .. } => {
                Some(*residual)
            }
            _ => None,
        }
    }

    /// The census the file states for the region.
    #[getter]
    fn file_census(&self) -> FaceCensus {
        census(self.0.file_census)
    }

    /// The census as THIS mint left the region — a mint-event value,
    /// not an at-rest one: a later normalization may split an edge the
    /// region shares, and this record is not revised. The records'
    /// deltas sum to the body's totals; per-face at-rest counts are the
    /// body's to answer.
    #[getter]
    fn kernel_census(&self) -> FaceCensus {
        census(self.0.kernel_census)
    }

    fn __repr__(&self) -> String {
        format!(
            "StructureNormalization(face={}, kind={})",
            self.0.face,
            normalization_kind_tag(&self.0.kind)
        )
    }
}

/// **One promoted curve carrier**: the file stated a NURBS carrier
/// whose deviation from an analytic curve certified at the import's
/// tolerance, so the edge adopted on the analytic carrier.
///
/// It re-mints nothing — the boundary graph is untouched — which is
/// why it carries no census where a [`StructureNormalization`] carries
/// two, and why its key is a curve entity rather than a face.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct CurvePromotion(pncad::step_import::CurvePromotion);

#[pymethods]
impl CurvePromotion {
    /// The curve entity instance the file states — the `EDGE_CURVE`'s
    /// carrier, not an edge that uses it: one carrier may serve
    /// several edges and the promotion is the carrier's.
    #[getter]
    fn curve(&self) -> u64 {
        self.0.curve
    }

    /// The analytic kind that certified, as a stable word.
    #[getter]
    fn kind(&self) -> &'static str {
        promoted_curve_kind_tag(&self.0.kind)
    }

    /// The certified residual sup (metres): the carrier's worst
    /// deviation from the promoted curve over its whole domain.
    #[getter]
    fn residual(&self) -> f64 {
        self.0.residual
    }

    fn __repr__(&self) -> String {
        format!(
            "CurvePromotion(curve={}, kind={})",
            self.0.curve,
            promoted_curve_kind_tag(&self.0.kind)
        )
    }
}

/// **One materialized assembly instance** — what the file said about
/// one solid of the imported body.
///
/// An assembly states N occurrences of M component representations,
/// and import materializes each occurrence as its own solid; one of
/// these travels with each, in `body.solids()` order. Flattening is
/// the right evaluation product and it is not forgetting: this is the
/// association a later import-as-assembly door rebuilds.
///
/// Every entity field names a real record in the file, or is `None`
/// because the file states no assembly — a file with no assembly
/// vocabulary still gets one row per solid, so a caller never has to
/// ask whether the record exists.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct PlacedInstance(pncad::step_import::PlacedInstance);

#[pymethods]
impl PlacedInstance {
    /// Which solid of the imported body this row describes, in
    /// `Body`'s own solid order.
    #[getter]
    fn index(&self) -> usize {
        self.0.index
    }

    /// The `MANIFOLD_SOLID_BREP` this instance is a copy of. Repeats
    /// across one component's several occurrences: that repetition IS
    /// the instancing.
    #[getter]
    fn solid(&self) -> u64 {
        self.0.solid
    }

    /// The shape representation that names `solid`.
    #[getter]
    fn component(&self) -> u64 {
        self.0.component
    }

    /// The `NEXT_ASSEMBLY_USAGE_OCCURRENCE` this instance is, where
    /// the file links one.
    #[getter]
    fn occurrence(&self) -> Option<u64> {
        self.0.occurrence
    }

    /// The `REPRESENTATION_RELATIONSHIP` complex stating the placement.
    #[getter]
    fn relationship(&self) -> Option<u64> {
        self.0.relationship
    }

    /// The `ITEM_DEFINED_TRANSFORMATION` the placement was read from.
    #[getter]
    fn transform(&self) -> Option<u64> {
        self.0.transform
    }

    /// The rigid map actually APPLIED to this copy, as the placement
    /// value the rest of this surface uses.
    ///
    /// `None` is the identity — the file stated a placement that is
    /// the identity at the import's tolerance, or stated none. It is
    /// left as `None` rather than materialized: the record says
    /// nothing moved, and an identity `Frame` here would read as a map
    /// the file chose.
    #[getter]
    fn placement(&self) -> Option<super::place::Frame> {
        self.0
            .placement
            .map(|a| super::place::Frame(d::Frame::from_affine(a)))
    }

    fn __repr__(&self) -> String {
        format!(
            "PlacedInstance(index={}, solid={}, component={})",
            self.0.index, self.0.solid, self.0.component
        )
    }
}

fn census(c: pncad::step_import::FaceCensus) -> FaceCensus {
    FaceCensus {
        faces: c.faces,
        edges: c.edges,
        vertices: c.vertices,
    }
}

/// **What a successful STEP import produced** — the body, the gate's
/// own measurement of it, and the record of what the adoption changed
/// about the file.
///
/// The body is a `Body` handle like any other and gains nothing from
/// arriving this way; the report is what is new. `enclosure` is the
/// certified `MassProperties` the import's own at-rest gate derived
/// and decided the body's orientation invariant on, handed back rather
/// than dropped — **not a second computation**. A caller that reads it
/// measures the imported body ONCE; a caller that calls
/// `report.body.mass_properties()` runs the certified quadrature a
/// second time over the same body at the same band and gets the same
/// four fields bit for bit.
///
/// The three record lists are the adoption's own report, "as data,
/// never silently": `normalizations` is every boundary graph the
/// kernel re-minted, `promotions` every NURBS curve carrier adopted as
/// an analytic one, and `instances` the assembly record — one row per
/// solid, kept whether or not the file states an assembly.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct ImportReport {
    /// The adopted body — Euler-built, certified, and tier-valid at
    /// rest, checked before it was handed out.
    #[pyo3(get)]
    body: Body,
    /// The at-rest gate's own enclosure of `body`.
    #[pyo3(get)]
    enclosure: MassProperties,
    /// The import's input tolerance in metres: the file's declared
    /// uncertainty, which is a separate quantity from the kernel's ε.
    #[pyo3(get)]
    eps_in: f64,
    normalizations: Vec<StructureNormalization>,
    promotions: Vec<CurvePromotion>,
    instances: Vec<PlacedInstance>,
}

#[pymethods]
impl ImportReport {
    /// Every boundary graph the adoption re-minted, in resolution
    /// order — empty for a file the kernel represents as stated.
    #[getter]
    fn normalizations(&self) -> Vec<StructureNormalization> {
        self.normalizations.clone()
    }

    /// Every NURBS curve carrier adopted as an analytic curve, by
    /// ascending curve entity id — empty for a file whose carriers are
    /// all stated analytically or all stay NURBS.
    #[getter]
    fn promotions(&self) -> Vec<CurvePromotion> {
        self.promotions.clone()
    }

    /// The assembly record: one row per solid of `body`, in its solid
    /// order.
    #[getter]
    fn instances(&self) -> Vec<PlacedInstance> {
        self.instances.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "ImportReport(volume={} m^3, {} normalization(s), {} promotion(s), {} instance(s))",
            self.enclosure.volume,
            self.normalizations.len(),
            self.promotions.len(),
            self.instances.len()
        )
    }
}

/// Parse a STEP text with the kernel's own importer and adopt its
/// solid, answering the whole [`ImportReport`]: the body, the gate's
/// own enclosure of it, and the record of what the adoption changed.
///
/// The report rather than the body alone is what makes the natural
/// journey — import a file, then ask what it encloses — measure the
/// solid once. The importer's gate has already run the certified
/// quadrature to decide the body's orientation invariant, so
/// `enclosure` is that measurement handed back and
/// `body.mass_properties()` is a second one over the same body.
#[pyfunction]
pub(crate) fn import_step(py: Python<'_>, text: &str) -> PyResult<ImportReport> {
    let tol = Tol::witness();
    match pncad::step_import::import_step(text, &pncad::step_import::ImportOptions::default(), tol)
    {
        Ok(pncad::step_import::StepImport::Solid {
            body,
            enclosure,
            eps_in,
            normalizations,
            curve_promotions,
            instances,
        }) => Ok(ImportReport {
            body: Body::plain(Arc::new(body)),
            enclosure: MassProperties {
                volume: enclosure.volume,
                surface_area: enclosure.surface_area,
                volume_pad: enclosure.volume_pad,
                area_pad: enclosure.area_pad,
            },
            eps_in,
            normalizations: normalizations
                .into_iter()
                .map(StructureNormalization)
                .collect(),
            promotions: curve_promotions.into_iter().map(CurvePromotion).collect(),
            instances: instances.into_iter().map(PlacedInstance).collect(),
        }),
        // Not a refusal variant: the import SUCCEEDED and produced
        // the other arm of `StepImport`, which this door does not
        // adopt. Its tag is the arm's name and shares the namespace
        // with `step_import_error_tag`'s, which contains no
        // `wireframe`.
        Ok(pncad::step_import::StepImport::Wireframe { .. }) => Err(typed_err(
            py,
            ErrorClass::StepImport,
            "the file parsed to a wireframe, not a solid",
            &[
                (
                    "variant",
                    PyString::new(py, "wireframe").unbind().into_any(),
                ),
                ("promoted_kind", py.None()),
            ],
        )),
        // The tag is the importer's own, through `crate::tags`. Every
        // arm of `StepImportError` is reachable here, and the entity
        // id and line that would tell them apart live in the message
        // prose — so one literal for all twenty-one would make them
        // indistinguishable to a caller.
        //
        // `promoted_kind` is the one arm's payload discriminant,
        // beside the tag rather than in place of it: the word
        // `recognition_ambiguous` names the condition, and which
        // analytic kind's estimator declined is the second question,
        // with its own recourse. `None` on every other arm, which is
        // this surface's every-attribute-always-present rule.
        Err(err) => Err(typed_err(
            py,
            ErrorClass::StepImport,
            err.to_string(),
            &[
                (
                    "variant",
                    PyString::new(py, step_import_error_tag(&err))
                        .unbind()
                        .into_any(),
                ),
                (
                    "promoted_kind",
                    match &err {
                        pncad::step_import::StepImportError::RecognitionAmbiguous {
                            kind, ..
                        } => PyString::new(py, promoted_kind_tag(kind))
                            .unbind()
                            .into_any(),
                        _ => py.None(),
                    },
                ),
            ],
        )),
    }
}

/// **The cooperative cancel token (spec D5)** — what a caller holds
/// to stop an evaluation it has already launched.
///
/// Shared, not copied: the token is a handle onto one flag, so the
/// thread that calls [`Self::cancel`] and the thread inside
/// [`super::value::evaluate`] are looking at the same bit. That is
/// why this class is FROZEN and `cancel` takes no mutable borrow —
/// setting the flag is not a mutation of the Python object, and a
/// token that had to be borrowed mutably could not be held by a
/// running evaluation at all.
///
/// **Cooperative, at NODE granularity.** The kernel reads the flag
/// between nodes (or between levels on the parallel schedule) and
/// never inside a kernel op, so `cancel()` does not abort the boolean
/// or the tessellation already in flight; it stops the run before the
/// next node starts. The token is one-way — there is no `reset`,
/// because the flag a canceled run observed cannot be un-observed and
/// a reusable token would let two runs disagree about what it meant.
/// Mint a new one per launch, which is what `evaluate` does when no
/// `cancel=` is passed.
///
/// What a canceled run ANSWERS is `Evaluation.canceled`, and the
/// answer is a partial result rather than a raise: see that property.
#[pyclass(frozen, module = "pncad")]
#[derive(Default)]
pub(crate) struct CancelToken {
    inner: d::CancelToken,
}

#[pymethods]
impl CancelToken {
    /// A fresh, un-canceled token.
    #[new]
    fn new() -> Self {
        Self::default()
    }

    /// **Request cancelation** — any thread, any time, idempotent.
    ///
    /// Setting the flag before the run starts is legal and is the
    /// deterministic case: the evaluation checks the token before its
    /// first node, so a pre-canceled token yields a run with the full
    /// `order()`, no results at all, and `canceled` true.
    fn cancel(&self) {
        self.inner.cancel();
    }

    /// Whether cancelation has been requested on THIS token.
    ///
    /// A property of the token, not of any run: it says the flag is
    /// set, never that an evaluation observed it. `Evaluation.canceled`
    /// is the run's own answer, and the two differ exactly when a run
    /// finished before the flag was set.
    #[getter]
    fn canceled(&self) -> bool {
        self.inner.is_canceled()
    }

    fn __repr__(&self) -> String {
        format!(
            "CancelToken(canceled={})",
            if self.canceled() { "True" } else { "False" }
        )
    }
}

impl CancelToken {
    /// The kernel token this handle carries — cloned, which shares
    /// the flag rather than copying it.
    pub(crate) fn token(&self) -> d::CancelToken {
        self.inner.clone()
    }
}

/// Evaluate a document, producing its per-node result DAG.
///
/// Total: evaluation never raises. Individual nodes may still have
/// failed — ask the returned object.
///
/// `resolver` is the DOCUMENT SEAM: what an `InstantiatePart` node
/// reaches the document it pins through. A `Workspace` IS a resolver
/// (`pncad::workspace`'s own impl), so the store is passed as itself.
/// `None` — the default — is a kernel-only evaluation, in which every
/// instantiate node refuses typed (`EvaluationError`, `kind ==
/// "part_no_resolver"`) rather than pretending a part is empty. The
/// parameter carries the kernel's ROLE name: resolving a reference is
/// the capability evaluation needs, and a workspace is today's only
/// thing that has it.
///
/// `prior` is the MEMO: a node whose content and naming keys match
/// its result in `prior` reuses that value instead of re-running its
/// op, so only the changed cone costs anything. Reuse is not a claim
/// the caller has to take on trust — `Evaluation.reused` and
/// `Evaluation.recomputed` count it, node for node.
///
/// The memo is PER DOCUMENT and node-id-keyed: the lookup finds
/// `prior`'s result for the SAME node id and then certifies it by
/// content. An evaluation of a different document is a legal prior
/// that reuses nothing — not because the ids miss, but because the
/// memo REFUSES it: an evaluation carries the id of the document it
/// was run on, and a prior of another document is dropped whole
/// before the schedule is built. Ids alone would not decide it: two
/// documents built from one recipe carry the SAME ids for the same
/// nodes, so every lookup would hit. The run stays total — every node
/// recomputed — and `Evaluation.reused` is 0. Use the prior
/// evaluation of THIS document.
///
/// **A memo hit is served WITHOUT re-running the seam's gates.** A
/// reused `InstantiatePart` node never asks the resolver, so the
/// AVAILABILITY refusals — `part_pin_mismatch`, `part_unresolved`,
/// and `part_no_resolver` with them — are raised only for nodes that
/// actually re-resolve. What the memo serves is what the document's
/// own `DocRef` PINS, certified by content key: it is never a
/// different part, and it is not re-checked against the store. Two
/// consequences, and both are real:
///
/// * Edit a part on disk and re-evaluate with a prior, and the run
///   succeeds serving the previously pinned body where a run without
///   the prior refuses `part_pin_mismatch`. Relative to the STORE
///   that is a stale answer; relative to the DOCUMENT it is the
///   pinned one.
/// * "A pin that moved refuses, and is never silently retargeted"
///   therefore holds for evaluations that cross the seam. It is not
///   weakened for the ones that do — nothing is retargeted either
///   way — but it is not RE-ASSERTED by a run that never asks.
///
/// Pass no prior when the question is "does this document still
/// resolve against the store as it stands".
///
/// `cancel` is the COOPERATIVE STOP: a [`CancelToken`] the caller
/// keeps a handle on, checked between nodes, so a run already under
/// way can be abandoned. Omitted, this door mints a fresh token
/// nobody holds — which is exactly what it did before the keyword
/// existed, and is why an evaluation without one is never canceled.
/// A canceled run is a PARTIAL ANSWER and not a raise: it returns
/// normally with `Evaluation.canceled` true, the full `order()`, and
/// results for the completed prefix only.
///
/// **The GIL is released for the kernel run**, and that is what makes
/// the token reachable rather than decorative: the flag is set by
/// ANOTHER Python thread while this one is inside the evaluation, and
/// a thread that cannot run cannot set it. `doc` is borrowed for the
/// whole call, so a concurrent MUTATION of the same document —
/// `Doc.insert`, `Doc.apply`, any door needing `&mut` — raises
/// pyo3's own `RuntimeError("Already borrowed")` instead of editing
/// a recipe out from under a running evaluation. Measured, not
/// reasoned: `tests/test_cancellation.py` executes it.
#[pyfunction]
#[pyo3(signature = (doc, *, resolver=None, prior=None, cancel=None))]
pub(crate) fn evaluate(
    py: Python<'_>,
    doc: &super::doc::Doc,
    resolver: Option<&super::store::Workspace>,
    prior: Option<&Evaluation>,
    cancel: Option<&CancelToken>,
) -> Evaluation {
    let tol = Tol::witness();
    let opts = d::EvalOptions {
        resolver: resolver.map(super::store::Workspace::resolver),
        ..d::EvalOptions::default()
    };
    let token = cancel.map_or_else(d::CancelToken::new, CancelToken::token);
    let recipe = &doc.inner;
    let memo = prior.map(|p| &p.inner);
    let inner = py.detach(|| d::evaluate::<f64>(recipe, memo, &token, &opts, tol));
    Evaluation {
        inner,
        params: doc.inner.param_env::<f64>(),
        doc: doc.inner.clone(),
        product: crate::product_memo::ProductMemo::default(),
    }
}

/// Register the value surface on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<CancelToken>()?;
    m.add_class::<Evaluation>()?;
    m.add_class::<Value>()?;
    m.add_class::<Body>()?;
    m.add_class::<MassProperties>()?;
    m.add_class::<ImportReport>()?;
    m.add_class::<StructureNormalization>()?;
    m.add_class::<CurvePromotion>()?;
    m.add_class::<PlacedInstance>()?;
    m.add_class::<FaceCensus>()?;
    m.add_class::<ValidationFinding>()?;
    m.add_class::<Datum>()?;
    m.add_class::<Measurement>()?;
    m.add_class::<Verdict>()?;
    m.add_function(wrap_pyfunction!(evaluate, m)?)?;
    m.add_function(wrap_pyfunction!(import_step, m)?)?;
    Ok(())
}
