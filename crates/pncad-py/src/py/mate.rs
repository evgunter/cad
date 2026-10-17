//! **Mates**: a placement's constraints as values, the solve that
//! reads them, and the admission table that says how far a class gets.
//!
//! A mate is ONE value carrying both halves — the placement constraint
//! (two poses of one kind equated) and the contact declaration
//! (`Rest`, `Tangent`) — so there is no second vocabulary to keep
//! synced. A value sets one freedom a mate leaves. Each is a constraint
//! of the placement that holds it, addressed by a [`MateRef`], never a
//! node; everything the solve and the gate then say about it is here.
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
    class_admission_tag, coord_tag, maintenance_tag, mate_fault_tag, no_chart_why_tag,
    placed_by_tag, subgroup_tag,
};
use std::sync::Arc;

use pncad::document as d;
use pncad::tolerance::Tol;

use super::doc::NodeId;
use super::place::Frame;
use super::quantity::{Angle, Length};

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

/// **A pose of the world, read inside a mate**: the world defines no
/// pose variable, so a mate's `to` reads it here — its frame, a role
/// of it, and the moves a target may make: a slide along a direction
/// or an axis, a spin about an axis, a flip.
///
/// `WorldRead.frame()` is the world's frame, `WorldRead.through(role)`
/// the frame read through a role (`"x"`, `"y"` or `"z"` its axes,
/// `"origin"`, `"xy"`, `"yz"` or `"zx"` its planes). Each move answers
/// a new read: `read.role("normal")` a role of a plane read,
/// `read.slide(along, by)` slid along `along` (a direction or an axis
/// read) by the length `by`, `read.spin(about, by)` spun about `about`
/// (an axis read) by the angle `by`, right-handed, `read.flip()` the
/// opposite sense. A lone `0` is the constant zero of the slot's
/// dimension.
///
/// A read whose kinds do not compose (a slide of a direction, a spin
/// about a plane, a role its pose's kind does not offer, a flipped
/// point) is built here and refused where its mate is added
/// (`mate_world_read`); `kind` answers `None` for it.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct WorldRead(pub(crate) d::WorldRead<d::Formula>);

/// A move's number, at its dimension.
fn move_by(
    py: Python<'_>,
    by: &super::expr::SlotArg,
    dim: d::Dimension,
    what: &str,
) -> PyResult<d::Formula> {
    super::pose::scalar_at(py, dim, by, what)
}

#[pymethods]
impl WorldRead {
    /// The world's own frame.
    #[staticmethod]
    fn frame() -> Self {
        Self(d::WorldRead::Frame)
    }

    /// The world's frame read through `role`: `"x"`, `"y"`, `"z"`,
    /// `"origin"`, `"xy"`, `"yz"` or `"zx"`.
    #[staticmethod]
    fn through(role: &str) -> PyResult<Self> {
        Ok(Self(d::WorldRead::through(super::pose::role_of(role)?)))
    }

    /// This read through `role` (a plane's `"normal"`, an axis's
    /// `"direction"`, a frame's axes, origin and planes).
    fn role(&self, role: &str) -> PyResult<Self> {
        Ok(Self(d::WorldRead::Role {
            pose: Box::new(self.0.clone()),
            role: super::pose::role_of(role)?,
        }))
    }

    /// This read slid along `along` by the length `by`.
    fn slide(
        &self,
        py: Python<'_>,
        along: &Self,
        by: super::expr::SlotArg,
    ) -> PyResult<Self> {
        let by = move_by(py, &by, d::Dimension::Length, "slide")?;
        Ok(Self(self.0.clone().slide(along.0.clone(), by)))
    }

    /// This read spun about `about` by the angle `by`, right-handed.
    fn spin(&self, py: Python<'_>, about: &Self, by: super::expr::SlotArg) -> PyResult<Self> {
        let by = move_by(py, &by, d::Dimension::Angle, "spin")?;
        Ok(Self(self.0.clone().spin(about.0.clone(), by)))
    }

    /// This read's opposite sense.
    fn flip(&self) -> Self {
        Self(self.0.clone().flip())
    }

    /// The kind of pose this read is (`"point"`, `"direction"`,
    /// `"axis"`, `"plane"` or `"frame"`), `None` where its moves do not
    /// compose.
    #[getter]
    fn kind(&self) -> Option<&'static str> {
        self.0.kind().ok().map(crate::errors::var_kind_tag)
    }

    /// Bit-exact equality: the same moves, each number equal.
    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __repr__(&self) -> String {
        world_text(&self.0)
    }
}

