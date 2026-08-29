//! **Mates**: the declaration node's authored payload, the solve that
//! reads it, and the admission table that says how far a class gets.
//!
//! A mate is ONE node carrying both halves — the placement constraint
//! (which frames coincide, at which axis senses, at which clocking)
//! and the contact declaration (`Rest`, `Tangent`) — so there is no
//! second vocabulary to keep synced. Authoring one is
//! [`super::doc::Node`]'s `mate` constructor; everything the solve
//! and the gate then say about it is here.
//!
//! # The solve is TOTAL, so its faults are VALUES
//!
//! [`solve_document`] never raises. A refusing cluster must not fail
//! an unrelated one, so the refusal is recorded per node and read back
//! through [`SolvedPoses::fault`] as a [`MateFault`] value. The one
//! door that RAISES is [`SolvedPoses::placement`], which must answer
//! with a frame or not at all — and it raises `MateError` carrying
//! that same value under `fault`, so there is one payload vocabulary
//! rather than a value type and an exception type that can disagree.
//!
//! # What Python cannot reach here, and why
//!
//! `MateFault`'s `mate_class_not_admitted` arm fires for a class
//! outside v1's vocabulary. The kernel's contact vocabulary is `Rest`
//! and `Tangent` today and the Python mirror carries both, so nothing
//! Python can author reaches that arm — it is bound, tagged and
//! unreachable until the kernel grows a class. `class_admission` is
//! the door that says so BEFORE an edit lands.

use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::errors::ErrorClass;
use crate::py::typed_err;
use crate::tags::mate_fault_tag;
use pncad::document as d;
use pncad::tolerance::Tol;

use super::doc::NodeId;
use super::place::Frame;
use super::quantity::{Angle, Length};

/// Metres in, as the `Frame` family spells a position.
fn meters(v: (Length, Length, Length)) -> [f64; 3] {
    [v.0.0.meters(), v.1.0.meters(), v.2.0.meters()]
}

/// Metres out.
fn lengths(v: [f64; 3]) -> (Length, Length, Length) {
    let len = |x: f64| Length(pncad::quantity::Length::from_meters(x));
    (len(v[0]), len(v[1]), len(v[2]))
}

/// A kernel point as three lengths.
fn point(p: pncad::geom_core::Point3<f64>) -> (Length, Length, Length) {
    lengths([p.x, p.y, p.z])
}

/// A kernel vector as three plain numbers — a DIRECTION carries no
/// dimension, the `PatternKind.linear` convention.
fn direction(v: pncad::geom_core::Vec3<f64>) -> (f64, f64, f64) {
    (v.x, v.y, v.z)
}

// ---- The authored payload ----

/// One side's **mate frame**, in that instance's own part
/// coordinates: an origin, the primary axis (a planar rest's normal, a
/// coaxial mate's axis), and the clocking reference that fixes roll.
///
/// **Authored data, not geometry read back.** The solve is structural
/// plus decided predicates over exactly these numbers, so a frame that
/// does not match the face its mate names is a silent disagreement
/// this library cannot see (issue #944 — nothing mints an alignment
/// frame from a selected face yet).
///
/// `axis` need not be unit and `reference` need not be perpendicular
/// to it: only the axis's direction and the reference's perpendicular
/// part are read. Both are plain numbers — a direction has no
/// dimension — while `origin` is three lengths.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct MateFrame(pub(crate) d::MateFrame);

#[pymethods]
impl MateFrame {
    #[new]
    fn new(
        origin: (Length, Length, Length),
        axis: (f64, f64, f64),
        reference: (f64, f64, f64),
    ) -> Self {
        Self(d::MateFrame {
            origin: meters(origin),
            axis: [axis.0, axis.1, axis.2],
            reference: [reference.0, reference.1, reference.2],
        })
    }

    /// The frame's origin, in the part's own coordinates.
    #[getter]
    fn origin(&self) -> (Length, Length, Length) {
        lengths(self.0.origin)
    }

    /// The primary axis, as authored (not normalised).
    #[getter]
    fn axis(&self) -> (f64, f64, f64) {
        (self.0.axis[0], self.0.axis[1], self.0.axis[2])
    }

