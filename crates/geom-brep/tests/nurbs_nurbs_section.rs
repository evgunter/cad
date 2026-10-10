//! **The NURBS × NURBS section** (`geom_brep::ssi::Section`): one complete
//! operation read at a seed (`branch_at`) or whole (`all`), certified by
//! the two-chart tube.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, NurbsSurface};
use geom_brep::ssi::BranchEnd;
use geom_brep::{BoundaryChart, Section, SsiDomain, SsiError, SsiTube};
use geom_core::Point3;
use geom_core::spline::KnotVector;

use crate::shared::tol::{band, eps};

/// A bilinear patch through four corners, `(u, v) ∈ [u0, u1] × [v0, v1]`.
fn bilinear(corners: [[f64; 3]; 4], u: (f64, f64), v: (f64, f64)) -> NurbsSurface<f64> {
    let k = |(a, b): (f64, f64)| KnotVector::clamped(vec![a, a, b, b], 1).unwrap();
    let [p00, p10, p01, p11] = corners.map(|c| Point3::new(c[0], c[1], c[2]));
    NurbsSurface::new(k(u), k(v), vec![p00, p01, p10, p11], vec![1.0; 4]).unwrap()
}

/// `z = c·x·y` over the unit square: `S₁(u, v) = (u, v, c·u·v)`.
const C: f64 = 0.4;
/// `x = ½ + e·y·z`: `S₂(u, v) = (½ + e·u·v, u, v)`.
const E: f64 = 0.3;

fn saddle() -> NurbsSurface<f64> {
    bilinear(
        [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, C],
        ],
        (0.0, 1.0),
        (0.0, 1.0),
    )
}

/// The second patch over `y ∈ [−0.2, 1.2]`, `z ∈ [−0.5, 1.5]`.
fn upright() -> NurbsSurface<f64> {
    let x = |y: f64, z: f64| 0.5 + E * y * z;
    let (y0, y1, z0, z1) = (-0.2, 1.2, -0.5, 1.5);
    bilinear(
        [
            [x(y0, z0), y0, z0],
            [x(y1, z0), y1, z0],
            [x(y0, z1), y0, z1],
            [x(y1, z1), y1, z1],
        ],
        (y0, y1),
        (z0, z1),
    )
}

/// The crease in closed form: on `y`, `x = ½ / (1 − c·e·y²)` and
/// `z = c·x·y`.
fn crease(y: f64) -> Point3<f64> {
    let x = 0.5 / (1.0 - C * E * y * y);
    Point3::new(x, y, C * x * y)
}

fn domain() -> SsiDomain {
    SsiDomain {
        center: Point3::new(0.5, 0.5, 0.5),
        half_extent: 2.0,
        extent: 1.0,
        floor_scale: 1.0,
    }
}

/// The largest distance of a carrier's dense samples from the closed
/// form at the same `y`.
fn off_crease(c: &Curve3<f64>, (t0, t1): (f64, f64)) -> f64 {
    (0..=400)
        .map(|i| {
            let p = c.eval(t0 + (t1 - t0) * f64::from(i) / 400.0);
            p.distance(crease(p.y))
        })
        .fold(0.0, f64::max)
}

