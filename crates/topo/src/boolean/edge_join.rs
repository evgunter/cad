//! **The join**: a vertex of valence 2 whose two edges lie on one
//! structural carrier between the same two faces is no corner, so the
//! two edges are one edge cut for no reason (`docs/DESIGN.md`, maximal
//! edges). The join kills the vertex and makes the two edges one.
//!
//! **The join door** ([`Body::join_edges`]) is the one public spelling:
//! every door that finishes a body ends with it (`docs/DESIGN.md`, the
//! merge stage), and it returns each join it made ([`EdgeJoin`]) for a
//! door that carries records or names keyed by the body's cells. The
//! door is whole: a reading in the band refuses before any kill, the
//! kills run on a staged clone, and a body that carried pcurve rows has
//! them re-derived, so no row is left keyed by a dead cell. A door that
//! is not the boolean carries its refusal typed ([`JoinRefusal`]).
//!
//! The boolean's output stages run it after the merge ([`join_stage`])
//! and write, per join, the substitution rows that carry every contact
//! record naming the cells it replaces onto the edge it makes, through
//! the op's one substitution door ([`super::ops::carry`]).
//!
//! Joinable is one predicate ([`joinable`]), shared by the join and by
//! [`joinable_vertices`]. The two edges' face pairs are one pair of
//! distinct faces, and the edges lie on one carrier by structure:
//! - **planar**: both faces on `Plane` carriers of distinct keys, both
//!   edges certified lines described as the `Intersection` of those
//!   keys. Two planes that cross meet in one line; two that do not
//!   certify no intersection, so coplanar faces sharing a bent boundary
//!   are never joined.
//! - **a locus of the surface pair**: both edges `Intersection`, or
//!   both `TangentIntersection`, citing the faces' two surface keys, and
//!   the locus one regular curve at the vertex: the surfaces definitely
//!   transverse there, or, for the tangent arm, definitely determined
//!   one order up (`tangent_second_order`).
//! - **an iso family of one chart**: both faces on one surface key,
//!   both edges `Chart` images in it on one iso family, and the chart
//!   regular at the vertex.
//!
//! Distinct components of one locus never share a point, and one iso
//! family has one member through a regular point, so the vertex decides
//! the branch and nothing is compared between the two edges. A surface
//! singularity (a cone's apex) or a chart's (a sphere's pole) is never
//! joinable; a reading in the margin band refuses typed
//! ([`JoinUndecided`]).
//!
//! A vertex that shares its point key with another vertex is not
//! joinable either: it is one cone of a pinch (Ev, PR 4057), and the
//! joined edge would run through the other cone's vertex on that point,
//! a vertex-on-edge contact no record names.
//!
//! **The closed join.** Two edges that share both ends join into one
//! closed edge: the kill leaves the kept edge a self-loop at the far
//! vertex. Where no other edge ends there, that vertex is the edge's
//! *conventional vertex* (DESIGN.md, maximal edges): a vertex whose only
//! edge is one closed edge, at both its ends ([`is_conventional_vertex`]).
//! It has no identity of its own, so the join substitutes it, as well as
//! the killed vertex, onto the kept edge.

use std::collections::{BTreeMap, BTreeSet};

use geom_core::{Band, Decide, Indeterminate, Margin, Real, Sign, Tol};

use super::ops::{Descendants, describe_minted_edges};
use super::{BooleanError, Cell};
use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, HalfEdgeKey, VertexKey};
use crate::geometry::PointKey;
use crate::live::{linked, proven};
use crate::readback::edge_sides_of;

/// A joinable vertex's two edges: `gone` dies with the vertex, `kept`
/// becomes the joined edge; `he` is `gone`'s half-edge that ends at the
/// vertex, the one `kev` kills it through. `far` is `gone`'s other end;
/// where it is `kept`'s other end too the join is closed.
struct Join {
    he: HalfEdgeKey,
    gone: EdgeKey,
    kept: EdgeKey,
    far: VertexKey,
    closed: bool,
    planar: bool,
}

/// One join an output stage made ([`join_stage`]): `vertex` and `gone`
/// are dead, and `kept` holds both their interiors and its own.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EdgeJoin {
    /// The joined-away vertex.
    pub vertex: VertexKey,
    /// The edge killed with it.
    pub gone: EdgeKey,
    /// The edge that is the two edges now.
    pub kept: EdgeKey,
    /// Where the join closed `kept` into a self-loop that is the
    /// surviving vertex's only edge: that vertex, now conventional.
    pub conventional: Option<VertexKey>,
}

