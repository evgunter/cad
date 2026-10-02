//! `point_in_solid` beside far carriers: a point in band of a face's
//! carrier is a question about that face only near the face.
//! Every row here asserts no wrong answer: a point within the band of
//! the boundary refuses or reads `OnBoundary`, never `In`/`Out`, and a
//! point the band leaves definite reads its true side.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/topo/src/boolean/solid_contain.rs",
    "crates/topo/src/splitting/containment.rs",
    "crates/topo/src/ray_parity.rs",
];

use crate::common;

use common::{brick, holed_block, prism_z};
use geom_core::{Band, Decide, Interval, Point3, Tol};
use topo::{Body, BooleanOp, SolidContainment, point_in_solid};

/// What a probe is allowed to read.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Want {
    In,
    Out,
    /// Within the band of the boundary: a refusal or `OnBoundary`.
    Near,
}

fn l_prism<T: Decide + topo::AtRestPolicy>() -> Body<T> {
    prism_z::<T>(
        &[
            (0.0, 0.0),
            (4.0, 0.0),
            (4.0, 2.0),
            (2.0, 2.0),
            (2.0, 4.0),
            (0.0, 4.0),
        ],
        0.0,
        1.0,
        Tol::witness(),
    )
    .body
}

/// Reads every probe; returns (probe, reading) for the ones that broke
/// `Want`, and counts the refusals among the definite ones.
fn read<T: Decide>(body: &Body<T>, probes: &[((f64, f64, f64), Want)]) -> (Vec<String>, usize) {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let mut wrong = Vec::new();
    let mut refused_definite = 0;
    let k = Tol::witness().get().k;
    let eps = Tol::witness().get().eps;
    for &((x, y, z), want) in probes {
        if want == Want::Near && probes_gap_is_definite(x, y, z, eps * k) {
            continue; // past the band, a "near" probe is a definite one
        }
        let q = Point3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z));
        let got = point_in_solid(body, q, band, tol);
        let ok = match (&got, want) {
            (Ok(SolidContainment::In), Want::In) | (Ok(SolidContainment::Out), Want::Out) => true,
            (Ok(SolidContainment::OnBoundary) | Err(_), Want::Near) => true,
            (Err(_), Want::In | Want::Out) => {
                refused_definite += 1;
                eprintln!("  refused a definite probe ({x}, {y}, {z}) want {want:?}: {got:?}");
                true
            }
            _ => false,
        };
        if !ok {
            wrong.push(format!("({x}, {y}, {z}) want {want:?}, got {got:?}"));
        }
    }
    (wrong, refused_definite)
}

/// Is every coordinate's distance from the nearest integer-or-half
/// lattice line beyond `limit`? The near probes sit at `lattice ± h`,
/// so past the band they are definite, and their side is not this
/// row's subject.
fn probes_gap_is_definite(x: f64, y: f64, z: f64, limit: f64) -> bool {
    let off = |c: f64| {
        let r = (c * 4.0).round() / 4.0;
        (c - r).abs()
    };
    [x, y, z].into_iter().map(off).fold(0.0, f64::max) >= limit
}

fn l_probes(h: f64) -> Vec<((f64, f64, f64), Want)> {
    use Want::*;
    vec![
        // In the leg, in band of the y = 2 face's carrier (x ∈ [2, 4]).
        ((1.0, 2.0 + h, 0.5), In),
        ((1.0, 2.0 - h, 0.5), In),
        ((0.25, 2.0 + h, 0.5), In),
        // In the foot, in band of the x = 2 face's carrier (y ∈ [2, 4]).
        ((2.0 + h, 1.0, 0.5), In),
        ((2.0 - h, 1.0, 0.5), In),
        // Outside, in band of a far carrier.
        ((6.0, 2.0 + h, 0.5), Out),
        ((-1.0, 2.0 + h, 0.5), Out),
        ((2.0 + h, 6.0, 0.5), Out),
        ((2.0 + h, -1.0, 0.5), Out),
        ((3.0, 2.0 + h, 7.0), Out),
        // In band of a far carrier AND of the top's carrier, foot of the
        // top inside the top: must refuse.
        ((1.0, 2.0 + h, 1.0 + h), Near),
        ((1.0, 2.0 + h, 1.0 - h), Near),
        // Over the y = 2 face itself.
        ((3.0, 2.0 + h, 0.5), Near),
        ((3.0, 2.0 - h, 0.5), Near),
        // The reflex vertical edge (2, 2), both sides, and its vertex.
        ((2.0 + h, 2.0 + h, 0.5), Near),
        ((2.0 - h, 2.0 - h, 0.5), Near),
        ((2.0 - h, 2.0 + h, 0.5), Near),
        ((2.0 + h, 2.0 - h, 0.5), Near),
        ((2.0 + h, 2.0 + h, 1.0 + h), Near),
        ((2.0 - h, 2.0 - h, 1.0 - h), Near),
        // The convex vertical edge (4, 2) and the y = 2 face's rim at
        // the reflex end, with the foot in band of the rim.
        ((4.0 + h, 2.0 + h, 0.5), Near),
        ((2.0 - h, 2.0 + h, 0.5), Near),
        ((4.0 - h, 2.0 + h, 0.5), Near),
        // Near the far rim of the top face at a far carrier's level.
        ((1.0, 4.0 + h, 1.0 + h), Near),
    ]
}

