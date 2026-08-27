//! ADVERSARIAL review battery for M5 PR 3, surfaces
//! (reviewer-authored, scratch): F3 jet vs central finite differences,
//! F5 u- AND v-direction knot-algebra invariance fuzz, F4 grid
//! removal-bound honesty in both directions. Tolerances as in the curve
//! attack file (stated there).
//!
//! The sweeps draw from `test_utils::fuzz` — a fresh seed per run, logged
//! unconditionally, with every count a multiple of `CAD_FUZZ_EFFORT`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(missing_docs)]

use geom_core::spline::KnotVector;
use test_utils::fuzz;
// Promotion adaptation (mechanical): dropped an unused Real import.
use geom::surfaces::nurbs::NurbsSurface;
use geom_core::{Point3, Vec3};

fn random_kv(rng: &mut fuzz::Rng, p: usize, interior: usize) -> KnotVector {
    let mut ks: Vec<f64> = vec![0.0; p + 1];
    let mut mids: Vec<f64> = (0..interior).map(|_| rng.range(0.1, 0.9)).collect();
    mids.sort_by(f64::total_cmp);
    mids.dedup();
    ks.extend_from_slice(&mids);
    ks.extend(std::iter::repeat_n(1.0, p + 1));
    KnotVector::clamped(ks, p).unwrap()
}

fn random_surface(
    rng: &mut fuzz::Rng,
    pu: usize,
    pv: usize,
    iu: usize,
    iv: usize,
) -> NurbsSurface<f64> {
    let ku = random_kv(rng, pu, iu);
    let kvv = random_kv(rng, pv, iv);
    let (nu, nv) = (ku.control_count(), kvv.control_count());
    let control = (0..nu * nv)
        .map(|_| {
            Point3::new(
                rng.range(-5.0, 5.0),
                rng.range(-5.0, 5.0),
                rng.range(-5.0, 5.0),
            )
        })
        .collect();
    let weights = (0..nu * nv).map(|_| rng.range(0.3, 3.0)).collect();
    NurbsSurface::new(ku, kvv, control, weights).unwrap()
}

fn supp(a: Point3<f64>, b: Point3<f64>) -> f64 {
    (a.x - b.x)
        .abs()
        .max((a.y - b.y).abs())
        .max((a.z - b.z).abs())
}

fn supv(a: Vec3<f64>, b: Vec3<f64>) -> f64 {
    (a.x - b.x)
        .abs()
        .max((a.y - b.y).abs())
        .max((a.z - b.z).abs())
}

/// F3 (surface): all six jet components vs central differences of eval
/// at random interior parameters away from knots.
#[test]
fn f3_surface_jet_matches_central_differences() {
    let mut rng = fuzz::start("review_m5_pr3_attack_surfaces::f3_jet_vs_fd");
    for case in 0..fuzz::scaled(2) {
        let pu = 1 + rng.below(4);
        let pv = 1 + rng.below(4);
        let s = random_surface(&mut rng, pu, pv, 2, 2);
        let h = 1e-5;
        let near_knot =
            |t: f64, kv: &KnotVector| kv.knots().iter().any(|k| (k - t).abs() < 10.0 * h);
        for _ in 0..fuzz::scaled(1) {
            let u = rng.range(0.05, 0.95);
            let v = rng.range(0.05, 0.95);
            if near_knot(u, s.knots_u()) || near_knot(v, s.knots_v()) {
                continue;
            }
            let jet = s.ders(u, v);
            assert!(
                supp(jet.point, s.eval(u, v)) < 1e-12,
                "jet point != eval — {}",
                fuzz::replay()
            );
            let fdu = (s.eval(u + h, v) - s.eval(u - h, v)) / (2.0 * h);
            let fdv = (s.eval(u, v + h) - s.eval(u, v - h)) / (2.0 * h);
            assert!(
                supv(jet.du, fdu) < 1e-4 * (1.0 + jet.du.norm()),
                "case {case}: Su vs FD at ({u},{v}) — {}",
                fuzz::replay()
            );
            assert!(
                supv(jet.dv, fdv) < 1e-4 * (1.0 + jet.dv.norm()),
                "case {case}: Sv vs FD at ({u},{v}) — {}",
                fuzz::replay()
            );
            let fduu = (s.ders(u + h, v).du - s.ders(u - h, v).du) / (2.0 * h);
            let fdvv = (s.ders(u, v + h).dv - s.ders(u, v - h).dv) / (2.0 * h);
            let fduv = (s.ders(u, v + h).du - s.ders(u, v - h).du) / (2.0 * h);
            assert!(
                supv(jet.duu, fduu) < 1e-3 * (1.0 + jet.duu.norm()),
                "case {case}: Suu at ({u},{v}): {:?} vs {:?} — {}",
                jet.duu,
                fduu,
                fuzz::replay()
            );
            assert!(
                supv(jet.dvv, fdvv) < 1e-3 * (1.0 + jet.dvv.norm()),
                "case {case}: Svv at ({u},{v}): {:?} vs {:?} — {}",
                jet.dvv,
                fdvv,
                fuzz::replay()
            );
            assert!(
                supv(jet.duv, fduv) < 1e-3 * (1.0 + jet.duv.norm()),
                "case {case}: Suv at ({u},{v}): {:?} vs {:?} — {}",
                jet.duv,
                fduv,
                fuzz::replay()
            );
        }
    }
}

