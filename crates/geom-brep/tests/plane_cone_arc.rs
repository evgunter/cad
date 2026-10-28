//! **The plane×cone parabola and hyperbola, minted exactly** (C1): the
//! section's arc over a window is a rational-quadratic `Curve3::Nurbs`,
//! weights `(1, w, 1)` per span, on both surfaces to rounding, with `w`
//! the closed form's, and limb 2 certifies it on the cone through the
//! analytic rung-3 lane, where the pair's section states the bounds that
//! retire the tube (C2).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, NurbsCurve3, Surface};
use geom_brep::Reach;
use geom_brep::intersect::{OpenConic, PlaneConeSection, SPAN_WEIGHT_MAX, plane_cone_section};
use geom_core::{Point3, Vec3};

use crate::shared::tol::band;

/// The cone of half-angle `alpha` about `z` with its apex at `(0, 0, 1)`.
fn cone(alpha: f64) -> Surface<f64> {
    Surface::Cone {
        apex: Point3::new(0.0, 0.0, 1.0),
        axis: Vec3::unit_z(),
        half_angle: alpha,
        u_ref: Vec3::unit_x(),
    }
}

/// The plane through `q` with unit normal `n`.
fn plane(q: Point3<f64>, n: Vec3<f64>) -> Surface<f64> {
    let u = if n.x.abs() < 0.9 {
        Vec3::unit_x()
    } else {
        Vec3::unit_y()
    };
    Surface::Plane {
        origin: q,
        normal: n,
        u_ref: (u - n * u.dot(n)).normalize(),
    }
}

/// A reach of `lever` metres about the apex.
fn reach(lever: f64) -> Reach<f64> {
    Reach::Measured {
        at: Point3::new(0.0, 0.0, 1.0),
        lever,
    }
}

/// A point's distance from the double cone `cone(alpha)`: exact for a
/// point whose foot lies on its own nappe.
fn off_cone(alpha: f64, p: Point3<f64>) -> f64 {
    let h = p.z - 1.0;
    let rho = p.x.hypot(p.y);
    (rho * alpha.cos() - h.abs() * alpha.sin()).abs()
}

/// The arc's NURBS payload.
fn nurbs(arc: &Curve3<f64>) -> &NurbsCurve3<f64> {
    let Curve3::Nurbs(n) = arc else {
        panic!("the arc is a NURBS carrier, got {arc:?}")
    };
    n
}

/// The arc's worst distance from the plane `(q, n)` and from the cone
/// over `samples` points of its domain.
fn worst_residual(arc: &NurbsCurve3<f64>, alpha: f64, (q, n): (Point3<f64>, Vec3<f64>)) -> f64 {
    let (lo, hi) = arc.domain();
    (0..=4096)
        .map(|i| {
            let p = arc.eval(lo + (hi - lo) * f64::from(i) / 4096.0);
            ((p - q).dot(n)).abs().max(off_cone(alpha, p))
        })
        .fold(0.0, f64::max)
}

/// The section of `plane(q, n)` and `cone(alpha)`, as an open conic.
fn open(alpha: f64, q: Point3<f64>, n: Vec3<f64>) -> geom_brep::OpenConicSection<f64> {
    match plane_cone_section(&plane(q, n), &cone(alpha), &reach(4.0), band()) {
        Ok(PlaneConeSection::OpenConic(s)) => s,
        other => panic!("expected an open conic, got {other:?}"),
    }
}

/// The point of the plane `x = d`'s hyperbola on `cone(alpha)` at
/// height `y`, on the nappe `side`.
fn vertical_point(alpha: f64, d: f64, y: f64, side: f64) -> Point3<f64> {
    Point3::new(d, y, 1.0 + side * d.hypot(y) / alpha.tan())
}

