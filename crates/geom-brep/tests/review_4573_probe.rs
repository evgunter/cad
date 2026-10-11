//! Reviewer probe for PR 4573 (sphere arm): an independent oracle for
//! Gauss–Bonnet area/flux and for the pole-free side reader, on random
//! spheres (off-origin, tilted chart), random circle-arc loops.
#![allow(clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::props::{
    LoopEdge, MaterialSign, PropsError, boundary_material_sign_loops, curved_face_loops,
};
use geom_core::{Band, Point3, Sign, Tol, Vec3};

type V = [f64; 3];
fn add(a: V, b: V) -> V {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn sub(a: V, b: V) -> V {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn mul(a: V, s: f64) -> V {
    [a[0] * s, a[1] * s, a[2] * s]
}
fn dot(a: V, b: V) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V, b: V) -> V {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn norm(a: V) -> f64 {
    dot(a, a).sqrt()
}
fn unit(a: V) -> V {
    mul(a, 1.0 / norm(a))
}
fn pt(a: V) -> Point3<f64> {
    Point3::new(a[0], a[1], a[2])
}
fn vc(a: V) -> Vec3<f64> {
    Vec3::new(a[0], a[1], a[2])
}
fn perp(a: V) -> V {
    let t = if a[0].abs() < 0.6 {
        [1.0, 0.0, 0.0]
    } else {
        [0.0, 1.0, 0.0]
    };
    unit(cross(a, t))
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.next()
    }
    fn dir(&mut self) -> V {
        loop {
            let v = [
                self.range(-1.0, 1.0),
                self.range(-1.0, 1.0),
                self.range(-1.0, 1.0),
            ];
            let n = norm(v);
            if n > 0.1 && n < 1.0 {
                return mul(v, 1.0 / n);
            }
        }
    }
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// One arc as the oracle sees it, plus the LoopEdge.
struct Arc {
    edge: LoopEdge<f64>,
    samples: Vec<V>,
}

/// Arc on the circle {plane normal n through p, q} from p to q, CCW about n.
/// `flip` stores it reversed (axis −n, traversed backward).
fn arc(c: V, r: f64, p: V, q: V, n: V, flip: bool, nsamp: usize) -> Arc {
    let n = unit(n);
    let cc = add(c, mul(n, dot(sub(p, c), n)));
    let rho = norm(sub(p, cc));
    let u = unit(sub(p, cc));
    let w = cross(n, u);
    let qq = sub(q, cc);
    let mut th = dot(w, qq).atan2(dot(u, qq));
    if th <= 0.0 {
        th += std::f64::consts::TAU;
    }
    let _ = r;
    let samples: Vec<V> = (0..nsamp)
        .map(|i| {
            let t = th * i as f64 / nsamp as f64;
            add(cc, add(mul(u, rho * t.cos()), mul(w, rho * t.sin())))
        })
        .collect();
    let edge = if !flip {
        LoopEdge::hand_built(
            Curve3::Circle {
                center: pt(cc),
                axis: vc(n),
                radius: rho,
                u_ref: vc(u),
            },
            0.0,
            th,
            true,
            0,
            0,
        )
    } else {
        // axis −n, u_ref at q; from q going CCW about −n by th reaches p.
        let u2 = unit(qq);
        LoopEdge::hand_built(
            Curve3::Circle {
                center: pt(cc),
                axis: vc(mul(n, -1.0)),
                radius: rho,
                u_ref: vc(u2),
            },
            0.0,
            th,
            false,
            0,
            0,
        )
    };
    Arc { edge, samples }
}

/// Signed solid angle of triangle (a,b,c) unit vectors.
fn tri(a: V, b: V, c: V) -> f64 {
    2.0 * dot(a, cross(b, c)).atan2(1.0 + dot(a, b) + dot(b, c) + dot(c, a))
}

/// Solid angle of the region LEFT of the closed polyline (outward normal),
/// in [0, 4π).
fn left_solid(c: V, poly: &[V]) -> f64 {
    let us: Vec<V> = poly.iter().map(|p| unit(sub(*p, c))).collect();
    let r = us[0];
    let mut s = 0.0;
    for i in 1..us.len() - 1 {
        s += tri(r, us[i], us[i + 1]);
    }
    let fpi = 4.0 * std::f64::consts::PI;
    ((s % fpi) + fpi) % fpi
}

/// Is direction `pole` left of the polyline: stereographic from a point
/// q off the loop; the bounded region is left iff the image runs CCW.
fn pole_left(c: V, poly: &[V], pole: V) -> bool {
    let us: Vec<V> = poly.iter().map(|p| unit(sub(*p, c))).collect();
    // choose q farthest from the loop among a few candidates
    let cands = [
        [0.267, -0.534, 0.802],
        [-0.6, 0.64, 0.48],
        [0.8, 0.36, -0.48],
        [-0.36, -0.48, -0.8],
        [0.0, 1.0, 0.0],
        [1.0, 0.0, 0.0],
    ];
    let mut q = unit(cands[0]);
    let mut best = -1.0;
    for cd in cands {
        let cq = unit(cd);
        let dmin = us
            .iter()
            .map(|u| norm(sub(*u, cq)))
            .fold(f64::INFINITY, f64::min)
            .min(norm(sub(pole, cq)));
        if dmin > best {
            best = dmin;
            q = cq;
        }
    }
    let e1 = perp(q);
    let e2 = cross(q, e1);
    let proj = |w: V| {
        let d = 1.0 - dot(w, q);
        (dot(w, e1) / d, dot(w, e2) / d)
    };
    let img: Vec<(f64, f64)> = us.iter().map(|w| proj(*w)).collect();
    let pp = proj(pole);
    let mut wind = 0.0;
    let mut a2 = 0.0;
    for i in 0..img.len() {
        let (a, b) = (img[i], img[(i + 1) % img.len()]);
        a2 += a.0 * b.1 - a.1 * b.0;
        let a1 = (a.1 - pp.1).atan2(a.0 - pp.0);
        let b1 = (b.1 - pp.1).atan2(b.0 - pp.0);
        let mut d = b1 - a1;
        while d > std::f64::consts::PI {
            d -= std::f64::consts::TAU;
        }
        while d < -std::f64::consts::PI {
            d += std::f64::consts::TAU;
        }
        wind += d;
    }
    // projecting from q: plane oriented by -q preserves the outward orientation
    let k = (wind / std::f64::consts::TAU).round() as i64;
    let ccw = a2 < 0.0; // (e1,e2,q) right-handed => orientation by -q flips sign
    if ccw { k.abs() == 1 } else { k == 0 }
}

/// Winding of the loop about the axis through c along `ax` (azimuth sum).
fn axis_winding(c: V, poly: &[V], ax: V) -> f64 {
    let e1 = perp(ax);
    let e2 = cross(ax, e1);
    let mut w = 0.0;
    for i in 0..poly.len() {
        let a = sub(poly[i], c);
        let b = sub(poly[(i + 1) % poly.len()], c);
        let mut d = dot(b, e2).atan2(dot(b, e1)) - dot(a, e2).atan2(dot(a, e1));
        while d > std::f64::consts::PI {
            d -= std::f64::consts::TAU;
        }
        while d < -std::f64::consts::PI {
            d += std::f64::consts::TAU;
        }
        w += d;
    }
    w / std::f64::consts::TAU
}

/// Self-intersection count of a polyline loop on the sphere (pairwise
/// segment test in a gnomonic-free way: great-circle segment crossing).
fn crossings(c: V, poly: &[V]) -> usize {
    let us: Vec<V> = poly.iter().map(|p| unit(sub(*p, c))).collect();
    let n = us.len();
    let step = (n / 600).max(1);
    let pts: Vec<V> = (0..n).step_by(step).map(|i| us[i]).collect();
    let m = pts.len();
    let mut cnt = 0;
    for i in 0..m {
        let (a, b) = (pts[i], pts[(i + 1) % m]);
        let nab = cross(a, b);
        for j in i + 2..m {
            if i == 0 && j == m - 1 {
                continue;
            }
            let (cc, dd) = (pts[j], pts[(j + 1) % m]);
            let ncd = cross(cc, dd);
            if dot(nab, cc) * dot(nab, dd) < 0.0
                && dot(ncd, a) * dot(ncd, b) < 0.0
                && dot(add(a, b), add(cc, dd)) > 0.0
            {
                cnt += 1;
            }
        }
    }
    cnt
}

fn vec_area(c: V, poly: &[V]) -> V {
    let mut a = [0.0; 3];
    for i in 0..poly.len() {
        a = add(
            a,
            cross(sub(poly[i], c), sub(poly[(i + 1) % poly.len()], c)),
        );
    }
    mul(a, 0.5)
}

#[derive(Debug, Default)]
struct Tally {
    residual: u32,
    ok: u32,
    refused_right: u32,
    unenc: u32,
    holed: u32,
    bad: Vec<String>,
}

/// Run one face: loops (outer first). Checks everything vs oracle.
fn check(
    t: &mut Tally,
    label: &str,
    c: V,
    r: f64,
    axis: V,
    loops: &[Vec<Arc>],
    sense: bool,
    rev: bool,
) {
    let uref = perp(axis);
    let s = Surface::Sphere {
        center: pt(c),
        radius: r,
        axis: vc(axis),
        u_ref: vc(uref),
    };
    let mut polys: Vec<Vec<V>> = loops
        .iter()
        .map(|l| l.iter().flat_map(|a| a.samples.iter().copied()).collect())
        .collect();
    let mut edges: Vec<Vec<LoopEdge<f64>>> = loops
        .iter()
        .map(|l| l.iter().map(|a| a.edge.clone()).collect())
        .collect();
    let sense = if rev {
        for p in &mut polys {
            p.reverse();
        }
        for e in &mut edges {
            e.reverse();
            for x in e.iter_mut() {
                x.forward = !x.forward;
            }
        }
        !sense
    } else {
        sense
    };
    let refs: Vec<&[LoopEdge<f64>]> = edges.iter().map(|v| v.as_slice()).collect();
    let fpi = 4.0 * std::f64::consts::PI;
    // Oracle: region left (chart normal) of each loop.
    let lefts: Vec<f64> = polys.iter().map(|p| left_solid(c, p)).collect();
    // face under sense: left of loops under outward normal.
    let face_solid = if sense {
        lefts[0] - lefts[1..].iter().map(|h| fpi - h).sum::<f64>()
    } else {
        (fpi - lefts[0]) - lefts[1..].iter().map(|h| *h).sum::<f64>()
    };
    let mut va = [0.0; 3];
    for p in &polys {
        va = add(va, vec_area(c, p));
    }
    let sig = if sense { 1.0 } else { -1.0 };
    let want_area = face_solid * r * r;
    let want_flux = sig * r * want_area + dot(c, va);
    // Pole occupancy of the face (one-loop only matters).
    let in_face = |pole: V| {
        let l = pole_left(c, &polys[0], pole);
        if sense { l } else { !l }
    };
    let n_in = in_face(axis);
    let s_in = in_face(mul(axis, -1.0));
    if polys.iter().any(|p| crossings(c, p) > 0) || face_solid <= 0.0 {
        t.residual += 1;
        return;
    }
    let got = curved_face_loops(&s, &refs, sense, band());
    let side = boundary_material_sign_loops(&s, &refs, band());
    let want_sign = if sense {
        Sign::Positive
    } else {
        Sign::Negative
    };
    if loops.len() > 1 {
        t.holed += 1;
        let flipped = curved_face_loops(&s, &refs, !sense, band());
        if face_solid > 0.0 && !matches!(flipped, Err(PropsError::SenseInverted)) {
            t.bad
                .push(format!("{label}: holed flipped bit {flipped:?}"));
        }
        match (&got, &side) {
            (Ok(fc), Ok(MaterialSign::Encoded(sg))) if *sg == want_sign => {
                let ea = (fc.area - want_area).abs() / (r * r);
                let ef = (fc.flux - want_flux).abs() / (r * r * (r + norm(c)));
                if ea > 1e-6 || ef > 1e-6 {
                    t.bad.push(format!(
                        "{label}: holed area {} want {} flux {} want {}",
                        fc.area, want_area, fc.flux, want_flux
                    ));
                } else {
                    t.ok += 1;
                }
            }
            _ => t.bad.push(format!(
                "{label}: holed got {got:?} side {side:?} want_solid {face_solid}"
            )),
        }
        return;
    }
    let expect_refuse = n_in && s_in;
    let expect_unenc = n_in != s_in;
    match &got {
        Ok(fc) if !expect_refuse => {
            let ea = (fc.area - want_area).abs() / (r * r);
            let ef = (fc.flux - want_flux).abs() / (r * r * (r + norm(c)));
            if ea > 1e-6 || ef > 1e-6 {
                t.bad.push(format!("{label}: area {} want {} (solid {}) flux {} want {} r {r} cross {}", fc.area, want_area, face_solid, fc.flux, want_flux, crossings(c, &polys[0])));
            } else { t.ok += 1; }
            let want_side = if expect_unenc { MaterialSign::Unencoded } else { MaterialSign::Encoded(want_sign) };
            if side != Ok(want_side) { t.bad.push(format!("{label}: side {side:?} want {want_side:?}")); }
            if expect_unenc { t.unenc += 1; }
        }
        Err(PropsError::SenseContradicted) if expect_refuse => {
            t.refused_right += 1;
            let want_side = MaterialSign::Encoded(want_sign.flip());
            if side != Ok(want_side) { t.bad.push(format!("{label}: side {side:?} want {want_side:?} (refused)")); }
        }
        _ => t.bad.push(format!("{label}: got {got:?} side {side:?}; n_in {n_in} s_in {s_in} solid {face_solid} axwind {:.4} cross {}", axis_winding(c, &polys[0], axis), crossings(c, &polys[0]))),
    }
}

fn rand_sphere(g: &mut Rng) -> (V, f64, V) {
    let r = 10f64.powf(g.range(-3.0, 0.0));
    let c = mul(g.dir(), r * g.range(0.0, 5.0));
    (c, r, g.dir())
}

/// Star polygon around d of k vertices at angular radii in [lo, hi],
/// edges bulged by β.
fn star(
    g: &mut Rng,
    c: V,
    r: f64,
    d: V,
    k: usize,
    lo: f64,
    hi: f64,
    bulge: f64,
    cw: bool,
) -> Vec<Arc> {
    let e1 = perp(d);
    let e2 = cross(d, e1);
    let mut az: Vec<f64> = (0..k)
        .map(|i| (i as f64 + g.range(0.1, 0.9)) * std::f64::consts::TAU / k as f64)
        .collect();
    if cw {
        az.reverse();
    }
    let verts: Vec<V> = az
        .iter()
        .map(|a| {
            let th = g.range(lo, hi);
            let dir = add(
                mul(d, th.cos()),
                mul(add(mul(e1, a.cos()), mul(e2, a.sin())), th.sin()),
            );
            add(c, mul(dir, r))
        })
        .collect();
    (0..k)
        .map(|i| {
            let p = verts[i];
            let q = verts[(i + 1) % k];
            let ng = unit(cross(sub(p, c), sub(q, c)));
            let m = unit(add(sub(p, c), sub(q, c)));
            let b = g.range(-bulge, bulge);
            let n = add(mul(ng, b.cos()), mul(m, b.sin()));
            arc(c, r, p, q, n, g.next() < 0.5, 3000)
        })
        .collect()
}

/// Lens loop: two circles through p, q.
fn lens(g: &mut Rng, c: V, r: f64) -> Vec<Arc> {
    let p = add(c, mul(g.dir(), r));
    let q = add(c, mul(g.dir(), r));
    let pq = unit(sub(q, p));
    let mid = sub(mul(add(p, q), 0.5), c);
    let a = perp(pq);
    let b = cross(pq, a);
    let _ = mid;
    loop {
        let t1 = g.range(0.0, std::f64::consts::TAU);
        let t2 = g.range(0.0, std::f64::consts::TAU);
        let n1 = add(mul(a, t1.cos()), mul(b, t1.sin()));
        let n2 = add(mul(a, t2.cos()), mul(b, t2.sin()));
        // keep planes intersecting the sphere well (|n·(p−c)| < 0.95 r) and distinct
        if dot(n1, sub(p, c)).abs() > 0.97 * r || dot(n2, sub(p, c)).abs() > 0.97 * r {
            continue;
        }
        if norm(cross(n1, n2)) < 0.05 {
            continue;
        }
        return vec![
            arc(c, r, p, q, n1, g.next() < 0.5, 40000),
            arc(c, r, q, p, n2, g.next() < 0.5, 40000),
        ];
    }
}

#[test]
fn review_4573_gb_oracle_sweep() {
    let mut g = Rng(0x9E3779B97F4A7C15);
    let mut t = Tally::default();
    for i in 0..1500 {
        let (c, r, axis) = rand_sphere(&mut g);
        let sense = g.next() < 0.5;
        // Bias chart axis near the loop centre sometimes.
        let d = g.dir();
        let ax = if i % 3 == 0 {
            unit(add(d, mul(g.dir(), 0.3)))
        } else {
            axis
        };
        let k = 3 + (g.next() * 5.0) as usize;
        let l = star(&mut g, c, r, d, k, 0.2, 1.4, 0.3, false);
        for rev in [false, true] {
            check(
                &mut t,
                &format!("star#{i}/{rev}"),
                c,
                r,
                ax,
                &[l.iter()
                    .map(|a| Arc {
                        edge: a.edge.clone(),
                        samples: a.samples.clone(),
                    })
                    .collect::<Vec<_>>()],
                sense,
                rev,
            );
        }
        let l = lens(&mut g, c, r);
        for rev in [false, true] {
            check(
                &mut t,
                &format!("lens#{i}/{rev}"),
                c,
                r,
                ax,
                &[l.iter()
                    .map(|a| Arc {
                        edge: a.edge.clone(),
                        samples: a.samples.clone(),
                    })
                    .collect::<Vec<_>>()],
                sense,
                rev,
            );
        }
        // holed: outer star around d, ring small star around d (CW).
        let outer = star(&mut g, c, r, d, k, 0.6, 1.4, 0.15, false);
        let ring = star(&mut g, c, r, d, 4, 0.1, 0.3, 0.1, true);
        for rev in [false, true] {
            check(
                &mut t,
                &format!("holed#{i}/{rev}"),
                c,
                r,
                ax,
                &[
                    outer
                        .iter()
                        .map(|a| Arc {
                            edge: a.edge.clone(),
                            samples: a.samples.clone(),
                        })
                        .collect::<Vec<_>>(),
                    ring.iter()
                        .map(|a| Arc {
                            edge: a.edge.clone(),
                            samples: a.samples.clone(),
                        })
                        .collect::<Vec<_>>(),
                ],
                true,
                rev,
            );
        }
    }
    eprintln!(
        "residual(non-simple) {} ok {} refused_right {} unenc {} holed {} bad {}",
        t.residual,
        t.ok,
        t.refused_right,
        t.unenc,
        t.holed,
        t.bad.len()
    );
    for b in t.bad.iter().take(30) {
        eprintln!("{b}");
    }
    assert!(t.bad.is_empty());
}

fn cone_edges(apex: V, a: V, uref: V, al: f64, rect: &[(f64, f64)]) -> Vec<LoopEdge<f64>> {
    // rect: polygon in (u, v) chart with axis-aligned edges; edges are rims (v const) or generators (u const).
    let w = cross(a, uref);
    let rad = |u: f64| add(mul(uref, u.cos()), mul(w, u.sin()));
    let n = rect.len();
    (0..n)
        .filter(|&i| !(rect[i].1 == 0.0 && rect[(i + 1) % n].1 == 0.0))
        .map(|i| {
            let (u0, v0) = rect[i];
            let (u1, v1) = rect[(i + 1) % n];
            if (v0 - v1).abs() < 1e-15 {
                let v = v0;
                let c = add(apex, mul(a, v * al.cos()));
                let rho = v.abs() * al.sin();
                let off = if v > 0.0 { 0.0 } else { std::f64::consts::PI };
                let (lo, hi, fwd) = if u1 > u0 {
                    (u0, u1, true)
                } else {
                    (u1, u0, false)
                };
                LoopEdge::hand_built(
                    Curve3::Circle {
                        center: pt(c),
                        axis: vc(a),
                        radius: rho,
                        u_ref: vc(uref),
                    },
                    lo + off,
                    hi + off,
                    fwd,
                    0,
                    0,
                )
            } else {
                let d = add(mul(a, al.cos()), mul(rad(u0), al.sin()));
                let (lo, hi, fwd) = if v1 > v0 {
                    (v0, v1, true)
                } else {
                    (v1, v0, false)
                };
                LoopEdge::hand_built(
                    Curve3::Line {
                        origin: pt(apex),
                        dir: vc(d),
                    },
                    lo,
                    hi,
                    fwd,
                    0,
                    0,
                )
            }
        })
        .collect()
}

#[test]
fn review_4573_cone_gate_sweep() {
    let mut g = Rng(0xABCDEF);
    let mut bad = Vec::new();
    let mut n = 0;
    for i in 0..400 {
        let apex = mul(g.dir(), g.range(0.0, 3.0));
        let a = g.dir();
        let uref = perp(a);
        let al = g.range(0.1, 1.4);
        let s = Surface::Cone {
            apex: pt(apex),
            axis: vc(a),
            half_angle: al,
            u_ref: vc(uref),
        };
        let nappe = if i % 2 == 0 { 1.0 } else { -1.0 };
        let near = i % 5 == 0;
        let (va, vb) = if near {
            (g.range(1e-6, 1e-3), g.range(0.5, 2.0))
        } else {
            (g.range(0.1, 1.0), g.range(1.1, 3.0))
        };
        let (v0, v1) = if nappe > 0.0 { (va, vb) } else { (-vb, -va) };
        let u0 = g.range(-3.0, 3.0);
        let u1 = u0 + g.range(0.2, 6.0);
        let um = 0.5 * (u0 + u1);
        let vm = 0.5 * (v0 + v1);
        // CCW rectangle and CCW L-shape in (u, v)
        let shapes: Vec<Vec<(f64, f64)>> = vec![
            vec![(u0, v0), (u1, v0), (u1, v1), (u0, v1)],
            vec![(u0, v0), (u1, v0), (u1, vm), (um, vm), (um, v1), (u0, v1)],
            if nappe > 0.0 {
                vec![(u0, 0.0), (u1, 0.0), (u1, vb), (u0, vb)]
            } else {
                vec![(u0, -va), (u1, -va), (u1, 0.0), (u0, 0.0)]
            },
        ];
        for (k, sh) in shapes.iter().enumerate() {
            let e = cone_edges(apex, a, uref, al, sh);
            for rot in 0..e.len() {
                let mut r: Vec<_> = e.clone();
                r.rotate_left(rot);
                for rev in [false, true] {
                    let mut rr = r.clone();
                    if rev {
                        rr.reverse();
                        for x in rr.iter_mut() {
                            x.forward = !x.forward;
                        }
                    }
                    let want = if rev { Sign::Negative } else { Sign::Positive };
                    n += 1;
                    let got = boundary_material_sign_loops(&s, &[&rr[..]], band());
                    if got != Ok(MaterialSign::Encoded(want)) {
                        bad.push(format!(
                            "cone#{i} shape{k} rot{rot} rev{rev} nappe{nappe} near{near}: {got:?}"
                        ));
                    }
                }
            }
        }
    }
    eprintln!("cone gate cases {n} bad {}", bad.len());
    for b in bad.iter().take(20) {
        eprintln!("{b}");
    }
    assert!(bad.is_empty());
}

#[test]
fn review_4573_pole_vertex_polygons() {
    // Polygons with one vertex AT the chart's north pole.
    let mut g = Rng(0x5555);
    let mut t = Tally::default();
    for i in 0..400 {
        let (c, r, axis) = rand_sphere(&mut g);
        let sense = g.next() < 0.5;
        let d = unit(add(axis, mul(g.dir(), 0.9)));
        let k = 3 + (g.next() * 4.0) as usize;
        // build a star around d then splice the north pole in as an extra vertex
        let e1 = perp(d);
        let e2 = cross(d, e1);
        let mut verts: Vec<V> = (0..k)
            .map(|j| {
                let a = (j as f64 + g.range(0.1, 0.9)) * std::f64::consts::TAU / k as f64;
                let th = g.range(0.3, 1.2);
                add(
                    c,
                    mul(
                        add(
                            mul(d, th.cos()),
                            mul(add(mul(e1, a.cos()), mul(e2, a.sin())), th.sin()),
                        ),
                        r,
                    ),
                )
            })
            .collect();
        // insert pole between the two vertices whose azimuth brackets the pole's
        let pa = {
            let p = sub(axis, mul(d, dot(axis, d)));
            dot(p, e2)
                .atan2(dot(p, e1))
                .rem_euclid(std::f64::consts::TAU)
        };
        let idx = (0..k)
            .find(|&j| {
                let a = {
                    let p = sub(sub(verts[j], c), mul(d, dot(sub(verts[j], c), d)));
                    dot(p, e2)
                        .atan2(dot(p, e1))
                        .rem_euclid(std::f64::consts::TAU)
                };
                a > pa
            })
            .unwrap_or(k);
        verts.insert(idx, add(c, mul(axis, r)));
        let n = verts.len();
        let l: Vec<Arc> = (0..n)
            .map(|j| {
                let p = verts[j];
                let q = verts[(j + 1) % n];
                let ng = unit(cross(sub(p, c), sub(q, c)));
                let m = unit(add(sub(p, c), sub(q, c)));
                let b = g.range(-0.2, 0.2);
                arc(
                    c,
                    r,
                    p,
                    q,
                    add(mul(ng, b.cos()), mul(m, b.sin())),
                    g.next() < 0.5,
                    400,
                )
            })
            .collect();
        // Oracle: north pole on loop ⇒ only south decides.
        let polys: Vec<V> = l.iter().flat_map(|a| a.samples.iter().copied()).collect();
        let s_left = pole_left(c, &polys, mul(axis, -1.0));
        let s_in = if sense { s_left } else { !s_left };
        if crossings(c, &polys) > 0 {
            t.residual += 1;
            continue;
        }
        let uref = perp(axis);
        let s = Surface::Sphere {
            center: pt(c),
            radius: r,
            axis: vc(axis),
            u_ref: vc(uref),
        };
        let edges: Vec<LoopEdge<f64>> = l.iter().map(|a| a.edge.clone()).collect();
        let got = curved_face_loops(&s, &[&edges[..]], sense, band());
        let side = boundary_material_sign_loops(&s, &[&edges[..]], band());
        let want_sign = if sense {
            Sign::Positive
        } else {
            Sign::Negative
        };
        let fpi = 4.0 * std::f64::consts::PI;
        let left = left_solid(c, &polys);
        let solid = if sense { left } else { fpi - left };
        match (&got, s_in) {
            (Ok(fc), false) => {
                if (fc.area / (r * r) - solid).abs() > 1e-6 {
                    t.bad.push(format!(
                        "pv#{i}: area {} want {} cross {} axwind {}",
                        fc.area / (r * r),
                        solid,
                        crossings(c, &polys),
                        axis_winding(c, &polys, axis)
                    ));
                }
                if side != Ok(MaterialSign::Encoded(want_sign)) {
                    t.bad.push(format!("pv#{i}: side {side:?}"));
                }
                t.ok += 1;
            }
            (Err(PropsError::SenseContradicted), true) => t.refused_right += 1,
            _ => t
                .bad
                .push(format!("pv#{i}: got {got:?} side {side:?} s_in {s_in}")),
        }
    }
    eprintln!(
        "pole-vertex ok {} refused {} bad {}",
        t.ok,
        t.refused_right,
        t.bad.len()
    );
    for b in t.bad.iter().take(20) {
        eprintln!("{b}");
    }
    assert!(t.bad.is_empty());
}

#[test]
fn review_4573_oracle_selfcheck() {
    let c = [0.0; 3];
    // CCW about +z (viewed from outside at north) small circle near north pole
    let poly: Vec<V> = (0..100)
        .map(|i| {
            let t = i as f64 * std::f64::consts::TAU / 100.0;
            [0.1 * t.cos(), 0.1 * t.sin(), (1.0f64 - 0.01).sqrt()]
        })
        .collect();
    eprintln!(
        "north in left: {} south in left: {} left solid {}",
        pole_left(c, &poly, [0.0, 0.0, 1.0]),
        pole_left(c, &poly, [0.0, 0.0, -1.0]),
        left_solid(c, &poly)
    );
    let s = Surface::Sphere {
        center: pt(c),
        radius: 1.0,
        axis: vc([0.0, 0.0, 1.0]),
        u_ref: vc([1.0, 0.0, 0.0]),
    };
    let h = (1.0f64 - 0.01).sqrt();
    let e = LoopEdge::hand_built(
        Curve3::Circle {
            center: pt([0.0, 0.0, h]),
            axis: vc([0.0, 0.0, 1.0]),
            radius: 0.1,
            u_ref: vc([1.0, 0.0, 0.0]),
        },
        0.0,
        std::f64::consts::TAU,
        true,
        0,
        0,
    );
    eprintln!(
        "kernel side: {:?}",
        boundary_material_sign_loops(&s, &[&[e][..]], band())
    );
    // tilted: same circle about +x
    let e = LoopEdge::hand_built(
        Curve3::Circle {
            center: pt([h, 0.0, 0.0]),
            axis: vc([1.0, 0.0, 0.0]),
            radius: 0.1,
            u_ref: vc([0.0, 1.0, 0.0]),
        },
        0.0,
        std::f64::consts::TAU,
        true,
        0,
        0,
    );
    eprintln!(
        "kernel side tilted cap: {:?} area {:?}",
        boundary_material_sign_loops(&s, &[&[e.clone()][..]], band()),
        curved_face_loops(&s, &[&[e][..]], true, band())
    );
    let poly: Vec<V> = (0..100)
        .map(|i| {
            let t = i as f64 * std::f64::consts::TAU / 100.0;
            [h, 0.1 * t.cos(), 0.1 * t.sin()]
        })
        .collect();
    eprintln!(
        "oracle tilted: N {} S {} solid {}",
        pole_left(c, &poly, [0.0, 0.0, 1.0]),
        pole_left(c, &poly, [0.0, 0.0, -1.0]),
        left_solid(c, &poly)
    );
}

/// The disclosed non-simple residual: a figure-8 loop (two whole great
/// circles crossing at X, one loop through X twice) is measured, not refused.
#[test]
fn review_4573_figure_eight_is_measured_silently() {
    let c = [0.0; 3];
    let s = Surface::Sphere {
        center: pt(c),
        radius: 1.0,
        axis: vc([0.0, 0.0, 1.0]),
        u_ref: vc([1.0, 0.0, 0.0]),
    };
    let x = [1.0, 0.0, 0.0];
    let tau = std::f64::consts::TAU;
    let na = unit([0.0, 0.3, 1.0]);
    let nb = unit([0.0, -0.3, 1.0]);
    let a = LoopEdge::hand_built(
        Curve3::Circle {
            center: pt(c),
            axis: vc(na),
            radius: 1.0,
            u_ref: vc(x),
        },
        0.0,
        tau,
        true,
        0,
        0,
    );
    let b = LoopEdge::hand_built(
        Curve3::Circle {
            center: pt(c),
            axis: vc(nb),
            radius: 1.0,
            u_ref: vc(x),
        },
        0.0,
        tau,
        true,
        0,
        0,
    );
    let got = curved_face_loops(&s, &[&[a.clone(), b.clone()][..]], true, band());
    let side = boundary_material_sign_loops(&s, &[&[a, b][..]], band());
    eprintln!("figure-eight: {got:?} side {side:?}");
    assert!(got.is_ok(), "the residual is a measured number: {got:?}");
}

/// A non-crossing pinch (one loop through X twice: two geodesic triangles
/// meeting only at X, both left of the traversal): GB vs the two areas.
#[test]
fn review_4573_pinch() {
    let mut g = Rng(0x777);
    let mut worst: f64 = 0.0;
    let mut bad = Vec::new();
    for i in 0..200 {
        let (c, r, axis) = rand_sphere(&mut g);
        let x = g.dir();
        let e1 = perp(x);
        let e2 = cross(x, e1);
        let at = |az: f64, th: f64| {
            add(
                c,
                mul(
                    add(
                        mul(x, th.cos()),
                        mul(add(mul(e1, az.cos()), mul(e2, az.sin())), th.sin()),
                    ),
                    r,
                ),
            )
        };
        let xp = add(c, mul(x, r));
        // lobe 1 in azimuth sector around 0, lobe 2 around π (opposite), both CCW about x
        let s1 = g.range(0.2, 1.0);
        let s2 = g.range(0.2, 1.0);
        let (a1, b1) = (at(-s1, g.range(0.3, 1.0)), at(s1, g.range(0.3, 1.0)));
        let (a2, b2) = (
            at(std::f64::consts::PI - s2, g.range(0.3, 1.0)),
            at(std::f64::consts::PI + s2, g.range(0.3, 1.0)),
        );
        let gc = |p: V, q: V| arc(c, r, p, q, cross(sub(p, c), sub(q, c)), false, 4000);
        let l = vec![
            gc(xp, a1),
            gc(a1, b1),
            gc(b1, xp),
            gc(xp, a2),
            gc(a2, b2),
            gc(b2, xp),
        ];
        let p1: Vec<V> = l[..3]
            .iter()
            .flat_map(|a| a.samples.iter().copied())
            .collect();
        let p2: Vec<V> = l[3..]
            .iter()
            .flat_map(|a| a.samples.iter().copied())
            .collect();
        let want = (left_solid(c, &p1) + left_solid(c, &p2)) * r * r;
        let s = Surface::Sphere {
            center: pt(c),
            radius: r,
            axis: vc(axis),
            u_ref: vc(perp(axis)),
        };
        let edges: Vec<LoopEdge<f64>> = l.iter().map(|a| a.edge.clone()).collect();
        let pin = [axis, mul(axis, -1.0)]
            .iter()
            .any(|&q| pole_left(c, &p1, q) || pole_left(c, &p2, q));
        if pin {
            eprintln!(
                "pinch#{i}: pole inside a lobe -> {:?} side {:?}",
                curved_face_loops(&s, &[&edges[..]], true, band()).map(|f| f.area),
                boundary_material_sign_loops(&s, &[&edges[..]], band())
            );
        }
        match curved_face_loops(&s, &[&edges[..]], true, band()) {
            Ok(fc) => {
                let e = (fc.area - want).abs() / (r * r);
                worst = worst.max(e);
                if e > 1e-5 {
                    bad.push(format!(
                        "pinch#{i}: area/R² {} want {}",
                        fc.area / (r * r),
                        want / (r * r)
                    ));
                }
            }
            Err(e) => bad.push(format!("pinch#{i}: {e:?}")),
        }
    }
    eprintln!("pinch worst {worst:.3e} bad {}", bad.len());
    for b in bad.iter().take(10) {
        eprintln!("  {b}");
    }
}

/// Overlapping rings (the same hole twice, holes summing past the region):
/// sense=true vs the reverted face (loops reversed, sense=false).
#[test]
fn review_4573_overlapping_rings_asymmetry() {
    let c = [0.0; 3];
    let s = Surface::Sphere {
        center: pt(c),
        radius: 1.0,
        axis: vc([0.0, 0.0, 1.0]),
        u_ref: vc([1.0, 0.0, 0.0]),
    };
    let tau = std::f64::consts::TAU;
    let cap = |th: f64, fwd: bool| {
        LoopEdge::hand_built(
            Curve3::Circle {
                center: pt([th.cos(), 0.0, 0.0]),
                axis: vc([1.0, 0.0, 0.0]),
                radius: th.sin(),
                u_ref: vc([0.0, 1.0, 0.0]),
            },
            0.0,
            tau,
            fwd,
            0,
            0,
        )
    };
    let (o, h) = (cap(1.0, true), cap(0.8, false));
    let fwd = curved_face_loops(
        &s,
        &[&[o.clone()][..], &[h.clone()][..], &[h.clone()][..]],
        true,
        band(),
    );
    let (or, hr) = (cap(1.0, false), cap(0.8, true));
    let rev = curved_face_loops(
        &s,
        &[&[or][..], &[hr.clone()][..], &[hr][..]],
        false,
        band(),
    );
    eprintln!("overlap: sense=true {fwd:?}\n         reverted {rev:?}");
}