/// **Two bilinear patches crossing at a crease: built and certified, and
/// the carrier is the closed form.** `all` finds one branch, from the
/// saddle's `v = 0` side to its `v = 1` side, certified by the two-chart
/// tube, within ε of the closed form everywhere; `branch_at` the crease's
/// middle returns it.
#[test]
fn two_bilinear_patches_cross_at_their_closed_form() {
    let (s1, s2) = (saddle(), upright());
    let mut section = Section::of(&s1, &s2, domain(), band()).expect("the section");
    let at = section
        .branch_at(crease(0.5))
        .expect("the branch through the crease's middle")
        .clone();
    assert!(
        matches!(at.certificate.tube, SsiTube::TwoChart { .. }),
        "limb 3 is the two-chart tube: {:?}",
        at.certificate.tube
    );
    let off = off_crease(&at.carrier, at.params);
    assert!(
        off <= 2.0 * eps(),
        "the carrier is {off:e} m off the closed form"
    );
    let BranchEnd::Crossings { from, to } = at.end else {
        panic!("an open branch between crossings, got {:?}", at.end);
    };
    for p in [from, to] {
        assert_eq!(
            p.chart,
            BoundaryChart::First,
            "it ends on the saddle's sides: {p:?}"
        );
    }
    let v1 = section
        .crossings()
        .into_iter()
        .find(|p| p.side.end == geom_brep::ChartEnd::High)
        .expect("the crossing on the saddle's v = 1 side");
    let want = crease(1.0).x;
    assert!(
        (v1.t - want).abs() <= eps(),
        "the v = 1 crossing is at u = {:e}, the closed form's {want:e}",
        v1.t
    );
    let whole = section.all().expect("every branch");
    assert_eq!(whole.branches.len(), 1, "one crease");
    assert_eq!(
        whole.branches[0].params, at.params,
        "`all` keeps the branch `branch_at` certified"
    );
    assert!(
        whole.exhaustiveness.accounted > 0,
        "{}",
        whole.exhaustiveness
    );
}

/// A biquadratic graph `z = g(x) + k(y)` over `xs × ys`, `(u, v) = (x,
/// y)`, `g` and `k` quadratics given by their coefficients.
fn graph(xs: (f64, f64), ys: (f64, f64), g: [f64; 3], k: [f64; 3]) -> NurbsSurface<f64> {
    let knots = |(a, b): (f64, f64)| KnotVector::clamped(vec![a, a, a, b, b, b], 2).unwrap();
    let q = |c: [f64; 3], x: f64| c[0] + c[1] * x + c[2] * x * x;
    let dq = |c: [f64; 3], x: f64| c[1] + 2.0 * c[2] * x;
    // A quadratic's Bernstein coefficients on [a, b].
    let bern =
        |c: [f64; 3], (a, b): (f64, f64)| [q(c, a), q(c, a) + 0.5 * (b - a) * dq(c, a), q(c, b)];
    let (gb, kb) = (bern(g, xs), bern(k, ys));
    let at = |(a, b): (f64, f64), i: usize| a + 0.5 * (b - a) * i as f64;
    let control = (0..9)
        .map(|n| {
            let (i, j) = (n / 3, n % 3);
            Point3::new(at(xs, i), at(ys, j), gb[i] + kb[j])
        })
        .collect();
    NurbsSurface::new(knots(xs), knots(ys), control, vec![1.0; 9]).unwrap()
}

/// `z = A·(x − ½)²` over the unit square.
const A: f64 = 1.0;

fn trough() -> NurbsSurface<f64> {
    graph((0.0, 1.0), (0.0, 1.0), [0.25 * A, -A, A], [0.0; 3])
}

/// The tilted plane `z = H + G·y` over `[−0.2, 1.2]²`, which cuts the
/// trough in two branches, `x = ½ ± √((H + G·y)/A)`.
const H: f64 = 0.04;
const G: f64 = 0.05;

fn tilted() -> NurbsSurface<f64> {
    graph((-0.2, 1.2), (-0.2, 1.2), [0.0; 3], [H, G, 0.0])
}

/// The trough's two branches at `y`, low `x` first.
fn branches_at(y: f64) -> [Point3<f64>; 2] {
    let r = ((H + G * y) / A).sqrt();
    let z = H + G * y;
    [Point3::new(0.5 - r, y, z), Point3::new(0.5 + r, y, z)]
}