    /// The clocking reference, as authored.
    #[getter]
    fn reference(&self) -> (f64, f64, f64) {
        (
            self.0.reference[0],
            self.0.reference[1],
            self.0.reference[2],
        )
    }

    /// The rigid placement this frame denotes: local +Z is `axis`,
    /// the local origin is `origin`, roll fixed by `reference`.
    ///
    /// Raises `FrameError` when the axis has no definite direction or
    /// the reference has no definite perpendicular offset from it —
    /// the same refusal the solve meets, reachable BEFORE authoring
    /// the mate that would carry it.
    fn placement(&self, py: Python<'_>) -> PyResult<Frame> {
        let tol = Tol::witness();
        self.0
            .placement(tol)
            .map(|affine| Frame(d::Frame::from_affine(affine)))
            .map_err(|err| super::place::frame_err(py, &err))
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __repr__(&self) -> String {
        format!(
            "MateFrame(origin={:?}, axis={:?}, reference={:?})",
            self.0.origin, self.0.axis, self.0.reference
        )
    }
}

/// Which way the two sides' axes point at each other.
///
/// `Opposed` is what kills every π-flip ambiguity: the senses are
/// AUTHORED, never inferred.
#[pyclass(eq, eq_int, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `editor_core::AxisSense` variant of the same name"
)]
pub(crate) enum AxisSense {
    Aligned,
    Opposed,
}

impl AxisSense {
    fn to_kernel(self) -> d::AxisSense {
        match self {
            Self::Aligned => d::AxisSense::Aligned,
            Self::Opposed => d::AxisSense::Opposed,
        }
    }

    pub(crate) fn from_kernel(sense: d::AxisSense) -> Self {
        match sense {
            d::AxisSense::Aligned => Self::Aligned,
            d::AxisSense::Opposed => Self::Opposed,
        }
    }
}

/// Which side of a mate a diagnostic is about.
#[pyclass(eq, eq_int, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `editor_core::MateSide` variant of the same name"
)]
pub(crate) enum MateSide {
    A,
    B,
}

impl MateSide {
    pub(crate) fn from_kernel(side: d::MateSide) -> Self {
        match side {
            d::MateSide::A => Self::A,
            d::MateSide::B => Self::B,
        }
    }
}

/// The **mate primitive**: which coset of rigid motions this mate pins
/// the pair's relative pose to.
///
/// Four constructors and no bare enum, because one of them carries a
/// length: `planar_rest` takes the signed standoff along `a`'s axis
/// (zero is the flush rest; nonzero is an authored standoff, never a
/// designed clearance). `clocking` is representable precisely so it
/// can be REFUSED — the coset table has no entry for a bare angular
/// relation, and an unrepresentable refusal is an untestable one.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct MatePrimitive(pub(crate) d::MatePrimitive);

#[pymethods]
impl MatePrimitive {
    /// The two mate frames coincide outright — residual trivial.
    #[staticmethod]
    fn frame_coincidence() -> Self {
        Self(d::MatePrimitive::FrameCoincidence)
    }

    /// The two axes coincide as a LINE — residual cylindrical.
    #[staticmethod]
    fn coaxial() -> Self {
        Self(d::MatePrimitive::Coaxial)
    }

    /// `b`'s plane rests on `a`'s, displaced by `offset` along `a`'s
    /// axis — residual planar.
    #[staticmethod]
    fn planar_rest(offset: &Length) -> Self {
        Self(d::MatePrimitive::PlanarRest {
            offset: offset.0.meters(),
        })
    }

    /// Clocking with NO carrying primitive: the table lacks the entry
    /// by design, so a mate authored with this refuses at the solve.
    #[staticmethod]
    fn clocking() -> Self {
        Self(d::MatePrimitive::Clocking)
    }

