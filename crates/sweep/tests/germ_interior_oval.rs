//! **A curved face that meets the other operand only in a closed loop
//! interior to both faces, while crossings exist elsewhere.**
//!
//! Nothing on the crossings path sees such a loop: no edge event marks
//! it, the join cuts nothing along it, and face-region propagation
//! carries each face's side across it. Before the guard these fixtures
//! came back as VALID bodies that were wrong — the overlap counted twice
//! under ∪ and dropped under ∩ and ∖. The section certificate
//! (`topo::boolean::section_cert`, run by `ops::interior_loop_verdict`
//! on the crossings path and by the fallback's section pass without
//! crossings) classifies every face pair's section and refuses a loop it
//! certifies interior to both faces. These rows pin those refusals, the
//! answers it gives back (with their closed forms), and the verdict it
//! reaches per pair (`topo::test_support::section_report`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop, test_support::bulge_loop};
use revolve_common::{axis_y, validated};
use sweep::{Revolution, revolve};
use topo::Body;

/// The certificate's verdicts on the crossings path, one string per
/// pair: `Ok([...])` with each component's witness, or `Err(...)` with
/// the refusal.
fn verdicts(a: &Body<f64>, b: &Body<f64>) -> Vec<String> {
    topo::test_support::section_report(topo::BooleanOp::Union, a, b, Tol::witness())
        .expect("the reduction runs")
        .into_iter()
        .map(|(_, _, v)| v)
        .collect()
}

fn in_solid(b: &Body<f64>, q: Point3<f64>) -> Option<bool> {
    let band = geom_core::Band::linear(Tol::witness()).expect("the run's band");
    match topo::point_in_solid(b, q, band, Tol::witness()) {
        Ok(topo::SolidContainment::In) => Some(true),
        Ok(topo::SolidContainment::Out) => Some(false),
        _ => None,
    }
}

/// A C-shaped profile in the `xz` plane, extruded symmetrically in `y`.
fn bracket_xz(pts: &[(f64, f64)], half_y: f64) -> Body<f64> {
    let lp = ProfileLoop::polygon(pts.iter().map(|&(x, z)| Point2::new(x, z)));
    let plane = profile::SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(Vec3::unit_x(), Vec3::unit_z(), -Vec3::unit_y()),
        Vec3::new(0.0, half_y, 0.0),
    ));
    let vp = profile::Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("the bracket profile validates");
    sweep::extrude(
        &vp,
        sweep::Extrusion::Distance(2.0 * half_y),
        Tol::witness(),
    )
    .expect("the bracket extrudes")
    .body
}

fn half_donut() -> Body<f64> {
    let vp = validated(vec![revolve_common::donut_profile()]);
    revolve(
        &vp,
        axis_y(),
        Revolution::Partial(std::f64::consts::PI),
        Tol::witness(),
    )
    .expect("the half donut revolves")
    .body
}

fn torus_bracket() -> Body<f64> {
    bracket_xz(
        &[
            (1.95, -0.1),
            (2.05, -0.1),
            (2.05, 0.8),
            (3.0, 0.8),
            (3.0, -2.45),
            (-1.0, -2.45),
            (-1.0, -2.8),
            (3.2, -2.8),
            (3.2, 1.0),
            (1.95, 1.0),
        ],
        0.3,
    )
}

fn dome() -> Body<f64> {
    let t = (std::f64::consts::PI / 8.0).tan();
    let lp = bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), t),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    let vp = validated(vec![lp]);
    let mut up = revolve(&vp, axis_y(), Revolution::Full, Tol::witness())
        .expect("the dome revolves")
        .body;
    up.merge_coplanar_faces(Tol::witness())
        .expect("the split base disc merges");
    up
}

fn boxed(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    let lp = ProfileLoop::polygon([
        Point2::new(x.0, y.0),
        Point2::new(x.1, y.0),
        Point2::new(x.1, y.1),
        Point2::new(x.0, y.1),
    ]);
    let plane = profile::SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()),
        Vec3::new(0.0, 0.0, z.0),
    ));
    let vp = profile::Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("the box profile validates");
    sweep::extrude(&vp, sweep::Extrusion::Distance(z.1 - z.0), Tol::witness())
        .expect("the box extrudes")
        .body
}

