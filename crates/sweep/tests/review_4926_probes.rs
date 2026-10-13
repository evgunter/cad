//! Review probes for PR 4926 (a chamfer's non-isosceles turn builds its
//! overrun). Oracle: each convex body is written down as its own
//! analytic half-spaces (not read off the body), each chamfer adds the
//! half-space through its two trimlines, and the volume is a big cube
//! CLIPPED plane by plane (Sutherland–Hodgman per face plus a cap
//! polygon), summed by the divergence theorem — no vertex enumeration.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::type_complexity
)]

use geom_core::{Bounds, Point2, Point3, Vec3};
use sweep::blend::BlendError;
use sweep::blend::build::{chamfer_edges, fillet_edges};
use sweep::test_support::{
    assert_naming_totality, at_rest, corners, prism_on, realized, sketch_from_axes,
};
use topo::boolean::BooleanOp;
use topo::{Body, ContactRecords, EdgeKey, VertexKey, validate_geometric, validate_pseudomanifold};

use crate::band_planar_cut_off::{tol, volume};

type V = [f64; 3];
/// `{x : n·x ≤ c}`.
type Half = (V, f64);

fn add(a: V, b: V) -> V {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn sub(a: V, b: V) -> V {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn mul(a: V, k: f64) -> V {
    [a[0] * k, a[1] * k, a[2] * k]
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

/// A half-space through `p` with outward normal along `n`.
fn half(n: V, p: V) -> Half {
    let n = unit(n);
    (n, dot(n, p))
}

/// Clip a polytope given as outward-oriented polygons by `{n·x ≤ c}`.
fn clip(polys: Vec<Vec<V>>, (n, c): Half) -> Vec<Vec<V>> {
    let mut out = Vec::new();
    let mut cut: Vec<V> = Vec::new();
    for poly in polys {
        let mut kept = Vec::new();
        for i in 0..poly.len() {
            let (p, q) = (poly[i], poly[(i + 1) % poly.len()]);
            let (dp, dq) = (dot(n, p) - c, dot(n, q) - c);
            if dp <= 0.0 {
                kept.push(p);
            }
            if (dp < 0.0 && dq > 0.0) || (dp > 0.0 && dq < 0.0) {
                let x = add(p, mul(sub(q, p), dp / (dp - dq)));
                kept.push(x);
                cut.push(x);
            } else if dp == 0.0 {
                cut.push(p);
            }
        }
        if kept.len() >= 3 {
            out.push(kept);
        }
    }
    if cut.len() >= 3 {
        let mid = mul(
            cut.iter().fold([0.0; 3], |s, p| add(s, *p)),
            1.0 / cut.len() as f64,
        );
        let far = cut
            .iter()
            .copied()
            .max_by(|a, b| norm(sub(*a, mid)).total_cmp(&norm(sub(*b, mid))))
            .unwrap();
        let u = unit(sub(far, mid));
        let w = cross(n, u);
        let mut ring: Vec<(f64, V)> = cut
            .iter()
            .map(|p| (dot(sub(*p, mid), w).atan2(dot(sub(*p, mid), u)), *p))
            .collect();
        ring.sort_by(|a, b| a.0.total_cmp(&b.0));
        out.push(ring.into_iter().map(|x| x.1).collect());
    }
    out
}

/// The volume of `⋂ halves`, a big cube clipped plane by plane.
fn clipped_volume(halves: &[Half]) -> f64 {
    let r = 1.0e4;
    let c = |x: f64, y: f64, z: f64| [x * r, y * r, z * r];
    let mut polys = vec![
        vec![
            c(-1., -1., -1.),
            c(-1., 1., -1.),
            c(1., 1., -1.),
            c(1., -1., -1.),
        ],
        vec![
            c(-1., -1., 1.),
            c(1., -1., 1.),
            c(1., 1., 1.),
            c(-1., 1., 1.),
        ],
        vec![
            c(-1., -1., -1.),
            c(1., -1., -1.),
            c(1., -1., 1.),
            c(-1., -1., 1.),
        ],
        vec![
            c(-1., 1., -1.),
            c(-1., 1., 1.),
            c(1., 1., 1.),
            c(1., 1., -1.),
        ],
        vec![
            c(-1., -1., -1.),
            c(-1., -1., 1.),
            c(-1., 1., 1.),
            c(-1., 1., -1.),
        ],
        vec![
            c(1., -1., -1.),
            c(1., 1., -1.),
            c(1., 1., 1.),
            c(1., -1., 1.),
        ],
    ];
    for h in halves {
        polys = clip(polys, *h);
    }
    polys
        .iter()
        .map(|p| {
            (1..p.len() - 1)
                .map(|i| dot(p[0], cross(p[i], p[i + 1])) / 6.0)
                .sum::<f64>()
        })
        .sum()
}

/// The kept side of a chamfer of the edge `a`–`b`, which lies on the
/// faces with outward normals `na`, `nb`: the plane through the two
/// lines `d` in from the edge on each face.
fn chamfer_plane(na: V, nb: V, a: V, b: V, d: f64) -> Half {
    let tau = unit(sub(b, a));
    let into = |n: V, other: V| {
        let w = unit(cross(n, tau));
        if dot(w, other) > 0.0 { mul(w, -1.0) } else { w }
    };
    let (xa, xb) = (add(a, mul(into(na, nb), d)), add(a, mul(into(nb, na), d)));
    let m = unit(cross(sub(xa, xb), tau));
    let m = if dot(m, sub(a, xa)) > 0.0 {
        m
    } else {
        mul(m, -1.0)
    };
    // `m` points at the edge, which is on the cut side: keep the other.
    (m, dot(m, xa))
}

fn corner_of(h: [Half; 3]) -> V {
    let [(a, ca), (b, cb), (c, cc)] = h;
    let det = dot(a, cross(b, c));
    mul(
        add(
            add(mul(cross(b, c), ca), mul(cross(c, a), cb)),
            mul(cross(a, b), cc),
        ),
        1.0 / det,
    )
}

fn point_at(body: &Body<f64>, v: VertexKey) -> V {
    let p = body
        .get_point(body.get_vertex(v).expect("a vertex").point)
        .expect("a point");
    [p.x, p.y, p.z]
}

fn ends(body: &Body<f64>, e: EdgeKey) -> [VertexKey; 2] {
    let he = body.get_edge(e).expect("an edge").he_plus;
    [
        body.get_half_edge(he).expect("a half").start,
        body.half_edge_end(he).expect("an end"),
    ]
}

/// The edge whose ends lie within `1e-7` of `a` and `b`.
fn edge_near(body: &Body<f64>, a: V, b: V) -> EdgeKey {
    let near = |v, w: V| norm(sub(point_at(body, v), w)) < 1e-7 * (1.0 + norm(w));
    topo::query::all_edges(body)
        .into_iter()
        .find(|&e| {
            let [p, q] = ends(body, e);
            (near(p, a) && near(q, b)) || (near(p, b) && near(q, a))
        })
        .unwrap_or_else(|| panic!("an edge between {a:?} and {b:?}"))
}

/// A requested edge as the two faces (indices into the body's halves)
/// it lies on and its two ends' third faces.
#[derive(Clone, Copy)]
struct Req {
    faces: [usize; 2],
    ends: [usize; 2],
}

fn req(f: [usize; 2], e: [usize; 2]) -> Req {
    Req { faces: f, ends: e }
}

/// A convex polytope fixture: the body and its own analytic half-spaces.
struct Poly {
    body: Body<f64>,
    halves: Vec<Half>,
}

impl Poly {
    fn edge(&self, r: Req) -> (EdgeKey, V, V) {
        let h = |i: usize| self.halves[i];
        let a = corner_of([h(r.faces[0]), h(r.faces[1]), h(r.ends[0])]);
        let b = corner_of([h(r.faces[0]), h(r.faces[1]), h(r.ends[1])]);
        (edge_near(&self.body, a, b), a, b)
    }
    fn chamfered_volume(&self, reqs: &[Req], d: f64) -> f64 {
        let mut all = self.halves.clone();
        for r in reqs {
            let (_, a, b) = self.edge(*r);
            all.push(chamfer_plane(
                self.halves[r.faces[0]].0,
                self.halves[r.faces[1]].0,
                a,
                b,
                d,
            ));
        }
        clipped_volume(&all)
    }
}

/// The leaning prism, generalised: profile `(0,0) (2,0) (2,h) (s,h)` in
/// `xz`, extruded from `y = 0` to `y = −1.5`, all scaled by `k`.
/// Halves: 0 bottom, 1 right, 2 top, 3 leaning wall, 4 front (`y = 0`),
/// 5 back.
fn leaning(s: f64, h: f64, k: f64) -> Poly {
    let plane = sketch_from_axes(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        tol(),
    );
    let body = prism_on(
        plane,
        corners(&[(0.0, 0.0), (2.0 * k, 0.0), (2.0 * k, h * k), (s * k, h * k)]),
        1.5 * k,
        tol(),
    );
    let halves = vec![
        half([0., 0., -1.], [0., 0., 0.]),
        half([1., 0., 0.], [2. * k, 0., 0.]),
        half([0., 0., 1.], [0., 0., h * k]),
        half([-h, 0., s], [0., 0., 0.]),
        half([0., 1., 0.], [0., 0., 0.]),
        half([0., -1., 0.], [0., -1.5 * k, 0.]),
    ];
    Poly { body, halves }
}

/// A generic hexahedron: the leaning prism (`x ≤ x1`) with its top
/// tilted (`z ≤ h + a·x`), its back at `y = −depth`, intersected with a
/// slab whose front wall `y ≤ b·z + c·x` leans and yaws. Halves: 0
/// bottom, 1 right, 2 top, 3 leaning wall, 4 front, 5 back. `c` makes
/// the top face's angle at the turn vertex obtuse or acute.
#[derive(Clone, Copy)]
struct Hex {
    s: f64,
    h: f64,
    a: f64,
    b: f64,
    c: f64,
    x1: f64,
    depth: f64,
}

const HEX: Hex = Hex {
    s: 0.4,
    h: 1.0,
    a: 0.2,
    b: 0.3,
    c: 0.0,
    x1: 2.0,
    depth: 1.5,
};

fn hexahedron(x: Hex) -> Poly {
    let Hex {
        s,
        h,
        a,
        b,
        c,
        x1,
        depth,
    } = x;
    let top_at = |x: f64| h + a * x;
    let pa = prism_on(
        sketch_from_axes(
            Point3::new(0.0, 6.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            tol(),
        ),
        corners(&[(0.0, 0.0), (x1, 0.0), (x1, top_at(x1)), (s, top_at(s))]),
        6.0 + depth,
        tol(),
    );
    let n_f = unit([-c, 1.0, -b]);
    let w = unit(cross(n_f, [1.0, 0.0, 0.0]));
    let v = cross(w, n_f);
    let o = mul(w, -20.0);
    let pb = prism_on(
        sketch_from_axes(
            Point3::new(o[0], o[1], o[2]),
            Vec3::new(n_f[0], n_f[1], n_f[2]),
            Vec3::new(v[0], v[1], v[2]),
            tol(),
        ),
        corners(&[(-20.0, -20.0), (0.0, -20.0), (0.0, 20.0), (-20.0, 20.0)]),
        40.0,
        tol(),
    );
    let body = realized(BooleanOp::Intersect, &pa, &pb, tol());
    let wall_dir = [s, 0.0, top_at(s)];
    let wall_n = cross(wall_dir, [0.0, 1.0, 0.0]);
    let wall_n = if dot(wall_n, [1.0, 0.0, 0.1]) > 0.0 {
        mul(wall_n, -1.0)
    } else {
        wall_n
    };
    let halves = vec![
        half([0., 0., -1.], [0., 0., 0.]),
        half([1., 0., 0.], [x1, 0., 0.]),
        half([-a, 0., 1.], [0., 0., h]),
        half(wall_n, [0., 0., 0.]),
        half(n_f, [0., 0., 0.]),
        half([0., -1., 0.], [0., -depth, 0.]),
    ];
    Poly { body, halves }
}

/// The top face's angle at the turn vertex, degrees.
fn theta_s(p: &Poly) -> f64 {
    let h = |i: usize| p.halves[i];
    let v = corner_of([h(2), h(3), h(4)]);
    let e1 = sub(corner_of([h(2), h(4), h(1)]), v);
    let e2 = sub(corner_of([h(2), h(3), h(5)]), v);
    (dot(e1, e2) / norm(e1) / norm(e2)).acos().to_degrees()
}

/// What a chamfer did.
#[derive(Debug)]
enum Got {
    /// Built; `ΔV − oracle` and the count of non-trivalent vertices.
    Built {
        err: f64,
        non_trivalent: usize,
    },
    Refused(String),
    Panicked(String),
    /// The fixture is not the hexahedron the halves describe.
    NoFixture,
}

fn kind(e: &BlendError) -> String {
    let s = format!("{e:?}");
    s.split([' ', '{', '(']).next().unwrap_or("").to_string()
        + match e {
            BlendError::UnsupportedRunOut { detail, .. } => {
                if detail.contains("trimline meets the end face's plane past its rim") {
                    "/FOOT_INSIDE_A_FACE"
                } else if detail.contains("fillet band") {
                    "/TURN_OVERRUN"
                } else {
                    "/other"
                }
            }
            _ => "",
        }
}

fn untyped(s: &str) -> bool {
    ["SurgeryInvariant", "BodyNotIntact", "Op", "Join", "Certify"]
        .iter()
        .any(|k| s.starts_with(k))
}

/// Chamfer `edges` at `d` with tiers 3/3′ and naming checked, and
/// `ΔV` against `want_removed` (convex) — `sign = −1` for a fill.
fn chamfer_probe(body: &Body<f64>, edges: &[EdgeKey], d: f64, want: f64, sign: f64) -> Got {
    let run = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        chamfer_edges(&at_rest(body, tol()), edges, d, tol())
    }));
    match run {
        Err(p) => Got::Panicked(
            p.downcast_ref::<String>()
                .cloned()
                .or_else(|| p.downcast_ref::<&str>().map(|s| (*s).to_string()))
                .unwrap_or_default(),
        ),
        Ok(Err(e)) => Got::Refused(kind(&e.error)),
        Ok(Ok(out)) => {
            if let Err(e) = validate_geometric(&out.body, tol()) {
                return Got::Panicked(format!("tier 3: {e:?}"));
            }
            if let Err(e) = validate_pseudomanifold(&out.body, &ContactRecords::default(), tol()) {
                return Got::Panicked(format!("tier 3′: {e:?}"));
            }
            assert_naming_totality(body, &out, edges, "probe");
            let non_trivalent = out
                .body
                .vertices()
                .filter(|(v, _)| out.body.edges_of_vertex(*v).expect("fan").len() != 3)
                .count();
            Got::Built {
                err: sign * (volume(body) - volume(&out.body)) - want,
                non_trivalent,
            }
        }
    }
}

fn convex(p: &Poly, reqs: &[Req], d: f64) -> Got {
    let found = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        reqs.iter().map(|r| p.edge(*r).0).collect::<Vec<EdgeKey>>()
    }));
    let Ok(edges) = found else {
        return Got::NoFixture;
    };
    let want = clipped_volume(&p.halves) - p.chamfered_volume(reqs, d);
    chamfer_probe(&p.body, &edges, d, want, 1.0)
}