/// F5 (surface): u AND v insertion/refinement/elevation are
/// evaluation-invariant on a dense grid.
#[test]
fn f5_surface_knot_algebra_invariant_both_directions() {
    let mut rng = fuzz::start("review_m5_pr3_attack_surfaces::f5_knot_algebra");
    let tol = 1e-9;
    for case in 0..fuzz::scaled(1) {
        let pu = 1 + rng.below(3);
        let pv = 1 + rng.below(3);
        let s = random_surface(&mut rng, pu, pv, 1, 1);
        let uu = rng.range(0.15, 0.85);
        let vv = rng.range(0.15, 0.85);
        let variants: Vec<(String, NurbsSurface<f64>)> = vec![
            ("insert_u".into(), s.insert_knot_u(uu, 1).unwrap()),
            ("insert_v".into(), s.insert_knot_v(vv, 1).unwrap()),
            ("refine_u".into(), s.refine_knots_u(&[0.2, 0.8]).unwrap()),
            (
                "refine_v".into(),
                if pv >= 2 {
                    s.refine_knots_v(&[0.3, 0.3]).unwrap()
                } else {
                    s.refine_knots_v(&[0.3]).unwrap()
                },
            ),
            ("elevate_u".into(), s.elevate_degree_u(1).unwrap()),
            ("elevate_v".into(), s.elevate_degree_v(1).unwrap()),
        ];
        // degree assertions for elevation
        assert_eq!(variants[4].1.knots_u().degree(), pu + 1);
        assert_eq!(variants[5].1.knots_v().degree(), pv + 1);
        for (name, w) in &variants {
            let grid = fuzz::scaled(3);
            for i in 0..=grid {
                for j in 0..=grid {
                    let (u, v) = (i as f64 / grid as f64, j as f64 / grid as f64);
                    let d = supp(s.eval(u, v), w.eval(u, v));
                    assert!(
                        d < tol,
                        "case {case} {name}: invariance broke at ({u},{v}): {d:e} — {}",
                        fuzz::replay()
                    );
                }
            }
        }
    }
}

