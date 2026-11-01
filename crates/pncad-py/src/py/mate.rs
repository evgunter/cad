//! **Mates**: a placement's constraints as values, the solve that
//! reads them, and the admission table that says how far a class gets.
//!
//! A mate is ONE value carrying both halves — the placement constraint
//! (two poses of one kind equated) and the contact declaration
//! (`Rest`, `Tangent`) — so there is no second vocabulary to keep
//! synced. A value sets one freedom a mate leaves. Each is a constraint
//! of the placement that holds it, addressed by a [`MateRef`], never a
//! node; everything the solve and the checks then say about it is here.
//!
//! # The solve is TOTAL, so its faults are VALUES
//!
//! [`solve_document`] never raises. A refusing bundle must not fail
//! an unrelated one, so the refusal is recorded per placement and read back
//! through [`SolvedPoses::fault`] as a [`MateFault`] value. The one
//! door that RAISES is [`SolvedPoses::placement`], which must answer
//! with a frame or not at all — and it raises `MateError` carrying
//! that same value under `fault`, so there is one payload vocabulary
//! rather than a value type and an exception type that can disagree.

use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::errors::ErrorClass;
use crate::py::typed_err;
use crate::tags::{coord_tag, maintenance_tag, mate_fault_tag, no_chart_why_tag};
use std::sync::Arc;

use pncad::document as d;
use pncad::tolerance::Tol;

use super::doc::NodeId;
use super::place::Frame;

/// Which pose of a mate a diagnostic is about: `On`, the copy's, or
/// `To`, the pose it is equated with.
#[pyclass(eq, eq_int, frozen, hash, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `editor_core::MateSide` variant of the same name"
)]
pub(crate) enum MateSide {
    On,
    To,
}

impl MateSide {
    pub(crate) fn from_kernel(side: d::MateSide) -> Self {
        match side {
            d::MateSide::On => Self::On,
            d::MateSide::To => Self::To,
        }
    }
}

// ---- The authored payload ----

/// **A mate**: two poses of one kind equated.
/// A mate is a constraint of the placement that holds it
/// (`Doc.constrain`, `Doc.place(shapes, constraints)`), never a node:
/// this is its payload as a value.
///
/// `on` is a pose read off the copied shapes' geometry (a `PoseDef`
/// over one of the placement's shapes, or over the instance itself);
/// `to` is a pose of the space the copy joins, read off geometry
/// there: a mate reads only geometry, never the world's frame. The
/// kind is the primitive: plane to plane rests, axis to axis is
/// coaxial, frame to frame coincides, point to point is a ball, and
/// direction to direction is parallel. A plain plane mate equates
/// outward normals, so two faces resting against each other carry a
/// flip on the target (`PoseDef.flip`). A mate holds no number: what
/// it leaves free is set by a `MateValue`, and a standoff is a slide of the
/// target plane along its own normal. A mate declares no contact: where
/// the copies it places touch is the at-rest census's to record.
///
/// `orient` is the mate's `Orientation`, where the copy's
/// axes lie along the target space's: the identity unless written, and
/// read only beside a mate fixing a direction (a plane, an axis, a
/// direction) — a point or a frame mate written with any other refuses
/// `mate_unoriented`.
///
/// Where it is added, the door refuses an `on` that does not read the
/// copy (`mate_not_on_the_copy`) and poses of two kinds
/// (`mate_kinds_differ`). A pose that does not evaluate — a face gone,
/// a curved face read as a plane — is the placement's own refusal
/// where it evaluates (`mate_side_unevaluated`).
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct Mate(pub(crate) d::Mate<d::Formula>);

#[pymethods]
impl Mate {
    #[new]
    #[pyo3(signature = (on, to, *, orient=None))]
    fn new(
        on: super::doc::OperandArg,
        to: super::doc::OperandArg,
        orient: Option<Orientation>,
    ) -> Self {
        Self(d::Mate {
            on: on.read(),
            to: to.read(),
            orient: orient.map_or(d::Orientation::IDENTITY, |o| o.0),
        })
    }

    /// The copy's pose, as written.
    #[getter]
    fn on(&self) -> super::pose::PoseDef {
        super::pose::PoseDef {
            inner: self.0.on.clone(),
        }
    }

    /// The pose the copy's is equated with, as written.
    #[getter]
    fn to(&self) -> super::pose::PoseDef {
        super::pose::PoseDef {
            inner: self.0.to.clone(),
        }
    }

    /// The orientation, as written.
    #[getter]
    fn orient(&self) -> Orientation {
        Orientation(self.0.orient)
    }

    /// BIT-exact equality: the same poses, the same orientation.
    fn __eq__(&self, other: &Self) -> bool {
        self.0.bit_eq(&other.0)
    }

    fn __repr__(&self) -> String {
        format!(
            "Mate(on={}, to={}, orient={})",
            self.0.on,
            self.0.to,
            Orientation(self.0.orient).__repr__()
        )
    }
}