/// Built exactly, every vertex trivalent; or refused typed.
fn sound(what: &str, got: &Got, scale: f64) -> bool {
    match got {
        Got::Built { err, non_trivalent } => {
            // a mitre's common foot is valence 4 by design: callers
            // that hold only overruns check `non_trivalent` themselves
            let _ = non_trivalent;
            err.abs() < 1e-10 * scale.powi(3).max(1.0) || {
                eprintln!("{what}: built WRONG {got:?}");
                false
            }
        }
        Got::Refused(k) => {
            !untyped(k) || {
                eprintln!("{what}: refused UNTYPED {k}");
                false
            }
        }
        Got::Panicked(m) => {
            eprintln!("{what}: PANIC {m}");
            false
        }
        Got::NoFixture => true,
    }
}

const TURN: [Req; 2] = [
    // top ∩ front, from the leaning wall to the right face
    Req {
        faces: [2, 4],
        ends: [3, 1],
    },
    // top ∩ leaning wall, from the front to the back
    Req {
        faces: [2, 3],
        ends: [4, 5],
    },
];

/// **Claim 1**: the witnesses at scales `1e-2…1e2`, both edge orders,
/// against the clip oracle.
#[test]
fn p1_witness_turn_at_scales_and_orders() {
    let mut bad = 0;
    for s in [-1.0, -0.5, 0.5, 1.0] {
        for k in [0.01, 1.0, 100.0] {
            let p = leaning(s, 1.0, k);
            for order in [[0, 1], [1, 0]] {
                let reqs = [TURN[order[0]], TURN[order[1]]];
                let got = convex(&p, &reqs, 0.1 * k);
                eprintln!("leaning s={s} k={k} order={order:?}: {got:?}");
                bad += usize::from(
                    !matches!(
                        got,
                        Got::Built {
                            non_trivalent: 0,
                            ..
                        }
                    ) || !sound("w", &got, k),
                );
            }
        }
    }
    assert_eq!(bad, 0);
}

