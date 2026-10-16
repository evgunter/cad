//! **Mates**: a placement's constraints as values, the solve that
//! reads them, and the admission table that says how far a class gets.
//!
//! A mate is ONE value carrying both halves — the placement constraint
//! (which frames coincide, at which axis senses, at which clocking)
//! and the contact declaration (`Rest`, `Tangent`) — so there is no
//! second vocabulary to keep synced. It is a constraint of the
//! placement that holds it, addressed by a [`MateRef`], never a node;
//! everything the solve and the gate then say about it is here.
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
};
use std::sync::Arc;

use pncad::document as d;
use pncad::tolerance::Tol;

use super::doc::NodeId;
use super::place::{Frame, Placement};
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

/// One side's **mate frame**: a base composed with an offset, a
/// `Placement` written in the base's frame (`ASSEMBLY.md` A3).
///
/// **Part base** (`MateFrame.on_part(offset)`): the side's part frame.
/// Three authored vectors — `MateFrame(origin, axis, reference)` —
/// are the part base with one literal step: local +Z is `axis`, the
/// local origin `origin`, the roll fixed by `reference`. `axis` need
/// not be unit and `reference` need not be perpendicular to it: only
/// the axis's direction and the reference's perpendicular part are
/// read.
///
/// **Face base** (`MateFrame.from_face()`, or `MateFrame.on_face(offset)`):
/// no name. The side's frame is its own HEAD's face — the face the
/// mate's reference on that side names, read in the mated part —
/// whose canonical pose the solve reads off the part's own evaluation
/// at every evaluation (`ASSEMBLY.md` A11 rule 5): the carrier's
/// origin, its CHART axis as local +Z, and its own in-frame reference
/// direction as local +Y (the `point_at` convention: local +X is the
/// reference crossed with the axis). The offset is written in that
/// frame, so it slides
/// along the face, turns about its normal, or sets back from it, and
/// it follows the face through any edit of the part. The face's
/// orientation sense is not folded into the axis; the mate's
/// `AxisSense` says which way the sides point. A face with no
/// canonical frame (a NURBS carrier) refuses at the solve, typed.
///
/// The offset is any rigid motion; the mate's contact class says which
/// offsets are legal — a `Rest` side set back from its face declares a
/// contact that is not there, and the at-rest gate refutes it. A rigid
/// step's expressions can read a document parameter, so a parameter
/// can drive where a side sits; a literal step is held to the
/// placement frame's bar (finite, proper, rigid) at the edit door.
///
/// A face base resolves on every lane: a seed run reads the pose with
/// its tangent, a box run an enclosure of it, and so does a parameter
/// an offset step reads.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct MateFrame(pub(crate) d::MateFrame<d::Formula>);

#[pymethods]
impl MateFrame {
    /// Three authored vectors: the part base with the one literal step
    /// they denote.
    ///
    /// Raises `FrameError` when the axis has no definite direction,
    /// when the reference has no definite perpendicular offset from
    /// it, or — asked before either sign — when the axis's length or
    /// that perpendicular offset is not a finite number, whose
    /// `variant` reads `non_finite_aim` or `non_finite_roll_reference`.
    ///
    /// The frame is decided at the process tolerance, the one ε this
    /// session holds: every door here — `Doc.insert`, `Doc.apply` —
    /// decides at it, and a document recording another ε is refused
    /// before any of its geometry is read (D4). The literal step it
    /// stores is judged again where it lands, by the placement frame
    /// rule (finite, proper, rigid) at the insert door and at load.
    #[new]
    fn new(
        py: Python<'_>,
        origin: (Length, Length, Length),
        axis: (f64, f64, f64),
        reference: (f64, f64, f64),
    ) -> PyResult<Self> {
        d::MateFrame::authored(
            meters(origin),
            [axis.0, axis.1, axis.2],
            [reference.0, reference.1, reference.2],
            Tol::witness(),
        )
        .map(Self)
        .map_err(|err| super::place::frame_err(py, &err))
    }

