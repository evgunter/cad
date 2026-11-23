//! The readings of where a conic is, each against a reference formed
//! beside the point the conic stores, on both placements and at both
//! scalars: a far-centred ellipse held on its arc (the centre a
//! megametre off, where any reading formed from it is off by about
//! `u·a`, `1e-7` m), the same ellipse re-seamed a quarter turn, and
//! conics held at their centres.

use core::f64::consts::FRAC_PI_2;

use geom_core::{Bounds, Interval, Point3, Real, Vec3};

use super::{Conic, Ellipse, Frame, Held};
use crate::Curve3;

const A: f64 = 1e9;
const B: f64 = 3e8;

/// The far ellipse of the arc-held ladder's numbers.
fn far() -> Ellipse<f64> {
    Ellipse::anchored(
        Point3::new(1.3, -0.7, 0.4),
        Vec3::new(0.5522240050794545, 0.3110026418424273, 0.7735153553621528),
        Vec3::new(0.6816514125372342, -0.702621773621506, -0.20414209516648582),
        A,
        B,
    )
}

/// Conics held at their centres, of a size whose readings hold to a few
/// roundings of a metre.
fn centred() -> [Curve3<f64>; 2] {
    let (c, n) = (Point3::new(0.5, -1.0, 2.0), Vec3::new(2.0, 2.0, 1.0) / 3.0);
    let u = Vec3::new(1.0, -2.0, 2.0) / 3.0;
    [
        Curve3::circle(c, n, 0.7, u),
        Curve3::Ellipse(Ellipse::new(Frame::new(c, n, u), 0.9, 0.4)),
    ]
}

fn stored(e: &Ellipse<f64>) -> (Point3<f64>, (f64, f64)) {
    match e.held {
        Held::Arc { phase } => (e.at.origin(), phase),
        Held::Centre => (e.at.origin(), (1.0, 0.0)),
    }
}

/// The point at `phi` from the anchor's own anomaly, summed from the
/// anchor with no cancellation: the reference the far rows read against.
fn beside(e: &Ellipse<f64>, phi: f64) -> Point3<f64> {
    let (s, (c0, s0)) = stored(e);
    let (u, v) = (e.at.u_ref(), e.at.v_ref());
    let (a, b) = e.semi_axes();
    // `cos(θ₀ + φ) − cos θ₀` and `sin(θ₀ + φ) − sin θ₀`, `θ₀` a quarter
    // turn's multiple.
    let (sp, k) = (phi.sin(), -2.0 * (0.5 * phi).sin().powi(2));
    let (ku, kv) = (c0 * k - s0 * sp, s0 * k + c0 * sp);
    s + (u * (a * ku) + v * (b * kv))
}

fn lift(p: Point3<f64>) -> Point3<Interval> {
    Point3::new(
        Interval::from_f64(p.x),
        Interval::from_f64(p.y),
        Interval::from_f64(p.z),
    )
}

fn lift_v(v: Vec3<f64>) -> Vec3<Interval> {
    Vec3::new(
        Interval::from_f64(v.x),
        Interval::from_f64(v.y),
        Interval::from_f64(v.z),
    )
}

fn holds(x: Interval, want: f64, slack: f64) -> bool {
    x.lo() <= want + slack && want - slack <= x.hi() && x.hi() - x.lo() <= 4.0 * slack
}

/// **A point's height over the plane is read from the stored point.**
/// Red where the far ellipse's is read from its centre (off by `1e-7`).
#[test]
fn a_points_height_over_the_plane_is_read_from_the_stored_point() {
    let e = far();
    let (s, _) = stored(&e);
    let ei = Curve3::Ellipse(e).map_scalar(Interval::from_f64);
    let ki = ei.conic().unwrap();
    for h in [1e-12, -3e-11, 0.0] {
        let p = s + e.v_ref() * 0.3 + e.axis() * h;
        let want = (p - s).dot(e.axis());
        let got = e.height_of(p);
        assert!(
            (got - want).abs() <= 1e-15,
            "h {h:e}: height {got:e}, want {want:e}"
        );
        assert!(
            holds(ki.height_of(lift(p)), want, 1e-15),
            "h {h:e}: the Interval height {:?} holds {want:e}",
            ki.height_of(lift(p))
        );
    }
    for c in centred() {
        let k = c.conic().unwrap();
        let p = k.point_at(0.4) + k.axis() * 1e-9;
        assert!(
            (k.height_of(p) - 1e-9).abs() <= 1e-15,
            "{c:?}: {:e}",
            k.height_of(p)
        );
    }
}