/// **Claim 1**: a generic hexahedron corner (no right angles), both
/// orders, several setbacks.
#[test]
fn p2_generic_hexahedron_turn() {
    let mut bad = 0;
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
        eprintln!("θ_S {}", theta_s(&p));
        for d in [0.05, 0.1, 0.2] {
            for order in [[0, 1], [1, 0]] {
                let reqs = [TURN[order[0]], TURN[order[1]]];
                let got = convex(&p, &reqs, d);
                eprintln!("hex s={s} a={a} b={b} d={d} {order:?}: {got:?}");
                bad += usize::from(!sound("hex", &got, 1.0));
            }
        }
    }
    assert_eq!(bad, 0);
}

/// **Claim 1/3**: whole rims, so a band is the far band at one turn and
/// the near band at another, and two turns share L when both rims go.
/// Short prisms push the two turns' feet on L together.
#[test]
fn p3_whole_rims_and_short_lateral_edges() {
    let top_rim = [
        req([2, 4], [3, 1]),
        req([2, 1], [4, 5]),
        req([2, 5], [1, 3]),
        req([2, 3], [5, 4]),
    ];
    let bottom_rim = [
        req([0, 4], [3, 1]),
        req([0, 1], [4, 5]),
        req([0, 5], [1, 3]),
        req([0, 3], [5, 4]),
    ];
    let mut bad = 0;
    let mut built = 0;
    for s in [-1.0, -0.5, 0.5, 1.0] {
        for h in [1.0, 0.5, 0.4, 0.33, 0.3, 0.27, 0.25, 0.22, 0.2] {
            let p = leaning(s, h, 1.0);
            let both: Vec<Req> = top_rim.iter().chain(&bottom_rim).copied().collect();
            for (name, reqs) in [("top", top_rim.to_vec()), ("both", both)] {
                let got = convex(&p, &reqs, 0.1);
                eprintln!("rim {name} s={s} h={h}: {got:?}");
                built += usize::from(matches!(got, Got::Built { .. }));
                bad += usize::from(!sound("rim", &got, 1.0));
            }
        }
    }
    for (s, a, b, c) in [(0.4, 0.2, 0.3, 0.5), (-0.6, -0.15, 0.25, -0.5)] {
        let p = hexahedron(Hex { s, a, b, c, ..HEX });
        let got = convex(&p, &top_rim, 0.1);
        eprintln!("hex top rim s={s}: {got:?}");
        bad += usize::from(!sound("hex rim", &got, 1.0));
    }
    eprintln!("built {built}");
    assert_eq!(bad, 0);
}

