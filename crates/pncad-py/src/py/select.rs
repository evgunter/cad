//! The selector surface: structural patterns, geometric predicates,
//! and the vocabulary both are written in.
//!
//! # What crosses, and what stays behind
//!
//! Everything here is a VALUE built in Python and evaluated in Rust:
//! a [`Selector`] is role-path shape, a [`GeomPred`] is a geometric
//! atom, and `Evaluation.select` / `Evaluation.select_where` hand both
//! to `pncad::select`'s own materializers. No geometry is interrogated
//! on the Python side, and no second matcher exists to drift.
//!
//! The names that come back are the SAME opaque texts the whole-body
//! materializers answer with (`doc::name_text`) — the one alphabet
//! `Node.fillet` reads. The rule that keeps: narrowing
//! a set happens through these doors, never by reading inside a name.
//!
//! # The exact/decided split survives the crossing
//!
//! `GeomPred`'s Rust docs split the atoms into EXACT (tag reads —
//! total, no funnel, cannot refuse) and DECIDED (`datum_distance`, a
//! funnel site whose in-band candidates REFUSE). The split crosses as
//! TYPED structure, not as a boolean: a decided in-band margin raises
//! `SelectRefusal` with `reason="in_band"`, a tied name whose
//! candidates disagree raises `reason="tied_disagrees"` — exactly the
//! honesty obligations `editor_core::names::SelectRefusal` documents,
//! never a silent include or drop.

use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::errors::{ErrorClass, dimension_tag};
use crate::py::doc::{NodeId, literal, name_from_text};
use crate::py::quantity::Length;
use crate::py::typed_err;
use crate::tags::select_refusal_tag;
use pncad::document as d;
use pncad::prelude::SurfaceKind as KSurfaceKind;
use pncad::select as s;

// ---------------------------------------------------------------
// The enum vocabulary: fieldless mirrors of the kernel's own enums,
// crossing one-way (Python constructs, Rust evaluates). Each mirror
// has a match in `growth_tripwire` (end of file) over the KERNEL
// enum, EXHAUSTIVE with no wildcard arm, so a kernel variant this
// file does not know stops the build instead of silently not
// crossing.
// ---------------------------------------------------------------

/// Which entity kind a name denotes.
#[pyclass(eq, eq_int, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `editor_core::EntityKind` variant of the same name"
)]
pub(crate) enum EntityKind {
    Body,
    Face,
    Edge,
    Vertex,
}

impl EntityKind {
    fn to_kernel(self) -> s::EntityKind {
        match self {
            Self::Body => s::EntityKind::Body,
            Self::Face => s::EntityKind::Face,
            Self::Edge => s::EntityKind::Edge,
            Self::Vertex => s::EntityKind::Vertex,
        }
    }
}

/// Crossing helper: the kernel kind as the Python mirror.
///
/// Exhaustive over the KERNEL enum with no wildcard arm, so a kernel
/// kind this file does not mirror stops the build — the direction the
/// tripwire module at the foot of this file asserts for every other
/// mirrored enum, and which this one asserts by being called.
pub(crate) fn entity_kind(kind: s::EntityKind) -> EntityKind {
    match kind {
        s::EntityKind::Body => EntityKind::Body,
        s::EntityKind::Face => EntityKind::Face,
        s::EntityKind::Edge => EntityKind::Edge,
        s::EntityKind::Vertex => EntityKind::Vertex,
    }
}

/// Which role-segment variant a [`SegPat`] names — the fieldless
/// mirror of the role vocabulary, one tag per op-minted role.
#[pyclass(eq, eq_int, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `editor_core::SegTag` variant of the same name"
)]
pub(crate) enum SegTag {
    // Shared
    OutputBody,
    // Extrude
    Cap,
    Lateral,
    RimEdge,
    LateralEdge,
    CapVertex,
    // Revolve
    Band,
    BandRim,
    BandRimPi,
    BandPi,
    Meridian,
    MeridianVertex,
    RevolveCap,
    Pole,
    AxisEdge,
    // Boolean
    FromA,
    FromB,
    Seam,
    Merged,
    Fragment,
    // Split
    SplitBody,
    SectionFace,
    SectionEdge,
    SplitFragment,
    CrossingVertex,
    OnToolVertex,
    // Fillet
    FromTarget,
    BlendFace,
    CornerFace,
    TrimEdge,
    FootVertex,
    CornerArc,
    BandFace,
    BandTrim,
    BandFoot,
    BandCross,
    BandCut,
    BandSlit,
    // Pattern
    Instance,
    // Instantiate part
    InPart,
}