/// **A foreign point's level, and its unit coordinates, are read from
/// the stored point.** A point `d` off the far arc beside its anchor
/// reads a clearance of `d`; read from the centre it would be off by
/// `u·a/2`, `5e-8` m. Its unit coordinates are its parameter's cosine
/// and sine.
#[test]
fn a_foreign_points_level_is_read_from_the_stored_point() {
    let small = |k: &dyn Conic<f64>| {
        let (a, b) = k.semi_axes();
        a.min(b)
    };
    for e in [far(), far().seamed_near(FRAC_PI_2)] {
        let ei = Curve3::Ellipse(e).map_scalar(Interval::from_f64);
        let ki = ei.conic().unwrap();
        let (_, (c0, s0)) = stored(&e);
        let theta0 = s0.atan2(c0);
        for phi in [1e-9, -1e-8, 3e-6] {
            let q = beside(&e, phi);
            let tan = e.derivative_at(theta0 + phi).normalize();
            let out = tan.cross(e.axis());
            for d in [1e-9, -2e-10, 0.0] {
                let p = q + out * d;
                let got = crate::level_clearance(e.level_at(p), small(&e));
                // The point's own rounding, at its distance from the anchor.
                let own = 8.0 * f64::EPSILON * (1.0 + (q - stored(&e).0).norm());
                assert!(
                    (got - d).abs() <= own + 1e-6 * d.abs(),
                    "φ {phi:e} d {d:e}: clearance {got:e}"
                );
                let got_i = crate::level_clearance(ki.level_at(lift(p)), Interval::from_f64(B));
                assert!(
                    holds(got_i, d, own + 1e-14),
                    "φ {phi:e} d {d:e}: at Interval {got_i:?}"
                );
            }
            let (x, y) = e.unit_in_plane(q);
            let (sin, cos) = (theta0 + phi).sin_cos();
            assert!(
                (x - cos).abs() <= 1e-15 && (y - sin).abs() <= 1e-15,
                "φ {phi:e}: unit coordinates ({x}, {y}), want ({cos}, {sin})"
            );
        }
    }
    for c in centred() {
        let k = c.conic().unwrap();
        let p = k.point_at(0.4);
        let tan = k.derivative_at(0.4).normalize();
        let got = crate::level_clearance(k.level_at(p + tan.cross(k.axis()) * 1e-7), small(&k));
        assert!((got - 1e-7).abs() <= 1e-13, "{c:?}: {got:e}");
    }
}

