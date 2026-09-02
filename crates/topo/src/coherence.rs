//! **Chart coherence of a body's own data** — a non-gating examination
//! that measures how far a curved face's CARRIERS and its VERTICES
//! disagree about the chart coordinates they both state.
//!
//! [`examine_chart_coherence`] answers one question, three ways, and
//! decides nothing: *the source states this face's boundary twice over
//! — once as carrier geometry and once as vertex positions — and by how
//! many metres do the two accounts differ?* Every condition is
//! computable from the body alone: no tessellation, no chord
//! subdivision, no δ. A body that meshes today meshes exactly the same
//! after this door has looked at it, because nothing consumes what it
//! returns.
//!
//! # The three conditions
//!
//! Each is a gap in CHART UNITS times a LEVER ARM in metres per chart
//! unit, judged against ε as a length. The unit matters more than the
//! bar: a fixed coordinate-rounding error in a source file subtends a
//! LARGER angle on a SMALLER feature, so an angular bar necessarily
//! mis-ranks small features while a spatial one does not.
//!
//! - [`CoherenceCondition::MeridianClosure`] — a meridian edge's
//!   carrier-MIDPOINT azimuth against the azimuth of one of its own
//!   endpoint vertices. Analytically these are one number; a gap
//!   between them says the source states that edge's endpoints off the
//!   axis the carrier runs along. Lever: [`Chart::radial`] at the
//!   vertex — the point's own distance from the chart axis, which
//!   varies over a cone and a sphere and is zero at a pole, where an
//!   azimuth carries no length at all.
//! - [`CoherenceCondition::RimContinuation`] — two edges carrying ONE
//!   rim row disagree in v. Two distinct `Circle` carriers stating the
//!   same analytic circle (what an exporter emits when a vertex lands
//!   on a rim) each derive their own row coordinate; the gap is the
//!   one a consumer discards when it gives the whole side one value.
//!   Lever: [`Chart::v_lever`], constant per kind.
//! - [`CoherenceCondition::MeridianContinuation`] — the same, one axis
//!   over: two edges carrying one meridian column disagree in u. Lever:
//!   [`Chart::radial`] at the junction between them.
//!
//! # What a finding is, and is not
//!
//! It is a MEASUREMENT, never a verdict about validity. The bodies
//! these conditions fire on are bodies whose meshes and volumes may be
//! perfectly correct — the two π-rad witnesses in tree (issue 723's
//! imported half-cap, issue 1571's Euler-door body) both certify
//! green through tier 3 and one of them meshes. Nothing here refuses,
//! nothing here is a tier, and no consumer is required to react.
//!
//! **Not covered by D9's byte-identity contract.** That contract is
//! about the MESH: the same body at the same δ yields the same
//! triangles, byte for byte. This report is a different value from a
//! different door, and `mesh::tessellate` neither calls it nor carries
//! it. What this report has instead is its own determinism, stated at
//! [`examine_chart_coherence`]: it is a pure function of (body, ε).
//!
//! # Where these conditions used to live
//!
//! In `mesh::walk`, as three `debug_assert!`s inside the boundary
//! walk — a tessellator asserting about the quality of somebody else's
//! coordinates, in a build profile that panicked on a file whose mesh
//! was correct. They measure a fact about the BODY, so they are stated
//! about the body, by a door whose whole output is the measurement.
//! The ledger those assertions carried, both its arguments and its two
//! traced witnesses, is kept below rather than dropped.
//!
//! # The ledger, moved
//!
//! **TRACED — the one real file that ever tripped one of these in
//! tree**, kept because it is the argument for the bar's UNIT and not
//! a census figure. `nist_ftc_09_asme1_rd.stp`'s hole generators state
//! their two endpoints non-co-azimuthally:
//!
//! ```text
//! p0 = (-3.1330000001, +0.0896, -4.2499999992) inch
//! p1 = (-3.1330000000,  0.0,    -4.2500000000) inch
//! ```
//!
//! The line runs along the hole axis and should hold x and z fixed;
//! the file's ~10-digit coordinates differ in the last one. That is
//! 21.4 pm PERPENDICULAR to the axis, subtending 7.2e-9 rad at
//! r = 2.9718e-3 m — and a closure gap is half that spread, because
//! one side of the comparison reads a vertex azimuth and the other the
//! carrier midpoint's. In metres the invariant quantity shows: 21 pm,
//! 1.6e6x inside that file's own ε, so nothing is reported. A
//! scale-free radian constant (the bar this once was) flagged the
//! 0.117-inch holes while passing the same physical error elsewhere.
//!
//! **The two π-rad witnesses**, both a meridian ARC that crosses a
//! pole mid-edge, so the carrier's midpoint sits a half-turn from its
//! own endpoint: issue 723's half-cap through import
//! (`step-import/tests/fixtures/halfcap/halfcap_eps7.step`, whose
//! offending endpoint is 1.0e-9 m from the axis, so the half-turn
//! opens 3.14 nm of arc and the report is band-shaped across the ε
//! rows), and issue 1571's Euler-door body, which the iso-rectangle
//! shape door admits. Nothing else on source data has been seen above
//! 3.6e-9 rad — and **nothing re-measures that**: no test in this repo
//! examines the wild or FreeCAD corpora, so the input class these
//! conditions are about is untested by construction and that sentence
//! is a trace, not a census. Issue 1571 owns FIXING the arc premise;
//! this door owns seeing it.