/// **An orientation**: one of the 24 ways the copy's axes lie along the
/// target space's, by where its `x` and its `z` lie — `Orientation("+x",
/// "-z")` puts the copy's `x` along the target's `+x` and its `z` along
/// `-z` (its `y` is `z × x`). A mate fixing a direction carries one; the
/// value chart's least turn carries where it puts the copy's mated
/// direction onto the mated direction, and the spin is what is left.
/// Raises ValueError for two axes of one letter, or a word that is no
/// signed axis.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct Orientation(pub(crate) d::Orientation);

#[pymethods]
impl Orientation {
    #[new]
    fn new(x: &str, z: &str) -> PyResult<Self> {
        d::Orientation::new(signed_axis(x)?, signed_axis(z)?)
            .map(Self)
            .ok_or_else(|| {
                pyo3::exceptions::PyValueError::new_err(format!(
                    "an orientation's x and z lie along two axes, not {x:?} and {z:?}"
                ))
            })
    }

    /// The copy's axes along the target's own.
    #[staticmethod]
    fn identity() -> Self {
        Self(d::Orientation::IDENTITY)
    }

    /// Where the copy's `x` lies: `"+x"` … `"-z"`.
    #[getter]
    fn x(&self) -> &'static str {
        self.0.x().name()
    }

    /// Where the copy's `y` lies.
    #[getter]
    fn y(&self) -> &'static str {
        self.0.y().name()
    }

    /// Where the copy's `z` lies.
    #[getter]
    fn z(&self) -> &'static str {
        self.0.z().name()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }

    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        self.0.hash(&mut h);
        h.finish()
    }

    fn __repr__(&self) -> String {
        format!(
            "Orientation({:?}, {:?})",
            self.0.x().name(),
            self.0.z().name()
        )
    }
}

/// A signed axis by its word: `"+x"`, `"-x"`, … `"-z"`.
fn signed_axis(word: &str) -> PyResult<d::SignedAxis> {
    d::SignedAxis::ALL
        .into_iter()
        .find(|a| a.name() == word)
        .ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err(format!(
                "not a signed axis: {word:?} — one of \"+x\", \"-x\", \"+y\", \"-y\", \"+z\" or \"-z\""
            ))
        })
}

/// **Which freedom a value sets**: `Coord.slide("x")`, `"y"` or
/// `"z"`, a slide along the chart's axis; `Coord.spin()`, the spin about
/// the target's own direction beside a mate fixing one; or `Coord.spin("x")`,
/// `"y"` or `"z"`, a link of a chain of turns about the target space's
/// axis where none is fixed.
///
/// Beside a mate fixing a direction, the chart is the target space's
/// own axes carried by the least turn from where the mate's
/// `Orientation` puts the copy's mated direction onto the mated one, and
/// the spin is the turn left about it. A plane mate leaves two slides
/// and the spin, an axis mate one slide and the spin, a direction mate
/// the three slides and the spin: where the copy's mated direction lies
/// on one of its axes, the slide the least turn carries onto the mated
/// direction is the one a plane fixes and an axis leaves. A point mate
/// leaves its rotation, a chain of spins about the target space's axes
/// through the mated point; a frame mate nothing. A bundle with no mate
/// leaves the three slides and a chain of spins about the target
/// space's axes, in the order written.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Coord(pub(crate) d::Coord);

#[pymethods]
impl Coord {
    /// The slide along the chart's `axis`: `"x"`, `"y"` or `"z"`.
    #[staticmethod]
    fn slide(axis: &str) -> PyResult<Self> {
        Ok(Self(d::Coord::Slide(target_axis(axis)?)))
    }

    /// The spin about the target's own direction, with no `axis`; or about the
    /// target space's `axis`, `"x"`, `"y"` or `"z"`, a link of a chain.
    #[staticmethod]
    #[pyo3(signature = (axis=None))]
    fn spin(axis: Option<&str>) -> PyResult<Self> {
        Ok(Self(d::Coord::Spin(axis.map(target_axis).transpose()?)))
    }

    /// The stable word: `slide_x`, `slide_y`, `slide_z`, `spin`,
    /// `spin_x`, `spin_y` or `spin_z`.
    #[getter]
    fn variant(&self) -> &'static str {
        coord_tag(self.0)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        self.0.hash(&mut h);
        h.finish()
    }

    fn __repr__(&self) -> String {
        let word = axis_word;
        match self.0 {
            d::Coord::Slide(axis) => format!("Coord.slide({:?})", word(axis)),
            d::Coord::Spin(Some(axis)) => format!("Coord.spin({:?})", word(axis)),
            d::Coord::Spin(None) => "Coord.spin()".to_string(),
        }
    }
}

/// The word of an axis of the target's frame: `"x"`, `"y"` or `"z"`.
pub(crate) fn axis_word(axis: d::TargetAxis) -> &'static str {
    match axis {
        d::TargetAxis::X => "x",
        d::TargetAxis::Y => "y",
        d::TargetAxis::Z => "z",
    }
}

