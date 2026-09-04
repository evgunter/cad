//! CERT-N3 blinded review (R1) — item 6: D31 bit identity, re-taken.
//! The two retired spellings are copied VERBATIM from the merge base
//! (`sweep::skin::make_compatible`'s union loop, b7f347254; `fit::
//! deviation_from`'s, b7f347254) and run beside `refine_to_union` on an
//! independent corpus; knots, control points and weights are compared
//! bit for bit. Probe branch only.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::too_many_lines)]

use geom::curves::nurbs::NurbsCurve3;
use geom_core::Point3;
use geom_core::spline::KnotVector;
use sweep::skin::make_compatible;

// ---- retired spelling A: sweep/src/skin.rs @ b7f347254 (verbatim body) ----
fn retired_skin_union(elevated: &[NurbsCurve3<f64>]) -> Vec<NurbsCurve3<f64>> {
    let mut union: Vec<(f64, usize)> = Vec::new();
    for c in elevated {
        for (value, mult) in c.knots().interior_knots() {
            match union.iter_mut().find(|(v, _)| *v == value) {
                Some((_, m)) => *m = (*m).max(mult),
                None => union.push((value, mult)),
            }
        }
    }
    union.sort_by(|a, b| a.0.total_cmp(&b.0));
    let merged: Vec<NurbsCurve3<f64>> = elevated
        .iter()
        .map(|c| {
            let own: Vec<(f64, usize)> = c.knots().interior_knots().collect();
            let mut add: Vec<f64> = Vec::new();
            for (value, want) in &union {
                let have = own
                    .iter()
                    .find(|(v, _)| *v == *value)
                    .map_or(0, |(_, m)| *m);
                for _ in have..*want {
                    add.push(*value);
                }
            }
            if add.is_empty() {
                Ok(c.clone())
            } else {
                c.refine_knots(&add)
            }
        })
        .collect::<Result<_, _>>()
        .unwrap();
    merged
}

// ---- retired spelling B: geom/src/curves/fit.rs @ b7f347254 (verbatim body) ----
fn retired_fit_pair(this: &NurbsCurve3<f64>, reference: &NurbsCurve3<f64>) -> (NurbsCurve3<f64>, NurbsCurve3<f64>) {
    let mut need: Vec<(f64, usize)> = Vec::new();
    for kv in [this.knots(), reference.knots()] {
        for (u, m) in kv.interior_knots() {
            match need.iter_mut().find(|(v, _)| *v == u) {
                Some(entry) => entry.1 = entry.1.max(m),
                None => need.push((u, m)),
            }
        }
    }
    let refine_to = |c: &NurbsCurve3<f64>| -> NurbsCurve3<f64> {
        let own: Vec<(f64, usize)> = c.knots().interior_knots().collect();
        let mut add: Vec<f64> = Vec::new();
        for (u, m) in &need {
            let have = own.iter().find(|(v, _)| *v == *u).map_or(0, |(_, s)| *s);
            for _ in have..*m {
                add.push(*u);
            }
        }
        if add.is_empty() {
            c.clone()
        } else {
            c.refine_knots(&add).unwrap()
        }
    };
    (refine_to(this), refine_to(reference))
}

fn bits(c: &NurbsCurve3<f64>) -> Vec<u64> {
    let mut v: Vec<u64> = c.knots().knots().iter().map(|x| x.to_bits()).collect();
    v.push(c.knots().degree() as u64);
    for p in c.control() {
        v.extend([p.x.to_bits(), p.y.to_bits(), p.z.to_bits()]);
    }
    v.extend(c.weights().iter().map(|w| w.to_bits()));
    v
}

fn curve(degree: usize, interior: &[(f64, usize)], seed: u64, rational: bool) -> NurbsCurve3<f64> {
    let mut knots = vec![0.0; degree + 1];
    for &(u, m) in interior {
        knots.extend(core::iter::repeat_n(u, m));
    }
    knots.extend(core::iter::repeat_n(1.0, degree + 1));
    let kv = KnotVector::clamped(knots, degree).unwrap();
    let n = kv.control_count();
    let mut s = seed;
    let mut rnd = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        (s % 10_000) as f64 / 10_000.0
    };
    let control: Vec<Point3<f64>> = (0..n)
        .map(|i| Point3::new(i as f64 + rnd(), 3.0 * rnd() - 1.0, 0.7 * rnd()))
        .collect();
    let weights: Vec<f64> = (0..n)
        .map(|_| if rational { 0.3 + 1.7 * rnd() } else { 1.0 })
        .collect();
    NurbsCurve3::new(kv, control, weights).unwrap()
}