use geom::Curve3;
use geom_brep::EdgeDescription;
use geom_core::{Point3, Tol};

use crate::body::Body;
use crate::chart::Chart;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};

/// Which of the three conditions a [`CoherenceFinding`] reports, and
/// what its gap was measured AGAINST.
///
/// The comparand rides inside the variant rather than beside it: a
/// closure gap is always against a vertex and a continuation gap is
/// always against another edge, so a struct carrying one
/// `Option<VertexKey>` and one `Option<EdgeKey>` would spell three
/// legal shapes and one illegal one. D2 addendum row 0, answered at
/// the type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoherenceCondition {
    /// The edge's carrier-midpoint azimuth against `vertex`'s own —
    /// one of that edge's two endpoints. Gap in radians of u; lever
    /// [`Chart::radial`] at `vertex`.
    MeridianClosure {
        /// The endpoint whose azimuth the carrier is compared with.
        vertex: VertexKey,
    },
    /// The edge's own rim v against the v of `opens`, the edge that
    /// OPENS the iso side both belong to. Gap in v's own units
    /// (metres on a cylinder or cone, radians on a sphere or torus);
    /// lever [`Chart::v_lever`].
    RimContinuation {
        /// The edge that opens the shared iso side.
        opens: EdgeKey,
    },
    /// The edge's own carrier-midpoint azimuth against that of
    /// `opens`, the edge that OPENS the meridian column both belong
    /// to. Gap in radians of u; lever [`Chart::radial`] at the
    /// junction between the two edges.
    MeridianContinuation {
        /// The edge that opens the shared iso side.
        opens: EdgeKey,
    },
}

/// One coherence finding: a measurement, in metres, of how far two of
/// the body's own statements about one chart coordinate disagree.
///
/// **A finding is not a verdict.** It says the source states this
/// twice and the two statements differ by this much; it does not say
/// the body is invalid, that the mesh is wrong, or that anything
/// should be refused. Reading it as any of those is reading intent
/// into a number, which is the move this kernel does not make.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CoherenceFinding {
    /// The curved face whose chart the coordinates are read in.
    pub face: FaceKey,
    /// The boundary loop the two edges (or the edge and the vertex)
    /// are traversed by.
    pub r#loop: LoopKey,
    /// The edge whose carrier stated the value being measured.
    pub edge: EdgeKey,
    /// Which condition, and what the gap was measured against.
    pub condition: CoherenceCondition,
    /// The disagreement in CHART units — radians of u, or v's own
    /// units by kind. Wrapped to the nearest representative on a
    /// periodic axis, so it never exceeds π there.
    pub gap: f64,
    /// The lever arm in metres per chart unit, at the point the gap is
    /// about. Zero on the axis, where an azimuth carries no length.
    pub lever: f64,
    /// `gap * lever` — the disagreement as a length, which is the only
    /// unit ε is in and the only one the finding is judged in.
    pub metres: f64,
    /// The band this was judged against: `metres < eps` is float noise
    /// and is not reported at all, so every finding satisfies
    /// `metres >= eps`.
    pub eps: f64,
}