fn hole_probes(h: f64) -> Vec<((f64, f64, f64), Want)> {
    use Want::*;
    // [0, 2]³ with the hole [0.5, 1.5]² through z.
    vec![
        // In the hole at the top's and bottom's levels: outside.
        ((1.0, 1.0, 2.0 + h), Out),
        ((1.0, 1.0, 2.0 - h), Out),
        ((1.0, 1.0, h), Out),
        ((1.0, 1.0, -h), Out),
        ((0.75, 1.25, 2.0 - h), Out),
        // In the material under the top: refuse.
        ((0.25, 1.0, 2.0 - h), Near),
        ((1.75, 1.75, 2.0 + h), Near),
        // At the hole's rim.
        ((0.5 + h, 1.0, 2.0 + h), Near),
        ((0.5 - h, 1.0, 2.0 - h), Near),
        ((0.5 + h, 0.5 + h, 2.0 + h), Near),
        // In band of a hole wall's carrier, beyond the wall.
        ((0.5 + h, 0.25, 1.0), In),
        ((0.5 - h, 0.25, 1.0), In),
        ((0.5 + h, 1.75, 1.0), In),
        ((0.5 + h, 3.0, 1.0), Out),
        ((0.5 + h, -1.0, 1.0), Out),
        // In band of a hole wall's carrier, over the wall.
        ((0.5 + h, 1.0, 1.0), Near),
        ((0.5 - h, 1.0, 1.0), Near),
    ]
}

fn gaps() -> Vec<f64> {
    let eps = Tol::witness().get().eps;
    [0.5, 2.0, 5.0, 9.0, 20.0].iter().map(|k| k * eps).collect()
}

fn every_probe<T: Decide + topo::AtRestPolicy>(label: &str) -> usize {
    let mut wrong = Vec::new();
    let mut refused = 0;
    let l = l_prism::<T>();
    let holed = holed_block::<T>(2.0, &[1.0], Tol::witness());
    for h in gaps() {
        eprintln!("{label} h = {h:e}");
        let (w, r) = read(&l, &l_probes(h));
        wrong.extend(w.into_iter().map(|s| format!("{label} L h={h:e}: {s}")));
        refused += r;
        let (w, r) = read(&holed, &hole_probes(h));
        wrong.extend(w.into_iter().map(|s| format!("{label} holed h={h:e}: {s}")));
        refused += r;
    }
    assert!(wrong.is_empty(), "wrong answers:\n{}", wrong.join("\n"));
    refused
}

/// No probe near a boundary answers `In`/`Out`, no probe the band
/// leaves definite answers wrong — at f64.
#[test]
fn far_carrier_probes_never_answer_wrong_f64() {
    let refused = every_probe::<f64>("f64");
    eprintln!("f64: {refused} definite probes refused");
}

/// The same at `Interval`.
#[test]
fn far_carrier_probes_never_answer_wrong_interval() {
    let refused = every_probe::<Interval>("Interval");
    eprintln!("Interval: {refused} definite probes refused");
}

/// Strict form, f64: every definite probe decides at every gap.
#[test]
fn far_carrier_probes_all_decide_f64() {
    assert_eq!(every_probe::<f64>("f64-strict"), 0);
}

