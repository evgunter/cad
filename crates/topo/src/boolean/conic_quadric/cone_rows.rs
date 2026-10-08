//! **The conic × cone rows** of the one conic × quadric door.
//!
//! Each answer is checked against the geometry, never against the door's
//! algebra: the cone's quadric form is evaluated at each point of the
//! carrier from the point itself, its sign changes sampled densely and
//! bisected, and a point's distance from the double cone is read in its
//! meridian half-plane as the distance to the nearer generator RAY.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use super::*;
use crate::boolean::conic_oracle::{crossings, ellipse, extremes, unit};
use geom_core::{Point3, Tol, Vec3};
use test_utils::fuzz;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn cone(apex: [f64; 3], axis: [f64; 3], half_angle: f64) -> geom::Surface<f64> {
    let axis = Vec3::from_array(axis).normalize();
    geom::Surface::Cone {
        apex: Point3::from_array(apex),
        axis,
        half_angle,
        u_ref: axis.orthonormal_basis().0,
    }
}

/// `(apex, axis, half-angle)` of a cone surface.
fn parts(s: &geom::Surface<f64>) -> (Point3<f64>, Vec3<f64>, f64) {
    let geom::Surface::Cone {
        apex,
        axis,
        half_angle,
        ..
    } = *s
    else {
        unreachable!("the rows build cones")
    };
    (apex, axis, half_angle)
}

/// The cone's quadric form at `p`, from `p`: positive outside both
/// nappes.
fn form(s: &geom::Surface<f64>, p: Point3<f64>) -> f64 {
    let (apex, axis, a) = parts(s);
    let q = p - apex;
    let h = q.dot(axis);
    let rho2 = q.norm_squared() - h * h;
    rho2 * a.cos().powi(2) - h * h * a.sin().powi(2)
}

/// The distance from `p` to the double cone: in the meridian half-plane
/// `(ρ, h)`, the distance to the nearer of the rays `s·(sin α, ±cos α)`,
/// `s ≥ 0`.
fn distance(s: &geom::Surface<f64>, p: Point3<f64>) -> f64 {
    let (apex, axis, a) = parts(s);
    let q = p - apex;
    let h = q.dot(axis);
    let rho = (q - axis * h).norm();
    let (sa, ca) = a.sin_cos();
    [ca, -ca]
        .into_iter()
        .map(|c| {
            let along = rho * sa + h * c;
            if along <= 0.0 {
                rho.hypot(h)
            } else {
                (rho * c - h * sa).abs()
            }
        })
        .fold(f64::INFINITY, f64::min)
}

fn door(e: &geom::Curve3<f64>, s: &geom::Surface<f64>, t0: f64, t1: f64) -> CircleRoots<f64> {
    conic_quadric_roots(e, t0, t1, s, band()).unwrap()
}

fn circle(c: [f64; 3], n: [f64; 3], r: f64) -> geom::Curve3<f64> {
    let n = Vec3::from_array(n).normalize();
    geom::Curve3::Circle {
        center: Point3::from_array(c),
        axis: n,
        radius: r,
        u_ref: n.orthonormal_basis().0,
    }
}

/// The door's certified roots on `[t0, t0 + 1]` match the oracle's sign
/// changes round the whole turn, in number and place, each on the cone.
/// Returns them, sorted.
fn assert_matches_oracle(
    label: &str,
    e: &geom::Curve3<f64>,
    s: &geom::Surface<f64>,
    t0: f64,
    want: usize,
) -> Vec<f64> {
    let t1 = t0 + 1.0;
    let CircleRoots::Certified { count, thetas } = door(e, s, t0, t1) else {
        panic!("{label}: certified roots, got {:?}", door(e, s, t0, t1));
    };
    assert_eq!(count, want, "{label}: the certified count");
    let mid = (t0 + t1) / 2.0;
    let mut got = thetas[..count].to_vec();
    for &t in &got {
        assert!((t - mid).abs() <= PI, "{label}: {t} within π of {mid}");
        let off = distance(s, e.eval(t));
        assert!(off < 1e-12, "{label}: root {t} lies {off} off the cone");
    }
    got.sort_by(f64::total_cmp);
    let truth = crossings(|t| form(s, e.eval(t)), mid - PI, mid + PI);
    assert_eq!(got.len(), truth.len(), "{label}: {got:?} vs {truth:?}");
    for (a, b) in got.iter().zip(&truth) {
        assert!(
            (a - b).abs() < 1e-9,
            "{label}: root {a} vs the oracle's {b}"
        );
    }
    got
}