impl SegTag {
    fn to_kernel(self) -> s::SegTag {
        match self {
            Self::OutputBody => s::SegTag::OutputBody,
            Self::Cap => s::SegTag::Cap,
            Self::Lateral => s::SegTag::Lateral,
            Self::RimEdge => s::SegTag::RimEdge,
            Self::LateralEdge => s::SegTag::LateralEdge,
            Self::CapVertex => s::SegTag::CapVertex,
            Self::Band => s::SegTag::Band,
            Self::BandRim => s::SegTag::BandRim,
            Self::BandRimPi => s::SegTag::BandRimPi,
            Self::BandPi => s::SegTag::BandPi,
            Self::Meridian => s::SegTag::Meridian,
            Self::MeridianVertex => s::SegTag::MeridianVertex,
            Self::RevolveCap => s::SegTag::RevolveCap,
            Self::Pole => s::SegTag::Pole,
            Self::AxisEdge => s::SegTag::AxisEdge,
            Self::FromA => s::SegTag::FromA,
            Self::FromB => s::SegTag::FromB,
            Self::Seam => s::SegTag::Seam,
            Self::Merged => s::SegTag::Merged,
            Self::Fragment => s::SegTag::Fragment,
            Self::SplitBody => s::SegTag::SplitBody,
            Self::SectionFace => s::SegTag::SectionFace,
            Self::SectionEdge => s::SegTag::SectionEdge,
            Self::SplitFragment => s::SegTag::SplitFragment,
            Self::CrossingVertex => s::SegTag::CrossingVertex,
            Self::OnToolVertex => s::SegTag::OnToolVertex,
            Self::FromTarget => s::SegTag::FromTarget,
            Self::BlendFace => s::SegTag::BlendFace,
            Self::CornerFace => s::SegTag::CornerFace,
            Self::TrimEdge => s::SegTag::TrimEdge,
            Self::FootVertex => s::SegTag::FootVertex,
            Self::CornerArc => s::SegTag::CornerArc,
            Self::BandFace => s::SegTag::BandFace,
            Self::BandTrim => s::SegTag::BandTrim,
            Self::BandFoot => s::SegTag::BandFoot,
            Self::BandCross => s::SegTag::BandCross,
            Self::BandCut => s::SegTag::BandCut,
            Self::BandSlit => s::SegTag::BandSlit,
            Self::Instance => s::SegTag::Instance,
            Self::InPart => s::SegTag::InPart,
        }
    }
}

/// The op group a role segment belongs to (`SegPat.group`'s argument).
#[pyclass(eq, eq_int, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `editor_core::OpGroup` variant of the same name"
)]
pub(crate) enum OpGroup {
    Shared,
    Extrude,
    Revolve,
    Boolean,
    Split,
    Fillet,
    Pattern,
    InstantiatePart,
}

impl OpGroup {
    fn to_kernel(self) -> s::OpGroup {
        match self {
            Self::Shared => s::OpGroup::Shared,
            Self::Extrude => s::OpGroup::Extrude,
            Self::Revolve => s::OpGroup::Revolve,
            Self::Boolean => s::OpGroup::Boolean,
            Self::Split => s::OpGroup::Split,
            Self::Fillet => s::OpGroup::Fillet,
            Self::Pattern => s::OpGroup::Pattern,
            Self::InstantiatePart => s::OpGroup::InstantiatePart,
        }
    }
}

/// An extrude/revolve cap end (`SegPat.side`'s extrude spelling).
#[pyclass(eq, eq_int, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `editor_core::CapEnd` variant of the same name"
)]
pub(crate) enum CapEnd {
    Top,
    Bottom,
}

/// A revolve meridian end.
#[pyclass(eq, eq_int, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `editor_core::MeridianEnd` variant of the same name"
)]
pub(crate) enum MeridianEnd {
    Start,
    End,
    Seam,
    Pi,
}

