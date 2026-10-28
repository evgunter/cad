//! **A classified pair's section states the bounds that retire the
//! tube** (C2, `geom_brep::intersect::tube_bounds`): for every pair the
//! section states them for, the stated crossing-angle floor lies under
//! the true `sin θ` sampled densely along the carrier, and no point in
//! the stated moat around the carrier lies within ε of both surfaces;
//! near-tangent and near-apex poses included. A pose whose second
//! overlap component stands inside the tube states nothing, and the edge
//! still owes the tube and refuses.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, NurbsCurve3, Surface};
use geom_brep::intersect::{
    OpenConic, PlaneConeSection, Unstated, plane_cone_section, tube_bounds,
};
use geom_brep::{AnalyticRung3Refusal, Reach};
use geom_core::{Band, Interval, KnotVector, Point3, Vec3};

use crate::shared::tol::band;

fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

/// One rational quadratic span on `[0, 1]`.
fn quad(p0: Point3<f64>, p1: Point3<f64>, p2: Point3<f64>, w: f64) -> NurbsCurve3<f64> {
    let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    NurbsCurve3::new(knots, vec![p0, p1, p2], vec![1.0, w, 1.0]).unwrap()
}

/// The arc `φ ∈ [φ₀, φ₁]` (under a quarter turn) of the unit circle in
/// the frame `(e1, e2)`, carried through the linear map `map` and
/// translated to `at`: a linear image of a rational quadratic keeps its
/// weights.
fn mapped_arc(
    (phi0, phi1): (f64, f64),
    map: impl Fn(f64, f64) -> Vec3<f64>,
    at: Point3<f64>,
) -> NurbsCurve3<f64> {
    let h = 0.5 * (phi1 - phi0);
    let m = 0.5 * (phi0 + phi1);
    let on = |phi: f64, r: f64| at + map(r * phi.cos(), r * phi.sin());
    quad(on(phi0, 1.0), on(m, 1.0 / h.cos()), on(phi1, 1.0), h.cos())
}