/// How many of the roots lie on the nappe `axis` opens along.
fn on_the_opening_nappe(e: &geom::Curve3<f64>, s: &geom::Surface<f64>, roots: &[f64]) -> usize {
    let (apex, axis, _) = parts(s);
    roots
        .iter()
        .filter(|&&t| (e.eval(t) - apex).dot(axis) > 0.0)
        .count()
}

/// The door's certified `roots` are the oracle's crossings `truth` (the
/// quadric form's sign changes round the turn, sorted), one for one, and
/// each lies within `eps` of its own ALONG THE ARC: on a shallow crossing
/// a root can lie on the cone and still be far along the carrier from
/// the true one, so its distance from the cone does not place it. The
/// place is widened only by the oracle's own resolution, the form's
/// rounding at the root over its slope along the arc.
fn assert_placed(
    label: &str,
    e: &geom::Curve3<f64>,
    s: &geom::Surface<f64>,
    roots: &[f64],
    truth: &[f64],
    eps: f64,
) {
    let (apex, _, _) = parts(s);
    let mut got = roots.to_vec();
    got.sort_by(f64::total_cmp);
    assert_eq!(
        got.len(),
        truth.len(),
        "{label}: {got:?} certified, the quadric form changes sign at {truth:?}"
    );
    for (g, t) in got.iter().zip(truth) {
        let step = 1e-7;
        let (before, after) = (e.eval(t - step), e.eval(t + step));
        let speed = (after - before).norm() / (2.0 * step);
        let slope = (form(s, after) - form(s, before)) / (2.0 * step);
        let p = e.eval(*t);
        let q = (p - apex).norm();
        let rounding = 8.0 * f64::EPSILON * q * (q + (p - Point3::origin()).norm());
        let resolved = speed * rounding / slope.abs();
        let along = (e.eval(*g) - p).norm();
        assert!(
            along <= eps + resolved,
            "{label}: the root {g} lies {along:e} along the arc from the oracle's {t} \
             ({resolved:e} unresolved)"
        );
    }
}