/// F4 (surface): grid removal bound honesty in u and (via transpose
/// conjugation) v, on reviewer-planted lossy grids.
#[test]
fn f4_surface_removal_bound_honest_u_and_v() {
    let mut rng = fuzz::start("review_m5_pr3_attack_surfaces::f4_removal_bound");
    // COVERAGE FLOOR: the row demands a QUOTA of three bounded removals
    // per direction, so the loop count here is a CAP on the attempts, not
    // a sweep size — cutting it by the usual /8 would make the quota
    // unreachable. The loop breaks as soon as the quota is met, so the
    // typical cost is three passes, not the cap.
    const QUOTA: usize = 3;
    for dir_v in [false, true] {
        let mut successes = 0;
        for _ in 0..fuzz::scaled(12) {
            if successes >= QUOTA {
                break;
            }
            // Reviewer-planted lossy case: insert a knot exactly (bound
            // ~0), then perturb the whole grid so removal is genuinely
            // lossy but stays weight-consistent (near-removable data).
            let base = random_surface(&mut rng, 3, 2, 1, 1);
            let u = 0.437;
            let ins = if dir_v {
                base.insert_knot_v(u, 1).unwrap()
            } else {
                base.insert_knot_u(u, 1).unwrap()
            };
            let mut ctrl = ins.control().to_vec();
            for (i, p) in ctrl.iter_mut().enumerate() {
                let d = 0.01 * ((i % 7) as f64 - 3.0) / 3.0;
                *p = *p + Vec3::new(d, -d, 0.5 * d);
            }
            let s = NurbsSurface::new(
                ins.knots_u().clone(),
                ins.knots_v().clone(),
                ctrl,
                ins.weights().to_vec(),
            )
            .unwrap();
            let res = if dir_v {
                s.remove_knot_v(u, 1)
            } else {
                s.remove_knot_u(u, 1)
            };
            let (hat, bound) = match res {
                Ok(x) => x,
                // WeightCollapse is the documented honest refusal for
                // non-removable random data; skip it but demand a quota
                // of successful bounded removals (checked below).
                Err(geom_core::spline::KnotAlgebraError::WeightCollapse { .. }) => continue,
                Err(e) => panic!("unexpected refusal: {e:?}"),
            };
            successes += 1;
            let mut worst = 0.0f64;
            let grid = fuzz::scaled(8);
            for i in 0..=grid {
                for j in 0..=grid {
                    let (uu, vv) = (i as f64 / grid as f64, j as f64 / grid as f64);
                    let e = supp(s.eval(uu, vv), hat.eval(uu, vv));
                    if e > worst {
                        worst = e;
                    }
                    assert!(
                        e <= bound + 1e-12,
                        "surface bound violated (dir_v={dir_v}) at ({uu},{vv}): \
                         {e:e} > {bound:e} — {}",
                        fuzz::replay()
                    );
                }
            }
            assert!(bound.is_finite(), "poisoned bound — {}", fuzz::replay());
            // Diagnostic honesty: bound should not be absurdly loose
            // when the removal is genuinely lossy (ratio sanity only,
            // not a hard contract): record via assert with huge slack.
            if worst > 1e-9 {
                assert!(
                    bound / worst < 1e6,
                    "bound uselessly loose: {bound:e} vs realized {worst:e} — {}",
                    fuzz::replay()
                );
            }
        }
        assert!(
            successes >= QUOTA,
            "quota not met (dir_v={dir_v}): only {successes} successful removals — {}",
            fuzz::replay()
        );
    }
}

/// Row-major indexing sanity: a bilinear all-weights-1 surface must
/// interpolate its four corners with the documented iu*nv+iv layout.
#[test]
fn row_major_layout_pinned_by_corner_interpolation() {
    let ku = KnotVector::unit_segment(1);
    let kv = KnotVector::unit_segment(1);
    let corners = vec![
        Point3::new(0.0, 0.0, 0.0),  // iu=0 (u=0), iv=0 (v=0)
        Point3::new(0.0, 1.0, 5.0),  // iu=0, iv=1 (v=1)
        Point3::new(1.0, 0.0, -5.0), // iu=1 (u=1), iv=0
        Point3::new(1.0, 1.0, 0.0),  // iu=1, iv=1
    ];
    let s = NurbsSurface::new(ku, kv, corners, vec![1.0; 4]).unwrap();
    assert!(supp(s.eval(0.0, 0.0), Point3::new(0.0, 0.0, 0.0)) < 1e-15);
    assert!(supp(s.eval(0.0, 1.0), Point3::new(0.0, 1.0, 5.0)) < 1e-15);
    assert!(supp(s.eval(1.0, 0.0), Point3::new(1.0, 0.0, -5.0)) < 1e-15);
    assert!(supp(s.eval(1.0, 1.0), Point3::new(1.0, 1.0, 0.0)) < 1e-15);
    // transpose involution
    let t2 = s.transposed().transposed();
    assert!(supp(t2.eval(0.3, 0.8), s.eval(0.3, 0.8)) < 1e-15);
    // transposed swaps parameters
    assert!(supp(s.transposed().eval(0.8, 0.3), s.eval(0.3, 0.8)) < 1e-12);
}
