//! CERT-N3 R2 blinded-review probe — probe branch only.
//!
//! Row D31's bit-identity claim, re-taken: the two RETIRED spellings
//! (copied verbatim from merge base `b7f34725`) against the one door.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::NurbsCurve3;
use geom_core::Point3;
use geom_core::spline::KnotVector;

/// `sweep::skin::make_compatible`'s union loop at the merge base,
/// verbatim (error mapping dropped; the arithmetic is what is compared).
fn retired_skin_spelling(elevated: &[NurbsCurve3<f64>]) -> Vec<NurbsCurve3<f64>> {
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
    elevated
        .iter()
        .map(|c| {
            let own: Vec<(f64, usize)> = c.knots().interior_knots().collect();
            let mut add: Vec<f64> = Vec::new();
            for (value, want) in &union {
                let have = own.iter().find(|(v, _)| *v == *value).map_or(0, |(_, m)| *m);
                for _ in have..*want {
                    add.push(*value);
                }
            }
            if add.is_empty() {
                c.clone()
            } else {
                c.refine_knots(&add).unwrap()
            }
        })
        .collect()
}

/// `geom::curves::fit::deviation_from`'s union loop at the merge base,
/// verbatim — note it does NOT sort the union.
fn retired_fit_spelling(a: &NurbsCurve3<f64>, b: &NurbsCurve3<f64>) -> Vec<NurbsCurve3<f64>> {
    let mut need: Vec<(f64, usize)> = Vec::new();
    for kv in [a.knots(), b.knots()] {
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
    vec![refine_to(a), refine_to(b)]
}

fn bits(c: &NurbsCurve3<f64>) -> String {
    let mut s = format!("deg {} knots", c.degree());
    for k in c.knots().knots() {
        s.push_str(&format!(" {:016x}", k.to_bits()));
    }
    s.push_str(" ctrl");
    for p in c.control() {
        s.push_str(&format!(
            " {:016x},{:016x},{:016x}",
            p.x.to_bits(),
            p.y.to_bits(),
            p.z.to_bits()
        ));
    }
    s.push_str(" w");
    for w in c.weights() {
        s.push_str(&format!(" {:016x}", w.to_bits()));
    }
    s
}

fn curve(degree: usize, interior: &[f64], weights: Option<&[f64]>) -> NurbsCurve3<f64> {
    let mut knots = vec![0.0; degree + 1];
    knots.extend_from_slice(interior);
    knots.extend(core::iter::repeat_n(1.0, degree + 1));
    let kv = KnotVector::clamped(knots, degree).unwrap();
    let n = kv.control_count();
    let control: Vec<Point3<f64>> = (0..n)
        .map(|i| {
            let t = i as f64;
            Point3::new(0.3 * t + 0.11, (0.7 * t).sin(), 1.0 / (1.0 + t))
        })
        .collect();
    let w: Vec<f64> = match weights {
        Some(w) => (0..n).map(|i| w[i % w.len()]).collect(),
        None => vec![1.0; n],
    };
    NurbsCurve3::new(kv, control, w).unwrap()
}

fn corpus() -> Vec<(String, Vec<NurbsCurve3<f64>>)> {
    let rat: &[f64] = &[1.0, 0.4, 2.3, 0.75, 1.7];
    vec![
        (
            "deg2 mismatched multiplicities, rational".into(),
            vec![
                curve(2, &[0.5], Some(rat)),
                curve(2, &[0.25, 0.5, 0.5], Some(rat)),
                curve(2, &[0.75, 0.75], Some(rat)),
            ],
        ),
        (
            "deg3 full-budget multiplicity + single span".into(),
            vec![
                curve(3, &[0.2, 0.2, 0.2], None),
                curve(3, &[], None),
                curve(3, &[0.6], Some(rat)),
            ],
        ),
        (
            "insertion ORDER differs (descending first vector)".into(),
            vec![
                curve(2, &[0.7, 0.9], None),
                curve(2, &[0.1, 0.3], None),
            ],
        ),
        (
            "rational + non-rational, same structure".into(),
            vec![curve(3, &[0.4], Some(rat)), curve(3, &[0.4], None)],
        ),
        (
            "already on the union (empty plan)".into(),
            vec![curve(2, &[0.5], None), curve(2, &[0.5], None)],
        ),
    ]
}

#[test]
fn n3r2_d31_union_door_is_bit_identical_to_both_retired_spellings() {
    for (name, curves) in corpus() {
        // The one door.
        let door = NurbsCurve3::refine_to_union(&curves).unwrap();
        // The retired skin spelling (n-ary).
        let skin = retired_skin_spelling(&curves);
        assert_eq!(door.len(), skin.len());
        for (i, (d, s)) in door.iter().zip(skin.iter()).enumerate() {
            assert_eq!(bits(d), bits(s), "{name}: skin spelling differs at {i}");
        }
        // The retired fit spelling (binary), over every pair.
        for i in 0..curves.len() {
            for j in 0..curves.len() {
                if i == j {
                    continue;
                }
                let fit = retired_fit_spelling(&curves[i], &curves[j]);
                let door2 = NurbsCurve3::refine_to_union([&curves[i], &curves[j]]).unwrap();
                for (k, (d, f)) in door2.iter().zip(fit.iter()).enumerate() {
                    assert_eq!(
                        bits(d),
                        bits(f),
                        "{name}: fit spelling differs for pair ({i},{j}) at {k}"
                    );
                }
            }
        }
    }
}

/// The elevation path: mixed degrees through `make_compatible`'s public
/// door against the retired spelling applied to the same elevated set.
#[test]
fn n3r2_d31_make_compatible_matches_the_retired_spelling_after_elevation() {
    let rat: &[f64] = &[1.0, 0.4, 2.3, 0.75];
    let sections = vec![
        curve(1, &[0.5], None),
        curve(2, &[0.25, 0.5], Some(rat)),
        curve(3, &[0.75], None),
    ];
    let got = sweep::skin::make_compatible(&sections).unwrap();
    let degree = sections.iter().map(NurbsCurve3::degree).max().unwrap();
    let elevated: Vec<NurbsCurve3<f64>> = sections
        .iter()
        .map(|c| {
            let raise = degree - c.degree();
            if raise == 0 {
                c.clone()
            } else {
                c.elevate_degree(raise).unwrap()
            }
        })
        .collect();
    let want = retired_skin_spelling(&elevated);
    assert_eq!(got.len(), want.len());
    for (i, (g, w)) in got.iter().zip(want.iter()).enumerate() {
        assert_eq!(bits(g), bits(w), "elevation path differs at section {i}");
    }
}
