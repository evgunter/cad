//! **A fillet's overrun** (`work/band/a-non-isosceles-turn-overruns-past-the-mitre.md`,
//! step 3): two requested edges at a trivalent plane–plane vertex whose
//! third edge `L` is unrequested, the two bands' feet on `L` definitely
//! apart. The band whose foot is further, `k`, is cut off past the
//! mitre by the other band's face of `L`, `Fⱼ`, in the circle or ellipse
//! `fillet3_cap_transverse` picks for any cut-off there: from the
//! mitre's end `q` on band `j`'s trimline down to `k`'s foot `p_k` on
//! `L`, where `L` ends. The arc is tangent to the mitre at `q` and to
//! `L` at `p_k`; every vertex is trivalent.
//!
//! **The oracle.** A fillet of radius `r` of an edge of a convex
//! polytope `B` (outward unit normals `nₐ`, `n_b`, interior dihedral
//! `β`) takes `B ∩ W`, `W` the infinite prism along the edge over the
//! region between the edge, its two faces and the ball: in the section
//! plane through the edge's end `a`, the kite `a, Tₐ, O, T_b` (the
//! ball's centre `O` at `r / sin(β/2)` along the faces' inward
//! bisector, its feet `Tᵢ` at `r·cot(β/2)` into each face) less its
//! circular sector at `O` of angle `π − β`. At a turn the two bands'
//! prisms overlap; the mitre is where the two cylinders meet and the
//! overrun the far cylinder's trace on `Fⱼ`, so a fillet built as ruled
//! takes `B ∩ (W₁ ∪ W₂)`, whose volume is
//! `vol(B ∩ W₁) + vol(B ∩ W₂) − vol(B ∩ W₁ ∩ W₂)`:
//!
//! - **Per band, in closed form.** Where inside `W` only the edge's two
//!   end faces bound `B` (checked at the section's three hull corners,
//!   which suffices because each face's bound is affine there), the
//!   length of `B ∩ W` along the edge over a section point `s` is affine
//!   in `s`, `α − β·s`, and `vol(B ∩ W) = α·A − β·M` with `A` and `M`
//!   the section's area and first moment: two triangles less a sector,
//!   whose centroid stands `2r·sin(ψ/2) / (3·ψ/2)` from `O`. Where
//!   another face bounds it too (a yawed hexahedron's far end), by the
//!   overlap's quadrature with `B`'s own chord.
//! - **The overlap, by quadrature.** Over band 1's section, in polar
//!   coordinates about `O₁` (the sector's angle `ψ₁ = π − β₁`, the
//!   radius from `r` out to the kite's side, `r / cos θ` from either
//!   foot), the length of `B ∩ W₂` along the line through the point
//!   parallel to band 1's edge is read in closed form: `B` and band 2's
//!   kite are half-spaces along the line, and band 2's cylinder a
//!   quadratic, so it is an interval less one. Both integrals are
//!   globally adaptive Gauss–Kronrod (7–15), to `1e-9·r³`.
//!
//! The oracle shares nothing with the kernel but the input's planes. At
//! a concave corner the fillet FILLS the air `A` (a convex polytope)
//! the same way, so the body gains `vol(A ∩ (W₁ ∪ W₂))`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::Curve3;
use geom_core::{Bounds, Interval, Point3};
use sweep::blend::build::{Blended, fillet_edges};
use sweep::blend::{BlendDecision, BlendError};
use sweep::test_support::{assert_naming_totality, at_rest, realized};
use topo::boolean::BooleanOp;
use topo::{Body, ContactRecords, EdgeKey, mass_properties, validate_geometric};
use topo::{VertexKey, validate_pseudomanifold};

use crate::band_planar_cut_off::{D, arc_residual, edge, pad_ceiling, tol, volume_enclosure};
use crate::band_turn_overrun_chamfer::{
    Half, Witness, ends, ends_iv, halves_of, overrun_cells, placed, point_at, witnesses,
};
use crate::band_turn_overrun_oracles::{
    HEX, Hex, Poly, Req, TURN, a_clearance, edge_near, foot_inside_a_face, hexahedron, leaning,
    non_trivalent, on_wall, pinned, req, theta_s,
};
use crate::band_turn_overrun_probe::{
    Cone, V, Verb, add, cross, dot, measure, mul, norm, sub, unit,
};
use crate::band_turn_overrun_verdict::{corner, far_band};
use crate::common::cavity::brick;
use crate::common::operands::{
    edge_between, leaning_turn, leaning_turn_at, leaning_turn_in_band, parallelepiped,
    parallelepiped_at,
};
use crate::common::poses::poses;

// ------------------------------------------------------------------
// The oracle.
// ------------------------------------------------------------------

/// One band's section: the kite `a, Tₐ, O, T_b` in the plane through
/// the edge's end `a` normal to its direction `tau`, less the sector at
/// `O` of angle `psi` (module docs).
struct Section {
    a: V,
    tau: V,
    o: V,
    ta: V,
    tb: V,
    r: f64,
    psi: f64,
}

/// Band `req`'s section at radius `r` on the convex polytope `halves`,
/// its edge from `a` to `b`.
fn section(halves: &[Half], req: Req, a: V, b: V, r: f64) -> Section {
    let tau = unit(sub(b, a));
    let [na, nb] = req.faces.map(|i| halves[i].0);
    // In each face, the unit from the edge into it.
    let into = |n: V, other: V| {
        let w = unit(cross(n, tau));
        if dot(w, other) > 0.0 { mul(w, -1.0) } else { w }
    };
    let (ua, ub) = (into(na, nb), into(nb, na));
    let beta = dot(ua, ub).clamp(-1.0, 1.0).acos();
    let o = add(a, mul(unit(add(ua, ub)), r / (beta / 2.0).sin()));
    let foot = r / (beta / 2.0).tan();
    Section {
        a,
        tau,
        o,
        ta: add(a, mul(ua, foot)),
        tb: add(a, mul(ub, foot)),
        r,
        psi: PI - beta,
    }
}