/// **Crossings of a cone**, two and four per turn, at several arcs: a
/// tilted circle through one nappe's wall, an ellipse across both of a
/// nappe's generators in a meridian plane, and a circle about the apex in
/// a meridian plane, which meets each nappe twice — two of its roots are
/// on the nappe a face below the apex does not hold, and the door
/// certifies all four: which nappe a face holds is its caller's question.
#[test]
fn crossings_match_the_quadric_form() {
    let wide = cone([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.6);
    let narrow = cone([0.3, -0.2, 0.1], [0.2, 0.1, 1.0], 0.25);
    for t0 in [0.0, 1.7, -2.9] {
        assert_matches_oracle(
            "a tilted circle across a wide cone",
            &circle([0.6, 0.0, 1.0], [0.4, 0.1, 1.0], 0.3),
            &wide,
            t0,
            2,
        );
        assert_matches_oracle(
            "an ellipse across both generators",
            &ellipse([0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0], 1.2, 0.4),
            &wide,
            t0,
            4,
        );
        let about_apex = circle([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], 0.5);
        let roots = assert_matches_oracle("a circle about the apex", &about_apex, &wide, t0, 4);
        assert_eq!(
            on_the_opening_nappe(&about_apex, &wide, &roots),
            2,
            "two roots on each nappe"
        );
        assert_matches_oracle(
            "an ellipse across a narrow leaning cone",
            &ellipse(
                [0.5, -0.1, 1.5],
                [1.0, 0.2, 0.3],
                [0.0, 1.0, 0.0],
                0.6,
                0.35,
            ),
            &narrow,
            t0,
            4,
        );
    }
}

/// **A coaxial circle**: its quadric form is constant along it, and the
/// first-harmonic arm reads it. On the cone it lies ON it; off the cone
/// by a definite margin, inside or outside, it is a miss.
#[test]
fn a_coaxial_circle_lies_on_the_cone_or_misses_it() {
    let a = 0.4f64;
    let s = cone([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], a);
    let on = circle([0.0, 0.0, 2.0], [0.0, 0.0, 1.0], 2.0 * a.tan());
    assert!(
        matches!(door(&on, &s, 0.0, TAU), CircleRoots::OnSurface),
        "the rim circle lies on the cone, got {:?}",
        door(&on, &s, 0.0, TAU)
    );
    for r in [0.5, 1.2] {
        let off = circle([0.0, 0.0, 2.0], [0.0, 0.0, 1.0], r);
        assert!(
            matches!(door(&off, &s, 0.0, TAU), CircleRoots::Miss),
            "radius {r} at height 2 misses, got {:?}",
            door(&off, &s, 0.0, TAU)
        );
        let signs: Vec<bool> = (0..=1000)
            .map(|k| form(&s, off.eval(TAU * f64::from(k) / 1000.0)) > 0.0)
            .collect();
        assert!(
            signs.iter().all(|&x| x == signs[0]),
            "radius {r}: the quadric form keeps one sign along it"
        );
    }
}

/// **A constant `F` in the band is not a residual in the band.** On a
/// narrow cone (`α = 0.01`), a coaxial circle twenty zero bands off the
/// wall has `|F| = |res|·(ρ cos α + h sin α)/R ≈ 0.02·|res|`, in the
/// band; its residual is definitely off it. Read on `F` alone it lay ON
/// the cone; the floor re-read leaves it uncertain.
#[test]
fn a_narrow_cones_near_coaxial_circle_is_not_on_it() {
    let b = band();
    let a = 0.01f64;
    let s = cone([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], a);
    let h = 1.0;
    let off = 20.0 * b.zero();
    let rim = circle([0.0, 0.0, h], [0.0, 0.0, 1.0], h * a.tan() + off / a.cos());
    let res = distance(&s, rim.eval(0.3));
    assert!(
        res > b.escalate(),
        "the pose: the circle is {res} off the cone, past the escalation threshold"
    );
    let got = door(&rim, &s, 0.0, TAU);
    assert!(
        matches!(got, CircleRoots::Uncertain | CircleRoots::Miss),
        "a circle {res} off the cone is not on it, got {got:?}"
    );
}

/// **At the apex the door refuses.** A circle through the apex in a
/// meridian plane, and a circle crossing a nappe a few zero bands from
/// the apex, at each of the three ε rows: neither a root nor a miss is
/// answered, and the refusal is `Uncertain` — the subdivision cannot read
/// a definite side beside the apex, or the slack meter refuses the root
/// it isolates there, before the apex rung reads it
/// ([`off_the_apex`]'s docs; the rung's own row is below).
#[test]
fn a_root_at_the_apex_refuses() {
    let s = cone([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.6);
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        let through = circle([0.5, 0.0, 0.0], [0.0, 1.0, 0.0], 0.5);
        let near = circle([0.5, 0.0, 3.0 * eps], [0.0, 1.0, 0.0], 0.5);
        for (label, e) in [("through the apex", through), ("beside the apex", near)] {
            let got = conic_quadric_roots(&e, 0.0, TAU, &s, band);
            assert!(
                matches!(got, Ok(CircleRoots::Uncertain)),
                "ε {eps}, {label}: refused, got {got:?}"
            );
        }
    }
}

/// **The apex rung answers `AtApex` for a root at the apex**, and for
/// one in the escalation band of it, and leaves roots definitely off it
/// standing: the rung read directly, on certified roots the door would
/// hand it (the door's own meters refuse such a root first, above).
#[test]
fn the_apex_rung_refuses_a_root_at_the_apex() {
    let b = band();
    // A circle through the apex at `θ = 0`: `C(0) = c + ρ·û` is the apex.
    let e = circle([0.5, 0.0, 0.0], [0.0, 1.0, 0.0], 0.5);
    let geom::Curve3::Circle { u_ref, .. } = e else {
        unreachable!("a circle")
    };
    let conic = geom_brep::Conic::of(&e).unwrap();
    // `θ` placing `C(θ)` about `shift` metres along the circle from `C(0)`.
    let toward = if u_ref.x < 0.0 { 0.0 } else { PI };
    let at = |shift: f64| toward + shift / 0.5;
    let apex = conic.point(toward);
    let certified = |thetas: &[f64]| {
        let mut all = [0.0; 8];
        all[..thetas.len()].copy_from_slice(thetas);
        CircleRoots::Certified {
            count: thetas.len(),
            thetas: all,
        }
    };
    for (label, shift) in [
        ("at the apex", 0.0),
        ("half a zero band off it", 0.5 * b.zero()),
        ("in its escalation band", 0.5 * (b.zero() + b.escalate())),
    ] {
        let got = off_the_apex(&conic, apex, certified(&[at(shift), at(1.0)]), b);
        assert!(matches!(got, CircleRoots::AtApex), "{label}: got {got:?}");
    }
    let clear = certified(&[at(3.0 * b.escalate()), at(1.0)]);
    let got = off_the_apex(&conic, apex, clear, b);
    assert!(
        matches!(got, CircleRoots::Certified { count: 2, .. }),
        "roots definitely off the apex stand, got {got:?}"
    );
    for other in [
        CircleRoots::Miss,
        CircleRoots::OnSurface,
        CircleRoots::Uncertain,
    ] {
        let label = format!("{other:?}");
        let got = off_the_apex(&conic, apex, other, b);
        assert_eq!(
            format!("{got:?}"),
            label,
            "a non-root answer passes through"
        );
    }
}

/// **A root the slack meter cannot place refuses** (the meter's row,
/// `bool_conic_cone_root_slack`). A circle of radius 1 crosses a right
/// cone transversally `d` from its apex, at ε 1e-12. The root is
/// bisected on the residual itself, so its true error is that
/// residual's rounding over its slope (about `1e-16` m here), which no
/// `f64` oracle resolves; what the meter is answerable for is its own
/// charge, so the row pins that against a closed form.
///
/// The charge is `speed·reach/lever`. The lever is at most `F`'s slope
/// at the root, `|Q′(θ)|/R` with `R` at least the carrier's farthest
/// reach from the apex `|c| + ρ`; at `θ = 0`, where `C′ = −ρ·x̂` and
/// `⊥q = d sin α·x̂`, `|Q′| = 2 cos²α · d sin α · ρ`. The reach is at
/// least the residual's charged rounding, which holds the `cos θ` ulp
/// carried along `û = −ẑ` into the height, `2u·ρ·sin α`
/// ([`geom_brep::conic_cone_residual`]). Where that bound is past the
/// escalation threshold the door must refuse; far from the apex the same
/// circle's roots are certified and placed.
#[test]
fn a_root_the_slack_meter_cannot_place_refuses() {
    let alpha = core::f64::consts::FRAC_PI_4;
    let s = cone([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], alpha);
    let band = Band::new(1e-12, 1e-11).unwrap();
    let rho = 1.0;
    let crossing = |d: f64| {
        let on = Point3::new(d * alpha.sin(), 0.0, d * alpha.cos());
        geom::Curve3::Circle {
            center: on + Vec3::new(0.0, 0.0, rho),
            axis: Vec3::new(0.0, 1.0, 0.0),
            radius: rho,
            u_ref: Vec3::new(0.0, 0.0, -1.0),
        }
    };
    for d in [1e-6, 1e-5] {
        let e = crossing(d);
        let geom::Curve3::Circle { center, .. } = e else {
            unreachable!("a circle")
        };
        let farthest = (center - Point3::origin()).norm() + rho;
        let slope = 2.0 * alpha.cos().powi(2) * d * alpha.sin() * rho;
        let reach = 2.0 * geom_core::UNIT_ROUNDOFF * rho * alpha.sin();
        let charge = rho * reach * farthest / slope;
        assert!(
            charge > band.escalate(),
            "the pose: the meter charges at least {charge:e} at d = {d:e}"
        );
        let got = conic_quadric_roots(&e, -0.5, 0.5, &s, band);
        assert!(
            matches!(got, Ok(CircleRoots::Uncertain)),
            "d = {d:e}: a root charged {charge:e} is refused, got {got:?}"
        );
    }
    let e = crossing(0.3);
    let Ok(CircleRoots::Certified { count, thetas }) = conic_quadric_roots(&e, -0.5, 0.5, &s, band)
    else {
        panic!("far from the apex the roots are certified")
    };
    let mut got = thetas[..count].to_vec();
    got.sort_by(f64::total_cmp);
    let truth = crossings(|t| form(&s, e.eval(t)), -PI, PI);
    assert_eq!(got.len(), truth.len(), "{got:?} vs the oracle's {truth:?}");
    for (g, t) in got.iter().zip(&truth) {
        assert!(
            (g - t).abs() * rho < 1e-12,
            "the root {g} vs the oracle's {t}"
        );
    }
}

/// **Every answer holds against the quadric form** (a counterexample
/// search): random circles and ellipses (semi-axes 1 mm to 1 m, any
/// stored sign) through a random point of a random cone (half-angle 0.05
/// to 1.5), at each of the three ε rows:
///
/// - every certified root lies on the double cone, its distance from it
///   inside the zero band, and none within the band of the apex;
/// - a certified answer has exactly the quadric form's sign changes
///   round the turn, each root within the zero band, along the arc, of
///   its own ([`assert_placed`]);
/// - a `Miss` is never certified for a carrier that crosses the cone or
///   comes within the zero band of it;
/// - `OnSurface` only for a carrier whose every sampled point lies within
///   the zero band of the cone.
#[test]
fn certified_answers_hold_against_the_quadric_form() {
    let mut rng = fuzz::start("conic_quadric::cone_answers_hold");
    let dense = 20_000;
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        let (mut certified, mut misses, mut uncertain, mut apex) = (0, 0, 0, 0);
        for i in 0..fuzz::scaled(400) {
            let s = cone(
                [
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                ],
                unit(&mut rng).to_array(),
                rng.range(0.05, 1.5),
            );
            let (tip, axis, a) = parts(&s);
            // A point of the cone, a slant `l` from the apex.
            let l = rng.range(0.05, 2.0);
            let (u, v) = axis.orthonormal_basis();
            let phi = rng.range(0.0, TAU);
            let nappe = if rng.below(2) == 0 { 1.0 } else { -1.0 };
            let radial = u * phi.cos() + v * phi.sin();
            let on = tip + (axis * (nappe * a.cos()) + radial * a.sin()) * l;
            let big = rng.range(1e-3, 1.0);
            let small = big / rng.range(1.0, 6.0);
            let n = unit(&mut rng);
            let w = unit(&mut rng);
            let uref = (w - n * w.dot(n)).normalize();
            let e = if i % 3 == 0 {
                geom::Curve3::Circle {
                    center: on - uref * big,
                    axis: n,
                    radius: big,
                    u_ref: uref,
                }
            } else {
                let major = if i % 4 == 1 { -big } else { big };
                geom::Curve3::Ellipse {
                    center: on - uref * major,
                    axis: n,
                    major,
                    minor: small,
                    u_ref: uref,
                }
            };
            let t0 = rng.range(0.0, TAU);
            let t1 = t0 + rng.range(0.1, TAU);
            let mid = (t0 + t1) / 2.0;
            let label = format!(
                "ε {eps}, case {i}: {e:?} against {s:?} on [{t0}, {t1}] — {}",
                fuzz::replay()
            );
            let Ok(found) = conic_quadric_roots(&e, t0, t1, &s, band) else {
                uncertain += 1;
                continue;
            };
            let at = |k: u32| mid - PI + TAU * f64::from(k) / f64::from(dense);
            let truth = crossings(|t| form(&s, e.eval(t)), mid - PI, mid + PI);
            let changes = truth.len();
            let closest = (0..=dense)
                .map(|k| distance(&s, e.eval(at(k))))
                .fold(f64::INFINITY, f64::min);
            match found {
                CircleRoots::Certified { count, thetas } => {
                    certified += 1;
                    for &t in &thetas[..count] {
                        let p = e.eval(t);
                        let off = distance(&s, p);
                        assert!(off <= eps, "{label}: root {t} lies {off} off the cone");
                        let from_apex = (p - tip).norm();
                        assert!(
                            from_apex > eps,
                            "{label}: root {t} lies {from_apex} from the apex"
                        );
                    }
                    assert_placed(&label, &e, &s, &thetas[..count], &truth, eps);
                }
                CircleRoots::Miss => {
                    misses += 1;
                    assert!(
                        changes == 0 && closest > eps,
                        "{label}: a Miss {closest} from the cone, {changes} sign changes"
                    );
                }
                CircleRoots::OnSurface => {
                    let (_, far) = extremes(|t| distance(&s, e.eval(t)), mid - PI, mid + PI);
                    assert!(far <= eps, "{label}: OnSurface, but a point lies {far} off");
                }
                CircleRoots::AtApex => apex += 1,
                CircleRoots::Uncertain => uncertain += 1,
                CircleRoots::CountDisagrees => panic!("{label}: CountDisagrees"),
            }
        }
        println!(
            "ε {eps}: {certified} certified, {misses} misses, {apex} at the apex, \
             {uncertain} uncertain or escalated"
        );
    }
}