/// An axis of the target's frame by its word: `"x"`, `"y"` or `"z"`.
pub(crate) fn target_axis(axis: &str) -> PyResult<d::TargetAxis> {
    match axis {
        "x" => Ok(d::TargetAxis::X),
        "y" => Ok(d::TargetAxis::Y),
        "z" => Ok(d::TargetAxis::Z),
        other => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "not an axis: {other:?} — one of \"x\", \"y\" or \"z\""
        ))),
    }
}

/// **A value**: one freedom the bundle's mates leave, set to a number
/// — a `Length` for a slide, an `Angle` for a spin (a lone `0` is the
/// constant zero). A value names no mate: the mates of its bundle
/// together leave its freedom.
///
/// Beside a mate fixing a direction it is charted on the target space's
/// own axes carried by the least turn from where the mate's orientation
/// puts the copy's mated direction onto the mated one: a slide is the
/// copy's origin measured along a carried axis, the spin the turn left
/// about the target's own direction. Zero says the copy's own coordinates agree
/// with the target space's, up to the orientation, as far as the mates
/// allow.
///
/// A bundle is written whole (`Doc.place`, `Doc.set_constraints`,
/// `Placed`); the door refuses a freedom the mates do not leave, or
/// one written twice (`mate_overconstrained`), a slide beside a ball
/// and a direction (`mate_no_chart`), and a bundle that leaves a
/// freedom unwritten (`mate_unpinned`).
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct MateValue(pub(crate) d::Value<d::Formula>);

/// The id a bundle written whole holds its entry at `at` under until
/// its write mints one.
fn placeholder(at: u32) -> d::ConstraintId {
    d::ConstraintId::placeholder(at.saturating_add(1))
}

/// **One entry of a bundle as Python writes it**: a mate, or a value.
#[derive(FromPyObject, Clone)]
pub(crate) enum ConstraintArg {
    /// A mate.
    Mate(Mate),
    /// A value.
    Value(MateValue),
}

/// **A bundle as Python writes it**: a sequence of mates and values,
/// or a `Placed`, the bundle its constructor wrote.
#[derive(FromPyObject, Clone)]
pub(crate) enum BundleArg {
    /// A bundle its constructor wrote.
    Placed(Placed),
    /// Mates and values, in order.
    Entries(Vec<ConstraintArg>),
}

/// **A bundle as Python writes it**, each entry under the id of its
/// position (`ConstraintId::placeholder`), which the edit writing it
/// mints.
pub(crate) fn bundle(entries: BundleArg) -> Vec<d::Constraint<d::Formula>> {
    match entries {
        BundleArg::Placed(placed) => placed.0.bundle(),
        BundleArg::Entries(entries) => (0..)
            .zip(entries)
            .map(|(at, entry)| match entry {
                ConstraintArg::Mate(mate) => d::Constraint::of_mate(placeholder(at), mate.0),
                ConstraintArg::Value(value) => d::Constraint::of_value(placeholder(at), value.0),
            })
            .collect(),
    }
}

impl MateValue {
    /// The value `by` on `coord`, `by` read at the freedom's dimension.
    pub(crate) fn of(py: Python<'_>, coord: d::Coord, by: &super::expr::SlotArg) -> PyResult<Self> {
        let by = super::pose::scalar_at(py, coord.dimension(), by, coord.name())?;
        Ok(Self(d::Value { coord, by }))
    }
}

#[pymethods]
impl MateValue {
    #[new]
    fn new(py: Python<'_>, coord: &Coord, by: super::expr::SlotArg) -> PyResult<Self> {
        Self::of(py, coord.0, &by)
    }

    /// Which freedom.
    #[getter]
    fn coord(&self) -> Coord {
        Coord(self.0.coord)
    }

    /// The number, as written.
    #[getter]
    fn by(&self) -> super::expr::Formula {
        super::expr::Formula(self.0.by.clone())
    }

    /// Bit-exact equality.
    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __repr__(&self) -> String {
        format!(
            "MateValue({}, {})",
            Coord(self.0.coord).__repr__(),
            self.0.by
        )
    }
}

/// **A placement's bundle, as its author wrote it** — the one
/// constructor of a bundle: its mates, then the values written, then
/// the constant `0` on each freedom the mates leave that no value
/// names (a slide along each axis the mates leave and the spin about
/// the target's own direction, or, where no direction is fixed, the chain's
/// missing turns about `x`, `y` and `z` until it turns three times).
/// It reads no geometry and writes no number its author did not, so a
/// quarter turn written `Formula.turn_over(4)` is one.
///
/// `Placed.identity()` is the identity bundle, no mate and the
/// constant `0` on every freedom, which an instance and a `Doc.place`
/// with no constraints are written; `Placed.unmated(values)` a bundle
/// with no mate; `Placed.of(doc, mates, values)` one with mates, the
/// freedoms they leave read off their kinds in `doc`'s recipe, nothing
/// evaluated. A `Placed` is passed where a bundle is (`Doc.place`,
/// `Doc.set_constraints`), or read as its entries (`bundle()`).
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct Placed(pub(crate) d::Placed);

