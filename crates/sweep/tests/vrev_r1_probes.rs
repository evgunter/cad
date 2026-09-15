//! **VREV R1 review probes** (PR 2627, head `53424215b`) — a reviewer's
//! rows against `NurbsSurface::reversed_v` / `reversed_u`, on the probes
//! branch only. Not for the permanent suite.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp,
    clippy::cast_precision_loss
)]

use geom::{KnotMirrorError, NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use std::sync::Arc;
use profile::RawLoop;
use topo::FaceSurface;

/// A private copy of the door's 2Sum, to talk about its values.
fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let a_head = s - b;
    let b_head = s - a_head;
    (s, (a - a_head) + (b - b_head))
}

/// A `3 × nv` net with no symmetry of its own on `knots_u = {0,0,½,1,1}`.
fn surface_on(knots_v: Vec<f64>, pv: usize) -> NurbsSurface<f64> {
    let kv = KnotVector::clamped(knots_v, pv).unwrap();
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
    let nv = kv.control_count();
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for iu in 0..3usize {
        for iv in 0..nv {
            control.push(Point3::new(
                iu as f64 + 0.37 * iv as f64,
                (iv * iv) as f64 * 0.11 - iu as f64,
                ((iu * 7 + iv * 3) % 5) as f64 * 0.5,
            ));
            weights.push(1.0 + ((iu * 3 + iv * 5) % 7) as f64 * 0.25);
        }
    }
    NurbsSurface::new(ku, kv, control, weights).unwrap()
}

fn ulps(a: f64, b: f64) -> i64 {
    ((a.to_bits() as i64) - (b.to_bits() as i64)).abs()
}

/// **P1.** Decimal-symmetric interior pairs — what a user types — under
/// the EXACT test. Every pair here has `fl(a + b) == 1.0`, so the spec's
/// rounded test admits all of them; the shipped exact test refuses the
/// ones whose real sum is not 1, which is most of them, including the
/// uniform thirds.
#[test]
fn p1_decimal_symmetric_pairs_under_the_exact_test() {
    let pairs: [(&str, f64, f64); 7] = [
        ("thirds", 1.0 / 3.0, 2.0 / 3.0),
        ("0.1/0.9", 0.1, 0.9),
        ("0.2/0.8", 0.2, 0.8),
        ("0.3/0.7", 0.3, 0.7),
        ("0.4/0.6", 0.4, 0.6),
        ("0.45/0.55", 0.45, 0.55),
        ("0.125/0.875", 0.125, 0.875),
    ];
    let mut refused = Vec::new();
    for (name, a, b) in pairs {
        assert_eq!(a + b, 1.0, "{name}: the ROUNDED sum is exactly lo + hi");
        let s = surface_on(vec![0.0, 0.0, 0.0, a, b, 1.0, 1.0, 1.0], 2);
        let exact = two_sum(a, b);
        let out = s.reversed_v();
        println!(
            "P1 {name:12} a={a:.20} b={b:.20} 2Sum=({}, {:e}) -> {}",
            exact.0,
            exact.1,
            match &out {
                Ok(_) => "accepted".to_string(),
                Err(e) => format!("REFUSED: {e}"),
            }
        );
        assert_eq!(
            out.is_err(),
            exact != (1.0, 0.0),
            "{name}: the door decides exactly the 2Sum identity"
        );
        if out.is_err() {
            refused.push(name);
        }
    }
    println!("P1 refused {} of {}: {refused:?}", refused.len(), pairs.len());
}

/// **P2.** Signed zero and subnormal knots: the comparison is IEEE
/// equality, not bit equality, so `-0.0` matches `0.0`; the subnormal
/// range is exact under 2Sum.
#[test]
fn p2_signed_zero_and_subnormal_knots() {
    // −0.0 as the middle knot of a symmetric [−1, 1] vector.
    let s = surface_on(vec![-1.0, -1.0, -0.0, 1.0, 1.0], 1);
    let r = s.reversed_v();
    println!("P2 signed zero: {:?}", r.as_ref().err());
    assert!(r.is_ok(), "−0.0 is real zero: the vector IS symmetric");
    assert_eq!(
        r.unwrap().knots_v().knots()[2].to_bits(),
        (-0.0f64).to_bits(),
        "the −0.0 is carried verbatim, sign bit and all"
    );
    // A whole domain in the subnormals.
    let tiny = f64::from_bits(1);
    let hi = tiny * 8.0;
    let sym = surface_on(vec![0.0, 0.0, 0.0, tiny, hi - tiny, hi, hi, hi], 2);
    assert!(sym.reversed_v().is_ok(), "subnormal symmetric accepted");
    let asym = surface_on(vec![0.0, 0.0, 0.0, tiny, hi - 2.0 * tiny, hi, hi, hi], 2);
    let e = asym.reversed_v().unwrap_err();
    println!("P2 subnormal asymmetric: {e}");
    assert!(matches!(e, KnotMirrorError::AsymmetricPair { index: 3, .. }));
}