/// **A hyperbolic arc lies on both surfaces to rounding, on either
/// branch and either way round, and its ends are the window's.** The
/// plane `x = 0.3` along the axis of the cone `α = π/6`, read on the
/// upper nappe's branch and the lower's, over a short window, one that
/// crosses the vertex, and one long enough to be halved; and a plane
/// tilted past the generator (`φ = 1.2`).
#[test]
fn a_hyperbolic_arc_lies_on_both_surfaces() {
    let alpha = core::f64::consts::FRAC_PI_6;
    let d = 0.3;
    let n = Vec3::unit_x();
    let q = Point3::new(d, 0.0, 0.0);
    let section = open(alpha, q, n);
    assert_eq!(section.conic(), OpenConic::Hyperbola);
    for side in [1.0, -1.0] {
        let branch = section
            .branch_at(Point3::new(d, 0.0, 1.0 + side), band())
            .expect("the seed names a branch");
        for (y0, y1) in [(0.1, 0.2), (-0.5, 0.8), (2.5, -3.0), (-0.01, 6.0)] {
            let (from, to) = (
                vertical_point(alpha, d, y0, side),
                vertical_point(alpha, d, y1, side),
            );
            let arc = branch
                .arc(from, to, band())
                .expect("the window is on the branch");
            let curve = nurbs(&arc);
            let (lo, hi) = curve.domain();
            let ends = curve
                .eval(lo)
                .distance(from)
                .max(curve.eval(hi).distance(to));
            let scale = (from - Point3::origin())
                .norm()
                .max((to - Point3::origin()).norm());
            let worst = worst_residual(curve, alpha, (q, n));
            println!(
                "[measured] hyperbola side {side} window ({y0}, {y1}): {} spans, worst \
                 residual {worst:e} at scale {scale}, ends {ends:e}",
                curve.weights().len() / 2
            );
            assert!(
                worst <= 32.0 * f64::EPSILON * scale && ends <= 8.0 * f64::EPSILON * scale,
                "side {side} window ({y0}, {y1}): worst {worst:e}, ends {ends:e}"
            );
        }
    }
    let phi = 1.2_f64;
    let n = Vec3::new(phi.sin(), 0.0, phi.cos());
    let q = Point3::new(0.0, 0.0, 4.0);
    let section = open(alpha, q, n);
    assert_eq!(section.conic(), OpenConic::Hyperbola);
    // Two points of the tilted plane's section on the upper nappe: its
    // trace on the cone at azimuths about the plane's lowest generator.
    let on_section = |azimuth: f64| {
        let g = Vec3::new(
            alpha.sin() * azimuth.cos(),
            alpha.sin() * azimuth.sin(),
            alpha.cos(),
        );
        let apex = Point3::new(0.0, 0.0, 1.0);
        let t = (q - apex).dot(n) / g.dot(n);
        apex + g * t
    };
    let (from, to) = (on_section(-2.1), on_section(1.5));
    let branch = section.branch_at(from, band()).expect("a branch");
    let arc = branch
        .arc(from, to, band())
        .expect("the window is on the branch");
    let worst = worst_residual(nurbs(&arc), alpha, (q, n));
    let scale = (from - Point3::origin())
        .norm()
        .max((to - Point3::origin()).norm());
    println!("[measured] tilted hyperbola: worst residual {worst:e} at scale {scale}");
    assert!(worst <= 32.0 * f64::EPSILON * scale, "tilted: {worst:e}");
}

/// **A seed names the branch on its own nappe whichever way the plane
/// leans across the axis.** The section's near and far branches swap
/// labels as `c = a·n` changes sign, and `branch_at` reads its side by
/// the same sign, so the branch a seed gets must not swap: the plane
/// `x = d` leaned by `ε` about `y` (`c = sin ε`, `ε` small of either
/// sign, and zero), with a seed above the apex and one below, gives each
/// the branch whose vertex is on the plane, on the cone and on the
/// seed's nappe, and the arc of it between two of the section's points
/// on that nappe (traced down the cone's generators) lies on both
/// surfaces.
#[test]
fn a_seed_names_its_own_nappes_branch_on_either_lean() {
    let alpha = core::f64::consts::FRAC_PI_6;
    let d = 0.3;
    let q = Point3::new(d, 0.0, 0.0);
    let apex = Point3::new(0.0, 0.0, 1.0);
    for lean in [1e-3, 1e-12, 0.0, -0.0, -1e-12, -1e-3] {
        let n = Vec3::new(f64::cos(lean), 0.0, f64::sin(lean));
        let section = open(alpha, q, n);
        assert_eq!(section.conic(), OpenConic::Hyperbola, "lean {lean:e}");
        // The plane's trace on the generator at `azimuth`: above the
        // apex where the generator's `x` is positive, below it where
        // negative.
        let on_section = |azimuth: f64| {
            let g = Vec3::new(
                alpha.sin() * azimuth.cos(),
                alpha.sin() * azimuth.sin(),
                alpha.cos(),
            );
            apex + g * ((q - apex).dot(n) / g.dot(n))
        };
        for (side, azimuths) in [(1.0, (-0.9, 0.6)), (-1.0, (2.2, -2.5))] {
            let branch = section
                .branch_at(Point3::new(d, 0.0, 1.0 + side), band())
                .expect("the seed names a branch");
            let v = branch.vertex();
            let (on_plane, on_cone) = ((v - q).dot(n).abs(), off_cone(alpha, v));
            assert!(
                on_plane <= 1e-14 && on_cone <= 1e-14 && (v.z - 1.0) * side > 0.1,
                "lean {lean:e} side {side}: the branch's vertex {v:?} is not on the seed's \
                 nappe's branch (plane {on_plane:e}, cone {on_cone:e})"
            );
            let (from, to) = (on_section(azimuths.0), on_section(azimuths.1));
            let arc = branch
                .arc(from, to, band())
                .expect("the window is on the seed's branch");
            let worst = worst_residual(nurbs(&arc), alpha, (q, n));
            assert!(
                worst <= 64.0 * f64::EPSILON,
                "lean {lean:e} side {side}: worst residual {worst:e}"
            );
        }
    }
}