/// A split output half.
#[pyclass(eq, eq_int, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `editor_core::SplitHalf` variant of the same name"
)]
pub(crate) enum SplitHalf {
    Above,
    Below,
}

/// Which support of a rim blend.
#[pyclass(eq, eq_int, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `editor_core::RimSupport` variant of the same name"
)]
pub(crate) enum RimSupport {
    Host,
    Mate,
}

/// The end/side argument `SegPat.side` accepts: any one of the four
/// closed side vocabularies, exactly as Rust's `side(impl Into<Side>)`
/// accepts them. The union is the BOUNDARY's typed extraction, not a
/// Python-side `isinstance` ladder — the same overloading Rust spells
/// with `From` impls.
#[derive(FromPyObject)]
pub(crate) enum SideArg {
    /// `CapEnd`.
    Cap(CapEnd),
    /// `MeridianEnd`.
    Meridian(MeridianEnd),
    /// `SplitHalf`.
    Split(SplitHalf),
    /// `RimSupport`.
    Rim(RimSupport),
}

impl SideArg {
    fn to_kernel(&self) -> s::Side {
        match self {
            Self::Cap(CapEnd::Top) => s::Side::Cap(s::CapEnd::Top),
            Self::Cap(CapEnd::Bottom) => s::Side::Cap(s::CapEnd::Bottom),
            Self::Meridian(MeridianEnd::Start) => s::Side::Meridian(s::MeridianEnd::Start),
            Self::Meridian(MeridianEnd::End) => s::Side::Meridian(s::MeridianEnd::End),
            Self::Meridian(MeridianEnd::Seam) => s::Side::Meridian(s::MeridianEnd::Seam),
            Self::Meridian(MeridianEnd::Pi) => s::Side::Meridian(s::MeridianEnd::Pi),
            Self::Split(SplitHalf::Above) => s::Side::Split(s::SplitHalf::Above),
            Self::Split(SplitHalf::Below) => s::Side::Split(s::SplitHalf::Below),
            Self::Rim(RimSupport::Host) => s::Side::Rim(s::RimSupport::Host),
            Self::Rim(RimSupport::Mate) => s::Side::Rim(s::RimSupport::Mate),
        }
    }
}

/// Which curve variant an edge's certified carrier is — the EXACT
/// atom `GeomPred.curve_kind` matches on.
#[pyclass(eq, eq_int, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `editor_core::CurveKind` variant of the same name"
)]
pub(crate) enum CurveKind {
    Line,
    Circle,
    Ellipse,
    Nurbs,
}

impl CurveKind {
    fn to_kernel(self) -> s::CurveKind {
        match self {
            Self::Line => s::CurveKind::Line,
            Self::Circle => s::CurveKind::Circle,
            Self::Ellipse => s::CurveKind::Ellipse,
            Self::Nurbs => s::CurveKind::Nurbs,
        }
    }
}

/// Which surface variant a face is — `GeomPred.surface_kind` and both
/// sides of `GeomPred.adjacent_kinds` match on it.
#[pyclass(eq, eq_int, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `geom_brep::SurfaceKind` variant of the same name"
)]
pub(crate) enum SurfaceKind {
    Plane,
    Cylinder,
    Cone,
    Sphere,
    Torus,
    Nurbs,
    Approx,
}

impl SurfaceKind {
    fn to_kernel(self) -> KSurfaceKind {
        match self {
            Self::Plane => KSurfaceKind::Plane,
            Self::Cylinder => KSurfaceKind::Cylinder,
            Self::Cone => KSurfaceKind::Cone,
            Self::Sphere => KSurfaceKind::Sphere,
            Self::Torus => KSurfaceKind::Torus,
            Self::Nurbs => KSurfaceKind::Nurbs,
            Self::Approx => KSurfaceKind::Approx,
        }
    }
}

/// The comparison `GeomPred.datum_distance` makes: the SIGN trilean
/// (within the ε-band / definitely greater / definitely less), never a
/// bare float equality. A candidate whose margin lands INSIDE the band
/// answers neither strict arm and REFUSES (`SelectRefusal`,
/// `reason="in_band"`).
#[pyclass(eq, eq_int, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `editor_core::Cmp` variant of the same name"
)]
pub(crate) enum Cmp {
    Approx,
    Greater,
    Less,
}