/// The near band (the one whose foot on L is nearer) at the turn
/// vertex: `d / sin φᵢ`, `φᵢ` between its edge and L.
fn near_band(p: &Poly) -> usize {
    let h = |i: usize| p.halves[i];
    let v = corner_of([h(2), h(3), h(4)]);
    let l = unit(sub(corner_of([h(3), h(4), h(0)]), v));
    let e0 = unit(sub(corner_of([h(2), h(4), h(1)]), v));
    let e1 = unit(sub(corner_of([h(2), h(3), h(5)]), v));
    let sin = |e: V| norm(cross(e, l));
    // larger sine, nearer foot
    usize::from(sin(e1) > sin(e0))
}

/// What band `k`'s half-space takes beyond band `j`'s: `vol(B ∩ Hⱼ) −
/// vol(B ∩ Hⱼ ∩ H_k)`.
fn beyond_near(p: &Poly, d: f64) -> f64 {
    let j = near_band(p);
    let mut hj = p.halves.clone();
    let (_, a, b) = p.edge(TURN[j]);
    hj.push(chamfer_plane(
        p.halves[TURN[j].faces[0]].0,
        p.halves[TURN[j].faces[1]].0,
        a,
        b,
        d,
    ));
    let mut hjk = hj.clone();
    let k = 1 - j;
    let (_, a, b) = p.edge(TURN[k]);
    hjk.push(chamfer_plane(
        p.halves[TURN[k].faces[0]].0,
        p.halves[TURN[k].faces[1]].0,
        a,
        b,
        d,
    ));
    clipped_volume(&hj) - clipped_volume(&hjk)
}