/// **A joinable reading the band cannot decide**: whether `vertex` is
/// a regular point of the carrier its two edges lie on, or which iso
/// family an edge's chart image is on, read inside the margin band, so
/// the vertex is neither joined nor left (D4 ¶3).
#[derive(Clone, Debug, PartialEq)]
pub struct JoinUndecided {
    /// The valence-2 vertex.
    pub vertex: VertexKey,
    /// The reading that escalated.
    pub reading: JoinReading,
}

/// Which reading of [`JoinUndecided`] escalated.
#[derive(Clone, Debug, PartialEq)]
pub enum JoinReading {
    /// A regularity margin at the vertex: its distance from a surface
    /// or chart singularity, the pair's wedge, or the tangent arm's
    /// second order.
    Regularity(Indeterminate),
    /// The class arm of an edge's chart image.
    ChartClass(geom_brep::IsoFamilyRefusal),
}

impl core::fmt::Display for JoinUndecided {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "whether valence-2 vertex {:?} joins its two edges is undecided: ",
            self.vertex
        )?;
        match &self.reading {
            JoinReading::Regularity(diag) => write!(f, "{diag}"),
            JoinReading::ChartClass(e) => write!(f, "{e}"),
        }
    }
}

/// `e`'s certified curve, where it is a line: the one home of "a
/// certified line edge", read by the join and by the carried-record
/// door, whose edge-edge lineage reads an edge as its two ends.
pub(super) fn certified_line<'a, T: Real>(
    body: &'a Body<T>,
    e: EdgeKey,
    d: &crate::entity::Edge,
) -> Option<&'a geom_brep::EdgeCurve<T>> {
    body.edge_curve_linked(e, d)
        .certified()
        .filter(|c| matches!(c.carrier(), geom::Curve3::Line { .. }))
}

/// Whether `v` is a **conventional vertex**: its only edge is one
/// closed edge, at both its ends. Defined by structure, so the tier-2
/// check and every reader that skips such a vertex read the same set.
/// A sweep's closed rim meets its seam strut there and stays outside.
pub fn is_conventional_vertex<T: Real>(body: &Body<T>, v: VertexKey) -> bool {
    let Some(vertex) = body.get_vertex(v) else {
        return false;
    };
    let Some(first) = vertex.emanating else {
        return false;
    };
    let Some(he) = body.get_half_edge(first) else {
        return false;
    };
    let Some(edge) = body.get_edge(he.edge) else {
        return false;
    };
    let ends = [edge.he_plus, edge.he_minus].map(|h| body.get_half_edge(h).map(|h| h.start));
    ends == [Some(v), Some(v)]
        && body.vertex_orbit(first).is_some_and(|orbit| {
            orbit
                .iter()
                .all(|&h| body.get_half_edge(h).is_some_and(|d| d.edge == he.edge))
        })
}

/// What one pass reads off the body before it asks any vertex: every
/// half-edge starting at each vertex, and the point keys more than one
/// vertex sits on (a pinch's cones).
struct Pass {
    starts: BTreeMap<VertexKey, Vec<HalfEdgeKey>>,
    shared: BTreeSet<PointKey>,
}

impl Pass {
    fn of<T: Real>(body: &Body<T>) -> Self {
        let mut starts: BTreeMap<VertexKey, Vec<HalfEdgeKey>> = BTreeMap::new();
        for (k, h) in body.half_edges() {
            starts.entry(h.start).or_default().push(k);
        }
        let mut seen = BTreeSet::new();
        let mut shared = BTreeSet::new();
        for (_, v) in body.vertices() {
            if !seen.insert(v.point) {
                shared.insert(v.point);
            }
        }
        Self { starts, shared }
    }
}

/// The structural carrier a curved edge's description names on a face
/// pair: a locus of the surface pair (transverse or tangent), or an iso
/// family of one surface's chart.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Locus {
    Transverse,
    Tangent,
    Iso(geom_brep::IsoFamily),
}

