//! **What the near-tangent census cannot decide**: a measurement probe
//! for `work/join/near-tangent-boolean-results-ship-with-an-escalated-tier-3-census.md`.
//!
//! The poses are PR 4026 review r1's near-tangent set (`r1_pierce_probes
//! cube` under `R1_NT_D`): a prism's corner `v` on the near face of a
//! cube of side 4, the face's normal at `a` sixteenths of a turn about
//! one of the corner's three edges `e` and tilted `d` along it, so the
//! face's plane lies `d` rad off that edge. Every op in both orders
//! prints one `outcome` line against a kernel-free clipping oracle.
//!
//! For a body that builds and fails tier 3′, the census runs again
//! through its trace door and every pair it decided against is dumped
//! as a `PAIR` JSON line: the two entities, their exact `f64`
//! coordinates, and the census's own reading of each predicate on the
//! pair. A refusal prints its findings. `scripts/near_tangent_census_classify.py`
//! re-reads every `PAIR` at 60 digits and classifies it.
//!
//! `NT_D` (comma list) sets the tilts, `NT_ONLY` one prism, `NT_POSE`
//! a substring of the pose tag, and `NT_DUMP=1` turns on the dump.
//!
//! `cargo run -p sweep --release --example near_tangent_census_probe`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

#[path = "../tests/common/differential.rs"]
#[allow(dead_code, unreachable_pub)]
mod differential;

use differential::outcome;
use geom_core::{Band, Point3, Tol};
use topo::entity::{EntityId, LoopBoundary};
use topo::test_support as fixtures;
use topo::{
    AtRestBody, Body, BooleanBody, BooleanDeclarations, BooleanError, BooleanResult,
    CensusStrategy, RegionLane, ValidationError, mass_properties,
};

type V3 = [f64; 3];
type Plane = (V3, f64); // n·x ≤ d

fn tol() -> Tol {
    Tol::witness()
}
fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn scale(a: V3, s: f64) -> V3 {
    a.map(|c| c * s)
}
fn unit(a: V3) -> V3 {
    scale(a, 1.0 / dot(a, a).sqrt())
}

/// A prism: CCW profile, height, convex pieces (CCW polygons), corner.
struct Prism {
    name: &'static str,
    profile: Vec<(f64, f64)>,
    pieces: Vec<Vec<(f64, f64)>>,
    h: f64,
    v: V3,
}

impl Prism {
    fn piece_planes(&self) -> Vec<Vec<Plane>> {
        self.pieces
            .iter()
            .map(|poly| {
                let n = poly.len();
                let mut out: Vec<Plane> = (0..n)
                    .map(|i| {
                        let (p, q) = (poly[i], poly[(i + 1) % n]);
                        let nn = [q.1 - p.1, -(q.0 - p.0), 0.0];
                        (nn, nn[0] * p.0 + nn[1] * p.1)
                    })
                    .collect();
                out.push(([0.0, 0.0, 1.0], self.h));
                out.push(([0.0, 0.0, -1.0], 0.0));
                out
            })
            .collect()
    }
}