/// The section's area and first moment (about the origin).
fn moments(s: &Section) -> (f64, V) {
    let tri = |p: V, q: V, w: V| {
        let area = 0.5 * norm(cross(sub(q, p), sub(w, p)));
        (area, mul(add(add(p, q), w), area / 3.0))
    };
    let (a1, m1) = tri(s.a, s.ta, s.o);
    let (a2, m2) = tri(s.a, s.o, s.tb);
    let half = s.psi / 2.0;
    let sector = half * s.r * s.r;
    let centroid = add(
        s.o,
        mul(unit(sub(s.a, s.o)), 2.0 * s.r * half.sin() / (3.0 * half)),
    );
    (a1 + a2 - sector, sub(add(m1, m2), mul(centroid, sector)))
}

/// The interval of `t` where `x + t·tau` satisfies every half-space.
fn clip_line(halves: &[Half], x: V, tau: V) -> (f64, f64) {
    let (mut lo, mut hi) = (f64::NEG_INFINITY, f64::INFINITY);
    for (n, c) in halves {
        let (along, at) = (dot(*n, tau), c - dot(*n, x));
        if along.abs() < 1e-15 {
            // Parallel: a point on the plane (a section's corner on its
            // own face) is inside it.
            if at < -1e-12 * (1.0 + c.abs()) {
                return (0.0, 0.0);
            }
        } else if along > 0.0 {
            hi = hi.min(at / along);
        } else {
            lo = lo.max(at / along);
        }
    }
    (lo, hi.max(lo))
}

/// `vol(B ∩ W)` for one band, in closed form (module docs).
fn band_volume(halves: &[Half], req: Req, s: &Section) -> f64 {
    let ends = [halves[req.ends[0]], halves[req.ends[1]]];
    let (mut alpha, mut slope) = (0.0, [0.0; 3]);
    for (n, c) in ends {
        let along = dot(n, s.tau);
        // `t ≤ (c − n·x)/(n·τ)` bounds above where `n·τ > 0`.
        let sign = if along > 0.0 { 1.0 } else { -1.0 };
        alpha += sign * c / along;
        slope = add(slope, mul(n, sign / along));
    }
    let only_ends = [s.a, s.ta, s.tb].into_iter().all(|x| {
        let (lo, hi) = clip_line(halves, x, s.tau);
        let (elo, ehi) = clip_line(&ends, x, s.tau);
        (hi - lo - (ehi - elo)).abs() < 1e-12 * (1.0 + hi - lo)
    });
    if only_ends {
        let (area, moment) = moments(s);
        alpha * area - dot(slope, moment)
    } else {
        band_quadrature(halves, s)
    }
}

/// `vol(B ∩ W)` for one band by quadrature over its section, the length
/// along the edge read off every face of `B`: where another face than
/// the two ends bounds the band (module docs).
fn band_quadrature(halves: &[Half], s: &Section) -> f64 {
    over_section(s, &|x: V| {
        let (lo, hi) = clip_line(halves, x, s.tau);
        hi - lo
    })
}

/// The length of `B ∩ W₂` along `x + t·τ₁` (module docs).
fn overlap_length(halves: &[Half], x: V, tau: V, two: &Section) -> f64 {
    let (mut lo, mut hi) = clip_line(halves, x, tau);
    // Band 2's kite, its sides' inward normals in its plane.
    let ring = [two.a, two.ta, two.o, two.tb];
    let mid = mul(ring.iter().fold([0.0; 3], |a, p| add(a, *p)), 0.25);
    for i in 0..4 {
        let (p, q) = (ring[i], ring[(i + 1) % 4]);
        let d = unit(sub(q, p));
        let g = sub(mid, p);
        let m = sub(g, mul(d, dot(g, d)));
        let (along, at) = (dot(m, tau), dot(m, sub(x, p)));
        // `at + t·along ≥ 0`.
        if along.abs() < 1e-15 {
            if at < 0.0 {
                return 0.0;
            }
        } else if along > 0.0 {
            lo = lo.max(-at / along);
        } else {
            hi = hi.min(-at / along);
        }
    }
    if hi <= lo {
        return 0.0;
    }
    // Inside band 2's cylinder: `|w + t·τ|² − ((w + t·τ)·τ₂)² < r²`.
    let w = sub(x, two.o);
    let ct = dot(tau, two.tau);
    let qa = 1.0 - ct * ct;
    let qb = 2.0 * (dot(w, tau) - dot(w, two.tau) * ct);
    let qc = dot(w, w) - dot(w, two.tau).powi(2) - two.r * two.r;
    let inside = if qa.abs() < 1e-15 {
        if qb.abs() < 1e-15 {
            if qc < 0.0 { Some((lo, hi)) } else { None }
        } else {
            let t = -qc / qb;
            Some(if qb > 0.0 {
                (f64::NEG_INFINITY, t)
            } else {
                (t, f64::INFINITY)
            })
        }
    } else {
        let disc = qb * qb - 4.0 * qa * qc;
        (disc > 0.0).then(|| {
            let root = disc.sqrt();
            ((-qb - root) / (2.0 * qa), (-qb + root) / (2.0 * qa))
        })
    };
    let removed = inside.map_or(0.0, |(a, b)| (b.min(hi) - a.max(lo)).max(0.0));
    hi - lo - removed
}

/// Globally adaptive Gauss–Kronrod (7–15) of `f` over `[a, b]`: the
/// panel with the largest error estimate is halved until the estimates
/// sum under `tol` (or a thousand panels stand).
fn integrate(f: &dyn Fn(f64) -> f64, a: f64, b: f64, tol: f64) -> f64 {
    const XK: [f64; 8] = [
        0.991_455_371_120_812_6,
        0.949_107_912_342_758_5,
        0.864_864_423_359_769_1,
        0.741_531_185_599_394_4,
        0.586_087_235_467_691_1,
        0.405_845_151_377_397_2,
        0.207_784_955_007_898_5,
        0.0,
    ];
    const WK: [f64; 8] = [
        0.022_935_322_010_529_22,
        0.063_092_092_629_978_55,
        0.104_790_010_322_250_18,
        0.140_653_259_715_525_92,
        0.169_004_726_639_267_9,
        0.190_350_578_064_785_4,
        0.204_432_940_075_298_9,
        0.209_482_141_084_727_83,
    ];
    const WG: [f64; 4] = [
        0.129_484_966_168_869_7,
        0.279_705_391_489_276_7,
        0.381_830_050_505_118_9,
        0.417_959_183_673_469_4,
    ];
    // A panel: its ends, its Kronrod estimate and its error estimate.
    let panel = |a: f64, b: f64| {
        let (c, h) = ((a + b) / 2.0, (b - a) / 2.0);
        let (mut k, mut g) = (0.0, 0.0);
        for i in 0..8 {
            let y = if XK[i] == 0.0 {
                f(c)
            } else {
                f(c + h * XK[i]) + f(c - h * XK[i])
            };
            k += WK[i] * y;
            if i % 2 == 1 {
                g += WG[i / 2] * y;
            }
        }
        (a, b, k * h, ((k - g) * h).abs())
    };
    let mut panels = vec![panel(a, b)];
    while panels.iter().map(|p| p.3).sum::<f64>() > tol && panels.len() < 1000 {
        let (worst, _) = panels
            .iter()
            .enumerate()
            .max_by(|x, y| x.1.3.total_cmp(&y.1.3))
            .expect("a panel");
        let (a, b, _, _) = panels.swap_remove(worst);
        let c = (a + b) / 2.0;
        panels.push(panel(a, c));
        panels.push(panel(c, b));
    }
    panels.iter().map(|p| p.2).sum()
}