/// Whether `s`'s chart is singular at its point `p`: a cone's apex, a
/// sphere's poles, a torus's points on its axis — where the `u`
/// coordinate's lever arm (the distance from the apex or the axis)
/// collapses. A pole is never joinable, whichever arm the edges are
/// on: no chart image runs one branch through it.
pub(super) fn singular_at<T: Decide>(
    s: &geom::Surface<T>,
    p: geom_core::Point3<T>,
    band: Band,
) -> Result<bool, Indeterminate> {
    let off_axis = |origin: geom_core::Point3<T>, axis: geom_core::Vec3<T>| {
        let w = p - origin;
        (w - axis * w.dot(axis)).norm()
    };
    let arm = match *s {
        geom::Surface::Cone { apex, .. } => (p - apex).norm(),
        geom::Surface::Sphere { center, axis, .. } => off_axis(center, axis),
        geom::Surface::Torus { center, axis, .. } => off_axis(center, axis),
        _ => return Ok(false),
    };
    geom_core::k_stats::decide("join_regular_point", Margin::of(arm), band)
        .map(|sign| sign != Sign::Positive)
}

/// `w`'s join, if `w` is joinable (module docs). The half-edges in
/// `pass` were read out of the arena, so every hop past them is a link.
///
/// # Errors
///
/// [`JoinUndecided`] where a regularity reading at `w` lands in the
/// margin band.
fn joinable<T: Decide>(
    body: &Body<T>,
    w: VertexKey,
    pass: &Pass,
    band: Band,
) -> Result<Option<Join>, JoinUndecided> {
    let [h1, h2] = pass.starts.get(&w).map_or(&[][..], Vec::as_slice) else {
        return Ok(None);
    };
    let w_point = proven(&body.vertices, w, EntityId::Vertex).point;
    // One cone of a pinch: the joined edge would run through another
    // vertex on this point (module docs).
    if pass.shared.contains(&w_point) {
        return Ok(None);
    }
    let edge = |h: HalfEdgeKey| proven(&body.half_edges, h, EntityId::HalfEdge).edge;
    let (e1, e2) = (edge(*h1), edge(*h2));
    if e1 == e2 {
        return Ok(None);
    }
    let d1 = linked(
        &body.edges,
        e1,
        EntityId::Edge,
        EntityId::HalfEdge(*h1),
        "edge",
    );
    let d2 = linked(
        &body.edges,
        e2,
        EntityId::Edge,
        EntityId::HalfEdge(*h2),
        "edge",
    );
    let (s1, s2) = (edge_sides_of(body, e1, d1), edge_sides_of(body, e2, d2));
    let pair = |s: crate::readback::EdgeSides| {
        let (f, g) = s.faces();
        if f <= g { (f, g) } else { (g, f) }
    };
    let (f, g) = pair(s1);
    if f == g || pair(s2) != (f, g) {
        return Ok(None);
    }
    let Some(he) = body.mate(*h1) else {
        return Ok(None);
    };
    let far_of = |h: HalfEdgeKey| {
        let m = body.mate(h)?;
        Some(proven(&body.half_edges, m, EntityId::HalfEdge).start)
    };
    let (Some(far), Some(far2)) = (far_of(*h1), far_of(*h2)) else {
        return Ok(None);
    };
    let join = |planar| Join {
        he,
        gone: e1,
        kept: e2,
        far,
        closed: far == far2,
        planar,
    };
    let (sf, sg) = s1.surfaces();
    let surface = |side: crate::readback::EdgeSide| {
        let face = proven(&body.faces, side.face, EntityId::Face);
        body.face_surface_linked(side.face, face)
    };
    let (surf_f, surf_g) = (surface(s1.plus), surface(s1.minus));
    let planar = |s: &geom::Surface<T>| matches!(s, geom::Surface::Plane { .. });
    if planar(surf_f) && planar(surf_g) {
        // One carrier by structure: both edges certified as the
        // intersection of these two planes, which is one line (planes
        // that do not cross certify no intersection).
        let on_the_pair = |e: EdgeKey, d| {
            certified_line(body, e, d).is_some_and(|c| {
                matches!(
                    c.description(),
                    geom_brep::EdgeDescription::Intersection { s1, s2, .. }
                        if Body::<T>::cites_pair((*s1, *s2), sf, sg)
                )
            })
        };
        let joins = sf != sg && on_the_pair(e1, d1) && on_the_pair(e2, d2);
        return Ok(joins.then(|| join(true)));
    }
    let undecided = |reading| JoinUndecided { vertex: w, reading };
    let locus_of = |e: EdgeKey, d| {
        let Some(c) = body.edge_curve_linked(e, d).certified() else {
            return Ok(None);
        };
        Ok(match c.description() {
            geom_brep::EdgeDescription::Intersection { s1, s2, .. }
                if sf != sg && Body::<T>::cites_pair((*s1, *s2), sf, sg) =>
            {
                Some(Locus::Transverse)
            }
            geom_brep::EdgeDescription::TangentIntersection { s1, s2, .. }
                if sf != sg && Body::<T>::cites_pair((*s1, *s2), sf, sg) =>
            {
                Some(Locus::Tangent)
            }
            geom_brep::EdgeDescription::Chart(chart) if sf == sg && chart.surface == sf => {
                geom_brep::chart_iso_family(c.carrier(), surf_f, band)
                    .map_err(|e| undecided(JoinReading::ChartClass(e)))?
                    .map(Locus::Iso)
            }
            _ => None,
        })
    };
    let Some(locus) = locus_of(e1, d1)? else {
        return Ok(None);
    };
    if locus_of(e2, d2)? != Some(locus) {
        return Ok(None);
    }
    let regularity = |diag| undecided(JoinReading::Regularity(diag));
    let p = body.linked_vertex_point(w, EntityId::HalfEdge(*h1), "start");
    for s in [surf_f, surf_g] {
        if singular_at(s, p, band).map_err(regularity)? {
            return Ok(None);
        }
    }
    if matches!(locus, Locus::Iso(_)) {
        return Ok(Some(join(false)));
    }
    // The pair's reading at `w`, levered over the shorter edge's extent,
    // so it does not depend on which edge the arena lists first.
    let extent = [(e1, d1), (e2, d2)]
        .into_iter()
        .filter_map(|(e, d)| {
            let c = body.edge_curve_linked(e, d).certified()?;
            let (t0, t1) = c.params();
            let chord = c.carrier().eval(t0).distance(c.carrier().eval(t1));
            Some(geom_brep::edge_extent(c.carrier(), t0, t1, chord))
        })
        .reduce(T::min);
    let Some(extent) = extent else {
        return Ok(None);
    };
    let class = geom_brep::classify_dihedral(surf_f, surf_g, p, extent, band)
        .map_err(|e| regularity(e.diag))?;
    let regular = match (locus, class) {
        (Locus::Transverse, geom_brep::DihedralClass::Transverse) => true,
        (Locus::Tangent, geom_brep::DihedralClass::Smooth) => {
            let Some(tangent) = tangent_at(body, e2, d2, w) else {
                return Ok(None);
            };
            let verdict =
                geom_brep::tangent_second_order(surf_f, surf_g, p, tangent, extent, band).verdict;
            verdict.map_err(regularity)?.sign == Sign::Positive
        }
        _ => false,
    };
    Ok(regular.then(|| join(false)))
}