fn basis(m: V3) -> (V3, V3) {
    let m = unit(m);
    let seed = if m[2].abs() < 0.9 {
        [0.0, 0.0, 1.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    let u = unit(cross(seed, m));
    (u, cross(m, u))
}

type Poly = Vec<Vec<V3>>; // faces, each CCW seen from outside

fn clip(poly: &Poly, (n, d): Plane) -> Poly {
    let mut out: Poly = Vec::new();
    let mut cap: Vec<V3> = Vec::new();
    for f in poly {
        let mut g = Vec::new();
        for i in 0..f.len() {
            let (p, q) = (f[i], f[(i + 1) % f.len()]);
            let (sp, sq) = (dot(n, p) - d, dot(n, q) - d);
            if sp <= 0.0 {
                g.push(p);
            }
            if (sp < 0.0 && sq > 0.0) || (sp > 0.0 && sq < 0.0) {
                let t = sp / (sp - sq);
                let x = add(p, scale(sub(q, p), t));
                g.push(x);
                cap.push(x);
            }
            if sp == 0.0 {
                cap.push(p);
            }
        }
        if g.len() >= 3 {
            out.push(g);
        }
    }
    if cap.len() >= 3 {
        let c = scale(
            cap.iter().fold([0.0; 3], |a, &b| add(a, b)),
            1.0 / cap.len() as f64,
        );
        let (e1, e2) = basis(unit(n));
        let mut pts: Vec<(f64, V3)> = cap
            .iter()
            .map(|&p| {
                let r = sub(p, c);
                (dot(r, e2).atan2(dot(r, e1)), p)
            })
            .collect();
        pts.sort_by(|a, b| a.0.total_cmp(&b.0));
        pts.dedup_by(|a, b| dot(sub(a.1, b.1), sub(a.1, b.1)) < 1e-24);
        out.push(pts.into_iter().map(|p| p.1).collect());
    }
    out
}

fn volume(poly: &Poly) -> f64 {
    poly.iter()
        .map(|f| {
            (1..f.len() - 1)
                .map(|i| dot(f[0], cross(f[i], f[i + 1])))
                .sum::<f64>()
        })
        .sum::<f64>()
        / 6.0
}

/// The cube `v + a u + b w + c m`, a, b ∈ [−2, 2], c ∈ [0, 4], as faces.
fn cube_poly(v: V3, u: V3, w: V3, m: V3) -> Poly {
    let p = |a: f64, b: f64, c: f64| add(v, add(scale(u, a), add(scale(w, b), scale(m, c))));
    let c = |i: usize| {
        let (a, b, z) = ((i & 1) as f64, ((i >> 1) & 1) as f64, ((i >> 2) & 1) as f64);
        p(-2.0 + 4.0 * a, -2.0 + 4.0 * b, 4.0 * z)
    };
    let quads = [
        [0, 2, 3, 1],
        [4, 5, 7, 6],
        [0, 1, 5, 4],
        [2, 6, 7, 3],
        [0, 4, 6, 2],
        [1, 3, 7, 5],
    ];
    quads
        .iter()
        .map(|q| q.iter().map(|&i| c(i)).collect())
        .collect()
}

fn common_cube(pr: &Prism, cube: &Poly) -> f64 {
    pr.piece_planes()
        .iter()
        .map(|pl| volume(&pl.iter().fold(cube.clone(), |acc, &p| clip(&acc, p))))
        .sum()
}

fn area(poly: &Poly) -> f64 {
    poly.iter()
        .map(|f| {
            let n = (1..f.len() - 1).fold([0.0; 3], |acc, i| {
                add(acc, cross(sub(f[i], f[0]), sub(f[i + 1], f[0])))
            });
            dot(n, n).sqrt() / 2.0
        })
        .sum()
}

/// Each convex piece of `pr ∩ cube`: its volume, its area, their ratio
/// (the role read's margin, `V/A`) and its greatest thickness off the
/// cube's near face (`m·x ≥ m·v`), the lump a sliver shell would be.
fn piece_lumps(pr: &Prism, cube: &Poly, v: V3, m: V3) -> Vec<String> {
    pr.piece_planes()
        .iter()
        .enumerate()
        .map(|(i, pl)| {
            let p = pl.iter().fold(cube.clone(), |acc, &q| clip(&acc, q));
            let (vol, ar) = (volume(&p), area(&p));
            let thick = p
                .iter()
                .flatten()
                .map(|&x| dot(m, sub(x, v)))
                .fold(0.0, f64::max);
            format!(
                "  LUMP piece{i} volume={vol:.6e} area={ar:.6e} v/a={:.6e} thickness={thick:.6e}",
                if ar > 0.0 { vol / ar } else { 0.0 }
            )
        })
        .collect()
}

fn prisms() -> Vec<Prism> {
    let l = vec![
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ];
    let l_pieces = vec![
        vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)],
        vec![(0.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)],
    ];
    let mirror = |p: &[(f64, f64)]| -> Vec<(f64, f64)> {
        let mut q: Vec<(f64, f64)> = p.iter().map(|&(x, y)| (-x, y)).collect();
        q.reverse();
        q
    };
    vec![
        Prism {
            name: "Ltop",
            profile: l.clone(),
            pieces: l_pieces.clone(),
            h: 1.0,
            v: [1.0, 1.0, 1.0],
        },
        Prism {
            name: "Lbot",
            profile: l.clone(),
            pieces: l_pieces.clone(),
            h: 1.0,
            v: [1.0, 1.0, 0.0],
        },
        Prism {
            name: "Lmirror",
            profile: mirror(&l),
            pieces: l_pieces.iter().map(|p| mirror(p)).collect(),
            h: 1.0,
            v: [-1.0, 1.0, 1.0],
        },
        Prism {
            name: "notch307",
            profile: vec![(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 1.0), (0.0, 2.0)],
            pieces: vec![
                vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 2.0)],
                vec![(2.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 1.0)],
            ],
            h: 1.0,
            v: [2.0, 1.0, 1.0],
        },
        Prism {
            name: "shallow200",
            profile: vec![(0.0, 0.0), (4.0, 0.0), (4.0, 1.0), (2.0, 0.6), (0.0, 1.0)],
            pieces: vec![
                vec![(0.0, 0.0), (2.0, 0.0), (2.0, 0.6), (0.0, 1.0)],
                vec![(2.0, 0.0), (4.0, 0.0), (4.0, 1.0), (2.0, 0.6)],
            ],
            h: 1.0,
            v: [2.0, 0.6, 1.0],
        },
        Prism {
            name: "convex",
            profile: l,
            pieces: l_pieces,
            h: 1.0,
            v: [2.0, 0.0, 1.0],
        },
        Prism {
            name: "vee300",
            profile: vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (2.0, 0.5), (0.0, 4.0)],
            pieces: vec![
                vec![(0.0, 0.0), (2.0, 0.0), (2.0, 0.5), (0.0, 4.0)],
                vec![(2.0, 0.0), (4.0, 0.0), (4.0, 4.0), (2.0, 0.5)],
            ],
            h: 1.0,
            v: [2.0, 0.5, 1.0],
        },
        Prism {
            name: "asym",
            profile: vec![(0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (1.0, 1.0), (0.0, 1.5)],
            pieces: vec![
                vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.5)],
                vec![(1.0, 0.0), (4.0, 0.0), (4.0, 3.0), (1.0, 1.0)],
            ],
            h: 1.0,
            v: [1.0, 1.0, 1.0],
        },
        wedge("w345", 345.0),
        wedge("w60", 60.0),
    ]
}

