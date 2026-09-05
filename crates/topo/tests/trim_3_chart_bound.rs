//! **TRIM-3 PR-1 acceptance — the chart-boundary description and its
//! outside test** (`topo::chart_bound`, `topo::chart_boundary`).
//!
//! Rows T1–T9 of `docs/TRIM-3-SPEC.md` §4. The stage-(1)/stage-(2)
//! rows run on hand-built [`ChartBound`]s at the interval scalar — the
//! description is a value, and building it by hand is what lets a row
//! state the geometry it is about instead of a construction that
//! happens to produce it. T7 and T9 exercise the MINTING door on real
//! bodies, because what they assert is a refusal of the loop walk.
//!
//! ε posture: no ε literal. T4's three cells are stated as multiples
//! of the run's own band (`band.zero()`, `band.escalate()`) and
//! asserted with definite outcomes — there is no ε-conditional early
//! return anywhere in this file.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{Curve3, NurbsCurve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::spline::KnotVector;
use geom_core::{Band, Bounds, Interval, Point2, Point3, Real, SpanLocate, Tol, Vec3};
use topo::{
    Body, ChartBound, ChartEdge, ChartLoop, FaceSurface, LoopBoundary, MefSite, MetredRect,
    MevSite, PcurveCertifyError, PcurveMintError, chart_boundary,
};

use crate::common;

// ---------------------------------------------------------------- //
// Vocabulary
// ---------------------------------------------------------------- //

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

fn p2(u: f64, v: f64) -> Point2<Interval> {
    Point2::new(iv(u), iv(v))
}

/// A closed chord polygon of exact-structural segments.
fn polygon(verts: &[(f64, f64)], ring: bool) -> ChartLoop<Interval> {
    let n = verts.len();
    ChartLoop {
        edges: (0..n)
            .map(|i| {
                let (a, b) = (verts[i], verts[(i + 1) % n]);
                ChartEdge::Segment {
                    a: p2(a.0, a.1),
                    b: p2(b.0, b.1),
                }
            })
            .collect(),
        ring,
    }
}

/// The L: unit square with the quadrant `u > 0.5, v > 0.5` notched
/// out. Every coordinate dyadic, so no vertex carries rounding.
const L_SHAPE: [(f64, f64); 6] = [
    (0.0, 0.0),
    (1.0, 0.0),
    (1.0, 0.5),
    (0.5, 0.5),
    (0.5, 1.0),
    (0.0, 1.0),
];

/// The L as a metred bound at the plane's exact arms `(1, 1)`.
fn l_bound() -> topo::MetredBound<Interval> {
    ChartBound::assembled(polygon(&L_SHAPE, false), Vec::new(), None)
        .metred((Interval::one(), Interval::one()))
}

// ---------------------------------------------------------------- //
// T1–T4, T8 — the outside test on the L
// ---------------------------------------------------------------- //

/// **T1** — a cell deep in the L's notch is certified outside.
/// Kills: a test that drops nothing.
#[test]
fn t1_a_cell_deep_in_the_notch_is_certified_outside() {
    assert!(
        l_bound().certifies_outside(MetredRect::new(0.7, 0.8, 0.7, 0.8), band()),
        "the notch is not material: a cell well inside it must certify"
    );
}

/// **T2** — a cell inside the material is not certified.
/// Kills: an inverted parity sense.
#[test]
fn t2_a_cell_inside_the_material_is_not_certified() {
    assert!(
        !l_bound().certifies_outside(MetredRect::new(0.1, 0.2, 0.1, 0.2), band()),
        "a cell inside the face must never certify outside"
    );
}

/// **T3** — a cell overlapping the notch wall, whose CENTRE is
/// outside, is not certified: stage (1) is what refuses it, and the
/// centre would have said outside.
/// Kills: skipping the separating-axis test and running parity only.
#[test]
fn t3_a_cell_overlapping_an_edge_is_not_certified_although_its_centre_is_outside() {
    let bound = l_bound();
    // The premise: the centre alone says outside.
    assert!(
        bound.certifies_outside(MetredRect::point(0.55, 0.75), band()),
        "premise: the cell's centre is outside the L"
    );
    assert!(
        !bound.certifies_outside(MetredRect::new(0.45, 0.65, 0.7, 0.8), band()),
        "a cell straddling the notch wall meets the boundary and must be kept"
    );
}

