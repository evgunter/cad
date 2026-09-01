//! BOOL-3 R2 review probes: `line_torus_roots`'s certified count and
//! constructed roots checked against an independent root counter.
//!
//! The independent counter never sees the arm's algebra: it evaluates
//! the torus implicit `F(t) = (|w|² + R² − r²)² − 4R²(|w|² − h²)`
//! directly along the line, brackets the root region from the
//! `|w| ≤ R + r` condition, counts sign changes on a fine grid, and
//! bisects each. Simple roots only — which is exactly the regime the
//! arm CLAIMS to answer in (a definite discriminant), so a Certified
//! answer whose count or root positions disagree with the counter is a
//! broken sign chain, not a counter artifact.

use geom_core::{Band, Point3, Tol, Vec3};

use super::solid_contain::{TorusRoots, line_torus_roots};

const R_MAJ: f64 = 1.0;
const R_MIN: f64 = 0.3;

fn f_torus(q: Point3<f64>, d: Vec3<f64>, t: f64) -> f64 {
    let w = Vec3::new(q.x + d.x * t, q.y + d.y * t, q.z + d.z * t);
    let h = w.y; // axis = +y, center = origin
    let n2 = w.norm_squared();
    (n2 + R_MAJ * R_MAJ - R_MIN * R_MIN).powi(2) - 4.0 * R_MAJ * R_MAJ * (n2 - h * h)
}

/// Independent simple-root finder: bracket, grid, sign changes, bisect.
fn bisect_roots(q: Point3<f64>, d: Vec3<f64>) -> Vec<f64> {
    let w0 = Vec3::new(q.x, q.y, q.z);
    let b = w0.dot(d);
    let reach = (R_MAJ + R_MIN) * 1.0000001;
    let disc = b * b - w0.norm_squared() + reach * reach;
    if disc <= 0.0 {
        return Vec::new();
    }
    let (lo, hi) = (-b - disc.sqrt() - 1e-6, -b + disc.sqrt() + 1e-6);
    let n = 20000usize;
    let step = (hi - lo) / n as f64;
    let mut roots = Vec::new();
    let mut prev_t = lo;
    let mut prev_f = f_torus(q, d, lo);
    for i in 1..=n {
        let t = lo + step * i as f64;
        let f = f_torus(q, d, t);
        if prev_f == 0.0 {
            roots.push(prev_t);
        } else if f != 0.0 && prev_f.signum() != f.signum() {
            let (mut a, mut c) = (prev_t, t);
            let mut fa = prev_f;
            for _ in 0..80 {
                let m = 0.5 * (a + c);
                let fm = f_torus(q, d, m);
                if fm == 0.0 {
                    a = m;
                    c = m;
                    break;
                }
                if fa.signum() != fm.signum() {
                    c = m;
                } else {
                    a = m;
                    fa = fm;
                }
            }
            roots.push(0.5 * (a + c));
        }
        prev_t = t;
        prev_f = f;
    }
    roots
}

fn probe(q: Point3<f64>, d_raw: Vec3<f64>, band: Band) -> (Option<(usize, Vec<f64>)>, bool) {
    let d = d_raw.normalize();
    match line_torus_roots(
        q,
        d,
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        R_MAJ,
        R_MIN,
        band,
    ) {
        Ok(TorusRoots::Miss) => (Some((0, Vec::new())), false),
        Ok(TorusRoots::Uncertain) => (None, false),
        Ok(TorusRoots::Certified { count, ts }) => {
            let mut v: Vec<f64> = ts[..count].to_vec();
            v.sort_by(f64::total_cmp);
            (Some((count, v)), false)
        }
        // An in-band escalation is the honest decline; only the rung-5
        // `Invalid` (constructed ≠ certified) is a broken premise.
        Err(e) => (
            None,
            matches!(e.margin, geom_core::MarginDiag::Invalid),
        ),
    }
}