/// `e`'s carrier tangent at its end `w`.
fn tangent_at<T: Decide>(
    body: &Body<T>,
    e: EdgeKey,
    d: &crate::entity::Edge,
    w: VertexKey,
) -> Option<geom_core::Vec3<T>> {
    let c = body.edge_curve_linked(e, d).certified()?;
    let (t0, t1) = c.params();
    let start = body.get_half_edge(d.he_plus)?.start;
    Some(c.carrier().ders1(if start == w { t0 } else { t1 }).1)
}

/// Every joinable vertex of `body`, in vertex-arena order: the vertices
/// an op's output must not hold (maximal edges), read through the one
/// predicate the join takes them by.
///
/// # Errors
///
/// [`JoinUndecided`] for the first vertex, in arena order, whose
/// reading lands in `band`.
pub fn joinable_vertices<T: Decide>(
    body: &Body<T>,
    band: Band,
) -> Result<Vec<VertexKey>, JoinUndecided> {
    let pass = Pass::of(body);
    let mut out = Vec::new();
    for (w, _) in body.vertices() {
        if joinable(body, w, &pass, band)?.is_some() {
            out.push(w);
        }
    }
    Ok(out)
}

/// Joins every joinable vertex of `body` ([`Body::join_edges`]) and
/// writes each join's substitution rows into `desc` (`w → kept`,
/// `gone → kept`, and where the join leaves a conventional vertex, that
/// vertex `→ kept`), so the op's one [`super::ops::carry`] takes every
/// record through its zips, its merge and its joins together. Runs
/// after the merge and its re-description, before the records are
/// carried.
///
/// # Errors
///
/// As [`Body::join_edges`].
pub(super) fn join_stage<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    desc: &mut Descendants,
    band: Band,
    tol: Tol,
) -> Result<Vec<EdgeJoin>, BooleanError> {
    let joined = body.join_edges(band, tol)?;
    for j in &joined {
        desc.substitute(Cell::Vertex(j.vertex), Cell::Edge(j.kept));
        desc.substitute(Cell::Edge(j.gone), Cell::Edge(j.kept));
        if let Some(v) = j.conventional {
            desc.substitute(Cell::Vertex(v), Cell::Edge(j.kept));
        }
    }
    Ok(joined)
}

