//! **Adopted adversarial-review end-to-end suite** (M5 PR 2 review):
//! the hull bound used as a *certificate*, not as a number.
//!
//! The implementer's C2.2 rehearsal measures how tight the bound is on
//! an exact case. This suite asks the question PR 4 will actually ask:
//! **does the certificate accept a good fit and refuse a corrupted one,
//! at a stated ε?** It is deliberately arithmetic-free in its setup —
//! every control point is dyadic and lies *exactly* on the plane
//! `n · P = 1.5`, so the clean residual is zero in ℝ and the whole bound
//! is the ring's own conservatism.
//!
//! The residual composite is exact, not sampled. For a spline
//! `C(t) = Σ N_j P_j` (or its rational form with positive weights), the
//! partition of unity gives
//!
//! ```text
//! n·C(t) − 1.5 = Σ_j N_j (n·P_j − 1.5)
//! ```
//!
//! so the residual is a scalar spline over the SAME knot vector whose
//! coefficients are `n·P_j − 1.5`, built here in the ring. That is
//! exactly the "residual composite sampled into B-spline coefficient
//! form" the hull primitives were specified to consume — with no
//! sampling at all, because for a plane the composite is closed-form.
//!
//! The plant: one control point moved `2e-9` along `z`. With `n_z =
//! 0.75` the corrupted coefficient is `1.5e-9` **exactly**, so the
//! expected bound is known to the bit and the test can pin three things
//! at once — the bound refuses at `ε = 1e-9`, it equals `|n_z|·2e-9`,
//! and it is under `10×` the plant (a bound that refused by being
//! wildly loose would pass the first check and fail the third).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use geom::NurbsCurve3;
use geom_core::spline::{KnotVector, hull};
use geom_core::{Point3, RingInterval, Vec3};

/// The plane `n · P = 1.5`. Both are dyadic: `n = (0.5, 0.5, 0.75)`,
/// offset `1.5`, so `n·P` is exact for dyadic `P` with modest exponents.
const N: [f64; 3] = [0.5, 0.5, 0.75];
const OFFSET: f64 = 1.5;
/// The certification band this suite certifies against.
const EPS: f64 = 1e-9;
/// The planted displacement, along `z`.
const PLANT: f64 = 2e-9;

fn plane_normal() -> Vec3<f64> {
    Vec3::new(N[0], N[1], N[2])
}

/// A dyadic point exactly on the plane: pick `x`, `y` on the `1/16`
/// grid and solve for `z = (1.5 − 0.5x − 0.5y)/0.75`. Choosing `x + y`
/// on the `1/16` grid keeps `z` dyadic too — `(1.5 − h)/0.75` is
/// `2 − 4h/3` only when `h` is a multiple of `3/4`, so instead the point
/// is built the other way round: choose `x` and `z` dyadic and solve for
/// `y = 2·(1.5 − 0.5x − 0.75z)`, which is exact for every dyadic input.
fn on_plane(x: f64, z: f64) -> Point3<f64> {
    let y = 2.0 * (OFFSET - N[0] * x - N[2] * z);
    Point3::new(x, y, z)
}

/// A degree-3 clamped NURBS curve whose control points all lie exactly
/// on the plane, with dyadic positive weights.
fn planar_curve() -> NurbsCurve3<f64> {
    let control: Vec<Point3<f64>> = [
        (-1.0, 0.5),
        (-0.5, 0.25),
        (0.0, -0.75),
        (0.5, 1.25),
        (1.0, -0.5),
        (1.5, 0.75),
    ]
    .into_iter()
    .map(|(x, z)| on_plane(x, z))
    .collect();
    let weights = vec![1.0, 2.0, 0.5, 4.0, 0.25, 1.0];
    let knots =
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.25, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    NurbsCurve3::new(knots, control, weights).unwrap()
}

/// The residual composite's coefficients, built **in the ring**:
/// `r_j = n·P_j − 1.5`, fixed association (ascending coordinate, then
/// the offset subtracted last).
fn residual_coeffs(curve: &NurbsCurve3<f64>) -> Vec<RingInterval> {
    let n = plane_normal();
    curve
        .control()
        .iter()
        .map(|p| {
            let mut acc = RingInterval::zero();
            for (nd, pd) in [(n.x, p.x), (n.y, p.y), (n.z, p.z)] {
                acc = acc + RingInterval::point(nd) * RingInterval::point(pd);
            }
            acc - RingInterval::point(OFFSET)
        })
        .collect()
}

/// The sampling oracle: the true signed distance functional along the
/// curve, evaluated independently of every ring/hull code path.
fn sampled_residual(curve: &NurbsCurve3<f64>, t: f64) -> f64 {
    let p = curve.eval(t);
    N[0] * p.x + N[1] * p.y + N[2] * p.z - OFFSET
}

#[test]
fn control_points_are_exactly_on_the_plane() {
    // The fixture's premise, pinned: if this drifts, every number below
    // is measuring the fixture instead of the ring.
    let curve = planar_curve();
    for (i, p) in curve.control().iter().enumerate() {
        let r = N[0] * p.x + N[1] * p.y + N[2] * p.z - OFFSET;
        assert_eq!(r, 0.0, "control point {i} is not exactly on the plane");
    }
}