fn try_hex(x: Hex) -> Option<Poly> {
    let p = std::panic::catch_unwind(|| hexahedron(x)).ok()?;
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        for r in TURN {
            let _ = p.edge(r);
        }
    }))
    .ok()?;
    Some(p)
}

/// **Claim 4**: the filed short-near-edge refusal, executed: an obtuse
/// top-face angle at the turn (yaw `c`), the NEAR band's requested
/// edge shortened (the back face, or the right face, moved in). A
/// refusal is spurious where band `k`'s half-space still takes only
/// the local overrun beyond band `j`'s (the long fixture's amount).
#[test]
fn p4_short_near_edge() {
    let d = 0.1;
    let mut spurious = Vec::new();
    for (s, c) in [
        (0.4, 1.0),
        (0.4, -0.6),
        (-0.6, 1.0),
        (-0.6, -0.4),
        (0.4, 1.8),
        (0.8, 2.5),
    ] {
        let base = Hex {
            s,
            a: 0.2,
            b: 0.3,
            c,
            depth: 4.0,
            x1: 3.0,
            ..HEX
        };
        let Some(long) = try_hex(base) else {
            eprintln!("s={s} c={c}: no long fixture");
            continue;
        };
        let (theta, j) = (theta_s(&long), near_band(&long));
        let local = beyond_near(&long, d);
        let ve = d / theta.to_radians().sin();
        let cross_proj = d * (1.0 + theta.to_radians().cos()) / theta.to_radians().sin();
        eprintln!(
            "s={s} c={c}: θ_S {theta:.1}, near band {j}, local overrun {local:.3e}, |VE| {ve:.4}, crossing on e_j {cross_proj:.4}"
        );
        let v = corner_of([long.halves[2], long.halves[3], long.halves[4]]);
        for len in [1.0, 0.4, 0.3, 0.25, 0.2, 0.17, 0.15, 0.13, 0.11, 0.09] {
            let x = if j == 1 {
                // top ∩ wall runs from the vertex's y down to −depth
                Hex {
                    depth: len - v[1],
                    ..base
                }
            } else {
                Hex {
                    x1: s + {
                        // the top ∩ front edge's length to x1, along its direction
                        let e = unit(sub(
                            corner_of([long.halves[2], long.halves[4], long.halves[1]]),
                            v,
                        ));
                        len * e[0]
                    },
                    ..base
                }
            };
            let Some(p) = try_hex(x) else {
                eprintln!("  len {len}: no fixture");
                continue;
            };
            let extra = beyond_near(&p, d);
            let local_only = (extra - local).abs() < 1e-12;
            let got = convex(&p, &TURN, d);
            eprintln!(
                "  len {len}: band k beyond j {extra:.3e} (local only: {local_only}) → {got:?}"
            );
            assert!(sound("short", &got, 1.0));
            if local_only && matches!(&got, Got::Refused(k) if k.ends_with("FOOT_INSIDE_A_FACE")) {
                spurious.push(format!("s={s} c={c} θ_S {theta:.1} len {len}"));
            }
        }
    }
    eprintln!("spurious FOOT_INSIDE_A_FACE: {spurious:#?}");
    // `an-overruns-sliver-refuses-a-short-near-edge` reproduces
    assert!(!spurious.is_empty());
}

