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
//!   Lever: [`Chart::v_lever`], constant per kind. **This one has no
//!   witness in any corpus**: a rim through two given points is unique
//!   on every chart kind here, so a v gap forces one carrier off the
//!   surface and the natively-constructible witness is synthetic. A
//!   committed fixture through the import door is issue 1588.
//! - [`CoherenceCondition::MeridianContinuation`] — the same, one axis
//!   over: two edges carrying one meridian column disagree in u. Lever:
//!   [`Chart::radial`] at the junction between them.
//!
//! # What a finding is, and is not
//!
//! It is a MEASUREMENT, never a verdict about validity, and in
//! particular it is not a tier: a body can be tier-3 green and carry
//! findings, or tier-3 refused and carry none. Both halves of that are
//! in tree, on the two π-rad witnesses. Issue 723's imported half-cap
//! (`step-import/tests/fixtures/halfcap/halfcap_eps7.step`) is tier-3
//! GREEN and reports; issue 1571's Euler-door body is tier-3 REFUSED
//! — twice for `ScaffoldAtRest` and once for `CurvedSenseInverted`,
//! construction artefacts of how it is assembled, none of them the arc
//! this examination is about — and reports the same half-turn anyway.
//! Nothing here refuses, nothing here is a tier, and no consumer is
//! required to react.
//!
//! # No shape door in front of it, and what that means
//!
//! `mesh` reaches its walk only through `props`' iso-rectangle door;
//! this examination has no door at all, deliberately — a report that
//! only looked at faces the tessellator already accepted could not
//! report on the faces most likely to be defective. Two consequences
//! a reader owes:
//!
//! - it reads faces the iso walk never walks, and states its
//!   conditions about them. On such a face the conditions are still
//!   well-defined (a carrier's midpoint azimuth against its own
//!   endpoint's is a fact about any circle and any vertex), but they
//!   are not statements about a mesh that exists — no lane consumed
//!   those coordinates and none will;
//! - the classification it reads them through is the WALK's
//!   ([`crate::chart_iso`]), so on a face whose boundary is not a
//!   chart iso curve the classification is a nearest-fit and a finding
//!   may be measuring a premise the face never had. Where the carrier
//!   is a conic or a spline that is visible — the loop is
//!   [`Unexaminable::NonIsoCarrier`] — and where it is a line or a
//!   circle that happens not to be iso, it is not.
//!
//! **Not covered by D9's byte-identity contract.** That contract is
//! about the MESH: the same body at the same δ yields the same
//! triangles, byte for byte. This report is a different value from a
//! different door, and `mesh::tessellate` neither calls it nor carries
//! it. What this report has instead is its own determinism, stated at
//! [`examine_chart_coherence`]: it is a pure function of (body, ε).
//!
//! # The ledger, and where the rest of it is
//!
//! **THE BAR IS SPATIAL, NOT ANGULAR, and one traced file is the whole
//! argument for that.** `nist_ftc_09_asme1_rd.stp` states its hole
//! generators' two endpoints non-co-azimuthally in the tenth digit:
//! 21.4 pm perpendicular to the axis, which subtends 7.2e-9 rad at
//! r = 2.9718e-3 m. In radians that is a large number on a small
//! feature and a scale-free radian bar flagged it; in metres it is
//! 21 pm, 1.6e6x inside that file's own ε, and nothing is reported.
//! A fixed source-coordinate error subtends a LARGER angle on a
//! SMALLER feature, so only the metres are comparable to ε.
//!
//! **The two π-rad witnesses**, both a meridian ARC crossing a pole
//! mid-edge so the carrier's midpoint sits a half-turn from its own
//! endpoint: issue 723's half-cap through import — the committed
//! fixture above, whose offending endpoint is 1.0e-9 m from the axis,
//! so the half-turn opens 3.14 nm and the report is band-shaped across
//! the ε rows — and issue 1571's Euler-door body. Issue 1571 owns
//! FIXING the arc premise; this door owns seeing it.
//!
//! **What nothing re-measures.** No test in this repo examines the
//! wild or FreeCAD corpora, so the input class these conditions are
//! about is untested by construction; "nothing else on source data has
//! been seen above 3.6e-9 rad" is a trace, not a census. The rest of
//! this deviation's history — what these conditions were before they
//! were a report, and the ledger that justified them there — is in the
//! PR that relocated them (issue 868) and in `docs/S-MESH-LOG.md`,
//! not here.

