//! Review probes for PR 6092's fix pass (`ae1ba4fa..4aeb53a3`): a
//! many-span face's unpriced per-span set-up beside the one per-face
//! wall, the cut lists' dedup, and the loose-knot end rule on a
//! degree-1 patch, on a face whose rectangle ends on its carrier's
//! interior C⁰ knot, and at a knot of multiplicity p + 1.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::type_complexity
)]

use crate::shared::patch::patch_net;
use crate::shared::ring::p3 as p;
use geom_brep::props::quad::{LaneEnd, LaneRun, RoundWindow, nurbs_patch_face_rounds};
use geom_core::interval::certification::Certification;
use geom_core::spline::KnotVector;
use geom_core::spline::algebra::split_points;
use geom_core::{Bounds, Interval};

fn uniform(degree: usize, spans: usize) -> KnotVector {
    let mut k = vec![0.0; degree + 1];
    k.extend((1..spans).map(|i| i as f64 / spans as f64));
    k.extend(vec![1.0; degree + 1]);
    KnotVector::clamped(k, degree).unwrap()
}

fn run(
    ku: &KnotVector,
    kv: &KnotVector,
    shape: &dyn Fn(f64, f64) -> [f64; 3],
    weight: &dyn Fn(usize, usize) -> f64,
    rect: (f64, f64, f64, f64),
    window: RoundWindow,
) -> LaneRun {
    let (nu, nv) = (ku.control_count(), kv.control_count());
    let mut cp = Vec::new();
    let mut w = Vec::new();
    for a in 0..nu {
        for b in 0..nv {
            let c = shape(a as f64 / (nu - 1) as f64, b as f64 / (nv - 1) as f64);
            cp.push(p(c[0], c[1], c[2]));
            w.push(weight(a, b));
        }
    }
    nurbs_patch_face_rounds::<f64>(
        &patch_net(ku, kv, &cp, &w),
        rect,
        0.0,
        0.0,
        crate::shared::tol::band(),
        window,
        &mut |_, _| Ok(false),
    )
    .expect("the patch has an enclosure")
}

fn wavy(x: f64, y: f64) -> [f64; 3] {
    [
        2.0 * x + 0.1 * y,
        1.5 * y,
        0.3 * (3.0 * x).sin() * (2.0 * y).cos() + 0.2,
    ]
}

/// **A many-span face's per-span set-up is linear in its span cells.**
/// The work wall prices the composite rounds' cells only; the area
/// pass, the hull blocks and the rational lane's statements run before
/// any round, so a set-up that grew faster than the span cells would
/// pass any wall. Each loop collapses u once per column and v per cell
/// (`net::grid_slice`). Measured in release, one thread, this box
/// (`DELTA_SPANS` × `DELTA_SPANS` uniform biquadratic spans; `DELTA_W`
/// the odd-node weight, 1 polynomial):
///
/// ```text
/// spans  w    main        head, a row collapse per cell   head
/// 10     1    0.51 s      6.9 s                           0.80 s
/// 30     1    6.6 s       160 s                           10.3 s
/// 12     1.5  4.6 s       16.3 s                          11.0 s
/// ```
///
/// The gate runs 10 × 10 against a 60 s budget, which the per-cell row
/// collapse (≈ N³) misses by about ten times in a dev build; other sizes
/// by `DELTA_SPANS`/`DELTA_W`.
#[test]
fn a_many_span_faces_set_up_is_linear_in_its_span_cells() {
    let env = |k: &str, d: f64| -> f64 {
        std::env::var(k)
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(d)
    };
    let spans = env("DELTA_SPANS", 10.0) as usize;
    let w = env("DELTA_W", 1.0);
    let k = uniform(2, spans);
    let t = std::time::Instant::now();
    let r = run(
        &k,
        &k,
        &wavy,
        &|a, b| if (a + b).is_multiple_of(2) { 1.0 } else { w },
        (0.0, 1.0, 0.0, 1.0),
        RoundWindow::SCHEDULE,
    );
    let took = t.elapsed();
    println!("delta-6092 {spans}x{spans} w {w}: {r:?} in {took:?}");
    assert!(
        took.as_secs_f64() < 60.0,
        "a {spans} x {spans}-span face took {took:?} under a per-face wall: {r:?}"
    );
}

