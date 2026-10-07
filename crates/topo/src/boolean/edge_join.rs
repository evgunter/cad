//! **The join**: a vertex of valence 2 whose two edges lie on one
//! structural carrier between the same two faces is no corner, so the
//! two edges are one edge cut for no reason (`docs/DESIGN.md`, maximal
//! edges). The join kills the vertex and makes the two edges one. Every
//! boolean output stage runs it after the merge ([`join_stage`]) and
//! writes, per join, the substitution rows that carry every contact
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

/// **The output stage's join**: joins every joinable vertex
/// ([`joinable_vertices`]) of `body`, one at a time in vertex-arena
/// order until none is left, and writes each join's substitution rows
/// into `desc` (`w → kept`, `gone → kept`, and where the join leaves a
/// conventional vertex, that vertex `→ kept`), so the op's one
/// [`super::ops::carry`] takes every record through its zips, its merge
/// and its joins together. Runs after the merge and its re-description,
/// before the records are carried. Returns the joins in the order made,
/// a later one's `gone` or `kept` possibly an earlier one's `kept`.
///
/// # Errors
///
/// [`BooleanError::JoinUndecided`], the kill's own refusal
/// ([`BooleanError::Euler`]), a carrier the joined edge cannot be
/// restated on ([`BooleanError::JoinCarrierUnsupported`]), or the
/// description's.
pub(super) fn join_stage<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    desc: &mut Descendants,
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
        desc.substitute(Cell::Vertex(w), Cell::Edge(join.kept));
        desc.substitute(Cell::Edge(join.gone), Cell::Edge(join.kept));
        // A closed join's survivor is conventional unless another edge
        // still ends there (a seam strut on the rim it closed).
        let conventional =
            (join.closed && is_conventional_vertex(body, join.far)).then_some(join.far);
        if let Some(v) = conventional {
            desc.substitute(Cell::Vertex(v), Cell::Edge(join.kept));
        }
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
            image: if c.seam { None } else { Some(c.pcurve.clone()) },
            seam: c.seam,
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