/// **Why the join refused**, as a door that is not the boolean carries
/// it ([`crate::MergeCoplanarError::Join`],
/// [`crate::ReplaceFaceError::Join`]): typed, keyless, and worded with
/// one recourse.
#[derive(Clone, Debug, PartialEq)]
pub enum JoinRefusal {
    /// A regularity or chart-class reading at a vertex landed in the
    /// band ([`BooleanError::JoinUndecided`]).
    Undecided(JoinUndecided),
    /// The joined edge's carrier has no period (a closed join) or no
    /// parameter inverse (a spline) to run it on
    /// ([`BooleanError::JoinCarrierUnsupported`]).
    CarrierUnsupported {
        /// The carrier's kind.
        carrier: geom::CurveKind,
        /// Whether the join closes.
        closed: bool,
    },
    /// The kill, the re-description or the re-mint refused on a body
    /// that passed tier 2: a kernel or file defect, by the refusal's
    /// kind.
    Kernel {
        /// The boolean refusal's kind.
        kind: super::BooleanErrorKind,
    },
}

/// A refusal kind as its variant's name.
fn kind_word(kind: super::BooleanErrorKind) -> String {
    format!("{kind:?}")
}

impl JoinRefusal {
    /// The join door's refusal, typed for a door that is not the
    /// boolean.
    #[must_use]
    pub fn of(refusal: &BooleanError) -> Self {
        match refusal {
            BooleanError::JoinUndecided(e) => Self::Undecided(e.clone()),
            BooleanError::JoinCarrierUnsupported {
                carrier, closed, ..
            } => Self::CarrierUnsupported {
                carrier: *carrier,
                closed: *closed,
            },
            other => Self::Kernel { kind: other.kind() },
        }
    }
}

impl core::fmt::Display for JoinRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let undecided = |f: &mut core::fmt::Formatter<'_>, diag: &Indeterminate| {
            write!(
                f,
                "whether two edges meeting at a vertex on one curve are one edge is undecided \
                 ({}). {}",
                diag.payload(),
                diag.ending("move the geometry")
            )
        };
        match self {
            Self::Undecided(JoinUndecided {
                reading: JoinReading::Regularity(diag),
                ..
            })
            | Self::Undecided(JoinUndecided {
                reading: JoinReading::ChartClass(geom_brep::IsoFamilyRefusal::Undecided(diag)),
                ..
            }) => undecided(f, diag),
            Self::Undecided(JoinUndecided {
                reading: JoinReading::ChartClass(geom_brep::IsoFamilyRefusal::Image(e)),
                ..
            }) => write!(
                f,
                "two edges meeting at a vertex on one surface have no chart image to read \
                 their family off ({e}). {}",
                geom_core::KERNEL_OR_FILE_DEFECT_ENDING
            ),
            Self::CarrierUnsupported { carrier, closed } => write!(
                f,
                "two edges meet at a vertex on one {} carrier, which the join cannot run an \
                 edge on {} yet (work/fuse/joining-a-spline-carrier-is-unbuilt); there is no \
                 way through this yet",
                carrier.name(),
                if *closed {
                    "over a whole period"
                } else {
                    "through the vertex"
                }
            ),
            Self::Kernel { kind } => write!(
                f,
                "the join of two edges meeting at a vertex on one curve refused ({}). {}",
                kind_word(*kind),
                geom_core::KERNEL_OR_FILE_DEFECT_ENDING
            ),
        }
    }
}