/// **A knot of multiplicity p + 1 inside a degree-2 vector**: printed,
/// whether the vector is built and what the patch door answers on it.
#[test]
fn a_full_multiplicity_interior_knot_is_answered_or_refused() {
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.4, 0.4, 0.4, 1.0, 1.0, 1.0], 2);
    println!("delta-6092 triple knot: {:?}", k.as_ref().map(|_| ()));
    if let Ok(ku) = k {
        let kv = uniform(2, 1);
        let (nu, nv) = (ku.control_count(), kv.control_count());
        let mut cp = Vec::new();
        for i in 0..nu {
            for j in 0..nv {
                let c = wavy(i as f64 / (nu - 1) as f64, j as f64 / (nv - 1) as f64);
                cp.push(p(c[0], c[1], c[2]));
            }
        }
        let r = nurbs_patch_face_rounds::<f64>(
            &patch_net(&ku, &kv, &cp, &vec![1.0; nu * nv]),
            (0.0, 1.0, 0.0, 1.0),
            0.0,
            0.0,
            crate::shared::tol::band(),
            RoundWindow::at(0),
            &mut |_, _| Ok(false),
        );
        println!("delta-6092 triple knot door: {r:?}");
    }
}

/// **The per-span split mints no duplicate cut on a three-ulp span.**
/// On `[1, 1 + 3 ulp]` in four pieces the second and third points both
/// round to `1 + 2 ulp`; `split_points` keeps one, so the cut lists and
/// the hull-block edges built on it hold no zero-width cell. Red while
/// it dropped only points collapsing onto an END.
#[test]
fn split_points_collapse_onto_each_other_on_a_three_ulp_span() {
    let lo = 1.0_f64;
    let hi = f64::from_bits(lo.to_bits() + 3);
    let pts: Vec<f64> = split_points(lo, hi, 4).collect();
    assert_eq!(
        pts,
        [
            f64::from_bits(lo.to_bits() + 1),
            f64::from_bits(lo.to_bits() + 2)
        ],
        "the collapsed point is dropped, the two distinct ones kept"
    );
}

/// The patch door on a degree-2 patch with a three-ulp interior span:
/// whatever it answers (an enclosure or a typed refusal), printed
/// beside the four-ulp control whose split points are distinct.
#[test]
fn a_three_ulp_span_against_a_four_ulp_one() {
    for (ulps, w) in [(3u64, 1.0), (4, 1.0), (3, 2.0), (4, 2.0)] {
        let a = 0.5_f64;
        let b = f64::from_bits(a.to_bits() + ulps);
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, a, b, 1.0, 1.0, 1.0], 2).unwrap();
        let kv = uniform(2, 1);
        let (nu, nv) = (ku.control_count(), kv.control_count());
        let mut cp = Vec::new();
        let mut ws = Vec::new();
        for i in 0..nu {
            for j in 0..nv {
                let c = wavy(i as f64 / (nu - 1) as f64, j as f64 / (nv - 1) as f64);
                cp.push(p(c[0], c[1], c[2]));
                ws.push(if (i + j).is_multiple_of(2) { 1.0 } else { w });
            }
        }
        let r = nurbs_patch_face_rounds::<f64>(
            &patch_net(&ku, &kv, &cp, &ws),
            (0.0, 1.0, 0.0, 1.0),
            0.0,
            0.0,
            crate::shared::tol::band(),
            RoundWindow::at(0),
            &mut |_, _| Ok(false),
        );
        println!("delta-6092 {ulps}-ulp span, w {w}: {r:?}");
    }
}