#[pymethods]
impl Placed {
    /// The identity bundle.
    #[staticmethod]
    fn identity() -> Self {
        Self(d::Placed::identity())
    }

    /// A bundle with no mate: `values`, then the constant `0` on each
    /// freedom of the target space's axes they leave unnamed.
    #[staticmethod]
    fn unmated(values: Vec<MateValue>) -> Self {
        Self(d::Placed::unmated(
            values.into_iter().map(|v| (v.0.coord, v.0.by)).collect(),
        ))
    }

    /// `mates`, then `values`, then the constant `0` on each freedom
    /// the mates leave that no value names, read off the mates' kinds
    /// in `doc`'s recipe.
    #[staticmethod]
    fn of(doc: &super::doc::Doc, mates: Vec<Mate>, values: Vec<MateValue>) -> Self {
        Self(d::Placed::of(
            &doc.inner,
            mates.into_iter().map(|m| m.0).collect(),
            values.into_iter().map(|v| (v.0.coord, v.0.by)).collect(),
        ))
    }

    /// The mates, in order.
    #[getter]
    fn mates(&self) -> Vec<Mate> {
        self.0.mates.iter().cloned().map(Mate).collect()
    }

    /// The values, the author's first in the order written, then the
    /// constant `0` on each freedom left unnamed.
    #[getter]
    fn values(&self) -> Vec<MateValue> {
        self.0
            .values
            .iter()
            .map(|(coord, by)| {
                MateValue(d::Value {
                    coord: *coord,
                    by: by.clone(),
                })
            })
            .collect()
    }

    /// The bundle as its entries: each mate, then each value.
    fn bundle(&self, py: Python<'_>) -> PyResult<Vec<Py<PyAny>>> {
        let mates = self
            .0
            .mates
            .iter()
            .map(|m| Ok(Py::new(py, Mate(m.clone()))?.into_any()));
        let values = self
            .values()
            .into_iter()
            .map(|v| Ok(Py::new(py, v)?.into_any()));
        mates.chain(values).collect()
    }

    /// Bit-exact equality: the same mates and values, in order.
    fn __eq__(&self, other: &Self) -> bool {
        self.0.mates.len() == other.0.mates.len()
            && self
                .0
                .mates
                .iter()
                .zip(&other.0.mates)
                .all(|(a, b)| a.bit_eq(b))
            && self.0.values == other.0.values
    }

    fn __repr__(&self) -> String {
        format!(
            "Placed({} mate(s), {} value(s))",
            self.0.mates.len(),
            self.0.values.len()
        )
    }
}

// ---- The admission table ----

// ---- The solve's read side ----

/// **A constraint's id**: minted by the edit that adds it to a
/// placement, never reused and never positional. Opaque: it is read
/// off a `MateRef` and handed back, compared and hashed, never built.
#[pyclass(eq, ord, hash, frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct ConstraintId(pub(crate) d::ConstraintId);

#[pymethods]
impl ConstraintId {
    fn __repr__(&self) -> String {
        format!("ConstraintId({})", self.0)
    }
}

/// **A constraint's address**: the placement that holds it and the id
/// minted with it. What every fault about one mate names, what
/// `Doc.constrain` answers, and what `DocEdit.remove_constraint`
/// takes. Hashable, and ordered by placement, then id.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct MateRef(pub(crate) d::MateRef);

#[pymethods]
impl MateRef {
    /// The constraint `constraint` of `placement`.
    #[new]
    fn new(placement: &NodeId, constraint: &ConstraintId) -> Self {
        Self(d::MateRef::new(placement.0, constraint.0))
    }

    /// The placement holding the constraint.
    #[getter]
    fn placement(&self) -> NodeId {
        NodeId(self.0.placement)
    }

    /// The constraint's id.
    #[getter]
    fn constraint(&self) -> ConstraintId {
        ConstraintId(self.0.id)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __lt__(&self, other: &Self) -> bool {
        self.0 < other.0
    }

    fn __le__(&self, other: &Self) -> bool {
        self.0 <= other.0
    }

    fn __gt__(&self, other: &Self) -> bool {
        self.0 > other.0
    }

    fn __ge__(&self, other: &Self) -> bool {
        self.0 >= other.0
    }

    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        self.0.hash(&mut h);
        h.finish()
    }

    fn __repr__(&self) -> String {
        format!(
            "MateRef(NodeId({}), {})",
            self.0.placement.full(),
            self.0.id
        )
    }
}