/// **The height along a direction is the first harmonic about the
/// stored point**: at a parameter, at its extremes and at its roots. Beside
/// the far anchor each reads to its own scale; formed about the centre
/// each is off by `u·a`, and a root posed about the centre by `u` of a
/// radian, `3e-8` m of arc.
#[test]
fn the_height_along_a_direction_is_read_about_the_stored_point() {
    let n = Vec3::new(0.3, -0.5, 0.81).normalize();
    for e in [far(), far().seamed_near(FRAC_PI_2)] {
        let (s, (c0, s0)) = stored(&e);
        let theta0 = s0.atan2(c0);
        let ei = Curve3::Ellipse(e).map_scalar(Interval::from_f64);
        let ki = ei.conic().unwrap();
        let o = s + Vec3::new(0.2, 0.1, -0.3);
        for phi in [1e-9, -4e-7] {
            let want = (beside(&e, phi) - o).dot(n);
            let r = e.height_along(o, n).running_at(theta0 + phi);
            assert!(
                r.value == e.height_along(o, n).at(theta0 + phi)
                    && (r.value - want).abs()
                        <= r.error * 4.0 + 1e-14 + 2.0 * f64::EPSILON * A * theta0.abs(),
                "φ {phi:e}: the running height {r:?} is the height and bounds its rounding"
            );
            let got = e.height_along(o, n).at(theta0 + phi);
            // A quarter turn from the seam the parameter itself carries
            // the turn: `θ₀ + φ` rounds at `u·|θ₀|` radians, so a point
            // beside the anchor holds to `2u·A·|θ₀|` metres, the semi-axis's
            // rounding and not its own. That is what this tolerance admits
            // (3.5e-7 m at `A = 1e9`): the re-seamed ellipse is read to the
            // general anchor's unit, not to the arc's scale.
            let turned = 2.0 * f64::EPSILON * A * theta0.abs();
            assert!(
                (got - want).abs() <= 1e-14 + turned,
                "φ {phi:e}: height {got:e}, want {want:e}"
            );
        }
        // The crest along the seam's direction stands at the seam's
        // vertex, which is the anchor or a quarter turn from it.
        let u = e.u_ref();
        let crest = (beside(&e, -theta0) - o).dot(u);
        let (lo, hi) = e.height_along(o, u).extremes();
        assert!(
            (hi - crest).abs() <= 1e-14 * (1.0 + crest.abs()),
            "crest {hi:e}, want {crest:e}"
        );
        let semi_u = e.semi_axes().0;
        assert!(
            (lo - (crest - 2.0 * semi_u)).abs() <= 8.0 * f64::EPSILON * semi_u,
            "trough {lo:e}, want {:e}",
            crest - 2.0 * semi_u
        );
        let (_, hi_i) = ki.height_along(lift(o), lift_v(u)).extremes();
        assert!(
            holds(hi_i, crest, 1e-14 * (1.0 + crest.abs())),
            "crest at Interval {hi_i:?}"
        );
        // A plane across the arc `0.3 m` from the anchor: one root there,
        // falling or rising as the slope reads, the other far round.
        let phi1 = 0.3 / B;
        let at = beside(&e, phi1);
        let tan = e.derivative_at(theta0 + phi1).normalize();
        for level in [0.0, 1e-6, -2e-9] {
            let roots = e.height_along(at, tan).roots(level);
            let near = roots
                .into_iter()
                .min_by(|x, y| {
                    (x - theta0 - phi1)
                        .abs()
                        .total_cmp(&(y - theta0 - phi1).abs())
                })
                .unwrap();
            assert!(
                (-core::f64::consts::PI..=core::f64::consts::PI).contains(&near),
                "level {level:e}: {near} within half a turn of the seam"
            );
            let want = theta0 + phi1 + level / e.speed_at(theta0 + phi1);
            let tol = 1e-12 * (phi1 + level.abs() / B) + 4.0 * f64::EPSILON * theta0.abs();
            assert!(
                (near - want).abs() <= tol,
                "level {level:e}: root {near:e}, want {want:e}"
            );
            let slope = |t: f64| e.derivative_at(t).dot(tan);
            assert!(
                slope(roots[0]) <= 0.0 && slope(roots[1]) >= 0.0,
                "level {level:e}: {roots:?} reads falling, rising"
            );
            let roots_i = ki
                .height_along(lift(at), lift_v(tan))
                .roots(Interval::from_f64(level));
            assert!(
                roots_i.iter().any(|r| holds(*r, want, tol + 1e-20)),
                "level {level:e}: the Interval roots {roots_i:?} hold {want:e}"
            );
        }
        // Past the crest: both roots are the crest.
        let h = e.height_along(at, tan);
        let [f, r] = h.roots(3.0 * A);
        let (_, crest) = h.extremes();
        assert!(
            f == r && (h.at(f) - crest).abs() <= 1e-6 * A,
            "past the crest: {f}, {r}, at {:e} of {crest:e}",
            h.at(f)
        );
    }
    for c in centred() {
        let k = c.conic().unwrap();
        let o = k.point_at(0.4);
        let tan = k.derivative_at(0.4).normalize();
        let roots = k.height_along(o, tan).roots(0.0);
        assert!(
            roots.iter().any(|r| (r - 0.4).abs() <= 1e-14),
            "{c:?}: {roots:?}"
        );
        let h = k.height_along(o, tan);
        let (lo, hi) = h.extremes();
        assert!(lo < 0.0 && hi > 0.0 && h.at(0.4).abs() <= 1e-15, "{c:?}");
    }
}