/// **`all` finds every branch of a pair with two.** The trough and the
/// tilted plane cross in two branches, each from the trough's `v = 0`
/// side to its `v = 1` side: both are found, certified, and within ε of
/// their closed forms, and the accounting closes over the product of the
/// two rectangles.
#[test]
fn all_finds_both_branches_of_a_trough_cut_twice() {
    let (s1, s2) = (trough(), tilted());
    let whole = Section::of(&s1, &s2, domain(), band())
        .expect("the section")
        .all()
        .expect("every branch");
    assert_eq!(whole.branches.len(), 2, "two branches");
    for (k, b) in whole.branches.iter().enumerate() {
        let off = (0..=200)
            .map(|i| {
                let p = b
                    .carrier
                    .eval(b.params.0 + (b.params.1 - b.params.0) * f64::from(i) / 200.0);
                let [lo, hi] = branches_at(p.y);
                p.distance(lo).min(p.distance(hi))
            })
            .fold(0.0, f64::max);
        assert!(
            off <= 2.0 * eps(),
            "branch {k} is {off:e} m off its closed form"
        );
    }
    let sides: Vec<f64> = whole
        .branches
        .iter()
        .map(|b| b.carrier.eval(b.params.0).x)
        .collect();
    assert!(
        (sides[0] - 0.5) * (sides[1] - 0.5) < 0.0,
        "one branch on each side of the trough's floor: {sides:?}"
    );
    assert!(
        whole.exhaustiveness.accounted > 0,
        "{}",
        whole.exhaustiveness
    );
}

/// **`branch_at` returns the branch whose certified tube holds the
/// seed.** A seed on each branch returns that branch, the one `all`
/// certifies there; a seed on the plane between them settles
/// onto one of the two, and the answer is that one.
#[test]
fn branch_at_answers_the_branch_its_seed_settles_on() {
    let (s1, s2) = (trough(), tilted());
    let whole = Section::of(&s1, &s2, domain(), band())
        .expect("the section")
        .all()
        .expect("every branch");
    let mut section = Section::of(&s1, &s2, domain(), band()).expect("the section");
    for (k, seed) in branches_at(0.4).into_iter().enumerate() {
        let b = section
            .branch_at(seed)
            .expect("the branch through the seed");
        let p = b.carrier.eval(0.5 * (b.params.0 + b.params.1));
        let [lo, hi] = branches_at(p.y);
        let near = if k == 0 { lo } else { hi };
        assert!(
            p.distance(near) <= 2.0 * eps(),
            "seed {k} answered the other branch: its middle is at {p:?}"
        );
    }
    let between = Point3::new(0.6, 0.5, H + G * 0.5);
    let b = section.branch_at(between).expect("a branch").clone();
    let one = whole.branches.iter().any(|w| {
        let p = w.carrier.eval(0.5 * (w.params.0 + w.params.1));
        let q = b.carrier.eval(0.5 * (b.params.0 + b.params.1));
        (p.x - 0.5).signum() == (q.x - 0.5).signum()
    });
    assert!(one, "the seed between them answered no branch `all` found");
    assert!(
        matches!(b.certificate.tube, SsiTube::TwoChart { .. }),
        "the answer is certified by the two-chart tube"
    );
}

/// **A near-tangent pair refuses at the seed.** `z = (x − ½)²` and
/// `z = h + (y − ½)²` with `h` far inside the band cross in two
/// branches that all but touch where both surfaces are level, at the
/// patches' centre: the section is built, its crossings far from there
/// transversal, and a seed at the centre settles where the two normals
/// agree within the band, which the section's own transversality
/// decision refuses.
#[test]
fn a_near_tangent_pair_refuses_at_the_seed() {
    let h = 1e-6 * eps() * eps();
    let s1 = graph((0.0, 1.0), (0.0, 1.0), [0.25, -1.0, 1.0], [0.0; 3]);
    let s2 = graph((0.1, 0.9), (-0.2, 1.2), [0.0; 3], [h + 0.25, -1.0, 1.0]);
    let mut section = Section::of(&s1, &s2, domain(), band()).expect("the section");
    let e = section
        .branch_at(Point3::new(0.5, 0.5, 0.0))
        .expect_err("a seed where the two surfaces are tangent");
    assert!(
        matches!(
            e,
            SsiError::TransversalityBand { .. }
                | SsiError::Escalated {
                    decision: geom_brep::TraceDecision::Transversality
                        | geom_brep::TraceDecision::TransversalityArm,
                    ..
                }
        ),
        "the seed's transversality refuses: {e:?}"
    );
}