/// **Grazes and apex passes never certify what they cannot** (a
/// counterexample search): circles tangent to a random cone at a random
/// point off its apex, and circles through its apex, each moved off by
/// `k·ε` along the cone's normal (or the axis), `k` in `[−4, 4]`, at each
/// of the three ε rows:
///
/// - a `Miss` only for a carrier the oracle finds definitely clear;
/// - every certified root on the cone and off the band of the apex, the
///   count exactly the oracle's and each root in its place along the arc
///   ([`assert_placed`]).
#[test]
fn grazes_and_apex_passes_never_certify_what_they_cannot() {
    let mut rng = fuzz::start("conic_quadric::cone_grazes_and_apex");
    let dense = 20_000;
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        let (mut certified, mut misses, mut refused) = (0, 0, 0);
        for i in 0..fuzz::scaled(300) {
            let s = cone(
                [0.0, 0.0, 0.0],
                unit(&mut rng).to_array(),
                rng.range(0.1, 1.4),
            );
            let (tip, axis, a) = parts(&s);
            let (u, v) = axis.orthonormal_basis();
            let phi = rng.range(0.0, TAU);
            let radial = u * phi.cos() + v * phi.sin();
            let k = rng.range(-4.0, 4.0) * eps;
            let r = rng.range(0.05, 1.0);
            let e = if i % 2 == 0 {
                // Tangent at `p`: centred out along the cone's outward
                // normal there, in the plane of that normal and the
                // circle direction square to the generator.
                let l = rng.range(0.2, 2.0);
                let p = tip + (axis * a.cos() + radial * a.sin()) * l;
                let normal = radial * a.cos() - axis * a.sin();
                let along = axis.cross(radial).normalize();
                geom::Curve3::Circle {
                    center: p + normal * (r + k),
                    axis: along.cross(normal).normalize(),
                    radius: r,
                    u_ref: normal * -1.0,
                }
            } else {
                // Through the apex, in a plane holding the axis.
                let side = axis.cross(radial).normalize();
                geom::Curve3::Circle {
                    center: tip + radial * r + axis * k,
                    axis: side,
                    radius: r,
                    u_ref: radial * -1.0,
                }
            };
            let t0 = rng.range(0.0, TAU);
            let t1 = t0 + rng.range(0.1, TAU);
            let mid = (t0 + t1) / 2.0;
            let label = format!(
                "ε {eps}, case {i}: {e:?} against {s:?} on [{t0}, {t1}] — {}",
                fuzz::replay()
            );
            let Ok(found) = conic_quadric_roots(&e, t0, t1, &s, band) else {
                refused += 1;
                continue;
            };
            let at = |j: u32| mid - PI + TAU * f64::from(j) / f64::from(dense);
            let truth = crossings(|t| form(&s, e.eval(t)), mid - PI, mid + PI);
            let changes = truth.len();
            let closest = (0..=dense)
                .map(|j| distance(&s, e.eval(at(j))))
                .fold(f64::INFINITY, f64::min);
            match found {
                CircleRoots::Certified { count, thetas } => {
                    certified += 1;
                    for &t in &thetas[..count] {
                        let p = e.eval(t);
                        let off = distance(&s, p);
                        assert!(off <= eps, "{label}: root {t} lies {off} off the cone");
                        let from_apex = (p - tip).norm();
                        assert!(
                            from_apex > eps,
                            "{label}: root {t} lies {from_apex} from the apex"
                        );
                    }
                    assert_placed(&label, &e, &s, &thetas[..count], &truth, eps);
                }
                CircleRoots::Miss => {
                    misses += 1;
                    assert!(
                        changes == 0 && closest > eps,
                        "{label}: a Miss {closest} from the cone, {changes} sign changes"
                    );
                }
                CircleRoots::OnSurface => panic!("{label}: no such circle lies on a cone"),
                CircleRoots::CountDisagrees => panic!("{label}: CountDisagrees"),
                CircleRoots::AtApex | CircleRoots::Uncertain => refused += 1,
            }
        }
        println!("ε {eps}: {certified} certified, {misses} misses, {refused} refused");
    }
}