/// **Claim 2**: the fillet still refuses `TURN_OVERRUN` at the
/// witnesses; and `leaning_turn_in_band`'s doc says it "escalates for
/// either verb".
#[test]
fn p5_fillet_refuses_and_in_band_fixture_per_verb() {
    for s in [-1.0, -0.5, 0.5, 1.0] {
        let (body, edges) = crate::common::operands::leaning_turn(s);
        let e = fillet_edges(&at_rest(&body, tol()), &edges, 0.1, tol())
            .map(|_| ())
            .expect_err("fillet overrun refuses");
        assert_eq!(
            kind(&e.error),
            "UnsupportedRunOut/TURN_OVERRUN",
            "fillet at {s}"
        );
    }
    let (body, edges) = crate::common::operands::leaning_turn_in_band(tol());
    let c = chamfer_edges(&at_rest(&body, tol()), &edges, 0.1, tol())
        .map(|_| ())
        .expect_err("chamfer in band");
    let f = fillet_edges(&at_rest(&body, tol()), &edges, 0.1, tol())
        .map(|_| ())
        .expect_err("fillet in band");
    eprintln!(
        "in-band fixture: chamfer {} / fillet {}",
        kind(&c.error),
        kind(&f.error)
    );
    assert!(kind(&c.error).starts_with("Escalated"));
    assert!(
        kind(&f.error).starts_with("Escalated"),
        "leaning_turn_in_band's doc: escalates for either verb; the fillet gave {}",
        kind(&f.error)
    );
}

/// A round pin of radius `r` standing out of the bracket `y = 0`
/// corner's leaning wall at `(a along L, b across)` from the vertex.
fn bossed(s: f64, a: f64, b: f64, r: f64, out: bool) -> Body<f64> {
    let p = leaning(s, 1.0, 1.0);
    let n_wall = p.halves[3].0; // outward
    let l_dir = unit([-s, 0.0, -1.0]); // from the vertex (s,0,1) down L
    let at = add(add([s, 0.0, 1.0], mul(l_dir, a)), [0.0, -b, 0.0]);
    let (start, dir) = if out {
        (add(at, mul(n_wall, -0.01)), n_wall)
    } else {
        (add(at, mul(n_wall, 0.01)), mul(n_wall, -1.0))
    };
    // sketch plane normal = u × v must be `dir`
    let u = [0.0, 1.0, 0.0];
    let v = cross(dir, u);
    let pin = prism_on(
        sketch_from_axes(
            Point3::new(start[0], start[1], start[2]),
            Vec3::new(u[0], u[1], u[2]),
            Vec3::new(v[0], v[1], v[2]),
            tol(),
        ),
        vec![(Point2::new(-r, 0.0), 1.0), (Point2::new(r, 0.0), 1.0)],
        0.04,
        tol(),
    );
    let op = if out {
        BooleanOp::Union
    } else {
        BooleanOp::Subtract
    };
    realized(op, &p.body, &pin, tol())
}