/// The cap direction: azimuth 90° off the dome's seam, latitude 30°.
fn cap_dir() -> Vec3<f64> {
    Vec3::new(0.0, 0.5, 0.75_f64.sqrt())
}

fn dome_bracket() -> Body<f64> {
    let top = boxed((-0.6, 0.6), (0.9, 1.5), (-0.6, 0.6));
    let u = cap_dir();
    let e2 = Vec3::new(0.0, 0.75_f64.sqrt(), -0.5);
    let foot = topo::transform_rigid(
        &boxed((-0.55, 0.55), (-0.6, 1.2), (0.95, 1.3)),
        &Affine3::from_parts(
            Mat3::from_cols(Vec3::unit_x(), e2, u),
            Vec3::new(0.0, 0.0, 0.0),
        ),
        Tol::witness(),
    )
    .expect("the foot tilts");
    let r = topo::union(&top, &foot, Tol::witness()).expect("the bracket's two boxes union");
    r.body().expect("non-empty").body.clone()
}

fn top_box() -> Body<f64> {
    boxed((-0.6, 0.6), (0.9, 1.5), (-0.6, 0.6))
}

fn volume(b: &Body<f64>) -> f64 {
    topo::mass_properties(b, Tol::witness())
        .expect("the volume integrates")
        .volume
}

fn close(got: f64, want: f64, what: &str) {
    assert!(
        (got - want).abs() <= 1e-9 * want.abs().max(1.0),
        "{what}: {got} against the closed form {want}"
    );
}

fn refuses_as_the_interior_loop_guard(
    r: Result<topo::BooleanResult<f64>, topo::BooleanError>,
    op: topo::BooleanOp,
    kind: geom_brep::SurfaceKind,
    what: &str,
) {
    refuses_at(r, topo::PairRefusalSite::InteriorLoopGuard, op, kind, what);
}

/// The refusal names its SITE, so a row can tell the guard from the
/// ∖/∩ revert roster, which refuses the same pair up front.
fn refuses_at(
    r: Result<topo::BooleanResult<f64>, topo::BooleanError>,
    site: topo::PairRefusalSite,
    op: topo::BooleanOp,
    kind: geom_brep::SurfaceKind,
    what: &str,
) {
    match r {
        Err(topo::BooleanError::CurvedPairUnsupported {
            op: Some(o),
            site: s,
            kind: k,
            ..
        }) => {
            assert_eq!((s, o, k), (site, op, kind), "{what}");
        }
        Err(e) => panic!("{what}: refused, but not at {site:?}: {e:?}"),
        Ok(r) => panic!(
            "{what}: answered {:?}, volume {:?}",
            r.body().map(|b| b.kind),
            r.body().map(|b| volume(&b.body))
        ),
    }
}

/// **The live case: a half donut and a bracket whose foot cuts an oval
/// off the outer equator.** The pin's crossings through the cap are the
/// only events, so the oval was never seen, and ∪ came back a valid
/// `Seamed` body of volume `π²/2 + 1.476 − 0.006` — the lens counted
/// twice. ∪ now refuses at the GUARD. ∩ and ∖ never reach it: the ∖/∩
/// revert roster has no torus and refuses the pair up front, and the
/// row says so rather than crediting the guard with it.
#[test]
fn a_torus_oval_refuses_union_at_the_guard_and_intersect_and_subtract_at_the_roster() {
    use topo::{BooleanOp as Op, PairRefusalSite as Site};
    let (h, c) = (half_donut(), torus_bracket());
    let torus = geom_brep::SurfaceKind::Torus;
    for (site, op, r, what) in [
        (
            Site::InteriorLoopGuard,
            Op::Union,
            topo::union(&h, &c, Tol::witness()),
            "h ∪ c",
        ),
        (
            Site::RevertRoster,
            Op::Intersect,
            topo::intersect(&h, &c, Tol::witness()),
            "h ∩ c",
        ),
        (
            Site::RevertRoster,
            Op::Subtract,
            topo::subtract(&h, &c, Tol::witness()),
            "h ∖ c",
        ),
        (
            Site::RevertRoster,
            Op::Subtract,
            topo::subtract(&c, &h, Tol::witness()),
            "c ∖ h",
        ),
    ] {
        refuses_at(r, site, op, torus, what);
    }
    // The foot's top face cuts a scrape oval with no event, witnessed
    // strictly inside both faces: a certified interior loop.
    assert!(
        verdicts(&h, &c).iter().any(|v| v == "Err(Loop)"),
        "{:?}",
        verdicts(&h, &c)
    );
}

