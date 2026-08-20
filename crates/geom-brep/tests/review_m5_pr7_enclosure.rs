//! **Promoted PR 7 review probe** (adversarial reviewer, adopted
//! verbatim as a committed row per the standing review policy).

//! REVIEW PROBE (PR 7): the exclusion enclosure cannot lie, and the
//! cylinder cancellation claim is real.
//!
//! Reimplements the cylinder/sphere implicit enclosures from the same
//! formulas `ssi::enclose` uses (RingInterval is public) and checks,
//! on dense samples over random and adversarial boxes, that the
//! enclosure always CONTAINS the pointwise linearized residual — the
//! soundness fact "excluded cell has no locus" reduces to.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    // Adopted verbatim from the reviewer's probe: the indexing and
    // cast shapes are theirs, and rewriting them would weaken the
    // "independent second implementation" the row exists to be.
    clippy::needless_range_loop,
    clippy::unnecessary_cast,
    clippy::unreachable
)]

use geom::Surface;
use geom_brep::implicit_residual;
use geom_core::{Point3, RingInterval, Vec3};
use test_utils::fuzz;

/// Uniform in `[-1, 1)`, the shape the reviewer's probe drew.
fn signed_unit(rng: &mut fuzz::Rng) -> f64 {
    rng.range(-1.0, 1.0)
}

fn ring(v: f64) -> RingInterval {
    RingInterval::point(v)
}

#[derive(Clone, Copy)]
struct B3 {
    x: RingInterval,
    y: RingInterval,
    z: RingInterval,
}

fn cyl_enclosure_good(origin: Point3<f64>, axis: Vec3<f64>, r: f64, b: B3) -> RingInterval {
    // The shipped form: w = q − â(q·â), then (|w|² − r²)/2r.
    let q = [
        b.x - ring(origin.x),
        b.y - ring(origin.y),
        b.z - ring(origin.z),
    ];
    let a = [ring(axis.x), ring(axis.y), ring(axis.z)];
    let h = q[0] * a[0] + q[1] * a[1] + q[2] * a[2];
    let w = [q[0] - a[0] * h, q[1] - a[1] * h, q[2] - a[2] * h];
    (w[0].sqr() + w[1].sqr() + w[2].sqr() - ring(r * r)) / ring(2.0 * r)
}

fn cyl_enclosure_naive(origin: Point3<f64>, axis: Vec3<f64>, r: f64, b: B3) -> RingInterval {
    // The algebraically-equal form the module docs warn about:
    // |q|² − (q·â)².
    let q = [
        b.x - ring(origin.x),
        b.y - ring(origin.y),
        b.z - ring(origin.z),
    ];
    let a = [ring(axis.x), ring(axis.y), ring(axis.z)];
    let h = q[0] * a[0] + q[1] * a[1] + q[2] * a[2];
    let q2 = q[0].sqr() + q[1].sqr() + q[2].sqr();
    (q2 - h.sqr() - ring(r * r)) / ring(2.0 * r)
}

/// Containment of the sample's own one-ulp rounding bracket. The
/// enclosure's contract (`RingInterval` module docs) binds the REAL
/// residual of points in the box; the f64 sample is itself a rounded
/// evaluation, so its final rounding may step across the enclosure's
/// endpoint (Lemma P1: a correctly rounded result lies within one
/// representable step of the true value). The honest oracle claim is
/// that the enclosure meets `[next_down(sample), next_up(sample)]` —
/// no relative slack: a genuinely lying enclosure misses by the
/// geometry's scale, orders of magnitude, never by one step.
fn brackets(e: RingInterval, sample: f64) -> bool {
    e.lo() <= sample.next_up() && sample.next_down() <= e.hi()
}

fn sph_enclosure(center: Point3<f64>, r: f64, b: B3) -> RingInterval {
    let q = [
        b.x - ring(center.x),
        b.y - ring(center.y),
        b.z - ring(center.z),
    ];
    (q[0].sqr() + q[1].sqr() + q[2].sqr() - ring(r * r)) / ring(2.0 * r)
}

