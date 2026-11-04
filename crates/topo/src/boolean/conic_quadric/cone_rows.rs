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

fn door(
    e: &geom::Curve3<f64>,
    s: &geom::Surface<f64>,
    t0: f64,
    t1: f64,
) -> Result<CircleRoots<f64>, Unsettled> {
    conic_quadric_roots(e, t0, t1, s, band()).unwrap()
}

fn circle(c: [f64; 3], n: [f64; 3], r: f64) -> geom::Curve3<f64> {
    let n = Vec3::from_array(n).normalize();
    geom::Curve3::circle(Point3::from_array(c), n, r, n.orthonormal_basis().0)
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
    let Ok(CircleRoots::Certified(found)) = door(e, s, t0, t1) else {
        panic!("{label}: certified roots, got {:?}", door(e, s, t0, t1));
    };
    let count = found.count();
    assert_eq!(count, want, "{label}: the certified count");
    let mid = (t0 + t1) / 2.0;
    let mut got = found.roots().to_vec();
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
    let mut got = roots.to_vec();
    got.sort_by(f64::total_cmp);
    assert_eq!(
        got.len(),
        truth.len(),
        "{label}: {got:?} certified, the quadric form changes sign at {truth:?}"
    );
    for (g, t) in got.iter().zip(truth) {
        let resolved = resolution(e, s, *t);
        let along = (e.eval(*g) - e.eval(*t)).norm();
        assert!(
            along <= eps + resolved,
            "{label}: the root {g} lies {along:e} along the arc from the oracle's {t} \
             ({resolved:e} unresolved)"
        );
    }
}

/// The oracle's own resolution at its crossing `t`, along the arc: the
/// quadric form's rounding there over its slope along the carrier.
fn resolution(e: &geom::Curve3<f64>, s: &geom::Surface<f64>, t: f64) -> f64 {
    let (apex, _, _) = parts(s);
    let step = 1e-7;
    let (before, after) = (e.eval(t - step), e.eval(t + step));
    let speed = (after - before).norm() / (2.0 * step);
    let slope = (form(s, after) - form(s, before)) / (2.0 * step);
    let p = e.eval(t);
    let q = (p - apex).norm();
    let rounding = 8.0 * f64::EPSILON * q * (q + (p - Point3::origin()).norm());
    speed * rounding / slope.abs()
}

/// **A certified answer holds against the oracle round the turn about
/// `mid`, the apex's double root included.** Every root lies on the
/// double cone, within `eps` of it. A root within `eps` of the apex is
/// the carrier through it (`through_the_apex`): there are none or two,
/// and they stand only for the oracle's crossings within `eps` of them
/// along the arc (at most two; a carrier through the apex alone has
/// none). Every other crossing of the oracle is one of the door's other
/// roots, one for one and in its place ([`assert_placed`]): a genuine
/// crossing beside the apex that the double root swallowed is counted
/// missing.
fn assert_holds(
    label: &str,
    e: &geom::Curve3<f64>,
    s: &geom::Surface<f64>,
    roots: &[f64],
    mid: f64,
    eps: f64,
) {
    let (tip, _, _) = parts(s);
    for &t in roots {
        let off = distance(s, e.eval(t));
        assert!(off <= eps, "{label}: root {t} lies {off} off the cone");
    }
    let truth = crossings(|t| form(s, e.eval(t)), mid - PI, mid + PI);
    let (at_apex, off_apex): (Vec<f64>, Vec<f64>) = roots
        .iter()
        .partition(|&&t| (e.eval(t) - tip).norm() <= eps);
    if at_apex.is_empty() {
        assert_placed(label, e, s, &off_apex, &truth, eps);
        return;
    }
    assert_eq!(
        at_apex.len(),
        2,
        "{label}: the apex's double root, got {roots:?}"
    );
    let apex_root = e.eval(at_apex[0]);
    let (beside, others): (Vec<f64>, Vec<f64>) = truth
        .iter()
        .partition(|&&t| (e.eval(t) - apex_root).norm() <= eps + resolution(e, s, t));
    assert!(
        beside.len() <= 2,
        "{label}: the apex's double root stands for {beside:?}"
    );
    assert_placed(label, e, s, &off_apex, &others, eps);
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
        matches!(door(&on, &s, 0.0, TAU), Ok(CircleRoots::On(_))),
        "the rim circle lies on the cone, got {:?}",
        door(&on, &s, 0.0, TAU)
    );
    for r in [0.5, 1.2] {
        let off = circle([0.0, 0.0, 2.0], [0.0, 0.0, 1.0], r);
        assert!(
            matches!(door(&off, &s, 0.0, TAU), Ok(CircleRoots::Miss)),
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
        matches!(got, Err(_) | Ok(CircleRoots::Miss)),
        "a circle {res} off the cone is not on it, got {got:?}"
    );
}