/// The `alpha`° wedge about the z axis, corner `(0, 0, 1)`, its
/// profile on the square of half-side 2 (`join_pierce_runs_sweep`'s).
fn wedge(name: &'static str, alpha: f64) -> Prism {
    let at = |t: f64| {
        let t = t.to_radians();
        let r = 2.0 / t.cos().abs().max(t.sin().abs());
        (r * t.cos(), r * t.sin())
    };
    let mut profile = vec![(0.0, 0.0), at(0.0)];
    let mut c = 45.0;
    while c < alpha {
        profile.push(at(c));
        c += 90.0;
    }
    profile.push(at(alpha));
    let pieces = (1..profile.len() - 1)
        .map(|k| vec![profile[0], profile[k], profile[k + 1]])
        .collect();
    Prism {
        name,
        profile,
        pieces,
        h: 1.0,
        v: [0.0, 0.0, 1.0],
    }
}

/// The near-tangent normals: the plane through `v` nearly holds one of
/// the corner's edges, `d` rad off it.
fn near_tangent(edges: &[V3], ds: &[f64]) -> Vec<(String, V3)> {
    let mut out = Vec::new();
    for (j, &e) in edges.iter().enumerate() {
        let e = unit(e);
        let (p1, p2) = basis(e);
        for a in 0..16 {
            let al = std::f64::consts::TAU * (f64::from(a) + 0.25) / 16.0;
            let base = add(scale(p1, al.cos()), scale(p2, al.sin()));
            for &d in ds {
                out.push((format!("nt e{j} a{a} d{d:e}"), unit(add(base, scale(e, d)))));
            }
        }
    }
    out
}

fn corner_edges(pr: &Prism) -> Vec<V3> {
    let n = pr.profile.len();
    let i = pr
        .profile
        .iter()
        .position(|&(x, y)| (x - pr.v[0]).abs() < 1e-12 && (y - pr.v[1]).abs() < 1e-12)
        .unwrap();
    let (p, a, b) = (
        pr.profile[i],
        pr.profile[(i + n - 1) % n],
        pr.profile[(i + 1) % n],
    );
    let vz = if pr.v[2] > 0.5 { -1.0 } else { 1.0 };
    vec![
        [a.0 - p.0, a.1 - p.1, 0.0],
        [b.0 - p.0, b.1 - p.1, 0.0],
        [0.0, 0.0, vz],
    ]
}