/// **Each span's weight is the closed form's, `cosh` of half its
/// hyperbolic span, and the arc is halved until it is at most `√2`.**
/// On the plane `x = d` the hyperbola is `y = d·sinh t`, so a window's
/// hyperbolic span is `asinh(y₁/d) − asinh(y₀/d)`; halving it `k` times
/// for the least `k` that brings `cosh(span/2^(k+1))` to `√2` or under
/// gives `2^k` spans of that weight. `NurbsCurve3::new` takes the
/// weights above 1. Each span's mid-parameter point, the one point the
/// weight places, is on the cone.
#[test]
fn the_weight_is_the_closed_forms() {
    let alpha = core::f64::consts::FRAC_PI_6;
    let d = 0.3;
    let section = open(alpha, Point3::new(d, 0.0, 0.0), Vec3::unit_x());
    let branch = section
        .branch_at(Point3::new(d, 0.0, 2.0), band())
        .expect("a branch");
    for (y0, y1) in [(0.1, 0.2), (-0.5, 0.8), (-0.01, 6.0), (-40.0, 30.0)] {
        let arc = branch
            .arc(
                vertical_point(alpha, d, y0, 1.0),
                vertical_point(alpha, d, y1, 1.0),
                band(),
            )
            .expect("an arc");
        let curve = nurbs(&arc);
        let span = (y1 / d).asinh() - (y0 / d).asinh();
        let mut spans = 1_u32;
        while (span / f64::from(2 * spans)).cosh() > SPAN_WEIGHT_MAX {
            spans *= 2;
        }
        let want = (span / f64::from(2 * spans)).cosh();
        let weights = curve.weights();
        assert_eq!(
            weights.len(),
            2 * spans as usize + 1,
            "({y0}, {y1}): span count"
        );
        let worst_w = weights
            .iter()
            .skip(1)
            .step_by(2)
            .map(|w| (w - want).abs() / want)
            .fold(0.0, f64::max);
        println!("[measured] window ({y0}, {y1}): weight relative error {worst_w:e}");
        for (i, w) in weights.iter().enumerate() {
            let expected = if i % 2 == 1 { want } else { 1.0 };
            assert!(
                (w - expected).abs() <= 8.0 * f64::EPSILON * expected,
                "({y0}, {y1}) weight {i}: {w} against the closed form {expected}"
            );
        }
        assert!(
            want > 1.0 && want <= SPAN_WEIGHT_MAX,
            "({y0}, {y1}): {want}"
        );
        let control = curve.control();
        for s in 0..spans as usize {
            let (p0, p1, p2) = (control[2 * s], control[2 * s + 1], control[2 * s + 2]);
            let mid = Point3::origin()
                + ((p0 - Point3::origin())
                    + (p1 - Point3::origin()) * (2.0 * want)
                    + (p2 - Point3::origin()))
                    / (2.0 + 2.0 * want);
            let scale = (p0 - Point3::origin())
                .norm()
                .max((p2 - Point3::origin()).norm());
            assert!(
                off_cone(alpha, mid) <= 16.0 * f64::EPSILON * scale,
                "({y0}, {y1}) span {s}: the weighted midpoint stands {:e} off the cone",
                off_cone(alpha, mid)
            );
        }
        println!("[measured] window ({y0}, {y1}): {spans} spans of weight {want}");
    }
}