/// **P3.** The `ReflectionNotFinite` rationale ("every pair would match
/// vacuously") describes a ROUNDED compare. Under the shipped 2Sum
/// compare an overflowing head carries a NaN residual, and NaN never
/// compares equal — so without the guard the loop refuses at index 0
/// with an `AsymmetricPair` naming the clamp pair. The guard is still
/// right (a clear reason instead of a misleading one); its stated
/// reason is not what the code would do.
#[test]
fn p3_overflow_without_the_guard_would_refuse_not_pass() {
    let r = two_sum(1e308, 1.5e308);
    println!("P3 two_sum(1e308, 1.5e308) = ({}, {})", r.0, r.1);
    assert!(r.0.is_infinite() && r.1.is_nan());
    assert!(r != r, "(inf, NaN) != (inf, NaN): the comparison REFUSES, never passes");
}

/// **P4.** The ulp band on a dense dyadic grid over the unit's own
/// fixture shape: is 4 enough headroom, and where is the worst?
#[test]
fn p4_ulp_band_on_a_dense_grid() {
    let s = surface_on(vec![0.0, 0.0, 0.0, 0.25, 0.75, 1.0, 1.0, 1.0], 2);
    let r = s.reversed_v().unwrap();
    let mut worst = (0i64, 0.0, 0.0);
    let n = 256;
    for i in 0..=n {
        for j in 0..=n {
            let (u, v) = (i as f64 / n as f64, j as f64 / n as f64);
            let got = r.eval(u, v);
            let want = s.eval(u, 1.0 - v);
            let gap = ulps(got.x, want.x)
                .max(ulps(got.y, want.y))
                .max(ulps(got.z, want.z));
            if gap > worst.0 {
                worst = (gap, u, v);
            }
        }
    }
    println!("P4 worst over a {n}×{n} dyadic grid: {} ulps at (u, v) = ({}, {})", worst.0, worst.1, worst.2);
    // A degree-3 v vector, symmetric, with more interior knots.
    let s3 = surface_on(
        vec![0.0, 0.0, 0.0, 0.0, 0.125, 0.5, 0.875, 1.0, 1.0, 1.0, 1.0],
        3,
    );
    let r3 = s3.reversed_v().unwrap();
    let mut worst3 = (0i64, 0.0, 0.0);
    for i in 0..=n {
        for j in 0..=n {
            let (u, v) = (i as f64 / n as f64, j as f64 / n as f64);
            let got = r3.eval(u, v);
            let want = s3.eval(u, 1.0 - v);
            let gap = ulps(got.x, want.x)
                .max(ulps(got.y, want.y))
                .max(ulps(got.z, want.z));
            if gap > worst3.0 {
                worst3 = (gap, u, v);
            }
        }
    }
    println!("P4 degree-3 worst: {} ulps at ({}, {})", worst3.0, worst3.1, worst3.2);
}

/// **P5.** Involution and conjugation on a degree-3 vector with a
/// middle knot (odd interior count), and `reversed_u` on an
/// asymmetric `knots_u` names `knots_u`'s indices.
#[test]
fn p5_odd_interior_and_reversed_u_indices() {
    let s = surface_on(vec![0.0, 0.0, 0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0, 1.0, 1.0], 3);
    let back = s.reversed_v().unwrap().reversed_v().unwrap();
    assert_eq!(back.weights(), s.weights());
    assert_eq!(back.knots_v().knots(), s.knots_v().knots());
    // reversed_u on an asymmetric u vector: knots_u = {0,0,0,¼,1,1,1}.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.25, 1.0, 1.0, 1.0], 2).unwrap();
    let control: Vec<_> = (0..8)
        .map(|i| Point3::new(i as f64, (i * i) as f64, 0.0))
        .collect();
    let s = NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap();
    let e = s.reversed_u().unwrap_err();
    println!("P5 reversed_u refusal: {e}");
    assert!(matches!(e, KnotMirrorError::AsymmetricPair { index: 3, mirror_index: 3, .. }));
}