/// **Through the apex the door reads the carrier whole; beside it, it
/// refuses.** A circle through the apex in a meridian plane, at each of
/// the three ε rows, certifies the apex's double root and the circle's
/// other two crossings, each the oracle's in its place
/// (`through_the_apex`, [`assert_holds`]). A circle crossing a nappe a few
/// zero bands from the apex answers neither a root nor a miss: the
/// subdivision cannot read a definite side beside the apex, or the slack
/// meter refuses the root it isolates there, before the apex rung reads
/// it, or the subdivision reads that side in the band's gap and the door
/// escalates ([`off_the_apex`]'s docs; the rung's own row is below).
#[test]
fn a_root_at_the_apex_is_the_carrier_through_it_and_beside_it_refuses() {
    let s = cone([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.6);
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        let through = circle([0.5, 0.0, 0.0], [0.0, 1.0, 0.0], 0.5);
        let got = conic_quadric_roots(&through, 0.0, TAU, &s, band);
        let Ok(Ok(CircleRoots::Certified(found))) = &got else {
            panic!("ε {eps}, through the apex: got {got:?}");
        };
        let label = format!("ε {eps}, through the apex");
        let at_apex = |t: f64| (through.eval(t) - Point3::origin()).norm() <= eps;
        let apex_roots = found.roots().iter().filter(|&&t| at_apex(t)).count();
        assert_eq!(apex_roots, 2, "{label}: {found:?}");
        // The other two are the circle's crossings of the generators
        // through the apex's meridian, each the oracle's, in its place.
        assert_eq!(found.count(), 4, "{label}: {found:?}");
        assert_holds(&label, &through, &s, found.roots(), PI, eps);
        // Its centre moved out along `x`, so the circle passes `3ε` from
        // the apex (along the axis it would pass `9ε²` from it: through).
        let near = circle([0.5 + 3.0 * eps, 0.0, 0.0], [0.0, 1.0, 0.0], 0.5);
        let got = conic_quadric_roots(&near, 0.0, TAU, &s, band);
        // A side beside the apex the subdivision reads in the band's
        // gap is a touch, escalated on the door's decision.
        assert!(
            match &got {
                Ok(Err(_)) => true,
                Err(BooleanError::Escalated { diag, .. }) => {
                    diag.predicate == Some("bool_conic_quadric_sub_side")
                }
                Ok(Ok(_)) | Err(_) => false,
            },
            "ε {eps}, beside the apex: refused, got {got:?}"
        );
    }
}