/// Why the solve refused for one node — a VALUE, because the solve is
/// total and records a refusal per node rather than failing the
/// document.
///
/// Every payload attribute is present on every arm, `None` where the
/// arm does not carry it: `mate`, `side`, `error`, `instance`,
/// `shape`, `placement`, `constraints`, `free`, `what`,
/// `expected_document`, `found_document`, `inner_variant`, `on_kind`,
/// `to_kind`, `value_ref`, `why`, `cause`. The human
/// message is the kernel's own prose, available as `str(fault)`; the
/// refusal it carries and does not quote is `cause`.
///
/// All of them read off ONE record, [`crate::mate_payload`], whose
/// match over the kernel enum is exhaustive with no wildcard: a fault
/// arm added there is a compile error rather than a mate that every
/// accessor here silently answers `None` about.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct MateFault(pub(crate) d::MateFault, pub(crate) Voice);

/// **Who says the nodes a fault's words name**: the document the fault
/// was raised over, as the solve that recorded it read it, or the nodes
/// an edit door kept when it refused the mate. A fault is memoized with
/// the solve, so it holds ids; its node getters cross them in full.
#[derive(Clone)]
pub(crate) enum Voice {
    /// The document the solve read.
    Doc(Arc<d::ProfileDoc>),
    /// The nodes the edit door kept ([`d::HeldNodes`]).
    Held(d::HeldNodes),
}

impl Voice {
    /// The speaker this voice says nodes by.
    fn speaker(&self) -> d::Speaker<'_> {
        match self {
            Self::Doc(doc) => d::Speaker::of(&**doc),
            Self::Held(held) => d::Speaker::held(held),
        }
    }

    /// The document a carried refusal's level in it is spoken from,
    /// when this voice holds one.
    fn doc(&self) -> Option<&d::ProfileDoc> {
        match self {
            Self::Doc(doc) => Some(doc),
            Self::Held(_) => None,
        }
    }
}

#[pymethods]
impl MateFault {
    /// The stable tag of the refusing arm.
    #[getter]
    fn variant(&self) -> &'static str {
        mate_fault_tag(&self.0)
    }

    /// The mate the fault is about, `None` for an arm whose subject is
    /// not one mate: a bundle's faults, whose subject is the
    /// `placement`; a value's, whose subject is the `value_ref`; and
    /// `mate_poses_of_another_document`, whose subject is two
    /// documents.
    #[getter]
    fn mate(&self) -> Option<MateRef> {
        self.payload().mate.map(MateRef)
    }

    /// Which side of the mate refused.
    #[getter]
    fn side(&self) -> Option<MateSide> {
        self.payload().side.map(MateSide::from_kernel)
    }

    /// **The evaluation's own refusal for a pose that did not
    /// evaluate** (`mate_side_unevaluated`), as the tag every node
    /// failure crosses with (`EvaluationError.kind`) — the same
    /// vocabulary, so a caller branches on one set of words whether
    /// the refusal reached them from the node or from the mate that
    /// read it. [`MateFault::cause`] carries its prose.
    #[getter]
    fn error(&self) -> Option<&'static str> {
        self.payload().error
    }

    /// **The refusal this fault carries, typed** — the pose's own on
    /// `mate_side_unevaluated`: the `EvaluationError` its evaluation
    /// raises, which `str(fault)` points at. The same value a raised
    /// `MateError` carries as its `__cause__`. `None` on every other
    /// arm.
    #[getter]
    fn cause(&self, py: Python<'_>) -> Option<Py<PyAny>> {
        super::value::carried_cause(py, self.0.carried_chain(), self.1.doc())
            .map(|cause| cause.into_value(py).into_any())
    }

    /// The instance whose part a lever's reach was asked of
    /// (`mate_unleverable`).
    #[getter]
    fn instance(&self) -> Option<NodeId> {
        self.payload().instance.map(NodeId)
    }

    /// The node whose copied body a lever's reach was asked of
    /// (`mate_unleverable`).
    #[getter]
    fn shape(&self) -> Option<NodeId> {
        self.payload().shape.map(NodeId)
    }

    /// The placement a bundle's fault is about: the one a constraint
    /// over-constrains (`mate_overconstrained`), or whose bundle leaves
    /// a freedom unwritten (`mate_unpinned`).
    #[getter]
    fn placement(&self) -> Option<NodeId> {
        self.payload().placement.map(NodeId)
    }

    /// The constraints of `placement` the fault names, by address: on
    /// `mate_overconstrained` each whose removal leaves the bundle's
    /// codimension unchanged.
    #[getter]
    fn constraints(&self) -> Option<Vec<MateRef>> {
        let payload = self.payload();
        let placement = payload.placement?;
        payload.constraints.map(|ids| {
            ids.iter()
                .map(|&id| MateRef(d::MateRef::new(placement, id)))
                .collect()
        })
    }

    /// How many of the copy's six freedoms an over-constrained bundle
    /// leaves (`mate_overconstrained`): `0` where the rest already pins
    /// it.
    #[getter]
    fn free(&self) -> Option<u8> {
        self.payload().free
    }

    /// The kind of pose the mate's `on` reads, where the two kinds
    /// differ (`mate_kinds_differ`).
    #[getter]
    fn on_kind(&self) -> Option<&'static str> {
        self.payload().on_kind.map(crate::errors::var_kind_tag)
    }

    /// The kind of pose its `to` reads (`mate_kinds_differ`).
    #[getter]
    fn to_kind(&self) -> Option<&'static str> {
        self.payload().to_kind.map(crate::errors::var_kind_tag)
    }

    /// **The value a value's fault is about**, by its address
    /// (`mate_no_chart`).
    #[getter]
    fn value_ref(&self) -> Option<MateRef> {
        self.payload().value_ref.map(MateRef)
    }

    /// Why a value's freedom has no chart (`mate_no_chart`):
    /// `"slide_turns_a_ball"` or `"slides_do_not_span"`.
    #[getter]
    fn why(&self) -> Option<&'static str> {
        self.payload().why.map(no_chart_why_tag)
    }

    /// The slides to write, by their axes' words, where the slides
    /// written do not span what the mate leaves (`slides_do_not_span`):
    /// `["y", "z"]` beside a plane mate fixing `x`.
    #[getter]
    fn slides(&self) -> Option<Vec<&'static str>> {
        match self.payload().why? {
            d::NoChartWhy::SlidesDoNotSpan { write, .. } => {
                Some(write.axes().map(axis_word).collect())
            }
            d::NoChartWhy::SlideTurnsABall => None,
        }
    }

    /// The orientation a mate was written with, where the mated direction
    /// lies opposite to where it puts the copy's (`mate_backwards`).
    #[getter]
    fn orient(&self) -> Option<Orientation> {
        self.payload().orient.map(Orientation)
    }

    /// The orientation to write in its place (`mate_backwards`).
    #[getter]
    fn write(&self) -> Option<Orientation> {
        self.payload().write.map(Orientation)
    }

    /// What a lever's unbounded face is, in its own words.
    #[getter]
    fn what(&self) -> Option<&'static str> {
        self.payload().what
    }

    /// The document whose placement was asked for, as `Doc.id`
    /// answers it — so a caller compares the two ids directly rather
    /// than reading them out of the message.
    #[getter]
    fn expected_document(&self) -> Option<String> {
        self.payload().expected_document.map(|id| id.hex())
    }

    /// The document the solve is OF.
    #[getter]
    fn found_document(&self) -> Option<String> {
        self.payload().found_document.map(|id| id.hex())
    }

    /// **The nested refusal's own word**: the lever refusal's. `None`
    /// on an arm with no nested refusal.
    #[getter]
    fn inner_variant(&self) -> Option<&'static str> {
        self.payload().inner_variant
    }

    /// The fault's words, each node it names spoken from the document
    /// it was raised over.
    fn __str__(&self) -> String {
        d::Said(&self.0, self.1.speaker()).to_string()
    }

    fn __repr__(&self) -> String {
        format!("MateFault({:?})", mate_fault_tag(&self.0))
    }
}