/// A world read as the Python that builds it.
fn world_text(read: &d::WorldRead<d::Formula>) -> String {
    match read {
        d::WorldRead::Frame => "WorldRead.frame()".to_owned(),
        d::WorldRead::Role { pose, role } => {
            format!("{}.role({:?})", world_text(pose), super::pose::role_word(*role))
        }
        d::WorldRead::Slide { pose, along, by } => {
            format!("{}.slide({}, {by})", world_text(pose), world_text(along))
        }
        d::WorldRead::Spin { pose, about, by } => {
            format!("{}.spin({}, {by})", world_text(pose), world_text(about))
        }
        d::WorldRead::Flip { pose } => format!("{}.flip()", world_text(pose)),
    }
}

/// **What a mate's `to` reads**: a world read, or a pose of the space
/// the copy joins.
#[derive(FromPyObject)]
pub(crate) enum TargetArg {
    /// The world.
    World(WorldRead),
    /// A pose operand.
    Pose(super::doc::OperandArg),
}

/// **A mate**: two poses of one kind equated, with the contact class it
/// declares. A mate is a constraint of the placement that holds it
/// (`Doc.constrain`, `Doc.place(shapes, constraints)`), never a node:
/// this is its payload as a value.
///
/// `on` is a pose read off the copied shapes' geometry (a `PoseDef`
/// over one of the placement's shapes, or over the instance itself);
/// `to` is a pose of the space the copy joins, or a `WorldRead`. The
/// kind is the primitive: plane to plane rests, axis to axis is
/// coaxial, frame to frame coincides, point to point is a ball, and
/// direction to direction is parallel. A plain plane mate equates
/// outward normals, so two faces resting against each other carry a
/// flip on the target (`PoseDef.flip`). A mate holds no number: what
/// it leaves free is set by a `Value`, and a standoff is a slide of the
/// target plane along its own normal.
///
/// `class_` is the declared contact class (trailing underscore:
/// `class` is a Python keyword); `class_admission` says how far each
/// class gets.
///
/// Where it is added, the door refuses an `on` that does not read the
/// copy (`mate_not_on_the_copy`), poses of two kinds
/// (`mate_kinds_differ`) and a world read whose moves do not compose
/// (`mate_world_read`). A pose that does not evaluate — a face gone,
/// a curved face read as a plane — is the placement's own refusal
/// where it evaluates (`mate_side_unevaluated`).
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct Mate(pub(crate) d::Mate<d::Formula>);

#[pymethods]
impl Mate {
    #[new]
    fn new(
        py: Python<'_>,
        on: super::doc::OperandArg,
        to: TargetArg,
        class_: super::contact::ContactClass,
    ) -> PyResult<Self> {
        Ok(Self(d::Mate {
            on: on.read(),
            to: match to {
                TargetArg::World(world) => d::Target::World(world.0),
                TargetArg::Pose(pose) => d::Target::Pose(pose.read()),
            },
            class: class_.to_kernel(py)?,
        }))
    }

    /// The copy's pose, as written.
    #[getter]
    fn on(&self) -> super::pose::PoseDef {
        super::pose::PoseDef {
            inner: self.0.on.clone(),
        }
    }

    /// The pose the copy's is equated with: a `PoseDef` or a
    /// `WorldRead`.
    #[getter]
    fn to(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(match &self.0.to {
            d::Target::Pose(read) => Py::new(
                py,
                super::pose::PoseDef {
                    inner: read.clone(),
                },
            )?
            .into_any(),
            d::Target::World(world) => Py::new(py, WorldRead(world.clone()))?.into_any(),
        })
    }

    /// The declared contact class.
    #[getter]
    fn class_(&self, py: Python<'_>) -> PyResult<super::contact::ContactClass> {
        super::contact::contact_class(py, self.0.class)
    }

    /// BIT-exact equality: the same poses, the same world moves, the
    /// same class.
    fn __eq__(&self, other: &Self) -> bool {
        self.0.bit_eq(&other.0)
    }

    fn __repr__(&self) -> String {
        let to = match &self.0.to {
            d::Target::Pose(read) => read.to_string(),
            d::Target::World(world) => world_text(world),
        };
        format!("Mate(on={}, to={to}, class_={:?})", self.0.on, self.0.class)
    }
}