fn pt(p: Point3<f64>) -> String {
    format!("[{:?},{:?},{:?}]", p.x, p.y, p.z)
}

/// A line edge's `(v0, v1)` and their points, the census's own reading
/// (`he_plus`'s start to its end).
fn edge_ends(body: &Body<f64>, e: topo::EdgeKey) -> Option<String> {
    let edge = body.get_edge(e)?;
    let plus = body.get_half_edge(edge.he_plus)?;
    let (v0, v1) = (plus.start, body.half_edge_end(edge.he_plus)?);
    let (k0, k1) = (body.get_vertex(v0)?.point, body.get_vertex(v1)?.point);
    let (p0, p1) = (*body.get_point(k0)?, *body.get_point(k1)?);
    let face = |he| -> Option<topo::FaceKey> {
        Some(body.get_loop(body.get_half_edge(he)?.parent_loop)?.face)
    };
    let (f0, f1) = (face(edge.he_plus)?, face(edge.he_minus)?);
    Some(format!(
        "{{\"key\":\"{e:?}\",\"v\":[\"{v0:?}\",\"{v1:?}\"],\"pk\":[\"{k0:?}\",\"{k1:?}\"],\"faces\":[\"{f0:?}\",\"{f1:?}\"],\"p\":[{},{}]}}",
        pt(p0),
        pt(p1)
    ))
}

/// A planar face's plane and loops, every loop's vertex points in walk
/// order.
fn face_geo(body: &Body<f64>, f: topo::FaceKey) -> Option<String> {
    let face = body.get_face(f)?;
    let (origin, normal) = match body.get_surface(face.surface)? {
        geom::Surface::Plane { origin, normal, .. } => (*origin, *normal),
        _ => return None,
    };
    let mut loops = Vec::new();
    for lk in std::iter::once(face.outer).chain(face.rings.iter().copied()) {
        let LoopBoundary::Cycle { first } = body.get_loop(lk)?.boundary else {
            continue;
        };
        let ring: Vec<String> = body
            .loop_cycle(first)?
            .into_iter()
            .filter_map(|he| {
                let v = body.get_half_edge(he)?.start;
                let k = body.get_vertex(v)?.point;
                Some(format!(
                    "{{\"v\":\"{v:?}\",\"pk\":\"{k:?}\",\"p\":{}}}",
                    pt(*body.get_point(k)?)
                ))
            })
            .collect();
        loops.push(format!("[{}]", ring.join(",")));
    }
    Some(format!(
        "{{\"key\":\"{f:?}\",\"origin\":{},\"normal\":[{:?},{:?},{:?}],\"loops\":[{}]}}",
        pt(origin),
        normal.x,
        normal.y,
        normal.z,
        loops.join(",")
    ))
}

fn vertex_geo(body: &Body<f64>, v: topo::VertexKey) -> Option<String> {
    let k = body.get_vertex(v)?.point;
    Some(format!(
        "{{\"key\":\"{v:?}\",\"pk\":\"{k:?}\",\"p\":{}}}",
        pt(*body.get_point(k)?)
    ))
}

fn entity_geo(body: &Body<f64>, id: EntityId) -> String {
    match id {
        EntityId::Vertex(v) => vertex_geo(body, v),
        EntityId::Edge(e) => edge_ends(body, e),
        EntityId::Face(f) => face_geo(body, f),
        _ => None,
    }
    .unwrap_or_else(|| "null".into())
}

/// One finding as a short tag: an escalation's predicate and margin,
/// anything else its `Debug` head.
fn finding(e: &ValidationError) -> String {
    match e {
        ValidationError::CensusEscalated { cause } => {
            format!("esc {} {:e}", cause.predicate.unwrap_or("?"), cause.margin)
        }
        other => format!("{other:?}").chars().take(160).collect(),
    }
}

