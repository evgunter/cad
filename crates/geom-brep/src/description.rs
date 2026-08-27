//! U2's **collapsed edge description** (`docs/PCURVE-UNIFY-DESIGN.md`,
//! ratified 2026-08-15): the form `geom-brep` certifies and stores,
//! in which every conventional edge is ONE thing — a chart surface
//! and the edge's image in that chart.
//!
//! # The taxonomy, after the collapse
//!
//! [`EdgeDescription`] has four arms and exactly ONE conventional
//! form:
//!
//! - the two INTRINSIC arms ([`EdgeDescription::Intersection`],
//!   [`EdgeDescription::TangentIntersection`]) are D2's, untouched —
//!   the surfaces determine the locus and the description names them;
//! - [`EdgeDescription::Chart`] is U2's one conventional form,
//!   `(surface, Pcurve)`. `Seam` and `IsoCurve` were not two classes
//!   of locus; they were two ways of writing a chart image down, and
//!   the certification lanes that DO differ survive where they belong
//!   — as [`Pcurve`]'s exactness variants;
//! - [`EdgeDescription::Scaffold`] is the fenced scaffolding door
//!   (D3, Evan 2026-08-27): a [`MappedCurve`] is legal as a
//!   description only while TRANSIENT. Tier 3 refuses it at rest.
//!
//! # The certified statement (D1)
//!
//! One meter: `|C(t) − S(P(t))| ≤ ε`, C4 verbatim, at the
//! certification schedule and on the carrier's OWN parameter (the
//! parameter [`Pcurve`] is defined on). The two seam predicates —
//! half-plane and side — are not a second form; they are an
//! **obligation carried by a chart image that claims to BE a periodic
//! chart's seam** ([`ChartCurve::seam`]), metered beside the one
//! meter and never instead of it.
//!
//! The alternative that was rejected: keeping `Seam` a peer variant.
//! It is cheaper and it forfeits the headline — one conventional
//! form — the migration exists for.
//!
//! # The authority record (U2 Q3)
//!
//! [`EdgeAuthority`] is the per-edge kernel datum that replaces
//! `MappedCurve`'s NEGATIVE SPACE. Tier 3's prefer-intrinsic rules
//! (`topo`'s `TransverseNotIntrinsic` / `TangentNotIntrinsic`) today
//! read "the description is a `MappedCurve`" as "the modeler declared
//! this locus"; once the conventional forms collapse there is no such
//! shape to read, so the declaration becomes recorded data. Sketch
//! truth is recorded, not derived.

use geom_core::Real;

use crate::edge_geometry::{EdgeGeometry, MappedCurve};
use crate::keys::SurfaceKey;
use crate::pcurve_cache::Pcurve;

/// U2's ONE conventional description form: a chart surface and the
/// edge's image in that chart (module docs).
///
/// The certified statement is the unified meter `|C(t) − S(P(t))| ≤ ε`
/// (D1). `pcurve` is on the carrier's own parameter — the same
/// contract [`Pcurve`] carries everywhere else, so a description's
/// image and a stored cache's image are the same object under the
/// same parameterization, which is what makes the two comparable at
/// all.
#[derive(Clone, Debug)]
pub struct ChartCurve<T: Real> {
    /// The chart the edge's locus is described IN (a body-arena key,
    /// lineage-scoped per Q1 — see [`crate::keys`]).
    pub surface: SurfaceKey,
    /// The edge's image in that chart, on the carrier's parameter.
    pub pcurve: Pcurve<T>,
    /// D1's obligation flag: this edge claims to BE the chart's
    /// parameterization seam (the `u_ref`-half-plane meridian), so the
    /// two seam predicates are metered beside the one meter. False on
    /// every other chart image — an iso boundary, a cap rim, a planar
    /// chord — which owe the meter and nothing else.
    ///
    /// It is a claim about THIS EDGE, not about the surface: a
    /// periodic chart has exactly one seam meridian and any number of
    /// other edges.
    pub seam: bool,
}