/// **The apex rung refuses `AtApex` for a root at the apex**, and for
/// one in the escalation band of it, and leaves roots definitely off it
/// standing: the rung read directly, on certified roots the door would
/// hand it (the door's own meters refuse such a root first, above).
#[test]
fn the_apex_rung_refuses_a_root_at_the_apex() {
    let b = band();
    // A circle through the apex at `θ = 0`: `C(0) = c + ρ·û` is the apex.
    let e = circle([0.5, 0.0, 0.0], [0.0, 1.0, 0.0], 0.5);
    let geom::Curve3::Circle(geom::Circle {
        frame: geom::Frame { u_ref, .. },
        ..
    }) = e
    else {
        unreachable!("a circle")
    };
    let conic = e.conic().unwrap();
    // `θ` placing `C(θ)` about `shift` metres along the circle from `C(0)`.
    let toward = if u_ref.x < 0.0 { 0.0 } else { PI };
    let at = |shift: f64| toward + shift / 0.5;
    let apex = conic.point_at(toward);
    let certified = |thetas: &[f64]| CircleRoots::of(thetas).unwrap();
    for (label, shift) in [
        ("at the apex", 0.0),
        ("half a zero band off it", 0.5 * b.zero()),
        ("in its escalation band", 0.5 * (b.zero() + b.escalate())),
    ] {
        let got = off_the_apex(&conic, apex, certified(&[at(shift), at(1.0)]), b);
        assert!(
            matches!(got, Err(Unsettled::AtApex)),
            "{label}: got {got:?}"
        );
    }
    let clear = certified(&[at(3.0 * b.escalate()), at(1.0)]);
    let got = off_the_apex(&conic, apex, clear, b);
    assert!(
        matches!(got, Ok(CircleRoots::Certified(found)) if found.count() == 2),
        "roots definitely off the apex stand, got {got:?}"
    );
    for other in [CircleRoots::Miss, CircleRoots::On(OnCarrier)] {
        let label = format!("{:?}", Ok::<_, Unsettled>(other));
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
        geom::Curve3::circle(
            on + Vec3::new(0.0, 0.0, rho),
            Vec3::new(0.0, 1.0, 0.0),
            rho,
            Vec3::new(0.0, 0.0, -1.0),
        )
    };
    for d in [1e-6, 1e-5] {
        let e = crossing(d);
        let geom::Curve3::Circle(geom::Circle {
            frame: geom::Frame { origin: center, .. },
            ..
        }) = e
        else {
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
            matches!(got, Ok(Err(_))),
            "d = {d:e}: a root charged {charge:e} is refused, got {got:?}"
        );
    }
    let e = crossing(0.3);
    let Ok(Ok(CircleRoots::Certified(found))) = conic_quadric_roots(&e, -0.5, 0.5, &s, band) else {
        panic!("far from the apex the roots are certified")
    };
    let mut got = found.roots().to_vec();
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
/// - `On` only for a carrier whose every sampled point lies within
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
                geom::Curve3::circle(on - uref * big, n, big, uref)
            } else {
                let major = if i % 4 == 1 { -big } else { big };
                geom::Curve3::Ellipse(geom::Ellipse::new(
                    geom::Frame::new(on - uref * major, n, uref),
                    major,
                    small,
                ))
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
                Ok(CircleRoots::Certified(found)) => {
                    let (count, thetas) = (found.count(), found.roots());
                    certified += 1;
                    assert_holds(&label, &e, &s, &thetas[..count], mid, eps);
                }
                Ok(CircleRoots::Miss) => {
                    misses += 1;
                    assert!(
                        changes == 0 && closest > eps,
                        "{label}: a Miss {closest} from the cone, {changes} sign changes"
                    );
                }
                Ok(CircleRoots::On(_)) => {
                    let (_, far) = extremes(|t| distance(&s, e.eval(t)), mid - PI, mid + PI);
                    assert!(far <= eps, "{label}: OnSurface, but a point lies {far} off");
                }
                Err(Unsettled::AtApex) => apex += 1,
                Err(_) => uncertain += 1,
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
                geom::Curve3::circle(
                    p + normal * (r + k),
                    along.cross(normal).normalize(),
                    r,
                    normal * -1.0,
                )
            } else {
                // Through the apex, in a plane holding the axis.
                let side = axis.cross(radial).normalize();
                geom::Curve3::circle(tip + radial * r + axis * k, side, r, radial * -1.0)
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
                Ok(CircleRoots::Certified(found)) => {
                    let (count, thetas) = (found.count(), found.roots());
                    certified += 1;
                    assert_holds(&label, &e, &s, &thetas[..count], mid, eps);
                }
                Ok(CircleRoots::Miss) => {
                    misses += 1;
                    assert!(
                        changes == 0 && closest > eps,
                        "{label}: a Miss {closest} from the cone, {changes} sign changes"
                    );
                }
                Ok(CircleRoots::On(_)) => panic!("{label}: no such circle lies on a cone"),
                Err(Unsettled::AtApex) | Err(_) => refused += 1,
            }
        }
        println!("ε {eps}: {certified} certified, {misses} misses, {refused} refused");
    }
}