/// The census's trace over a built body: every finding, and every
/// pair any sweep decided against, with its geometry.
fn dump(tag: &str, body: &BooleanBody<f64>) -> Vec<String> {
    let mut out = Vec::new();
    let band = Band::linear(tol()).unwrap();
    let (errors, trace) = topo::census_traces(
        &body.body,
        &body.contacts,
        band,
        tol(),
        Some(RegionLane::certified()),
        CensusStrategy::Realized,
    );
    for e in &errors {
        out.push(format!("  FINDING {tag} | {}", finding(e)));
    }
    for (kind, pairs) in [
        ("vv", &trace.vv),
        ("ve", &trace.ve),
        ("vf", &trace.vf),
        ("ef", &trace.ef),
        ("ee", &trace.ee),
    ] {
        for &(a, b) in &pairs.accepted {
            out.push(format!(
                "PAIR {{\"tag\":\"{tag}\",\"kind\":\"{kind}\",\"a\":{},\"b\":{}}}",
                entity_geo(&body.body, a),
                entity_geo(&body.body, b)
            ));
        }
    }
    out
}

type Op = fn(
    &AtRestBody<f64>,
    &AtRestBody<f64>,
    &BooleanDeclarations,
    Tol,
) -> Result<BooleanResult<f64>, BooleanError>;

fn run_all(
    tag: &str,
    prism: &AtRestBody<f64>,
    cube: &AtRestBody<f64>,
    va: f64,
    vb: f64,
    common: f64,
    lumps: &[String],
) {
    let decls = BooleanDeclarations::default();
    let dumping = std::env::var("NT_DUMP").is_ok();
    for (order, x, y, vx) in [("pc", prism, cube, va), ("cp", cube, prism, vb)] {
        let ops: [(&str, Op, f64); 3] = [
            ("U", topo::union_with, va + vb - common),
            ("I", topo::intersect_with, common),
            ("S", topo::subtract_with, vx - common),
        ];
        for (op, f, want) in ops {
            let r = f(x, y, &decls, tol());
            let line = format!("{tag} {order} {op}");
            let mut after = Vec::new();
            match &r {
                Ok(res) => {
                    if let Some(bb) = res.body()
                        && let Err(errors) =
                            topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol())
                    {
                        for e in &errors {
                            after.push(format!("  T3 {line} | {}", finding(e)));
                        }
                        if dumping {
                            after.extend(dump(&line, bb));
                        }
                    }
                }
                Err(BooleanError::ResultInvalid { errors }) => {
                    for e in errors {
                        after.push(format!("  REFUSAL {line} | {e:?}"));
                    }
                    if op == "I" {
                        after.extend(lumps.iter().cloned());
                    }
                }
                Err(_) => {}
            }
            println!("{line}: {}", outcome(r, want, tol()));
            for a in after {
                println!("{a}");
            }
        }
    }
}

fn main() {
    let ds: Vec<f64> = std::env::var("NT_D").map_or_else(
        |_| {
            vec![
                1e-5, -1e-5, 1e-6, -1e-6, 1e-7, -1e-7, 1e-8, -1e-8, 1e-9, -1e-9,
            ]
        },
        |s| s.split(',').map(|x| x.parse().unwrap()).collect(),
    );
    let only = std::env::var("NT_ONLY").ok();
    let pose = std::env::var("NT_POSE").ok();
    for pr in prisms() {
        if only.as_deref().is_some_and(|o| o != pr.name) {
            continue;
        }
        let body =
            AtRestBody::validate(fixtures::prism::<f64>(&pr.profile, pr.h, tol()).body, tol())
                .unwrap();
        let va = mass_properties(&body, tol()).unwrap().volume;
        for (dn, m) in near_tangent(&corner_edges(&pr), &ds) {
            let tag = format!("{} {dn}", pr.name);
            if pose.as_deref().is_some_and(|p| !tag.contains(p)) {
                continue;
            }
            let m = unit(m);
            let (u, w) = basis(m);
            let v = pr.v;
            let cube = fixtures::mapped_cube::<f64>(
                move |x, y, z| {
                    let (a, b, c) = (-2.0 + 4.0 * x, -2.0 + 4.0 * y, 4.0 * z);
                    let p = add(v, add(scale(u, a), add(scale(w, b), scale(m, c))));
                    Point3::new(p[0], p[1], p[2])
                },
                tol(),
            );
            let cube = AtRestBody::validate(cube, tol()).unwrap();
            let poly = cube_poly(v, u, w, m);
            let common = common_cube(&pr, &poly);
            let lumps = piece_lumps(&pr, &poly, v, m);
            run_all(&tag, &body, &cube, va, 64.0, common, &lumps);
        }
    }
}