impl MateFault {
    /// This refusal's payload, read once per attribute.
    ///
    /// Every accessor above reads a field off THIS record rather than
    /// matching the enum itself, so the arm table is written once —
    /// exhaustively, with no wildcard, in `crate::mate_payload` — and
    /// an arm added kernel-side is a compile error there instead of
    /// seventeen attributes silently answering `None`.
    fn payload(&self) -> crate::mate_payload::MateFaultPayload<'_> {
        crate::mate_payload::mate_payload(&self.0)
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
pub(crate) fn mate_err(py: Python<'_>, fault: &d::MateFault, voice: &Voice) -> PyErr {
    let value = Py::new(py, MateFault(fault.clone(), voice.clone()))
        .map(|v| v.into_any())
        .unwrap_or_else(|_| py.None());
    let err = typed_err(
        py,
        ErrorClass::Mate,
        d::Said(fault, voice.speaker()).to_string(),
        &[
            (
                "variant",
                PyString::new(py, mate_fault_tag(fault)).unbind().into_any(),
            ),
            ("fault", value),
        ],
    );
    // The refusal the fault carries, typed, as the cause — the value's
    // own `cause`, and what a node failure does with one.
    super::value::with_carried(py, err, fault.carried_chain(), voice.doc())
}

/// [`d::SolvedPoses::placement`]'s refusal, raised: the solve's own
/// fault as `MateError`, spoken from `solved`, the document the solve
/// recorded it over; a placement that did not evaluate for a reason
/// not its bundle's as its own standing in `evaluation`; and a copy
/// placed into a space other than the world as the `EvaluationError` a
/// node reading that space beside the world's carries
/// (`space_mismatch`), spoken from `doc`, the document the placement
/// door read it off.
fn pose_err(
    py: Python<'_>,
    placement: NodeId,
    refusal: &d::PoseRefusal,
    solved: &Arc<d::ProfileDoc>,
    evaluation: &d::Evaluation<f64>,
    doc: &d::ProfileDoc,
) -> PyErr {
    let kind = match refusal {
        d::PoseRefusal::Mate(fault) => {
            return mate_err(py, fault, &Voice::Doc(Arc::clone(solved)));
        }
        d::PoseRefusal::Unevaluated { .. } => {
            return super::value::standing_err(py, solved, evaluation, placement);
        }
        d::PoseRefusal::OtherSpace { space, .. } => d::NodeErrorKind::SpaceMismatch {
            a: *space,
            b: d::VarId::WORLD,
        },
    };
    let said = d::Said(refusal, d::Speaker::of(doc)).to_string();
    let err = super::value::refused(py, placement, &kind, said, None);
    super::value::with_carried(py, err, kind.carried_chain(), Some(doc))
}

/// The document's solved poses: each placement's pose in its space,
/// and the per-placement refusals.
///
/// It keeps a copy of the document as it solved it: a fault is the
/// solve's, recorded over that version, so its words are spoken from it
/// even after the document is edited.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct SolvedPoses(d::SolvedPoses, Arc<d::ProfileDoc>, Arc<d::Evaluation<f64>>);

#[pymethods]
impl SolvedPoses {
    /// The placement's recorded fault, `None` if the solve did not
    /// refuse for it.
    ///
    /// A refusal is recorded against the placement whose bundle it is
    /// about, and against every placement pinned to it that
    /// consequently has no pose — a refusal reaches the copies it
    /// actually affects and no further.
    fn fault(&self, placement: &NodeId) -> Option<MateFault> {
        self.0
            .fault(placement.0)
            .cloned()
            .map(|fault| MateFault(fault, Voice::Doc(Arc::clone(&self.1))))
    }