/// **T4** — three cells off one edge at `0.5ε`, `3ε` and `20ε`
/// (`K = 10`): kept, kept, dropped. Every outcome definite.
/// Kills: a raw comparison, and any minted widening constant.
#[test]
fn t4_the_epsilon_scale_cells_off_one_edge_decide_through_the_band() {
    let band = band();
    let (eps, escalate) = (band.zero(), band.escalate());
    // The row's three numbers are K = 10's. A run configured
    // otherwise fails here loudly rather than asserting a different
    // claim under this row's name.
    assert!(
        (escalate / eps - 10.0).abs() < 1e-9,
        "T4's cells are stated at the ratified K = 10; this run has K = {}",
        escalate / eps
    );
    let bound = l_bound();
    let cell = |gap: f64| MetredRect::new(0.5 + gap, 0.55 + gap, 0.7, 0.8);
    assert!(
        !bound.certifies_outside(cell(0.5 * eps), band),
        "a gap of ε/2 reads Zero on the wall: the cell is KEPT"
    );
    assert!(
        !bound.certifies_outside(cell(3.0 * eps), band),
        "a gap of 3ε is in the ambiguity band: the cell is KEPT"
    );
    assert!(
        bound.certifies_outside(cell(20.0 * eps), band),
        "a gap of 20ε = 2·K·ε is a definite separation: the cell is DROPPED"
    );
}

/// **T8** — a grazing centre: an edge vertex on each of the two
/// schedule members' ray lines. The parent is not certified; both
/// children, whose centres graze only the first member, are.
/// Kills: hiding the single-ray delay (treating a graze as a verdict).
#[test]
fn t8_a_grazing_centre_is_not_certified_and_its_children_are() {
    // The L again, with one extra vertex on the bottom edge at
    // u = 1.5 and one on the left edge at v = 1.5 — the two the
    // parent cell's centre grazes.
    let verts = [
        (0.0, 0.0),
        (1.5, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
        (0.0, 1.5),
    ];
    let bound = ChartBound::assembled(polygon(&verts, false), Vec::new(), None)
        .metred((Interval::one(), Interval::one()));
    let b = band();
    assert!(
        !bound.certifies_outside(MetredRect::new(1.2, 1.8, 1.2, 1.8), b),
        "both schedule members graze at (1.5, 1.5): no verdict, so the cell is kept"
    );
    for child in [
        MetredRect::new(1.2, 1.5, 1.2, 1.8),
        MetredRect::new(1.5, 1.8, 1.2, 1.8),
    ] {
        assert!(
            bound.certifies_outside(child, b),
            "the child's centre grazes only the +u member; the +v member answers: {child:?}"
        );
    }
}

// ---------------------------------------------------------------- //
// T5 — ring copies
// ---------------------------------------------------------------- //

/// **T5** — a ring is lifted by every whole-period shift, and a cell
/// inside EITHER the `k = 0` copy or the `k = -1` lift is dropped: the
/// two the spec's row names, at the two cells it names.
/// Kills: emitting a single ring copy.
#[test]
fn t5_a_ring_is_lifted_by_every_whole_period_shift() {
    let outer = polygon(&[(-3.0, 0.0), (3.2, 0.0), (3.2, 2.0), (-3.0, 2.0)], false);
    let ring = polygon(&[(3.1, 0.5), (3.4, 0.5), (3.4, 1.5), (3.1, 1.5)], true);
    let bound = ChartBound::assembled(outer, vec![ring], Some(Interval::tau()));
    assert_eq!(
        bound.loops.len(),
        4,
        "the outer loop plus the ring lifted at k = -1, 0 and +1"
    );
    let metred = bound.metred((Interval::one(), Interval::one()));
    let b = band();
    assert!(
        metred.certifies_outside(MetredRect::new(-2.96, -2.94, 0.9, 1.1), b),
        "the cell at u = -2.95 lies in the ring's k = -1 lift: a hole, so off the face"
    );
    assert!(
        metred.certifies_outside(MetredRect::new(3.13, 3.17, 0.9, 1.1), b),
        "the cell at u = 3.15 lies in the ring's k = 0 copy"
    );
    assert!(
        !metred.certifies_outside(MetredRect::new(0.0, 0.1, 0.9, 1.1), b),
        "a cell inside the outer loop and in no ring copy is material"
    );
}

// ---------------------------------------------------------------- //
// T6 — the envelope's slack
// ---------------------------------------------------------------- //

/// **T6** — a `General`-shaped envelope edge: `metred` widens the
/// control-hull box by the stored certificate's envelope (metres), so
/// a cell inside the widening is kept and one outside it is dropped.
/// Kills: using the control hull bare.
#[test]
fn t6_an_envelope_box_is_widened_by_its_certificate_slack() {
    let slack = 0.01_f64;
    let outer = ChartLoop {
        edges: vec![
            ChartEdge::Segment {
                a: p2(0.0, 0.0),
                b: p2(2.0, 0.0),
            },
            // The control hull of a general image bulging to u = 2.1.
            ChartEdge::Envelope {
                a: p2(2.0, 0.0),
                b: p2(2.0, 2.0),
                image: Point2::new(
                    iv(2.0).enclosure_hull(iv(2.1)),
                    iv(0.0).enclosure_hull(iv(2.0)),
                ),
                slack: iv(slack),
            },
            ChartEdge::Segment {
                a: p2(2.0, 2.0),
                b: p2(0.0, 2.0),
            },
            ChartEdge::Segment {
                a: p2(0.0, 2.0),
                b: p2(0.0, 0.0),
            },
        ],
        ring: false,
    };
    // A cylinder chart's exact arms: `(r, 1)` with r = 2, so the
    // chart's u = 2.1 is 4.2 metres and the metre slack widens it to
    // 4.21 — a chart-space widening of `envelope / arm`.
    let bound = ChartBound::assembled(outer, Vec::new(), Some(Interval::tau()))
        .metred((iv(2.0), Interval::one()));
    let b = band();
    assert!(
        !bound.certifies_outside(MetredRect::new(4.205, 4.3, 0.5, 1.5), b),
        "a cell 0.005 m past the bare hull is inside the certificate's slack: kept"
    );
    assert!(
        bound.certifies_outside(MetredRect::new(4.22, 4.3, 0.5, 1.5), b),
        "a cell 0.01 m past the WIDENED box separates definitely: dropped"
    );
}

// ---------------------------------------------------------------- //
// T7 — the wrap fence, on a real body
// ---------------------------------------------------------------- //

fn unit_cylinder() -> Surface<Interval> {
    Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
        radius: Interval::one(),
        u_ref: Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
    }
}