/// `∫ len` over band `one`'s section, in polar coordinates about its
/// ball's centre (module docs).
fn over_section(one: &Section, len: &dyn Fn(V) -> f64) -> f64 {
    let e0 = unit(sub(one.ta, one.o));
    let e1 = {
        let w = sub(one.tb, one.o);
        unit(sub(w, mul(e0, dot(w, e0))))
    };
    let r = one.r;
    let tol = 1e-9 * r.powi(3);
    let reach = |theta: f64| r / theta.min(one.psi - theta).cos();
    let inner = |theta: f64| {
        let dir = add(mul(e0, theta.cos()), mul(e1, theta.sin()));
        integrate(
            &|rho: f64| rho * len(add(one.o, mul(dir, rho))),
            r,
            reach(theta),
            tol / one.psi,
        )
    };
    let mid = one.psi / 2.0;
    integrate(&inner, 0.0, mid, tol / 2.0) + integrate(&inner, mid, one.psi, tol / 2.0)
}

/// `vol(B ∩ W₁ ∩ W₂)`, by quadrature over band 1's section (module
/// docs).
fn overlap(halves: &[Half], one: &Section, two: &Section) -> f64 {
    over_section(one, &|x: V| overlap_length(halves, x, one.tau, two))
}

/// The requested edge from `a` to `b` of the convex polytope `halves`:
/// the two faces holding it and each end's third face.
fn req_at(halves: &[Half], a: V, b: V) -> Req {
    let on = |i: usize, p: V| (dot(halves[i].0, p) - halves[i].1).abs() < 1e-9;
    let faces: Vec<usize> = (0..halves.len())
        .filter(|&i| on(i, a) && on(i, b))
        .collect();
    let [f0, f1] = faces[..] else {
        panic!("an edge lies on two faces, got {faces:?}")
    };
    let end = |p: V| {
        (0..halves.len())
            .find(|&i| i != f0 && i != f1 && on(i, p))
            .expect("an end's third face")
    };
    req([f0, f1], [end(a), end(b)])
}

/// **What a fillet of `reqs` at `r` takes from** (or, concave, gives to)
/// the convex polytope `halves`, each request with its edge's two ends:
/// the bands' volumes less every adjacent pair's overlap. Bands that
/// share no vertex do not overlap on the fixtures here (asserted: their
/// overlap reads zero).
fn taken(halves: &[Half], reqs: &[(Req, V, V)], r: f64) -> f64 {
    let sections: Vec<Section> = reqs
        .iter()
        .map(|(q, a, b)| section(halves, *q, *a, *b, r))
        .collect();
    let mut total: f64 = reqs
        .iter()
        .zip(&sections)
        .map(|((q, _, _), s)| band_volume(halves, *q, s))
        .sum();
    for i in 0..reqs.len() {
        for j in i + 1..reqs.len() {
            let ([a1, b1], [a2, b2]) = ([reqs[i].1, reqs[i].2], [reqs[j].1, reqs[j].2]);
            let touch = [a1, b1]
                .iter()
                .any(|p| [a2, b2].iter().any(|q| norm(sub(*p, *q)) < 1e-9));
            let both = overlap(halves, &sections[i], &sections[j]);
            if touch {
                total -= both;
            } else {
                assert!(both.abs() < 1e-12, "bands apart overlap by {both}");
            }
        }
    }
    total
}

/// [`taken`] on a body's own edges.
fn taken_on(halves: &[Half], body: &Body<f64>, edges: &[EdgeKey], r: f64) -> f64 {
    let reqs: Vec<(Req, V, V)> = edges
        .iter()
        .map(|e| {
            let [a, b] = ends(body, *e).map(|v| point_at(body, v));
            (req_at(halves, a, b), a, b)
        })
        .collect();
    taken(halves, &reqs, r)
}

/// Whether band `q`'s section at `r` meets a face of `p` other than its
/// two end faces along its edge: where a trimline runs into that face
/// first.
fn runs_into_another_face(p: &Poly, q: Req, r: f64) -> bool {
    let (_, a, b) = p.edge(q);
    let s = section(&p.halves, q, a, b, r);
    let ends = [p.halves[q.ends[0]], p.halves[q.ends[1]]];
    [s.ta, s.tb].into_iter().any(|x| {
        let (lo, hi) = clip_line(&p.halves, x, s.tau);
        let (elo, ehi) = clip_line(&ends, x, s.tau);
        (hi - lo - (ehi - elo)).abs() > 1e-12 * (1.0 + hi - lo)
    })
}

/// [`taken`] on a [`Poly`]'s requests.
fn poly_taken(p: &Poly, reqs: &[Req], r: f64) -> f64 {
    let reqs: Vec<(Req, V, V)> = reqs
        .iter()
        .map(|q| {
            let (_, a, b) = p.edge(*q);
            (*q, a, b)
        })
        .collect();
    taken(&p.halves, &reqs, r)
}

// ------------------------------------------------------------------
// The build.
// ------------------------------------------------------------------

/// Fillet `edges` of `body` at `r`; a build is held to tiers 3 and 3′,
/// naming totality and every minted arc on both its faces before it is
/// returned.
fn filleted(
    body: &Body<f64>,
    edges: &[EdgeKey],
    r: f64,
    what: &str,
) -> Result<Blended<f64>, BlendError> {
    let out = fillet_edges(&at_rest(body, tol()), edges, r, tol()).map_err(|e| e.error)?;
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, {e:?}"));
    validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
        .unwrap_or_else(|e| panic!("{what}: tier 3′, {e:?}"));
    assert_naming_totality(body, &out, edges, what);
    let (_, worst) = arc_residual(&out);
    assert!(
        worst < 1e-12 * r.max(1.0),
        "{what}: every minted arc lies on both its faces, off by {worst:e}"
    );
    Ok(out)
}

