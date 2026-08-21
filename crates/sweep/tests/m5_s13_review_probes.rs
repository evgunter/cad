//! M5 S13 review probes, ADOPTED (fix pass): the blinded reviewer's
//! adversarial rows, kept as permanent guards. Probe 9 is the review's
//! MAJOR — the multi-escape attack — now pinned at the F1 refusal.
//!
//! History notes (merge-base evidence, reviewer's `zz_mb_probes.rs`):
//! the S12 finding (union = 16.0) reproduced at the merge base, and the
//! dev-2 face-box hazard was CONFIRMED LIVE ON MAIN there — the
//! realized lane's vertex-hull face box for sphere faces pruned the
//! dir-1 slab-edge × sphere-face pierce candidates, so the fin×ball
//! union answered as if the overlapping pair were disjoint (a silent
//! self-overlapping body) while the idealized lane examined the pair
//! and refused: a strategy divergence AND a silent wrong volume. The
//! S13 sphere face-box arm (`boxes.rs`, center ± r) closes both; probe
//! 1 is its guard.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use profile::RawLoop;

use geom_core::{Affine3, Mat3, Point2, Point3, Tolerance, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanDeclarations, BooleanError};
use geom_core::Tol;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn slack() -> f64 {
    (1e3 * Tol::witness().get().eps).max(1e-9)
}

fn vol(body: &Body<f64>) -> f64 {
    topo::mass_properties(body, Tol::witness()).unwrap().volume
}

fn boxy(x0: f64, y0: f64, x1: f64, y1: f64, h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(
        [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(h), Tol::witness()).unwrap().body
}

fn slab() -> Body<f64> {
    boxy(0.0, 0.0, 4.0, 4.0, 1.0)
}

fn ball_at(r: f64, centre: Vec3<f64>) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -r), 1.0),
        ProfileVertex::new(p2(0.0, r), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    };
    let ball = revolve(&vp, axis, Revolution::Full, Tol::witness()).unwrap().body;
    topo::transform_rigid(&ball, &Affine3::translation(centre), Tol::witness()).unwrap()
}

/// Rotate `body` about `pivot` by `theta` around the world x-axis.
fn rot_x_about(body: &Body<f64>, pivot: Vec3<f64>, theta: f64) -> Body<f64> {
    let (s, c) = theta.sin_cos();
    let linear = Mat3::from_cols(
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, c, s),
        Vec3::new(0.0, -s, c),
    );
    let map = Affine3::from_parts(linear, pivot - linear * pivot);
    topo::transform_rigid(body, &map, Tol::witness()).unwrap()
}

/// Rotate `body` about `pivot` by `theta` around the world y-axis.
fn rot_y_about(body: &Body<f64>, pivot: Vec3<f64>, theta: f64) -> Body<f64> {
    let (s, c) = theta.sin_cos();
    let linear = Mat3::from_cols(
        Vec3::new(c, 0.0, -s),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(s, 0.0, c),
    );
    let map = Affine3::from_parts(linear, pivot - linear * pivot);
    topo::transform_rigid(body, &map, Tol::witness()).unwrap()
}

fn cap(r: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * r - h) / 3.0
}

/// PROBE 1 (dev 2 guard): a fin whose TOP EDGES pierce the sphere belly
/// while every ball edge stays clear of the fin — dir-1 crossings only,
/// exactly the events main's vertex-hull face box pruned (history note
/// in the module docs). Both strategies must agree and never answer
/// silently wrong (typed refusal is acceptable; a wrong volume is not).
#[test]
fn probe_belly_pierce_no_silent_answer_and_lanes_agree() {
    let fin = boxy(0.0, 1.9, 4.0, 2.1, 0.8);
    let ball = ball_at(0.6, Vec3::new(2.0, 2.0, 1.2));
    let decls = BooleanDeclarations::none();
    let r = boolean_op_with(
        BooleanOp::Union,
        &fin,
        &ball,
        &decls,
        SweepStrategy::Realized,
        Tol::witness(),
    );
    let i = boolean_op_with(
        BooleanOp::Union,
        &fin,
        &ball,
        &decls,
        SweepStrategy::Idealized,
        Tol::witness(),
    );
    match (&r, &i) {
        (Ok(rb), Ok(ib)) => {
            let rv = vol(&rb.body().unwrap().body);
            let iv = vol(&ib.body().unwrap().body);
            let fin_v = 4.0 * 0.2 * 0.8;
            let ball_v = 4.0 * PI * 0.6_f64.powi(3) / 3.0;
            assert!(
                rv < fin_v + ball_v - 1e-6,
                "realized union metered the disjoint sum {rv} (silent overlap loss)"
            );
            assert!((rv - iv).abs() < slack(), "lanes disagree: {rv} vs {iv}");
        }
        (Err(_), Err(_)) => {} // typed refusal both lanes: honest.
        (a, b) => panic!("strategy divergence: realized {a:?} vs idealized {b:?}"),
    }
}

