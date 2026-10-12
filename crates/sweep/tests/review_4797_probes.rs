//! Review probes for PR 4797 (predicate 2 and the cap meter read three
//! ways). Each probe compares the verdict against an independently
//! computed truth for the fixture, never against the code's own output.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Bounds, Decide, ErrorTextReading, Interval, Point2, Point3, Real, Tol};
use profile::{ProfileLoop, SketchPlane, test_support::bulge_loop};
use sweep::blend::{BlendError, chamfer_edges, fillet_edges};
use sweep::test_support::{
    ROD_FILLET, ROD_FLAT, ROD_L, ROD_R, extruded, rod_chord_at, rod_creases,
};
use topo::{AtRestBody, AtRestPolicy, Body, EdgeKey, query};

use crate::common::interval::iv;

fn tol() -> Tol {
    Tol::witness()
}

const R: f64 = 0.5;

/// The block `[0, 4]³` less the box void `[x0, x1] × [y0, y1] × [z0, z1]`.
fn voided<T: Decide + Bounds + AtRestPolicy>(
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    (z0, z1): (f64, f64),
) -> AtRestBody<T> {
    let t = tol();
    let block = sweep::test_support::finished(
        "block",
        sweep::test_support::brick::<T>((0.0, 4.0), (0.0, 4.0), (0.0, 4.0), t),
        t,
    );
    let void = sweep::test_support::finished(
        "void",
        sweep::test_support::brick::<T>((x0, x1), (y0, y1), (z0, z1), t),
        t,
    );
    topo::subtract(&block, &void, t)
        .expect("the cut succeeds")
        .body()
        .expect("material")
        .body
        .clone()
}

