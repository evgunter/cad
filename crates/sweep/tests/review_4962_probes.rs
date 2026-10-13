//! **Review probes, PR 4962** (the fillet's turn overrun, step 3): an
//! oracle derived apart from the lane's (each band's section from a
//! solved spine and its own Cartesian quadrature with exact chord
//! limits, the overlap integrated along a generic direction rather than
//! over band 1's section), a membership grid of the built body against
//! the model `B ∖ (W₁ ∪ W₂)`, the `CapEllipse` window walked in the
//! lean, feature rows on the fillet's overrun face, and the overrun
//! arc's direction at `Interval`.
//!
//! P1 and P3 cost ~5–6 min each in the debug profile (the oracle's
//! nested quadrature); the oracle reads the box corner's hand form to
//! `5e-10`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Bounds, Interval, Point3};
use sweep::blend::build::{Blended, fillet_edges};
use sweep::blend::{BlendDecision, BlendError};
use sweep::test_support::at_rest;
use topo::{
    Body, ContactRecords, EdgeKey, SolidContainment, mass_properties, point_in_solid,
    validate_geometric, validate_pseudomanifold,
};

use crate::band_planar_cut_off::{D, edge, tol};
use crate::band_turn_overrun_chamfer::{Half, ends, halves_of, placed, point_at, witnesses};
use crate::band_turn_overrun_oracles::{a_clearance, non_trivalent, pinned};
use crate::band_turn_overrun_probe::{V, Verb, add, cross, dot, mul, norm, sub, unit};
use crate::common::operands::{leaning_turn, leaning_turn_at, parallelepiped};

// ------------------------------------------------------------------
// The independent oracle.
// ------------------------------------------------------------------

/// One band: spine point `o` (on the section plane through edge end
/// `a`), direction `tau`, outward face normals and offsets, the kite's
/// corners and radius.
struct Band {
    tau: V,
    o: V,
    r: f64,
    faces: [Half; 2],
    /// The kite `a, Tₐ, O, T_b` in order.
    kite: [V; 4],
}

/// Solve the 3×3 system `rows · x = rhs` by Cramer's rule.
fn solve(rows: [V; 3], rhs: V) -> V {
    let det = dot(rows[0], cross(rows[1], rows[2]));
    let col = |i: usize| {
        let mut m = rows;
        for (k, row) in m.iter_mut().enumerate() {
            row[i] = rhs[k];
        }
        dot(m[0], cross(m[1], m[2])) / det
    };
    [col(0), col(1), col(2)]
}

/// The band over edge `a → b` of the convex polytope `halves` at `r`.
fn band(halves: &[Half], a: V, b: V, r: f64) -> Band {
    let on = |h: &Half, p: V| (dot(h.0, p) - h.1).abs() < 1e-9;
    let faces: Vec<Half> = halves
        .iter()
        .filter(|h| on(h, a) && on(h, b))
        .copied()
        .collect();
    assert_eq!(faces.len(), 2, "an edge on two faces");
    let tau = unit(sub(b, a));
    // The spine: `r` inside both faces, in the section plane through `a`.
    let o = solve(
        [faces[0].0, faces[1].0, tau],
        [faces[0].1 - r, faces[1].1 - r, dot(tau, a)],
    );
    let ta = add(o, mul(faces[0].0, r));
    let tb = add(o, mul(faces[1].0, r));
    Band {
        tau,
        o,
        r,
        faces: [faces[0], faces[1]],
        kite: [a, ta, o, tb],
    }
}

/// `t`-intervals, sorted and disjoint.
type Set = Vec<(f64, f64)>;

fn meet(a: &Set, b: &Set) -> Set {
    let mut out = Vec::new();
    for &(a0, a1) in a {
        for &(b0, b1) in b {
            let (lo, hi) = (a0.max(b0), a1.min(b1));
            if hi > lo {
                out.push((lo, hi));
            }
        }
    }
    out.sort_by(|x, y| x.0.total_cmp(&y.0));
    out
}

fn measure_set(s: &Set) -> f64 {
    s.iter().map(|(a, b)| b - a).sum()
}

/// The half-space `n·x ≤ c` along `x + t·g`, as an interval.
fn half_line(n: V, c: f64, x: V, g: V) -> (f64, f64) {
    let (along, at) = (dot(n, g), c - dot(n, x));
    if along.abs() < 1e-300 {
        if at >= 0.0 {
            (f64::NEG_INFINITY, f64::INFINITY)
        } else {
            (0.0, 0.0)
        }
    } else if along > 0.0 {
        (f64::NEG_INFINITY, at / along)
    } else {
        (at / along, f64::INFINITY)
    }
}