/// PROBE 2: ball exactly tangent to BOTH slab faces from inside
/// (r = 0.5 buried at mid-height): the extent gap is exactly zero —
/// the scan's tangency arm refuses typed, never answers.
#[test]
fn probe_exact_tangency_from_inside_refuses_typed() {
    let b = ball_at(0.5, Vec3::new(2.0, 2.0, 0.5));
    let err = topo::union(&slab(), &b, Tol::witness()).expect_err("tangency must not answer");
    let BooleanError::FallbackExtentUnsupported { what, .. } = err else {
        panic!("expected the scan's tangency arm, got {err:?}");
    };
    assert!(what.contains("tangent"), "{what}");
}

/// PROBE 3 (door corrected in adoption): the section circle crosses the
/// top face's boundary — but the configuration NEVER reaches the scan's
/// near-boundary arm, because a circle crossing a boundary edge means
/// that edge passes within `r` of the sphere center (the two conditions
/// are the same inequality, `cx² + s² < r²`), so the REDUCE-stage
/// pierce frontier fires first: `CurvedPierceUnsupported`, typed. The
/// scan's near-boundary arm remains as certified-enclosure
/// defense-in-depth behind that door (its residual live width is the
/// box pad; the shadowing is structural — the pierce door runs before
/// any fallback — so this pin is stable).
#[test]
fn probe_edge_escape_refuses_typed_at_the_pierce_frontier() {
    let b = ball_at(0.5, Vec3::new(0.3, 2.0, 1.2));
    let err = topo::union(&slab(), &b, Tol::witness()).expect_err("edge escape must not certify");
    let BooleanError::CurvedPierceUnsupported { .. } = err else {
        panic!("expected the pierce frontier, got {err:?}");
    };
}

/// PROBE 4: bit-replay — the flipped finding row twice in-process,
/// debug-identical (within-run D9; cross-version bits are NOT pinned).
#[test]
fn probe_flipped_row_replays_bit_identical() {
    let a = slab();
    let b = ball_at(1.0, Vec3::new(2.0, 2.0, 0.5));
    let decls = BooleanDeclarations::none();
    let one = boolean_op_with(BooleanOp::Union, &a, &b, &decls, SweepStrategy::Realized, Tol::witness())
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone();
    let two = boolean_op_with(BooleanOp::Union, &a, &b, &decls, SweepStrategy::Realized, Tol::witness())
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone();
    assert_eq!(format!("{one:?}"), format!("{two:?}"));
    assert!((vol(&one) - 17.30899693899575).abs() < slack());
}