/// **A crossing beside the apex is never read as the apex's double
/// root** (the spread rung, `bool_conic_cone_apex_spread`). Three
/// carriers a few zero bands from a cone's apex, found by a reviewer's
/// counterexample search (PR 6424), at the ε each was found at: each
/// genuinely crosses the cone within some tens of zero bands of the
/// apex. Their apex is within the band of the carrier, so the carrier is
/// read whole; its spread is past the band, and the door refuses
/// [`Unsettled::AtApex`] rather than let the double root stand for the
/// crossing. Read with the spread forced into the band, the first
/// certifies the double root at `3.842013` against the oracle's crossing
/// at `3.842026`, which [`assert_holds`] counts missing.
#[test]
fn a_crossing_beside_the_apex_is_never_the_apex_double_root() {
    let frame = |c: [f64; 3], n: [f64; 3], u: [f64; 3]| {
        geom::Frame::new(
            Point3::from_array(c),
            Vec3::from_array(n),
            Vec3::from_array(u),
        )
    };
    let cone_at =
        |apex: [f64; 3], axis: [f64; 3], half_angle: f64, u_ref: [f64; 3]| geom::Surface::Cone {
            apex: Point3::from_array(apex),
            axis: Vec3::from_array(axis),
            half_angle,
            u_ref: Vec3::from_array(u_ref),
        };
    let cases = [
        (
            1e-6,
            geom::Curve3::Circle(geom::Circle {
                frame: frame(
                    [0.38180397980370573, 0.9260832315064935, 2.5664775268927267],
                    [
                        0.9482094717695619,
                        0.18958701473660672,
                        -0.25486380969009714,
                    ],
                    [-0.19606156387255988, 0.9805915883648227, 0.0],
                ),
                radius: 1.6277963338351105,
            }),
            cone_at(
                [0.3636073164749831, -0.3467551218854421, 1.5519461902326226],
                [
                    -0.8399785874790753,
                    -0.34446130208127795,
                    0.4192640980875038,
                ],
                1.2949231107682244,
                [0.37941936482656813, -0.9252248081383267, 0.0],
            ),
            (0.4146872962400363, 6.697872603419622),
        ),
        (
            1e-9,
            geom::Curve3::Ellipse(geom::Ellipse::new(
                frame(
                    [
                        -0.07865939183576044,
                        0.8711875840729832,
                        -1.7567297099952395,
                    ],
                    [-0.9386889114025205, 0.12566415864540245, 0.3210477329648881],
                    [-0.204223320486114, 0.5475812165719661, -0.8114478705543514],
                ),
                1.1507354785010555,
                0.7460761506568013,
            )),
            cone_at(
                [
                    0.11458741229190483,
                    0.13902426180565675,
                    -0.9051263741318243,
                ],
                [
                    -0.10100755757632167,
                    -0.6357891417669068,
                    -0.7652252220907033,
                ],
                0.21836420913669308,
                [-0.9914005851862641, 0.0, 0.13086206361024905],
            ),
            (-1.5209842899012085, 4.762201017278378),
        ),
        (
            1e-9,
            geom::Curve3::Circle(geom::Circle {
                frame: frame(
                    [-2.4691715662523097, 1.3671848009225616, -1.6066775348685667],
                    [
                        -0.026199321635919894,
                        -0.5277911491498032,
                        0.8489700221002788,
                    ],
                    [0.9995241661618604, 0.0, 0.030845441453121267],
                ),
                radius: 1.8490826456950697,
            }),
            cone_at(
                [-0.7237669286148725, 1.8600212669128329, -1.2464255426747348],
                [
                    -0.20719898913991616,
                    0.9781735347238777,
                    -0.01565614464648785,
                ],
                0.5264346410179193,
                [-0.9782934392041549, -0.2072243875853094, 0.0],
            ),
            (2.212092624691656, 8.495277931871243),
        ),
    ];
    for (k, (eps, e, s, (t0, t1))) in cases.into_iter().enumerate() {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        let label = format!("case {k}, ε {eps}");
        let got = conic_quadric_roots(&e, t0, t1, &s, band);
        if let Ok(Ok(CircleRoots::Certified(found))) = &got {
            assert_holds(
                &label,
                &e,
                &s,
                &found.roots()[..found.count()],
                (t0 + t1) / 2.0,
                eps,
            );
        }
        assert!(
            matches!(got, Ok(Err(Unsettled::AtApex))),
            "{label}: refused at the apex, got {got:?}"
        );
    }
}