/// `B` along the line, as a set.
fn poly_line(halves: &[Half], x: V, g: V) -> Set {
    let (mut lo, mut hi) = (f64::NEG_INFINITY, f64::INFINITY);
    for h in halves {
        let (a, b) = half_line(h.0, h.1, x, g);
        lo = lo.max(a);
        hi = hi.min(b);
    }
    if hi > lo { vec![(lo, hi)] } else { Vec::new() }
}

/// The kite's half-planes, each `n·x ≤ c` (`n ⊥ τ`).
fn kite_halves(b: &Band) -> Vec<Half> {
    let mid = mul(b.kite.iter().fold([0.0; 3], |s, p| add(s, *p)), 0.25);
    (0..4)
        .map(|i| {
            let (p, q) = (b.kite[i], b.kite[(i + 1) % 4]);
            let n = unit(cross(sub(q, p), b.tau));
            let n = if dot(n, sub(mid, p)) > 0.0 {
                mul(n, -1.0)
            } else {
                n
            };
            (n, dot(n, p))
        })
        .collect()
}

/// `W` along the line: the kite prism less the cylinder.
fn band_line(b: &Band, kh: &[Half], x: V, g: V) -> Set {
    let prism = poly_line(kh, x, g);
    if prism.is_empty() {
        return prism;
    }
    let w = sub(x, b.o);
    let (gp, wp) = (
        sub(g, mul(b.tau, dot(g, b.tau))),
        sub(w, mul(b.tau, dot(w, b.tau))),
    );
    let (qa, qb, qc) = (dot(gp, gp), 2.0 * dot(gp, wp), dot(wp, wp) - b.r * b.r);
    let disc = qb * qb - 4.0 * qa * qc;
    if qa < 1e-300 || disc <= 0.0 {
        return if qa < 1e-300 && qc < 0.0 {
            Vec::new()
        } else {
            prism
        };
    }
    let root = disc.sqrt();
    let (c0, c1) = ((-qb - root) / (2.0 * qa), (-qb + root) / (2.0 * qa));
    let outside = vec![(f64::NEG_INFINITY, c0), (c1, f64::INFINITY)];
    meet(&prism, &outside)
}

/// Adaptive Gauss–Legendre (10 points), halved where the halves'
/// sum disagrees with the whole.
fn quad(f: &dyn Fn(f64) -> f64, a: f64, b: f64, tol: f64, depth: u32) -> f64 {
    const X: [f64; 5] = [
        0.148_874_338_981_631_2,
        0.433_395_394_129_247_2,
        0.679_409_568_299_024_4,
        0.865_063_366_688_984_5,
        0.973_906_528_517_171_7,
    ];
    const W: [f64; 5] = [
        0.295_524_224_714_752_9,
        0.269_266_719_309_996_4,
        0.219_086_362_515_982_04,
        0.149_451_349_150_580_6,
        0.066_671_344_308_688_14,
    ];
    let gl = |a: f64, b: f64| {
        let (c, h) = ((a + b) / 2.0, (b - a) / 2.0);
        h * (0..5)
            .map(|i| W[i] * (f(c + h * X[i]) + f(c - h * X[i])))
            .sum::<f64>()
    };
    fn go(gl: &dyn Fn(f64, f64) -> f64, a: f64, b: f64, whole: f64, tol: f64, depth: u32) -> f64 {
        let c = (a + b) / 2.0;
        let (l, r) = (gl(a, c), gl(c, b));
        if depth == 0 || (l + r - whole).abs() < tol.max(1e-14 * (l + r).abs()) {
            l + r
        } else {
            go(gl, a, c, l, (tol / 2.0).max(1e-17), depth - 1)
                + go(gl, c, b, r, (tol / 2.0).max(1e-17), depth - 1)
        }
    }
    go(&gl, a, b, gl(a, b), tol, depth)
}