#[test]
fn enclosures_contain_every_sampled_residual_hence_exclusion_cannot_lie() {
    let sph = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let cyl = Surface::Cylinder {
        origin: Point3::new(0.03, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 0.08,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let mut rng = fuzz::start("review_m5_pr7_enclosure::enclosures_contain_residuals");
    // Random boxes across the slab, plus boxes planted ON the locus
    // (centered at points of the two polar loops) — the adversarial
    // case: an excluded cell there would be a lie.
    let mut boxes: Vec<(Point3<f64>, f64)> = Vec::new();
    for _ in 0..fuzz::scaled(75) {
        let c = Point3::new(
            signed_unit(&mut rng) * 1.5,
            signed_unit(&mut rng) * 1.5,
            signed_unit(&mut rng) * 1.5,
        );
        let r = 10f64.powf(-3.0 * (0.5 * (signed_unit(&mut rng) + 1.0))) * 0.8; // 0.8 down to ~8e-4
        boxes.push((c, r));
    }
    // Locus points of x²+... : points on the cylinder at angle t, height
    // solved onto the sphere.
    let loop_pts = fuzz::scaled(25);
    for i in 0..loop_pts {
        let t = std::f64::consts::TAU * (i as f64) / (loop_pts as f64);
        let x = 0.03 + 0.08 * t.cos();
        let y = 0.08 * t.sin();
        let z2 = 1.0 - x * x - y * y;
        if z2 > 0.0 {
            let z = z2.sqrt();
            for sgn in [1.0, -1.0] {
                boxes.push((Point3::new(x, y, sgn * z), 1e-4));
                boxes.push((Point3::new(x, y, sgn * z), 1e-8));
            }
        }
    }
    for (c, r) in boxes {
        let b = B3 {
            x: RingInterval::from_bounds(c.x - r, c.x + r),
            y: RingInterval::from_bounds(c.y - r, c.y + r),
            z: RingInterval::from_bounds(c.z - r, c.z + r),
        };
        let (
            Surface::Sphere { center, radius, .. },
            Surface::Cylinder {
                origin,
                axis,
                radius: cr,
                ..
            },
        ) = (&sph, &cyl)
        else {
            unreachable!()
        };
        let es = sph_enclosure(*center, *radius, b);
        let ec = cyl_enclosure_good(*origin, *axis, *cr, b);
        assert!(
            !es.is_poison() && !ec.is_poison(),
            "enclosure poisoned on box centre {c:?} — {}",
            fuzz::replay()
        );
        // The 5×5×5 lattice is corners + face centres + interior of the
        // box, i.e. the structure of the containment claim itself rather
        // than a sweep count — dropping to 2 per axis would test only
        // corners, where an interval enclosure is least likely to lie.
        // It stays fixed; the BOX counts above carry the EFFORT dial.
        for i in 0..5 {
            for j in 0..5 {
                for k in 0..5 {
                    // Clamped into the box: the lattice formula rounds,
                    // and an unclamped corner can land a ulp OUTSIDE the
                    // box — a point the enclosure makes no claim about,
                    // whose residual the gradient pushes several ulps
                    // past the bound. The containment claim's hypothesis
                    // is "point in the box"; the clamp makes the sample
                    // satisfy it exactly.
                    let p = Point3::new(
                        (c.x - r + 2.0 * r * (i as f64) / 4.0).clamp(b.x.lo(), b.x.hi()),
                        (c.y - r + 2.0 * r * (j as f64) / 4.0).clamp(b.y.lo(), b.y.hi()),
                        (c.z - r + 2.0 * r * (k as f64) / 4.0).clamp(b.z.lo(), b.z.hi()),
                    );
                    let fs = implicit_residual(&sph, p);
                    let fc = implicit_residual(&cyl, p);
                    assert!(
                        brackets(es, fs),
                        "sphere enclosure lies at {p:?}: {fs} not in [{}, {}] — {}",
                        es.lo(),
                        es.hi(),
                        fuzz::replay()
                    );
                    assert!(
                        brackets(ec, fc),
                        "cylinder enclosure lies at {p:?}: {fc} not in [{}, {}] — {}",
                        ec.lo(),
                        ec.hi(),
                        fuzz::replay()
                    );
                }
            }
        }
    }
}

#[test]
fn the_cylinder_cancellation_claim_is_real_on_the_doc_fixture() {
    // The module docs claim the naive |q|² − (q·â)² form has width
    // ~0.8 m² where the true width is ~0 on a box straddling z ≈ 1.
    let origin = Point3::new(0.03, 0.0, 0.0);
    let axis = Vec3::new(0.0, 0.0, 1.0);
    let r = 0.08;
    let b = B3 {
        x: RingInterval::from_bounds(0.10, 0.12),
        y: RingInterval::from_bounds(-0.01, 0.01),
        z: RingInterval::from_bounds(0.95, 1.05),
    };
    let good = cyl_enclosure_good(origin, axis, r, b);
    let naive = cyl_enclosure_naive(origin, axis, r, b);
    let wg = good.hi() - good.lo();
    let wn = naive.hi() - naive.lo();
    println!("good width = {wg:e}, naive width = {wn:e}");
    assert!(
        wn > 10.0 * wg,
        "expected the naive form to be catastrophically wider: {wg} vs {wn}"
    );
    // And the good form can exclude while the naive form cannot: a box
    // clearly off the cylinder wall.
    let off = B3 {
        x: RingInterval::from_bounds(0.5, 0.52),
        y: RingInterval::from_bounds(0.5, 0.52),
        z: RingInterval::from_bounds(0.95, 1.05),
    };
    let g2 = cyl_enclosure_good(origin, axis, r, off);
    assert!(g2.lo() > 0.0, "the shipped form must exclude here");
}