/// **The pin alone answers its closed form** (the stopgap refused it on
/// reach). The pin's `x = 1.95` and `x = 2.05` faces cut scrape ovals
/// with no event whose witness points lie outside the pin face (W3); its
/// `y` faces cut two parallels and its `z` faces two `(0,1)` loops (W2).
/// The union is `vol(H) + vol(C) − 0.006`, the pin's piece in the tube.
/// Red against: the per-op reach kept, `σ` swapped in the scrape
/// witness, and W3 reading the torus face only.
#[test]
fn the_pin_alone_answers_its_closed_form() {
    let pin_only = bracket_xz(
        &[
            (1.95, -0.1),
            (2.05, -0.1),
            (2.05, 0.8),
            (3.0, 0.8),
            (3.0, -2.45),
            (3.2, -2.45),
            (3.2, 1.0),
            (1.95, 1.0),
        ],
        0.3,
    );
    let h = half_donut();
    let v = verdicts(&h, &pin_only);
    assert!(v.iter().all(|x| x.starts_with("Ok(")), "{v:?}");
    assert!(
        v.iter().any(|x| x.contains("Out(")),
        "a W3 clearance: {v:?}"
    );
    let r = topo::union(&h, &pin_only, Tol::witness())
        .unwrap_or_else(|e| panic!("the pin alone: {e:?}"));
    let b = &r.body().expect("non-empty").body;
    assert_eq!(topo::validate_geometric(b, Tol::witness()), Ok(()));
    close(
        volume(b),
        volume(&h) + volume(&pin_only) - 0.006,
        "the pin's union",
    );
    for (q, want) in [
        // The pin's piece inside the tube.
        (Point3::new(2.0, 0.0, -0.05), true),
        // Torus only, beside the pin, inside a scrape oval's lens side.
        (Point3::new(1.9, 0.45, -0.05), true),
        // Torus only, far round the ring.
        (Point3::new(0.0, 0.0, -2.0), true),
        // Bracket only.
        (Point3::new(3.1, 0.0, 0.0), true),
        // Neither: between the tube and the bracket wall.
        (Point3::new(2.7, 0.0, -0.1), false),
    ] {
        assert_eq!(in_solid(b, q), Some(want), "{q:?}");
    }
}

/// **The sphere analogue: a dome and a bracket whose foot cuts a cap off
/// the dome's side, clear of its seams, while the bracket's top crosses
/// the dome's crown.** On main every op answered a valid body missing
/// the side cap: `dome ∩ bracket` came back as the crown alone, and a
/// point inside the side cap, inside both operands, read `Out` of it.
#[test]
fn a_sphere_cap_behind_crossings_elsewhere_refuses_every_op() {
    let (d, c) = (dome(), dome_bracket());
    let band = geom_core::Band::linear(Tol::witness()).expect("the run's band");
    let q = Point3::origin() + cap_dir() * 0.97;
    for (name, body) in [("dome", &d), ("bracket", &c)] {
        assert!(
            matches!(
                topo::point_in_solid(body, q, band, Tol::witness()),
                Ok(topo::SolidContainment::In)
            ),
            "the witness point is inside the {name}"
        );
    }
    for (op, r) in [
        (topo::BooleanOp::Union, topo::union(&d, &c, Tol::witness())),
        (
            topo::BooleanOp::Intersect,
            topo::intersect(&d, &c, Tol::witness()),
        ),
        (
            topo::BooleanOp::Subtract,
            topo::subtract(&d, &c, Tol::witness()),
        ),
        (
            topo::BooleanOp::Subtract,
            topo::subtract(&c, &d, Tol::witness()),
        ),
    ] {
        refuses_as_the_interior_loop_guard(
            r,
            op,
            geom_brep::SurfaceKind::Sphere,
            "dome and bracket",
        );
    }
    assert!(
        verdicts(&d, &c).iter().any(|v| v == "Err(Loop)"),
        "{:?}",
        verdicts(&d, &c)
    );
}