impl Cmp {
    fn to_kernel(self) -> s::Cmp {
        match self {
            Self::Approx => s::Cmp::Approx,
            Self::Greater => s::Cmp::Greater,
            Self::Less => s::Cmp::Less,
        }
    }
}

/// A set-of-kinds argument: one kind, or a list of them. Mirrors the
/// Rust pair `CurveKindSet::just` / `CurveKindSet::of` as one door —
/// the SET semantics is the kernel's (an empty list matches nothing).
#[derive(FromPyObject)]
pub(crate) enum CurveKinds {
    /// The singleton set — the common case.
    One(CurveKind),
    /// The set of exactly these kinds.
    Many(Vec<CurveKind>),
}

impl CurveKinds {
    fn to_kernel(&self) -> s::CurveKindSet {
        match self {
            Self::One(k) => s::CurveKindSet::just(k.to_kernel()),
            Self::Many(ks) => s::CurveKindSet::of(ks.iter().map(|k| k.to_kernel())),
        }
    }
}

/// [`CurveKinds`]'s face-side twin.
#[derive(FromPyObject)]
pub(crate) enum SurfaceKinds {
    /// The singleton set — the common case.
    One(SurfaceKind),
    /// The set of exactly these kinds.
    Many(Vec<SurfaceKind>),
}

impl SurfaceKinds {
    fn to_kernel(&self) -> s::SurfaceKindSet {
        match self {
            Self::One(k) => s::SurfaceKindSet::just(k.to_kernel()),
            Self::Many(ks) => s::SurfaceKindSet::of(ks.iter().map(|k| k.to_kernel())),
        }
    }
}

// ---------------------------------------------------------------
// The structural pattern classes. Each wraps the kernel value and the
// builder verbs return NEW frozen values (Rust's consuming builders,
// crossed as Python's immutable ones). Matching happens in Rust —
// `matches` parses the opaque name text at the boundary and asks the
// kernel's own matcher, so no second matcher exists to drift.
// ---------------------------------------------------------------

/// A pattern over ONE role segment: which variant, which side, what
/// its sub-name arguments look like — the three axes a segment has.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct SegPat(pub(crate) s::SegPat);

#[pymethods]
impl SegPat {
    /// The wildcard segment pattern.
    #[staticmethod]
    fn any() -> Self {
        Self(s::SegPat::any())
    }

    /// Segments of this variant.
    #[staticmethod]
    fn tag(tag: SegTag) -> Self {
        Self(s::SegPat::tag(tag.to_kernel()))
    }

    /// Segments minted by this op.
    #[staticmethod]
    fn group(group: OpGroup) -> Self {
        Self(s::SegPat::group(group.to_kernel()))
    }

    /// Constrains the end/side tag (`.side(CapEnd.Top)`). Accepts any
    /// of the four side vocabularies: `CapEnd`, `MeridianEnd`,
    /// `SplitHalf`, `RimSupport`.
    fn side(&self, side: SideArg) -> Self {
        Self(self.0.clone().side(side.to_kernel()))
    }