fn base_plane() -> Surface<Interval> {
    Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
        u_ref: Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
    }
}

/// A half-turn rim arc of the unit circle in `z = 0`, starting at
/// azimuth `start`.
fn half_arc(start: f64) -> Curve3<Interval> {
    Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
        radius: Interval::one(),
        u_ref: Vec3::new(iv(start.cos()), iv(start.sin()), iv(0.0)),
    }
}

/// **T7** — two half-turn rim arcs make a loop whose chart walk closes
/// one whole period off. The walk accepts it (the seam arm); the
/// description refuses it, because the chord polygon of an open lift
/// bounds a region the face does not have.
/// Kills: describing the polygon of an open lift.
#[test]
fn t7_a_loop_that_wraps_the_chart_refuses() {
    let pi = core::f64::consts::PI;
    let mut body = Body::<Interval>::new();
    let p0 = Point3::new(iv(1.0), iv(0.0), iv(0.0));
    let p1 = Point3::new(iv(-1.0), iv(0.0), iv(0.0));
    let seed = body.mvfs(p0).unwrap();
    let cyl = body
        .set_face_surface(seed.face, FaceSurface::New(unit_cylinder()))
        .unwrap();
    // A second seed carries the plane the rim arcs' description names;
    // it takes part in no loop of the face under test.
    let anchor = body.mvfs(Point3::origin()).unwrap();
    let pln = body
        .set_face_surface(anchor.face, FaceSurface::New(base_plane()))
        .unwrap();
    let spec = |start: f64| {
        let carrier = half_arc(start);
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                s1: cyl,
                s2: pln,
                witness: carrier.eval(iv(pi / 2.0)),
            },
            carrier,
            param_start: iv(0.0),
            param_end: iv(pi),
        }
    };
    let made = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p1,
            spec(0.0),
            Tol::witness(),
        )
        .expect("the first rim arc certifies");
    let edge = body.get_edge(made.edge).unwrap();
    let (he_plus, he_minus) = (edge.he_plus, edge.he_minus);
    let split = body
        .mef(
            // The new edge runs `start(he_minus) → start(he_plus)`,
            // i.e. the second half turn back to the seed vertex.
            MefSite::Chords {
                he1: he_minus,
                he2: he_plus,
            },
            spec(pi),
            FaceSurface::Inherit,
            Tol::witness(),
        )
        .expect("the second rim arc closes the turn");
    let chart = unit_cylinder();
    let mut wraps = 0usize;
    for face in [seed.face, split.face] {
        match chart_boundary(&body, face, &chart, band()) {
            Err(PcurveMintError::LoopWraps { .. }) => wraps += 1,
            Err(other) => {
                // The other face's walk may refuse earlier (its first
                // half-edge is traversed backwards, so its branch
                // anchor differs); what must never happen is a
                // description.
                eprintln!("t7: face refused before the wrap fence: {other}");
            }
            Ok(_) => panic!("a wrapping loop must never describe a chart polygon"),
        }
    }
    assert!(
        wraps >= 1,
        "at least one of the two lune faces must refuse with LoopWraps"
    );
}