/// [`filleted`] at `D`, built, an overrun (no common foot).
fn fillet(body: &Body<f64>, edges: &[EdgeKey], what: &str) -> Blended<f64> {
    let out = filleted(body, edges, D, what)
        .unwrap_or_else(|e| panic!("{what}: the overrun builds, got {e}"));
    assert!(
        out.coincidences
            .iter()
            .all(|r| r.relation != topo::Relation::CommonFoot),
        "{what}: an overrun records no common foot"
    );
    out
}

/// **What a build took**, `before − after` (negative where it filled),
/// held to `want`: the volume of a fillet's band is certified, its
/// enclosure's half-width under [`pad_ceiling`], so the row admits
/// `slack` plus that half-width.
fn assert_delta(before: &Body<f64>, after: &Body<f64>, want: f64, slack: f64, what: &str) {
    let (v0, pad0) = volume_enclosure(before);
    let (v1, pad1) = volume_enclosure(after);
    let pad = pad0 + pad1;
    assert!(
        pad < pad_ceiling(),
        "{what}: the enclosure is a claim, ±{pad:e}"
    );
    assert!(
        (v0 - v1 - want).abs() < slack + pad,
        "{what}: ΔV {} against the oracle's {want} (±{pad:e})",
        v0 - v1
    );
}

/// The shared vertex of a turn's two requested edges.
fn turn_vertex(body: &Body<f64>, edges: [EdgeKey; 2]) -> VertexKey {
    let [a, b] = [ends(body, edges[0]), ends(body, edges[1])];
    a.into_iter()
        .find(|v| b.contains(v))
        .expect("a shared vertex")
}

/// The unit tangent of `edge`'s carrier at its end at `p`, pointing into
/// the edge.
fn tangent_into(body: &Body<f64>, edge: EdgeKey, p: V) -> V {
    let c = body
        .get_curve_geom(body.get_edge(edge).expect("an edge").curve)
        .and_then(|g| g.certified())
        .expect("a certified carrier");
    let (t0, t1) = c.params();
    let at = |t: f64| {
        let x = c.carrier().eval(t);
        [x.x, x.y, x.z]
    };
    let h = (t1 - t0) * 1e-7;
    if norm(sub(at(t0), p)) < norm(sub(at(t1), p)) {
        unit(sub(at(t0 + h), at(t0)))
    } else {
        unit(sub(at(t1 - h), at(t1)))
    }
}

/// **The overrun's arc**: the far band's `EndArc` at the vertex runs
/// from `p_k` to `q` on the probe's circle or ellipse (minor `r`, major
/// `r / |e_k · n̂ⱼ|`), tangent to the mitre at `q` and to `L`'s remaining
/// piece at `p_k`.
fn overrun_arc(source: &Body<f64>, edges: [EdgeKey; 2], out: &Blended<f64>, what: &str) {
    let (k, p_k, q) = placed(source, edges, Verb::Fillet);
    assert_eq!(
        k,
        far_band(source, edges, Verb::Fillet),
        "{what}: the far band"
    );
    overrun_cells(out, p_k, q, what);
    let body = &out.body;
    let vertex = turn_vertex(source, edges);
    let rec = out.naming.as_ref().expect("births");
    let at = |x: V| {
        body.vertices()
            .map(|(v, _)| v)
            .find(|v| norm(sub(point_at(body, *v), x)) < 1e-9)
            .expect("a vertex")
    };
    let (foot, end) = (at(p_k), at(q));
    let arc = rec
        .arcs
        .iter()
        .find(|(e, v, x)| {
            *v == vertex && *x == edges[k] && {
                let [a, b] = ends(body, *e);
                [a, b] == [foot, end] || [a, b] == [end, foot]
            }
        })
        .map(|(e, _, _)| *e)
        .unwrap_or_else(|| panic!("{what}: the overrun is the far band's EndArc"));
    let c = measure(&cone_of_turn(source, edges), Verb::Fillet, D, 2);
    let (minor, major) = c.ellipse.expect("a fillet's overrun ellipse");
    let carrier = body
        .get_curve_geom(body.get_edge(arc).expect("the arc").curve)
        .and_then(|g| g.certified())
        .expect("a certified arc")
        .carrier()
        .clone();
    match carrier {
        Curve3::Circle { radius, .. } => assert!(
            (radius - D).abs() < 1e-15 && (major - minor).abs() < 1e-6 * D,
            "{what}: a circle where the probe reads ({minor}, {major}), radius {radius}"
        ),
        Curve3::Ellipse {
            major: got_major,
            minor: got_minor,
            ..
        } => assert!(
            (got_minor - D).abs() < 1e-15 && (got_major - major).abs() < 1e-9,
            "{what}: the ellipse ({got_minor}, {got_major}) against the probe's ({minor}, {major})"
        ),
        other => panic!("{what}: the overrun arc is a circle or an ellipse, got {other:?}"),
    }
    // G1: the arc continues the mitre at `q`, and leaves `L` along it.
    let mitre = rec.mitres.first().expect("a mitre").0;
    let piece = body
        .edges_of_vertex(foot)
        .expect("a fan")
        .into_iter()
        .find(|e| {
            *e != arc && !rec.arcs.iter().any(|(a, _, _)| a == e) && {
                let he = body.get_edge(*e).expect("an edge").he_plus;
                let other = if body.get_half_edge(he).expect("a half").start == foot {
                    body.half_edge_end(he).expect("an end")
                } else {
                    body.get_half_edge(he).expect("a half").start
                };
                let d = unit(sub(point_at(body, other), p_k));
                dot(d, cone_of_turn(source, edges).l).abs() > 1.0 - 1e-9
            }
        })
        .expect("L's remaining piece at the turn foot");
    for (other, p, name) in [(mitre, q, "the mitre at q"), (piece, p_k, "L at p_k")] {
        let (x, y) = (tangent_into(body, arc, p), tangent_into(body, other, p));
        assert!(
            dot(x, y) < -1.0 + 1e-9,
            "{what}: the overrun arc is tangent to {name}: {x:?} against {y:?}"
        );
    }
}