    /// The primitive's stable tag: `frame_coincidence`, `coaxial`,
    /// `planar_rest`, `clocking`.
    #[getter]
    fn variant(&self) -> &'static str {
        primitive_tag(self.0)
    }

    /// The planar rest's signed standoff, `None` for every other
    /// primitive — the attribute is present on all four (the payload
    /// posture the typed refusals use), so reading it never needs a
    /// branch on `variant` first.
    #[getter]
    fn offset(&self) -> Option<Length> {
        match self.0 {
            d::MatePrimitive::PlanarRest { offset } => {
                Some(Length(pncad::quantity::Length::from_meters(offset)))
            }
            _ => None,
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __repr__(&self) -> String {
        match self.0 {
            d::MatePrimitive::PlanarRest { offset } => {
                format!("MatePrimitive.planar_rest({offset} m)")
            }
            other => format!("MatePrimitive.{}()", primitive_tag(other)),
        }
    }
}

/// The stable tag for a mate primitive. Exhaustive over the kernel
/// enum, so a primitive added there stops this build.
fn primitive_tag(primitive: d::MatePrimitive) -> &'static str {
    match primitive {
        d::MatePrimitive::FrameCoincidence => "frame_coincidence",
        d::MatePrimitive::Coaxial => "coaxial",
        d::MatePrimitive::PlanarRest { .. } => "planar_rest",
        d::MatePrimitive::Clocking => "clocking",
    }
}

/// The **alignment datum**: which frames coincide, at which axis
/// sense, with which clocking rider.
///
/// `clocking` is a RIDER, never a primitive: on `coaxial` it cuts the
/// residual to prismatic along the axis; on `frame_coincidence` it is
/// redundant-or-contradictory and gets decided; on a planar rest the
/// table has no entry and the solve refuses typed.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct Alignment(pub(crate) d::Alignment);

#[pymethods]
impl Alignment {
    #[new]
    #[pyo3(signature = (a, b, primitive, sense, clocking=None))]
    fn new(
        a: &MateFrame,
        b: &MateFrame,
        primitive: &MatePrimitive,
        sense: AxisSense,
        clocking: Option<Angle>,
    ) -> Self {
        Self(d::Alignment {
            a: a.0,
            b: b.0,
            primitive: primitive.0,
            sense: sense.to_kernel(),
            clocking: clocking.map(|angle| angle.0.radians()),
        })
    }

    /// The `a` side's mate frame, in `a`'s part coordinates.
    #[getter]
    fn a(&self) -> MateFrame {
        MateFrame(self.0.a)
    }

    /// The `b` side's mate frame, in `b`'s part coordinates.
    #[getter]
    fn b(&self) -> MateFrame {
        MateFrame(self.0.b)
    }

    /// Which coset this mate pins.
    #[getter]
    fn primitive(&self) -> MatePrimitive {
        MatePrimitive(self.0.primitive)
    }

    /// Which way the axes point at each other.
    #[getter]
    fn sense(&self) -> AxisSense {
        AxisSense::from_kernel(self.0.sense)
    }

    /// The clocking rider, `None` when unclocked.
    #[getter]
    fn clocking(&self) -> Option<Angle> {
        self.0
            .clocking
            .map(|r| Angle(pncad::quantity::Angle::from_radians(r)))
    }

    /// The **lever arm** this mate's angular decisions turn on: the
    /// largest distance in its own authored data over which an angular
    /// error accumulates into a gap. Floored at one metre, so a mate
    /// authored AT the origin cannot claim an arbitrarily tight
    /// angular threshold.
    #[getter]
    fn lever_arm(&self) -> Length {
        Length(pncad::quantity::Length::from_meters(self.0.lever_arm()))
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __repr__(&self) -> String {
        format!(
            "Alignment(primitive={}, sense={:?}, clocking={:?})",
            primitive_tag(self.0.primitive),
            self.0.sense,
            self.0.clocking
        )
    }
}

// ---- The admission table ----

/// **How far a contact class gets in v1**, as a value both enforcing
/// doors read.
///
/// The two doors want different things of a class: the solve needs a
/// coset the alignment table can fold, and the assembly gate's mint
/// needs a kernel record type that can carry the declaration at rest.
/// A class can satisfy the first and not the second — which is why a
/// tool asks this table BEFORE committing an edit rather than
/// discovering the refusal after it lands.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct ClassAdmission(d::ClassAdmission);

#[pymethods]
impl ClassAdmission {
    /// The stable tag: `mints`, `no_at_rest_record`, `not_admitted`.
    #[getter]
    fn variant(&self) -> &'static str {
        match self.0 {
            d::ClassAdmission::Mints => "mints",
            d::ClassAdmission::NoAtRestRecord { .. } => "no_at_rest_record",
            d::ClassAdmission::NotAdmitted => "not_admitted",
        }
    }