/// **Claim 2 (meter)**: features at and around the overrun on the
/// near face (the leaning wall at the bracket's `y = 0` corner, the
/// overrun the triangle `L ∈ [d, √2 d]`, across `≤ (1 − 1/√2) d`).
#[test]
fn p6_features_around_the_overrun() {
    let d = 0.1;
    let base = leaning(-1.0, 1.0, 1.0);
    let edges_of = |b: &Body<f64>| {
        let v = [-1.0, 0.0, 1.0];
        [
            edge_near(b, v, [2.0, 0.0, 1.0]),
            edge_near(b, v, [-1.0, -1.5, 1.0]),
        ]
    };
    let want = clipped_volume(&base.halves) - base.chamfered_volume(&TURN, d);
    let mut rows = Vec::new();
    // (name, a, b, r, boss?, must refuse?)
    let cases: &[(&str, f64, f64, f64, bool, Option<bool>)] = &[
        ("boss in overrun", 1.1 * d, 0.1 * d, 0.004, true, Some(true)),
        (
            "hole in overrun",
            1.1 * d,
            0.1 * d,
            0.004,
            false,
            Some(true),
        ),
        (
            "boss straddling chord",
            1.207 * d,
            0.146 * d,
            0.004,
            true,
            Some(true),
        ),
        (
            "hole straddling chord",
            1.207 * d,
            0.146 * d,
            0.004,
            false,
            Some(true),
        ),
        (
            "boss near p_k outside",
            1.5 * d,
            0.08 * d,
            0.004,
            true,
            None,
        ),
        (
            "boss past chord near q",
            1.15 * d,
            0.45 * d,
            0.004,
            true,
            None,
        ),
        (
            "boss far along L",
            3.0 * d,
            0.1 * d,
            0.004,
            true,
            Some(false),
        ),
        (
            "hole far along L",
            3.0 * d,
            0.1 * d,
            0.004,
            false,
            Some(false),
        ),
    ];
    let mut bad = 0;
    for &(name, a, b, r, boss, must) in cases {
        let body = bossed(-1.0, a, b, r, boss);
        let edges = edges_of(&body);
        // the boss/hole lies off both half-spaces' cut regions when it
        // builds, so its ΔV is the plain one
        let got = chamfer_probe(&body, &edges, d, want, 1.0);
        let ok = match (must, &got) {
            (Some(true), Got::Refused(k)) => !untyped(k),
            (Some(false) | None, Got::Built { err, non_trivalent }) => {
                err.abs() < 1e-10 && *non_trivalent <= 8
            }
            (None, Got::Refused(k)) => !untyped(k),
            _ => false,
        };
        rows.push(format!("{name}: {got:?} ok={ok}"));
        bad += usize::from(!ok);
    }
    for r in &rows {
        eprintln!("{r}");
    }
    assert_eq!(bad, 0);
}

/// **Claim 3**: L chamfered later at the overrun's turn foot — what the
/// first setback's refusal is, and the tenth's volume against the clip
/// oracle (the chamfered body is still a convex polytope).
#[test]
fn p7_later_chamfer_of_l() {
    for s in [0.5, -0.5, -1.0, 1.0] {
        let p = leaning(s, 1.0, 1.0);
        let reqs = TURN;
        let edges: Vec<EdgeKey> = reqs.iter().map(|r| p.edge(*r).0).collect();
        let out =
            chamfer_edges(&at_rest(&p.body, tol()), &edges, 0.1, tol()).expect("overrun builds");
        let b = &out.body;
        let foot = out.naming.as_ref().expect("births").turn_feet[0].0;
        let pf = point_at(b, foot);
        let l = b
            .edges_of_vertex(foot)
            .expect("fan")
            .into_iter()
            .find(|e| ends(b, *e).iter().any(|v| norm(point_at(b, *v)) < 1e-12))
            .expect("L's piece");
        // the chamfered body's halves: the input's plus the two cuts
        let mut halves = p.halves.clone();
        for r in reqs {
            let (_, a, bb) = p.edge(r);
            halves.push(chamfer_plane(
                p.halves[r.faces[0]].0,
                p.halves[r.faces[1]].0,
                a,
                bb,
                0.1,
            ));
        }
        let (n_wall, n_front) = (p.halves[3].0, p.halves[4].0);
        for d2 in [0.1, 0.05, 0.03, 0.02, 0.01] {
            let mut cut = halves.clone();
            cut.push(chamfer_plane(n_wall, n_front, [0.0; 3], pf, d2));
            let want = clipped_volume(&halves) - clipped_volume(&cut);
            let got = chamfer_probe(b, &[l], d2, want, 1.0);
            eprintln!("s={s} later L at {d2}: {got:?}");
            assert!(sound("later L", &got, 1.0), "s={s} d2={d2}: {got:?}");
        }
    }
}