/// PROBE 5: a TILTED ball chart (axis 0.3 rad off vertical, seam kept
/// clear of the slab) — the re-cut handles an arbitrary source chart,
/// not just the perpendicular fixture: green with the exact cap volume.
#[test]
fn probe_tilted_chart_recut_still_cuts_exact() {
    let b0 = ball_at(0.5, Vec3::new(2.0, 2.0, 1.2));
    let b = rot_x_about(&b0, Vec3::new(2.0, 2.0, 1.2), 0.3);
    let cut = boolean_op_with(
        BooleanOp::Subtract,
        &slab(),
        &b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .expect("the re-cut re-charts any tilted source chart")
    .body()
    .unwrap()
    .body
    .clone();
    assert!(
        (vol(&cut) - (16.0 - cap(0.5, 0.3))).abs() < slack(),
        "tilted-chart pip volume: {}",
        vol(&cut)
    );
}

/// PROBE 6: NEAR-PARALLEL chart (axis 0.05 rad off the escape normal):
/// the seam then crosses the slab, so the pipeline goes through the
/// crossing layer and the tilted-section frontier — a typed refusal or
/// an exact answer are both honest; a wrong volume is not.
#[test]
fn probe_near_parallel_axis_never_answers_wrong() {
    let b0 = ball_at(0.5, Vec3::new(2.0, 2.0, 1.2));
    let b = rot_x_about(
        &b0,
        Vec3::new(2.0, 2.0, 1.2),
        core::f64::consts::FRAC_PI_2 - 0.05,
    );
    match boolean_op_with(
        BooleanOp::Subtract,
        &slab(),
        &b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        Tol::witness(),
    ) {
        Ok(out) => {
            let v = vol(&out.body().unwrap().body);
            assert!(
                (v - (16.0 - cap(0.5, 0.3))).abs() < slack(),
                "near-parallel answered WRONG: {v}"
            );
        }
        Err(
            BooleanError::Escalated { .. }
            | BooleanError::FallbackExtentUnsupported { .. }
            | BooleanError::Join(_)
            | BooleanError::JoinDesync { .. },
        ) => {}
        Err(other) => panic!("unexpected refusal shape: {other:?}"),
    }
}

/// PROBE 7 (door corrected in adoption; FLIPPED at the M6 rider —
/// history kept per the S9 pattern): strictly nested sphere×sphere.
///
/// At M5 the inner ball's circle edges hit the UNCONDITIONAL
/// conic-carrier pierce arm, so this pinned a typed refusal and the
/// scan's nested arm sat shadowed behind it as defense-in-depth. The
/// M6 rider (`bool_circle_curved_clearance`) proves the inner ball's
/// circles DEFINITELY inside the outer sphere and the outer ball's
/// circles definitely outside the inner one — no examined pair
/// survives — so the pair reaches the containment walk, whose
/// whole-sphere arm (both operands are CLOSED sphere groups) answers
/// soundly: the union of a ball and a ball nested inside it is the
/// outer ball. The shadowed arm turned out to be a fully working
/// door, and this row now pins the ANSWER instead of the mask.
#[test]
fn probe_nested_spheres_union_to_the_outer_ball() {
    use core::f64::consts::PI;
    let big = ball_at(1.0, Vec3::new(2.0, 2.0, 0.0));
    let small = ball_at(0.3, Vec3::new(2.0, 2.0, 0.2));
    let out = topo::union(&big, &small, Tol::witness()).expect("the whole-sphere containment arm answers");
    let body = &out.body().expect("a body").body;
    assert_eq!(body.shells().count(), 1, "one shell: the outer ball");
    let vol = topo::mass_properties(body, Tol::witness()).unwrap().volume;
    let want = 4.0 * PI / 3.0;
    assert!(
        (vol - want).abs() <= 1e-9 * want,
        "the union IS the outer ball: {vol} vs {want}"
    );
}

/// PROBE 8: dense tilted-plane residual sweep for the C5 circle —
/// 10_007 samples against BOTH surfaces (the unit's own row samples 17).
#[test]
fn probe_plane_sphere_section_dense_residual() {
    use geom::Surface;
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let n = Vec3::new(0.3, -0.55, 0.9).normalize();
    let plane = Surface::Plane {
        origin: Point3::new(0.1, -0.2, 0.35),
        normal: n,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let c = Point3::new(0.4, 0.1, 0.9);
    let r = 1.7;
    let sphere = Surface::Sphere {
        center: c,
        radius: r,
        axis: Vec3::new(0.2, 0.9, -0.1).normalize(),
        u_ref: Vec3::new(0.9, -0.2, 0.1).normalize(),
    };
    let geom_brep::PlaneSphereSection::Circle(circ) =
        geom_brep::plane_sphere_section(&plane, &sphere, band).unwrap()
    else {
        panic!("expected a circle");
    };
    let geom::Curve3::Circle {
        center: cc,
        axis: ca,
        radius: cr,
        u_ref: cu,
    } = circ
    else {
        panic!("non-circle carrier");
    };
    assert!(
        cu.dot(ca).abs() < 1e-14,
        "u_ref not in-plane: {}",
        cu.dot(ca)
    );
    let v = ca.cross(cu);
    for k in 0..10_007u32 {
        let t = 2.0 * PI * f64::from(k) / 10_007.0;
        let p = cc + cu * (cr * t.cos()) + v * (cr * t.sin());
        let pr = geom_brep::implicit_residual(&plane, p).abs();
        let sr = geom_brep::implicit_residual(&sphere, p).abs();
        assert!(pr < 1e-12, "plane residual {pr} at {k}");
        assert!(sr < 1e-12, "sphere residual {sr} at {k}");
    }
}

/// PROBE 9 — **the review's MAJOR, pinned at the F1 refusal**: ONE
/// sphere group escaping through TWO NON-PARALLEL faces (top z = 1 and
/// side x = 0) with the seam great circle steered clear of both — no
/// crossings, so only the extent scan sees the configuration. Before
/// the fix the scan kept the FIRST escape normal only, the re-charted
/// seam cut one circle, and the union answered 16 + cap_top exactly —
/// tier-3 valid and silently short one cap. Now the scan collects
/// EVERY definite escape normal and refuses typed unless all are
/// parallel (`bool_sphere_escape_parallel`, metered at the group's
/// radius); multi-chart re-cutting stays banked as an extension.
#[test]
fn probe_two_nonparallel_escapes_refuse_typed() {
    let r = 0.4;
    let (cx, cy, cz) = (0.383, 2.0, 0.85);
    let pivot = Vec3::new(cx, cy, cz);
    let b0 = ball_at(r, pivot);
    let b1 = rot_x_about(&b0, pivot, -0.2137);
    let b = rot_y_about(&b1, pivot, 0.312);
    let decls = BooleanDeclarations::none();
    for strat in [SweepStrategy::Realized, SweepStrategy::Idealized] {
        let err = boolean_op_with(BooleanOp::Union, &slab(), &b, &decls, strat, Tol::witness())
            .expect_err("two non-parallel escapes must refuse");
        let BooleanError::FallbackExtentUnsupported { what, .. } = err else {
            panic!("{strat:?}: expected the multi-escape refusal, got {err:?}");
        };
        assert!(what.contains("NON-PARALLEL"), "{strat:?}: {what}");
    }
}