use geom_core::{Point3, Tol};

use crate::body::Body;
use crate::chart::Chart;
use crate::chart_iso::{TravKind, classify_kind, unwrap_near};
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
    ///
    /// **Per FINDING, not per report**, and the eight repeated bytes
    /// are the price of the decision: a finding is a value that
    /// travels — into a report, into a check arm, into a log line, on
    /// its own — and `metres` read without the band it was judged
    /// against is a number without a claim. Putting ε on the container
    /// would make every finding depend on its container to mean
    /// anything, which is the property this type is trying not to
    /// have. Every finding of one call carries the same value, by
    /// construction: [`examine_chart_coherence`] reads ε once.
    pub eps: f64,
}

/// Why one loop of a chart-bearing face was not examined.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unexaminable {
    /// The loop's structure did not resolve. Tier 1's business, named
    /// here rather than dropped: this door reports coherence, and
    /// re-reporting a structural defect in a second vocabulary would
    /// give it a second home.
    Corrupt {
        /// Which read failed — a closed set, so a consumer can match
        /// on it, rather than a `&'static str` a consumer can only
        /// print.
        at: StructureRead,
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

/// Which structural read failed on a loop this door could not walk.
///
/// One variant per fallible arena read the traversal makes, in the
/// order it makes them. A closed enum rather than a message: every
/// one of these is a tier-1 defect with a name of its own over in
/// [`mod@crate::validate`], and a consumer routing this to that vocabulary
/// needs to match, not to parse.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StructureRead {
    /// The loop key does not resolve.
    Loop,
    /// The loop carries no cycle — construction scaffolding at rest.
    EmptyLoop,
    /// The half-edge cycle from the loop's first half-edge is broken.
    Cycle,
    /// A half-edge key in the cycle does not resolve.
    HalfEdge,
    /// A half-edge's edge key does not resolve.
    Edge,
    /// An edge's curve key does not resolve.
    Curve,
    /// A half-edge's mate does not resolve, so the traversal has no
    /// end vertex.
    Mate,
    /// A junction vertex, or its point, does not resolve.
    VertexPoint,
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
/// at.
///
/// Two lists rather than one, for the reason `editor_core`'s
/// `ChecksReport` has two: a report that lists only its findings
/// cannot be told apart from one that had nothing to look at.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CoherenceReport {
    /// Every finding, in the order described at
    /// [`examine_chart_coherence`].
    pub findings: Vec<CoherenceFinding>,
    /// Every loop of a chart-bearing face this door could not walk, in
    /// the same traversal order.
    ///
    /// **Not `ChecksReport::skipped`, and the parallel is only in the
    /// shape.** That field lists checks a CONFIGURATION turned off —
    /// a choice, reversible by changing the configuration. This one
    /// lists loops the DATA put out of reach: nothing a caller can set
    /// makes them examinable, and each entry is a defect or a lane
    /// boundary rather than a preference. A consumer that renders the
    /// two the same way is telling its user something false about what
    /// to do next.
    pub unexamined: Vec<Unexamined>,
}

/// The unsigned distance between two values of a PERIODIC chart
/// coordinate, through the walk's OWN branch selection
/// ([`unwrap_near`]): `a`'s nearest representative to `b`, minus `b`.
/// The answer never exceeds half a period.
///
/// This is the same reduction the consuming walk performs before it
/// discards one of the two values — literally the same function, so a
/// change to how the walk picks a branch changes what this measures —
/// read here as a measurement rather than as a selection: on a full
/// turn of skew the two statements agree, and a report that called
/// that a π gap would be reporting the reduction and not the data.
fn wrapped(a: f64, b: f64) -> f64 {
    (unwrap_near(a, b) - b).abs()
}