    /// The part base composed with `offset`, written in the part's own
    /// coordinates.
    #[staticmethod]
    fn on_part(offset: &Placement) -> Self {
        Self(d::MateFrame::on_part(offset.0.clone()))
    }

    /// The side's own head face, with no offset: the face's pose
    /// itself (the class docs say how it is read).
    #[staticmethod]
    fn from_face() -> Self {
        Self(d::MateFrame::from_face())
    }

    /// The side's own head face composed with `offset`, written in the
    /// face's frame: origin on the face, +Z along its chart axis, +Y
    /// along its reference direction.
    #[staticmethod]
    fn on_face(offset: &Placement) -> Self {
        Self(d::MateFrame::on_face(offset.0.clone()))
    }

    /// The world's frame composed with `offset`, written in world
    /// coordinates: the frame of a side that reads the world
    /// (`Mate.world_pin`'s `a` side).
    #[staticmethod]
    fn on_world(offset: &Placement) -> Self {
        Self(d::MateFrame {
            base: d::FrameBase::World,
            offset: offset.0.clone(),
        })
    }

    /// What the offset is written in: `"part"`, `"face"` or `"world"`.
    #[getter]
    fn base(&self) -> &'static str {
        self.0.base.name()
    }

    /// The offset, in the base's frame.
    #[getter]
    fn offset(&self) -> Placement {
        Placement(self.0.offset.clone())
    }

    /// BIT-exact equality: the same base, and offsets equal by
    /// `Placement.__eq__`'s rule.
    fn __eq__(&self, other: &Self) -> bool {
        self.0.bit_eq(&other.0)
    }

    fn __repr__(&self) -> String {
        let offset = Placement(self.0.offset.clone()).__repr__();
        match self.0.base {
            d::FrameBase::Part => format!("MateFrame.on_part({offset})"),
            d::FrameBase::Face => format!("MateFrame.on_face({offset})"),
            d::FrameBase::World => format!("MateFrame.on_world({offset})"),
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
pub(crate) struct Alignment(pub(crate) d::Alignment<d::Formula>);

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

    /// BIT-exact equality, `MateFrame.__eq__`'s rule over both frames,
    /// with the primitive's lengths and the rider compared by bits.
    fn __eq__(&self, other: &Self) -> bool {
        self.0.bit_eq(&other.0)
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
    class_: super::contact::ContactClass,
) -> PyResult<ClassAdmission> {
    Ok(ClassAdmission(d::class_admission(class_.to_kernel(py)?)))
}

// ---- The solve's read side ----

/// **A constraint's id**: minted by the edit that adds it to a
/// placement, never reused and never positional. Opaque: it is read
/// off a `MateRef` and handed back, compared and hashed, never built.
#[pyclass(eq, ord, hash, frozen, module = "pncad", skip_from_py_object)]
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
        format!("MateRef(NodeId({}), {})", self.0.placement.full(), self.0.id)
    }
}

/// **A mate**: two sides and the alignment datum relating them, with
/// the contact class it declares. A mate is a constraint of the
/// placement that holds it (`Doc.constrain`, `Doc.place(shapes,
/// constraints)`), never a node: this is its payload as a value.
///
/// Each face side is TWO things: `a_at` / `b_at` is the node the face
/// is read at — the node whose geometry the mate speaks about — and
/// `a` / `b` the name text of a face of that node's output. One side
/// must be on the placement's own copy: a face of one of its shapes,
/// or of the instance itself. The other reads a pose outside the copy.
///
/// `class_` is the declared contact class (trailing underscore:
/// `class` is a Python keyword); `class_admission` says how far each
/// class gets. `alignment` is the datum: which frames coincide, at
/// which axis sense, with which clocking.
///
/// **A head must name a FACE, and that is refused here**: `a`/`b`
/// naming an edge raises `EditError` with `variant ==
/// "mate_head_not_a_face"` at this call. A dangling reference is not
/// refused here: the solve refuses typed naming its head
/// (`mate_dangling_head`), or the side's read when it is no face of a
/// live body (`mate_side_unresolved`).
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct Mate(pub(crate) d::Mate<d::Formula>);