/// `vol(B ∩ W)` for one band: over the section plane through `a` in
/// Cartesian `(p, q)`, the region's exact `q`-chords at each `p`, the
/// length along `τ` read off every face of `B`.
fn band_vol(halves: &[Half], b: &Band) -> f64 {
    let a = b.kite[0];
    let e1 = unit(sub(b.kite[1], a));
    let e2 = cross(b.tau, e1);
    let pq = |x: V| (dot(sub(x, a), e1), dot(sub(x, a), e2));
    let corners: Vec<(f64, f64)> = b.kite.iter().map(|x| pq(*x)).collect();
    let (op, oq) = pq(b.o);
    // The kite's `q`-range at `p`, from its four sides.
    let chords = |p: f64| -> Set {
        let mut qs: Vec<f64> = Vec::new();
        for i in 0..4 {
            let ((p0, q0), (p1, q1)) = (corners[i], corners[(i + 1) % 4]);
            if (p1 - p0).abs() > 1e-300 && (p - p0) * (p - p1) <= 0.0 {
                qs.push(q0 + (q1 - q0) * (p - p0) / (p1 - p0));
            }
        }
        if qs.len() < 2 {
            return Vec::new();
        }
        let (lo, hi) = qs
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(l, h), q| {
                (l.min(*q), h.max(*q))
            });
        let kite = vec![(lo, hi)];
        let dp = p - op;
        if dp.abs() >= b.r {
            return kite;
        }
        let h = (b.r * b.r - dp * dp).sqrt();
        meet(
            &kite,
            &vec![(f64::NEG_INFINITY, oq - h), (oq + h, f64::INFINITY)],
        )
    };
    let len = |p: f64, q: f64| {
        let x = add(a, add(mul(e1, p), mul(e2, q)));
        measure_set(&poly_line(halves, x, b.tau))
    };
    let (p0, p1) = corners
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(l, h), c| {
            (l.min(c.0), h.max(c.0))
        });
    let tol = 1e-13;
    let inner = |p: f64| {
        chords(p)
            .iter()
            .map(|&(q0, q1)| quad(&|q| len(p, q), q0, q1, tol, 8))
            .sum::<f64>()
    };
    // Break the outer integral at every corner's and the disk's `p`.
    let mut breaks: Vec<f64> = corners.iter().map(|c| c.0).collect();
    breaks.extend([op - b.r, op + b.r, p0, p1]);
    breaks.retain(|x| *x >= p0 && *x <= p1);
    breaks.sort_by(f64::total_cmp);
    breaks.dedup_by(|x, y| (*x - *y).abs() < 1e-15);
    breaks
        .windows(2)
        .map(|w| quad(&inner, w[0], w[1], tol, 22))
        .sum()
}

/// `vol(B ∩ W₁ ∩ W₂)`, along a generic direction over a box about the
/// shared vertex `v`.
fn overlap_vol(halves: &[Half], b1: &Band, b2: &Band, v: V) -> f64 {
    let g = unit(add(
        add(b1.tau, mul(b2.tau, 0.37)),
        mul(unit(cross(b1.tau, b2.tau)), 0.61),
    ));
    let seed = if g[0].abs() < 0.9 {
        [1.0, 0.0, 0.0]
    } else {
        [0.0, 1.0, 0.0]
    };
    let e1 = unit(cross(g, seed));
    let e2 = cross(g, e1);
    let (k1, k2) = (kite_halves(b1), kite_halves(b2));
    let reach = 2.5 * norm(sub(b1.o, b1.kite[0])).max(norm(sub(b2.o, b2.kite[0])));
    let len = |u: f64, w: f64| {
        let x = add(v, add(mul(e1, u), mul(e2, w)));
        let s = meet(
            &poly_line(halves, x, g),
            &meet(&band_line(b1, &k1, x, g), &band_line(b2, &k2, x, g)),
        );
        measure_set(&s)
    };
    let tol = 1e-11;
    // The overlap must die before the box's edge.
    for i in 0..64 {
        let t = -reach + 2.0 * reach * f64::from(i) / 64.0;
        for (u, w) in [(reach, t), (-reach, t), (t, reach), (t, -reach)] {
            assert!(
                len(u, w) == 0.0,
                "the overlap reaches the box's edge at ({u}, {w})"
            );
        }
    }
    let split = |f: &dyn Fn(f64) -> f64| {
        let n = 24;
        (0..n)
            .map(|i| {
                let a = -reach + 2.0 * reach * f64::from(i) / f64::from(n);
                let b = -reach + 2.0 * reach * f64::from(i + 1) / f64::from(n);
                quad(f, a, b, tol / f64::from(n), 22)
            })
            .sum::<f64>()
    };
    split(&|u| split(&|w| len(u, w)))
}