/// The probe's cone at a turn of `body`.
fn cone_of_turn(body: &Body<f64>, edges: [EdgeKey; 2]) -> Cone {
    crate::band_turn_overrun_verdict::cone_of(body, edges)
}

// ------------------------------------------------------------------
// The rows.
// ------------------------------------------------------------------

/// **The oracle against a hand closed form**: a box's top corner, two
/// edges of lengths `2` and `1.5` filleted at `r`, takes
/// `(1 − π/4)·r²·(2 + 1.5) − (5/3 − π/2)·r³`, the overlap being
/// `∫₀ʳ (r − √(r² − w²))² dw`; and the kernel's mitre there, built before
/// any overrun, is the oracle's. The overlap's quadrature also reads one
/// band's own volume, against its closed form.
#[test]
fn the_oracle_reads_a_box_corner_by_hand_and_the_mitre_it_built() {
    let p = leaning(0.0, 1.0, 1.0);
    for r in [0.05, 0.1, 0.3] {
        let want = (1.0 - PI / 4.0) * r * r * 3.5 - (5.0 / 3.0 - PI / 2.0) * r.powi(3);
        let got = poly_taken(&p, &TURN, r);
        assert!(
            (got - want).abs() < 1e-12,
            "r {r}: the oracle {got} against the hand form {want}"
        );
        let what = format!("box corner, r {r}");
        let out = filleted(&p.body, &p.edges(&TURN), r, &what)
            .unwrap_or_else(|e| panic!("{what}: the mitre builds, got {e}"));
        assert_delta(&p.body, &out.body, want, 1e-12, &what);
    }
    // The quadrature against the closed form on each band of a generic
    // corner.
    let p = hexahedron(HEX);
    for q in TURN {
        let (_, a, b) = p.edge(q);
        let s = section(&p.halves, q, a, b, 0.1);
        let (own, read) = (
            band_volume(&p.halves, q, &s),
            band_quadrature(&p.halves, &s),
        );
        assert!(
            (own - read).abs() < 1e-12,
            "a band's own volume: closed form {own}, quadrature {read}"
        );
    }
}

/// **The witnesses build** ([`witnesses`]: the bracket's two
/// section-face corners, the leaning prism at `±0.5`, the skewed box),
/// and the sheared box's supplementary corner, where the chamfer
/// mitres — tiers 3 and 3′, naming total, `ΔV` the oracle's to `1e-10`,
/// the overrun's cells and arc where the probe puts them.
#[test]
fn the_witnesses_build_at_the_oracles_volumes() {
    for (what, body, edges) in fillet_witnesses() {
        let out = fillet(&body, &edges, what);
        let want = taken_on(&halves_of(&body), &body, &edges, D);
        assert_delta(&body, &out.body, want, 1e-12, what);
        assert_eq!(
            non_trivalent(&out.body),
            0,
            "{what}: every vertex trivalent"
        );
        overrun_arc(&body, edges, &out, what);
    }
}