fn plane(q: Point3<f64>, n: Vec3<f64>) -> Surface<f64> {
    let n = n.normalize();
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

fn cylinder(origin: Point3<f64>, axis: Vec3<f64>, radius: f64) -> Surface<f64> {
    let u = if axis.x.abs() < 0.9 {
        Vec3::unit_x()
    } else {
        Vec3::unit_y()
    };
    Surface::Cylinder {
        origin,
        axis,
        radius,
        u_ref: (u - axis * u.dot(axis)).normalize(),
    }
}

fn sphere(center: Point3<f64>, radius: f64) -> Surface<f64> {
    Surface::Sphere {
        center,
        radius,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

/// The cone of half-angle `alpha` about `z` with its apex at `(0, 0, 1)`.
fn cone(alpha: f64) -> Surface<f64> {
    Surface::Cone {
        apex: p(0.0, 0.0, 1.0),
        axis: Vec3::unit_z(),
        half_angle: alpha,
        u_ref: Vec3::unit_x(),
    }
}

/// A point's distance from the surface, and the surface's unit normal
/// there (for the cone, the normal of the nearer generator's plane).
fn measure(s: &Surface<f64>, y: Point3<f64>) -> (f64, Vec3<f64>) {
    match *s {
        Surface::Plane { origin, normal, .. } => ((y - origin).dot(normal).abs(), normal),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let v = y - origin;
            let r = v - axis * v.dot(axis);
            ((r.norm() - radius).abs(), r.normalize())
        }
        Surface::Sphere { center, radius, .. } => {
            let v = y - center;
            ((v.norm() - radius).abs(), v.normalize())
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let v = y - apex;
            let z = v.dot(axis);
            let radial = v - axis * z;
            let rho = radial.norm();
            let (sa, ca) = half_angle.sin_cos();
            // The distance from each nappe's generator ray in the
            // meridian half-plane: the segment's foot, or the apex.
            let ray = |side: f64| {
                let t = rho * sa + side * z * ca;
                if t >= 0.0 {
                    (rho * ca - side * z * sa).abs()
                } else {
                    rho.hypot(z)
                }
            };
            let side = if z >= 0.0 { 1.0 } else { -1.0 };
            let normal = radial.normalize() * ca - axis * (side * sa);
            (ray(1.0).min(ray(-1.0)), normal)
        }
        _ => unreachable!("the rows read quadrics"),
    }
}

/// **The stated bounds enclose the truth over the carrier.** The floor
/// lies under the true `sin θ` at every one of 2049 points of the
/// carrier; and at 129 of them, along 32 directions of the normal plane
/// and at 6 distances across the moat `(radius, radius + clearance]`, the
/// point is farther than ε from one of the surfaces.
fn encloses(what: &str, carrier: &NurbsCurve3<f64>, s1: &Surface<f64>, s2: &Surface<f64>) {
    let band = band();
    let eps = band.zero();
    let (lo, hi) = carrier.domain();
    let bounds = tube_bounds(carrier, (lo, hi), s1, s2, band)
        .unwrap_or_else(|e| panic!("{what}: the section states no bounds: {e}"));
    let truth = (0..=2048)
        .map(|i| {
            let y = carrier.eval(lo + (hi - lo) * f64::from(i) / 2048.0);
            let (_, n1) = measure(s1, y);
            let (_, n2) = measure(s2, y);
            n1.cross(n2).norm()
        })
        .fold(f64::INFINITY, f64::min);
    println!(
        "[measured] {what}: floor {:e} under the sampled sin θ {truth:e}, radius {:e}, \
         clearance {:e}",
        bounds.sin_floor, bounds.radius, bounds.clearance
    );
    assert!(
        bounds.sin_floor <= truth && bounds.sin_floor > 0.0,
        "{what}: the floor {:e} is not under the sampled sin θ {truth:e}",
        bounds.sin_floor
    );
    for i in 0..=128 {
        let y = carrier.eval(lo + (hi - lo) * f64::from(i) / 128.0);
        let (_, n1) = measure(s1, y);
        let (_, n2) = measure(s2, y);
        let e1 = n1;
        let e2 = (n2 - e1 * n2.dot(e1)).normalize();
        for k in 0..32 {
            let a = core::f64::consts::TAU * f64::from(k) / 32.0;
            let d = e1 * a.cos() + e2 * a.sin();
            for j in 1..=6 {
                let t = bounds.radius + bounds.clearance * f64::from(j) / 6.0;
                let x = y + d * t;
                let (d1, _) = measure(s1, x);
                let (d2, _) = measure(s2, x);
                assert!(
                    d1.max(d2) > eps,
                    "{what}: a point {t:e} off the carrier in the moat lies within ε of both \
                     surfaces ({d1:e}, {d2:e})"
                );
            }
        }
    }
}

/// The open conic of `plane(q, n)` and `cone(alpha)`.
fn open(alpha: f64, q: Point3<f64>, n: Vec3<f64>, band: Band) -> geom_brep::OpenConicSection<f64> {
    let reach = Reach::Measured {
        at: p(0.0, 0.0, 1.0),
        lever: 4.0,
    };
    match plane_cone_section(&plane(q, n), &cone(alpha), &reach, band) {
        Ok(PlaneConeSection::OpenConic(s)) => s,
        other => panic!("expected an open conic, got {other:?}"),
    }
}

/// The section of the plane `x = d` and `cone(alpha)` at height `y` on
/// the nappe `side`.
fn vertical_point(alpha: f64, d: f64, y: f64, side: f64) -> Point3<f64> {
    p(d, y, 1.0 + side * d.hypot(y) / alpha.tan())
}

fn nurbs(arc: &Curve3<f64>) -> NurbsCurve3<f64> {
    let Curve3::Nurbs(n) = arc else {
        panic!("a NURBS arc, got {arc:?}")
    };
    (**n).clone()
}

#[test]
fn plane_by_plane_states_the_angle_between_the_normals() {
    let carrier = quad(p(-1.0, 0.0, 0.0), p(0.2, 0.0, 0.0), p(2.0, 0.0, 0.0), 1.0);
    for angle in [1.2_f64, 0.3, 1e-3] {
        let s1 = plane(p(0.0, 0.0, 0.0), Vec3::unit_z());
        let s2 = plane(p(0.0, 0.0, 0.0), Vec3::new(0.0, angle.sin(), angle.cos()));
        encloses(&format!("plane×plane at {angle}"), &carrier, &s1, &s2);
    }
}

#[test]
fn plane_by_cylinder_states_an_ellipse_and_a_near_tangent_line() {
    let r = 0.5;
    let wall = cylinder(p(0.0, 0.0, 0.0), Vec3::unit_z(), r);
    for slope in [0.7, 3.0] {
        let carrier = mapped_arc(
            (-0.6, 0.9),
            |x, y| Vec3::new(r * x, r * y, slope * r * x),
            p(0.0, 0.0, 0.0),
        );
        let cut = plane(p(0.0, 0.0, 0.0), Vec3::new(-slope, 0.0, 1.0));
        encloses(
            &format!("plane×cylinder, slope {slope}"),
            &carrier,
            &cut,
            &wall,
        );
    }
    // The plane `x = δ` near the tangent one, `sin θ = 0.05`: the
    // generator `y = +√(R² − δ²)`.
    let s = 0.05_f64;
    let delta = r * (1.0 - s * s).sqrt();
    let y = r * s;
    let carrier = quad(p(delta, y, -1.0), p(delta, y, 0.1), p(delta, y, 2.0), 1.0);
    encloses(
        "plane×cylinder near tangent",
        &carrier,
        &plane(p(delta, 0.0, 0.0), Vec3::unit_x()),
        &wall,
    );
}

#[test]
fn plane_by_sphere_states_a_circle_and_a_near_tangent_one() {
    let ball = sphere(p(0.0, 0.0, 0.0), 1.0);
    for h in [0.6_f64, 0.999] {
        let rc = (1.0 - h * h).sqrt();
        let carrier = mapped_arc(
            (0.0, 1.2),
            |x, y| Vec3::new(rc * x, rc * y, 0.0),
            p(0.0, 0.0, h),
        );
        encloses(
            &format!("plane×sphere at height {h}"),
            &carrier,
            &plane(p(0.0, 0.0, h), Vec3::unit_z()),
            &ball,
        );
    }
}

#[test]
fn plane_by_cone_states_its_hyperbola_parabola_and_a_near_apex_arc() {
    let band = band();
    let alpha = core::f64::consts::FRAC_PI_6;
    // The axis-parallel plane `x = d` at a stand-off of 0.3 and of
    // `1e5·ε` (near the apex at the stretch's pad's scale), on either
    // nappe.
    let near = 1e5 * band.zero();
    for (d, windows) in [
        (0.3, [(-0.5, 0.8), (-0.01, 6.0)]),
        (near, [(-2.0 * near, 3.0 * near), (0.1 * near, 0.5)]),
    ] {
        let n = Vec3::unit_x();
        let q = p(d, 0.0, 0.0);
        let section = open(alpha, q, n, band);
        assert_eq!(section.conic(), OpenConic::Hyperbola);
        for side in [1.0, -1.0] {
            let branch = section
                .branch_at(p(d, 0.0, 1.0 + side), band)
                .expect("a branch");
            for (y0, y1) in windows {
                let arc = branch
                    .arc(
                        vertical_point(alpha, d, y0, side),
                        vertical_point(alpha, d, y1, side),
                        band,
                    )
                    .expect("an arc");
                encloses(
                    &format!("plane×cone hyperbola d {d} side {side} ({y0}, {y1})"),
                    &nurbs(&arc),
                    &plane(q, n),
                    &cone(alpha),
                );
            }
        }
    }
    // The parabola: the plane tilted to the generator.
    for alpha in [0.3, core::f64::consts::FRAC_PI_6, 1.2] {
        let phi = core::f64::consts::FRAC_PI_2 - alpha;
        let n = Vec3::new(phi.sin(), 0.0, phi.cos());
        let q = p(0.0, 0.0, 3.0);
        let section = open(alpha, q, n, band);
        assert_eq!(section.conic(), OpenConic::Parabola);
        let branch = section.branch_at(q, band).expect("the branch");
        let arc = branch
            .arc(branch.point(-1.5), branch.point(2.0), band)
            .expect("an arc");
        encloses(
            &format!("plane×cone parabola α {alpha}"),
            &nurbs(&arc),
            &plane(q, n),
            &cone(alpha),
        );
    }
}

#[test]
fn equal_cylinders_state_their_ellipse_away_from_the_crossing() {
    let r = 0.5;
    let (c1, c2) = (
        cylinder(p(0.0, 0.0, 0.0), Vec3::unit_x(), r),
        cylinder(p(0.0, 0.0, 0.0), Vec3::unit_y(), r),
    );
    for (sign, window) in [(1.0, (-1.2, 0.3)), (-1.0, (0.2, 1.3))] {
        // `y = sign·x` on both: `(R cos φ, sign·R cos φ, R sin φ)`.
        let carrier = mapped_arc(
            window,
            |x, z| Vec3::new(r * x, sign * r * x, r * z),
            p(0.0, 0.0, 0.0),
        );
        encloses(
            &format!("cylinder×cylinder, y = {sign}·x"),
            &carrier,
            &c1,
            &c2,
        );
    }
    // An arc reaching `φ = π/2`, where the two ellipses cross tangent,
    // states nothing.
    let carrier = mapped_arc(
        (0.9, 1.6),
        |x, z| Vec3::new(r * x, r * x, r * z),
        p(0.0, 0.0, 0.0),
    );
    let got = tube_bounds(&carrier, carrier.domain(), &c1, &c2, band());
    assert!(
        matches!(got, Err(Unstated::Pose(_))),
        "an arc through the crossing states nothing, got {got:?}"
    );
}

/// **At general poses the stated floor lies under the section's own
/// sin θ.** Each carrier is an exact arc of its pair's section (to
/// rounding), short enough that its floor reads within a few percent of
/// the truth, so a floor lifted past the truth anywhere along it is
/// caught by [`encloses`]' dense sampling:
///
/// - two equal cylinders whose axes cross at `β = π/3` (`k = 1/2`), on
///   both bisector ellipses — `x = R cos φ/sin(β/2)` along `(â₁ + â₂)`'s
///   unit, or `R cos φ/cos(β/2)` along `(â₂ − â₁)`'s, and `z = R sin φ` —
///   where the two leans `1 ∓ k` differ;
/// - an off-axis plane tilted across a cylinder, whose ellipse is the
///   circle's image under the plane's height map;
/// - a plane tilted past the cone's generator, cutting a hyperbola.
#[test]
fn general_poses_state_floors_under_the_section() {
    let r = 0.5;
    let beta = core::f64::consts::FRAC_PI_3;
    let (a1, a2) = (Vec3::unit_x(), Vec3::new(beta.cos(), beta.sin(), 0.0));
    let (c1, c2) = (
        cylinder(p(0.0, 0.0, 0.0), a1, r),
        cylinder(p(0.0, 0.0, 0.0), a2, r),
    );
    let plus = (a1 + a2).normalize();
    let minus = (a2 - a1).normalize();
    for (what, dir, scale, window) in [
        (
            "the (â₁ + â₂) ellipse",
            plus,
            r / (0.5 * beta).sin(),
            (-1.2, 0.3),
        ),
        (
            "the (â₂ − â₁) ellipse",
            minus,
            r / (0.5 * beta).cos(),
            (0.2, 1.3),
        ),
    ] {
        let carrier = mapped_arc(
            window,
            |x, z| dir * (scale * x) + Vec3::unit_z() * (r * z),
            p(0.0, 0.0, 0.0),
        );
        encloses(&format!("oblique cylinders, {what}"), &carrier, &c1, &c2);
    }

    let wall = cylinder(p(0.0, 0.0, 0.0), Vec3::unit_z(), r);
    let n = Vec3::new(-0.7, 0.3, 1.0);
    let q = p(0.1, 0.05, 0.2);
    let carrier = mapped_arc(
        (-0.4, 0.6),
        |x, y| Vec3::new(r * x, r * y, -(n.x * r * x + n.y * r * y) / n.z),
        p(0.0, 0.0, q.z + (n.x * q.x + n.y * q.y) / n.z),
    );
    encloses(
        "an off-axis tilted plane×cylinder",
        &carrier,
        &plane(q, n),
        &wall,
    );

    let band = band();
    let alpha = core::f64::consts::FRAC_PI_6;
    let phi = 1.2_f64;
    let n = Vec3::new(phi.sin(), 0.0, phi.cos());
    let q = p(0.0, 0.0, 4.0);
    let section = open(alpha, q, n, band);
    assert_eq!(section.conic(), OpenConic::Hyperbola);
    let on_section = |azimuth: f64| {
        let g = Vec3::new(
            alpha.sin() * azimuth.cos(),
            alpha.sin() * azimuth.sin(),
            alpha.cos(),
        );
        let apex = p(0.0, 0.0, 1.0);
        apex + g * ((q - apex).dot(n) / g.dot(n))
    };
    let (from, to) = (on_section(-0.6), on_section(0.4));
    let branch = section.branch_at(from, band).expect("a branch");
    let arc = branch.arc(from, to, band).expect("an arc");
    encloses(
        "a tilted plane×cone hyperbola",
        &nurbs(&arc),
        &plane(q, n),
        &cone(alpha),
    );
}

/// **The torus rows owe the tube**: a plane×torus section states no
/// bounds.
#[test]
fn a_torus_section_states_nothing() {
    let torus = Surface::Torus {
        center: p(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        major_radius: 1.25,
        minor_radius: 0.25,
        u_ref: Vec3::unit_x(),
    };
    let carrier = mapped_arc((0.0, 1.0), |x, y| Vec3::new(x, y, 0.0), p(0.0, 0.0, 0.0));
    let got = tube_bounds(
        &carrier,
        carrier.domain(),
        &plane(p(0.0, 0.0, 0.0), Vec3::unit_z()),
        &torus,
        band(),
    );
    assert!(matches!(got, Err(Unstated::Pair { .. })), "{got:?}");
}

/// **A second overlap component inside the tube owes it, and refuses.**
/// The plane `x = δ` at `sin θ = √ε/2` from tangent to the unit cylinder
/// cuts its two generators `2·sin θ = √ε` apart, inside `ε/sin θ = 2√ε`
/// of each other: the other generator's overlap stands in the tube's
/// moat, so the section states nothing for the one generator, by the
/// moat, and the edge refuses on the tube it still owes.
#[test]
fn a_second_component_inside_the_tube_owes_it_and_refuses() {
    let band = band();
    let eps = band.zero();
    let s = 0.5 * eps.sqrt();
    let delta = (1.0 - s * s).sqrt();
    let wall = cylinder(p(0.0, 0.0, 0.0), Vec3::unit_z(), 1.0);
    let cut = plane(p(delta, 0.0, 0.0), Vec3::unit_x());
    let carrier = quad(p(delta, s, -1.0), p(delta, s, 0.0), p(delta, s, 1.0), 1.0);
    assert!(
        2.0 * s < eps / s,
        "the other generator, {:e} away, lies inside ε/sin θ = {:e}",
        2.0 * s,
        eps / s
    );
    let got = tube_bounds(&carrier, carrier.domain(), &cut, &wall, band);
    assert_eq!(
        got,
        Err(Unstated::Pose(
            "the ε-overlap may reach the tube's moat (another component of the section, \
             or a near miss of the two surfaces)"
        )),
        "the near-tangent cylinder states nothing, by the moat"
    );
    let got = geom_brep::analytic_rung3(&carrier, carrier.domain(), &cut, &wall, band);
    assert!(
        matches!(
            got,
            Err(AnalyticRung3Refusal::TubeStraddles { .. }
                | AnalyticRung3Refusal::TubeNotOneArc { .. }
                | AnalyticRung3Refusal::Escalated { .. })
        ),
        "the near-tangent cylinder's generator refuses on the tube, got {got:?}"
    );
}

/// **A stretch reaching the apex's height owes the tube, and refuses.**
/// On the flat cone `α = 1.4`, the plane `x = 5e-5` beside the apex, at a
/// band of zero `1e-6` (pinned to the row's scale: the one pose of its
/// sweep near the apex where limb 2 still holds and the section is still
/// minted past the escalation), the arc's vertex stands `d/tan α ≈ 9e-6`
/// above the apex, under the stretch's pad: the padded stretch straddles
/// the apex's height, so the section states nothing, and the edge
/// refuses naming the cone, which no tube encloses.
#[test]
fn a_stretch_at_the_apex_height_owes_the_tube_and_refuses() {
    let wide = Band::new(1e-6, 1e-5).unwrap();
    let (alpha, d) = (1.4_f64, 5e-5_f64);
    let section = open(alpha, p(d, 0.0, 0.0), Vec3::unit_x(), wide);
    let branch = section
        .branch_at(p(d, 0.0, 2.0), wide)
        .expect("the upper branch");
    let arc = branch
        .arc(
            vertical_point(alpha, d, 2e-5, 1.0),
            vertical_point(alpha, d, 1e-4, 1.0),
            wide,
        )
        .expect("an arc");
    let arc = nurbs(&arc);
    let (q, n) = (p(d, 0.0, 0.0), Vec3::unit_x());
    let got = tube_bounds(&arc, arc.domain(), &plane(q, n), &cone(alpha), wide);
    assert_eq!(
        got,
        Err(Unstated::Pose(
            "the carrier's stretch reaches the height of the cone's apex"
        )),
        "the flat cone beside its apex states nothing"
    );
    let got = geom_brep::analytic_rung3(&arc, arc.domain(), &plane(q, n), &cone(alpha), wide);
    assert_eq!(
        got,
        Err(AnalyticRung3Refusal::NoTubeEnclosure {
            operand: geom::SurfaceKind::Cone
        }),
        "the edge owes the tube and refuses naming the cone"
    );
}

/// **A stretch that nears the apex, clear of its height, owes the tube
/// by the moat.** The plane `x = d` just past the band's escalation
/// beside the apex of `α = π/4`, and the arc from the height `45ε` above
/// it to `200ε`: the stretch's pad (`32ε/s₀ ≈ 32ε`) keeps clear of the
/// apex's height and the crossing angle's floor holds (`sin θ ≈ 0.8` at
/// the padded stretch's lowest height, `≈ 13ε`), but the cone's factor
/// `m = 2|z|·sin α`, floored at that lowest height, is under what the
/// moat needs at `w = 8ε/s` (`m·3ε > 64ε²/s²`, `|z| ≳ 24ε` here). Read at
/// the stretch's highest height instead, `m` would clear it.
#[test]
fn a_stretch_nearing_the_apex_owes_the_tube_by_the_moat() {
    let band = band();
    let eps = band.zero();
    let alpha = core::f64::consts::FRAC_PI_4;
    let d = 1.1 * band.escalate();
    let section = open(alpha, p(d, 0.0, 0.0), Vec3::unit_x(), band);
    let branch = section
        .branch_at(p(d, 0.0, 2.0), band)
        .expect("the upper branch");
    let at_height = |z: f64| {
        let rho = z * alpha.tan();
        vertical_point(alpha, d, (rho * rho - d * d).sqrt(), 1.0)
    };
    let arc = nurbs(
        &branch
            .arc(at_height(45.0 * eps), at_height(200.0 * eps), band)
            .expect("an arc"),
    );
    let got = tube_bounds(
        &arc,
        arc.domain(),
        &plane(p(d, 0.0, 0.0), Vec3::unit_x()),
        &cone(alpha),
        band,
    );
    assert_eq!(
        got,
        Err(Unstated::Pose(
            "the ε-overlap may reach the tube's moat (another component of the section, \
             or a near miss of the two surfaces)"
        )),
        "the stretch nearing the apex fails the moat"
    );
}

/// **Over a box, the bounds hold for every member or are not stated.**
/// At the interval scalar the hyperbola's plane offset widened by
/// `1e-13` still states its bounds and the edge certifies; widened by a
/// millimetre, no section point is certified near the carrier for every
/// member, so nothing is stated.
#[test]
fn over_a_box_the_bounds_hold_for_every_member_or_are_not_stated() {
    let band = band();
    let alpha = core::f64::consts::FRAC_PI_6;
    let d = 0.3;
    let section = open(alpha, p(d, 0.0, 0.0), Vec3::unit_x(), band);
    let branch = section.branch_at(p(d, 0.0, 2.0), band).expect("a branch");
    let arc = nurbs(
        &branch
            .arc(
                vertical_point(alpha, d, -0.5, 1.0),
                vertical_point(alpha, d, 0.8, 1.0),
                band,
            )
            .expect("an arc"),
    );
    let lifted = arc.map_scalar(geom_core::Real::from_f64);
    let (d0, d1) = arc.domain();
    let params = (Interval::from_bounds(d0, d0), Interval::from_bounds(d1, d1));
    let wall = cone(alpha).map_scalar(geom_core::Real::from_f64);
    for (width, states) in [(1e-13, true), (1e-3, false)] {
        let cut = Surface::Plane {
            origin: Point3::new(
                Interval::from_bounds(d - width, d + width),
                Interval::from_bounds(0.0, 0.0),
                Interval::from_bounds(0.0, 0.0),
            ),
            normal: Vec3::new(
                Interval::from_bounds(1.0, 1.0),
                Interval::from_bounds(0.0, 0.0),
                Interval::from_bounds(0.0, 0.0),
            ),
            u_ref: Vec3::new(
                Interval::from_bounds(0.0, 0.0),
                Interval::from_bounds(1.0, 1.0),
                Interval::from_bounds(0.0, 0.0),
            ),
        };
        let got = tube_bounds(&lifted, params, &cut, &wall, band);
        if states {
            assert!(
                got.is_ok(),
                "width {width:e}: the bounds are stated, got {got:?}"
            );
        } else {
            assert_eq!(
                got,
                Err(Unstated::Pose(
                    "no point of the section is certified within the moat of the carrier's point"
                )),
                "width {width:e}"
            );
        }
    }
}