fn width(x: Interval) -> f64 {
    x.hi() - x.lo()
}

/// **A degree-1 patch read whole and as its tiles.** Every interior
/// knot of a degree-1 vector is C⁰, and its derivative lines are degree
/// 0 on loose knots, where `loose_spans_over`'s end rule is the whole
/// story. Whole and tiles' sum at rounds 0 and 2: widths alike and the
/// two brackets overlapping.
#[test]
fn a_degree_one_patch_reads_as_its_tiles() {
    let eu = [0.0, 0.13, 0.61, 1.0];
    let ev = [0.2, 0.55, 3.7];
    let kv_of = |e: &[f64]| {
        let mut k = vec![e[0]; 2];
        k.extend(&e[1..e.len() - 1]);
        k.extend(vec![*e.last().unwrap(); 2]);
        KnotVector::clamped(k, 1).unwrap()
    };
    let shape = |a: usize, b: usize| {
        let (x, y) = (eu[a], ev[b]);
        [
            2.0 * x + 0.1 * y + 0.05 * (a * b) as f64,
            1.5 * y,
            0.3 * (3.0 * x).sin() * (2.0 * y).cos() + 0.2,
        ]
    };
    let go =
        |eu: &[f64], ev: &[f64], ia: usize, jb: usize, w: &dyn Fn(usize, usize) -> f64, round| {
            let (ku, kv) = (kv_of(eu), kv_of(ev));
            let mut cp = Vec::new();
            let mut ws = Vec::new();
            for a in 0..eu.len() {
                for b in 0..ev.len() {
                    let c = shape(ia + a, jb + b);
                    cp.push(p(c[0], c[1], c[2]));
                    ws.push(w(ia + a, jb + b));
                }
            }
            nurbs_patch_face_rounds::<f64>(
                &patch_net(&ku, &kv, &cp, &ws),
                (eu[0], *eu.last().unwrap(), ev[0], *ev.last().unwrap()),
                0.0,
                0.0,
                crate::shared::tol::band(),
                RoundWindow::at(round),
                &mut |_, _| Ok(false),
            )
            .expect("an enclosure")
        };
    let weights: [(&str, &dyn Fn(usize, usize) -> f64); 2] = [
        ("polynomial", &|_, _| 1.0),
        ("rational", &|a, b| {
            if (a + b).is_multiple_of(2) { 0.7 } else { 1.6 }
        }),
    ];
    for (name, w) in weights {
        for round in [0, 2] {
            let whole = go(&eu, &ev, 0, 0, w, round);
            let (mut flux, mut area) = (Interval::zero(), Interval::zero());
            for i in 0..eu.len() - 1 {
                for j in 0..ev.len() - 1 {
                    let t = go(&eu[i..=i + 1], &ev[j..=j + 1], i, j, w, round);
                    flux = flux + t.bounds.flux;
                    area = area + t.bounds.area;
                }
            }
            println!(
                "delta-6092 deg1 {name} r{round}: whole flux {:?} w {:e} tiles {:?} w {:e} | \
                 whole area {:?} w {:e} tiles {:?} w {:e}",
                whole.bounds.flux,
                width(whole.bounds.flux),
                flux,
                width(flux),
                whole.bounds.area,
                width(whole.bounds.area),
                area,
                width(area)
            );
            for (what, a, b) in [
                ("flux", whole.bounds.flux, flux),
                ("area", whole.bounds.area, area),
            ] {
                assert!(
                    (width(a) - width(b)).abs() <= 1e-9 * width(b) + 1e-12,
                    "{name} r{round}: whole {what} width {:e} vs tiles {:e}",
                    width(a),
                    width(b)
                );
                assert!(
                    a.lo() <= b.hi() && b.lo() <= a.hi(),
                    "{name} r{round}: whole {what} {a:?} and tiles {b:?} are disjoint"
                );
            }
        }
    }
}