    /// Constrains the segment's sub-name arguments, positionally, as
    /// a PREFIX: `[p]` constrains the first argument only.
    fn of(&self, args: Vec<NamePat>) -> Self {
        Self(self.0.clone().of(args.into_iter().map(|p| p.0)))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

/// A pattern over a whole stable name: entity kind, minting node, and
/// the exact shape of the role path.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct NamePat(pub(crate) s::NamePat);

#[pymethods]
impl NamePat {
    /// The wildcard: matches every name.
    #[staticmethod]
    fn any() -> Self {
        Self(s::NamePat::any())
    }

    /// Names of this entity kind.
    #[staticmethod]
    fn of_kind(kind: EntityKind) -> Self {
        Self(s::NamePat::of_kind(kind.to_kernel()))
    }

    /// Constrains the minting node.
    fn node(&self, node: &NodeId) -> Self {
        Self(self.0.clone().node(node.0))
    }

    /// Constrains the role path (exact length, segment-for-segment).
    fn path(&self, path: Vec<SegPat>) -> Self {
        Self(self.0.clone().path(path.into_iter().map(|p| p.0)))
    }

    /// Constrains the role path to ONE segment — the common case.
    fn seg(&self, seg: &SegPat) -> Self {
        Self(self.0.clone().seg(seg.0.clone()))
    }

    /// Whether a materialized name matches this pattern.
    ///
    /// `name` is the opaque text a materializer answered with; the
    /// BINDING reads it (the one licensed reader), your code never
    /// does. Text that is not a name at all is a `ValueError`.
    fn matches(&self, name: &str) -> PyResult<bool> {
        Ok(self.0.matches(&name_from_text(name)?))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

/// A selector: a UNION of name patterns, matched on name shape alone.
///
/// This is the STRUCTURAL half of the selector language (which op
/// minted the entity, which role it plays, which side). It narrows a
/// name table by SHAPE; geometry is `Evaluation.select_where`'s
/// second stage, never a pattern field.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct Selector(pub(crate) s::Selector);

#[pymethods]
impl Selector {
    /// The selector of exactly one pattern.
    #[staticmethod]
    fn of(pat: &NamePat) -> Self {
        Self(s::Selector::of(pat.0.clone()))
    }

    /// The union of these patterns. An EMPTY selector matches nothing.
    #[staticmethod]
    fn any_of(pats: Vec<NamePat>) -> Self {
        Self(s::Selector::any_of(pats.into_iter().map(|p| p.0)))
    }

    /// The union with one more pattern — Rust's `.or`; the trailing
    /// underscore is the `inch` precedent (`or` is a Python keyword).
    fn or_(&self, pat: &NamePat) -> Self {
        Self(self.0.clone().or(pat.0.clone()))
    }

    /// Whether a materialized name matches — see `NamePat.matches`
    /// for the one-licensed-reader contract on `name`.
    fn matches(&self, name: &str) -> PyResult<bool> {
        Ok(self.0.matches(&name_from_text(name)?))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

/// One geometric atom; a `list[GeomPred]` is their CONJUNCTION (run
/// `select_where` twice and concatenate for a union — the same
/// additive algebra as the structural union).
///
/// The atoms split EXACT/DECIDED, and the split is the whole design
/// (module docs above): `curve_kind` / `surface_kind` /
/// `adjacent_kinds` read the carrier's enum tag — total, no funnel,
/// cannot refuse. `datum_distance` is a real length comparison — a
/// `k_stats` funnel site whose in-band candidates refuse typed.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct GeomPred(pub(crate) s::GeomPred);

#[pymethods]
impl GeomPred {
    /// EXACT: the edge's certified carrier is one of these kinds.
    /// Non-edge names never match; an uncertified carrier is an
    /// honest no, not a refusal.
    #[staticmethod]
    fn curve_kind(kinds: CurveKinds) -> Self {
        Self(s::GeomPred::CurveKind(kinds.to_kernel()))
    }

    /// EXACT: the face's surface is one of these kinds. Non-face
    /// names never match.
    #[staticmethod]
    fn surface_kind(kinds: SurfaceKinds) -> Self {
        Self(s::GeomPred::SurfaceKind(kinds.to_kernel()))
    }

    /// EXACT: the two faces across an EDGE have kinds drawn one from
    /// each set — UNORDERED, so `(Plane, Sphere)` matches a rim
    /// whichever side carries which. Non-edge names never match.
    #[staticmethod]
    fn adjacent_kinds(a: SurfaceKinds, b: SurfaceKinds) -> Self {
        Self(s::GeomPred::AdjacentKinds(a.to_kernel(), b.to_kernel()))
    }

    /// DECIDED: the entity's distance to a datum node, compared
    /// against a stated `Length` — signed against a datum plane
    /// (along its normal), unsigned to an axis or point. The datum is
    /// a node reference like every other input, which is what keeps
    /// the rule equivariant: move the datum with the part and the
    /// selection commutes.
    #[staticmethod]
    fn datum_distance(py: Python<'_>, datum: &NodeId, cmp: Cmp, value: Length) -> PyResult<Self> {
        Ok(Self(s::GeomPred::DatumDistance {
            datum: datum.0,
            cmp: cmp.to_kernel(),
            value: literal(py, value.0.meters(), d::Dimension::Length)?,
        }))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

/// Raise `SelectRefusal` mirroring the kernel's refusal: `reason` is
/// the arm's stable tag (`crate::tags::select_refusal_tag`), and the
/// per-arm payload rides as attributes that are ALWAYS present —
/// `None` where inapplicable — so stub-guided reads cannot
/// `AttributeError` (the house rule from `ExportError`).
///
/// The human message is written per arm here rather than taken from
/// the kernel's `Display`, because a candidate is spelled through
/// `name_text` — the `StableName` alphabet Python speaks — where the
/// kernel spells it kind-plus-minting-node. The arms this binding does
/// not mirror fall back to that kernel prose. The fields are the
/// contract; the message is prose.
pub(crate) fn select_refusal(py: Python<'_>, err: &s::SelectRefusal) -> PyErr {
    use s::SelectRefusal as R;
    let reason = select_refusal_tag(err);
    let none = || py.None().into_any();
    let text = |v: &str| PyString::new(py, v).unbind().into_any();
    // `name` renders through `name_text` — the same alphabet every
    // other door speaks. A serialization failure surfaces as its own
    // raise rather than being swallowed.
    let mut fields: Vec<(&str, Py<PyAny>)> = vec![
        ("reason", text(reason)),
        ("name", none()),
        ("predicate", none()),
        ("matched", none()),
        ("candidates", none()),
        ("datum", none()),
        ("found", none()),
        ("dim", none()),
    ];
    let message = match err {
        R::InBand {
            name,
            predicate,
            source,
        } => {
            let name = match crate::py::doc::name_text(py, name) {
                Ok(t) => t,
                Err(failed) => return failed,
            };
            fields[1] = ("name", text(&name));
            fields[2] = ("predicate", text(predicate));
            format!(
                "a candidate's decided margin is inside the ambiguity band \
                 for `{predicate}` — neither side of the comparison is \
                 certified, so the query refuses rather than silently \
                 including or dropping it: {source}"
            )
        }
        R::TiedDisagrees {
            name,
            matched,
            candidates,
        } => {
            let name = match crate::py::doc::name_text(py, name) {
                Ok(t) => t,
                Err(failed) => return failed,
            };
            fields[1] = ("name", text(&name));
            // `usize` → Python int is infallible: the error type is
            // `Infallible`, discharged by matching the empty enum.
            let int_obj = |v: usize| -> Py<PyAny> {
                match v.into_pyobject(py) {
                    Ok(bound) => bound.unbind().into_any(),
                    Err(never) => match never {},
                }
            };
            fields[3] = ("matched", int_obj(*matched));
            fields[4] = ("candidates", int_obj(*candidates));
            format!(
                "a tied name's candidates disagree under the filter \
                 ({matched} of {candidates} match) — a name cannot be \
                 half-selected, so the query refuses"
            )
        }
        R::Unreadable { name, error } => {
            let name = match crate::py::doc::name_text(py, name) {
                Ok(t) => t,
                Err(failed) => return failed,
            };
            fields[1] = ("name", text(&name));
            format!(
                "a decided atom could not read a candidate's position \
                 (the read-back refusal, surfaced rather than swallowed): \
                 {error}"
            )
        }
        R::NotADatum { datum, found } => {
            let datum_obj = match NodeId(*datum).into_pyobject(py) {
                Ok(bound) => bound.unbind().into_any(),
                Err(failed) => return failed,
            };
            fields[5] = ("datum", datum_obj);
            fields[6] = ("found", text(found));
            format!(
                "the node `datum_distance` references is not an evaluated \
                 datum (found: {found})"
            )
        }
        R::NotALength { dim } => {
            fields[7] = ("dim", text(dimension_tag(*dim)));
            "the comparand of a distance must be a length".to_string()
        }
        R::PairInBand {
            pair,
            predicate,
            source,
        } => {
            let a = match crate::py::doc::name_text(py, &pair.0) {
                Ok(t) => t,
                Err(failed) => return failed,
            };
            fields[1] = ("name", text(&a));
            fields[2] = ("predicate", text(predicate));
            format!(
                "a candidate pair's verify-door margin is inside the \
                 ambiguity band for `{predicate}`: {source}"
            )
        }
        R::BadValue(inner) => format!("the stated value did not evaluate: {inner}"),
        R::Band => "the ambiguity band itself could not be built (a broken \
                    ambient tolerance)"
            .to_string(),
        // `SelectRefusal` is `#[non_exhaustive]`: a kernel arm this
        // binding does not know crosses with the kernel's own prose
        // and the `unclassified` tag rather than being dropped. That
        // typed crossing is the whole guard — no compile-time alarm is
        // possible here, and the tag pin in `src/tests.rs` enumerates
        // the arms this binding speaks without being able to fail on a
        // new one.
        other => other.to_string(),
    };
    typed_err(py, ErrorClass::Select, message, &fields)
}

/// **Kernel-growth tripwires**: one function per mirrored enum,
/// matching the KERNEL type with NO wildcard arm — so a kernel
/// variant this file does not mirror stops the `python`-feature build
/// here instead of silently never crossing. Never called; the
/// exhaustiveness IS the assertion (the `SegTag::of` precedent,
/// `editor-core/src/names/select.rs`).
///
/// **What it does not catch**, for the next widener: the tripwire sees
/// the KERNEL enum and this file's mirror, not `pncad.pyi`. A variant
/// RENAMED on both sides leaves the stub's attribute names stale with
/// nothing failing — the census compares top-level names, not an
/// enum's attributes. Issue #1309 owns that gap; until it closes, a
/// rename here is a manual edit of the stub.
#[allow(
    dead_code,
    reason = "compile-time exhaustiveness tripwires; the match is the check, no caller needed"
)]
mod growth_tripwire {
    use super::{
        CapEnd, Cmp, CurveKind, KSurfaceKind, MeridianEnd, OpGroup, RimSupport, SegTag, SideArg,
        SplitHalf, SurfaceKind, s,
    };

    // `EntityKind`'s tripwire is `super::entity_kind`, which is the
    // same exhaustive match with a caller — a mirror that CROSSES
    // needs no dead twin to assert what the crossing already asserts.

    fn seg_tag(k: s::SegTag) -> SegTag {
        match k {
            s::SegTag::OutputBody => SegTag::OutputBody,
            s::SegTag::Cap => SegTag::Cap,
            s::SegTag::Lateral => SegTag::Lateral,
            s::SegTag::RimEdge => SegTag::RimEdge,
            s::SegTag::LateralEdge => SegTag::LateralEdge,
            s::SegTag::CapVertex => SegTag::CapVertex,
            s::SegTag::Band => SegTag::Band,
            s::SegTag::BandRim => SegTag::BandRim,
            s::SegTag::BandRimPi => SegTag::BandRimPi,
            s::SegTag::BandPi => SegTag::BandPi,
            s::SegTag::Meridian => SegTag::Meridian,
            s::SegTag::MeridianVertex => SegTag::MeridianVertex,
            s::SegTag::RevolveCap => SegTag::RevolveCap,
            s::SegTag::Pole => SegTag::Pole,
            s::SegTag::AxisEdge => SegTag::AxisEdge,
            s::SegTag::FromA => SegTag::FromA,
            s::SegTag::FromB => SegTag::FromB,
            s::SegTag::Seam => SegTag::Seam,
            s::SegTag::Merged => SegTag::Merged,
            s::SegTag::Fragment => SegTag::Fragment,
            s::SegTag::SplitBody => SegTag::SplitBody,
            s::SegTag::SectionFace => SegTag::SectionFace,
            s::SegTag::SectionEdge => SegTag::SectionEdge,
            s::SegTag::SplitFragment => SegTag::SplitFragment,
            s::SegTag::CrossingVertex => SegTag::CrossingVertex,
            s::SegTag::OnToolVertex => SegTag::OnToolVertex,
            s::SegTag::FromTarget => SegTag::FromTarget,
            s::SegTag::BlendFace => SegTag::BlendFace,
            s::SegTag::CornerFace => SegTag::CornerFace,
            s::SegTag::TrimEdge => SegTag::TrimEdge,
            s::SegTag::FootVertex => SegTag::FootVertex,
            s::SegTag::CornerArc => SegTag::CornerArc,
            s::SegTag::BandFace => SegTag::BandFace,
            s::SegTag::BandTrim => SegTag::BandTrim,
            s::SegTag::BandFoot => SegTag::BandFoot,
            s::SegTag::BandCross => SegTag::BandCross,
            s::SegTag::BandCut => SegTag::BandCut,
            s::SegTag::BandSlit => SegTag::BandSlit,
            s::SegTag::Instance => SegTag::Instance,
            s::SegTag::InPart => SegTag::InPart,
        }
    }

    fn op_group(k: s::OpGroup) -> OpGroup {
        match k {
            s::OpGroup::Shared => OpGroup::Shared,
            s::OpGroup::Extrude => OpGroup::Extrude,
            s::OpGroup::Revolve => OpGroup::Revolve,
            s::OpGroup::Boolean => OpGroup::Boolean,
            s::OpGroup::Split => OpGroup::Split,
            s::OpGroup::Fillet => OpGroup::Fillet,
            s::OpGroup::Pattern => OpGroup::Pattern,
            s::OpGroup::InstantiatePart => OpGroup::InstantiatePart,
        }
    }

    fn side(k: s::Side) -> SideArg {
        match k {
            s::Side::Cap(s::CapEnd::Top) => SideArg::Cap(CapEnd::Top),
            s::Side::Cap(s::CapEnd::Bottom) => SideArg::Cap(CapEnd::Bottom),
            s::Side::Meridian(s::MeridianEnd::Start) => SideArg::Meridian(MeridianEnd::Start),
            s::Side::Meridian(s::MeridianEnd::End) => SideArg::Meridian(MeridianEnd::End),
            s::Side::Meridian(s::MeridianEnd::Seam) => SideArg::Meridian(MeridianEnd::Seam),
            s::Side::Meridian(s::MeridianEnd::Pi) => SideArg::Meridian(MeridianEnd::Pi),
            s::Side::Split(s::SplitHalf::Above) => SideArg::Split(SplitHalf::Above),
            s::Side::Split(s::SplitHalf::Below) => SideArg::Split(SplitHalf::Below),
            s::Side::Rim(s::RimSupport::Host) => SideArg::Rim(RimSupport::Host),
            s::Side::Rim(s::RimSupport::Mate) => SideArg::Rim(RimSupport::Mate),
        }
    }

    fn curve_kind(k: s::CurveKind) -> CurveKind {
        match k {
            s::CurveKind::Line => CurveKind::Line,
            s::CurveKind::Circle => CurveKind::Circle,
            s::CurveKind::Ellipse => CurveKind::Ellipse,
            s::CurveKind::Nurbs => CurveKind::Nurbs,
        }
    }

    fn surface_kind(k: KSurfaceKind) -> SurfaceKind {
        match k {
            KSurfaceKind::Plane => SurfaceKind::Plane,
            KSurfaceKind::Cylinder => SurfaceKind::Cylinder,
            KSurfaceKind::Cone => SurfaceKind::Cone,
            KSurfaceKind::Sphere => SurfaceKind::Sphere,
            KSurfaceKind::Torus => SurfaceKind::Torus,
            KSurfaceKind::Nurbs => SurfaceKind::Nurbs,
            KSurfaceKind::Approx => SurfaceKind::Approx,
        }
    }

    fn cmp(k: s::Cmp) -> Cmp {
        match k {
            s::Cmp::Approx => Cmp::Approx,
            s::Cmp::Greater => Cmp::Greater,
            s::Cmp::Less => Cmp::Less,
        }
    }
}

/// Register the selector surface on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<EntityKind>()?;
    m.add_class::<SegTag>()?;
    m.add_class::<OpGroup>()?;
    m.add_class::<CapEnd>()?;
    m.add_class::<MeridianEnd>()?;
    m.add_class::<SplitHalf>()?;
    m.add_class::<RimSupport>()?;
    m.add_class::<CurveKind>()?;
    m.add_class::<SurfaceKind>()?;
    m.add_class::<Cmp>()?;
    m.add_class::<SegPat>()?;
    m.add_class::<NamePat>()?;
    m.add_class::<Selector>()?;
    m.add_class::<GeomPred>()?;
    Ok(())
}