// ---------------------------------------------------------------------
// End to end: a user reverses a lofted wall and puts it back.
// ---------------------------------------------------------------------

fn square() -> sweep::Section {
    let v = |x: f64, y: f64| profile::ProfileVertex::new(Point2::new(x, y), 0.0);
    vec![profile::ProfileLoop::new(vec![
        v(-1.0, -1.0),
        v(1.0, -1.0),
        v(1.0, 1.0),
        v(-1.0, 1.0),
    ])]
}

/// A `k`-section loft of equally spaced squares — the shape a user
/// builds first. Returns which of its walls the door reverses and which
/// it refuses, and validates the body after a reversal is put back.
fn reverse_a_lofted_wall(k: usize, spacing: f64) -> (Vec<f64>, Result<(), String>) {
    let sections: Vec<_> = (0..k).map(|_| square()).collect();
    let places: Vec<_> = (0..k)
        .map(|j| Affine3::translation(Vec3::new(0.0, 0.0, j as f64 * spacing)))
        .collect();
    let lofted = sweep::loft_body::<f64>(&sections, &places, 2, Tol::witness())
        .expect("the loft builds");
    let mut body = lofted.body;
    let fk = lofted.side_faces[0][0];
    let sk = body.get_face(fk).unwrap().surface;
    let n = match body.get_surface(sk) {
        Some(Surface::Nurbs(n)) => (**n).clone(),
        other => panic!("a wall is a NURBS chart: {other:?}"),
    };
    let knots_v = n.knots_v().knots().to_vec();
    let out = match n.reversed_v() {
        Err(e) => {
            // Would the spec's ROUNDED test have admitted this vector?
            let k = n.knots_v().knots();
            let (lo, hi) = n.knots_v().domain();
            let m = k.len() - 1;
            let rounded_ok = (0..=m / 2).all(|i| k[i] + k[m - i] == lo + hi);
            Err(format!("{e} [rounded test would accept: {rounded_ok}]"))
        }
        Ok(r) => {
            // Same point set, sampled.
            let mut worst = 0.0f64;
            for i in 0..=16 {
                for j in 0..=16 {
                    let (u, v) = (i as f64 / 16.0, j as f64 / 16.0);
                    let a = r.eval(u, v);
                    let b = n.eval(u, 1.0 - v);
                    worst = worst.max((a.x - b.x).abs().max((a.y - b.y).abs()).max((a.z - b.z).abs()));
                }
            }
            body.set_face_surface(fk, FaceSurface::New(Surface::Nurbs(Arc::new(r))))
                .expect("the swap is a key swap");
            let tier1 = topo::validate(&body);
            Ok(()).and_then(|()| {
                if worst > 1e-12 {
                    return Err(format!("point set moved by {worst:e}"));
                }
                tier1.map_err(|e| format!("tier 1: {e:?}"))
            })
        }
    };
    (knots_v, out)
}

/// **E2E.** Reverse one wall of a `k`-section loft, `k = 3..=8`, and put
/// it back. Reports the v knots the skin chose and the door's verdict.
#[test]
fn e2e_reverse_a_lofted_wall_for_each_section_count() {
    let mut verdicts = Vec::new();
    for k in 3..=8 {
        let (knots, out) = reverse_a_lofted_wall(k, 1.0);
        println!("E2E k={k}: knots_v = {knots:?}");
        println!("E2E k={k}: {}", match &out {
            Ok(()) => "reversed, reattached, tier 1 valid".to_string(),
            Err(e) => format!("REFUSED/FAILED: {e}"),
        });
        verdicts.push((k, out.is_ok()));
    }
    println!("E2E verdicts: {verdicts:?}");
    assert!(verdicts[0].1, "the unit's own fixture shape (k = 3) reverses");
    // Non-unit spacing: chord-length params are then not dyadic.
    for k in [4usize, 5] {
        let (knots, out) = reverse_a_lofted_wall(k, 0.7);
        println!("E2E spacing 0.7 k={k}: knots_v = {knots:?} -> {:?}", out);
    }
}