fn the_edge<T: Bounds>(body: &Body<T>) -> EdgeKey {
    let near = |p: &Point3<T>, q: [f64; 3]| {
        [p.x, p.y, p.z]
            .iter()
            .zip(q)
            .all(|(c, w)| c.lo() <= w + 1e-12 && w - 1e-12 <= c.hi())
    };
    let (a, b) = ([0.0, 0.0, 0.0], [4.0, 0.0, 0.0]);
    query::all_edges(body)
        .into_iter()
        .find(|&e| {
            let he = body.get_edge(e).unwrap().he_plus;
            let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            let (s, t) = (
                p(body.get_half_edge(he).unwrap().start),
                p(body.half_edge_end(he).unwrap()),
            );
            (near(&s, a) && near(&t, b)) || (near(&s, b) && near(&t, a))
        })
        .expect("the edge y = z = 0")
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Verb {
    Fillet,
    Chamfer,
}

/// What a request did, coarsely.
#[derive(Debug, PartialEq, Clone, Copy)]
enum Got {
    Built,
    Inside,
    Crossing,
    Bound,
    Other,
}

fn coarse<B>(r: &Result<B, BlendError>) -> Got {
    match r {
        Ok(_) => Got::Built,
        Err(BlendError::WhollyInside { .. }) => Got::Inside,
        Err(BlendError::FaceClearance { bounded: false, .. })
        | Err(BlendError::RingClearance { bounded: false, .. }) => Got::Crossing,
        Err(BlendError::FaceClearance { bounded: true, .. })
        | Err(BlendError::RingClearance { bounded: true, .. }) => Got::Bound,
        Err(_) => Got::Other,
    }
}

fn run(verb: Verb, body: &AtRestBody<f64>, size: f64) -> Result<(), BlendError> {
    let e = [the_edge(body)];
    match verb {
        Verb::Fillet => fillet_edges(body, &e, size, tol()).map(|f| {
            topo::validate_geometric(&f.body, tol()).expect("tier 3");
        }),
        Verb::Chamfer => chamfer_edges(body, &e, size, tol()).map(|c| {
            topo::validate_geometric(&c.body, tol()).expect("tier 3");
        }),
    }
    .map_err(|e| e.error)
}

/// Whether the (y, z) point lies in the material the convex band of
/// size `R` removes from the right dihedral `y, z ≥ 0`, and how far from
/// that region's boundary (signed: positive inside).
fn material(verb: Verb, y: f64, z: f64) -> f64 {
    match verb {
        Verb::Fillet => {
            let d = ((y - R).powi(2) + (z - R).powi(2)).sqrt() - R;
            d.min(R - y).min(R - z).min(y).min(z)
        }
        Verb::Chamfer => ((R - y - z) / 2f64.sqrt()).min(y).min(z),
    }
}

/// The void's section read whole: the least and most of `material`
/// over a fine grid of its rectangle `[y0, y1] × [z0, z1]`.
fn truth(verb: Verb, (y0, y1): (f64, f64), (z0, z1): (f64, f64)) -> (f64, f64) {
    let n = 200;
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for i in 0..=n {
        for j in 0..=n {
            let y = y0 + (y1 - y0) * f64::from(i) / f64::from(n);
            let z = z0 + (z1 - z0) * f64::from(j) / f64::from(n);
            let m = material(verb, y, z);
            lo = lo.min(m);
            hi = hi.max(m);
        }
    }
    (lo, hi)
}

/// **A void whose near faces lie wholly inside the band's material
/// while its far faces cross it is a CROSSING feature** (the ruling:
/// "a feature crossing it still refuses"). The void `[0.01, 0.35]²`
/// in `(y, z)`: its faces `y = 0.01` and `z = 0.01` are wholly inside the
/// band's material (the fillet's: `(0.01 − 0.5)² + (0.35 − 0.5)² =
/// 0.2626 > 0.25`; the chamfer's: `0.36 < 0.5`), its faces `y = 0.35`,
/// `z = 0.35` and both end faces cross (corner `(0.35, 0.35)` is kept).
/// A `WhollyInside` refusal tells the author "delete the feature, which
/// the band's material wholly contains" about a feature it does not
/// contain.
#[test]
fn a_void_with_one_face_inside_and_others_crossing_reads_as_crossing() {
    let body = voided::<f64>((1.9, 2.1), (0.01, 0.35), (0.01, 0.35));
    for verb in [Verb::Fillet, Verb::Chamfer] {
        let (lo, hi) = truth(verb, (0.01, 0.35), (0.01, 0.35));
        assert!(
            lo < -0.01 && hi > 0.009,
            "{verb:?}: premise, the void crosses ({lo}, {hi})"
        );
        let got = run(verb, &body, R);
        assert_eq!(
            coarse(&got),
            Got::Crossing,
            "{verb:?}: a void crossing the band is read as a crossing, got {got:?}"
        );
    }
}

/// **A witness the scan below found**: the void `[0.12, 0.14] × [0.12,
/// 0.2]` in `(y, z)` crosses the fillet's arc (its corner `(0.12, 0.2)`
/// is `0.484 < R` from the ball's axis, so kept; its face `z = 0.12` is
/// wholly in the removed material, `(0.14, 0.12)` at `0.523`), yet the
/// request refuses `WhollyInside` naming that one face: the verdict is
/// the first face read, not the feature's.
#[test]
fn a_crossing_void_whose_first_face_read_is_inside_refuses_as_a_crossing() {
    let (y, z) = ((0.12, 0.14), (0.12, 0.2));
    let (lo, hi) = truth(Verb::Fillet, y, z);
    assert!(
        lo < -0.03 && hi > 0.03,
        "premise: the void crosses ({lo}, {hi})"
    );
    let body = voided::<f64>((1.9, 2.1), y, z);
    let got = run(Verb::Fillet, &body, R);
    assert_eq!(
        coarse(&got),
        Got::Crossing,
        "a void crossing the band is read as a crossing, got {got:?}"
    );
}

/// **Counterexample search over box voids** in the band's corner, both
/// verbs. Truth is the void's section read on a fine grid against the
/// closed-form material region; cases within `0.005` of the boundary
/// are skipped. A `WhollyInside` must be a void wholly inside, a
/// measured crossing a void that crosses, and a build a void wholly in
/// kept material; a bound is always allowed.
#[test]
fn box_voids_against_the_closed_form_region() {
    let starts = [0.01, 0.05, 0.12, 0.2, 0.3];
    let sizes = [0.02, 0.08, 0.25];
    let mut bad = Vec::new();
    let mut tally = std::collections::BTreeMap::new();
    for verb in [Verb::Fillet, Verb::Chamfer] {
        for &y0 in &starts {
            for &z0 in &starts {
                for &sy in &sizes {
                    let sz = sizes[(sizes.iter().position(|s| *s == sy).unwrap() + 1) % 3];
                    let (y1, z1) = (y0 + sy, z0 + sz);
                    let (lo, hi) = truth(verb, (y0, y1), (z0, z1));
                    let near = lo.abs() < 0.005 || hi.abs() < 0.005;
                    if near {
                        continue;
                    }
                    let body = voided::<f64>((1.9, 2.1), (y0, y1), (z0, z1));
                    let got = run(verb, &body, R);
                    let g = coarse(&got);
                    *tally.entry(format!("{verb:?} {g:?}")).or_insert(0) += 1;
                    let ok = match g {
                        Got::Inside => lo > 0.0,
                        Got::Crossing => lo < 0.0 && hi > 0.0,
                        Got::Built => hi < 0.0,
                        Got::Bound => true,
                        Got::Other => false,
                    };
                    if !ok {
                        bad.push(format!(
                            "{verb:?} y [{y0}, {y1}] z [{z0}, {z1}] truth ({lo:.4}, {hi:.4}) got {got:?}"
                        ));
                    }
                }
            }
        }
    }
    eprintln!("tally: {tally:?}");
    assert!(
        bad.is_empty(),
        "verdicts against truth:\n{}",
        bad.join("\n")
    );
}

/// **A void whose corner sits ON the fillet's arc** (`hi = R − R/√2`) is
/// never certified inside, nor built.
#[test]
fn a_void_touching_the_region_boundary_is_never_certified_inside() {
    let on_arc = R - R / 2f64.sqrt();
    let body = voided::<f64>((1.9, 2.1), (0.03, on_arc), (0.03, on_arc));
    let got = run(Verb::Fillet, &body, R);
    assert!(
        matches!(coarse(&got), Got::Bound | Got::Crossing),
        "a corner on the arc: not inside, not built, got {got:?}"
    );
}

/// **A void near the window's end** (`x ∈ [0.01, 0.2]`, the end face
/// `x = 0` being a cap of the band's window) wholly inside the band's
/// material is still read inside, or as a bound — never built, never
/// a measured crossing.
#[test]
fn a_void_near_the_windows_end_plane() {
    let body = voided::<f64>((0.01, 0.2), (0.03, 0.08), (0.03, 0.08));
    for verb in [Verb::Fillet, Verb::Chamfer] {
        let got = run(verb, &body, R);
        assert!(
            matches!(coarse(&got), Got::Inside | Got::Bound),
            "{verb:?}: got {got:?}"
        );
    }
}

/// **The hair cases replay at `Interval`**: `1e-3` inside the arc is
/// `WhollyInside` with an enclosure below zero; `1e-3` past it a
/// measured crossing.
#[test]
fn the_hair_cases_replay_at_interval() {
    let gap = 1e-3;
    let s2 = 2f64.sqrt();
    let inside = R - (R + gap) / s2;
    let across = R - (R - gap) / s2;
    let read = |hi: f64| {
        let b = voided::<Interval>((1.9, 2.1), (0.03, hi), (0.03, hi));
        fillet_edges(&b, &[the_edge(&b)], iv(R), tol())
            .map(|_| ())
            .map_err(|e| e.error)
    };
    let got = read(inside);
    let Err(BlendError::WhollyInside { margin, .. }) = &got else {
        panic!("1e-3 inside at Interval: WhollyInside, got {got:?}")
    };
    match margin.reading.diagnostic_f64_for_error_text() {
        ErrorTextReading::Enclosure { hi, .. } => assert!(hi < 0.0, "{margin}"),
        other => panic!("an enclosure, got {other:?}"),
    }
    let got = read(across);
    assert_eq!(
        coarse(&got),
        Got::Crossing,
        "1e-3 across at Interval, got {got:?}"
    );
}

// ---------------------------------------------------------------- the cap

fn bore<T: Real>(x: f64, y: f64, a: f64) -> ProfileLoop<T> {
    let f = T::from_f64;
    bulge_loop(vec![
        (Point2::new(f(x + a), f(y)), f(1.0)),
        (Point2::new(f(x - a), f(y)), f(1.0)),
    ])
}

fn d_loop<T: Real>() -> ProfileLoop<T> {
    let (c, f) = (rod_chord_at(ROD_FLAT), T::from_f64);
    bulge_loop(vec![
        (Point2::new(f(ROD_FLAT), f(c.half)), f(c.wall_bulge)),
        (Point2::new(f(ROD_FLAT), f(-c.half)), f(0.0)),
    ])
}

fn bored_rod(x: f64, y: f64, a: f64) -> Body<f64> {
    extruded(
        SketchPlane::xy(),
        vec![d_loop(), bore(x, y, a)],
        ROD_L,
        tol(),
    )
}

/// The sliver `S` of the upper crease's cut-off on the D-rod's cap:
/// outside the ball's section about `c`, inside the D, and in the wedge
/// between the feet. Signed: positive inside `S`.
fn in_sliver(x: f64, y: f64) -> f64 {
    let r = ROD_FILLET;
    let cx = ROD_FLAT - r;
    let cy = ((ROD_R - r).powi(2) - cx * cx).sqrt();
    let (fa, fb) = (
        (ROD_FLAT, cy),
        (cx * ROD_R / (ROD_R - r), cy * ROD_R / (ROD_R - r)),
    );
    let (qx, qy) = (x - cx, y - cy);
    let cross = |u: (f64, f64), v: (f64, f64)| u.0 * v.1 - u.1 * v.0;
    let (ua, ub) = ((fa.0 - cx, fa.1 - cy), (fb.0 - cx, fb.1 - cy));
    let wedge = (cross(ua, (qx, qy)) / r).min(cross((qx, qy), ub) / r);
    let out = qx.hypot(qy) - r;
    let flat = ROD_FLAT - x;
    let wall = ROD_R - x.hypot(y);
    out.min(flat).min(wall).min(wedge.max(-1.0))
}

/// **Counterexample search over bores on the D-rod's cap** around the
/// upper sliver. Truth: 128 points of the bore's circle read against
/// [`in_sliver`]. `WhollyInside` must be a bore wholly in `S`, a
/// measured crossing one with points either side, a build one with no
/// point in `S`.
#[test]
fn cap_bores_against_the_closed_form_sliver() {
    let a = 0.0015;
    let mut bad = Vec::new();
    let mut tally = std::collections::BTreeMap::new();
    let mut n = 0;
    for i in 0..12 {
        for j in 0..18 {
            let x = 0.235 + 0.0055 * f64::from(i);
            let y = 0.325 + 0.0065 * f64::from(j);
            if x + a > ROD_FLAT - 2e-4 || x.hypot(y) + a > ROD_R - 2e-4 {
                continue;
            }
            let pts: Vec<f64> = (0..128)
                .map(|k| {
                    let t = std::f64::consts::TAU * f64::from(k) / 128.0;
                    in_sliver(x + a * t.cos(), y + a * t.sin())
                })
                .collect();
            let lo = pts.iter().copied().fold(f64::INFINITY, f64::min);
            let hi = pts.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            if lo.abs() < 2e-4 || hi.abs() < 2e-4 {
                continue;
            }
            n += 1;
            let body = bored_rod(x, y, a);
            let creases = rod_creases(&body);
            let got = fillet_edges(
                &sweep::test_support::at_rest(&body, tol()),
                &creases,
                ROD_FILLET,
                tol(),
            )
            .map(|o| {
                topo::validate_geometric(&o.body, tol()).expect("tier 3");
            })
            .map_err(|e| e.error);
            let g = coarse(&got);
            *tally.entry(format!("{g:?}")).or_insert(0) += 1;
            let ok = match g {
                Got::Inside => lo > 0.0,
                Got::Crossing => lo < 0.0 && hi > 0.0,
                Got::Built => hi < 0.0,
                Got::Bound => true,
                Got::Other => false,
            };
            if !ok {
                bad.push(format!(
                    "bore ({x:.4}, {y:.4}) truth ({lo:.5}, {hi:.5}) got {got:?}"
                ));
            }
        }
    }
    eprintln!("{n} bores, tally: {tally:?}");
    assert!(n > 50, "the scan reads enough bores ({n})");
    assert!(
        bad.is_empty(),
        "verdicts against truth:\n{}",
        bad.join("\n")
    );
}

/// **The cap's hair cases at `Interval`**: on the ray from `c` to `V`, a
/// bore `1e-4` past the arc is `WhollyInside` with an enclosure holding
/// `−1e-4`; one `1e-4` short of it a measured crossing.
#[test]
fn cap_hair_cases_replay_at_interval() {
    let r = ROD_FILLET;
    let cx = ROD_FLAT - r;
    let c = (cx, ((ROD_R - r).powi(2) - cx * cx).sqrt());
    let v = (ROD_FLAT, (ROD_R.powi(2) - ROD_FLAT.powi(2)).sqrt());
    let (dx, dy) = (v.0 - c.0, v.1 - c.1);
    let n = dx.hypot(dy);
    let (a, hair) = (0.004, 1e-4);
    let on_ray = |d: f64| (c.0 + d * dx / n, c.1 + d * dy / n);
    let read = |(x, y): (f64, f64)| {
        let body = sweep::test_support::finished(
            "bored",
            extruded(
                SketchPlane::xy(),
                vec![d_loop(), bore::<Interval>(x, y, a)],
                Interval::from_f64(ROD_L),
                tol(),
            ),
            tol(),
        );
        fillet_edges(&body, &rod_creases(&body), iv(ROD_FILLET), tol())
            .map(|_| ())
            .map_err(|e| e.error)
    };
    let got = read(on_ray(r + a + hair));
    let Err(BlendError::WhollyInside { margin, .. }) = &got else {
        panic!("a hair past the arc at Interval: WhollyInside, got {got:?}")
    };
    let ErrorTextReading::Enclosure { lo, hi } = margin.reading.diagnostic_f64_for_error_text()
    else {
        panic!("an enclosure: {margin}")
    };
    assert!(
        lo <= -hair + 1e-12 && hi >= -hair - 1e-12 && hi < 0.0,
        "the enclosure holds −1e-4: [{lo}, {hi}]"
    );
    let got = read(on_ray(r + a - hair));
    assert_eq!(
        coarse(&got),
        Got::Crossing,
        "a hair short at Interval, got {got:?}"
    );
}

/// **Two bores on one cap, one wholly inside the sliver and one crossing
/// its arc**: the request crosses whichever ring is read first, so its
/// verdict should not depend on the order the profile lists them in.
/// Truth: the crossing bore refuses whatever is done with the inside one
/// (the ruling: "a feature crossing it still refuses").
#[test]
fn a_cap_with_an_inside_bore_and_a_crossing_bore_reads_as_a_crossing_in_either_order() {
    let r = ROD_FILLET;
    let cx = ROD_FLAT - r;
    let c = (cx, ((ROD_R - r).powi(2) - cx * cx).sqrt());
    let v = (ROD_FLAT, (ROD_R.powi(2) - ROD_FLAT.powi(2)).sqrt());
    let (dx, dy) = (v.0 - c.0, v.1 - c.1);
    let n = dx.hypot(dy);
    // Centred on the arc, on the ray from c to V: crosses it.
    let across = (c.0 + r * dx / n, c.1 + r * dy / n, 0.003);
    let inside = (0.297, 0.395, 0.002);
    let mut seen = Vec::new();
    for order in [[inside, across], [across, inside]] {
        let body = extruded(
            SketchPlane::xy(),
            vec![
                d_loop(),
                bore(order[0].0, order[0].1, order[0].2),
                bore(order[1].0, order[1].1, order[1].2),
            ],
            ROD_L,
            tol(),
        );
        let got = fillet_edges(
            &sweep::test_support::at_rest(&body, tol()),
            &rod_creases(&body),
            ROD_FILLET,
            tol(),
        )
        .map(|_| ())
        .map_err(|e| e.error);
        seen.push(coarse(&got));
    }
    assert!(
        seen.iter().all(|g| *g == Got::Crossing),
        "a cap carrying a crossing bore reads as a crossing in either order: {seen:?}"
    );
}

/// **The whole-rim reach (a plane and a cylinder) reads three ways
/// too**: the disc `ρ ≤ 2, z ∈ [0, 4]`, its top rim filleted at `0.5`,
/// removes the meridian region `ρ ∈ [1.5, 2], z ∈ [3.5, 4]` outside the
/// circle of radius `0.5` about `(1.5, 3.5)`. A box void at
/// `[1.94, 1.97] × [−0.01, 0.01] × [3.93, 3.97]` lies wholly in it
/// (nearest corner `0.615` from that centre, `0.03` under the top and the
/// wall); `[1.6, 1.97] × … × [3.6, 3.97]` crosses it (corner `(1.6, 3.6)`
/// is `0.14` from the centre); `[1.55, 1.6] × … × [3.55, 3.6]` is kept.
/// No row of the PR exercises the circular reach's three-way reading;
/// on this two-arc rim (no exact core) both read as a bound.
#[test]
fn a_rim_band_reads_a_void_three_ways() {
    let t = tol();
    let disc =
        sweep::test_support::finished("disc", sweep::test_support::disc_of_arcs(2, 2.0, 4.0, t), t);
    let read = |x: (f64, f64), z: (f64, f64)| {
        let void = sweep::test_support::finished(
            "void",
            sweep::test_support::brick::<f64>(x, (-0.01, 0.01), z, t),
            t,
        );
        let body = topo::subtract(&disc, &void, t)
            .expect("the cut")
            .body()
            .expect("material")
            .body
            .clone();
        let rim: Vec<EdgeKey> = query::all_edges(&body)
            .into_iter()
            .filter(|&e| {
                let he = body.get_edge(e).unwrap().he_plus;
                let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
                let (s, q) = (
                    p(body.get_half_edge(he).unwrap().start),
                    p(body.half_edge_end(he).unwrap()),
                );
                (s.z - 4.0).abs() < 1e-12
                    && (q.z - 4.0).abs() < 1e-12
                    && (s.x.hypot(s.y) - 2.0).abs() < 1e-9
            })
            .collect();
        assert_eq!(rim.len(), 2, "the top rim's two arcs");
        let got = fillet_edges(&body, &rim, 0.5, t)
            .map(|o| {
                topo::validate_geometric(&o.body, t).expect("tier 3");
            })
            .map_err(|e| e.error);
        coarse(&got)
    };
    let inside = read((1.94, 1.97), (3.93, 3.97));
    let across = read((1.6, 1.97), (3.6, 3.97));
    let kept = read((1.55, 1.6), (3.55, 3.6));
    // A rim of two links has no exact core (`circular_reach`'s `exact`
    // needs one closed link), so inside and crossing may only read as a
    // bound here; what must hold is the sound direction.
    assert!(
        matches!(inside, Got::Inside | Got::Bound)
            && matches!(across, Got::Crossing | Got::Bound)
            && kept == Got::Built,
        "the rim band: inside {inside:?}, crossing {across:?}, kept {kept:?}"
    );
}
