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
//! [`solve_document`] never raises. A refusing group must not fail
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
use crate::tags::{
    class_admission_tag, maintenance_tag, mate_fault_tag, mate_primitive_tag, subgroup_tag,
    unplaced_tag,
};
use std::sync::Arc;

use pncad::document as d;
use pncad::tolerance::Tol;

use super::doc::NodeId;
use super::place::Frame;
use super::quantity::{Angle, Length};

/// Metres in, as the `Frame` family spells a position.
fn meters(v: (Length, Length, Length)) -> [f64; 3] {
    [v.0.0.meters(), v.1.0.meters(), v.2.0.meters()]
}

/// One bare metre figure as the length it is.
fn length(m: f64) -> Length {
    Length(pncad::quantity::Length::from_meters(m))
}

/// Metres out.
fn lengths(v: [f64; 3]) -> (Length, Length, Length) {
    (length(v[0]), length(v[1]), length(v[2]))
}

/// A kernel point as three lengths.
fn point(p: pncad::geom_core::Point3<f64>) -> (Length, Length, Length) {
    lengths(p.to_array())
}

/// A kernel vector as three plain numbers — a DIRECTION carries no
/// dimension, the `PatternKind.linear` convention.
fn direction(v: pncad::geom_core::Vec3<f64>) -> (f64, f64, f64) {
    (v.x, v.y, v.z)
}

// ---- The authored payload ----

/// One side's **mate frame**, in that instance's own part
/// coordinates — two arms, closed.
///
/// **Authored**: three vectors the author writes — an origin, the
/// primary axis (a planar rest's normal, a coaxial mate's axis), and
/// the clocking reference that fixes roll. `axis` need not be unit
/// and `reference` need not be perpendicular to it: only the axis's
/// direction and the reference's perpendicular part are read. Both
/// are plain numbers — a direction has no dimension — while `origin`
/// is three lengths.
///
/// **From a face** (`MateFrame.from_face()`): no name and no number.
/// The side's frame is its own HEAD's face — the face the mate's
/// reference on that side names, read in the mated part — whose
/// canonical pose the solve reads off the part's own evaluation at
/// every evaluation (`ASSEMBLY.md` A11 rule 5): the carrier's origin,
/// its CHART axis, and its own in-frame reference direction as the
/// roll — so a face frame's roll is the carrier's, and a side that
/// needs a roll of its own takes authored vectors.
/// The face's orientation sense is not folded into the axis; the
/// mate's `AxisSense` says which way the sides point. Nothing is
/// stored twice: edit the part so the face moves, and the mate
/// follows. A face with no canonical frame (a NURBS carrier) refuses
/// at the solve, typed, and keeps taking authored vectors.
///
/// A face frame resolves at the NOMINAL value only: under an analysis
/// lane — `stackup.sensitivities`' dual passes, a certified
/// `clearance`'s interval leaf — its side refuses `unpinned`, so those
/// doors refuse an assembly holding one, where the same mate authored
/// as vectors still solves.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct MateFrame(pub(crate) d::MateFrame);

#[pymethods]
impl MateFrame {
    #[new]
    fn new(
        origin: (Length, Length, Length),
        axis: (f64, f64, f64),
        reference: (f64, f64, f64),
    ) -> Self {
        Self(d::MateFrame::authored(
            meters(origin),
            [axis.0, axis.1, axis.2],
            [reference.0, reference.1, reference.2],
        ))
    }

    /// A frame resolved from the side's own head face (the class docs
    /// say how). It takes nothing: the head is the face, and its roll
    /// is the carrier's own.
    #[staticmethod]
    fn from_face() -> Self {
        Self(d::MateFrame::FromFace)
    }