#[pymethods]
impl Mate {
    #[new]
    fn new(
        py: Python<'_>,
        a_at: &NodeId,
        a: &str,
        b_at: &NodeId,
        b: &str,
        class_: super::contact::ContactClass,
        alignment: &Alignment,
    ) -> PyResult<Self> {
        let face = |at: &NodeId, text: &str| -> PyResult<d::Operand> {
            Ok(d::SitedFace::new(at.0, super::doc::face_name_from_text(py, text)?).into())
        };
        Ok(Self(d::Mate {
            a: d::SideRead::Face(face(a_at, a)?),
            b: d::SideRead::Face(face(b_at, b)?),
            class: class_.to_kernel(py)?,
            alignment: alignment.0.clone(),
        }))
    }

    /// **A copy pinned to the world at `pose`**, the identity when
    /// `None`: one coincidence of the world's frame, moved by `pose`,
    /// with the placement's own copy's frame. The world is the `a`
    /// side, so the copy sits at `pose` bit for bit.
    ///
    /// The pose is a `Placement` chain, `Node.transform_by`'s, and each
    /// rigid step's components are checked against the slot they land
    /// in, as there.
    #[staticmethod]
    #[pyo3(signature = (pose=None))]
    fn world_pin(py: Python<'_>, pose: Option<&Placement>) -> PyResult<Self> {
        let mate = d::Mate::world_pin(pose.map_or(d::Placement::IDENTITY, |p| p.0.clone()));
        let placeholder = d::ConstraintId(d::MintId::new(0, 0));
        super::doc::Node::check_slots(
            py,
            &d::Node::Place {
                shapes: Vec::new(),
                constraints: vec![d::Constraint {
                    id: placeholder,
                    mate: mate.clone(),
                }],
            },
        )?;
        Ok(Self(mate))
    }

    /// The declared contact class.
    #[getter]
    fn class_(&self, py: Python<'_>) -> PyResult<super::contact::ContactClass> {
        super::contact::contact_class(py, self.0.class)
    }

    /// The alignment datum.
    #[getter]
    fn alignment(&self) -> Alignment {
        Alignment(self.0.alignment.clone())
    }

    /// What the `a` side reads: `"face"` (a face, its frame read off
    /// it), `"copy"` (the placement's own copy, by its part frame) or
    /// `"world"`.
    #[getter]
    fn a(&self) -> &'static str {
        side_word(&self.0.a)
    }

    /// What the `b` side reads, in `a`'s words.
    #[getter]
    fn b(&self) -> &'static str {
        side_word(&self.0.b)
    }

    /// BIT-exact equality: the same sides and class, and alignments
    /// equal by `Alignment.__eq__`'s rule.
    fn __eq__(&self, other: &Self) -> bool {
        self.0.bit_eq(&other.0)
    }

    fn __repr__(&self) -> String {
        format!(
            "Mate(a={:?}, b={:?}, class_={:?}, primitive={})",
            side_word(&self.0.a),
            side_word(&self.0.b),
            self.0.class,
            mate_primitive_tag(self.0.alignment.primitive)
        )
    }
}

