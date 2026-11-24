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

use crate::common::operands::slab;
use core::f64::consts::PI;

use geom_core::Tol;
use geom_core::{Affine3, Mat3, Point3, Vec3};
use sweep::test_support::{ball_poled_y, brick, finished};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanError};

fn slack() -> f64 {
    (1e3 * Tol::witness().get().eps).max(1e-9)
}

fn vol(body: &Body<f64>) -> f64 {
    topo::test_support::pinned_volume(
        "the measured body",
        &topo::test_support::finished("the measured body", topo::Body::clone(body), Tol::witness()),
        Tol::witness(),
    )
    .volume
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
    topo::transform_rigid(body, &geom_core::test_support::rigid(&map, Tol::witness())).unwrap()
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
    topo::transform_rigid(body, &geom_core::test_support::rigid(&map, Tol::witness())).unwrap()
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
    let fin = brick::<f64>((0.0, 4.0), (1.9, 2.1), (0.0, 0.8), Tol::witness());
    let fin = finished("the fin", fin, Tol::witness());
    let ball = ball_poled_y(0.6, Vec3::new(2.0, 2.0, 1.2), Tol::witness());
    let ball = finished("the ball", ball, Tol::witness());
    let r = boolean_op_with(
        BooleanOp::Union,
        &fin,
        &ball,
        &topo::BooleanInputs::default(),
        SweepStrategy::Realized,
        Tol::witness(),
    );
    let i = boolean_op_with(
        BooleanOp::Union,
        &fin,
        &ball,
        &topo::BooleanInputs::default(),
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
/// the scan's tangency arm refuses typed, never answers, as the sphere's
/// question against the plane with the margin it decided.
#[test]
fn probe_exact_tangency_from_inside_refuses_typed() {
    let b = ball_poled_y(0.5, Vec3::new(2.0, 2.0, 0.5), Tol::witness());
    let (slab, b) = (
        finished("the slab", slab::<f64>(), Tol::witness()),
        finished("the ball", b, Tol::witness()),
    );
    let err = topo::union(&slab, &b, Tol::witness()).expect_err("tangency must not answer");
    let BooleanError::Escalated {
        decision: topo::BooleanDecision::Sphere(topo::SphereQuestion::AgainstPlane),
        diag,
    } = err
    else {
        panic!("expected the scan's tangency arm, got {err:?}");
    };
    assert_eq!(diag.predicate, Some("bool_sphere_extent_gap"));
    assert!(
        !diag.margin.is_invalid(),
        "the decided zero carries the margin the funnel decided: {diag:?}"
    );
}

/// PROBE 3 (door corrected in adoption): the section circle crosses the
/// top face's boundary — but the configuration NEVER reaches the scan's
/// near-boundary arm, because a circle crossing a boundary edge means
/// that edge passes within `r` of the sphere center (the two conditions
/// are the same inequality, `cx² + s² < r²`), so the REDUCE stage meets
/// the edge first: the line × sphere roots pierce it, the pierce
/// point's sector side certifies, and the section passes through the
/// ball's face as a ring, whose island the join winds without a chart
/// (`chord_join::path_island_winding`). The union keeps that ring as a
/// hole of the ball's face, which the sphere arm measures over both its
/// loops, so the union builds and passes tier 3; its volume and the
/// pose's other ops are `a_ring_on_a_sphere_face`'s. The scan's near-boundary
/// arm remains as certified-enclosure defense-in-depth behind that door
/// (its residual live width is the box pad; the shadowing is structural
/// — the reduction runs before any fallback — so this pin is stable).
#[test]
fn probe_edge_escape_lands_a_ring_before_the_scan() {
    let b = ball_poled_y(0.5, Vec3::new(0.3, 2.0, 1.2), Tol::witness());
    let (slab, b) = (
        finished("the slab", slab::<f64>(), Tol::witness()),
        finished("the ball", b, Tol::witness()),
    );
    let out = topo::union(&slab, &b, Tol::witness())
        .unwrap_or_else(|e| panic!("the pierce lands as a ring and the union builds: {e:?}"));
    let body = &out.body().expect("a body").body;
    assert_eq!(
        topo::validate_geometric(body, Tol::witness()),
        Ok(()),
        "the ringed ball face passes tier 3"
    );
}

/// PROBE 4: bit-replay — the flipped finding row twice in-process,
/// debug-identical (within-run D9; cross-version bits are NOT pinned).
#[test]
fn probe_flipped_row_replays_bit_identical() {
    let a = finished("the slab", slab::<f64>(), Tol::witness());
    let b = ball_poled_y(1.0, Vec3::new(2.0, 2.0, 0.5), Tol::witness());
    let b = finished("the ball", b, Tol::witness());
    let one = boolean_op_with(
        BooleanOp::Union,
        &a,
        &b,
        &topo::BooleanInputs::default(),
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .unwrap()
    .body()
    .unwrap()
    .body
    .clone();
    let two = boolean_op_with(
        BooleanOp::Union,
        &a,
        &b,
        &topo::BooleanInputs::default(),
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .unwrap()
    .body()
    .unwrap()
    .body
    .clone();
    assert_eq!(format!("{one:?}"), format!("{two:?}"));
    assert!((vol(&one) - 17.30899693899575).abs() < slack());
}

/// PROBE 5: a TILTED ball chart (axis 0.3 rad off vertical, seam kept
/// clear of the slab) — the cut-in serves an arbitrary source chart,
/// not just the perpendicular fixture: green with the exact cap volume.
#[test]
fn probe_tilted_chart_recut_still_cuts_exact() {
    let b0 = ball_poled_y(0.5, Vec3::new(2.0, 2.0, 1.2), Tol::witness());
    let b = rot_x_about(&b0, Vec3::new(2.0, 2.0, 1.2), 0.3);
    let (slab, b) = (
        finished("the slab", slab::<f64>(), Tol::witness()),
        finished("the tilted ball", b, Tol::witness()),
    );
    let cut = boolean_op_with(
        BooleanOp::Subtract,
        &slab,
        &b,
        &topo::BooleanInputs::default(),
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .expect("the cut-in serves any tilted source chart")
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

/// PROBE 6: NEAR-PARALLEL chart (axis 0.05 rad off the slab's normal):
/// the seam then crosses the slab, so the pipeline goes through the
/// crossing layer and the tilted-section frontier — a typed refusal or
/// an exact answer are both honest; a wrong volume is not.
#[test]
fn probe_near_parallel_axis_never_answers_wrong() {
    let b0 = ball_poled_y(0.5, Vec3::new(2.0, 2.0, 1.2), Tol::witness());
    let b = rot_x_about(
        &b0,
        Vec3::new(2.0, 2.0, 1.2),
        core::f64::consts::FRAC_PI_2 - 0.05,
    );
    let (slab, b) = (
        finished("the slab", slab::<f64>(), Tol::witness()),
        finished("the tilted ball", b, Tol::witness()),
    );
    match boolean_op_with(
        BooleanOp::Subtract,
        &slab,
        &b,
        &topo::BooleanInputs::default(),
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
            | BooleanError::Join(_),
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
/// M6 rider (`bool_conic_curved_clearance`) proves the inner ball's
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
    let big = ball_poled_y(1.0, Vec3::new(2.0, 2.0, 0.0), Tol::witness());
    let big = finished("the big ball", big, Tol::witness());
    let small = ball_poled_y(0.3, Vec3::new(2.0, 2.0, 0.2), Tol::witness());
    let small = finished("the small ball", small, Tol::witness());
    let out = topo::union(&big, &small, Tol::witness())
        .expect("the whole-sphere containment arm answers");
    let body = &out.body().expect("a body").body;
    assert_eq!(body.shells().count(), 1, "one shell: the outer ball");
    let vol = topo::test_support::pinned_volume("the measured body", body, Tol::witness()).volume;
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
        frame: geom::Frame::new(Point3::new(0.1, -0.2, 0.35), n, Vec3::new(1.0, 0.0, 0.0)),
    };
    let c = Point3::new(0.4, 0.1, 0.9);
    let r = 1.7;
    let sphere = Surface::Sphere {
        frame: geom::Frame::new(
            c,
            Vec3::new(0.2, 0.9, -0.1).normalize(),
            Vec3::new(0.9, -0.2, 0.1).normalize(),
        ),
        radius: r,
    };
    let geom_brep::PlaneSphereSection::Circle(circ) =
        geom_brep::plane_sphere_section(&plane, &sphere, band).unwrap()
    else {
        panic!("expected a circle");
    };
    let geom::Curve3::Circle(conic_k) = circ else {
        panic!("non-circle carrier");
    };
    let (cc, ca, cu) = (
        conic_k.frame().origin(),
        conic_k.frame().axis(),
        conic_k.frame().u_ref(),
    );
    let cr = conic_k.radius();
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

/// PROBE 9 — **one sphere group escaping through TWO NON-PARALLEL
/// faces builds both caps under every op**: the slab's top `z = 1` and
/// side `x = 0`, with the ball's seam steered clear of both, so no edge
/// crosses and only the extent scan sees the pose. Each escape is cut in
/// along the ball's own meridian through its circle, so both circles
/// land, and every op meets its closed form from the two caps (heights
/// 0.25 and 0.017, disjoint: no point of the ball lies past both faces).
/// Red on a body short a cap, on an invalid body, or on a refusal, in
/// either strategy.
#[test]
fn probe_two_nonparallel_escapes_build_both_caps_under_every_op() {
    let r = 0.4;
    let (cx, cy, cz) = (0.383, 2.0, 0.85);
    let pivot = Vec3::new(cx, cy, cz);
    let b0 = ball_poled_y(r, pivot, Tol::witness());
    let b1 = rot_x_about(&b0, pivot, -0.2137);
    let b = rot_y_about(&b1, pivot, 0.312);
    let (slab, b) = (
        finished("the slab", slab::<f64>(), Tol::witness()),
        finished("the turned ball", b, Tol::witness()),
    );
    let caps = cap(r, cz + r - 1.0) + cap(r, r - cx);
    let ball = 4.0 / 3.0 * PI * r.powi(3);
    for strat in [SweepStrategy::Realized, SweepStrategy::Idealized] {
        for (op, x, y, want, solids) in [
            (BooleanOp::Union, &slab, &b, 16.0 + caps, 1),
            (BooleanOp::Intersect, &slab, &b, ball - caps, 1),
            (BooleanOp::Subtract, &slab, &b, 16.0 - (ball - caps), 1),
            (BooleanOp::Subtract, &b, &slab, caps, 2),
        ] {
            let what = format!("{strat:?}, {op:?}");
            let out = boolean_op_with(
                op,
                x,
                y,
                &topo::BooleanInputs::default(),
                strat,
                Tol::witness(),
            )
            .unwrap_or_else(|e| panic!("{what}: both escapes cut in and the op builds: {e:?}"));
            let body = &out.body().unwrap_or_else(|| panic!("{what}: a body")).body;
            assert_eq!(
                topo::validate_geometric(body, Tol::witness()),
                Ok(()),
                "{what}: the result passes tier 3"
            );
            assert_eq!(body.solids().count(), solids, "{what}: its solids");
            assert!(
                (vol(body) - want).abs() < slack(),
                "{what}: volume {}, closed form {want}",
                vol(body)
            );
        }
    }
}

/// Rotate `body` about the line through `pivot` along `axis` by `theta`.
fn spun_about(body: &Body<f64>, pivot: Vec3<f64>, axis: Vec3<f64>, theta: f64) -> Body<f64> {
    let map = Affine3::rotation_about_axis(Point3::origin() + pivot, axis, theta);
    topo::transform_rigid(body, &geom_core::test_support::rigid(&map, Tol::witness())).unwrap()
}

/// The ball's sphere frame: its centre, polar axis and seam direction.
fn sphere_frame(body: &Body<f64>) -> (Point3<f64>, Vec3<f64>, Vec3<f64>) {
    body.faces()
        .find_map(|(f, _)| match body.face_surface(f) {
            Some(geom::Surface::Sphere { frame, .. }) => {
                Some((frame.origin(), frame.axis(), frame.u_ref()))
            }
            _ => None,
        })
        .expect("a ball has a sphere face")
}

/// Whether the ball's seam (the revolve's two meridians, pole to pole
/// through `u_ref` and back through `−u_ref`: one great circle) passes
/// from one side to the other of any of `planes` (a point and a normal
/// each): sampled densely, which is enough to tell a crossing these
/// poses make from none.
fn seam_crosses(body: &Body<f64>, r: f64, planes: &[(Point3<f64>, Vec3<f64>)]) -> bool {
    let (c, axis, u) = sphere_frame(body);
    planes.iter().any(|&(o, n)| {
        let side = |k: u32| {
            let phi = 2.0 * PI * f64::from(k) / 8192.0;
            let p = c + (u * phi.sin() + axis * phi.cos()) * r;
            (p - o).dot(n) > 0.0
        };
        (1..=8192).any(|k| side(k) != side(k - 1))
    })
}

/// Every op of the slab and `ball` against the caps `caps` the slab's
/// faces cut off the ball (disjoint): each builds at its closed form,
/// tier-3 valid, or refuses at the crossings path's interior-loop guard
/// (`work/germ/interior-loop-cut-in.md`), which a seam crossing a face
/// puts it on. Every op that builds cuts the ball in `cuts` times.
/// `true` where every op built.
fn every_op_or_the_guard(what: &str, ball: &Body<f64>, r: f64, caps: f64, cuts: usize) -> bool {
    let slab = finished("the slab", slab::<f64>(), Tol::witness());
    let ball = finished("the ball", ball.clone(), Tol::witness());
    let whole = 4.0 / 3.0 * PI * r.powi(3);
    let mut built = true;
    for (op, x, y, want) in [
        (BooleanOp::Union, &slab, &ball, 16.0 + caps),
        (BooleanOp::Intersect, &slab, &ball, whole - caps),
        (BooleanOp::Subtract, &slab, &ball, 16.0 - (whole - caps)),
        (BooleanOp::Subtract, &ball, &slab, caps),
    ] {
        let label = format!("{what}, {op:?}");
        match boolean_op_with(
            op,
            x,
            y,
            &topo::BooleanInputs::default(),
            SweepStrategy::Realized,
            Tol::witness(),
        ) {
            Ok(out) => {
                let body = &out.body().unwrap_or_else(|| panic!("{label}: a body")).body;
                assert_eq!(
                    topo::validate_geometric(body, Tol::witness()),
                    Ok(()),
                    "{label}: tier 3"
                );
                assert!(
                    (vol(body) - want).abs() < slack(),
                    "{label}: volume {}, closed form {want}",
                    vol(body)
                );
                let naming = &out
                    .body()
                    .unwrap_or_else(|| panic!("{label}: a body"))
                    .naming;
                assert_eq!(
                    naming.chart_cuts.iter().map(Vec::len).sum::<usize>(),
                    cuts,
                    "{label}: the ball's cuts"
                );
            }
            Err(BooleanError::CurvedPairUnsupported {
                site: topo::PairRefusalSite::InteriorLoopGuard,
                ..
            }) => built = false,
            Err(e) => panic!("{label}: a closed form or the interior-loop guard, not {e:?}"),
        }
    }
    built
}

/// **The non-parallel escapes, the ball spun about its own pole in 30°
/// steps**: probe 9's two-plane pose (top `z = 1` and side `x = 0`), and
/// a three-plane pose (r 0.4 at (0.37, 0.37, 0.63), poled along
/// (1, −1, 0), poking 0.03 through `x = 0`, `y = 0` and `z = 1`). The
/// three-plane pose's side circles lie on one meridian of the ball, so
/// the second cut finds its circle across the first cut and takes none
/// of its own (`apply_cut_ins`). Every spin of each builds every op at
/// its closed form or refuses at the interior-loop guard; every spin
/// whose seam crosses no escape face builds. Red on a wrong volume, an
/// invalid body, another refusal, or a seam-clear spin that does not
/// build.
#[test]
fn every_spin_of_the_nonparallel_escapes_builds_or_meets_the_guard() {
    let two = {
        let r = 0.4;
        let pivot = Vec3::new(0.383, 2.0, 0.85);
        let b0 = ball_poled_y(r, pivot, Tol::witness());
        let b = rot_y_about(&rot_x_about(&b0, pivot, -0.2137), pivot, 0.312);
        let caps = cap(r, pivot.z + r - 1.0) + cap(r, r - pivot.x);
        let planes = vec![
            (Point3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 1.0)),
            (Point3::new(0.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0)),
        ];
        ("two planes", r, pivot, b, caps, planes, 2)
    };
    let three = {
        let r = 0.4;
        let pivot = Vec3::new(0.37, 0.37, 0.63);
        let b0 = ball_poled_y(r, pivot, Tol::witness());
        let b = spun_about(&b0, pivot, Vec3::new(0.0, 0.0, 1.0), -0.75 * PI);
        let caps = 3.0 * cap(r, 0.03);
        let planes = vec![
            (Point3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 1.0)),
            (Point3::new(0.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0)),
            (Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, -1.0, 0.0)),
        ];
        ("three planes", r, pivot, b, caps, planes, 2)
    };
    for (pose, r, pivot, ball, caps, planes, cuts) in [two, three] {
        let (_, axis, _) = sphere_frame(&ball);
        let mut clear = 0;
        for k in 0..12u32 {
            let spin = f64::from(k) * PI / 6.0;
            let spun = spun_about(&ball, pivot, axis, spin);
            let what = format!("{pose}, spin {}°", 30 * k);
            let built = every_op_or_the_guard(&what, &spun, r, caps, cuts);
            if !seam_crosses(&spun, r, &planes) {
                clear += 1;
                assert!(
                    built,
                    "{what}: its seam crosses no escape face, so every op builds"
                );
            }
        }
        assert!(
            clear > 0,
            "{pose}: some spin keeps its seam off every escape face"
        );
    }
}