    /// Whether both doors admit the class: the solve folds it AND the
    /// gate mints it into the product's record set.
    #[getter]
    fn mints(&self) -> bool {
        matches!(self.0, d::ClassAdmission::Mints)
    }

    /// Whether the SOLVE admits it — `mints` or `no_at_rest_record`.
    /// A class the solve refuses never reaches the gate at all.
    #[getter]
    fn solves(&self) -> bool {
        !matches!(self.0, d::ClassAdmission::NotAdmitted)
    }

    /// Why the assembly gate carries nothing at rest for this class,
    /// in the class's own terms. Present on every arm — the deferral
    /// sentence for `not_admitted`, the table's own reason for
    /// `no_at_rest_record`, and `None` for `mints`, which carries one.
    #[getter]
    fn why(&self) -> Option<&'static str> {
        match self.0 {
            d::ClassAdmission::Mints => None,
            d::ClassAdmission::NoAtRestRecord { why } => Some(why),
            d::ClassAdmission::NotAdmitted => Some(d::CLASS_DEFERRAL),
        }
    }

    fn __repr__(&self) -> String {
        format!("ClassAdmission({:?})", self.variant())
    }
}

/// How far `class_` gets in v1 — the table, read.
///
/// The whole class policy as one value, so a mate-authoring tool can
/// offer only what the vocabulary can execute. Nothing is restated
/// here: `assemble` and `solve_document` read this same table.
#[pyfunction]
pub(crate) fn class_admission(
    py: Python<'_>,
    class_: super::flush::ContactClass,
) -> PyResult<ClassAdmission> {
    Ok(ClassAdmission(d::class_admission(class_.to_kernel(py)?)))
}

// ---- The solve's read side ----

/// What a mate did in the solve.
#[pyclass(eq, eq_int, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `editor_core::MateRole` variant of the same name"
)]
pub(crate) enum MateRole {
    Determining,
    Declaring,
    Refused,
}

impl MateRole {
    fn from_kernel(role: d::MateRole) -> Self {
        match role {
            d::MateRole::Determining => Self::Determining,
            d::MateRole::Declaring => Self::Declaring,
            d::MateRole::Refused => Self::Refused,
        }
    }
}

/// A residual subgroup: what a fold left free.
///
/// The payload attributes — `normal`, `point`, `direction` — are
/// present on every arm and `None` where that arm does not carry one,
/// so reading `residual.direction` never needs a branch on `variant`
/// first. `point` is three LENGTHS (a point on a line); `normal` and
/// `direction` are plain unit numbers.
///
/// **Point-free arms are point-free on purpose.** `planar` and
/// `prismatic` carry no base point because rotations about any
/// parallel axis, and translations along any parallel line, are in the
/// group — a base point would suggest a distinction the algebra does
/// not make.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct Subgroup(d::Subgroup);