/// **A parabola pose gives `w = 1`, one span at any length.** The plane
/// tilted to the generator, `φ = π/2 − α`, on cones from a needle to a
/// flat one.
#[test]
fn a_parabola_has_unit_weights() {
    for alpha in [0.05, core::f64::consts::FRAC_PI_6, 1.3] {
        let phi = core::f64::consts::FRAC_PI_2 - alpha;
        let n = Vec3::new(phi.sin(), 0.0, phi.cos());
        let q = Point3::new(0.0, 0.0, 3.0);
        let section = open(alpha, q, n);
        assert_eq!(section.conic(), OpenConic::Parabola, "α {alpha}");
        let branch = section
            .branch_at(q, band())
            .expect("the parabola's one branch");
        let vertex = branch.vertex();
        // Two section points: the vertex's own generator plane is `y = 0`,
        // so step off it along `v̂ = n × û = ±y` and read the branch's
        // own point there, then check it against the cone independently.
        let (from, to) = (branch.point(-1.5), branch.point(2.0));
        let arc = branch.arc(from, to, band()).expect("an arc");
        let curve = nurbs(&arc);
        assert_eq!(
            curve.weights(),
            &[1.0, 1.0, 1.0],
            "α {alpha}: one span, unit weights"
        );
        let scale = (from - Point3::origin())
            .norm()
            .max((to - Point3::origin()).norm())
            .max((vertex - Point3::origin()).norm());
        let worst = worst_residual(curve, alpha, (q, n));
        println!("[measured] parabola α {alpha}: worst residual {worst:e} at scale {scale}");
        assert!(worst <= 32.0 * f64::EPSILON * scale, "α {alpha}: {worst:e}");
    }
}

/// **A window off the branch, or of one point, refuses.**
#[test]
fn a_window_off_the_branch_refuses() {
    let alpha = core::f64::consts::FRAC_PI_6;
    let d = 0.3;
    let section = open(alpha, Point3::new(d, 0.0, 0.0), Vec3::unit_x());
    let branch = section
        .branch_at(Point3::new(d, 0.0, 2.0), band())
        .expect("a branch");
    let on = vertical_point(alpha, d, 0.5, 1.0);
    let off = on + Vec3::new(0.0, 0.0, 1e-3);
    assert!(
        matches!(
            branch.arc(on, off, band()),
            Err(geom_brep::ConicArcError::WindowOffSection {
                end: geom_brep::intersect::WindowEnd::To,
                ..
            })
        ),
        "a window end a millimetre off the branch refuses"
    );
    // The lower branch's point read on the upper branch.
    let below = vertical_point(alpha, d, 0.5, -1.0);
    assert!(
        matches!(
            branch.arc(below, on, band()),
            Err(geom_brep::ConicArcError::WindowOffSection { .. })
        ),
        "the other branch's point refuses"
    );
    assert!(
        matches!(
            branch.arc(on, on, band()),
            Err(geom_brep::ConicArcError::EmptyWindow)
        ),
        "a one-point window refuses"
    );
}

/// **Limb 2 certifies the arc on the cone and on the plane, and the
/// section's bounds retire the tube** (C2): the plane×cone section states
/// the crossing angle's floor over each window and the clearance of the
/// tube it buys, so the analytic rung-3 lane owes no tube, which has no
/// cone enclosure.
#[test]
fn limb_two_certifies_the_arc_and_the_sections_bounds_retire_the_tube() {
    let alpha = core::f64::consts::FRAC_PI_6;
    let d = 0.3;
    let n = Vec3::unit_x();
    let q = Point3::new(d, 0.0, 0.0);
    let section = open(alpha, q, n);
    let branch = section
        .branch_at(Point3::new(d, 0.0, 2.0), band())
        .expect("a branch");
    for (y0, y1) in [(-0.5, 0.8), (-0.01, 6.0)] {
        let arc = branch
            .arc(
                vertical_point(alpha, d, y0, 1.0),
                vertical_point(alpha, d, y1, 1.0),
                band(),
            )
            .expect("an arc");
        let curve = nurbs(&arc);
        let got =
            geom_brep::analytic_rung3(curve, curve.domain(), &plane(q, n), &cone(alpha), band());
        assert_eq!(
            got,
            Ok(()),
            "({y0}, {y1}): limb 2 holds and the tube is retired"
        );
    }
}