/// **Claim 4**: `corner()`'s `(135°, 90°)` cone at `γ = 90°` — what it
/// refuses now, and whether the valence-4 vertex is the fixture's.
#[test]
fn p8_corner_135_90_refusal() {
    use crate::band_turn_overrun_probe::Cone;
    let c = Cone::angles(
        [135f64.to_radians(), 90f64.to_radians()],
        90f64.to_radians(),
    );
    let (body, edges) = crate::band_turn_overrun_verdict::corner(&c);
    let valences: Vec<usize> = body
        .vertices()
        .map(|(v, _)| body.edges_of_vertex(v).expect("fan").len())
        .collect();
    eprintln!("corner(135, 90) valences {valences:?}");
    let got = chamfer_edges(&at_rest(&body, tol()), &edges, 0.1, tol()).map(|_| ());
    eprintln!(
        "corner(135, 90) chamfer: {:?}",
        got.as_ref().map_err(|e| format!("{:?}", e.error))
    );
    // and each edge alone
    for e in edges {
        let one = chamfer_edges(&at_rest(&body, tol()), &[e], 0.1, tol()).map(|_| ());
        eprintln!("  one edge: {:?}", one.map_err(|e| kind(&e.error)));
    }
}

/// **Claim 1**: concave, the generic hexahedron as a void.
#[test]
fn p9_generic_void() {
    let mut bad = 0;
    for (s, a, b, c) in [(0.4, 0.2, 0.3, 0.5), (-0.6, -0.15, 0.25, -0.5)] {
        let p = hexahedron(Hex { s, a, b, c, ..HEX });
        let block =
            crate::common::cavity::brick(Point3::new(-5.0, -5.0, -5.0), Point3::new(5.0, 5.0, 5.0));
        let void = realized(BooleanOp::Subtract, &block, &p.body, tol());
        let reqs = TURN;
        let pts: Vec<(V, V)> = reqs
            .iter()
            .map(|r| {
                let (_, a, b) = p.edge(*r);
                (a, b)
            })
            .collect();
        let edges: Vec<EdgeKey> = pts.iter().map(|(a, b)| edge_near(&void, *a, *b)).collect();
        let want = clipped_volume(&p.halves) - p.chamfered_volume(&reqs, 0.1);
        let got = chamfer_probe(&void, &edges, 0.1, want, -1.0);
        eprintln!("void hex s={s}: {got:?}");
        bad += usize::from(
            !matches!(
                got,
                Got::Built {
                    non_trivalent: 0,
                    ..
                }
            ) || !sound("void", &got, 1.0),
        );
    }
    assert_eq!(bad, 0);
}

/// **Claim 2**: across the band edge — leans from `1e-7°` to `10°`,
/// both signs — every build (mitre or overrun) is at the half-space
/// volume, and the rest refuse typed.
#[test]
fn p10_every_build_across_the_band_edge_is_exact() {
    let mut tally = std::collections::BTreeMap::<String, usize>::new();
    let mut bad = 0;
    for exp in -28..=4 {
        let deg = 10f64.powf(f64::from(exp) / 4.0);
        for sign in [1.0, -1.0] {
            let s = sign * deg.to_radians().tan();
            let p = leaning(s, 1.0, 1.0);
            let got = convex(&p, &TURN, 0.1);
            let key = match &got {
                Got::Built { non_trivalent, .. } => format!("built, {non_trivalent} non-trivalent"),
                Got::Refused(k) => k.clone(),
                other => format!("{other:?}"),
            };
            eprintln!("lean {:+e}°: {got:?}", sign * deg);
            *tally.entry(key).or_default() += 1;
            bad += usize::from(!sound("lean", &got, 1.0));
        }
    }
    eprintln!("{tally:#?}");
    assert_eq!(bad, 0);
}

/// **Claim 1**: the bracket's two corners at `Interval`, the volume
/// enclosure holding the clip oracle.
#[test]
fn p11_bracket_corners_at_interval() {
    use crate::common::interval::iv;
    use geom_core::Interval;
    for s in [-1.0, 1.0] {
        let (ibody, iedges) = crate::common::operands::leaning_turn_at::<Interval>(s);
        let p = leaning(s, 1.0, 1.0);
        let want = clipped_volume(&p.halves) - p.chamfered_volume(&TURN, 0.1);
        let ibody = sweep::test_support::finished("operand", ibody, tol());
        let out = chamfer_edges(&ibody, &iedges, iv(0.1), tol())
            .unwrap_or_else(|e| panic!("s={s}: builds at Interval, got {:?}", e.error));
        validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("tier 3 {e:?}"));
        let v0 = topo::mass_properties(&ibody, tol()).expect("props").volume;
        let pr = topo::mass_properties(&out.body, tol()).expect("props");
        let (removed, pad) = (v0 - pr.volume, pr.volume_pad.hi());
        eprintln!("s={s}: ΔV {removed:?} ± {pad}, oracle {want}");
        assert!(removed.lo() - pad <= want && want <= removed.hi() + pad);
        assert!(removed.hi() - removed.lo() + 2.0 * pad < 1e-6);
    }
}