/// A boolean case: its name, the other operand, and the volume each of
/// union, subtract and intersect must carry (`None`: empty).
type Case = (&'static str, Body<f64>, [Option<f64>; 3]);

/// Booleans of the L-prism with a brick whose faces sit in band of
/// the L's far carriers (and whose carriers pass in band of the L's
/// vertices): nested, poking through the top, and disjoint. An answer
/// must carry the right volume; refusals are printed.
#[test]
fn booleans_beside_a_far_carrier_never_answer_wrong() {
    let tol = Tol::witness();
    let eps = tol.get().eps;
    let l = l_prism::<f64>();
    let mut wrong = Vec::new();
    let mut refused = Vec::new();
    for k in [2.0, 5.0, 9.0] {
        let d = k * eps;
        let cases: [Case; 3] = [
            (
                "nested",
                brick((1.0, 1.5), (1.0, 2.0 + d), (0.25, 0.75), tol),
                [
                    Some(12.0),
                    Some(12.0 - 0.25 * (1.0 + d)),
                    Some(0.25 * (1.0 + d)),
                ],
            ),
            (
                "through the top",
                brick((1.0, 1.5), (1.0, 2.0 + d), (0.5, 1.5), tol),
                [
                    Some(12.0 + 0.25 * (1.0 + d)),
                    Some(12.0 - 0.25 * (1.0 + d)),
                    Some(0.25 * (1.0 + d)),
                ],
            ),
            (
                "disjoint",
                brick((-3.0, -2.0), (2.0 + d, 3.0), (0.0, 1.0), tol),
                [Some(13.0 - d), Some(12.0), None],
            ),
        ];
        for (name, c, want) in cases {
            for (op, want) in [BooleanOp::Union, BooleanOp::Subtract, BooleanOp::Intersect]
                .into_iter()
                .zip(want)
            {
                let decls = topo::BooleanDeclarations::none();
                let got =
                    topo::boolean_op_with(op, &l, &c, &decls, topo::SweepStrategy::Realized, tol);
                match got {
                    Ok(r) => {
                        let v = r
                            .body()
                            .map(|bb| topo::mass_properties(&bb.body, tol).unwrap().volume);
                        let fine = match (v, want) {
                            (Some(v), Some(w)) => (v - w).abs() < 1e-9,
                            (None, None) => true,
                            _ => false,
                        };
                        eprintln!("k={k} {name} {op:?}: volume {v:?} (want {want:?})");
                        if !fine {
                            wrong.push(format!("k={k} {name} {op:?}: {v:?} want {want:?}"));
                        }
                    }
                    Err(e) => {
                        eprintln!("k={k} {name} {op:?}: REFUSED {e:?}");
                        refused.push(format!("k={k} {name} {op:?}"));
                    }
                }
            }
        }
    }
    eprintln!("refused: {refused:?}");
    assert!(wrong.is_empty(), "wrong answers:\n{}", wrong.join("\n"));
}

// ---- Fuzz against a closed-form oracle -------------------------------

fn seg_dist(p: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let t = (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
    ((p.0 - a.0 - t * dx).powi(2) + (p.1 - a.1 - t * dy).powi(2)).sqrt()
}

fn in_poly(p: (f64, f64), poly: &[(f64, f64)]) -> bool {
    let mut inside = false;
    for i in 0..poly.len() {
        let (a, b) = (poly[i], poly[(i + 1) % poly.len()]);
        if (a.1 > p.1) != (b.1 > p.1) && p.0 < a.0 + (p.1 - a.1) * (b.0 - a.0) / (b.1 - a.1) {
            inside = !inside;
        }
    }
    inside
}

/// (inside, distance to the boundary) for the prism `poly × [0, h]`.
fn oracle(q: (f64, f64, f64), poly: &[(f64, f64)], h: f64) -> (bool, f64) {
    let p = (q.0, q.1);
    let edge = (0..poly.len())
        .map(|i| seg_dist(p, poly[i], poly[(i + 1) % poly.len()]))
        .fold(f64::INFINITY, f64::min);
    let in_xy = in_poly(p, poly);
    let in_z = q.2 > 0.0 && q.2 < h;
    let dz = if in_z { 0.0 } else { (-q.2).max(q.2 - h) };
    if in_xy && in_z {
        (true, edge.min(q.2).min(h - q.2))
    } else {
        let dxy = if in_xy { 0.0 } else { edge };
        (false, (dxy * dxy + dz * dz).sqrt())
    }
}

/// Points sampled near the prism's carriers — on a random side
/// carrier or cap plane, anywhere along it out to twice the profile's
/// span, lifted off it by up to twice the band — and read against the
/// closed form. Any `In`/`Out` must be the oracle's; a point within the
/// band of the boundary must not answer `In`/`Out`.
fn fuzz<T: Decide + topo::AtRestPolicy>(poly: &[(f64, f64)], height: f64, n: usize, label: &str) {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let (eps, k) = (tol.get().eps, tol.get().k);
    let body = prism_z::<T>(poly, 0.0, height, tol).body;
    let mut rng = test_utils::fuzz::start(label);
    let n = test_utils::fuzz::scaled(n);
    let (mut wrong, mut decided, mut refused_far, mut far) = (Vec::new(), 0, 0, 0);
    let (lo, hi) = poly
        .iter()
        .fold(((f64::MAX, f64::MAX), (f64::MIN, f64::MIN)), |(l, h), p| {
            ((l.0.min(p.0), l.1.min(p.1)), (h.0.max(p.0), h.1.max(p.1)))
        });
    let span = (hi.0 - lo.0).max(hi.1 - lo.1);
    for _ in 0..n {
        let lift = rng.range(-2.0, 2.0) * k * eps;
        let pick = (rng.unit() * (poly.len() + 2) as f64) as usize;
        let q = if pick < poly.len() {
            // A side carrier: along the edge's line, any z.
            let (a, b) = (poly[pick], poly[(pick + 1) % poly.len()]);
            let len = ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt();
            let (ux, uy) = ((b.0 - a.0) / len, (b.1 - a.1) / len);
            let s = rng.range(-span, len + span);
            // Outward normal of a CCW profile edge: (uy, -ux).
            let (x, y) = (a.0 + ux * s + uy * lift, a.1 + uy * s - ux * lift);
            let z = match (rng.unit() * 4.0) as usize {
                0 => rng.range(-0.5, height + 0.5),
                1 => rng.range(-1.0, 1.0) * 2.0 * k * eps,
                2 => height + rng.range(-1.0, 1.0) * 2.0 * k * eps,
                _ => rng.range(0.0, height),
            };
            (x, y, z)
        } else {
            let z = if pick == poly.len() { 0.0 } else { height } + lift;
            (
                rng.range(lo.0 - span, hi.0 + span),
                rng.range(lo.1 - span, hi.1 + span),
                z,
            )
        };
        let (inside, dist) = oracle(q, poly, height);
        let p = Point3::new(T::from_f64(q.0), T::from_f64(q.1), T::from_f64(q.2));
        let got = point_in_solid(&body, p, band, tol);
        if dist > 2.0 * k * eps {
            far += 1;
        }
        match &got {
            Ok(SolidContainment::In | SolidContainment::Out) => {
                decided += 1;
                let said_in = matches!(got, Ok(SolidContainment::In));
                if dist < 0.9 * k * eps || said_in != inside {
                    wrong.push(format!(
                        "{q:?}: inside {inside}, dist {dist:e}, got {got:?}"
                    ));
                }
            }
            Ok(SolidContainment::OnBoundary) => {
                if dist > 1.1 * k * eps {
                    wrong.push(format!("{q:?}: dist {dist:e}, got OnBoundary"));
                }
            }
            Err(_) => {
                if dist > 2.0 * k * eps {
                    refused_far += 1;
                    if refused_far <= 5 {
                        eprintln!("  refused far probe {q:?} dist {dist:e}: {got:?}");
                    }
                }
            }
        }
    }
    eprintln!(
        "fuzz {label}: {n} probes, {decided} decided, {far} far, {refused_far} far refused, {} wrong",
        wrong.len()
    );
    assert!(
        wrong.is_empty(),
        "{label}: wrong answers ({}):\n{}",
        test_utils::fuzz::replay(),
        wrong.join("\n")
    );
}

fn rotated(poly: &[(f64, f64)], theta: f64) -> Vec<(f64, f64)> {
    let (s, c) = theta.sin_cos();
    poly.iter()
        .map(|&(x, y)| (c * x - s * y, s * x + c * y))
        .collect()
}

const L: [(f64, f64); 6] = [
    (0.0, 0.0),
    (4.0, 0.0),
    (4.0, 2.0),
    (2.0, 2.0),
    (2.0, 4.0),
    (0.0, 4.0),
];
const DART: [(f64, f64); 5] = [(0.0, 0.0), (4.0, 0.0), (3.0, 1.0), (4.0, 2.0), (0.0, 2.0)];
/// A comb: three teeth whose tips and roots share carriers far apart.
const COMB: [(f64, f64); 12] = [
    (0.0, 0.0),
    (7.0, 0.0),
    (7.0, 3.0),
    (6.0, 3.0),
    (6.0, 1.0),
    (4.0, 1.0),
    (4.0, 3.0),
    (3.0, 3.0),
    (3.0, 1.0),
    (1.0, 1.0),
    (1.0, 3.0),
    (0.0, 3.0),
];

#[test]
fn fuzz_axis_aligned_f64() {
    fuzz::<f64>(&L, 1.0, 3000, "L");
    fuzz::<f64>(&DART, 1.0, 3000, "dart");
    fuzz::<f64>(&COMB, 1.0, 3000, "comb");
}

#[test]
fn fuzz_rotated_f64() {
    fuzz::<f64>(&rotated(&L, 0.37), 1.0, 3000, "L turned");
    fuzz::<f64>(&rotated(&DART, 1.1), 1.0, 3000, "dart turned");
    fuzz::<f64>(&rotated(&COMB, 0.2), 0.7, 3000, "comb turned");
}

#[test]
fn fuzz_interval() {
    fuzz::<Interval>(&L, 1.0, 1000, "L interval");
    fuzz::<Interval>(&rotated(&COMB, 0.2), 0.7, 1000, "comb turned interval");
}

/// Points within twice the band of each corner of the prism, in all
/// three coordinates: the rim and vertex neighbourhoods, where a foot
/// lands in band of a face's loop.
fn corner_fuzz<T: Decide + topo::AtRestPolicy>(poly: &[(f64, f64)], height: f64, label: &str) {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let (eps, k) = (tol.get().eps, tol.get().k);
    let body = prism_z::<T>(poly, 0.0, height, tol).body;
    let mut rng = test_utils::fuzz::start(label);
    let mut wrong = Vec::new();
    let (mut decided, mut n) = (0, 0);
    for &(x, y) in poly {
        for z in [0.0, height] {
            for _ in 0..test_utils::fuzz::scaled(60) {
                let r = 2.0 * k * eps;
                let q = (
                    x + rng.range(-r, r),
                    y + rng.range(-r, r),
                    z + rng.range(-r, r),
                );
                let (inside, dist) = oracle(q, poly, height);
                let p = Point3::new(T::from_f64(q.0), T::from_f64(q.1), T::from_f64(q.2));
                let got = point_in_solid(&body, p, band, tol);
                n += 1;
                if let Ok(SolidContainment::In | SolidContainment::Out) = got {
                    decided += 1;
                    let said_in = matches!(got, Ok(SolidContainment::In));
                    if dist < 0.9 * k * eps || said_in != inside {
                        wrong.push(format!(
                            "{q:?}: inside {inside}, dist {dist:e}, got {got:?}"
                        ));
                    }
                }
            }
        }
    }
    eprintln!(
        "corner fuzz {label}: {n} probes, {decided} decided, {} wrong",
        wrong.len()
    );
    assert!(
        wrong.is_empty(),
        "{label}: wrong answers ({}):\n{}",
        test_utils::fuzz::replay(),
        wrong.join("\n")
    );
}

#[test]
fn corner_fuzz_f64_and_interval() {
    corner_fuzz::<f64>(&L, 1.0, "L corners");
    corner_fuzz::<f64>(&rotated(&COMB, 0.2), 0.7, "comb corners");
    corner_fuzz::<f64>(&rotated(&DART, 1.1), 1.0, "dart corners");
    corner_fuzz::<Interval>(&rotated(&L, 0.37), 1.0, "L corners interval");
}

/// A ray parallel to a face's carrier within the band, from a point in
/// band of that carrier whose foot is outside the face, may still meet
/// the face: here the +x ray from q runs `0.15ε` above the carrier of
/// the edge (0, 1) → (1, 1 + m), `m = 0.3ε`, and crosses that face at
/// x ≈ 0.5, where it enters the material. Skipping the face as
/// "parallel" read the exit through the next face and answered `In`; q
/// is outside, `500ε` above the edge (−1, 1 − 1000ε) → (0, 1). Both
/// lanes; the pose scales with ε, so it reads the same at every row.
#[test]
fn a_ray_along_a_carrier_q_is_in_band_of_is_not_skipped() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let eps = tol.get().eps;
    let (m, phi, e) = (0.3 * eps, 1000.0 * eps, 0.15 * eps);
    let profile = [
        (-1.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0 + m - phi),
        (1.0, 1.0 + m),
        (0.0, 1.0),
        (-1.0, 1.0 - phi),
    ];
    fn read<T: Decide + topo::AtRestPolicy>(
        profile: &[(f64, f64)],
        q: (f64, f64, f64),
        band: Band,
        tol: Tol,
    ) -> Result<SolidContainment, topo::PointInSolidError> {
        let body: Body<T> = prism_z::<T>(profile, 0.0, 1.0, tol).body;
        let q = Point3::new(T::from_f64(q.0), T::from_f64(q.1), T::from_f64(q.2));
        point_in_solid(&body, q, band, tol)
    }
    let q = (-0.5, 1.0 + e, 0.5);
    let f = read::<f64>(&profile, q, band, tol);
    let i = read::<Interval>(&profile, q, band, tol);
    assert!(
        matches!(f, Ok(SolidContainment::Out)),
        "f64: q is outside, got {f:?}"
    );
    assert!(
        matches!(i, Ok(SolidContainment::Out)),
        "Interval: q is outside, got {i:?}"
    );
}