/// The word for what a side reads.
fn side_word<R>(side: &d::SideRead<R>) -> &'static str {
    match side {
        d::SideRead::Face(_) => "face",
        d::SideRead::Copy => "copy",
        d::SideRead::World => "world",
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

    /// The plane's unit normal, for `planar` and `plane_translation`.
    #[getter]
    fn normal(&self) -> Option<(f64, f64, f64)> {
        use d::Subgroup as S;
        match self.0 {
            S::Planar { normal } | S::PlaneTranslation { normal } => Some(direction(normal.get())),
            S::Se3
            | S::Cylindrical { .. }
            | S::Prismatic { .. }
            | S::Revolute { .. }
            | S::Spherical { .. }
            | S::Parallel { .. }
            | S::Translation
            | S::Trivial
            | S::Empty => None,
        }
    }

    /// A point on the axis, for `cylindrical` and `revolute`; the fixed
    /// point, for `spherical`.
    #[getter]
    fn point(&self) -> Option<(Length, Length, Length)> {
        use d::Subgroup as S;
        match self.0 {
            S::Cylindrical { point: p, .. }
            | S::Revolute { point: p, .. }
            | S::Spherical { point: p } => Some(point(p)),
            // `planar`, `prismatic`, `parallel` and `plane_translation`
            // are point-free on purpose, as the class doc says; the
            // remaining arms have no geometry to be based at.
            S::Planar { .. }
            | S::Prismatic { .. }
            | S::Parallel { .. }
            | S::PlaneTranslation { .. }
            | S::Translation
            | S::Se3
            | S::Trivial
            | S::Empty => None,
        }
    }

    /// The unit direction, for `cylindrical`, `prismatic`, `revolute`
    /// and `parallel`.
    #[getter]
    fn direction(&self) -> Option<(f64, f64, f64)> {
        use d::Subgroup as S;
        match self.0 {
            S::Cylindrical { direction: v, .. }
            | S::Prismatic { direction: v }
            | S::Revolute { direction: v, .. }
            | S::Parallel { direction: v } => Some(direction(v.get())),
            S::Se3
            | S::Planar { .. }
            | S::PlaneTranslation { .. }
            | S::Spherical { .. }
            | S::Translation
            | S::Trivial
            | S::Empty => None,
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
/// `instance`, `placement`, `constraints`, `base`, `residual`, `held`,
/// `added`,
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
    fn mate(&self) -> Option<MateRef> {
        self.payload().mate.map(MateRef)
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

    /// The instance a self-mate names twice, or the instance whose
    /// part a lever or a face frame was asked of.
    #[getter]
    fn instance(&self) -> Option<NodeId> {
        self.payload().instance.map(NodeId)
    }

    /// The placement a bundle's fault is about: the one a constraint
    /// over-constrains (`mate_overconstrained`), or whose constraints
    /// target different copies (`mate_targets_differ`).
    #[getter]
    fn placement(&self) -> Option<NodeId> {
        self.payload().placement.map(NodeId)
    }

    /// The constraints of `placement` the fault names, by address: on
    /// `mate_overconstrained` each whose removal leaves the bundle's
    /// codimension unchanged, on `mate_targets_differ` the bundle's
    /// constraints whose targets differ.
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

    /// The frame base a side carries that its read does not admit
    /// (`mate_side_base`): `"part"`, `"face"` or `"world"`.
    #[getter]
    fn base(&self) -> Option<&'static str> {
        self.payload().base.map(d::FrameBase::name)
    }

    /// The face a `from_face` side read — its head's face, as its name
    /// text in the PART's own spelling, or the head itself where it
    /// names no face of the part — where the refusal is about one
    /// (`mate_face_unresolved`, whose `inner_variant` names why: the
    /// part not in hand, the face's own row, tie or carrier, or
    /// `no_part_face`).
    #[getter]
    fn face(&self, py: Python<'_>) -> PyResult<Option<String>> {
        self.payload()
            .face
            .map(|face| super::doc::name_text(py, face))
            .transpose()
    }

    /// What the rest of an over-constrained bundle already holds the
    /// copy to (`mate_overconstrained`).
    #[getter]
    fn residual(&self) -> Option<Subgroup> {
        self.payload().residual.map(Subgroup)
    }

    /// The mate already folded, for a contradictory pair.
    #[getter]
    fn held(&self) -> Option<MateRef> {
        self.payload().held.map(MateRef)
    }

    /// The mate whose intersection died against it.
    #[getter]
    fn added(&self) -> Option<MateRef> {
        self.payload().added.map(MateRef)
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
/// recorded it over; and a loose copy as the `EvaluationError` a node
/// reading its space beside the world's carries (`space_mismatch`),
/// spoken from `doc`, the document the placement door read it off.
fn pose_err(
    py: Python<'_>,
    placement: NodeId,
    refusal: &d::PoseRefusal,
    solved: &Arc<d::ProfileDoc>,
    doc: &d::ProfileDoc,
) -> PyErr {
    let kind = match refusal {
        d::PoseRefusal::Mate(fault) => {
            return mate_err(py, fault, &Voice::Doc(Arc::clone(solved)));
        }
        d::PoseRefusal::Loose { root, .. } => d::NodeErrorKind::SpaceMismatch {
            a: d::Space::Loose { root: *root },
            b: d::Space::World,
        },
    };
    let err = super::value::refused(py, placement, &kind, refusal.spoken(doc), None);
    super::value::with_carried(py, err, kind.carried_chain(), Some(doc))
}

/// The document's solved poses: each placement's pose in its target's
/// space, and the per-placement refusals.
///
/// It keeps a copy of the document as it solved it: a fault is the
/// solve's, recorded over that version, so its words are spoken from it
/// even after the document is edited.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct SolvedPoses(d::SolvedPoses, Arc<d::ProfileDoc>);

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

    /// **The placement's solved pose in its target's space**: where its
    /// bundle puts its copy relative to the copy it is pinned to (the
    /// world, or another placement's copy), and in a loose copy's own
    /// space the pose it is evaluated at. `None` when the solve placed
    /// nothing there: a node that is no placement, or a placement whose
    /// bundle refused.
    fn pose(&self, placement: &NodeId) -> Option<Frame> {
        self.0.relative(placement.0).map(Frame)
    }

    /// **The placement's world pose, at the document's own
    /// parameters**: its solved pose composed into the world pose of
    /// the copy it is pinned to.
    ///
    /// A copy pinned to the world at `pose` returns that pose's frame,
    /// bit for bit, and at the identity the identity.
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
    /// loose, or pinned to a loose copy: it lives in a space of its own
    /// and has no world pose.
    fn placement(
        &self,
        py: Python<'_>,
        doc: &super::doc::Doc,
        placement: &NodeId,
    ) -> PyResult<Frame> {
        self.0
            .placement(&doc.inner, placement.0)
            .map(Frame)
            .map_err(|refusal| pose_err(py, *placement, &refusal, &self.1, &doc.inner))
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
            d::Maintenance::Strand { node, .. } | d::Maintenance::StrandedRead { node, .. } => {
                Some(NodeId(node.id()))
            }
            d::Maintenance::StrandedSelection { readers, .. } => {
                readers.first().map(|reader| NodeId(reader.id()))
            }
            d::Maintenance::StrandedAppearance { .. }
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
            d::Maintenance::StrandedRead { .. } | d::Maintenance::AnonymousVarRemoved { .. } => {
                Ok(None)
            }
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
    m.add_class::<ConstraintId>()?;
    m.add_class::<MateRef>()?;
    m.add_class::<Mate>()?;
    m.add_class::<Subgroup>()?;
    m.add_class::<MateFault>()?;
    m.add_class::<SolvedPoses>()?;
    m.add_class::<ClassAdmission>()?;
    m.add_class::<Maintenance>()?;
    m.add_function(wrap_pyfunction!(solve_document, m)?)?;
    m.add_function(wrap_pyfunction!(relative_freedom_components, m)?)?;
    m.add_function(wrap_pyfunction!(class_admission, m)?)?;
    m.add("CLASS_DEFERRAL", d::CLASS_DEFERRAL)?;
    m.add("OVERCONSTRAINED_RECOURSE", d::OVERCONSTRAINED_RECOURSE)?;
    m.add("CONTRADICTORY_RECOURSE", d::CONTRADICTORY_RECOURSE)?;
    m.add("NO_AT_REST_RECORD_RECOURSE", d::NO_AT_REST_RECORD_RECOURSE)?;
    Ok(())
}