/// Why one loop of a chart-bearing face was not examined.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unexaminable {
    /// The loop's structure did not resolve — a stale key or a broken
    /// half-edge cycle. Tier 1's business, named here rather than
    /// dropped: this door reports coherence, and re-reporting a
    /// structural defect in a second vocabulary would give it a second
    /// home.
    Corrupt {
        /// What did not resolve.
        what: &'static str,
    },
    /// An edge of the loop carries construction scaffolding rather
    /// than a certified curve, so it states no coordinate to compare.
    NullScaffoldEdge {
        /// The scaffolding edge.
        edge: EdgeKey,
    },
    /// The face's outer loop carries a conic or spline trim carrier,
    /// so its boundary is not a chart iso curve and none of the three
    /// conditions is about it. The same test routes the face away from
    /// the iso walk in `mesh`.
    NonIsoCarrier {
        /// The edge whose carrier is neither a line nor a circle.
        edge: EdgeKey,
    },
}

/// One loop the examination could not reach, and why.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unexamined {
    /// The face owning the loop.
    pub face: FaceKey,
    /// The loop itself.
    pub r#loop: LoopKey,
    /// The reason, typed.
    pub why: Unexaminable,
}

/// What [`examine_chart_coherence`] found, and what it could not look
/// at — the shape `editor_core`'s `ChecksReport` uses, for the same
/// reason: a report that lists only its findings cannot be told apart
/// from one that had nothing to look at.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CoherenceReport {
    /// Every finding, in the order described at
    /// [`examine_chart_coherence`].
    pub findings: Vec<CoherenceFinding>,
    /// Every loop of a chart-bearing face that was skipped, in the
    /// same traversal order.
    pub unexamined: Vec<Unexamined>,
}

/// Full turn, the period of every wrapped axis here.
const TAU: f64 = core::f64::consts::TAU;

/// The unsigned distance between two values of a PERIODIC chart
/// coordinate: `|a − b|` with `b`'s nearest representative of `a`
/// chosen, so the answer never exceeds half a period.
///
/// This is the same reduction the consuming walk performs before it
/// discards one of the two values, spelled here as the measurement it
/// is rather than as a branch selection: on a full turn of skew the
/// two statements agree, and a report that called that a π gap would
/// be reporting the reduction and not the data.
fn wrapped(a: f64, b: f64) -> f64 {
    let d = a - b;
    (d - TAU * (d / TAU).round()).abs()
}

/// **The band, and its only spelling in this module.** Is a chart
/// discrepancy of `gap`, at a point whose lever arm for that axis is
/// `lever` metres per chart unit, float noise rather than something to
/// report?
///
/// `gap * lever` is the LENGTH the two statements disagree by and ε is
/// a length: the only unit in which the comparison means anything. The
/// band is EXCLUDED (`<`), which fixes two edges deliberately — a zero
/// band reports every nonzero gap, and a NaN coordinate is never noise,
/// so a poisoned carrier surfaces as a finding rather than passing as
/// quiet. At `lever == 0` (a vertex on the chart axis: a pole, a cone's
/// apex) the coordinate carries no length, so every gap is noise — the
/// correct limit, reached without a special case or a division.
fn is_noise(gap: f64, lever: f64, eps: f64) -> bool {
    gap * lever < eps
}

/// A boundary edge as this examination reads it: the two coordinates
/// the source states about it, and the two junctions it runs between.
struct Trav {
    edge: EdgeKey,
    start: VertexKey,
    end: VertexKey,
    start_p: Point3<f64>,
    end_p: Point3<f64>,
    kind: TravKind,
}

/// The iso classification of a boundary edge, and the raw constant
/// coordinate its own carrier states.
enum TravKind {
    /// A circle about the chart axis: the raw row v.
    Rim { v_raw: f64 },
    /// A u = const boundary: the raw column azimuth, read at the
    /// carrier's mid-parameter point.
    Meridian { u_raw: f64 },
}

impl TravKind {
    /// Whether two traversals could be two edges of ONE iso side —
    /// necessary, not sufficient: the junction between them must also
    /// lie off the chart axis.
    fn same_kind(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Rim { .. }, Self::Rim { .. }) | (Self::Meridian { .. }, Self::Meridian { .. })
        )
    }
}

/// The azimuth of an edge's mid-parameter carrier point — a
/// representative INTERIOR point, never an endpoint, so a meridian
/// running into a pole is still read somewhere the azimuth exists.
fn mid_azimuth(chart: &Chart, curve: &geom_brep::EdgeCurve<f64>) -> f64 {
    let (t0, t1) = curve.params();
    chart.u_of(curve.carrier().eval(t0 + (t1 - t0) * 0.5))
}