impl<T: Decide + crate::props::AtRestPolicy> Body<T> {
    /// **The join** (`docs/DESIGN.md`, maximal edges): every joinable
    /// vertex ([`joinable_vertices`]) of the body, one at a time in
    /// vertex-arena order until none is left, killed and its two edges
    /// made one. The door every finisher ends with, so no body reaches
    /// rest holding a vertex the predicate would join
    /// (`ValidationError::JoinableVertexAtRest`). Returns the joins in
    /// the order made, a later one's `gone` or `kept` possibly an
    /// earlier one's `kept`: a door carrying records or names keyed by
    /// the body's cells reads each join as `vertex → kept`,
    /// `gone → kept`, and `conventional → kept` where it is set.
    ///
    /// # Errors
    ///
    /// [`BooleanError::JoinUndecided`], the kill's own refusal
    /// ([`BooleanError::Euler`]), a carrier the joined edge cannot be
    /// restated on ([`BooleanError::JoinCarrierUnsupported`]), or the
    /// description's.
    pub fn join_edges(&mut self, band: Band, tol: Tol) -> Result<Vec<EdgeJoin>, BooleanError> {
        // Nothing to join: no clone. Every reading in the band refuses
        // here, before any kill.
        if joinable_vertices(self, band)
            .map_err(BooleanError::JoinUndecided)?
            .is_empty()
        {
            return Ok(Vec::new());
        }
        // Staged: a refusal past the first kill leaves `self` as found.
        let mut work = self.clone();
        let joined = join_all(&mut work, band, tol)?;
        // A kill leaves the rows keyed by its half-edges; a body that
        // carried rows has them re-derived whole (`pcurves::mint_pcurves`
        // clears the map first), so none is left keyed by a dead cell.
        if !work.pcurves.is_empty() || !work.joints.is_empty() {
            crate::pcurves::mint_pcurves(&mut work, tol)
                .map_err(|source| BooleanError::Pcurves { source })?;
        }
        self.adopt(work);
        Ok(joined)
    }
}

/// [`Body::join_edges`]' loop, on the staged body.
fn join_all<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    band: Band,
    tol: Tol,
) -> Result<Vec<EdgeJoin>, BooleanError> {
    let mut joined = Vec::new();
    loop {
        let pass = Pass::of(body);
        let mut next = None;
        for (w, _) in body.vertices() {
            if let Some(j) = joinable(body, w, &pass, band).map_err(BooleanError::JoinUndecided)? {
                next = Some((w, j));
                break;
            }
        }
        let Some((w, join)) = next else {
            return Ok(joined);
        };
        join_one(body, w, &join, band, tol)?;
        // A closed join's survivor is conventional unless another edge
        // still ends there (a seam strut on the rim it closed).
        let conventional =
            (join.closed && is_conventional_vertex(body, join.far)).then_some(join.far);
        joined.push(EdgeJoin {
            vertex: w,
            gone: join.gone,
            kept: join.kept,
            conventional,
        });
    }
}