/// **Which freedom of its mate a value sets**: `Coord.slide("x")`,
/// `"y"` or `"z"` — a slide along the chart's axis, `z` the mated
/// direction — or `Coord.spin()`, the turn about the mated direction.
///
/// A plane mate leaves the slides along `x` and `y` and the spin; an
/// axis mate the slide along `z` (its axis) and the spin; a direction
/// mate all four; a point mate's rotation has no chart; a frame mate
/// leaves nothing.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Coord(pub(crate) d::Coord);

#[pymethods]
impl Coord {
    /// The slide along the chart's `axis`: `"x"`, `"y"` or `"z"`.
    #[staticmethod]
    fn slide(axis: &str) -> PyResult<Self> {
        Ok(Self(d::Coord::Slide(match axis {
            "x" => d::SlideAxis::X,
            "y" => d::SlideAxis::Y,
            "z" => d::SlideAxis::Z,
            other => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "not a slide axis: {other:?} — one of \"x\", \"y\" or \"z\""
                )));
            }
        })))
    }

    /// The spin about the mated direction.
    #[staticmethod]
    fn spin() -> Self {
        Self(d::Coord::Spin)
    }

    /// The stable word: `slide_x`, `slide_y`, `slide_z` or `spin`.
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
        match self.0 {
            d::Coord::Slide(d::SlideAxis::X) => "Coord.slide(\"x\")".to_owned(),
            d::Coord::Slide(d::SlideAxis::Y) => "Coord.slide(\"y\")".to_owned(),
            d::Coord::Slide(d::SlideAxis::Z) => "Coord.slide(\"z\")".to_owned(),
            d::Coord::Spin => "Coord.spin()".to_owned(),
        }
    }
}

/// **A value**: one freedom a mate of the same bundle leaves, set to a
/// number — a `Length` for a slide, an `Angle` for a spin (a lone `0`
/// is the constant zero).
///
/// It is charted on the two bodies' own frames, each swung onto the
/// mated direction by the shortest turn: a slide is the copy's origin
/// measured from the target's along the chart's axis, a spin the angle
/// about the mated direction from the target's reference to the
/// copy's. Zero says the two bodies' own coordinates agree as far as
/// the mate allows.
///
/// `mate` names the mate by its id (a `MateRef`'s `constraint`, or the
/// `MateRef` itself). `Doc.add_value` adds one; the door refuses an id
/// that is no mate of the bundle (`mate_value_names_no_mate`), a
/// freedom the mate's kind does not leave (`mate_not_a_freedom`) and a
/// point mate's rotation (`mate_no_chart`).
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct Value(pub(crate) d::Value<d::Formula>);

/// **The mate a value names**: its id, or its address.
#[derive(FromPyObject)]
pub(crate) enum MateIdArg {
    /// An address: its id is read.
    Ref(MateRef),
    /// An id.
    Id(ConstraintId),
}

impl MateIdArg {
    fn id(&self) -> d::ConstraintId {
        match self {
            Self::Ref(r) => r.0.id,
            Self::Id(id) => id.0,
        }
    }
}

impl Value {
    /// The value `by` on `coord` of `mate`, `by` read at the
    /// freedom's dimension.
    pub(crate) fn of(
        py: Python<'_>,
        mate: d::ConstraintId,
        coord: d::Coord,
        by: &super::expr::SlotArg,
    ) -> PyResult<Self> {
        let by = super::pose::scalar_at(py, coord.dimension(), by, coord.name())?;
        Ok(Self(d::Value { mate, coord, by }))
    }
}

#[pymethods]
impl Value {
    #[new]
    fn new(
        py: Python<'_>,
        mate: MateIdArg,
        coord: &Coord,
        by: super::expr::SlotArg,
    ) -> PyResult<Self> {
        Self::of(py, mate.id(), coord.0, &by)
    }

    /// The id of the mate whose freedom it sets.
    #[getter]
    fn mate(&self) -> ConstraintId {
        ConstraintId(self.0.mate)
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
            "Value({}, {}, {})",
            self.0.mate,
            Coord(self.0.coord).__repr__(),
            self.0.by
        )
    }
}

