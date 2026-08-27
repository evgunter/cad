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

/// **P1 — #1082 reproduced on a fixture the PR never built**, and the
/// "spurious ring" re-derived as a GEOMETRIC inconsistency rather than
/// accepted as a wrong number:
///
/// 1. each mouth half-disc carries exactly one ring;
/// 2. every edge of both rings lies on the FULL circle of radius
///    `neck − t` centred on the axis in the mouth plane — and the two
///    faces' rings are the SAME circle, claimed twice;
/// 3. each of those faces ALSO keeps an outer-loop edge whose two
///    endpoints sit at radial distances straddling `neck − t` — so the
///    ring's carrier CROSSES the face's own outer boundary (a
///    continuous path from radius ~0 to radius `neck` passes through
///    radius `neck − t`). An interior ring that crosses its outer loop
///    is not a valid trim of any face, so the body is WRONG as a body,
///    not merely differently-documented: correct topology for this cup
///    would be two half-annuli with no rings at all (each half-disc
///    minus the inner disc is simply connected), Euler characteristic
///    2, genus 0 — which is also what `topo::shell`'s module docs
///    promise ("one opening gives a cup, which is genus 0").
///
/// The CDT's refusal on exactly such a face then follows, and tiers
/// 1–3 all passing is re-asserted (the "validated wrong body" class).
#[test]
fn p1_shell_open_reproduced_and_rederived_on_my_own_revolve() {
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

    // Tiers 1-3 all bless it.
    assert!(pncad::topo::validate(&cup).is_ok(), "tier 1 passes");
    assert!(pncad::topo::validate_closed(&cup).is_ok(), "tier 2 passes");
    assert_eq!(
        pncad::topo::validate_geometric(&cup, tol),
        Ok(()),
        "tier 3 passes"
    );

    // The two mouth faces each carry one ring; both rings are the same
    // full circle of radius neck - t on the axis.
    let r_ring = VASE_NECK - t;
    let mut crossing_faces = 0;
    let mut ring_faces = 0;
    for (fk, f) in cup.faces() {
        let on_mouth = matches!(cup.get_surface(f.surface),
            Some(Surface::Plane { origin, .. }) if (origin.y - VASE_TOP).abs() < 1e-12);
        if !on_mouth {
            continue;
        }
        ring_faces += 1;
        assert_eq!(
            f.rings.len(),
            1,
            "face {fk:?}: each designated half-disc carries exactly one ring"
        );
        for c in loop_carriers(&cup, f.rings[0]) {
            match c {
                Curve3::Circle { center, radius, .. } => {
                    assert!(
                        center.x.abs() < 1e-12
                            && center.z.abs() < 1e-12
                            && (center.y - VASE_TOP).abs() < 1e-12
                            && (radius - r_ring).abs() < 1e-12,
                        "every ring edge lies on the FULL inner circle (axis, r = {r_ring}); \
                         got center {center:?} radius {radius}"
                    );
                }
                other => panic!(
                    "a ring edge that is not on the inner circle would allow a valid \
                     D-shaped half-annulus reading; got {other:?}"
                ),
            }
        }
        // The outer loop still runs from the axis out to the neck
        // radius: some edge's endpoints straddle r_ring, so the ring
        // crosses the outer boundary.
        let straddles = loop_edge_ends(&cup, f.outer).iter().any(|(a, b)| {
            let (ra, rb) = (a.x.hypot(a.z), b.x.hypot(b.z));
            ra.min(rb) < r_ring - 1e-9 && ra.max(rb) > r_ring + 1e-9
        });
        if straddles {
            crossing_faces += 1;
        }
    }
    assert_eq!(ring_faces, 2, "both half-discs came back");
    assert_eq!(
        crossing_faces, 2,
        "on BOTH half-discs the ring's carrier crosses the face's outer boundary — the \
         trim is geometrically inconsistent, so this is a wrong body, not a wrong doc"
    );
    assert_eq!(rings(&cup), 2, "and those are the body's only rings");

    // Euler bookkeeping on the returned data reads genus 1 (the
    // issue's number) — computed here from raw counts, not via the
    // scene's helper.
    let (v, e, f) = (
        cup.vertices().count() as i64,
        cup.edges().count() as i64,
        cup.faces().count() as i64,
    );
    let s = cup.shells().count() as i64;
    assert_eq!(
        s - (v - e + f - rings(&cup) as i64) / 2,
        1,
        "Euler–Poincaré over the returned arenas reads genus 1 where a cup is genus 0"
    );

    // And it will not tessellate, at a budget the PR never ran, with
    // the refusal on a mouth half-disc carrying the spurious ring.
    let e = pncad::mesh::tessellate(&cup, 5e-4, tol)
        .err()
        .expect("the wrong trim cannot mesh");
    let pncad::mesh::TessellateError::Triangulation { face } = e else {
        panic!("expected the CDT insertion refusal, got {e:?}");
    };
    let f = cup.get_face(face).expect("the refusing face");
    assert!(
        matches!(cup.get_surface(f.surface),
            Some(Surface::Plane { origin, .. }) if (origin.y - VASE_TOP).abs() < 1e-12)
            && f.rings.len() == 1,
        "the refusing face is a mouth half-disc with the spurious ring"
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
    let want = r - ((4.0 / 64.0 - t).powi(2) + (1.0 / 64.0 - 4.0 / 64.0).powi(2)).sqrt();
    println!("wall-1 gap = {gap}, closed form = {want}");
    assert!(
        (gap - want).abs() < 1e-15,
        "the payload is the mechanism's own length: got {gap}, want {want}"
    );
}