    /// Which arm: `"authored"` or `"from_face"`.
    #[getter]
    fn variant(&self) -> &'static str {
        match &self.0 {
            d::MateFrame::Authored(_) => "authored",
            d::MateFrame::FromFace => "from_face",
        }
    }

    /// The frame's origin, in the part's own coordinates; `None` on a
    /// `from_face` frame, whose origin is the face's and is read at
    /// the solve.
    #[getter]
    fn origin(&self) -> Option<(Length, Length, Length)> {
        self.0.authored_vectors().map(|f| lengths(f.origin))
    }

    /// The primary axis, as authored (not normalised); `None` on a
    /// `from_face` frame.
    #[getter]
    fn axis(&self) -> Option<(f64, f64, f64)> {
        self.0
            .authored_vectors()
            .map(|f| (f.axis[0], f.axis[1], f.axis[2]))
    }

    /// The clocking reference, as authored; `None` on a `from_face`
    /// frame, whose roll is the carrier's own.
    #[getter]
    fn reference(&self) -> Option<(f64, f64, f64)> {
        self.0
            .authored_vectors()
            .map(|f| (f.reference[0], f.reference[1], f.reference[2]))
    }

    /// The rigid placement an AUTHORED frame denotes: local +Z is
    /// `axis`, the local origin is `origin`, roll fixed by
    /// `reference`.
    ///
    /// Raises `FrameError` when the axis has no definite direction,
    /// when the reference has no definite perpendicular offset from
    /// it, or — asked before either sign — when the axis's length or
    /// that perpendicular offset is not a finite number, whose
    /// `variant` reads `non_finite_aim` or `non_finite_roll_reference`.
    /// The same refusal the solve meets, reachable BEFORE authoring
    /// the mate that would carry it. Raises `TypeError` on a
    /// `from_face` frame, which denotes no placement until the solve
    /// resolves it against the part: ask the solved document.
    fn placement(&self, py: Python<'_>) -> PyResult<Frame> {
        let Some(authored) = self.0.authored_vectors() else {
            return Err(pyo3::exceptions::PyTypeError::new_err(
                "a from_face frame denotes no placement of its own: the solve resolves it \
                 against the part's face at evaluation, so ask the solved document",
            ));
        };
        let tol = Tol::witness();
        authored
            .placement(tol)
            .map(|affine| Frame(d::Frame::from_affine(affine)))
            .map_err(|err| super::place::frame_err(py, &err))
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __repr__(&self) -> String {
        match &self.0 {
            d::MateFrame::Authored(f) => format!(
                "MateFrame(origin={:?}, axis={:?}, reference={:?})",
                f.origin, f.axis, f.reference
            ),
            d::MateFrame::FromFace => "MateFrame.from_face()".to_owned(),
        }
    }
}