/// Rim-vs-meridian classification: `Seam` descriptions and line
/// carriers are meridians; circle carriers split on axis alignment
/// (structurally either parallel — a rim — or perpendicular — a
/// meridian; 0.5 splits the two classes with maximal margin).
///
/// A meridian's column always comes from the mid-point chart
/// inversion, never from the edge kind: a `Seam` edge is the surface's
/// `u_ref`-half-plane meridian, whose chart u is 0 on ordinary kinds
/// but π on a cone's mirror nappe.
fn classify(chart: &Chart, curve: &geom_brep::EdgeCurve<f64>) -> Option<TravKind> {
    if matches!(curve.description(), EdgeDescription::Chart(c) if c.seam) {
        return Some(TravKind::Meridian {
            u_raw: mid_azimuth(chart, curve),
        });
    }
    match *curve.carrier() {
        Curve3::Line { .. } => Some(TravKind::Meridian {
            u_raw: mid_azimuth(chart, curve),
        }),
        Curve3::Circle {
            center,
            axis,
            radius,
            ..
        } => {
            if axis.dot(chart.axis).abs() > 0.5 {
                Some(TravKind::Rim {
                    v_raw: chart.rim_v(center, radius),
                })
            } else {
                Some(TravKind::Meridian {
                    u_raw: mid_azimuth(chart, curve),
                })
            }
        }
        Curve3::Ellipse { .. } | Curve3::Nurbs(_) => None,
    }
}

/// Reads one loop into the traversal list the conditions are stated
/// over, or names why it could not be read.
fn traversals(body: &Body<f64>, chart: &Chart, lk: LoopKey) -> Result<Vec<Trav>, Unexaminable> {
    let corrupt = |what| Unexaminable::Corrupt { what };
    let point = |body: &Body<f64>, v: VertexKey| -> Option<Point3<f64>> {
        body.get_vertex(v)
            .and_then(|vx| body.get_point(vx.point))
            .copied()
    };
    let lp = body
        .get_loop(lk)
        .ok_or(corrupt("loop key does not resolve"))?;
    let LoopBoundary::Cycle { first } = lp.boundary else {
        return Err(corrupt("empty loop (construction scaffolding at rest)"));
    };
    let cycle: Vec<HalfEdgeKey> = body
        .loop_cycle(first)
        .ok_or(corrupt("broken half-edge cycle"))?;
    let mut out = Vec::with_capacity(cycle.len());
    for hek in cycle {
        let he = body
            .get_half_edge(hek)
            .ok_or(corrupt("half-edge key does not resolve"))?;
        let edge = body
            .get_edge(he.edge)
            .ok_or(corrupt("edge key does not resolve"))?;
        let curve = body
            .get_curve_geom(edge.curve)
            .ok_or(corrupt("curve key does not resolve"))?
            .certified()
            .ok_or(Unexaminable::NullScaffoldEdge { edge: he.edge })?;
        let end = body
            .half_edge_end(hek)
            .ok_or(corrupt("half-edge mate does not resolve"))?;
        let start_p = point(body, he.start).ok_or(corrupt("vertex point does not resolve"))?;
        let end_p = point(body, end).ok_or(corrupt("vertex point does not resolve"))?;
        let kind = classify(chart, curve).ok_or(Unexaminable::NonIsoCarrier { edge: he.edge })?;
        out.push(Trav {
            edge: he.edge,
            start: he.start,
            end,
            start_p,
            end_p,
            kind,
        });
    }
    Ok(out)
}