    /// **The placement's solved pose in its target space**: where its
    /// copy sits in that space's coordinates — the world's for a copy
    /// placed into the world. `None` when the solve placed nothing there: a node
    /// that is no placement, or a placement whose bundle refused.
    fn pose(&self, placement: &NodeId) -> Option<Frame> {
        self.0.relative(placement.0).map(Frame)
    }

    /// **The placement's world pose, at the document's own
    /// parameters**: its pose in its target space, for a copy placed
    /// into the world.
    ///
    /// A copy pinned to the world at a pose returns that pose, to the
    /// rounding of the values its pin is computed in.
    ///
    /// `doc` must be the document this solve is OF: passing a different
    /// one would compose this document's poses onto that one's
    /// placements, which is not a pose of either. The door refuses that
    /// first — a `SolvedPoses` carries the id of the document
    /// `solve_document` solved, and a mismatch raises `MateError` with
    /// tag `mate_poses_of_another_document` before any frame is read.
    ///
    /// Raises `MateError` when the placement's bundle did not solve,
    /// and `EvaluationError` — kind `space_mismatch` — when the copy is
    /// placed into a space other than the world, and so has no world
    /// pose.
    fn placement(
        &self,
        py: Python<'_>,
        doc: &super::doc::Doc,
        placement: &NodeId,
    ) -> PyResult<Frame> {
        self.0
            .placement(&doc.inner, placement.0)
            .map(Frame)
            .map_err(|refusal| pose_err(py, *placement, &refusal, &self.1, &self.2, &doc.inner))
    }

    fn __repr__(&self) -> String {
        "SolvedPoses()".to_string()
    }
}

/// Solve the document's bundles: each placement's constraints folded
/// into the coset they pin its copy to, yielding every placement's pose
/// in its target's space.
///
/// **Total — this never raises.** A refusing bundle must not fail an
/// unrelated one, so refusals are recorded per placement and read back
/// through `SolvedPoses.fault`.
///
/// A bundle folds where its placement evaluates: each mate's poses are
/// read off the evaluated geometry, and a parallelism verdict is
/// levered over the mated bodies' own extent. So `resolver` is the
/// document seam `evaluate(doc, resolver=)` crosses: a `Workspace`, or
/// `None`, under which a placement over a part's instance refuses in
/// the resolver's own voice (`part_no_resolver`) rather than reading
/// nothing.
#[pyfunction]
#[pyo3(signature = (doc, *, resolver=None))]
pub(crate) fn solve_document(
    doc: &super::doc::Doc,
    resolver: Option<&super::store::Workspace>,
) -> SolvedPoses {
    let tol = Tol::witness();
    let opts = d::EvalOptions {
        resolver: super::doc::seam(resolver),
        ..d::EvalOptions::default()
    };
    let evaluation: d::Evaluation<f64> =
        d::evaluate(&doc.inner, None, &d::CancelToken::new(), &opts, tol);
    SolvedPoses(
        d::SolvedPoses::of(&doc.inner, &evaluation),
        Arc::new(doc.inner.clone()),
        Arc::new(evaluation),
    )
}

/// The **relative-freedom partition**: the recipe's components over its
/// reads, each in document order. A constraint couples its copy to its
/// target because the placement holding it reads the target, and two
/// copies pinned against the world share the world.
#[pyfunction]
pub(crate) fn relative_freedom_components(doc: &super::doc::Doc) -> Vec<Vec<NodeId>> {
    d::relative_freedom_components(&doc.inner)
        .into_iter()
        .map(|c| c.into_iter().map(NodeId).collect())
        .collect()
}

// ---- The accepted edit's maintenance ----

/// One act of automatic maintenance an accepted edit performed: a
/// reference its delete stranded, or an anonymous variable it removed.
///
/// It rides the accepted edit rather than being an edit of its own —
/// automatic maintenance is deterministic from the edit, so a replay
/// reproduces it and undo (keeping the prior document) restores it
/// exactly. What the record adds is VISIBILITY: a stranded name is
/// said at the delete rather than at the next evaluation.
///
/// Payload attributes are present on every arm, `None` where
/// inapplicable: `node` and `name` for a strand, `node` (the selection's first reader) and `name` for a
/// `stranded_selection`, and `name` alone for a `stranded_appearance`,
/// whose carrier is the appearance store and not a node.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct Maintenance(pub(crate) d::Maintenance);

#[pymethods]
impl Maintenance {
    /// The stable tag: one of the words the stub lists for this
    /// attribute. The word decides which of the payload
    /// attributes below carry.
    // The map is `crate::tags::maintenance_tag`, whose words
    // `TAG_INVENTORY` pins.
    #[getter]
    fn variant(&self) -> &'static str {
        maintenance_tag(&self.0)
    }