#[pymethods]
impl Subgroup {
    /// The stable tag: `se3`, `planar`, `cylindrical`, `prismatic`,
    /// `revolute`, `trivial`, `empty`.
    #[getter]
    fn variant(&self) -> &'static str {
        match self.0 {
            d::Subgroup::Se3 => "se3",
            d::Subgroup::Planar { .. } => "planar",
            d::Subgroup::Cylindrical { .. } => "cylindrical",
            d::Subgroup::Prismatic { .. } => "prismatic",
            d::Subgroup::Revolute { .. } => "revolute",
            d::Subgroup::Trivial => "trivial",
            d::Subgroup::Empty => "empty",
        }
    }

    /// The plane's unit normal, for `planar`.
    #[getter]
    fn normal(&self) -> Option<(f64, f64, f64)> {
        match self.0 {
            d::Subgroup::Planar { normal } => Some(direction(normal)),
            _ => None,
        }
    }

    /// A point on the axis, for `cylindrical` and `revolute`.
    #[getter]
    fn point(&self) -> Option<(Length, Length, Length)> {
        match self.0 {
            d::Subgroup::Cylindrical { point: p, .. } | d::Subgroup::Revolute { point: p, .. } => {
                Some(point(p))
            }
            _ => None,
        }
    }

    /// The unit direction, for `cylindrical`, `prismatic`, `revolute`.
    #[getter]
    fn direction(&self) -> Option<(f64, f64, f64)> {
        match self.0 {
            d::Subgroup::Cylindrical { direction: v, .. }
            | d::Subgroup::Prismatic { direction: v }
            | d::Subgroup::Revolute { direction: v, .. } => Some(direction(v)),
            _ => None,
        }
    }

    fn __repr__(&self) -> String {
        format!("Subgroup({:?})", self.variant())
    }
}

/// Why the solve refused for one node — a VALUE, because the solve is
/// total and records a refusal per node rather than failing the
/// document.
///
/// Every payload attribute is present on every arm, `None` where the
/// arm does not carry it: `mate`, `side`, `head`, `instance`,
/// `parent`, `child`, `residual`, `held`, `added`, `predicate`,
/// `clash`, `what`. The human message is the kernel's own prose,
/// available as `str(fault)`.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct MateFault(pub(crate) d::MateFault);

#[pymethods]
impl MateFault {
    /// The stable tag of the refusing arm.
    #[getter]
    fn variant(&self) -> &'static str {
        mate_fault_tag(&self.0)
    }

    /// The mate the fault is about, `None` for the two arms whose
    /// subject is not one mate (`mate_band`, and `mate_contradictory`,
    /// which names `held` and `added` instead).
    #[getter]
    fn mate(&self) -> Option<NodeId> {
        use d::MateFault as F;
        match &self.0 {
            F::Frame { mate, .. }
            | F::ClassNotAdmitted { mate }
            | F::TableLacks { mate, .. }
            | F::Indeterminate { mate, .. }
            | F::Under { mate, .. }
            | F::DanglingHead { mate, .. }
            | F::SelfMate { mate, .. } => Some(NodeId(*mate)),
            F::Band { .. } | F::Contradictory { .. } => None,
        }
    }

    /// Which side of the mate refused.
    #[getter]
    fn side(&self) -> Option<MateSide> {
        use d::MateFault as F;
        match &self.0 {
            F::Frame { side, .. } | F::DanglingHead { side, .. } => {
                Some(MateSide::from_kernel(*side))
            }
            _ => None,
        }
    }

    /// The instantiate node a dangling reference head claims.
    #[getter]
    fn head(&self) -> Option<NodeId> {
        match &self.0 {
            d::MateFault::DanglingHead { head, .. } => Some(NodeId(*head)),
            _ => None,
        }
    }

    /// The instance a self-mate names twice.
    #[getter]
    fn instance(&self) -> Option<NodeId> {
        match &self.0 {
            d::MateFault::SelfMate { instance, .. } => Some(NodeId(*instance)),
            _ => None,
        }
    }

    /// The instance an under-determined tree mate extended FROM.
    #[getter]
    fn parent(&self) -> Option<NodeId> {
        match &self.0 {
            d::MateFault::Under { parent, .. } => Some(NodeId(*parent)),
            _ => None,
        }
    }

    /// The instance it failed to place.
    #[getter]
    fn child(&self) -> Option<NodeId> {
        match &self.0 {
            d::MateFault::Under { child, .. } => Some(NodeId(*child)),
            _ => None,
        }
    }

    /// What survived an under-determined fold.
    #[getter]
    fn residual(&self) -> Option<Subgroup> {
        match &self.0 {
            d::MateFault::Under { residual, .. } => Some(Subgroup(residual.clone())),
            _ => None,
        }
    }

    /// The mate already folded, for a contradictory pair.
    #[getter]
    fn held(&self) -> Option<NodeId> {
        match &self.0 {
            d::MateFault::Contradictory { held, .. } => Some(NodeId(*held)),
            _ => None,
        }
    }

    /// The mate whose intersection died against it.
    #[getter]
    fn added(&self) -> Option<NodeId> {
        match &self.0 {
            d::MateFault::Contradictory { added, .. } => Some(NodeId(*added)),
            _ => None,
        }
    }

    /// The predicate that decided against a contradictory pair.
    #[getter]
    fn predicate(&self) -> Option<&'static str> {
        match &self.0 {
            d::MateFault::Contradictory { predicate, .. } => Some(predicate),
            _ => None,
        }
    }

    /// The measured clash: the margin that should have been zero and
    /// was not.
    #[getter]
    fn clash(&self) -> Option<Length> {
        match &self.0 {
            d::MateFault::Contradictory { clash, .. } => {
                Some(Length(pncad::quantity::Length::from_meters(*clash)))
            }
            _ => None,
        }
    }

    /// What the coset table was asked for, in its own words.
    #[getter]
    fn what(&self) -> Option<&'static str> {
        match &self.0 {
            d::MateFault::TableLacks { what, .. } => Some(what),
            _ => None,
        }
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("MateFault({:?})", mate_fault_tag(&self.0))
    }
}