/// **The extent of a steep arc, bracketed.** On a hyperbolic arc of a
/// needle cone, symmetric about its vertex and turning nearly a half
/// turn, the farthest pair of points is the vertex and an end, not the
/// two ends: the chord is under a fifth of the diameter. The bound from
/// below (`edge_extent_inf`) reads the vertex as the arc's middle and
/// comes within a thousandth of the diameter; the bound from above
/// (`edge_extent_sup`, the control net's farthest pair) stays over it. A lever from below that
/// stopped at the chord would let a margin Positive at the diameter
/// read a definite Zero.
#[test]
fn the_extent_on_a_steep_arc() {
    for (alpha, (y0, y1), steep) in [
        (core::f64::consts::FRAC_PI_6, (-2.0, 2.0), false),
        (0.1, (-2.0, 2.0), true),
        (0.05, (-6.0, 6.0), true),
        (0.1, (-1.0, 4.0), false),
    ] {
        let d = 0.3;
        let section = open(alpha, Point3::new(d, 0.0, 0.0), Vec3::unit_x());
        let branch = section
            .branch_at(Point3::new(d, 0.0, 2.0), band())
            .expect("a branch");
        let (from, to) = (
            vertical_point(alpha, d, y0, 1.0),
            vertical_point(alpha, d, y1, 1.0),
        );
        let arc = branch.arc(from, to, band()).expect("an arc");
        let curve = nurbs(&arc);
        let (lo, hi) = curve.domain();
        let pts: Vec<Point3<f64>> = (0..=256)
            .map(|i| curve.eval(lo + (hi - lo) * f64::from(i) / 256.0))
            .collect();
        let diameter = pts
            .iter()
            .flat_map(|a| pts.iter().map(move |b| a.distance(*b)))
            .fold(0.0, f64::max);
        let chord = from.distance(to);
        let inf = geom_brep::edge_extent_inf(&arc, lo, hi, chord);
        let sup = geom_brep::edge_extent_sup(&arc, lo, hi, chord);
        println!(
            "[measured] steep arc α {alpha}, y {y0}..{y1}: chord {chord}, diameter {diameter}, \
             inf {inf}, sup {sup}, chord/D {}, inf/D {}, sup/D {}",
            chord / diameter,
            inf / diameter,
            sup / diameter
        );
        assert_eq!(
            chord < 0.25 * diameter,
            steep,
            "α {alpha}: the chord reads {} of the diameter",
            chord / diameter
        );
        assert!(
            inf <= diameter * (1.0 + 1e-12),
            "α {alpha}: the bound from below {inf} exceeds the diameter {diameter}"
        );
        assert!(
            inf >= diameter * 0.999,
            "α {alpha}: the bound from below {inf} is short of the diameter {diameter} \
             (the chord is {chord})"
        );
        assert!(
            sup >= diameter,
            "α {alpha}: the bound from above {sup} is short of the diameter {diameter}"
        );
    }
}

/// The point of the section of the plane `(q, n)`, `n` square to `y`,
/// with `cone(alpha)` at height `y` along `ŷ`, on the branch that stays
/// finite as the plane tilts to a generator: solved in the world frame,
/// apart from the mint's own frame, as the quadratic
/// `A·x² + B·x + C = 0` in the plane coordinate `x` along `n × ŷ`, its
/// grazing coefficient `A = (n_z·cos α − n_x·sin α)(n_z·cos α +
/// n_x·sin α)` read with its cancellation compensated.
fn section_at_height(alpha: f64, (q, n): (Point3<f64>, Vec3<f64>), y: f64) -> Point3<f64> {
    let (sa, ca) = alpha.sin_cos();
    let p = n.x * sa;
    let grazing = n.z.mul_add(ca, -p) - n.x.mul_add(sa, -p);
    let a = grazing * n.z.mul_add(ca, p);
    let h0 = q.z - 1.0;
    let b = 2.0 * (q.x * n.z * ca * ca + h0 * n.x * sa * sa);
    let c = (q.x * q.x + y * y) * ca * ca - h0 * h0 * sa * sa;
    let x = -2.0 * c / (b + (b * b - 4.0 * a * c).sqrt().copysign(b));
    Point3::new(q.x + x * n.z, y, q.z - x * n.x)
}