    /// The node this row is about: the surviving node whose payload carries a
    /// stranded name, the first node that reads a selection naming
    /// one (a `stranded_selection`), or the node whose operand reads
    /// what a delete removed (a `stranded_read`). `None` for a
    /// `stranded_appearance`, which has no carrying node to name.
    ///
    /// The arms answer different questions with one attribute on
    /// purpose: each is the node a reader would go and look at, which
    /// is the whole use of the getter. `variant` says which question
    /// was answered.
    #[getter]
    fn node(&self) -> Option<NodeId> {
        match &self.0 {
            d::Maintenance::Strand { node, .. }
            | d::Maintenance::StrandedRead { node, .. }
            | d::Maintenance::StrandedSpace { node, .. }
            | d::Maintenance::ValueDropped {
                placement: node, ..
            } => Some(NodeId(node.id())),
            d::Maintenance::StrandedSelection { readers, .. } => {
                readers.first().map(|reader| NodeId(reader.id()))
            }
            d::Maintenance::StrandedAppearance { .. }
            | d::Maintenance::ProductOutsideWorld { .. }
            | d::Maintenance::AnonymousVarRemoved { .. } => None,
        }
    }

    /// The name this row is about, in the opaque text every name door
    /// on this surface speaks — the stranded payload name for a
    /// `strand`, the selected name for a `stranded_selection`, the
    /// appearance store's stranded key for a `stranded_appearance`. A
    /// stranded name is spelled as the document holds it — its minting
    /// node deleted, or its profile piece no longer drawn — and
    /// `DocEdit.rebind` from that spelling is the repair this surface
    /// carries (with `body=` the selection's body for a
    /// `stranded_selection`).
    #[getter]
    fn name(&self, py: Python<'_>) -> PyResult<Option<String>> {
        match &self.0 {
            d::Maintenance::Strand { name, .. }
            | d::Maintenance::StrandedSelection { name, .. }
            | d::Maintenance::StrandedAppearance { name, .. } => {
                super::doc::name_text(py, name.name()).map(Some)
            }
            d::Maintenance::StrandedRead { .. }
            | d::Maintenance::StrandedSpace { .. }
            | d::Maintenance::ProductOutsideWorld { .. }
            | d::Maintenance::ValueDropped { .. }
            | d::Maintenance::AnonymousVarRemoved { .. } => Ok(None),
        }
    }

    fn __repr__(&self) -> String {
        format!("Maintenance({:?})", self.variant())
    }
}

/// Register the mate surface on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MateSide>()?;
    m.add_class::<Coord>()?;
    m.add_class::<MateValue>()?;
    m.add_class::<Orientation>()?;
    m.add_class::<Placed>()?;
    m.add_class::<ConstraintId>()?;
    m.add_class::<MateRef>()?;
    m.add_class::<Mate>()?;
    m.add_class::<MateFault>()?;
    m.add_class::<SolvedPoses>()?;
    m.add_class::<Maintenance>()?;
    m.add_function(wrap_pyfunction!(solve_document, m)?)?;
    m.add_function(wrap_pyfunction!(relative_freedom_components, m)?)?;
    m.add("OVERCONSTRAINED_RECOURSE", d::OVERCONSTRAINED_RECOURSE)?;
    m.add("UNPINNED_RECOURSE", d::UNPINNED_RECOURSE)?;
    Ok(())
}