/// **The sphere lane the guard must leave answering.** The bracket's top
/// box alone cuts the dome's crown at `y = 0.9`: the section circle
/// crosses the dome's seams, so the pair has events, and the other box
/// faces stand clear of the ball. The answers are the closed forms —
/// the crown cap `πh²(3r − h)/3` at `h = 0.1` and its complements —
/// valid at tier 3.
#[test]
fn the_crown_alone_still_answers_its_closed_form() {
    let (d, t) = (dome(), top_box());
    let cap = std::f64::consts::PI * 0.01 * (3.0 - 0.1) / 3.0;
    let dome_v = 2.0 * std::f64::consts::PI / 3.0;
    let box_v = 1.2 * 0.6 * 1.2;
    for (what, r, want) in [
        ("dome ∩ top", topo::intersect(&d, &t, Tol::witness()), cap),
        (
            "dome ∖ top",
            topo::subtract(&d, &t, Tol::witness()),
            dome_v - cap,
        ),
        (
            "dome ∪ top",
            topo::union(&d, &t, Tol::witness()),
            dome_v + box_v - cap,
        ),
    ] {
        let r = r.unwrap_or_else(|e| panic!("{what}: {e:?}"));
        let b = &r.body().expect("non-empty").body;
        assert_eq!(
            topo::validate_geometric(b, Tol::witness()),
            Ok(()),
            "{what}"
        );
        close(volume(b), want, what);
    }
}

/// **Whatever ∪ answers for the half donut and bracket, it does not
/// count the lens twice.** The pre-guard answer was a valid body of
/// volume `vol(H) + vol(C) − 0.006` — only the pin's overlap removed.
/// The true union is smaller by the lens, so a body at that volume (or
/// above) is the wrong answer, and a refusal is the only other outcome
/// this row accepts.
#[test]
fn the_half_donut_union_never_counts_the_lens_twice() {
    let (h, c) = (half_donut(), torus_bracket());
    let truth = volume(&h) + volume(&c) - (0.006 + oval_lens_volume());
    if let Ok(r) = topo::union(&h, &c, Tol::witness()) {
        let got = volume(&r.body().expect("non-empty").body);
        assert!(
            (got - truth).abs() <= 1e-6,
            "the union's volume {got} is not the true union's {truth} \
             (vol A + vol B − the pin's 0.006 − the oval's lens)"
        );
    }
}

/// **The oval's lens**: the part of the donut (`R = 2`, `r = 0.5` about
/// `y`) below `z = −2.45`, which lies inside the bracket's foot. At
/// height `y` the tube's section is the annulus `2 ± s(y)`,
/// `s = √(0.25 − y²)`; the plane `z = −2.45` cuts off the outer disc's
/// circular segment `a²·acos(d/a) − d·√(a² − d²)` (`a = 2 + s`,
/// `d = 2.45`) wherever `a > d`, and the inner disc never reaches it.
/// Integrated over `y` by composite Simpson; the integrand's square-root
/// edge at `|y| = √(0.25 − 0.45²)` is resolved by the step count.
fn oval_lens_volume() -> f64 {
    let d = 2.45_f64;
    let y0 = (0.25_f64 - 0.45 * 0.45).sqrt();
    let segment = |y: f64| {
        let a = 2.0 + (0.25 - y * y).max(0.0).sqrt();
        if a <= d {
            0.0
        } else {
            a * a * (d / a).acos() - d * (a * a - d * d).sqrt()
        }
    };
    let n = 20_000;
    let h = 2.0 * y0 / f64::from(n);
    let mut sum = segment(-y0) + segment(y0);
    for i in 1..n {
        let w = if i % 2 == 1 { 4.0 } else { 2.0 };
        sum += w * segment(-y0 + h * f64::from(i));
    }
    sum * h / 3.0
}

/// **The lens is far above the row's tolerance**, so the row above
/// tells the true union from the lens counted twice. The bracket: the
/// segment at `y = 0` (`≈ 0.0147 m²`) over a width of at most the chord
/// `2·√(0.25 − 0.45²) ≈ 0.436 m` bounds it above, and it is not a
/// sliver below.
#[test]
fn the_oval_lens_is_well_above_the_rows_tolerance() {
    let lens = oval_lens_volume();
    assert!(lens > 1e-3, "the lens volume {lens}");
    assert!(lens < 0.02, "the lens volume {lens}");
}