#[test]
fn the_clean_fit_is_certified_and_the_plant_is_refused() {
    let curve = planar_curve();
    let kv = curve.knots();
    let weights = curve.weights();

    // --- the clean curve certifies -----------------------------------
    let clean = residual_coeffs(&curve);
    let bound = hull::sup_norm_bound(kv, &clean);
    let bound_rat = hull::sup_norm_bound_rational(kv, &clean, weights);
    assert!(bound.is_finite() && bound_rat.is_finite(), "poisoned bound");
    assert!(
        bound <= EPS,
        "clean fit must certify at eps={EPS:e}, bound was {bound:e}"
    );
    assert_eq!(
        bound.to_bits(),
        bound_rat.to_bits(),
        "the rational entry point must return the same hull — the weights \
         license the claim, they do not change it"
    );
    // The clean bound is pure ring conservatism (the true residual is
    // identically zero), so it must sit at fp scale, not merely under eps.
    assert!(
        bound < 1e-14,
        "clean bound {bound:e} is far above fp scale — the ring is padding \
         more than one step per operation somewhere"
    );

    // --- the plant is refused ----------------------------------------
    let mut control = curve.control().to_vec();
    let victim = 3;
    control[victim] = Point3::new(
        control[victim].x,
        control[victim].y,
        control[victim].z + PLANT,
    );
    let corrupted =
        NurbsCurve3::new(kv.clone(), control, weights.to_vec()).expect("still a valid curve");
    let dirty = residual_coeffs(&corrupted);
    let dirty_bound = hull::sup_norm_bound(kv, &dirty);
    assert!(
        dirty_bound > EPS,
        "the {PLANT:e} plant must be refused at eps={EPS:e}, bound was {dirty_bound:e}"
    );
    // Known to the ring's noise floor: only the victim coefficient is
    // nonzero, and it is `n_z · PLANT`. The comparison tolerance is set
    // by the INTERMEDIATE scale, not the result's: the coefficient is
    // formed as `0.5x + 0.5y + 0.75z − 1.5`, four quantities of size
    // ~1.5 that cancel to ~1.5e-9, so each of the ring's one-step
    // outward pads is an ulp of 1.5 (≈2.2e-16), not an ulp of the
    // answer. That cancellation is real and the enclosure reports it
    // honestly — which is the whole reason certification bounds rather
    // than evaluates.
    let expected = N[2] * PLANT;
    let noise = 32.0 * f64::EPSILON * OFFSET;
    assert!(
        (dirty_bound - expected).abs() <= noise,
        "bound {dirty_bound:e} is not |n_z|·plant = {expected:e} within the \
         ring's cancellation noise floor {noise:e}"
    );
    // And it refuses by being RIGHT, not by being loose.
    assert!(
        dirty_bound < 10.0 * PLANT,
        "bound {dirty_bound:e} exceeds 10x the plant {PLANT:e} — a loose \
         refusal is not a certificate"
    );

    // --- the per-span form refuses too -------------------------------
    let mut refusing_spans = 0;
    let mut certifying_spans = 0;
    for index in kv.first_span()..=kv.last_span() {
        let Some(span) = kv.span(index) else { continue };
        assert!(
            hull::sup_norm_bound_span(kv, &clean, span) <= EPS,
            "clean span {index} must certify"
        );
        if hull::sup_norm_bound_span(kv, &dirty, span) > EPS {
            refusing_spans += 1;
        } else {
            certifying_spans += 1;
        }
    }
    assert!(
        refusing_spans > 0,
        "no span refused — the granular form must localise the corruption"
    );

    // --- the bound dominates dense sampling --------------------------
    let (mut max_clean, mut max_dirty) = (0.0f64, 0.0f64);
    for k in 0..=4096 {
        let t = f64::from(k) / 4096.0;
        max_clean = max_clean.max(sampled_residual(&curve, t).abs());
        max_dirty = max_dirty.max(sampled_residual(&corrupted, t).abs());
    }
    assert!(
        max_clean <= bound,
        "clean sample {max_clean:e} escapes the bound {bound:e}"
    );
    assert!(
        max_dirty <= dirty_bound,
        "corrupted sample {max_dirty:e} escapes the bound {dirty_bound:e}"
    );
    println!(
        "[review-scratch e2e] clean bound {bound:.3e} m (certifies at eps={EPS:e}; \
         densest sample {max_clean:.3e}); planted {PLANT:e} m gives bound \
         {dirty_bound:.3e} m = |n_z|*plant, REFUSED; {refusing_spans} span(s) \
         refuse, {certifying_spans} still certify; densest corrupted sample \
         {max_dirty:.3e} m"
    );
}

#[test]
fn a_sub_eps_perturbation_still_certifies() {
    // The complement of the refusal: the certificate is not a tripwire
    // that fires on any change. A plant an order of magnitude under the
    // band leaves the bound under the band.
    let curve = planar_curve();
    let kv = curve.knots();
    let mut control = curve.control().to_vec();
    control[1] = Point3::new(control[1].x, control[1].y, control[1].z + 1e-10);
    let nudged = NurbsCurve3::new(kv.clone(), control, curve.weights().to_vec()).unwrap();
    let bound = hull::sup_norm_bound(kv, &residual_coeffs(&nudged));
    assert!(
        bound <= EPS,
        "a 1e-10 nudge must still certify at eps={EPS:e}, bound was {bound:e}"
    );
    assert!(bound > 0.0, "the bound must still see the nudge");
}