/// Raise `MateError` for a solve refusal.
///
/// The exception carries `variant` — the house payload every typed
/// refusal has — and `fault`, the value itself. The per-arm payload
/// lives on the value rather than being copied flat onto the
/// exception, because [`SolvedPoses::fault`] hands the SAME value back
/// without raising: two spellings of one payload is exactly the drift
/// a single vocabulary avoids.
pub(crate) fn mate_err(py: Python<'_>, fault: &d::MateFault) -> PyErr {
    let value = Py::new(py, MateFault(fault.clone()))
        .map(|v| v.into_any())
        .unwrap_or_else(|_| py.None());
    typed_err(
        py,
        ErrorClass::Mate,
        fault.to_string(),
        &[
            (
                "variant",
                PyString::new(py, mate_fault_tag(fault)).unbind().into_any(),
            ),
            ("fault", value),
        ],
    )
}

/// The document's solved poses: each instance's pose relative to its
/// cluster gauge, each mate's role, and the per-node refusals.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct SolvedPoses(d::SolvedPoses);

#[pymethods]
impl SolvedPoses {
    /// The node's recorded fault, `None` if the solve did not refuse
    /// for it.
    ///
    /// Recorded against the refusing MATE and against every instance
    /// in its cluster that consequently has no pose — a refusal
    /// reaches the nodes it actually affects and no further.
    fn fault(&self, node: &NodeId) -> Option<MateFault> {
        self.0.fault(node.0).cloned().map(MateFault)
    }

    /// A mate's role, `None` if the node is not a live mate.
    fn role(&self, mate: &NodeId) -> Option<MateRole> {
        self.0.role(mate.0).map(MateRole::from_kernel)
    }

    /// An instance's cluster gauge, `None` if the node is not a live
    /// instance. A singleton cluster is its own gauge.
    fn gauge(&self, instance: &NodeId) -> Option<NodeId> {
        self.0.gauge(instance.0).map(NodeId)
    }

    /// An instance's pose RELATIVE TO ITS CLUSTER GAUGE. The gauge's
    /// own entry is the identity, bit-exactly.
    fn relative(&self, instance: &NodeId) -> Option<Frame> {
        self.0.relative(instance.0).map(Frame)
    }

    /// **The instance's world placement**: the cluster's recorded
    /// frame composed onto the solved relative pose.
    ///
    /// A singleton cluster returns its recorded frame VERBATIM — the
    /// mate-less document's placement is bit-for-bit what it was
    /// before mates existed.
    ///
    /// `doc` is read for its placement registry, and it must be the
    /// document this solve is OF: passing a different one composes
    /// this document's relative poses onto that one's cluster frames,
    /// which is not a pose of either. Nothing here can check that —
    /// a document carries no identity of the solve that read it.
    ///
    /// Raises `MateError` when the instance's cluster did not solve.
    fn placement(
        &self,
        py: Python<'_>,
        doc: &super::doc::Doc,
        instance: &NodeId,
    ) -> PyResult<Frame> {
        self.0
            .placement(&doc.inner, instance.0)
            .map(Frame)
            .map_err(|fault| mate_err(py, &fault))
    }