/// **A measurement, not a gate**: an arc-held ellipse's roots at a touch
/// on `Interval`, posed about the anchor (as [`Height::roots`] reads them)
/// beside the centre's posing of the same numbers. Prints each
/// enclosure's middle offset from the touch and its width. Run with
/// `cargo nextest run -p geom --run-ignored only an_arc_held_touchs_roots_at_interval --no-capture`.
#[test]
#[ignore = "a measurement: prints the touch's root enclosures"]
fn an_arc_held_touchs_roots_at_interval() {
    use super::{Height, anchored_roots, centred_roots};
    let e = far();
    let ki = Curve3::Ellipse(e).map_scalar(Interval::from_f64);
    let ki = ki.conic().unwrap();
    for phi in [1e-9, 1e-6, 1e-3] {
        let q = beside(&e, phi);
        let out = e.derivative_at(phi).normalize().cross(e.axis());
        for by in [0.0, -1e-12, 1e-12, -1e-9] {
            let o = q + out * by;
            let h: Height<Interval> = ki.height_along(lift(o), lift_v(out));
            let anchored = anchored_roots(
                h.at,
                h.p,
                h.q,
                (Interval::from_f64(1.0), Interval::from_f64(0.0)),
            );
            let centred = centred_roots(h.at - h.p, h.p, h.q);
            let show = |r: [Interval; 2]| {
                r.map(|x| {
                    format!(
                        "{:+.3e}±{:.1e}",
                        0.5 * (x.lo() + x.hi()) - phi,
                        0.5 * (x.hi() - x.lo())
                    )
                })
            };
            eprintln!(
                "φ {phi:e} lift {by:+e}: about the anchor {:?}, about the centre {:?}",
                show(anchored),
                show(centred)
            );
        }
    }
}

/// **At an exact touch at the anchor the roots are the anchor.** Along
/// `−u_ref` from the anchor the height is `a·(1 − cos θ) ≥ 0`, touching
/// zero at `θ = 0` alone; along `+u_ref` it is its negation. Both roots
/// are `0` (modulo a turn) at `f64`, where a tie broken toward the crest
/// reads `π`, the antipode, `2a` away; at `Interval` each enclosure holds
/// the anchor; the centre-held twin of the same locus reads the anchor.
#[test]
fn an_exact_touch_at_the_anchor_reads_the_anchor() {
    let wrap = |t: f64| {
        (t + core::f64::consts::PI).rem_euclid(core::f64::consts::TAU) - core::f64::consts::PI
    };
    for e in [
        far(),
        Ellipse::anchored(
            Point3::new(0.3, -0.2, 0.1),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
            5.0,
            3.0,
        ),
    ] {
        let (s, _) = stored(&e);
        let ei = Curve3::Ellipse(e).map_scalar(Interval::from_f64);
        let ki = ei.conic().unwrap();
        let centred = Ellipse::new(Conic::frame(&e), e.semi_axes().0, e.semi_axes().1);
        let at_vertex = centred.point_at(0.0);
        for sign in [-1.0, 1.0] {
            let n = e.u_ref() * sign;
            let roots = e.height_along(s, n).roots(0.0);
            assert!(
                roots.iter().all(|r| wrap(*r).abs() <= 1e-12),
                "{e:?} along {sign}·u: roots {roots:?}, want the anchor"
            );
            let roots_i = ki
                .height_along(lift(s), lift_v(n))
                .roots(Interval::from_f64(0.0));
            // At `Interval` each root's enclosure holds the anchor: where
            // the touch is exact the second root is `0/0` and its enclosure
            // is the turn, sound and wide.
            assert!(
                roots_i
                    .iter()
                    .all(|r| r.hi() - r.lo() >= core::f64::consts::TAU - 1e-9
                        || (r.lo() <= 1e-12 && -1e-12 <= r.hi())
                        || (r.lo() <= core::f64::consts::TAU + 1e-12
                            && core::f64::consts::TAU - 1e-12 <= r.hi())),
                "{e:?} along {sign}·u at Interval: {roots_i:?}"
            );
            let twin = centred.height_along(at_vertex, n).roots(0.0);
            assert!(
                twin.iter().all(|r| wrap(*r).abs() <= 1e-6),
                "{e:?} along {sign}·u, held at its centre: {twin:?}"
            );
        }
    }
}