/// The joined edge's spec: `kept`'s carrier over its interval run on
/// through `w` to `gone`'s far end, a whole period where the join
/// closes, its description restated over the new interval.
fn joined_spec<T: Decide>(
    body: &Body<T>,
    w: VertexKey,
    join: &Join,
) -> Result<geom_brep::EdgeCurveSpec<T>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let curve = |e: EdgeKey| {
        let d = body.get_edge(e)?;
        Some((body.edge_curve_linked(e, d).certified()?, d))
    };
    let ((kept, kd), (gone, _)) = curve(join.kept)
        .zip(curve(join.gone))
        .ok_or(desync("a joined edge is not certified"))?;
    let carrier = kept.carrier();
    let (t0, t1) = kept.params();
    let backward = body.get_half_edge(kd.he_plus).map(|h| h.start) == Some(w);
    let (g0, g1) = gone.params();
    let far = *body
        .get_vertex(join.far)
        .and_then(|v| body.get_point(v.point))
        .ok_or(desync("a joined edge's far end does not resolve"))?;
    let tw = if backward { t0 } else { t1 };
    let tx = if join.closed {
        let period = T::from_f64(core::f64::consts::TAU);
        match carrier {
            geom::Curve3::Circle { .. }
            | geom::Curve3::Ellipse { .. }
            | geom::Curve3::Spiric { .. } => {
                if backward {
                    t1 - period
                } else {
                    t0 + period
                }
            }
            _ => {
                return Err(BooleanError::JoinCarrierUnsupported {
                    edge: join.kept,
                    carrier: carrier.kind(),
                    closed: true,
                });
            }
        }
    } else {
        carrier
            .param_near(gone.carrier().mid_point(g0, g1), tw)
            .and_then(|tm| carrier.param_near(far, tm))
            .ok_or(BooleanError::JoinCarrierUnsupported {
                edge: join.kept,
                carrier: carrier.kind(),
                closed: false,
            })?
    };
    let (a, b) = if backward { (tx, t1) } else { (t0, tx) };
    let description = match kept.description() {
        geom_brep::EdgeDescription::Intersection { s1, s2, .. } => {
            geom_brep::EdgeDescriptionSpec::Intersection {
                s1: *s1,
                s2: *s2,
                witness: carrier.mid_point(a, b),
            }
        }
        geom_brep::EdgeDescription::TangentIntersection { s1, s2, .. } => {
            geom_brep::EdgeDescriptionSpec::TangentIntersection {
                s1: *s1,
                s2: *s2,
                witness: carrier.mid_point(a, b),
            }
        }
        geom_brep::EdgeDescription::Chart(c) => geom_brep::EdgeDescriptionSpec::Chart {
            surface: c.surface,
            image: if c.wrap { None } else { Some(c.pcurve.clone()) },
            wrap: c.wrap,
            // A declaration's parameter is affine in the carrier's, so
            // the kept edge's runs on over the joined span exactly as a
            // split restricts it; certification re-meters it.
            declared: match kept.authority() {
                geom_brep::EdgeAuthority::Declared(mc) => {
                    let span = t1 - t0;
                    Some(mc.restrict((a - t0) / span, (b - t0) / span))
                }
                geom_brep::EdgeAuthority::Derived => None,
            },
        },
        geom_brep::EdgeDescription::Scaffold(_) => {
            return Err(desync("a joined edge is scaffolding at rest"));
        }
    };
    Ok(geom_brep::EdgeCurveSpec {
        description,
        carrier: carrier.clone(),
        param_start: a,
        param_end: b,
    })
}

/// Joins at `w` (module docs): `gone` dies with `w` through `kev`, and
/// `kept`, re-based onto `gone`'s far end, is restated over both edges'
/// span: a planar join as the chord its faces then describe, a curved
/// one on its own carrier ([`joined_spec`]).
fn join_one<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    w: VertexKey,
    join: &Join,
    band: Band,
    tol: Tol,
) -> Result<(), BooleanError> {
    let spec = if join.planar {
        let members = body
            .kev_merged_members(join.he)
            .map_err(BooleanError::Euler)?;
        members
            .iter()
            .find(|m| m.edge == join.kept)
            .map(|m| crate::EdgeCurveSpec::line_between(m.start, m.end))
            .ok_or(BooleanError::JoinDesync {
                what: "a joined edge is not the kill's merged member",
            })?
    } else {
        joined_spec(body, w, join)?
    };
    let killed = body
        .kev_describing(join.he, &[(join.kept, spec)], tol)
        .map_err(BooleanError::Euler)?;
    debug_assert_eq!(killed.killed_vertex, w, "the kill takes the joined vertex");
    if !join.planar {
        return Ok(());
    }
    describe_minted_edges(
        body,
        &[join.kept],
        &crate::merge_faces::MergeCoplanarOutcome::default(),
        band,
        tol,
    )
}

#[cfg(test)]
mod conventional {
    //! **[`is_conventional_vertex`] is structural**: a vertex whose
    //! only edge is one closed edge, at both its ends. A self-loop's
    //! lone vertex is conventional; an open edge's end is not, and nor
    //! is a self-loop's vertex once another edge leaves it (a seam
    //! strut on the rim it closes, as `pi_seam`'s puck keeps).
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use geom_core::{Point3, Tol};

    use super::is_conventional_vertex;
    use crate::{Body, MefSite, MevSite};