/// The unit's own fixture, verbatim (`reversal_tests::symmetric`).
fn unit_fixture() -> NurbsSurface<f64> {
    const NET: [(f64, f64, f64); 15] = [
        (0.0, 0.0, 0.0),
        (1.0, 2.0, -1.0),
        (2.0, -1.0, 3.0),
        (3.0, 1.0, -2.0),
        (4.0, 0.5, 1.0),
        (0.5, 3.0, 1.0),
        (1.5, -2.0, 2.0),
        (2.5, 2.0, 0.0),
        (3.5, 0.0, 3.0),
        (4.5, 1.0, -1.0),
        (1.0, -1.5, 2.5),
        (2.0, 0.0, -3.0),
        (3.0, 2.5, 1.5),
        (4.0, -1.0, 0.0),
        (5.0, 1.5, 2.0),
    ];
    const WEIGHTS: [f64; 15] = [
        1.0, 2.0, 0.5, 4.0, 1.5, 0.25, 3.0, 1.0, 2.5, 0.75, 1.25, 0.5, 2.0, 1.0, 3.5,
    ];
    NurbsSurface::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.25, 0.75, 1.0, 1.0, 1.0], 2).unwrap(),
        NET.iter().map(|(x, y, z)| Point3::new(*x, *y, *z)).collect(),
        WEIGHTS.to_vec(),
    )
    .unwrap()
}

/// **P6.** The unit's pin is "≤ 4 ulps, worst 2" over 35 points. On the
/// SAME fixture over a dense dyadic grid: the worst ulps, the worst
/// absolute gap, the worst ulps where the coordinate is not near zero —
/// and whether the v-basis rows are mirror images bit for bit (if they
/// are, summation order is the whole story; if not, it is not).
#[test]
fn p6_ulp_band_on_the_units_fixture_and_the_basis_mirror() {
    let s = unit_fixture();
    let r = s.reversed_v().unwrap();
    let n = 128;
    let (mut worst_ulps, mut worst_abs, mut worst_floored) = ((0i64, 0.0, 0.0, 0.0), (0.0f64, 0.0, 0.0), (0i64, 0.0, 0.0));
    for i in 0..=n {
        for j in 0..=n {
            let (u, v) = (i as f64 / n as f64, j as f64 / n as f64);
            let got = r.eval(u, v);
            let want = s.eval(u, 1.0 - v);
            for (g, w) in [(got.x, want.x), (got.y, want.y), (got.z, want.z)] {
                let ul = ulps(g, w);
                if ul > worst_ulps.0 {
                    worst_ulps = (ul, u, v, w);
                }
                let ab = (g - w).abs();
                if ab > worst_abs.0 {
                    worst_abs = (ab, u, v);
                }
                if w.abs() >= 0.5 && ul > worst_floored.0 {
                    worst_floored = (ul, u, v);
                }
            }
        }
    }
    println!(
        "P6 unit fixture, {n}×{n} dyadic grid: worst {} ulps at ({}, {}) where the coordinate is {:e}; worst |gap| {:e} at ({}, {}); worst ulps with |coord| ≥ 0.5: {} at ({}, {})",
        worst_ulps.0, worst_ulps.1, worst_ulps.2, worst_ulps.3, worst_abs.0, worst_abs.1, worst_abs.2, worst_floored.0, worst_floored.1, worst_floored.2
    );
    // Are the v-basis rows mirror images, bit for bit?
    let kv = s.knots_v();
    let (mut mismatched, mut checked) = (0usize, 0usize);
    let mut first = None;
    for j in 0..=n {
        let v = j as f64 / n as f64;
        let a = geom_core::spline::basis::basis_funs(kv.span_at(v), v);
        let b = geom_core::spline::basis::basis_funs(kv.span_at(1.0 - v), 1.0 - v);
        let mirrored: Vec<f64> = b.iter().rev().copied().collect();
        checked += 1;
        if a.iter().zip(&mirrored).any(|(x, y)| x.to_bits() != y.to_bits()) {
            mismatched += 1;
            if first.is_none() {
                first = Some((v, a.clone(), mirrored.clone()));
            }
        }
    }
    println!("P6 basis mirror: {mismatched} of {checked} v samples have a row that is NOT the bitwise mirror; first: {first:?}");
}