// ---------------------------------------------------------------- //
// T9 — a carrier kind the chart cannot image
// ---------------------------------------------------------------- //

/// The prism's face surface, cloned.
fn face_chart(body: &Body<Interval>, face: topo::FaceKey) -> Surface<Interval> {
    let key = body.get_face(face).unwrap().surface;
    body.get_surface(key).unwrap().clone()
}

/// **T9** — a `Curve3::Nurbs` carrier on a PLANE chart has no
/// closed-form image, and the refusal propagates out of
/// `chart_boundary` typed rather than being swallowed into an empty
/// bound. The same test states the positive premise first: an
/// untouched planar face describes.
/// Kills: swallowing the refusal into an empty description.
#[test]
fn t9_a_nurbs_carrier_on_a_plane_chart_refuses_typed() {
    let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let built = common::prism::<Interval>(&square, 1.0);
    let mut body = built.body;
    let face = built.bottom_face;
    let chart = face_chart(&body, face);

    // Premise: the untouched planar face describes as four structural
    // segments, and its description answers the outside test.
    let described = chart_boundary(&body, face, &chart, band()).expect("a planar face describes");
    assert_eq!(described.loops.len(), 1, "one outer loop, no rings");
    assert_eq!(described.loops[0].edges.len(), 4, "four straight edges");
    assert!(
        described.loops[0]
            .edges
            .iter()
            .all(|e| matches!(e, ChartEdge::Segment { .. })),
        "a line carrier on a plane chart is a segment BY STRUCTURE"
    );
    let metred = described.metred((Interval::one(), Interval::one()));
    let b = band();
    assert!(
        metred.certifies_outside(MetredRect::new(100.0, 101.0, 100.0, 101.0), b),
        "a cell far from the face certifies outside"
    );
    let centre = {
        let vs: Vec<Point2<Interval>> = described.loops[0].edges.iter().map(ChartEdge::a).collect();
        let n = f64::from(u32::try_from(vs.len()).unwrap());
        let sum = vs
            .iter()
            .fold(Point2::new(Interval::zero(), Interval::zero()), |acc, p| {
                Point2::new(acc.x + p.x, acc.y + p.y)
            });
        (
            0.5 * (sum.x.lo() + sum.x.hi()) / n,
            0.5 * (sum.y.lo() + sum.y.hi()) / n,
        )
    };
    assert!(
        !metred.certifies_outside(MetredRect::point(centre.0, centre.1), b),
        "the face's own centre is material"
    );

    // Now swap one carrier for a degree-1 NURBS along the same chord —
    // the same locus, a kind the plane chart has no image for.
    let edge_key = {
        let lp = body.get_face(face).unwrap().outer;
        let LoopBoundary::Cycle { first } = body.get_loop(lp).unwrap().boundary else {
            panic!("the outer loop is a cycle")
        };
        body.get_half_edge(first).unwrap().edge
    };
    let edge = body.get_edge(edge_key).unwrap().clone();
    let face_of = |body: &Body<Interval>, he| {
        let lp = body.get_half_edge(he).unwrap().parent_loop;
        body.get_loop(lp).unwrap().face
    };
    let s1 = body.get_face(face_of(&body, edge.he_plus)).unwrap().surface;
    let s2 = body
        .get_face(face_of(&body, edge.he_minus))
        .unwrap()
        .surface;
    let start = body.get_half_edge(edge.he_plus).unwrap().start;
    let end = body.half_edge_end(edge.he_plus).unwrap();
    let q0 = *body
        .get_point(body.get_vertex(start).unwrap().point)
        .unwrap();
    let q1 = *body.get_point(body.get_vertex(end).unwrap().point).unwrap();
    let knots = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let spline = NurbsCurve3::new(knots, vec![q0, q1], vec![1.0, 1.0])
        .expect("the chord as a degree-1 spline");
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            s1,
            s2,
            witness: q0.lerp(q1, iv(0.5)),
        },
        carrier: Curve3::Nurbs(Arc::new(spline)),
        param_start: iv(0.0),
        param_end: Interval::one(),
    };
    body.set_edge_curve(edge_key, spec, Tol::witness())
        .expect("a rung-3 carrier under an Intersection description certifies");

    let err = chart_boundary(&body, face, &chart, band())
        .expect_err("a Nurbs carrier has no closed-form plane-chart image");
    assert!(
        matches!(
            err,
            PcurveMintError::Certify {
                error: PcurveCertifyError::UnsupportedCarrier,
                ..
            }
        ),
        "the refusal must name the carrier kind it cannot image: {err}"
    );
}