    #[test]
    fn only_a_lone_self_loops_vertex_is_conventional() {
        let tol = Tol::witness();
        let mut body = Body::<f64>::new();
        let born = body.mvfs(Point3::new(0.0, 0.0, 0.0), true).unwrap();
        assert!(!is_conventional_vertex(&body, born.vertex), "a lone vertex");
        let closed = body
            .mef_chord(
                MefSite::Lone {
                    r#loop: born.r#loop,
                },
                tol,
            )
            .unwrap();
        assert!(is_conventional_vertex(&body, born.vertex), "a self-loop's");
        let he = body.get_edge(closed.edge).unwrap().he_plus;
        let strut = body
            .mev_line(
                MevSite::Fan { he1: he, he2: he },
                Point3::new(0.0, 0.0, 1.0),
                tol,
            )
            .unwrap();
        // Whichever half-edge the vertex records as its first, the
        // strut leaving it is read: the closed edge's two halves, and
        // the strut's own.
        let e = body.get_edge(closed.edge).unwrap();
        let s = body.get_edge(strut.edge).unwrap();
        let out = [s.he_plus, s.he_minus]
            .into_iter()
            .find(|&h| body.get_half_edge(h).unwrap().start == born.vertex)
            .unwrap();
        let firsts = [e.he_plus, e.he_minus, out];
        for first in firsts {
            body.vertices.get_mut(born.vertex).unwrap().emanating = Some(first);
            assert!(
                !is_conventional_vertex(&body, born.vertex),
                "a self-loop's vertex another edge leaves, first {first:?}"
            );
        }
        let far = body
            .half_edge_end(body.get_edge(strut.edge).unwrap().he_plus)
            .unwrap();
        assert!(!is_conventional_vertex(&body, far), "an open edge's end");
    }
}

#[cfg(test)]
mod join_door {
    //! **The join door and the doors that end with it**: a brick whose
    //! edge was split at its middle holds one joinable vertex; the join
    //! door takes it back to the brick, and the public merge door and
    //! an offset door, ending with the join, leave none either.
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use geom_core::{Band, Tol};

    use super::joinable_vertices;
    use crate::body::Body;
    use crate::split::SplitEdgeCreated;
    use crate::test_support_fixtures::brick;

    /// A unit brick with one edge split at its middle, the split's
    /// record, and the brick's own arena counts.
    fn split_brick() -> (
        Body<f64>,
        SplitEdgeCreated,
        crate::test_support::ArenaCounts,
    ) {
        let tol = Tol::witness();
        let mut body = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let whole = body.arena_counts();
        let (e, d) = body.edges().next().unwrap();
        let (t0, t1) = body
            .get_curve_geom(d.curve)
            .and_then(crate::CurveGeom::certified)
            .unwrap()
            .params();
        let made = body.split_edge(e, 0.5 * (t0 + t1), tol).unwrap();
        (body, made, whole)
    }

    #[test]
    fn the_join_door_takes_a_split_edge_back() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let (mut body, made, whole) = split_brick();
        assert_eq!(joinable_vertices(&body, band).unwrap(), vec![made.vertex]);
        let joins = body.join_edges(band, tol).unwrap();
        assert_eq!(joins.len(), 1);
        assert_eq!(joins[0].vertex, made.vertex);
        assert_eq!(joins[0].conventional, None);
        assert_eq!(body.arena_counts(), whole, "the brick again");
        assert!(joinable_vertices(&body, band).unwrap().is_empty());
        crate::validate_geometric(&body, tol).unwrap();
        assert!(body.join_edges(band, tol).unwrap().is_empty(), "idempotent");
    }

    #[test]
    fn the_merge_door_ends_with_the_join_and_reports_it() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let (mut body, made, whole) = split_brick();
        let outcome = body.merge_coplanar_faces(tol).unwrap();
        assert!(
            outcome.groups.is_empty(),
            "nothing to merge, and still joined"
        );
        assert_eq!(outcome.joins.len(), 1);
        assert_eq!(outcome.joins[0].vertex, made.vertex);
        assert_eq!(body.arena_counts(), whole);
        assert!(joinable_vertices(&body, band).unwrap().is_empty());
    }

    #[test]
    fn an_offset_door_ends_with_the_join() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let (mut body, made, whole) = split_brick();
        // A face the split edge does not bound, so the offset keeps
        // the vertex and only the join takes it.
        let far = body
            .faces()
            .map(|(f, _)| f)
            .find(|&f| {
                body.face_loops_linked(f, body.get_face(f).unwrap())
                    .all(|(_, l)| match l.boundary {
                        crate::LoopBoundary::Cycle { first } => body
                            .loop_cycle(first)
                            .unwrap()
                            .iter()
                            .all(|&h| body.get_half_edge(h).unwrap().start != made.vertex),
                        crate::LoopBoundary::Empty { .. } => true,
                    })
            })
            .unwrap();
        crate::replace_face_offset(&mut body, far, 0.25, tol).unwrap();
        assert!(joinable_vertices(&body, band).unwrap().is_empty());
        assert_eq!(body.arena_counts(), whole);
    }
}