/// **What the "place part" gesture wrote** (`Doc.place_at`): the
/// placement, its mate and its values, and how the copy was placed —
/// `by == "planar_face"`, a plane mate on the copy's first planar face
/// to the world plane it lies in at the pose, and the three values
/// (slides along `x` and `y`, and the spin) that put it there; or
/// `by == "pin"`, a copy with no planar face pinned over a frame read
/// off its geometry, with no freedom left to set.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct Placed {
    pub(crate) placement: NodeId,
    pub(crate) mate: MateRef,
    pub(crate) values: Vec<MateRef>,
    pub(crate) by: d::PlacedBy,
}

#[pymethods]
impl Placed {
    /// The placement.
    #[getter]
    fn placement(&self) -> NodeId {
        self.placement
    }

    /// The mate's address.
    #[getter]
    fn mate(&self) -> MateRef {
        self.mate
    }

    /// The values' addresses: slide `x`, slide `y`, spin; empty for a
    /// pin.
    #[getter]
    fn values(&self) -> Vec<MateRef> {
        self.values.clone()
    }

    /// `"planar_face"` or `"pin"`.
    #[getter]
    fn by(&self) -> &'static str {
        placed_by_tag(self.by)
    }

    fn __repr__(&self) -> String {
        format!(
            "Placed(placement={}, by={:?})",
            self.placement.0.full(),
            placed_by_tag(self.by)
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
/// arm does not carry it: `mate`, `side`, `error`, `instance`,
/// `shape`, `placement`, `constraints`, `residual`, `held`, `added`,
/// `predicate`, `clash`, `what`, `expected_document`,
/// `found_document`, `inner_variant`, `margin`, `margin_low`,
/// `margin_high`, `zero`, `escalate`, `field`, `value`, `lever_tilt`,
/// `lever_residual`, `lever_arm`, `on_kind`, `to_kind`, `value_ref`,
/// `named`, `coord`, `mate_kind`, `why`, `cause`. The human
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
    /// (`mate_value_names_no_mate`, `mate_not_a_freedom`,
    /// `mate_no_chart`).
    #[getter]
    fn value_ref(&self) -> Option<MateRef> {
        self.payload().value_ref.map(MateRef)
    }

    /// The id a value names that is no mate of its bundle
    /// (`mate_value_names_no_mate`).
    #[getter]
    fn named(&self) -> Option<ConstraintId> {
        self.payload().named.map(ConstraintId)
    }

    /// The freedom a value sets that its mate does not leave
    /// (`mate_not_a_freedom`).
    #[getter]
    fn coord(&self) -> Option<Coord> {
        self.payload().coord.map(Coord)
    }

    /// The kind of the mate whose kind alone fixes that freedom, `None`
    /// where the bundle's other constraints fix it
    /// (`mate_not_a_freedom`).
    #[getter]
    fn mate_kind(&self) -> Option<&'static str> {
        self.payload().mate_kind.map(crate::errors::var_kind_tag)
    }

    /// Why a value's freedom has no chart (`mate_no_chart`):
    /// `"point_rotation"` or `"two_bodies"`.
    #[getter]
    fn why(&self) -> Option<&'static str> {
        self.payload().why.map(no_chart_why_tag)
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

    /// **The nested refusal's own word**: the band constructor's, the
    /// lever refusal's, or the world read's (`mate_world_read`). `None` on an arm whose payload is a struct
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
/// recorded it over; a placement that did not evaluate for a reason
/// not its bundle's as its own standing in `evaluation`; and a loose
/// copy as the `EvaluationError` a node reading its space beside the
/// world's carries (`space_mismatch`), spoken from `doc`, the document
/// the placement door read it off.
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
        d::PoseRefusal::Loose { root, .. } => d::NodeErrorKind::SpaceMismatch {
            a: d::Space::Loose { root: *root },
            b: d::Space::World,
        },
    };
    let said = d::Said(refusal, d::Speaker::of(doc)).to_string();
    let err = super::value::refused(py, placement, &kind, said, None);
    super::value::with_carried(py, err, kind.carried_chain(), Some(doc))
}

/// The document's solved poses: each placement's pose in its target's
/// space, and the per-placement refusals.
///
/// It keeps a copy of the document as it solved it: a fault is the
/// solve's, recorded over that version, so its words are spoken from it
/// even after the document is edited.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct SolvedPoses(
    d::SolvedPoses,
    Arc<d::ProfileDoc>,
    Arc<d::Evaluation<f64>>,
);

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
    m.add_class::<MateSide>()?;
    m.add_class::<WorldRead>()?;
    m.add_class::<Coord>()?;
    m.add_class::<Value>()?;
    m.add_class::<Placed>()?;
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
