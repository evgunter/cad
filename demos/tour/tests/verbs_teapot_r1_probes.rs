//! VERBS-TEAPOT review probes (R1, ordinal 100) — unique-signal
//! verification of PR #1078's two defect claims (#1081, #1082), on
//! fixtures OUTSIDE the PR's own enumeration, plus a geometric
//! re-derivation of what makes the opened body WRONG (rather than
//! correct topology mis-documented).
//!
//! Review-lane only; not part of the PR under review.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::authoring::{p2, validated};
use pncad::geom::{Curve3, Surface};
use pncad::geom_core::{Band, Point2, Point3, Tol, Vec2};
use pncad::prelude::{Open, Start};
use pncad::profile::{ProfileLoop, SketchPlane};
use pncad::sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use pncad::topo::{Body, FaceKey, LoopBoundary, ReplaceFaceError, ShellError};

const FIT_TOL: f64 = 1e-6;

fn band(tol: Tol) -> Band {
    Band::linear(tol).expect("the run's band")
}

fn revolved(lp: ProfileLoop<f64>, tol: Tol) -> Body<f64> {
    revolve(
        &validated(SketchPlane::xy(), vec![lp], tol).expect("the meridian validates"),
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol,
    )
    .expect("the meridian fully revolves")
    .body
}

fn extruded(lp: ProfileLoop<f64>, h: f64, tol: Tol) -> Body<f64> {
    extrude(
        &validated(SketchPlane::xy(), vec![lp], tol).expect("the footprint validates"),
        Extrusion::Distance(h),
        tol,
    )
    .expect("the footprint extrudes")
    .body
}

/// MY fixture for #1082 — not the PR's drum, not the teapot meridian:
/// a squared vase (wide base, narrow neck), stations, wall thickness
/// (0.004) and chord budget (5e-4) all outside the PR's sweep.
const VASE_TOP: f64 = 0.1;
const VASE_NECK: f64 = 0.03;
fn vase(tol: Tol) -> Body<f64> {
    revolved(
        Open.at(Point2::new(0.0, 0.0))
            .line_to(Point2::new(0.05, 0.0), tol)
            .expect("base")
            .line_to(Point2::new(0.05, 0.04), tol)
            .expect("lower wall")
            .line_to(Point2::new(VASE_NECK, 0.04), tol)
            .expect("shoulder")
            .line_to(Point2::new(VASE_NECK, VASE_TOP), tol)
            .expect("neck")
            .line_to(Point2::new(0.0, VASE_TOP), tol)
            .expect("mouth")
            .line_to(Start, tol)
            .expect("axis")
            .into(),
        tol,
    )
}

/// The point a vertex sits at.
fn at(body: &Body<f64>, v: pncad::topo::VertexKey) -> Point3<f64> {
    *body
        .get_point(body.get_vertex(v).expect("vertex").point)
        .expect("point")
}

/// The (start, end) points of every edge of loop `lk`.
fn loop_edge_ends(body: &Body<f64>, lk: pncad::topo::LoopKey) -> Vec<(Point3<f64>, Point3<f64>)> {
    let lp = body.get_loop(lk).expect("loop");
    let LoopBoundary::Cycle { first } = lp.boundary else {
        panic!("an empty loop where a cycle was expected");
    };
    body.loop_cycle(first)
        .expect("cycle")
        .into_iter()
        .map(|he| {
            let start = body.get_half_edge(he).expect("he").start;
            let end = body.half_edge_end(he).expect("end");
            (at(body, start), at(body, end))
        })
        .collect()
}

/// Every carrier curve of loop `lk`'s edges.
fn loop_carriers(body: &Body<f64>, lk: pncad::topo::LoopKey) -> Vec<Curve3<f64>> {
    let lp = body.get_loop(lk).expect("loop");
    let LoopBoundary::Cycle { first } = lp.boundary else {
        panic!("an empty loop where a cycle was expected");
    };
    body.loop_cycle(first)
        .expect("cycle")
        .into_iter()
        .map(|he| {
            let e = body
                .get_edge(body.get_half_edge(he).expect("he").edge)
                .expect("edge");
            body.get_curve_geom(e.curve)
                .and_then(|g| g.certified())
                .expect("certified carrier")
                .carrier()
                .clone()
        })
        .collect()
}

fn rings(body: &Body<f64>) -> usize {
    body.faces().map(|(_, f)| f.rings.len()).sum()
}