/// **A hyperbolic arc's points are the section's own, near the
/// parabola too.** The residual against the two surfaces cannot see an
/// arc that slides along its own curve, so the window's ends and every
/// sampled point of the arc are held to the section's point at the same
/// height, solved apart from the mint ([`section_at_height`]). The
/// planes tilt past the generator by `tilt`, from a plain hyperbola to
/// grazing ones whose far branch stands kilometres off, over windows
/// reaching tens of metres down the near branch.
#[test]
fn an_arc_is_the_sections_own_points_near_the_parabola() {
    let pi6 = core::f64::consts::FRAC_PI_6;
    for (alpha, tilt, reach) in [
        (pi6, 0.3, 2.0),
        (pi6, 1e-2, 20.0),
        (pi6, 1e-3, 20.0),
        (pi6, 1e-4, 60.0),
        (pi6, 1e-5, 60.0),
        (0.05, 1e-3, 5.0),
        (1.3, 1e-3, 20.0),
    ] {
        let phi = core::f64::consts::FRAC_PI_2 - alpha + tilt;
        let n = Vec3::new(phi.sin(), 0.0, phi.cos());
        let q = Point3::new(0.0, 0.0, 3.0);
        let section = open(alpha, q, n);
        assert_eq!(
            section.conic(),
            OpenConic::Hyperbola,
            "α {alpha} tilt {tilt}"
        );
        let at = |y: f64| section_at_height(alpha, (q, n), y);
        let (from, to) = (at(-reach), at(0.8 * reach));
        let branch = section.branch_at(from, band()).expect("a branch");
        let scale = (from - Point3::origin())
            .norm()
            .max((to - Point3::origin()).norm());
        // The budget: the closed form's rounding, plus what the conic-type
        // discriminant's own rounding moves a point at `X` from the vertex
        // along the curve, at most `X²·δK/|L|` with `δK ≤ ε·M²` (`D` is a
        // difference of two products whose sum is `M`, each rounded to
        // half an ulp): a grazing plane's far section is ill-conditioned
        // in the plane, and that rounding of `D` is all it may cost.
        let (sa, ca) = alpha.sin_cos();
        let m = sa * n.x.hypot(n.y) + ca * n.z.abs();
        let l = 2.0 * (Point3::new(0.0, 0.0, 1.0) - q).dot(n).abs() * sa * ca;
        let vertex = branch.vertex();
        let budget = |p: Point3<f64>| {
            let x = p.distance(vertex);
            f64::EPSILON * (4.0 * scale + m * m * x * x / l)
        };
        // The end meter reads an absolute gap, so where the budget at an
        // end reaches the band the window may refuse on that rounding
        // alone (module docs, "The window"); then only the refusal's
        // grounds are checked.
        let arc = match branch.arc(from, to, band()) {
            Ok(arc) => arc,
            Err(
                e @ (geom_brep::ConicArcError::Escalated(_)
                | geom_brep::ConicArcError::WindowOffSection { .. }),
            ) => {
                let reach_of_rounding = budget(from).max(budget(to));
                println!(
                    "SKIPPED α {alpha} tilt {tilt}: the end meter refused ({e}) where the ends' \
                     rounding budget {reach_of_rounding:e} m meets the band {:e} m",
                    band().zero()
                );
                assert!(
                    reach_of_rounding >= band().zero(),
                    "α {alpha} tilt {tilt}: refused ({e}) with an end budget of \
                     {reach_of_rounding:e} m, under the band"
                );
                continue;
            }
            Err(e) => panic!("α {alpha} tilt {tilt}: the window is on the branch, got {e}"),
        };
        let curve = nurbs(&arc);
        let (lo, hi) = curve.domain();
        let worst = |p: Point3<f64>, truth: Point3<f64>| p.distance(truth) / budget(truth);
        let ends = worst(curve.eval(lo), from).max(worst(curve.eval(hi), to));
        let points = (0..=4096)
            .map(|i| {
                let p = curve.eval(lo + (hi - lo) * f64::from(i) / 4096.0);
                worst(p, at(p.y))
            })
            .fold(0.0, f64::max);
        println!(
            "[measured] α {alpha} tilt {tilt} window y ({}, {}): {} spans at scale {scale}, \
             ends {ends:.3} of the budget, points {points:.3}",
            -reach,
            0.8 * reach,
            curve.weights().len() / 2
        );
        assert!(
            ends <= 1.0 && points <= 1.0,
            "α {alpha} tilt {tilt}: ends {ends:.3} of the budget, points {points:.3}"
        );
    }
}