/// A rational quadratic quarter arc (weights 1, √2/2, 1) at degree 2.
fn quarter_arc() -> NurbsCurve3<f64> {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let w = core::f64::consts::FRAC_1_SQRT_2;
    NurbsCurve3::new(
        kv,
        vec![Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 1.0, 0.0), Point3::new(0.0, 1.0, 0.0)],
        vec![1.0, w, 1.0],
    )
    .unwrap()
}

#[test]
fn n3r1_d31_bit_identity_against_both_retired_spellings() {
    let sets: Vec<(&str, Vec<NurbsCurve3<f64>>)> = vec![
        (
            "deg2 mismatched mults, rational",
            vec![
                curve(2, &[(0.5, 1)], 11, true),
                curve(2, &[(0.25, 1), (0.5, 2)], 12, true),
                curve(2, &[(0.75, 2)], 13, true),
            ],
        ),
        (
            "deg3 full-budget run + single span, mixed rational",
            vec![
                curve(3, &[(0.2, 3)], 21, false),
                curve(3, &[], 22, true),
                curve(3, &[(0.2, 1), (0.6, 2)], 23, true),
            ],
        ),
        (
            "insertion ORDER differs between the retired spellings (0.7 before 0.3)",
            vec![curve(2, &[(0.7, 1)], 31, true), curve(2, &[(0.3, 1)], 32, false)],
        ),
        (
            "rational arc + non-rational quadratic with interior knots",
            vec![quarter_arc(), curve(2, &[(0.4, 2), (0.9, 1)], 41, false)],
        ),
        (
            "already on the union (empty plans)",
            vec![curve(2, &[(0.5, 1)], 51, true), curve(2, &[(0.5, 1)], 52, true)],
        ),
        (
            "one curve holds a knot at multiplicity = degree",
            vec![curve(3, &[(0.5, 3)], 61, true), curve(3, &[(0.5, 1), (0.8, 1)], 62, true)],
        ),
    ];
    let mut compared = 0usize;
    for (what, set) in &sets {
        // Skin spelling (sorted union) vs the new door, whole set.
        let a = retired_skin_union(set);
        let b = NurbsCurve3::refine_to_union(set).unwrap();
        assert_eq!(a.len(), b.len());
        for (x, y) in a.iter().zip(&b) {
            let (bx, by) = (bits(x), bits(y));
            assert_eq!(bx, by, "{what}: skin spelling vs refine_to_union differ");
            compared += bx.len();
        }
        // Fit spelling (insertion-order union) vs the new door, per pair.
        for i in 0..set.len() {
            for j in 0..set.len() {
                if i == j {
                    continue;
                }
                let (p, q) = retired_fit_pair(&set[i], &set[j]);
                let r = NurbsCurve3::refine_to_union([&set[i], &set[j]]).unwrap();
                assert_eq!(bits(&p), bits(&r[0]), "{what}: fit spelling ({i},{j}) self differs");
                assert_eq!(bits(&q), bits(&r[1]), "{what}: fit spelling ({i},{j}) reference differs");
                compared += bits(&p).len() + bits(&q).len();
            }
        }
        // Every output shares one bit-identical knot vector.
        for y in &b {
            assert_eq!(y.knots().knots(), b[0].knots().knots(), "{what}: not on one vector");
        }
    }
    // Mixed degrees through the public skin door vs elevate + retired loop.
    let mixed = vec![curve(1, &[(0.5, 1)], 71, false), curve(2, &[(0.3, 1)], 72, true), curve(3, &[(0.5, 2)], 73, true)];
    let head = make_compatible(&mixed).unwrap();
    let elevated: Vec<NurbsCurve3<f64>> = mixed.iter().map(|c| c.elevate_degree(3 - c.knots().degree()).unwrap()).collect();
    let retired = retired_skin_union(&elevated);
    for (x, y) in head.iter().zip(&retired) {
        assert_eq!(bits(x), bits(y), "mixed degrees: make_compatible vs retired spelling differ");
        compared += bits(x).len();
    }
    eprintln!("n3r1 d31: {compared} u64 components compared bit-identical over {} sets + mixed", sets.len());
}