/// Adversarial rays: through the hole (four roots), axis-parallel,
/// midplane, tangent to the inner and outer equators, through the
/// centre, near-tangent to the top circle.
#[test]
fn r2_adversarial_rays_agree_with_the_independent_counter() {
    let band = Band::linear(Tol::witness()).unwrap();
    let ex = Vec3::new(1.0, 0.0, 0.0);
    let ez = Vec3::new(0.0, 0.0, 1.0);
    let ey = Vec3::new(0.0, 1.0, 0.0);
    // (query, direction, expected certainty: Some(count) definite,
    // None = the door may honestly decline)
    let cases: Vec<(Point3<f64>, Vec3<f64>, Option<usize>)> = vec![
        (Point3::new(0.0, 0.0, 0.0), ex, Some(4)), // hole centre, midplane
        // The exact axis line: perp = 0 makes the quartic (y² + M)² —
        // Δ ≡ 0 identically, so declining is the ladder's rung 1, even
        // though the geometric miss is definite. (Verified: the arm
        // returns Uncertain here, and the schedule's rays are never
        // axis-aligned from a body's own interior lattice.)
        (Point3::new(0.0, 0.0, 0.0), ey, None),
        (Point3::new(2.0, 0.0, 0.0), ex, Some(4)),
        (Point3::new(0.65, 0.0, -5.0), ez, Some(4)), // chord inside the hole radius
        (Point3::new(0.7, 0.0, -5.0), ez, None),     // tangent to the inner equator
        (Point3::new(1.3, 0.0, -5.0), ez, None),     // tangent to the outer equator
        (Point3::new(1.0, 0.0, -5.0), ez, Some(2)),
        (Point3::new(1.0, -5.0, 0.0), ey, Some(2)), // axis-parallel through the spine
        (Point3::new(1.3 - 1e-12, 0.0, -5.0), ez, None), // inside the tangency band
        (Point3::new(0.31, 0.27, -3.0), ez, Some(4)), // grazes high in the tube
        // Through the centre: pierces the tube on both sides.
        (Point3::new(2.0, 1.0, 3.0), Vec3::new(-2.0, -1.0, -3.0), Some(4)),
    ];
    for (q, d_raw, expect) in cases {
        let (got, escalated) = probe(q, d_raw, band);
        assert!(!escalated, "typed Invalid at {q:?} {d_raw:?}");
        let independent = bisect_roots(q, d_raw.normalize());
        match (expect, got) {
            (Some(want), Some((count, ts))) => {
                assert_eq!(count, want, "count at {q:?} {d_raw:?}");
                assert_eq!(count, independent.len(), "vs counter at {q:?} {d_raw:?}");
                for (a, b) in ts.iter().zip(independent.iter()) {
                    assert!(
                        (a - b).abs() < 1e-6,
                        "root {a} vs bisected {b} at {q:?} {d_raw:?}"
                    );
                }
            }
            (Some(want), None) => panic!("declined a definite {want}-root ray {q:?} {d_raw:?}"),
            (None, _) => {} // near-degenerate: declining is the contract
        }
    }
}

/// A deterministic lattice: 5×5×5 origins × 18 directions. Zero
/// tolerance for a Certified answer that disagrees with the counter in
/// count or in any root position; Uncertain/escalation rate bounded.
#[test]
fn r2_lattice_certified_counts_agree_with_the_independent_counter() {
    let band = Band::linear(Tol::witness()).unwrap();
    let mut dirs: Vec<Vec3<f64>> = Vec::new();
    for &x in &[-1.0, 0.0, 1.0] {
        for &y in &[-1.0, 0.0, 1.0] {
            for &z in &[-1.0, 0.0, 1.0] {
                if x != 0.0 || y != 0.0 || z != 0.0 {
                    dirs.push(Vec3::new(x, y, z));
                }
            }
        }
    }
    dirs.push(Vec3::new(0.21, 0.63, 0.75));
    dirs.push(Vec3::new(-0.4, 0.11, 0.91));
    let mut total = 0usize;
    let mut declined = 0usize;
    let mut mismatches: Vec<String> = Vec::new();
    for i in 0..5 {
        for j in 0..5 {
            for k in 0..5 {
                let q = Point3::new(
                    -2.0 + f64::from(i),
                    (-2.0 + f64::from(j)) * 0.31,
                    -2.0 + f64::from(k) * 0.97,
                );
                for d_raw in &dirs {
                    total += 1;
                    let (got, escalated) = probe(q, *d_raw, band);
                    if escalated {
                        mismatches.push(format!("typed Invalid at {q:?} {d_raw:?}"));
                        continue;
                    }
                    let Some((count, ts)) = got else {
                        declined += 1;
                        continue;
                    };
                    let independent = bisect_roots(q, d_raw.normalize());
                    if count != independent.len() {
                        mismatches.push(format!(
                            "count {count} vs counter {} at {q:?} {d_raw:?}",
                            independent.len()
                        ));
                        continue;
                    }
                    for (a, b) in ts.iter().zip(independent.iter()) {
                        if (a - b).abs() > 1e-6 {
                            mismatches.push(format!(
                                "root {a} vs bisected {b} at {q:?} {d_raw:?}"
                            ));
                        }
                    }
                }
            }
        }
    }
    assert!(
        mismatches.is_empty(),
        "{} certified answers disagree with the independent counter, e.g. {}",
        mismatches.len(),
        mismatches[0]
    );
    assert!(total >= 3000, "lattice too small: {total}");
    assert!(
        declined * 10 < total,
        "declined {declined} of {total}: the certified door is refusing generic rays"
    );
}