/// Which way the two sides' axes point at each other.
///
/// `Opposed` is what kills every π-flip ambiguity: the senses are
/// AUTHORED, never inferred.
#[pyclass(eq, eq_int, frozen, hash, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
#[pyclass(eq, eq_int, frozen, hash, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
    /// `planar_rest` or `clocking` — one word per constructor on this
    /// class, spelled the same way, so the vocabulary is the class's
    /// own staticmethods.
    // The map is `crate::tags::mate_primitive_tag`, whose words
    // `TAG_INVENTORY` pins.
    #[getter]
    fn variant(&self) -> &'static str {
        mate_primitive_tag(self.0)
    }

    /// The planar rest's signed standoff, `None` for every other
    /// primitive — the attribute is present on all four (the payload
    /// posture the typed refusals use), so reading it never needs a
    /// branch on `variant` first.
    #[getter]
    fn offset(&self) -> Option<Length> {
        use d::MatePrimitive as P;
        match self.0 {
            P::PlanarRest { offset } => Some(Length(pncad::quantity::Length::from_meters(offset))),
            P::FrameCoincidence | P::Coaxial | P::Clocking => None,
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
            other => format!("MatePrimitive.{}()", mate_primitive_tag(other)),
        }
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
#[derive(Clone)]
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
            a: a.0.clone(),
            b: b.0.clone(),
            primitive: primitive.0,
            sense: sense.to_kernel(),
            clocking: clocking.map(|angle| angle.0.radians()),
        })
    }

    /// The `a` side's mate frame, in `a`'s part coordinates.
    #[getter]
    fn a(&self) -> MateFrame {
        MateFrame(self.0.a.clone())
    }

    /// The `b` side's mate frame, in `b`'s part coordinates.
    #[getter]
    fn b(&self) -> MateFrame {
        MateFrame(self.0.b.clone())
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

    /// The **datum's own contribution to the lever** this mate's
    /// angular decisions turn on: both mate frames' distances from
    /// their parts' origins plus every length the primitive authors,
    /// summed. The lever itself adds the two mated parts' own extent
    /// (an upper bound from each evaluated body), which only the solve
    /// has in hand — so this is the part an alignment can answer
    /// alone, never the whole, and it is zero for a datum authored at
    /// both origins with no length, which is the ordinary spelling of
    /// an axis-to-axis mate rather than a defect. `None` when a side
    /// is a `from_face` frame: its origin is the face's, resolved at
    /// the solve, so the term is formed there.
    #[getter]
    fn lever_arm(&self) -> Option<Length> {
        let a = self.0.a.authored_vectors()?;
        let b = self.0.b.authored_vectors()?;
        Some(Length(pncad::quantity::Length::from_meters(
            self.0.lever_arm(a, b),
        )))
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __repr__(&self) -> String {
        format!(
            "Alignment(primitive={}, sense={:?}, clocking={:?})",
            mate_primitive_tag(self.0.primitive),
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
    /// The stable tag: `mints`, `no_at_rest_record` or
    /// `not_admitted`, the three the stub lists for this attribute.
    // The map is `crate::tags::class_admission_tag`, whose words
    // `TAG_INVENTORY` pins.
    #[getter]
    fn variant(&self) -> &'static str {
        class_admission_tag(&self.0)
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
#[pyclass(eq, eq_int, frozen, hash, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
    /// `revolute`, `trivial` or `empty`, the seven the stub lists for
    /// this attribute. `empty` is the contradictory answer and
    /// `trivial` the fully located one.
    // The map is `crate::tags::subgroup_tag`, whose words
    // `TAG_INVENTORY` pins.
    #[getter]
    fn variant(&self) -> &'static str {
        subgroup_tag(&self.0)
    }

    /// The plane's unit normal, for `planar`.
    #[getter]
    fn normal(&self) -> Option<(f64, f64, f64)> {
        use d::Subgroup as S;
        match self.0 {
            S::Planar { normal } => Some(direction(normal.get())),
            S::Se3
            | S::Cylindrical { .. }
            | S::Prismatic { .. }
            | S::Revolute { .. }
            | S::Trivial
            | S::Empty => None,
        }
    }

    /// A point on the axis, for `cylindrical` and `revolute`.
    #[getter]
    fn point(&self) -> Option<(Length, Length, Length)> {
        use d::Subgroup as S;
        match self.0 {
            S::Cylindrical { point: p, .. } | S::Revolute { point: p, .. } => Some(point(p)),
            // `planar` and `prismatic` are point-free on purpose, as
            // the class doc says; the three remaining arms have no
            // geometry to be based at.
            S::Planar { .. } | S::Prismatic { .. } | S::Se3 | S::Trivial | S::Empty => None,
        }
    }

    /// The unit direction, for `cylindrical`, `prismatic`, `revolute`.
    #[getter]
    fn direction(&self) -> Option<(f64, f64, f64)> {
        use d::Subgroup as S;
        match self.0 {
            S::Cylindrical { direction: v, .. }
            | S::Prismatic { direction: v }
            | S::Revolute { direction: v, .. } => Some(direction(v.get())),
            S::Se3 | S::Planar { .. } | S::Trivial | S::Empty => None,
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
/// arm does not carry it: `mate`, `side`, `head`, `placer`, `error`,
/// `instance`, `parent`, `child`, `residual`, `held`, `added`,
/// `predicate`, `clash`, `part`, `named`, `selected`, `what`,
/// `expected_document`, `found_document`, `inner_variant`, `margin`,
/// `margin_low`, `margin_high`, `zero`, `escalate`, `field`, `value`,
/// `lever_tilt`, `lever_residual`, `lever_arm`, `cause`. The human
/// message is the kernel's own prose, available as `str(fault)`; the
/// refusal it carries and does not quote is `cause`.
///
/// **The classifier's words are the frame door's words.** `margin` /
/// `margin_low` / `margin_high`, `zero` / `escalate`, `field` /
/// `value` and `predicate` are spelled here exactly as
/// [`super::place::frame_err`] spells them, because an escalation a
/// mate reports and one a frame constructor reports are the same
/// value; the fork itself is `crate::escalation`, which both doors
/// call.
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

    /// The mate the fault is about, `None` for the three arms whose
    /// subject is not one mate (`mate_band`; `mate_contradictory`,
    /// which names `held` and `added` instead; and
    /// `mate_poses_of_another_document`, whose subject is two
    /// documents).
    #[getter]
    fn mate(&self) -> Option<NodeId> {
        self.payload().mate.map(NodeId)
    }

    /// Which side of the mate refused.
    #[getter]
    fn side(&self) -> Option<MateSide> {
        self.payload().side.map(MateSide::from_kernel)
    }

    /// The instantiate node a dangling reference head claims.
    #[getter]
    fn head(&self) -> Option<NodeId> {
        self.payload().head.map(NodeId)
    }

    /// The placer whose pose could not be derived — the pattern or
    /// the transform on the reference's chain that refused.
    #[getter]
    fn placer(&self) -> Option<NodeId> {
        self.payload().placer.map(NodeId)
    }

    /// **The evaluation's own refusal for that placer**, as the tag
    /// every node failure crosses with (`EvaluationError.kind`) — the
    /// same vocabulary, so a caller branches on one set of words
    /// whether the refusal reached them from the node or from the
    /// mate that placed it. [`MateFault::cause`] carries its prose.
    #[getter]
    fn error(&self) -> Option<&'static str> {
        self.payload().error
    }

    /// **The refusal this fault carries, typed** — the placer's own,
    /// on `mate_placer_refused` where the placer is poisoned and cannot
    /// state it: the `EvaluationError` the placer's own evaluation
    /// raises, which `str(fault)` points at and never quotes. The same
    /// value a raised `MateError` carries as its `__cause__`. `None` on
    /// every other arm, and where the placer fails in its own right and
    /// its own failure states it.
    #[getter]
    fn cause(&self, py: Python<'_>) -> Option<Py<PyAny>> {
        super::value::carried_cause(py, self.0.carried_chain(), self.1.doc())
            .map(|cause| cause.into_value(py).into_any())
    }

    /// The instance a self-mate names twice, the instance whose part
    /// a lever or a face frame was asked of, or whose checked offset
    /// faulted (`mate_offset_disagrees`, `mate_offset_unchecked`).
    #[getter]
    fn instance(&self) -> Option<NodeId> {
        self.payload().instance.map(NodeId)
    }

    /// The root of a faulted checked offset's group: the member whose
    /// offset places the group the solve placed the instance in.
    #[getter]
    fn root(&self) -> Option<NodeId> {
        self.payload().root.map(NodeId)
    }

    /// The face a `from_face` side read — its head's face, as its name
    /// text in the PART's own spelling, or the head itself where it
    /// names no face of the part — where the refusal is about one
    /// (`mate_face_unresolved`, whose `inner_variant` names why: the
    /// part not in hand, the face's own row, tie or carrier, a product
    /// on an analysis lane, or `no_part_face`).
    #[getter]
    fn face(&self, py: Python<'_>) -> PyResult<Option<String>> {
        self.payload()
            .face
            .map(|face| super::doc::name_text(py, face))
            .transpose()
    }

    /// The instance an under-determined tree mate extended FROM.
    #[getter]
    fn parent(&self) -> Option<NodeId> {
        self.payload().parent.map(NodeId)
    }

    /// The instance it failed to place.
    #[getter]
    fn child(&self) -> Option<NodeId> {
        self.payload().child.map(NodeId)
    }

    /// What survived an under-determined fold.
    #[getter]
    fn residual(&self) -> Option<Subgroup> {
        self.payload().residual.map(Subgroup)
    }

    /// The mate already folded, for a contradictory pair.
    #[getter]
    fn held(&self) -> Option<NodeId> {
        self.payload().held.map(NodeId)
    }

    /// The mate whose intersection died against it.
    #[getter]
    fn added(&self) -> Option<NodeId> {
        self.payload().added.map(NodeId)
    }

    /// The predicate that decided against a contradictory pair.
    #[getter]
    fn predicate(&self) -> Option<&'static str> {
        self.payload().predicate
    }

    /// The measured clash: the margin that should have been zero and
    /// was not — a length verbatim, or a lever's product. `None` for
    /// the structural refusal (`mate_member_empty`), which measures
    /// nothing.
    #[getter]
    fn clash(&self) -> Option<Length> {
        self.payload().clash.map(length)
    }

    /// The `Part` node whose index expression disagrees with the copy
    /// the reference's name names.
    #[getter]
    fn part(&self) -> Option<NodeId> {
        self.payload().part.map(NodeId)
    }

    /// The copy that reference's NAME names — the authority on which
    /// copy a mate is about.
    #[getter]
    fn named(&self) -> Option<u32> {
        self.payload().named
    }

    /// What the `Part`'s index expression evaluates to instead, at the
    /// document's own parameter bindings.
    #[getter]
    fn selected(&self) -> Option<i64> {
        self.payload().selected
    }

    /// What the coset table was asked for, in its own words.
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

    /// **The nested refusal's own word**: the frame ladder's
    /// (`FrameError.variant`'s vocabulary), the band constructor's, or
    /// the lever refusal's. `None` on an arm whose payload is a struct
    /// rather than an enum — an escalation has no inner word, and its
    /// shape is which margin attribute is set.
    #[getter]
    fn inner_variant(&self) -> Option<&'static str> {
        self.payload().inner_variant
    }

    /// The in-band margin the classifier saw, when it saw a value.
    ///
    /// For error text only, not a decision input: what the escalation
    /// contract forbids is recovering the margin to make the sign
    /// decision the classifier refused.
    #[getter]
    fn margin(&self) -> Option<Length> {
        self.payload().margin.map(length)
    }

    /// The classified enclosure's lower bound, where the classifier
    /// saw an enclosure rather than a value. For error text only, not
    /// a decision input.
    #[getter]
    fn margin_low(&self) -> Option<Length> {
        self.payload().margin_low.map(length)
    }

    /// Its upper bound. For error text only, not a decision input.
    #[getter]
    fn margin_high(&self) -> Option<Length> {
        self.payload().margin_high.map(length)
    }

    /// The coincidence threshold of the band a margin was classified
    /// against, or of the band a constructor could not form.
    ///
    /// A plain real, as the frame door answers it: a `Band`'s
    /// thresholds are whatever its predicate measures in, and the
    /// same type carries angular ones.
    #[getter]
    fn zero(&self) -> Option<f64> {
        self.payload().zero
    }

    /// Its escalation threshold.
    #[getter]
    fn escalate(&self) -> Option<f64> {
        self.payload().escalate
    }

    /// WHICH band threshold a rejected value was — `zero` or
    /// `escalate`.
    #[getter]
    fn field(&self) -> Option<&'static str> {
        self.payload().field
    }

    /// The rejected number: a threshold, or a lever arm handed to the
    /// band constructor.
    #[getter]
    fn value(&self) -> Option<f64> {
        self.payload().value
    }

    /// The lever's TILT, when a contradictory clash levered an
    /// authored roll (the clocking rider's `mate_clocking_redundant`).
    /// One of this and `lever_residual` is set on a levered clash,
    /// never both: which one says what kind of number the predicate
    /// measured.
    #[getter]
    fn lever_tilt(&self) -> Option<Angle> {
        self.payload()
            .lever_tilt
            .map(|r| Angle(pncad::quantity::Angle::from_radians(r)))
    }

    /// The lever's RESIDUAL — a pure number, named by `predicate`: a
    /// sine, a cosine, a Frobenius departure from the identity, a
    /// reachability defect — when a contradictory clash levered one
    /// rather than an authored roll. Dimensionless, so a bare float
    /// and not a quantity.
    #[getter]
    fn lever_residual(&self) -> Option<f64> {
        self.payload().lever_residual
    }

    /// The lever's ARM — an upper bound on the two mated parts'
    /// extent together from the datum: each part's reach from its own
    /// origin plus its frame's distance, plus the authored lengths. It
    /// is NOT a contact feature, so it names the parts' scale and
    /// nothing else in the model.
    ///
    /// `clash` is the PRODUCT of the two halves: a levered refusal
    /// reports `lever_tilt * lever_arm` or `lever_residual *
    /// lever_arm` as its deviation. An arm that measured its margin
    /// without a lever carries none of the three.
    #[getter]
    fn lever_arm(&self) -> Option<Length> {
        self.payload().lever_arm.map(length)
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
/// recorded it over; and the two placement refusals as the
/// `EvaluationError` the instance's own row would carry, spoken from
/// `doc`, the document the placement door read them off.
fn pose_err(
    py: Python<'_>,
    instance: NodeId,
    refusal: &d::PoseRefusal,
    solved: &Arc<d::ProfileDoc>,
    doc: &d::ProfileDoc,
) -> PyErr {
    let kind = match refusal {
        d::PoseRefusal::Mate(fault) => {
            return mate_err(py, fault, &Voice::Doc(Arc::clone(solved)));
        }
        d::PoseRefusal::Unplaced { group, cause, .. } => d::NodeErrorKind::Unplaced {
            group: *group,
            cause: *cause,
        },
        d::PoseRefusal::Placement { node, error } => d::NodeErrorKind::PlacementRefused {
            node: *node,
            error: error.clone(),
        },
    };
    let err = super::value::refused(py, instance, &kind, refusal.spoken(doc), None);
    super::value::with_carried(py, err, kind.carried_chain(), Some(doc))
}

/// The document's solved poses: each instance's pose relative to its
/// group root, each mate's role, and the per-node refusals.
///
/// It keeps a copy of the document as it solved it: a fault is the
/// solve's, recorded over that version, so its words are spoken from it
/// even after the document is edited.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct SolvedPoses(d::SolvedPoses, Arc<d::ProfileDoc>);

#[pymethods]
impl SolvedPoses {
    /// The node's recorded fault, `None` if the solve did not refuse
    /// for it.
    ///
    /// Recorded against the refusing MATE and against every instance
    /// in its group that consequently has no pose — a refusal
    /// reaches the nodes it actually affects and no further.
    fn fault(&self, node: &NodeId) -> Option<MateFault> {
        self.0
            .fault(node.0)
            .cloned()
            .map(|fault| MateFault(fault, Voice::Doc(Arc::clone(&self.1))))
    }

    /// A mate's role, `None` if the node is not a live mate.
    fn role(&self, mate: &NodeId) -> Option<MateRole> {
        self.0.role(mate.0).map(MateRole::from_kernel)
    }

    /// An instance's group root, `None` if the node is not a live
    /// instance. A singleton group is its own root.
    fn root(&self, instance: &NodeId) -> Option<NodeId> {
        self.0.root(instance.0).map(NodeId)
    }

    /// An instance's pose in its group's own space: where it sits when
    /// the group's frame is the identity — relative to its root when no
    /// placer stands on the path from the root, and in an unplaced
    /// group the pose the group is evaluated at.
    fn relative(&self, instance: &NodeId) -> Option<Frame> {
        self.0.relative(instance.0).map(Frame)
    }

    /// Why an instance's group is **unplaced** — `no_offset` (no member
    /// carries an offset) or `dead_gauge` (its gauge chain names a
    /// deleted gauge) — or `None` when it is placed or the node is not
    /// a live instance. An unplaced group lives in its own space: it
    /// evaluates in its own frame, and nothing outside it is compared
    /// with it.
    fn unplaced(&self, instance: &NodeId) -> Option<&'static str> {
        self.0
            .unplaced(instance.0)
            .map(|cause| unplaced_tag(&cause))
    }

    /// **The instance's world placement, at the document's own
    /// parameters**: its group's frame — the gauge chain composed with
    /// the root's offset — composed into the solved pose.
    ///
    /// A lone instance returns its offset's frame on its gauge, bit for
    /// bit, and on the world at the empty offset the identity.
    ///
    /// `doc` is read for its gauges and offsets, and it must be the
    /// document this solve is OF: passing a different one would
    /// compose this document's poses onto that one's placements, which
    /// is not a pose of either. The door refuses that first — a
    /// `SolvedPoses` carries the id of the document `solve_document`
    /// solved, and a mismatch raises `MateError` with tag
    /// `mate_poses_of_another_document` before any frame is read.
    ///
    /// Raises `MateError` when the instance's group did not solve, and
    /// `EvaluationError` — kind `unplaced`, or `placement_refused` with
    /// the placement's own refusal as the cause — when nothing places
    /// its group or a placement on its frame does not evaluate.
    fn placement(
        &self,
        py: Python<'_>,
        doc: &super::doc::Doc,
        instance: &NodeId,
    ) -> PyResult<Frame> {
        self.0
            .placement(&doc.inner, instance.0)
            .map(Frame)
            .map_err(|refusal| pose_err(py, *instance, &refusal, &self.1, &doc.inner))
    }

    fn __repr__(&self) -> String {
        "SolvedPoses()".to_string()
    }
}

/// Solve the document's mates: the per-pair coset fold along a
/// deterministic spanning tree, yielding every instance's pose
/// relative to its group root and every mate's role.
///
/// **Total — this never raises.** A refusing group must not fail an
/// unrelated one, so refusals are recorded per node and read back
/// through `SolvedPoses.fault`.
///
/// The solve reads no geometry except what each mated part's own
/// evaluation answers: its extent — an upper bound taken from its
/// evaluated body, entering only as the lever a parallelism verdict
/// is decided over — and, for a `MateFrame.from_face` side, that
/// face's canonical pose. So `resolver` is the same document seam
/// `evaluate(doc, resolver=)` crosses: a `Workspace`, or `None`, under
/// which every mate on a part refuses in the resolver's own voice
/// (`part_no_resolver`) rather than reading nothing — a face side as
/// `mate_face_unresolved` (read first), an authored one as
/// `mate_unleverable`. A face frame IS its face, so it cannot drift
/// from the part; the solve does NOT check that AUTHORED vectors
/// match the faces the mate's references name, which is why a
/// document authored so can solve cleanly and still refuse at the
/// at-rest gate.
#[pyfunction]
#[pyo3(signature = (doc, *, resolver=None))]
pub(crate) fn solve_document(
    doc: &super::doc::Doc,
    resolver: Option<&super::store::Workspace>,
) -> SolvedPoses {
    let tol = Tol::witness();
    let seam = super::doc::seam(resolver);
    let reach = d::PartReach::<f64>::with_resolver(seam.as_ref(), tol);
    SolvedPoses(
        d::solve_document(&doc.inner, &reach, tol),
        Arc::new(doc.inner.clone()),
    )
}

/// The **placement groups**: instances coupled by PLACING mates — both
/// instances on one gauge — each listed with its group's members in
/// document order. A mate-less document's groups are all singletons.
#[pyfunction]
pub(crate) fn groups(doc: &super::doc::Doc) -> Vec<Vec<NodeId>> {
    d::groups(&doc.inner)
        .into_iter()
        .map(|c| c.into_iter().map(NodeId).collect())
        .collect()
}

/// An instance's group ROOT: the earliest member, in document order,
/// that carries an offset — whose offset on the group's gauge places
/// the whole group — or the earliest instance when none does or its
/// gauge chain names a deleted gauge, and the group is unplaced.
///
/// Answers the node itself for a node that is not a live instance,
/// which is the kernel's own total shape.
#[pyfunction]
pub(crate) fn root_of(doc: &super::doc::Doc, instance: &NodeId) -> NodeId {
    NodeId(d::root_of(&doc.inner, instance.0))
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
/// Coarser than `groups`, which partitions instances alone.
#[pyfunction]
pub(crate) fn relative_freedom_components(doc: &super::doc::Doc) -> Vec<Vec<NodeId>> {
    d::relative_freedom_components(&doc.inner)
        .into_iter()
        .map(|c| c.into_iter().map(NodeId).collect())
        .collect()
}

// ---- The accepted edit's maintenance ----

/// One act of automatic maintenance an accepted edit performed: the
/// offset the mate door cleared, or a reference its delete stranded.
///
/// It rides the accepted edit rather than being an edit of its own —
/// automatic maintenance is deterministic from the edit, so a replay
/// reproduces it and undo (keeping the prior document) restores it
/// exactly. What the record adds is VISIBILITY: a cleared offset is
/// said where it was cleared, with the offset it held, and a stranded
/// name is said at the delete rather than at the next evaluation.
///
/// Payload attributes are present on every arm, `None` where
/// inapplicable: `node` and `offset` for an `offset_cleared` — a
/// member of the mate's first operand's group, now placed on the
/// second's, and the offset it gave up; `node` and `name` for a
/// strand, and `name` alone for a `stranded_appearance`, whose carrier is
/// the appearance store and not a node.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct Maintenance(pub(crate) d::Maintenance);

#[pymethods]
impl Maintenance {
    /// The stable tag: `offset_cleared`, `strand` or
    /// `stranded_appearance`, the three the stub lists for this
    /// attribute. The word decides which of the payload
    /// attributes below carry.
    // The map is `crate::tags::maintenance_tag`, whose words
    // `TAG_INVENTORY` pins.
    #[getter]
    fn variant(&self) -> &'static str {
        maintenance_tag(&self.0)
    }

    /// The node this row is about: the instance whose offset the mate
    /// door cleared, or the surviving node whose payload carries a
    /// stranded name. `None` for a `stranded_appearance`, which has
    /// no carrying node to name.
    ///
    /// The arms answer different questions with one attribute on
    /// purpose: each is the node a reader would go and look at, which
    /// is the whole use of the getter. `variant` says which question
    /// was answered.
    #[getter]
    fn node(&self) -> Option<NodeId> {
        match &self.0 {
            d::Maintenance::OffsetCleared { instance, .. } => Some(NodeId(instance.id())),
            d::Maintenance::Strand { node, .. } => Some(NodeId(node.id())),
            d::Maintenance::StrandedAppearance { .. } => None,
        }
    }

    /// The name this row is about, in the opaque text every name door
    /// on this surface speaks — the stranded payload name for a
    /// `strand`, the appearance store's stranded key for a
    /// `stranded_appearance`. A stranded name is spelled as the
    /// document holds it — its minting node deleted, or its profile
    /// piece no longer drawn — and `DocEdit.rebind` from that spelling
    /// is the repair this surface carries.
    #[getter]
    fn name(&self, py: Python<'_>) -> PyResult<Option<String>> {
        match &self.0 {
            d::Maintenance::Strand { name, .. } | d::Maintenance::StrandedAppearance { name } => {
                super::doc::name_text(py, name.name()).map(Some)
            }
            d::Maintenance::OffsetCleared { .. } => Ok(None),
        }
    }

    /// The offset the mate door cleared, for an `offset_cleared`.
    #[getter]
    fn offset(&self) -> Option<super::place::Placement> {
        match &self.0 {
            d::Maintenance::OffsetCleared { offset, .. } => {
                Some(super::place::Placement(offset.clone()))
            }
            d::Maintenance::Strand { .. } | d::Maintenance::StrandedAppearance { .. } => None,
        }
    }

    fn __repr__(&self) -> String {
        format!("Maintenance({:?})", self.variant())
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
    m.add_class::<Maintenance>()?;
    m.add_function(wrap_pyfunction!(solve_document, m)?)?;
    m.add_function(wrap_pyfunction!(groups, m)?)?;
    m.add_function(wrap_pyfunction!(root_of, m)?)?;
    m.add_function(wrap_pyfunction!(reading_edges, m)?)?;
    m.add_function(wrap_pyfunction!(relative_freedom_components, m)?)?;
    m.add_function(wrap_pyfunction!(class_admission, m)?)?;
    m.add("CLASS_DEFERRAL", d::CLASS_DEFERRAL)?;
    m.add("UNDER_RECOURSE", d::UNDER_RECOURSE)?;
    m.add("CONTRADICTORY_RECOURSE", d::CONTRADICTORY_RECOURSE)?;
    m.add("NO_AT_REST_RECORD_RECOURSE", d::NO_AT_REST_RECORD_RECOURSE)?;
    m.add("OFFSET_RECOURSE", d::OFFSET_RECOURSE)?;
    m.add("UNPLACED_RECOURSE", d::UNPLACED_RECOURSE)?;
    Ok(())
}