/// **The band, and its only spelling in this crate.** Is a chart
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
///
/// # The same expression as `mesh`'s, and why there are two of them
///
/// `mesh::walk::gap_is_noise` is `Eps::dominates(gap * lever)`, which
/// is this comparison with this band edge. It is not shared, and that
/// is a decision rather than an oversight: MESH-4 made `Eps` a
/// `mesh`-local newtype whose whole property is that it has no
/// accessor, so its band cannot cross a crate boundary as a number
/// and the two spellings cannot be one function without giving it one.
/// What holds them together instead is an executed row rather than a
/// promise — `mesh::walk::tests::the_two_spellings_of_the_band_agree`
/// runs the walk's own literal ladder through both and reds if either
/// drifts.
pub fn gap_is_noise(gap: f64, lever: f64, eps: f64) -> bool {
    gap * lever < eps
}

/// A boundary edge as this examination reads it: the classification
/// the source states about it, and the two junctions it runs between.
///
/// [`TravKind`] is shared with the walk; this struct is not, and the
/// fields are why — the walk's own traversal carries a chord-id list
/// into a mesh, and there is no mesh here. Arena keys and the two
/// junction POINTS are what a body-side reading has.
struct Trav {
    edge: EdgeKey,
    start: VertexKey,
    end: VertexKey,
    start_p: Point3<f64>,
    end_p: Point3<f64>,
    kind: TravKind,
}