/// The **authority record** (U2 Q3): who determined this edge's
/// locus. Per-edge kernel data, because tier 3's prefer-intrinsic
/// enforcement must read it and the naming layer is invisible to the
/// kernel.
///
/// `Copy` — [`MappedCurve`] is plain sketch data with no heap payload,
/// so the record costs nothing to carry.
#[derive(Clone, Copy, Debug)]
pub enum EdgeAuthority<T: Real> {
    /// Nothing was declared: the locus is whatever the geometry makes
    /// it. Every intrinsic description, and every conventional chart
    /// image the kernel derived for itself (seams, iso boundaries,
    /// cap rims).
    Derived,
    /// The locus was **declared** by a sketch entity under a sweep map
    /// — the pushforward `MappedCurve` used to say by BEING the
    /// description. The source is kept whole: provenance recorded, not
    /// re-derived.
    Declared(MappedCurve<T>),
}

impl<T: Real> EdgeAuthority<T> {
    /// Whether a modeler DECLARED this locus — the read that replaces
    /// `matches!(description, EdgeGeometry::MappedCurve(_))` at tier
    /// 3's prefer-intrinsic rules.
    pub fn is_declared(&self) -> bool {
        matches!(*self, EdgeAuthority::Declared(_))
    }
}

/// The canonical edge description (module docs): D2's two intrinsic
/// arms, U2's ONE conventional form, and the fenced scaffolding door.
///
/// **Not `Copy`** — [`Pcurve`] carries heap payloads on two of its
/// four variants. See [`crate::EdgeCurve::canonical`] for how the
/// certified product hands it out (by reference, never by copy: an
/// edge description is read, not moved around).
#[derive(Clone, Debug)]
pub enum EdgeDescription<T: Real> {
    /// Intrinsic: the transverse intersection component selected by
    /// `witness` (D2, verbatim from [`EdgeGeometry::Intersection`]).
    Intersection {
        /// The first surface.
        s1: SurfaceKey,
        /// The second surface.
        s2: SurfaceKey,
        /// The mid-parameter witness point.
        witness: geom_core::Point3<T>,
    },
    /// Intrinsic, one differential order up: the tangential contact
    /// component selected by `witness` (D2 as sharpened by OQ7).
    TangentIntersection {
        /// The first surface.
        s1: SurfaceKey,
        /// The second surface.
        s2: SurfaceKey,
        /// The mid-parameter witness point.
        witness: geom_core::Point3<T>,
    },
    /// **Conventional — the one form** (U2): a chart and the edge's
    /// image in it.
    Chart(ChartCurve<T>),
    /// The fenced scaffolding door (D3): a pushforward standing in as
    /// a description while the edge is TRANSIENT — Euler-op ring
    /// scaffolding whose surfaces do not exist yet. Legal here, and
    /// refused by tier 3 at rest.
    Scaffold(MappedCurve<T>),
}

impl<T: Real> EdgeDescription<T> {
    /// The chart image, when this description has one — the accessor
    /// consumers read instead of matching two conventional variants.
    pub fn chart(&self) -> Option<&ChartCurve<T>> {
        match self {
            EdgeDescription::Chart(c) => Some(c),
            _ => None,
        }
    }
}

/// The **shim** view of a description, in the pre-collapse vocabulary.
///
/// This exists so P-1a is a `geom-brep`-only diff: the six consumer
/// crates keep reading [`EdgeGeometry`] and P-1b moves them onto
/// [`EdgeDescription`] as its own reviewable change. It is the only
/// place the two vocabularies meet, which is what makes the crate
/// boundary checkable — delete this function and the shim is gone.
///
/// The authority record is read off the same shim: a `MappedCurve`
/// description IS a declaration, which is exactly the negative space
/// tier 3 reads today.
pub fn authority_of<T: Real>(description: &EdgeGeometry<T>) -> EdgeAuthority<T> {
    match *description {
        EdgeGeometry::MappedCurve(mc) => EdgeAuthority::Declared(mc),
        EdgeGeometry::Intersection { .. }
        | EdgeGeometry::TangentIntersection { .. }
        | EdgeGeometry::Seam { .. }
        | EdgeGeometry::IsoCurve { .. } => EdgeAuthority::Derived,
    }
}