/// **A face whose rectangle ENDS on an interior C⁰ knot of its
/// carrier**, against the same face built as its own one-span tile:
/// the carrier's net runs past the face edge, so the end rule decides
/// whether the face's last cells read the span beyond. Degree 2,
/// double knot at 0.61, rect `[0, 0.61] × [0, 1]`, and the mirror rect
/// starting on it.
#[test]
fn a_face_ending_on_its_carriers_c0_knot_reads_as_the_tile() {
    let c0 = |ends: &[f64]| {
        let mut k = vec![ends[0]; 3];
        for e in &ends[1..ends.len() - 1] {
            k.extend([*e, *e]);
        }
        k.extend(vec![*ends.last().unwrap(); 3]);
        KnotVector::clamped(k, 2).unwrap()
    };
    let eu = [0.0, 0.61, 1.0];
    let kv = uniform(2, 1);
    let shape = |a: usize, b: usize| {
        let x = a as f64 / 4.0;
        let y = b as f64 / 2.0;
        [
            2.0 * x + 0.1 * y,
            1.5 * y + 0.2 * (a % 2) as f64,
            0.3 * (3.0 * x).sin() * (2.0 * y).cos() + 0.2,
        ]
    };
    let net_of = |rows: std::ops::RangeInclusive<usize>, w: &dyn Fn(usize, usize) -> f64| {
        let mut cp = Vec::new();
        let mut ws = Vec::new();
        for a in rows {
            for b in 0..3 {
                let c = shape(a, b);
                cp.push(p(c[0], c[1], c[2]));
                ws.push(w(a, b));
            }
        }
        (cp, ws)
    };
    let weights: [(&str, &dyn Fn(usize, usize) -> f64); 2] = [
        ("polynomial", &|_, _| 1.0),
        ("rational", &|a, b| {
            if (a + b).is_multiple_of(2) { 0.7 } else { 1.6 }
        }),
    ];
    for (name, w) in weights {
        for (side, rect, rows, tile_ends) in [
            ("left", (0.0, 0.61, 0.0, 1.0), 0..=2usize, [0.0, 0.61]),
            ("right", (0.61, 1.0, 0.0, 1.0), 2..=4usize, [0.61, 1.0]),
        ] {
            let ku = c0(&eu);
            let (cp, ws) = net_of(0..=4, w);
            let face = nurbs_patch_face_rounds::<f64>(
                &patch_net(&ku, &kv, &cp, &ws),
                rect,
                0.0,
                0.0,
                crate::shared::tol::band(),
                RoundWindow::at(1),
                &mut |_, _| Ok(false),
            )
            .expect("the face has an enclosure");
            let kt = c0(&tile_ends);
            let (cpt, wst) = net_of(rows, w);
            let tile = nurbs_patch_face_rounds::<f64>(
                &patch_net(&kt, &kv, &cpt, &wst),
                rect,
                0.0,
                0.0,
                crate::shared::tol::band(),
                RoundWindow::at(1),
                &mut |_, _| Ok(false),
            )
            .expect("the tile has an enclosure");
            println!(
                "delta-6092 edge-knot {name} {side}: face flux {:?} area {:?}\n\
                 \x20  tile flux {:?} area {:?}",
                face.bounds.flux, face.bounds.area, tile.bounds.flux, tile.bounds.area
            );
            for (what, a, b) in [
                ("flux", face.bounds.flux, tile.bounds.flux),
                ("area", face.bounds.area, tile.bounds.area),
            ] {
                assert!(
                    (width(a) - width(b)).abs() <= 1e-9 * width(b) + 1e-12,
                    "{name} {side}: face {what} width {:e} vs tile {:e}",
                    width(a),
                    width(b)
                );
            }
            assert!(!matches!(face.end, LaneEnd::Wall));
        }
    }
}