/// `vol(B ∩ (W₁ ∪ W₂))` for a turn of two edges sharing `v`.
fn removed(halves: &[Half], edges: [(V, V); 2], r: f64) -> f64 {
    let bands = edges.map(|(a, b)| band(halves, a, b, r));
    let shared = {
        let [(a1, b1), (a2, b2)] = edges;
        [a1, b1]
            .into_iter()
            .find(|p| norm(sub(*p, a2)) < 1e-9 || norm(sub(*p, b2)) < 1e-9)
            .expect("a shared vertex")
    };
    band_vol(halves, &bands[0]) + band_vol(halves, &bands[1])
        - overlap_vol(halves, &bands[0], &bands[1], shared)
}

/// Whether `x` is in `B ∖ (W₁ ∪ W₂)`, and how far it stands from every
/// constraint that decides it.
fn kept(halves: &[Half], bands: &[Band], x: V) -> (bool, f64) {
    let mut margin = f64::INFINITY;
    let mut in_b = true;
    for h in halves {
        let s = h.1 - dot(h.0, x);
        margin = margin.min(s.abs());
        in_b &= s > 0.0;
    }
    let mut taken = false;
    for b in bands {
        let mut in_prism = true;
        for (n, c) in kite_halves(b).into_iter().chain(b.faces) {
            let s = c - dot(n, x);
            margin = margin.min(s.abs());
            in_prism &= s > 0.0;
        }
        let w = sub(x, b.o);
        let d = norm(sub(w, mul(b.tau, dot(w, b.tau)))) - b.r;
        margin = margin.min(d.abs());
        taken |= in_prism && d > 0.0;
    }
    (in_b && !taken, margin)
}

// ------------------------------------------------------------------
// Builds.
// ------------------------------------------------------------------

fn build(
    body: &Body<f64>,
    edges: &[EdgeKey],
    r: f64,
    what: &str,
) -> Result<Blended<f64>, BlendError> {
    let out = fillet_edges(&at_rest(body, tol()), edges, r, tol()).map_err(|e| e.error)?;
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, {e:?}"));
    validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
        .unwrap_or_else(|e| panic!("{what}: tier 3′, {e:?}"));
    Ok(out)
}

fn vol(body: &Body<f64>) -> (f64, f64) {
    let p = mass_properties(body, tol()).expect("props");
    (p.volume, p.volume_pad)
}

fn edge_points(body: &Body<f64>, e: EdgeKey) -> (V, V) {
    let [a, b] = ends(body, e).map(|v| point_at(body, v));
    (a, b)
}

/// The fillet witnesses: the step-2 set and the sheared box's
/// supplementary corner.
fn all_witnesses() -> Vec<(&'static str, Body<f64>, [EdgeKey; 2])> {
    let mut all = witnesses();
    let s = 0.3;
    let body = parallelepiped(s);
    let v = [2.0 + s, s, 1.0];
    let edges = [
        edge(&body, v, [s, s, 1.0]),
        edge(&body, v, [2.0 + s, 1.5 + s, 1.0]),
    ];
    all.push(("sheared box, supplementary", body, edges));
    all
}

// ------------------------------------------------------------------
// The rows.
// ------------------------------------------------------------------

/// **P1.** The independent oracle reads the box corner's hand form, and
/// every witness's built `ΔV` midpoint against it, both orders.
#[test]
fn p1_an_independent_oracle_reads_every_witness() {
    // The hand form at a box corner, edges of 2 and 1.5.
    let box_halves: Vec<Half> = vec![
        ([0., 0., -1.], 0.0),
        ([1., 0., 0.], 2.0),
        ([0., 0., 1.], 1.0),
        ([-1., 0., 0.], 0.0),
        ([0., 1., 0.], 0.0),
        ([0., -1., 0.], 1.5),
    ];
    let r = 0.1;
    let want = (1.0 - core::f64::consts::PI / 4.0) * r * r * 3.5
        - (5.0 / 3.0 - core::f64::consts::PI / 2.0) * r.powi(3);
    let got = removed(
        &box_halves,
        [([0., 0., 1.], [2., 0., 1.]), ([0., 0., 1.], [0., -1.5, 1.])],
        r,
    );
    eprintln!(
        "P1 box hand form {want:.15e} mine {got:.15e} diff {:e}",
        got - want
    );
    assert!(
        (got - want).abs() < 2e-9,
        "the box corner: {got} against {want}"
    );
    let mut report = Vec::new();
    for (what, body, edges) in all_witnesses() {
        let halves = halves_of(&body);
        let mine = removed(&halves, edges.map(|e| edge_points(&body, e)), D);
        for order in [[0, 1], [1, 0]] {
            let ord = [edges[order[0]], edges[order[1]]];
            let out = build(&body, &ord, D, what).unwrap_or_else(|e| panic!("{what}: {e}"));
            assert_eq!(non_trivalent(&out.body), 0, "{what}: trivalent");
            let (v0, p0) = vol(&body);
            let (v1, p1) = vol(&out.body);
            let d = v0 - v1 - mine;
            eprintln!(
                "P1 {what} order {order:?}: ΔV {:.12e} mine {mine:.12e} diff {d:e} pad {:e}",
                v0 - v1,
                p0 + p1
            );
            report.push((what, d, p0 + p1));
        }
    }
    for (what, d, pad) in report {
        assert!(
            d.abs() < 2e-9_f64.max(pad),
            "{what}: ΔV off my oracle by {d:e} (pad {pad:e})"
        );
    }
}