/// **P1 — #1082's fix, re-derived on the same fixture that first
/// measured the defect** (a squared vase on stations, wall thickness
/// and chord budget all outside the PR's sweep, kept verbatim).
///
/// What this row measured before, and pinned as the defect: each mouth
/// half-disc came back carrying a ring that was the cavity
/// counterpart's own half-disc boundary — an inner half-circle plus
/// two radial legs meeting a ring vertex AT the axis point the OUTER
/// loop already owned, running back along the outer loop's own seam
/// legs. An interior ring that meets its outer loop is not a valid
/// trim of any face, which is why the CDT refused; the correct
/// topology is the mouth annulus, simply connected once its hole is a
/// hole, Euler characteristic 2, genus 0 — which is also what
/// `topo::shell`'s module docs promise ("one opening gives a cup,
/// which is genus 0").
///
/// It now reads that way, and the row checks the SAME three things it
/// used to falsify, in the positive direction:
///
/// 1. the mouth plane is worn by ONE face, carrying exactly one ring —
///    the revolve's seam split is retired before the glue rather than
///    inherited into the rim;
/// 2. that ring is the full circle of radius `neck − t` about the
///    axis, and it shares NO vertex position and NO edge with the
///    face's outer loop (the contact that made the old body wrong);
/// 3. the Euler-Poincare census over the returned arenas reads genus
///    0, and the body triangulates at the same 5e-4 budget that used
///    to refuse.
#[test]
fn p1_shell_open_is_a_disjoint_ring_on_my_own_revolve() {
    let tol = Tol::witness();
    let t = 0.004;
    let body = vase(tol);
    let mouth: Vec<FaceKey> = body
        .faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(Surface::Plane { origin, .. }) if (origin.y - VASE_TOP).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(mouth.len(), 2, "a full revolve's cap is two half-discs");

    let cup = pncad::topo::shell_open(&body, t, &mouth, FIT_TOL, band(tol), tol)
        .expect("the opened arm returns a body on my vase too");

    // Tiers 1-3 still bless it — and tier 3 now also carries the
    // ring-vs-outer invariant the old body violated.
    assert!(pncad::topo::validate(&cup).is_ok(), "tier 1 passes");
    assert!(pncad::topo::validate_closed(&cup).is_ok(), "tier 2 passes");
    assert_eq!(
        pncad::topo::validate_geometric(&cup, tol),
        Ok(()),
        "tier 3 passes"
    );

    let r_ring = VASE_NECK - t;
    let axis_pt = Point3::new(0.0, VASE_TOP, 0.0);
    let dist_pt = |p: Point3<f64>, q: Point3<f64>| {
        ((p.x - q.x).powi(2) + (p.y - q.y).powi(2) + (p.z - q.z).powi(2)).sqrt()
    };
    let rim: Vec<FaceKey> = cup
        .faces()
        .filter(|(_, f)| {
            matches!(cup.get_surface(f.surface),
                Some(Surface::Plane { origin, .. }) if (origin.y - VASE_TOP).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(rim.len(), 1, "the mouth plane is worn by ONE rim face");
    let f = cup.get_face(rim[0]).expect("the rim");
    assert_eq!(f.rings.len(), 1, "the rim carries exactly one ring");

    // The ring is the inner circle, and ONLY arcs of it.
    let carriers = loop_carriers(&cup, f.rings[0]);
    for c in &carriers {
        match c {
            Curve3::Circle { center, radius, .. } => {
                assert!(
                    center.x.abs() < 1e-9
                        && center.z.abs() < 1e-9
                        && (radius - r_ring).abs() < 1e-9,
                    "the ring must be the circle of radius neck - t about the axis, got {c:?}"
                );
            }
            other => panic!(
                "the ring carries a non-arc edge: {other:?} — a radial leg is \
                             exactly the old defect"
            ),
        }
    }

    // And it is DISJOINT from the outer loop: no shared vertex
    // position, and the axis point neither loop owns any more.
    let ring_ends = loop_edge_ends(&cup, f.rings[0]);
    let outer_ends = loop_edge_ends(&cup, f.outer);
    for (a, b) in ring_ends.iter().chain(&outer_ends) {
        for p in [a, b] {
            assert!(
                dist_pt(*p, axis_pt) > 1e-9,
                "no loop of the rim may own the axis point any more; {p:?} does"
            );
        }
    }
    for (ra, rb) in &ring_ends {
        for (oa, ob) in &outer_ends {
            for (r, o) in [(ra, oa), (ra, ob), (rb, oa), (rb, ob)] {
                assert!(
                    dist_pt(*r, *o) > 1e-9,
                    "the ring shares a vertex position with the outer loop at {r:?}"
                );
            }
        }
    }
    assert_eq!(rings(&cup), 1, "and that is the body's only ring");

    // Euler bookkeeping on the returned data reads genus 0 — computed
    // here from raw counts, not via the scene's helper.
    let (v, e, f) = (
        cup.vertices().count() as i64,
        cup.edges().count() as i64,
        cup.faces().count() as i64,
    );
    let s = cup.shells().count() as i64;
    assert_eq!(
        s - (v - e + f - rings(&cup) as i64) / 2,
        0,
        "Euler-Poincare over the returned arenas reads genus 0, as a cup's is"
    );

    // And it tessellates, at the budget that used to refuse.
    let m = pncad::mesh::tessellate(&cup, 5e-4, tol).expect("the rim triangulates");
    assert!(
        m.patches.iter().map(|q| q.triangles.len()).sum::<usize>() > 0,
        "a mesh with no triangles is not a mesh"
    );
}

/// **P2 — #1081's class claim tested OUTSIDE the PR's enumeration.**
/// The claimed law: a junction survives `shell` exactly when the
/// neighbouring surface is invariant under the moved face's own offset
/// motion — so obliquity, not curvature, is the class. Two all-plane
/// fixtures the PR never built:
///
/// - a right prism on a REGULAR HEXAGON (every side-to-side dihedral
///   120°, every side-to-cap 90°) must refuse `ReanchorOffCarrier`;
/// - a right prism on a RIGHT TRAPEZOID (a box with ONE beveled side;
///   three square footprint corners, two oblique ones) must refuse the
///   same way — one oblique junction is enough.
///
/// The mechanism is also checked QUANTITATIVELY on the hexagon: the
/// re-anchor gap is the distance from a moved vertex to the
/// neighbour's edge carrier, which for two planes meeting at dihedral
/// θ under wall t is t·|cos θ| — here t/2 exactly (θ = 60° between
/// normals). A tag masquerading as a length would not land there.
#[test]
fn p2_the_oblique_class_holds_outside_the_enumeration() {
    let tol = Tol::witness();
    let t = 0.02;
    let s3 = 3.0_f64.sqrt();
    let hexagon = extruded(
        Open.at(Point2::new(0.2, 0.0))
            .line_to(Point2::new(0.1, 0.1 * s3), tol)
            .expect("a")
            .line_to(Point2::new(-0.1, 0.1 * s3), tol)
            .expect("b")
            .line_to(Point2::new(-0.2, 0.0), tol)
            .expect("c")
            .line_to(Point2::new(-0.1, -0.1 * s3), tol)
            .expect("d")
            .line_to(Point2::new(0.1, -0.1 * s3), tol)
            .expect("e")
            .line_to(Start, tol)
            .expect("f")
            .into(),
        0.25,
        tol,
    );
    let beveled_box = extruded(
        Open.at(Point2::new(0.0, 0.0))
            .line_to(Point2::new(0.4, 0.0), tol)
            .expect("a")
            .line_to(Point2::new(0.3, 0.3), tol)
            .expect("bevel")
            .line_to(Point2::new(0.0, 0.3), tol)
            .expect("c")
            .line_to(Start, tol)
            .expect("d")
            .into(),
        0.25,
        tol,
    );
    for (what, body, expect_gap) in [
        ("a right prism on a regular hexagon", hexagon, Some(t / 2.0)),
        ("a box with one beveled side", beveled_box, None),
    ] {
        let e = pncad::topo::shell(&body, t, FIT_TOL, band(tol), tol)
            .expect_err("an oblique all-plane junction must refuse");
        let ShellError::Face { error, .. } = e else {
            panic!("{what}: not the offset door's refusal: {e}");
        };
        let ReplaceFaceError::ReanchorOffCarrier { gap, .. } = *error else {
            panic!("{what}: not the re-anchor refusal: {error}");
        };
        assert!(gap > 0.0, "{what}: the gap is a length, got {gap}");
        println!("{what}: ReanchorOffCarrier gap = {gap}");
        if let Some(want) = expect_gap {
            assert!(
                (gap - want).abs() < 1e-12,
                "{what}: the mechanism predicts gap = t·cos60° = {want}, got {gap}"
            );
        }
    }
}

/// **P2b — the surviving class's POSITIVE half on a fixture the PR
/// never built**: a right prism on a plus-sign (all-square, nonconvex
/// in two directions) hollows, per the invariance law.
#[test]
fn p2b_an_all_square_plus_prism_still_hollows() {
    let tol = Tol::witness();
    let plus = extruded(
        Open.at(Point2::new(0.1, 0.0))
            .line_to(Point2::new(0.2, 0.0), tol)
            .expect("a")
            .line_to(Point2::new(0.2, 0.1), tol)
            .expect("b")
            .line_to(Point2::new(0.3, 0.1), tol)
            .expect("c")
            .line_to(Point2::new(0.3, 0.2), tol)
            .expect("d")
            .line_to(Point2::new(0.2, 0.2), tol)
            .expect("e")
            .line_to(Point2::new(0.2, 0.3), tol)
            .expect("f")
            .line_to(Point2::new(0.1, 0.3), tol)
            .expect("g")
            .line_to(Point2::new(0.1, 0.2), tol)
            .expect("h")
            .line_to(Point2::new(0.0, 0.2), tol)
            .expect("i")
            .line_to(Point2::new(0.0, 0.1), tol)
            .expect("j")
            .line_to(Point2::new(0.1, 0.1), tol)
            .expect("k")
            .line_to(Start, tol)
            .expect("l")
            .into(),
        0.25,
        tol,
    );
    let hollow = pncad::topo::shell(&plus, 0.02, FIT_TOL, band(tol), tol)
        .expect("an all-square nonconvex prism is inside the surviving class");
    assert_eq!(hollow.shells().count(), 2, "outer + cavity");
}

/// **P3 — the bellied pot's wall-1 gap, re-derived in closed form.**
/// The PR pins `gap: 0.006097308927399331` for the sphere-zone pot.
/// Under the re-anchor mechanism (the foot cylinder shrinks by t and
/// the sphere's seam-meridian arc is asked to end at the moved
/// vertex), the gap is the moved vertex's distance to that meridian
/// circle: R − √((r_foot − t)² + (y_foot − c)²) with R = 5/64 the
/// sphere radius, r_foot = 4/64, c = 1/16 the sphere centre, t =
/// 1/128. This probe re-runs the PR's own fixture and checks the
/// payload equals the closed form to 1e-15 — the strongest available
/// evidence that the gap is the mechanism's number, not a tag.
#[test]
fn p3_wall1_gap_matches_the_reanchor_closed_form() {
    let tol = Tol::witness();
    let t = 1.0 / 128.0;
    let bellied = revolved(
        Open.at(Point2::new(0.0, 0.0))
            .line_to(Point2::new(4.0 / 64.0, 0.0), tol)
            .expect("base")
            .line_to(Point2::new(4.0 / 64.0, 1.0 / 64.0), tol)
            .expect("foot")
            .arc_to(
                pncad::profile::Center {
                    c: Point2::new(0.0, 4.0 / 64.0),
                    winding: pncad::profile::ArcSweep::Ccw,
                    p: Point2::new(3.0 / 64.0, 8.0 / 64.0),
                },
                tol,
            )
            .expect("belly")
            .line_to(Point2::new(0.0, 8.0 / 64.0), tol)
            .expect("mouth")
            .line_to(Start, tol)
            .expect("axis")
            .into(),
        tol,
    );
    let e = pncad::topo::shell(&bellied, t, FIT_TOL, band(tol), tol)
        .expect_err("the bellied pot refuses (wall 1)");
    let ShellError::Face { error, .. } = e else {
        panic!("not the offset door: {e}");
    };
    let ReplaceFaceError::ReanchorOffCarrier { gap, .. } = *error else {
        panic!("not the re-anchor refusal: {error}");
    };
    let r = 5.0 / 64.0;
    let dy: f64 = 1.0 / 64.0 - 4.0 / 64.0;
    let want = r - ((4.0 / 64.0 - t).powi(2) + dy.powi(2)).sqrt();
    println!("wall-1 gap = {gap}, closed form = {want}");
    assert!(
        (gap - want).abs() < 1e-15,
        "the payload is the mechanism's own length: got {gap}, want {want}"
    );
}