    fn __repr__(&self) -> String {
        "SolvedPoses()".to_string()
    }
}

/// Solve the document's mates: the per-pair coset fold along a
/// deterministic spanning tree, yielding every instance's pose
/// relative to its cluster gauge and every mate's role.
///
/// **Total — this never raises.** A refusing cluster must not fail an
/// unrelated one, so refusals are recorded per node and read back
/// through `SolvedPoses.fault`.
///
/// Nothing here inspects geometry: the solve is recipe data plus
/// decided predicates over the authored alignment numbers. In
/// particular it does NOT check that a mate's frames match the faces
/// its references name — that is issue #944, and it is why a document
/// can solve cleanly and still refuse at the at-rest gate.
#[pyfunction]
pub(crate) fn solve_document(doc: &super::doc::Doc) -> SolvedPoses {
    let tol = Tol::witness();
    SolvedPoses(d::solve_document(&doc.inner, tol))
}

/// The **placement clusters**: instances coupled by mates, each
/// listed with its cluster's members in document order.
///
/// The partition placement is keyed by. A mate-less document's
/// clusters are all singletons, which is why placement stayed
/// per-instance before mates existed.
#[pyfunction]
pub(crate) fn clusters(doc: &super::doc::Doc) -> Vec<Vec<NodeId>> {
    d::clusters(&doc.inner)
        .into_iter()
        .map(|c| c.into_iter().map(NodeId).collect())
        .collect()
}

/// An instance's cluster GAUGE: the document-order-first instance of
/// its cluster, whose recorded frame places the whole cluster.
///
/// Answers the instance itself for a node that is not in any cluster,
/// which is the kernel's own total shape — a singleton is its own
/// gauge and a non-instance has no cluster to be second in.
#[pyfunction]
pub(crate) fn gauge_of(doc: &super::doc::Doc, instance: &NodeId) -> NodeId {
    NodeId(d::gauge_of(&doc.inner, instance.0))
}

/// The **reading edges**: for each mate, the instantiate node each of
/// its references resolves through.
///
/// Recomputed from the name heads every time, never stored — a mate's
/// references are not recipe edges (inserting a mate transfers no
/// root), and this is the second sort of edge the partition reads on
/// top of the consuming ones.
#[pyfunction]
pub(crate) fn reading_edges(doc: &super::doc::Doc) -> Vec<(NodeId, NodeId)> {
    d::reading_edges(&doc.inner)
        .into_iter()
        .map(|(a, b)| (NodeId(a), NodeId(b)))
        .collect()
}

/// The **relative-freedom partition**: components over consuming ∪
/// reading edges, so mates couple what they constrain.
///
/// Coarser than `clusters`, which partitions instances alone.
#[pyfunction]
pub(crate) fn relative_freedom_components(doc: &super::doc::Doc) -> Vec<Vec<NodeId>> {
    d::relative_freedom_components(&doc.inner)
        .into_iter()
        .map(|c| c.into_iter().map(NodeId).collect())
        .collect()
}

// ---- Cluster-record maintenance ----

/// One recorded act of cluster-record maintenance: what an ordinary
/// edit's motion of the mate graph forced on the placement registry.
///
/// It rides the accepted edit rather than being an edit of its own —
/// automatic maintenance is the invariant's own bookkeeping,
/// deterministic from the edit, so a replay reproduces it and undo
/// (keeping the prior document) restores it exactly. What the record
/// adds is VISIBILITY: an absorbed cluster's frame is consumed here,
/// where a caller can read what was consumed.
///
/// Payload attributes are present on every arm, `None` where
/// inapplicable: `survived`, `absorbed`, `absorbed_frame`, `source`,
/// `target`, `frame`, `gauge`. (`source`/`target` rather than
/// `from`/`to`: `from` is a Python keyword.)
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct ClusterMaintenance(pub(crate) d::ClusterMaintenance);