/// Which traversals OPEN an iso side.
///
/// An iso side may be carried by SEVERAL edges — a vertex dropped on
/// it by [`Body::split_edge`], a boolean, or an exporter emitting two
/// collinear edges (which is what every exporter emits when a vertex
/// lands on such an edge). Two consecutive traversals continue ONE
/// side when they are the same kind AND the junction between them lies
/// off the chart axis: at the axis the two are a genuine corner (a
/// pole fan, a cone apex), not a split side, and their coordinates are
/// under no obligation to agree.
///
/// The `>` is strict, the band excluded, matching the walk that acts
/// on this grouping: a junction exactly ε from the axis is NOT
/// separated from it.
///
/// Cyclic, with one forced opening: if every traversal reads as a
/// continuation the whole loop is a single cyclic run with no opening,
/// and index 0 is made to open it. That is the same resolution the
/// walk makes, and it moves only WHICH member of the run is the
/// reference, never how far the members are from each other.
fn iso_side_starts(travs: &[Trav], chart: &Chart, eps: f64) -> Vec<bool> {
    let m = travs.len();
    if m < 2 {
        return vec![true; m];
    }
    let mut starts: Vec<bool> = (0..m)
        .map(|k| {
            let same = travs[(k + m - 1) % m].kind.same_kind(&travs[k].kind);
            !(same && chart.radial(travs[k].start_p) > eps)
        })
        .collect();
    if !starts.iter().any(|&s| s) {
        starts[0] = true;
    }
    starts
}

/// For each traversal, the index of the traversal that OPENS its iso
/// side — itself when it opens one, else the nearest opening before it
/// cyclically.
///
/// Every continuation in a run is measured against the run's OPENING
/// and not against its immediate predecessor, because that is the
/// value a consumer keeps: one iso side carries ONE coordinate, the
/// opening edge's, and every later edge of the side has its own
/// discarded in favour of it. Measuring pairwise would report the
/// increments of a quantity whose total is what was actually thrown
/// away.
fn openings(starts: &[bool]) -> Vec<usize> {
    let m = starts.len();
    let mut out = vec![0usize; m];
    let mut open = 0usize;
    // Two passes, because the first traversal's opening may lie behind
    // it in the cycle. The second pass sees the settled value.
    for _ in 0..2 {
        for k in 0..m {
            if starts[k] {
                open = k;
            }
            out[k] = open;
        }
    }
    out
}

/// The three conditions over one loop's traversals, in traversal
/// order: each traversal's continuation gap (when it continues a side)
/// before its two closure gaps (when it is a meridian).
fn loop_findings(
    travs: &[Trav],
    chart: &Chart,
    face: FaceKey,
    lk: LoopKey,
    eps: f64,
) -> Vec<CoherenceFinding> {
    let starts = iso_side_starts(travs, chart, eps);
    let opens = openings(&starts);
    let mut out = Vec::new();
    let mut push = |edge: EdgeKey, condition: CoherenceCondition, gap: f64, lever: f64| {
        if !is_noise(gap, lever, eps) {
            out.push(CoherenceFinding {
                face,
                r#loop: lk,
                edge,
                condition,
                gap,
                lever,
                metres: gap * lever,
                eps,
            });
        }
    };
    for (k, t) in travs.iter().enumerate() {
        if !starts[k] {
            let o = &travs[opens[k]];
            match (&o.kind, &t.kind) {
                (TravKind::Rim { v_raw: opened }, TravKind::Rim { v_raw: own }) => {
                    let gap = if chart.v_periodic() {
                        wrapped(*own, *opened)
                    } else {
                        (own - opened).abs()
                    };
                    push(
                        t.edge,
                        CoherenceCondition::RimContinuation { opens: o.edge },
                        gap,
                        chart.v_lever(),
                    );
                }
                (TravKind::Meridian { u_raw: opened }, TravKind::Meridian { u_raw: own }) => {
                    push(
                        t.edge,
                        CoherenceCondition::MeridianContinuation { opens: o.edge },
                        wrapped(*own, *opened),
                        chart.radial(t.start_p),
                    );
                }
                // A traversal continues a side only if it and its
                // PREDECESSOR are the same kind, and a run's opening
                // is reached from a continuation by walking that
                // relation backwards — so every member of a run is the
                // kind its opening is. Stated rather than defaulted:
                // a default would silently measure a rim against a
                // meridian and report the difference between two
                // unrelated coordinates as a defect in the data.
                _ => unreachable!("an iso side's continuations are the kind its opening edge is"),
            }
        }
        if let TravKind::Meridian { u_raw } = t.kind {
            for (vertex, p) in [(t.start, t.start_p), (t.end, t.end_p)] {
                push(
                    t.edge,
                    CoherenceCondition::MeridianClosure { vertex },
                    wrapped(u_raw, chart.u_of(p)),
                    chart.radial(p),
                );
            }
        }
    }
    out
}