/// **The corner bar: a second lens, never a body.** A `0.2`-square bar
/// across the donut's hole, cut to the length that puts all eight
/// corners on the inner face. Its end squares' edges along `y` keep a
/// constant `ρ`, so their interiors run inside the tube, and the lens
/// between each end square and the tube is bounded only by the corners'
/// own contacts. The section certificate would clear it (each end
/// square's section is one component with the corners' events on it,
/// W4; the side faces cut `(0,1)` and `(1,0)` pairs, W2): the lens arcs
/// are evidenced, and tracing them is the crossing layer's business.
/// What keeps the result from being a body is downstream of the guard —
/// the chord rule and the sagitta charge, where the reduction refuses
/// today — and that is what this row pins.
#[test]
fn the_corner_bar_never_comes_back_a_body() {
    let d = {
        let vp = validated(vec![revolve_common::donut_profile()]);
        revolve(&vp, axis_y(), Revolution::Full, Tol::witness())
            .expect("the donut revolves")
            .body
    };
    let hw = 0.1_f64;
    let rho = 2.0 - (0.25 - hw * hw).sqrt();
    let z = (rho * rho - hw * hw).sqrt();
    let b = boxed((-hw, hw), (-hw, hw), (-z, z));
    for (what, r) in [
        ("∪", topo::union(&d, &b, Tol::witness())),
        ("∩", topo::intersect(&d, &b, Tol::witness())),
        ("∖", topo::subtract(&d, &b, Tol::witness())),
        ("∖ reversed", topo::subtract(&b, &d, Tol::witness())),
    ] {
        if let Ok(r) = r {
            panic!(
                "{what}: the corner bar came back {:?}",
                r.body().map(|x| (x.kind, volume(&x.body)))
            );
        }
    }
}

/// **A wedge clear of the donut's carrier answers although its box
/// overlaps.** A wedge above the donut (`R = 2`, `r = 0.5` about `y`), its
/// underside tilted along the plane `0.3x + y = 1.3`: every face's
/// carrier plane clears the torus — the underside by its support
/// `R·|n⊥| + r ≈ 1.075` against a distance `≈ 1.245`, the top, the ends
/// and the caps by more — while the underside's BOX, running down to
/// `y = 0.4` at `x = 3`, overlaps the donut faces' boxes. There are no
/// crossings, so the fallback runs, and the section pass certifies each
/// pair apart (W0: the torus × plane classification returns no
/// component), so the fallback answers the two disjoint solids — `π²`
/// (the donut, `2π²Rr²`) plus the wedge's `7.2 × 6`.
#[test]
fn a_wedge_clear_of_the_donuts_carrier_is_answered_though_its_box_overlaps() {
    let donut = {
        let vp = validated(vec![revolve_common::donut_profile()]);
        revolve(&vp, axis_y(), Revolution::Full, Tol::witness())
            .expect("the donut revolves")
            .body
    };
    let wedge = {
        let lp = ProfileLoop::polygon([
            Point2::new(-3.0, 2.2),
            Point2::new(3.0, 0.4),
            Point2::new(3.0, 2.5),
            Point2::new(-3.0, 2.5),
        ]);
        let plane = profile::SketchPlane::new(Affine3::from_parts(
            Mat3::from_cols(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()),
            Vec3::new(0.0, 0.0, -3.0),
        ));
        let vp = profile::Profile::new(plane, vec![lp])
            .validate(Tol::witness())
            .expect("the wedge profile validates");
        sweep::extrude(&vp, sweep::Extrusion::Distance(6.0), Tol::witness())
            .expect("the wedge extrudes")
            .body
    };
    let r = topo::union(&donut, &wedge, Tol::witness())
        .unwrap_or_else(|e| panic!("the certified-apart pair is answered: {e:?}"));
    let b = r.body().expect("non-empty");
    assert_eq!(b.kind, topo::BooleanResultKind::Assembly);
    close(
        volume(&b.body),
        std::f64::consts::PI.powi(2) + 7.2 * 6.0,
        "donut ∪ wedge",
    );
}