#[pymethods]
impl ClusterMaintenance {
    /// The stable tag: `join`, `split`, `gauge_rewrite`, `drop`.
    #[getter]
    fn variant(&self) -> &'static str {
        use d::ClusterMaintenance as M;
        match self.0 {
            M::Join { .. } => "join",
            M::Split { .. } => "split",
            M::GaugeRewrite { .. } => "gauge_rewrite",
            M::Drop { .. } => "drop",
        }
    }

    /// The gauge that survived a join: the earlier of the two.
    #[getter]
    fn survived(&self) -> Option<NodeId> {
        match self.0 {
            d::ClusterMaintenance::Join { survived, .. } => Some(NodeId(survived)),
            _ => None,
        }
    }

    /// The absorbed cluster's former gauge.
    #[getter]
    fn absorbed(&self) -> Option<NodeId> {
        match self.0 {
            d::ClusterMaintenance::Join { absorbed, .. } => Some(NodeId(absorbed)),
            _ => None,
        }
    }

    /// The absorbed cluster's frame, consumed into this record —
    /// `None` when the row was absent (the identity).
    #[getter]
    fn absorbed_frame(&self) -> Option<Frame> {
        match self.0 {
            d::ClusterMaintenance::Join {
                absorbed_frame: f, ..
            } => f.map(Frame),
            _ => None,
        }
    }

    /// The gauge a split separated FROM, or a rewrite's dead gauge.
    #[getter]
    fn source(&self) -> Option<NodeId> {
        use d::ClusterMaintenance as M;
        match self.0 {
            M::Split { from, .. } | M::GaugeRewrite { from, .. } => Some(NodeId(from)),
            _ => None,
        }
    }

    /// The new cluster's gauge, or the dead gauge's successor.
    #[getter]
    fn target(&self) -> Option<NodeId> {
        use d::ClusterMaintenance as M;
        match self.0 {
            M::Split { to, .. } | M::GaugeRewrite { to, .. } => Some(NodeId(to)),
            _ => None,
        }
    }

    /// The minted, rewritten or dropped frame, `None` for the
    /// identity.
    #[getter]
    fn frame(&self) -> Option<Frame> {
        use d::ClusterMaintenance as M;
        match self.0 {
            M::Split { frame, .. } | M::GaugeRewrite { frame, .. } | M::Drop { frame, .. } => {
                frame.map(Frame)
            }
            _ => None,
        }
    }

    /// The dead gauge whose record went with its last instance.
    #[getter]
    fn gauge(&self) -> Option<NodeId> {
        match self.0 {
            d::ClusterMaintenance::Drop { gauge, .. } => Some(NodeId(gauge)),
            _ => None,
        }
    }

    fn __repr__(&self) -> String {
        format!("ClusterMaintenance({:?})", self.variant())
    }
}

/// Register the mate surface on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MateFrame>()?;
    m.add_class::<MatePrimitive>()?;
    m.add_class::<Alignment>()?;
    m.add_class::<AxisSense>()?;
    m.add_class::<MateSide>()?;
    m.add_class::<MateRole>()?;
    m.add_class::<Subgroup>()?;
    m.add_class::<MateFault>()?;
    m.add_class::<SolvedPoses>()?;
    m.add_class::<ClassAdmission>()?;
    m.add_class::<ClusterMaintenance>()?;
    m.add_function(wrap_pyfunction!(solve_document, m)?)?;
    m.add_function(wrap_pyfunction!(clusters, m)?)?;
    m.add_function(wrap_pyfunction!(gauge_of, m)?)?;
    m.add_function(wrap_pyfunction!(reading_edges, m)?)?;
    m.add_function(wrap_pyfunction!(relative_freedom_components, m)?)?;
    m.add_function(wrap_pyfunction!(class_admission, m)?)?;
    m.add("CLASS_DEFERRAL", d::CLASS_DEFERRAL)?;
    m.add("UNDER_RECOURSE", d::UNDER_RECOURSE)?;
    Ok(())
}