/// **P2.** Point membership of the built body over a grid about the
/// turn vertex agrees with `B ∖ (W₁ ∪ W₂)` wherever the model's
/// deciding constraints stand clear.
#[test]
fn p2_the_built_body_is_the_model_point_by_point() {
    let band = geom_core::Band::linear(tol()).expect("band");
    let mut wrong = Vec::new();
    let mut checked = 0;
    for (what, body, edges) in all_witnesses() {
        let halves = halves_of(&body);
        let pts = edges.map(|e| edge_points(&body, e));
        let bands: Vec<Band> = pts
            .iter()
            .map(|(a, b)| self::band(&halves, *a, *b, D))
            .collect();
        let [(a1, b1), (a2, b2)] = pts;
        let v = [a1, b1]
            .into_iter()
            .find(|p| norm(sub(*p, a2)) < 1e-9 || norm(sub(*p, b2)) < 1e-9)
            .unwrap();
        let out = build(&body, &edges, D, what).unwrap();
        let n = 11;
        let h = 2.5 * D;
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    let s = |m: usize| -h + 2.0 * h * (m as f64 + 0.5) / n as f64;
                    let x = add(v, [s(i), s(j), s(k)]);
                    let (want, margin) = kept(&halves, &bands, x);
                    if margin < 1e-6 {
                        continue;
                    }
                    checked += 1;
                    match point_in_solid(&out.body, Point3::new(x[0], x[1], x[2]), band, tol()) {
                        Ok(SolidContainment::In) if want => {}
                        Ok(SolidContainment::Out) if !want => {}
                        got => wrong.push(format!("{what} {x:?}: {got:?}, model kept {want}")),
                    }
                }
            }
        }
    }
    eprintln!("P2 checked {checked}, wrong {}", wrong.len());
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// **P3.** The `CapEllipse` window: the leaning prism walked in its
/// lean from a box corner. Every build is valid and at the oracle; the
/// outcomes run turn readings → `CapEllipse` → build, monotone.
#[test]
fn p3_the_cap_ellipse_window_is_monotone_and_where_the_closed_form_puts_it() {
    let band = geom_core::Band::linear(tol()).expect("band");
    let esc = band.escalate();
    let edge_deg = (1.0 / (1.0 + esc / D)).acos().to_degrees();
    eprintln!(
        "P3 zero {:e} escalate {esc:e}: closed-form edge {edge_deg:e}° (√(2e/r) {:e}°)",
        band.zero(),
        (2.0 * esc / D).sqrt().to_degrees()
    );
    let mut seq = Vec::new();
    let mut bad = Vec::new();
    for sign in [1.0, -1.0] {
        // 0 mitre, 1 the turn's readings escalate, 2 the cut-off's
        // section escalates, 3 the overrun builds.
        let mut stage = 0;
        for deg in [
            1e-7, 1e-6, 3e-6, 1e-5, 3e-5, 1e-4, 1e-3, 0.01, 0.02, 0.024, 0.025, 0.0253, 0.0256,
            0.0259, 0.0262, 0.027, 0.03, 0.05, 0.1,
        ] {
            let s = sign * f64::to_radians(deg).tan();
            let (body, edges) = leaning_turn(s);
            let what = format!("lean {sign}·{deg}°");
            let got = match build(&body, &edges, D, &what) {
                Ok(out) => {
                    let halves = halves_of(&body);
                    let (v0, p0) = vol(&body);
                    let (v1, p1) = vol(&out.body);
                    // The oracle is costly: read it where the overrun
                    // starts, on either side.
                    let d = if [0.0259, 1e-5, 3e-5].contains(&deg) {
                        v0 - v1 - removed(&halves, edges.map(|e| edge_points(&body, e)), D)
                    } else {
                        0.0
                    };
                    if d.abs() > 2e-9_f64.max(p0 + p1) {
                        bad.push(format!("{what}: ΔV off my oracle by {d:e}"));
                    }
                    let mitre = out
                        .coincidences
                        .iter()
                        .any(|r| r.relation == topo::Relation::CommonFoot);
                    if mitre {
                        (0, format!("mitre (ΔV−oracle {d:e})"))
                    } else {
                        (
                            3,
                            format!(
                                "overrun (ΔV−oracle {d:e}, trivalent-off {})",
                                non_trivalent(&out.body)
                            ),
                        )
                    }
                }
                Err(BlendError::Escalated { decision, .. }) => match decision {
                    BlendDecision::CapEllipse | BlendDecision::CapTransverse => {
                        (2, format!("{decision:?}"))
                    }
                    _ => (1, format!("{decision:?}")),
                },
                Err(e) => {
                    bad.push(format!("{what}: refused {e}"));
                    (9, format!("refused {e}"))
                }
            };
            seq.push(format!("{what}: {}", got.1));
            if got.0 < stage {
                bad.push(format!("{what}: {} after stage {stage}", got.1));
            }
            stage = stage.max(got.0);
            if sign > 0.0 && deg > 1e-3 {
                let want = if deg < edge_deg * 0.99 {
                    2
                } else if deg > edge_deg * 1.01 {
                    3
                } else {
                    got.0
                };
                if got.0 != want {
                    bad.push(format!(
                        "{what}: {} against the closed-form edge {edge_deg}°",
                        got.1
                    ));
                }
            }
        }
    }
    eprintln!("P3\n{}", seq.join("\n"));
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **P4.** Features on the fillet's overrun face (the front face at the
/// bracket's `y = 0` corner): the far band's circle about
/// `c = (−1 + (1 + √2)·d, 1 − d)`, `q = (−1 + √2·d, 1 − d)`, `L` on
/// `x + z = 0`.
#[test]
fn p4_features_about_the_fillet_overrun_arc() {
    let (plain, plain_edges) = leaning_turn(-1.0);
    let free = {
        let out = build(&plain, &plain_edges, D, "free").unwrap();
        vol(&plain).0 - vol(&out.body).0
    };
    let c = [-1.0 + (1.0 + 2f64.sqrt()) * D, 1.0 - D];
    let polar = |deg: f64, rho: f64| {
        let t = deg.to_radians();
        [c[0] + rho * t.cos(), 0.0, c[1] + rho * t.sin()]
    };
    let n = [0.0, 1.0, 0.0];
    // (what, centre, hole depth, builds)
    let rows: Vec<(&str, V, Option<f64>, bool)> = vec![
        (
            "boss inside the circle by the arc's middle",
            polar(202.5, 0.088),
            None,
            true,
        ),
        (
            "hole inside the circle by the arc's middle",
            polar(202.5, 0.088),
            Some(0.002),
            true,
        ),
        (
            "boss inside the circle below q",
            polar(185.0, 0.09),
            None,
            true,
        ),
        (
            "boss right of the circle below the trimline",
            [-0.64, 0.0, 0.86],
            None,
            true,
        ),
        ("boss straddling the arc", polar(202.5, 0.1), None, false),
        (
            "hole straddling the arc",
            polar(202.5, 0.1),
            Some(0.002),
            false,
        ),
        (
            "boss in the overrun's cusp by p_k",
            [-0.873, 0.0, 0.885],
            None,
            false,
        ),
        (
            "hole in the overrun's cusp by p_k",
            [-0.873, 0.0, 0.885],
            Some(0.002),
            false,
        ),
        (
            "boss just outside the arc, in the overrun",
            polar(210.0, 0.106),
            None,
            false,
        ),
    ];
    let mut out = Vec::new();
    for (what, at, hole, builds) in rows {
        let (body, edges) = pinned(at, n, hole.is_none(), hole.unwrap_or(0.0));
        let got = build(&body, &edges, D, what);
        let line = match (&got, builds) {
            (Ok(b), true) => {
                let d = vol(&body).0 - vol(&b.body).0 - free;
                if d.abs() > 1e-8 {
                    format!("FAIL {what}: built, ΔV off the feature-free by {d:e}")
                } else {
                    format!("ok {what}: builds at the feature-free ΔV ({d:e})")
                }
            }
            (Ok(_), false) => format!("FAIL {what}: built where the fillet cuts it"),
            (Err(e), true) => format!("FAIL? {what}: refused where clear: {e}"),
            (Err(e), false) if a_clearance(e) => format!("ok {what}: refuses {e}"),
            (Err(e), false) => format!("FAIL {what}: refuses, not at a clearance meter: {e}"),
        };
        out.push(line);
    }
    eprintln!("P4\n{}", out.join("\n"));
    let hard: Vec<&String> = out.iter().filter(|l| l.starts_with("FAIL ")).collect();
    assert!(hard.is_empty(), "{hard:?}");
}

/// **P5.** At `Interval` the overrun arc runs the same short way as at
/// `f64`: its window's midpoint is the `f64` arc's.
#[test]
fn p5_the_interval_overrun_arc_runs_the_f64_way() {
    use crate::common::interval::iv;
    for lean in [0.5, -0.5, 1.0, -1.0] {
        let what = format!("lean {lean}");
        let (fbody, fedges) = leaning_turn(lean);
        let fout = build(&fbody, &fedges, D, &what).unwrap();
        let (k, p_k, q) = placed(&fbody, fedges, Verb::Fillet);
        let mid_of = |c: &geom::Curve3<f64>, t0: f64, t1: f64| {
            let p = c.eval(0.5 * (t0 + t1));
            [p.x, p.y, p.z]
        };
        let farc = arc_between(&fout.body, p_k, q).expect("the f64 overrun arc");
        let fmid = {
            let g = fout
                .body
                .get_curve_geom(fout.body.get_edge(farc).unwrap().curve)
                .unwrap();
            let g = g.certified().unwrap();
            let (t0, t1) = g.params();
            mid_of(g.carrier(), t0, t1)
        };
        let (ibody, iedges) = leaning_turn_at::<Interval>(lean);
        let ibody = sweep::test_support::finished("operand", ibody, tol());
        let iout = fillet_edges(&ibody, &iedges, iv(D), tol())
            .unwrap_or_else(|e| panic!("{what}: builds at Interval, {:?}", e.error));
        // The Interval arc: the edge whose ends sit at p_k and q.
        let b = &iout.body;
        let at = |v: topo::VertexKey| {
            let p = b.get_point(b.get_vertex(v).unwrap().point).unwrap();
            [
                0.5 * (p.x.lo() + p.x.hi()),
                0.5 * (p.y.lo() + p.y.hi()),
                0.5 * (p.z.lo() + p.z.hi()),
            ]
        };
        let mut found = 0;
        for e in topo::query::all_edges(b) {
            let he = b.get_edge(e).unwrap().he_plus;
            let s = at(b.get_half_edge(he).unwrap().start);
            let t = at(b.half_edge_end(he).unwrap());
            let hit = |x: V, y: V| norm(sub(x, p_k)) < 1e-9 && norm(sub(y, q)) < 1e-9;
            if !(hit(s, t) || hit(t, s)) {
                continue;
            }
            found += 1;
            let g = b.get_curve_geom(b.get_edge(e).unwrap().curve).unwrap();
            let g = g.certified().unwrap();
            let (t0, t1) = g.params();
            let p = g.carrier().eval((t0 + t1) * iv(0.5));
            let imid = [
                0.5 * (p.x.lo() + p.x.hi()),
                0.5 * (p.y.lo() + p.y.hi()),
                0.5 * (p.z.lo() + p.z.hi()),
            ];
            assert!(
                norm(sub(imid, fmid)) < 1e-9,
                "{what}: Interval arc mid {imid:?} against f64 {fmid:?} (far band {k})"
            );
        }
        assert_eq!(found, 1, "{what}: one Interval overrun edge");
        eprintln!("P5 {what}: far band {k}, mid {fmid:?}");
    }
}

fn arc_between(body: &Body<f64>, p: V, q: V) -> Option<EdgeKey> {
    topo::query::all_edges(body).into_iter().find(|e| {
        let (a, b) = edge_points(body, *e);
        (norm(sub(a, p)) < 1e-9 && norm(sub(b, q)) < 1e-9)
            || (norm(sub(a, q)) < 1e-9 && norm(sub(b, p)) < 1e-9)
    })
}

/// **P6.** The turn recourse's legs followed from a near-box fillet turn
/// in the turn's band, at a lean that tilts the far band's cut-off face
/// (`+`): a larger setback, and a slightly more asymmetric corner.
#[test]
fn p6_the_turn_recourse_from_a_near_box_fillet_turn() {
    let mut seq = Vec::new();
    let classify = |deg: f64, r: f64| {
        let (body, edges) = leaning_turn(f64::to_radians(deg).tan());
        match fillet_edges(&at_rest(&body, tol()), &edges, r, tol()) {
            Ok(out) => {
                let mitre = out
                    .coincidences
                    .iter()
                    .any(|c| c.relation == topo::Relation::CommonFoot);
                if mitre {
                    "mitre".to_string()
                } else {
                    "overrun".to_string()
                }
            }
            Err(e) => match e.error {
                BlendError::Escalated { decision, .. } => format!("escalated {decision:?}"),
                other => format!("refused {other}"),
            },
        }
    };
    let start = classify(3e-6, D);
    seq.push(format!("lean 3e-6°, r = d: {start}"));
    for r in [0.2, 0.3, 0.45] {
        seq.push(format!("lean 3e-6°, r = {r}: {}", classify(3e-6, r)));
    }
    for deg in [1e-4, 1e-3, 0.01, 0.03] {
        seq.push(format!("lean {deg}°, r = d: {}", classify(deg, D)));
    }
    eprintln!("P6\n{}", seq.join("\n"));
    assert!(
        start.contains("Turn"),
        "the start is the turn's own escalation: {start}"
    );
}

/// **P7.** Concave: the skewed box as a void in a block. The fill adds
/// `vol(A ∩ (W₁ ∪ W₂))` of the air `A` by the independent oracle, and
/// membership about the turn agrees with `A ∖ (W₁ ∪ W₂)` being air.
#[test]
fn p7_the_concave_skewed_box_fills_the_model() {
    let s = 0.3;
    let convex = parallelepiped(s);
    let block =
        crate::common::cavity::brick(Point3::new(-1.0, -1.0, -1.0), Point3::new(4.0, 3.0, 2.0));
    let pocket =
        sweep::test_support::realized(topo::boolean::BooleanOp::Subtract, &block, &convex, tol());
    let v = [s, s, 1.0];
    let edges = [
        edge(&pocket, v, [2.0 + s, s, 1.0]),
        edge(&pocket, v, [0.0, 0.0, 0.0]),
    ];
    let air = halves_of(&convex);
    let pts = edges.map(|e| edge_points(&pocket, e));
    let mine = removed(&air, pts, D);
    let out = build(&pocket, &edges, D, "concave skewed box").unwrap();
    let (v0, p0) = vol(&pocket);
    let (v1, p1) = vol(&out.body);
    let d = v1 - v0 - mine;
    eprintln!(
        "P7 gained {:.12e} mine {mine:.12e} diff {d:e} pad {:e}",
        v1 - v0,
        p0 + p1
    );
    assert!(
        d.abs() < 2e-9_f64.max(p0 + p1),
        "the fill off my oracle by {d:e}"
    );
    let bands: Vec<Band> = pts.iter().map(|(a, b)| band(&air, *a, *b, D)).collect();
    let pb = geom_core::Band::linear(tol()).expect("band");
    let (mut checked, mut wrong) = (0, Vec::new());
    let n = 11;
    let h = 2.5 * D;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                let t = |m: usize| -h + 2.0 * h * (m as f64 + 0.5) / n as f64;
                let x = add(v, [t(i), t(j), t(k)]);
                // Air stays air where the model keeps it.
                let (air_kept, margin) = kept(&air, &bands, x);
                if margin < 1e-6 {
                    continue;
                }
                checked += 1;
                let want_material = !air_kept;
                match point_in_solid(&out.body, Point3::new(x[0], x[1], x[2]), pb, tol()) {
                    Ok(SolidContainment::In) if want_material => {}
                    Ok(SolidContainment::Out) if !want_material => {}
                    got => wrong.push(format!("{x:?}: {got:?}, material {want_material}")),
                }
            }
        }
    }
    eprintln!("P7 checked {checked}, wrong {}", wrong.len());
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}