/// **A small carrier through the apex refuses at the rung that cannot
/// read it** (`through_the_apex`'s three last rungs, each red without
/// it). Circles through a cone's apex in a meridian plane, a millimetre
/// or less across, at ε 1e-6: their other crossings lie within a few
/// hundred zero bands of the apex, where the brackets (sixty-four
/// escalation bands of arc each side) cannot all be read.
///
/// - **Clear of the apex** (`bool_conic_cone_deflated_clear`): on a
///   needle (half-angle 0.2), a 1 mm circle's bracket reaches the apex's
///   spread, and refuses [`Unsettled::AtApex`].
/// - **A definite side change** (`bool_conic_cone_deflated_side`): on a
///   wide cone (1.2), a 0.3 mm circle's bracket ends read no definite
///   side, and refuse [`Unsettled::Tangency`].
/// - **The slack meter** (`bool_conic_cone_root_slack`): on a 0.6 cone, a
///   1 mm circle's bracket holds the sign change, but the slope bound
///   over it cannot place the root to the band: [`Unsettled::Meter`].
#[test]
fn a_small_carrier_through_the_apex_refuses_at_the_rung_that_cannot_read_it() {
    let band = Band::new(1e-6, 1e-5).unwrap();
    for (label, half, rad, want) in [
        ("clear of the apex", 0.2, 1e-3, Unsettled::AtApex),
        ("a definite side change", 1.2, 3e-4, Unsettled::Tangency),
        ("the slack meter", 0.6, 1e-3, Unsettled::Meter),
    ] {
        let s = cone([1.0, -0.7, 0.3], [0.0, 0.0, 1.0], half);
        let u = Vec3::new(1.0, 0.0, 0.0);
        let e = geom::Curve3::circle(
            Point3::new(1.0, -0.7, 0.3) + u * rad,
            Vec3::new(0.0, 1.0, 0.0),
            rad,
            u * -1.0,
        );
        let got = conic_quadric_roots(&e, -PI, PI, &s, band);
        assert!(
            matches!(&got, Ok(Err(u)) if *u == want),
            "{label}: {want:?}, got {got:?}"
        );
    }
}

/// **The apex's spread bound refuses where either factor of `λ` is not
/// positive** ([`apex_spread_bound`]). With `|ε₀| = 50`, `ε₁ = 0` and
/// `|G(0)| = R = 1`, `G(0)` alone gives `10`, the window `Φ = 20`: past
/// `√12`, so `1 − Φ²/12 < 0`, and `|G(0)| − RΦ < 0` too. Their product is
/// positive, and read as `λ` it would give a finite "bound" that holds
/// nothing: the bound reads infinite, which refuses. A small spread
/// inside its window is bounded, and finite: `|ε₀| = 10⁻¹⁸`,
/// `|ε₁| = 10⁻⁹`, `|G(0)| = 1`, `R = 0.1`.
#[test]
fn the_apex_spread_bound_refuses_where_a_factor_is_not_positive() {
    let both_negative = apex_spread_bound((50.0, 0.0), 1.0, 1.0);
    assert!(
        both_negative.is_infinite(),
        "two negative factors bound nothing, got {both_negative}"
    );
    let small = apex_spread_bound((1e-18, 1e-9), 1.0, 0.1);
    assert!(
        small.is_finite() && small > 0.0 && small < 1e-8,
        "a small spread is bounded, got {small}"
    );
}