/// **Examine a body's chart coherence.** For every face carrying an
/// analytic chart, measure how far that face's own carriers and
/// vertices disagree about the chart coordinates they both state, and
/// report every disagreement that clears ε as a length.
///
/// # This door decides nothing
///
/// It has no failure mode, no tier, and no verdict. It refuses no
/// body, it changes no body, and no operation in this kernel consults
/// what it returns. A body that meshes, validates, or exports today
/// does exactly the same after this has looked at it — which is the
/// whole point of stating these conditions here rather than as
/// assertions inside a tessellator.
///
/// # Order, and determinism
///
/// A pure function of `(body, ε)`: every input is a declared vertex's
/// position or a carrier evaluation at a mid-parameter point, and
/// there is no δ, no chord subdivision and no mesh state anywhere in
/// it. The order is total and structural — faces in arena order, each
/// face's outer loop then its rings in list order, each loop's
/// traversals in cycle order, and within one traversal its
/// continuation gap before its two closure gaps.
///
/// **`ε` is read once**, from `tol`, and stamped on every finding: a
/// finding read without the band it was judged at is a number without
/// a claim, and the band is per-run configuration.
///
/// # What is out of domain, and what is unexamined
///
/// A face whose surface carries no chart — a plane, a NURBS or an
/// approximating surface — is OUT OF DOMAIN and is not reported at
/// all: there is no chart, so there is no coordinate for two
/// statements to differ about. A chart-bearing face whose loop cannot
/// be READ is different, and is named in
/// [`CoherenceReport::unexamined`] rather than dropped. A face whose
/// OUTER loop carries a conic or spline trim carrier takes every one
/// of its loops out with it: that carrier is what routes the face away
/// from the iso-curve lane entirely, so none of the three conditions
/// is about it.
///
/// # Not the byte contract
///
/// D9's byte-identity contract is about the MESH — same body, same δ,
/// same triangles, byte for byte. This report is a different value
/// from a different door and is covered by the determinism above
/// instead. Nothing here is mesh bytes and nothing here may be cited
/// as evidence about them.
pub fn examine_chart_coherence(body: &Body<f64>, tol: Tol) -> CoherenceReport {
    let eps = tol.eps();
    let mut report = CoherenceReport::default();
    for (fk, face) in body.faces() {
        let loops: Vec<LoopKey> = core::iter::once(face.outer)
            .chain(face.rings.iter().copied())
            .collect();
        let Some(surface) = body.get_surface(face.surface) else {
            report.unexamined.extend(loops.iter().map(|&lk| Unexamined {
                face: fk,
                r#loop: lk,
                why: Unexaminable::Corrupt {
                    what: "face surface key does not resolve",
                },
            }));
            continue;
        };
        let Some(chart) = Chart::of(surface) else {
            continue;
        };
        let read: Vec<Result<Vec<Trav>, Unexaminable>> = loops
            .iter()
            .map(|&lk| traversals(body, &chart, lk))
            .collect();
        // The FACE-level routing test: a conic or spline carrier on the
        // outer loop takes the whole face out of the iso lane, rings
        // included, because that is the granularity the routing has.
        if let Some(&Err(why @ Unexaminable::NonIsoCarrier { .. })) = read.first() {
            report.unexamined.extend(loops.iter().map(|&lk| Unexamined {
                face: fk,
                r#loop: lk,
                why,
            }));
            continue;
        }
        for (&lk, got) in loops.iter().zip(read) {
            match got {
                Ok(travs) => report
                    .findings
                    .extend(loop_findings(&travs, &chart, fk, lk, eps)),
                Err(why) => report.unexamined.push(Unexamined {
                    face: fk,
                    r#loop: lk,
                    why,
                }),
            }
        }
    }
    report
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// **The bar is SPATIAL, not angular.** The gap that falsified the
    /// bare-radian form this predicate replaced —
    /// `nist_ftc_09_asme1_rd.stp` closes at 3.56e-9 rad on a 2.97 mm
    /// hole — is noise at any real lever arm, and stops being noise
    /// only at an absurd one. A scale-free radian constant cannot
    /// express that, which is the whole reason the unit is metres.
    #[test]
    fn the_bar_is_spatial_not_angular() {
        let gap = 3.56e-9;
        assert!(
            is_noise(gap, 0.05, 3.38e-5),
            "3.56e-9 rad at 50 mm displaces ~1.8e-10 m — far under the band"
        );
        assert!(
            !is_noise(gap, 1e5, 3.38e-5),
            "the same gap must NOT pass at a 100 km lever arm"
        );
    }

    /// Growing the lever arm tightens the angular bar proportionally —
    /// the property a bare radian constant did not have.
    #[test]
    fn the_bar_tightens_as_the_lever_arm_grows() {
        assert!(is_noise(1e-6, 1.0, 1e-5), "1e-6 m is under the band");
        assert!(
            !is_noise(1e-6, 100.0, 1e-5),
            "1e-4 m is over it — the same angle, a hundred times the arc"
        );
    }

    /// On the axis the coordinate carries no length, so every gap is
    /// noise. A plain comparison reaches that limit; a division would
    /// have to special-case it.
    #[test]
    fn on_the_axis_every_gap_is_noise() {
        assert!(is_noise(core::f64::consts::PI, 0.0, 1e-9));
    }

    /// The band is EXCLUDED at both ends it can be excluded at: a zero
    /// band calls nothing noise (so a zero-lever gap is reported rather
    /// than swallowed), and a gap exactly ON the band is reported. The
    /// two edges together are what make "every finding satisfies
    /// `metres >= eps`" true as written.
    #[test]
    fn the_band_is_excluded_at_both_edges() {
        assert!(!is_noise(1.0, 0.0, 0.0), "a zero band dominates nothing");
        assert!(
            !is_noise(1.0, 1e-9, 1e-9),
            "exactly on the band is reported"
        );
        assert!(is_noise(1.0, 1e-9, 1.000_001e-9), "just inside it is not");
    }

    /// A NaN coordinate is never noise: a poisoned carrier surfaces as
    /// a finding rather than passing as quiet, which is the direction
    /// this comparison has to fail in.
    #[test]
    fn a_poisoned_gap_is_not_noise() {
        assert!(!is_noise(f64::NAN, 1.0, 1e-9));
        assert!(!is_noise(1.0, f64::NAN, 1e-9));
    }

    /// A periodic coordinate's disagreement is measured after
    /// reduction, so a whole number of turns is AGREEMENT and the
    /// answer never exceeds half a period. Reporting the reduction
    /// itself would be reporting the branch and not the data.
    #[test]
    fn a_whole_turn_of_skew_is_no_disagreement() {
        for turns in [-2.0_f64, -1.0, 1.0, 2.0] {
            let a = 0.7 + turns * TAU;
            assert!(wrapped(a, 0.7) < 1e-15, "{turns} turns read as a gap");
        }
        let half = wrapped(core::f64::consts::PI, 0.0);
        assert!((half - core::f64::consts::PI).abs() < 1e-15, "{half}");
        assert!(
            wrapped(core::f64::consts::PI + 0.1, 0.0) <= core::f64::consts::PI,
            "never more than half a period"
        );
    }

    /// The lever arms are the chart's own, and the two axes take
    /// different ones for a reason the type carries: u's varies over a
    /// cone and a sphere because it is the point's distance from the
    /// axis, and v's does not.
    #[test]
    fn the_two_axes_take_the_levers_their_charts_state() {
        let sphere = Chart::of(&geom::Surface::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 3.0,
            axis: geom_core::Vec3::new(0.0, 0.0, 1.0),
            u_ref: geom_core::Vec3::new(1.0, 0.0, 0.0),
        })
        .unwrap();
        assert!((sphere.v_lever() - 3.0).abs() < 1e-15);
        assert!((sphere.radial(Point3::new(3.0, 0.0, 0.0)) - 3.0).abs() < 1e-15);
        assert!(
            sphere.radial(Point3::new(0.0, 0.0, 3.0)).abs() < 1e-15,
            "a pole"
        );
    }

    /// A plane is OUT OF DOMAIN, not unexamined: it carries no chart,
    /// so there is no coordinate for two statements to differ about
    /// and nothing to report either way.
    #[test]
    fn a_plane_is_out_of_domain_rather_than_unexamined() {
        assert!(
            Chart::of(&geom::Surface::Plane {
                origin: Point3::new(0.0, 0.0, 0.0),
                normal: geom_core::Vec3::new(0.0, 0.0, 1.0),
                u_ref: geom_core::Vec3::new(1.0, 0.0, 0.0),
            })
            .is_none()
        );
    }
}