/// [`witnesses`] and the sheared box's supplementary corner.
fn fillet_witnesses() -> Vec<Witness> {
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

/// **At poses**: each witness moved rigidly through the six-pose set
/// builds tier-3 valid at the oracle's `ΔV`.
#[test]
fn the_witnesses_build_at_every_pose() {
    for (what, body, edges) in fillet_witnesses() {
        let want = taken_on(&halves_of(&body), &body, &edges, D);
        for (pose, map) in poses() {
            let moved = topo::transform_rigid(&body, &map, tol()).expect("a rigid pose");
            let what = format!("{what} at {pose}");
            let out = fillet(&moved, &edges, &what);
            assert_delta(&moved, &out.body, want, 1e-11, &what);
            assert_eq!(
                non_trivalent(&out.body),
                0,
                "{what}: every vertex trivalent"
            );
        }
    }
}

/// A certified replay: its name, its `f64` body, and its `Interval`
/// body with the two requested edges.
type IntervalCase = (String, Body<f64>, Body<Interval>, [EdgeKey; 2]);

/// **At the certified scalar**: the leaning prism at `±0.5` and `±1`
/// (the bracket's corners) at `Interval` build, tier-3 valid and naming
/// total, the volume enclosure bracketing the oracle. The skewed box is
/// [`the_skewed_boxs_fillet_refuses_at_interval_as_its_plain_cut_off_does`].
#[test]
fn the_overrun_carves_at_the_certified_scalar() {
    use crate::common::interval::iv;
    let cases: Vec<IntervalCase> = [0.5, -0.5, 1.0, -1.0]
        .into_iter()
        .map(|lean| {
            let (ibody, iedges) = leaning_turn_at::<Interval>(lean);
            (
                format!("leaning prism {lean}"),
                leaning_turn(lean).0,
                ibody,
                iedges,
            )
        })
        .collect();
    for (what, body, ibody, iedges) in cases {
        let fedges = iedges.map(|e| {
            let [a, b] = ends_iv(&ibody, e);
            edge_between(&body, a, b)
        });
        let want = taken_on(&halves_of(&body), &body, &fedges, D);
        let ibody = sweep::test_support::finished("operand", ibody, tol());
        let out = fillet_edges(&ibody, &iedges, iv(D), tol())
            .unwrap_or_else(|e| panic!("{what}: builds at Interval, got {:?}", e.error));
        validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, {e:?}"));
        assert_naming_totality(&ibody, &out, &iedges, &what);
        let v0 = mass_properties(&ibody, tol()).expect("props");
        let p = mass_properties(&out.body, tol()).expect("props");
        let removed = v0.volume - p.volume;
        let pad = v0.volume_pad.hi() + p.volume_pad.hi();
        assert!(
            removed.lo() - pad <= want && want <= removed.hi() + pad,
            "{what}: ΔV {removed:?} ± {pad} brackets {want}"
        );
        assert!(
            removed.hi() - removed.lo() + 2.0 * pad < 1e-5,
            "{what}: the enclosure is a claim: {removed:?} ± {pad}"
        );
    }
}

/// **The skewed box at `Interval` refuses**, as the plain cut-off of its
/// top edge does there, where `f64` builds both
/// (`work/band/an-interval-fillet-cut-off-at-an-oblique-end-reads-its-arc-backward.md`):
/// the fillet's cut-off arc at an oblique end reads backward. The fix
/// flips both, and the overrun's `Interval` row then takes the box.
#[test]
fn the_skewed_boxs_fillet_refuses_at_interval_as_its_plain_cut_off_does() {
    use crate::common::interval::iv;
    let s = 0.3;
    let v = [s, s, 1.0];
    let ibox = parallelepiped_at::<Interval>(s);
    let iedges = [
        edge_between(&ibox, v, [2.0 + s, s, 1.0]),
        edge_between(&ibox, v, [0.0, 0.0, 0.0]),
    ];
    let ibody = sweep::test_support::finished("operand", ibox, tol());
    for (what, edges) in [
        ("the top edge alone", &iedges[..1]),
        ("the turn", &iedges[..]),
    ] {
        match fillet_edges(&ibody, edges, iv(D), tol()) {
            Err(e) => assert!(
                matches!(&e.error, BlendError::BodyNotIntact { detail, .. }
                    if detail.contains("does not run forward from one foot")),
                "{what}: the cut-off's arc reads backward, got {:?}",
                e.error
            ),
            Ok(_) => panic!("{what}: built at Interval; the filed item is fixed, flip this row"),
        }
    }
}

/// **Concave**: the leaning prism at `±0.5` cut up into a block from
/// `z = 0.5`, its corner a pocket's ceiling corner, and the skewed box
/// as a void inside a block — the fillet fills the overrun, tier 3 and
/// 3′, naming total, the body gaining what the oracle reads of the air,
/// the overrun's cells and arc where the probe puts them.
#[test]
fn a_concave_overrun_fills_at_the_oracles_volume() {
    let s = 0.3;
    let block = |lo: V, hi: V| {
        brick(
            Point3::new(lo[0], lo[1], lo[2]),
            Point3::new(hi[0], hi[1], hi[2]),
        )
    };
    let floor: Half = ([0.0, 0.0, -1.0], -0.5);
    let leaning_case = |lean: f64| {
        let (prism, _) = leaning_turn(lean);
        let corner = [lean, 0.0, 1.0];
        (
            prism,
            block([-1.0, -2.5, 0.5], [3.0, 1.0, 2.0]),
            [corner, [2.0, 0.0, 1.0]],
            [corner, [lean, -1.5, 1.0]],
            true,
        )
    };
    let v = [s, s, 1.0];
    let cases = [
        ("leaning prism 0.5, concave", leaning_case(0.5)),
        ("leaning prism −0.5, concave", leaning_case(-0.5)),
        (
            "skewed box, concave",
            (
                parallelepiped(s),
                block([-1.0, -1.0, -1.0], [4.0, 3.0, 2.0]),
                [v, [2.0 + s, s, 1.0]],
                [v, [0.0, 0.0, 0.0]],
                false,
            ),
        ),
    ];
    for (what, (convex, block, a, b, floored)) in cases {
        let pocket = realized(BooleanOp::Subtract, &block, &convex, tol());
        let edges = [edge(&pocket, a[0], a[1]), edge(&pocket, b[0], b[1])];
        let out = fillet(&pocket, &edges, what);
        let mut air = halves_of(&convex);
        if floored {
            air.push(floor);
        }
        let want = taken_on(&air, &pocket, &edges, D);
        assert_delta(&pocket, &out.body, -want, 1e-11, what);
        assert_eq!(
            non_trivalent(&out.body),
            0,
            "{what}: every vertex trivalent"
        );
        overrun_arc(&pocket, edges, &out, what);
    }
}

/// **The sweep**: over face-angle pairs `φ₁, φ₂ ∈ 30°…150°` (20° steps)
/// at `γ ∈ {60°, 90°, 120°}`, every corner whose fillet overruns (all
/// but the isosceles diagonal) builds — tiers 3 and 3′, naming total —
/// at the oracle's `ΔV`, its turn foot at the probe's far foot, the
/// mitre's end at the probe's `q`, its arc the probe's circle or
/// ellipse, tangent to the mitre and to `L`, the far band the probe's.
#[test]
fn the_sweep_builds_every_convex_overrun_at_the_probes_closed_forms() {
    let mut built = 0;
    for gamma in [60.0f64, 90.0, 120.0] {
        for p1 in (30..=150).step_by(20) {
            for p2 in (30..=150).step_by(20) {
                let c = Cone::angles(
                    [f64::from(p1).to_radians(), f64::from(p2).to_radians()],
                    gamma.to_radians(),
                );
                let t = measure(&c, Verb::Fillet, D, 2);
                if t.far.is_none() || t.gap < 1e-6 {
                    continue;
                }
                let what = format!("({p1}°, {p2}°, γ {gamma}°)");
                let (body, edges) = corner(&c);
                let out = fillet(&body, &edges, &what);
                let want = taken_on(&halves_of(&body), &body, &edges, D);
                assert_delta(&body, &out.body, want, 1e-11, &what);
                overrun_arc(&body, edges, &out, &what);
                built += 1;
            }
        }
    }
    assert_eq!(built, 3 * (49 - 7), "every overrun corner built");
}

/// **The sweep, concave**: the corners of
/// [`the_sweep_builds_every_convex_overrun_at_the_probes_closed_forms`]
/// at 40° steps, each the air of a void inside a block, its overrun
/// filled at the oracle's volume of the air, its cells and arc where the
/// probe puts them.
#[test]
fn the_sweep_fills_concave_overruns() {
    let mut built = 0;
    for gamma in [60.0f64, 90.0, 120.0] {
        for p1 in (30..=150).step_by(40) {
            for p2 in (30..=150).step_by(40) {
                let c = Cone::angles(
                    [f64::from(p1).to_radians(), f64::from(p2).to_radians()],
                    gamma.to_radians(),
                );
                let t = measure(&c, Verb::Fillet, D, 2);
                if t.far.is_none() || t.gap < 1e-6 {
                    continue;
                }
                let what = format!("({p1}°, {p2}°, γ {gamma}°), concave");
                let (wedge, wedge_edges) = corner(&c);
                let ends = wedge_edges.map(|e| ends(&wedge, e).map(|v| point_at(&wedge, v)));
                let block = brick(
                    Point3::new(-50.0, -50.0, -50.0),
                    Point3::new(50.0, 50.0, 50.0),
                );
                let void = realized(BooleanOp::Subtract, &block, &wedge, tol());
                let edges = ends.map(|[a, b]| edge(&void, a, b));
                let out = fillet(&void, &edges, &what);
                let want = taken_on(&halves_of(&wedge), &void, &edges, D);
                assert_delta(&void, &out.body, -want, 1e-10, &what);
                overrun_arc(&void, edges, &out, &what);
                built += 1;
            }
        }
    }
    // Per γ, 16 pairs less the isosceles diagonal (4).
    assert_eq!(built, 3 * 12, "every concave overrun corner built");
}

/// **Leaning prisms over leans, radii and scales**, both request
/// orders: every one builds, every vertex trivalent, at the oracle's
/// `ΔV`. The small scale is `10⁷` zero widths (`0.01` at the default
/// band), as the chamfer's rows take it.
#[test]
fn leaning_prisms_build_at_the_oracle_over_leans_radii_and_scales() {
    let mut failures = Vec::new();
    let small = 1e7
        * geom_core::Band::linear(tol())
            .expect("the run's band")
            .zero();
    for k in [1.0, small, 100.0] {
        for s in [-2.0, -1.0, -0.5, 0.25, 0.5, 1.0, 1.5] {
            let p = leaning(s, 1.0, k);
            for r in [0.05, 0.3] {
                let want = poly_taken(&p, &TURN, r * k);
                for order in [[0, 1], [1, 0]] {
                    let what = format!("lean {s} scale {k} r {r} order {order:?}");
                    let edges = p.edges(&order.map(|i| TURN[i]));
                    match filleted(&p.body, &edges, r * k, &what) {
                        Ok(out) => {
                            if non_trivalent(&out.body) != 0 {
                                failures.push(format!("{what}: a vertex not trivalent"));
                            }
                            let (v0, p0) = volume_enclosure(&p.body);
                            let (v1, p1) = volume_enclosure(&out.body);
                            let slack = 1e-11 * k.powi(3) + p0 + p1;
                            if (v0 - v1 - want).abs() > slack {
                                failures.push(format!(
                                    "{what}: ΔV {} against the oracle's {want} (±{slack:e})",
                                    v0 - v1
                                ));
                            }
                        }
                        Err(e) => failures.push(format!("{what}: refused {e}")),
                    }
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// **Generic hexahedra**: the seven corners with no right angle the
/// chamfer's rows build, at three radii and both request orders, build
/// at the oracle's `ΔV`, every vertex trivalent — except where a band's
/// trimline runs into a face before its end face (the yawed front's
/// far end at `r = 0.2`, read off the oracle's section), which refuses
/// `FOOT_INSIDE_A_FACE` as any cut-off there does; two of them as voids
/// fill at it.
#[test]
fn generic_hexahedra_build_and_fill_at_the_oracle() {
    let mut built = 0;
    for (s, a, b, c) in [
        (0.4, 0.2, 0.3, 0.0),
        (-0.6, -0.15, 0.25, 0.0),
        (0.7, 0.3, -0.2, 0.0),
        (-0.3, 0.25, -0.35, 0.0),
        (0.4, 0.2, 0.3, 0.8),
        (-0.6, -0.15, 0.25, -0.7),
        (0.5, 0.1, 0.2, -0.5),
    ] {
        let p = hexahedron(Hex { s, a, b, c, ..HEX });
        let theta = theta_s(&p);
        for r in [0.05, 0.1, 0.2] {
            let runs_in = TURN.iter().any(|q| runs_into_another_face(&p, *q, r));
            for order in [[0, 1], [1, 0]] {
                let what =
                    format!("hexahedron ({s}, {a}, {b}, {c}), θ_S {theta:.1}°, r {r}, {order:?}");
                let edges = p.edges(&order.map(|i| TURN[i]));
                match filleted(&p.body, &edges, r, &what) {
                    Ok(out) if !runs_in => {
                        assert_eq!(
                            non_trivalent(&out.body),
                            0,
                            "{what}: every vertex trivalent"
                        );
                        assert_delta(&p.body, &out.body, poly_taken(&p, &TURN, r), 1e-11, &what);
                    }
                    Err(e) if runs_in => assert!(
                        foot_inside_a_face(&e),
                        "{what}: a trimline runs into a face, refusing as at any cut-off, got {e}"
                    ),
                    other => panic!(
                        "{what}: a trimline runs into another face: {runs_in}, got {:?}",
                        other.map(|_| ())
                    ),
                }
                built += usize::from(!runs_in);
            }
        }
    }
    assert!(built >= 40, "most hexahedra build: {built} of 42");
    for (s, a, b, c) in [(0.4, 0.2, 0.3, 0.5), (-0.6, -0.15, 0.25, -0.5)] {
        let what = format!("the hexahedron ({s}, {a}, {b}, {c}) as a void");
        let p = hexahedron(Hex { s, a, b, c, ..HEX });
        let block = brick(Point3::new(-5.0, -5.0, -5.0), Point3::new(5.0, 5.0, 5.0));
        let void = realized(BooleanOp::Subtract, &block, &p.body, tol());
        let edges: Vec<EdgeKey> = TURN
            .iter()
            .map(|r| {
                let (_, a, b) = p.edge(*r);
                edge_near(&void, a, b)
            })
            .collect();
        let out =
            filleted(&void, &edges, D, &what).unwrap_or_else(|e| panic!("{what}: fills, got {e}"));
        assert_eq!(
            non_trivalent(&out.body),
            0,
            "{what}: every vertex trivalent"
        );
        assert_delta(&void, &out.body, -poly_taken(&p, &TURN, D), 1e-11, &what);
    }
}

/// **Features in and around the overrun** at the bracket's `y = 0`
/// corner (`leaning_turn(−1)`), where the fillet's far band is the
/// leaning wall's (its dihedral `45°`) and the overrun lies on the front
/// face `y = 0`: in its `(x, z)`, band `j`'s trimline is `z = 1 − r`,
/// the far band's circle has radius `r` about `(−1 + (1 + √2)·r, 1 − r)`,
/// and the overrun is bounded by `L` (`x + z = 0`) from `p_j` to `p_k`,
/// the trimline from `p_j` to `q = (−1 + √2·r, 1 − r)`, and the arc from
/// `q` down to `p_k`; `(−0.863, 0.876)` stands inside it, `0.007` clear
/// of the arc. A feature the fillet cuts into refuses at a clearance
/// meter; one clear of everything it takes builds, at the feature-free
/// `ΔV`.
#[test]
fn features_in_and_around_the_overrun_refuse_or_build_as_their_regions_say() {
    let (plain, plain_edges) = leaning_turn(-1.0);
    let plain_out = fillet(&plain, &plain_edges, "feature-free");
    let (v0, p0) = volume_enclosure(&plain);
    let (v1, p1) = volume_enclosure(&plain_out.body);
    let free = v0 - v1;
    let front = |x: f64, z: f64| ([x, 0.0, z], [0.0, 1.0, 0.0]);
    // (what, where, boss or hole depth, builds)
    let rows = [
        (
            "a front boss in the overrun",
            front(-0.863, 0.876),
            None,
            false,
        ),
        (
            "a shallow front hole in the overrun",
            front(-0.863, 0.876),
            Some(0.002),
            false,
        ),
        (
            "a front boss clear of both bands",
            front(-0.5, 0.7),
            None,
            true,
        ),
        (
            "a front hole clear of both bands",
            front(-0.5, 0.7),
            Some(0.002),
            true,
        ),
        ("a wall boss in k's strip", on_wall(2.0, 0.5), None, false),
        ("a wall boss past k's strip", on_wall(3.0, 0.5), None, true),
    ];
    let mut failures = Vec::new();
    for (what, (at, n), hole, builds) in rows {
        let (body, edges) = pinned(at, n, hole.is_none(), hole.unwrap_or(0.0));
        match (filleted(&body, &edges, D, what), builds) {
            (Ok(out), true) => {
                let (w0, q0) = volume_enclosure(&body);
                let (w1, q1) = volume_enclosure(&out.body);
                let slack = 1e-11 + p0 + p1 + q0 + q1;
                if (w0 - w1 - free).abs() > slack {
                    failures.push(format!(
                        "{what}: ΔV {} against the feature-free {free} (±{slack:e})",
                        w0 - w1
                    ));
                }
            }
            (Ok(_), false) => failures.push(format!("{what}: built where the fillet cuts it")),
            (Err(e), true) => failures.push(format!("{what}: refused where clear, {e}")),
            (Err(e), false) => {
                if !a_clearance(&e) {
                    failures.push(format!("{what}: refuses at a clearance meter, got {e}"));
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// **A void in the overrun's volume, touching no face**: a cube of
/// half-size `h` about a point `0.003` in from the front face over the
/// overrun region of
/// [`features_in_and_around_the_overrun_refuse_or_build_as_their_regions_say`],
/// inside the far band's cut region and outside the near band's. The far
/// band's reach, its cap at the front face, refuses it as wholly inside
/// the material that band removes.
#[test]
fn a_void_in_the_overruns_volume_refuses() {
    let (plain, _) = leaning_turn(-1.0);
    let c = [-0.863, -0.003, 0.876];
    for h in [0.0012, 0.0005] {
        let void = brick(
            Point3::new(c[0] - h, c[1] - h, c[2] - h),
            Point3::new(c[0] + h, c[1] + h, c[2] + h),
        );
        let body = realized(BooleanOp::Subtract, &plain, &void, tol());
        let v = [-1.0, 0.0, 1.0];
        let edges = [
            edge_near(&body, v, [2.0, 0.0, 1.0]),
            edge_near(&body, v, [-1.0, -1.5, 1.0]),
        ];
        match filleted(&body, &edges, D, "a void in the overrun") {
            Err(e) => assert!(
                matches!(e, BlendError::WhollyInside { .. }),
                "h {h}: refuses as wholly inside, got {e}"
            ),
            Ok(_) => panic!("h {h}: a void in the overrun built"),
        }
    }
}

/// **In band, still escalated**: the leaning prism at a lean its
/// chamfer turn reads in band escalates the fillet too, on its overrun's
/// cut-off at the near face (a face tilted by the lean off the far
/// band's axis: the ellipse's axes in band); a `60°` corner skewed by a
/// few millionths of a degree escalates on the turn's own readings. The
/// near-T needle is `band_turn_overrun_verdict`'s.
#[test]
fn a_fillet_turn_in_band_escalates_through_the_door() {
    let (body, edges) = leaning_turn_in_band(tol());
    match filleted(&body, &edges, D, "the leaning prism in band") {
        Err(BlendError::Escalated {
            decision: BlendDecision::CapEllipse,
            ..
        }) => {}
        other => panic!(
            "the leaning prism in band escalates its overrun's ellipse, got {:?}",
            other.map(|_| ())
        ),
    }
    let z = geom_core::Band::linear(tol())
        .expect("the run's band")
        .zero()
        / 1e-9;
    let (skewed, edges) = corner(&Cone::angles(
        [60f64.to_radians(), (60.0 + 2e-6 * z).to_radians()],
        90f64.to_radians(),
    ));
    match filleted(&skewed, &edges, D, "a 60° corner in band") {
        Err(BlendError::Escalated {
            decision:
                BlendDecision::TurnFeetGap | BlendDecision::TurnOverrunChord | BlendDecision::TurnFeet,
            ..
        }) => {}
        other => panic!(
            "a 60° corner in band escalates on the turn's readings, got {:?}",
            other.map(|_| ())
        ),
    }
}

/// **L filleted in a later call** ends at the overrun's turn foot, in
/// the far band's face — a cylinder, a curved end face — and refuses
/// as one, at each of the leaning prism's overrun corners (`±0.5`,
/// `±1`).
#[test]
fn a_later_fillet_of_l_refuses_at_the_far_bands_curved_face() {
    for s in [0.5, -0.5, 1.0, -1.0] {
        let what = format!("lean {s}");
        let (body, edges) = leaning_turn(s);
        let out = fillet(&body, &edges, &what);
        let foot = out.naming.as_ref().expect("births").turn_feet[0].0;
        let b = &out.body;
        let l = b
            .edges_of_vertex(foot)
            .expect("a fan")
            .into_iter()
            .find(|e| ends(b, *e).iter().any(|v| norm(point_at(b, *v)) < 1e-12))
            .expect("L's piece runs from the turn foot to the origin");
        match fillet_edges(&at_rest(b, tol()), &[l], 0.1 * D, tol()) {
            Err(e) => assert!(
                matches!(&e.error, BlendError::UnsupportedRunOut { detail, .. }
                    if *detail == sweep::blend::battery::END_FACE_CURVED),
                "{what}: L later refuses at a curved end face, got {}",
                e.error
            ),
            Ok(_) => panic!("{what}: L later must refuse"),
        }
    }
}