/// Reads one loop into the traversal list the conditions are stated
/// over, or names why it could not be read.
fn traversals(body: &Body<f64>, chart: &Chart, lk: LoopKey) -> Result<Vec<Trav>, Unexaminable> {
    let corrupt = |at| Unexaminable::Corrupt { at };
    let point = |body: &Body<f64>, v: VertexKey| -> Option<Point3<f64>> {
        body.get_vertex(v)
            .and_then(|vx| body.get_point(vx.point))
            .copied()
    };
    let lp = body.get_loop(lk).ok_or(corrupt(StructureRead::Loop))?;
    let LoopBoundary::Cycle { first } = lp.boundary else {
        return Err(corrupt(StructureRead::EmptyLoop));
    };
    let cycle: Vec<HalfEdgeKey> = body
        .loop_cycle(first)
        .ok_or(corrupt(StructureRead::Cycle))?;
    let mut out = Vec::with_capacity(cycle.len());
    for hek in cycle {
        let he = body
            .get_half_edge(hek)
            .ok_or(corrupt(StructureRead::HalfEdge))?;
        let edge = body.get_edge(he.edge).ok_or(corrupt(StructureRead::Edge))?;
        let curve = body
            .get_curve_geom(edge.curve)
            .ok_or(corrupt(StructureRead::Curve))?
            .certified()
            .ok_or(Unexaminable::NullScaffoldEdge { edge: he.edge })?;
        let end = body
            .half_edge_end(hek)
            .ok_or(corrupt(StructureRead::Mate))?;
        let start_p = point(body, he.start).ok_or(corrupt(StructureRead::VertexPoint))?;
        let end_p = point(body, end).ok_or(corrupt(StructureRead::VertexPoint))?;
        let kind =
            classify_kind(chart, curve).ok_or(Unexaminable::NonIsoCarrier { edge: he.edge })?;
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
    // The RULE is `chart_iso::iso_side_starts`; what is here is this
    // door's band read (`radial > eps`, strict, the band excluded —
    // the comparison `mesh` spells `Eps::separates`) and this door's
    // resolution of the case the rule deliberately leaves open.
    let kinds: Vec<TravKind> = travs.iter().map(|t| t.kind).collect();
    let separated: Vec<bool> = travs
        .iter()
        .map(|t| chart.radial(t.start_p) > eps)
        .collect();
    let mut starts = crate::chart_iso::iso_side_starts(&kinds, &separated);
    if !starts.is_empty() && !starts.iter().any(|&s| s) {
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
    let poles = chart.poles();
    let mut out = Vec::new();
    let mut push = |edge: EdgeKey, condition: CoherenceCondition, gap: f64, lever: f64| {
        if !gap_is_noise(gap, lever, eps) {
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
                // A POLE ENDPOINT IS EXEMPT, by the walk's own rule.
                // Where a junction lies within ε of a chart pole the
                // walk identifies it with that pole: it substitutes
                // the pole's exact v and emits a fan, and it never
                // reads the junction's azimuth at all — so there is
                // no second account of u there for this condition to
                // compare against. What `u_of` returns at such a point
                // is an `atan2` of two quantities that are both noise;
                // measuring a carrier against it reports the noise.
                //
                // The band is INCLUSIVE here because it is inclusive
                // in the walk (`Eps::coincident`): the two doors must
                // identify the same set of junctions with the same
                // poles, or this report contradicts the mesh about
                // which points have an azimuth. Measured cost of NOT
                // exempting: at ε = 1e-12 a tilted-axis sphere of
                // R ≳ 1.4 km puts its own pole vertex ~R·ulp off the
                // analytic pole, a lever of ~1e-10 m against an
                // arbitrary gap — a finding about float placement, on
                // a body the walk meshes without a word.
                if poles.iter().any(|&(pp, _)| (p - pp).norm() <= eps) {
                    continue;
                }
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
/// # This door decides nothing, and today nothing calls it
///
/// It has no failure mode, no tier, and no verdict. It refuses no
/// body, it changes no body, and no operation in this kernel consults
/// what it returns — **on the day it lands it has zero production
/// callers**, only tests. That is the honest state and it is stated
/// rather than implied: the two consumers it was shaped for are
/// scheduled, not built (issue 1587 — `editor_core`'s checks surface,
/// and `step-import`'s diagnostics at the door defective coordinates
/// actually arrive at). A door nobody walks through reports nothing to
/// anybody; what it does buy today is that the three conditions are
/// stated where they are true, and a tessellator no longer panics
/// about them. A body that meshes, validates, or exports today
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
                    at: StructureRead::Loop,
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

    // The band's LADDER — the spatial-vs-angular rows, the
    // lever-arm scaling and the zero-lever limit — is not restated
    // here: it has one home, `mesh::walk`'s test module, where the
    // predicate it belongs to has always lived, and
    // `the_two_spellings_of_the_band_agree` runs that same ladder
    // through this crate's spelling too. What is below is what is
    // this module's own.

    /// The band is EXCLUDED at both ends it can be excluded at: a zero
    /// band calls nothing noise (so a zero-lever gap is reported rather
    /// than swallowed), and a gap exactly ON the band is reported. The
    /// two edges together are what make "every finding satisfies
    /// `metres >= eps`" true as written.
    #[test]
    fn the_band_is_excluded_at_both_edges() {
        assert!(
            !gap_is_noise(1.0, 0.0, 0.0),
            "a zero band dominates nothing"
        );
        assert!(
            !gap_is_noise(1.0, 1e-9, 1e-9),
            "exactly on the band is reported"
        );
        assert!(
            gap_is_noise(1.0, 1e-9, 1.000_001e-9),
            "just inside it is not"
        );
    }

    /// A NaN coordinate is never noise: a poisoned carrier surfaces as
    /// a finding rather than passing as quiet, which is the direction
    /// this comparison has to fail in.
    #[test]
    fn a_poisoned_gap_is_not_noise() {
        assert!(!gap_is_noise(f64::NAN, 1.0, 1e-9));
        assert!(!gap_is_noise(1.0, f64::NAN, 1e-9));
    }

    /// A periodic coordinate's disagreement is measured after
    /// reduction, so a whole number of turns is AGREEMENT and the
    /// answer never exceeds half a period. Reporting the reduction
    /// itself would be reporting the branch and not the data.
    #[test]
    fn a_whole_turn_of_skew_is_no_disagreement() {
        for turns in [-2.0_f64, -1.0, 1.0, 2.0] {
            let a = 0.7 + turns * crate::chart_iso::TAU;
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
