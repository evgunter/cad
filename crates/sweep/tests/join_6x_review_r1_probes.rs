//! Review r1 probes for PR 4050 (a six-crossing vertex pair nests its
//! pairing in B). Two corner-carrying operands meet vertex to vertex:
//! A is a reflex prism corner (several notch angles, a sheared notch,
//! the 4-valent roof-cross valley) and B is a cube's corner or edge, or
//! a second reflex prism corner, turned about the shared point. Every
//! op in both orders, each body read by [`outcome`] against a
//! kernel-free oracle: each operand is a signed sum of convex pieces
//! given as half-spaces, and `vol(A ∩ B) = Σ sᵢ tⱼ vol(Pᵢ ∩ Qⱼ)`.
//!
//! `R1X_SET=<rand|tilt|turn> R1X_SHARD=k/n R1X_OUT=<file> cargo test
//! -p sweep --release --test all r1x_sweep -- --ignored --nocapture`.
//! Each run is announced on stderr (`R1XRUN <tag>`) before it runs, so
//! a trace build's lines attribute.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol};
use topo::test_support as fixtures;
use topo::{AtRestBody, Body, BooleanDeclarations, BooleanError, BooleanResult};

use crate::common::differential::outcome;

fn tol() -> Tol {
    Tol::witness()
}

type V3 = [f64; 3];
type Half = (V3, f64);
type M3 = [V3; 3];

fn unit(m: V3) -> V3 {
    let n = m.iter().map(|c| c * c).sum::<f64>().sqrt();
    m.map(|c| c / n)
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
fn rot(x: V3, k: V3, ang: f64) -> V3 {
    let k = unit(k);
    let (c, s) = (ang.cos(), ang.sin());
    let kx = cross(k, x);
    let kd = dot(k, x);
    [0, 1, 2].map(|i| x[i] * c + kx[i] * s + k[i] * kd * (1.0 - c))
}
/// `R x` for the matrix whose COLUMNS are `r`'s rows' images: `r[i]`
/// is where the local axis `eᵢ` goes.
fn apply(r: &M3, x: V3) -> V3 {
    [0, 1, 2].map(|t| r[0][t] * x[0] + r[1][t] * x[1] + r[2][t] * x[2])
}

/// As `join_pierce_runs_sweep::frame`.
fn frame(m: V3, psi: f64) -> M3 {
    let m = unit(m);
    let seed = if m[2].abs() < 0.9 {
        [0.0, 0.0, 1.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    let u0 = unit(cross(seed, m));
    let w0 = cross(m, u0);
    let (c, s) = (psi.cos(), psi.sin());
    let u = [0, 1, 2].map(|i| c * u0[i] + s * w0[i]);
    let w = cross(m, u);
    [u, w, m]
}

/// As `join_pierce_runs_sweep::convex_volume`.
fn convex_volume(planes: &[Half]) -> f64 {
    const EPS: f64 = 1e-9;
    let mut pts: Vec<V3> = Vec::new();
    let n = planes.len();
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let (a, b, c) = (planes[i], planes[j], planes[k]);
                let det = dot(a.0, cross(b.0, c.0));
                if det.abs() < 1e-12 {
                    continue;
                }
                let bc = cross(b.0, c.0);
                let ca = cross(c.0, a.0);
                let ab = cross(a.0, b.0);
                let p = [0, 1, 2].map(|t| (a.1 * bc[t] + b.1 * ca[t] + c.1 * ab[t]) / det);
                if planes.iter().all(|&(nn, d)| dot(nn, p) <= d + EPS)
                    && !pts
                        .iter()
                        .any(|q| (0..3).all(|t| (q[t] - p[t]).abs() < EPS))
                {
                    pts.push(p);
                }
            }
        }
    }
    if pts.len() < 4 {
        return 0.0;
    }
    let inner = [0, 1, 2].map(|t| pts.iter().map(|p| p[t]).sum::<f64>() / pts.len() as f64);
    let mut vol = 0.0;
    for &(nn, d) in planes {
        let on: Vec<V3> = pts
            .iter()
            .copied()
            .filter(|&p| (dot(nn, p) - d).abs() < EPS)
            .collect();
        if on.len() < 3 {
            continue;
        }
        let c = [0, 1, 2].map(|t| on.iter().map(|p| p[t]).sum::<f64>() / on.len() as f64);
        let e1 = unit([0, 1, 2].map(|t| on[0][t] - c[t]));
        let e2 = cross(unit(nn), e1);
        let mut ring: Vec<(f64, V3)> = on
            .iter()
            .map(|&p| {
                let r = [0, 1, 2].map(|t| p[t] - c[t]);
                (dot(r, e2).atan2(dot(r, e1)), p)
            })
            .collect();
        ring.sort_by(|a, b| a.0.total_cmp(&b.0));
        let area: f64 = (0..ring.len())
            .map(|i| {
                let (p, q) = (ring[i].1, ring[(i + 1) % ring.len()].1);
                let r1 = [0, 1, 2].map(|t| p[t] - c[t]);
                let r2 = [0, 1, 2].map(|t| q[t] - c[t]);
                dot(cross(r1, r2), unit(nn)) / 2.0
            })
            .sum();
        let h = d / dot(nn, nn).sqrt() - dot(unit(nn), inner);
        vol += area.abs() * h / 3.0;
    }
    vol
}

/// A local operand: its corner at the origin, a builder of its body
/// under a point map, and itself as signed convex pieces.
struct Local {
    name: &'static str,
    build: Box<dyn Fn(&dyn Fn(V3) -> V3) -> Body<f64> + Sync>,
    pieces: Vec<(f64, Vec<Half>)>,
}

/// The half-spaces of `pieces` under `x ↦ v + R x`.
fn moved(pieces: &[(f64, Vec<Half>)], r: &M3, v: V3) -> Vec<(f64, Vec<Half>)> {
    pieces
        .iter()
        .map(|(s, hs)| {
            (
                *s,
                hs.iter()
                    .map(|&(n, d)| {
                        let n2 = apply(r, n);
                        (n2, d + dot(n2, v))
                    })
                    .collect(),
            )
        })
        .collect()
}

fn pair_volume(a: &[(f64, Vec<Half>)], b: &[(f64, Vec<Half>)]) -> f64 {
    let mut v = 0.0;
    for (s, p) in a {
        for (t, q) in b {
            let mut all = p.clone();
            all.extend_from_slice(q);
            v += s * t * convex_volume(&all);
        }
    }
    v
}

fn self_volume(a: &[(f64, Vec<Half>)]) -> f64 {
    a.iter().map(|(s, p)| s * convex_volume(p)).sum()
}

/// The square `[−2, 2]²` less the wedge below the ray at `360 − alpha`
/// degrees above the ray at 0: a reflex `alpha` corner at the origin,
/// fanned from it; a prism over `z ∈ [−1, 0]` sheared
/// `z' = z + sx·x + sy·y` (volume-preserving, faces stay planar).
fn notch(name: &'static str, alpha: f64, sx: f64, sy: f64) -> Local {
    notch_turned(name, alpha, 0.0, sx, sy)
}

/// [`notch`] with its profile turned `twist` degrees about the corner.
fn notch_turned(name: &'static str, alpha: f64, twist: f64, sx: f64, sy: f64) -> Local {
    let at = |t: f64| {
        let t = t.to_radians();
        let r = 2.0 / t.cos().abs().max(t.sin().abs());
        (r * t.cos(), r * t.sin())
    };
    let mut prof = vec![(0.0, 0.0), at(0.0)];
    let mut c = 45.0;
    while c < alpha - 1e-9 {
        prof.push(at(c));
        c += 90.0;
    }
    prof.push(at(alpha));
    let (c, s) = (twist.to_radians().cos(), twist.to_radians().sin());
    let prof: Vec<(f64, f64)> = prof
        .iter()
        .map(|&(x, y)| (c * x - s * y, s * x + c * y))
        .collect();
    let mut pieces = Vec::new();
    for k in 1..prof.len() - 1 {
        let tri = [prof[0], prof[k], prof[k + 1]];
        let mut hs: Vec<Half> = vec![([0.0, 0.0, -1.0], 1.0), ([0.0, 0.0, 1.0], 0.0)];
        for e in 0..3 {
            let (p, q) = (tri[e], tri[(e + 1) % 3]);
            let n = [q.1 - p.1, -(q.0 - p.0), 0.0];
            hs.push((n, n[0] * p.0 + n[1] * p.1));
        }
        // Under the shear z' = z + sx x + sy y: n·S⁻¹x' ≤ d.
        let hs = hs
            .into_iter()
            .map(|(n, d)| ([n[0] - n[2] * sx, n[1] - n[2] * sy, n[2]], d))
            .collect();
        pieces.push((1.0, hs));
    }
    let build = move |map: &dyn Fn(V3) -> V3| {
        let mut b = Body::<f64>::new();
        fixtures::prism_ops(
            &mut b,
            &prof,
            (-1.0, 0.0),
            |x, y, z| {
                let p = map([x, y, z + sx * x + sy * y]);
                Point3::new(p[0], p[1], p[2])
            },
            fixtures::FaceGeometry::Certified,
            tol(),
        );
        fixtures::describe_as_intersections(&mut b, tol());
        b
    };
    Local {
        name,
        build: Box::new(build),
        pieces,
    }
}

const SIDE: f64 = 4.0;

/// The cube `lo + [0, 4]³`: `lo = 0` its corner at the origin,
/// `(0, −2, 0)` the origin inside its edge along y.
fn cube(name: &'static str, lo: V3) -> Local {
    let mut hs = Vec::new();
    for t in 0..3 {
        let mut e = [0.0; 3];
        e[t] = 1.0;
        hs.push((e, lo[t] + SIDE));
        hs.push((e.map(|c| -c), -lo[t]));
    }
    let build = move |map: &dyn Fn(V3) -> V3| {
        fixtures::mapped_cube::<f64>(
            |x, y, z| {
                let p = map([lo[0] + SIDE * x, lo[1] + SIDE * y, lo[2] + SIDE * z]);
                Point3::new(p[0], p[1], p[2])
            },
            tol(),
        )
    };
    Local {
        name,
        build: Box::new(build),
        pieces: vec![(1.0, hs)],
    }
}

/// r2's two roofs whose ridges cross at the origin, united: the
/// 4-valent cross valley, as a prism-ops pair united by the kernel.
fn valley4() -> Local {
    let roof_halves = |along_x: bool| -> Vec<Half> {
        let (a, b) = if along_x {
            ([0.0, 0.5, 1.0], [0.0, -0.5, 1.0])
        } else {
            ([0.5, 0.0, 1.0], [-0.5, 0.0, 1.0])
        };
        let (ext, lat) = if along_x { (0, 1) } else { (1, 0) };
        let (half, floor) = if along_x { (2.0, 1.0) } else { (2.4, 1.2) };
        let mut e = [0.0; 3];
        e[ext] = 1.0;
        let mut l = [0.0; 3];
        l[lat] = 1.0;
        vec![
            (a, 0.0),
            (b, 0.0),
            ([0.0, 0.0, -1.0], floor),
            (e, 2.5),
            (e.map(|c| -c), 2.5),
            (l, half),
            (l.map(|c| -c), half),
        ]
    };
    let (hx, hy) = (roof_halves(true), roof_halves(false));
    let both: Vec<Half> = hx.iter().chain(&hy).copied().collect();
    let build = |map: &dyn Fn(V3) -> V3| {
        let tri = [(-2.0, -1.0), (2.0, -1.0), (0.0, 0.0)];
        let mut w1 = Body::<f64>::new();
        fixtures::prism_ops(
            &mut w1,
            &tri,
            (-2.5, 2.5),
            |p, q, t| {
                let x = map([t, p, q]);
                Point3::new(x[0], x[1], x[2])
            },
            fixtures::FaceGeometry::Certified,
            tol(),
        );
        fixtures::describe_as_intersections(&mut w1, tol());
        let tri2 = [(-2.4, -1.2), (2.4, -1.2), (0.0, 0.0)];
        let mut w2 = Body::<f64>::new();
        fixtures::prism_ops(
            &mut w2,
            &tri2,
            (-2.5, 2.5),
            |p, q, t| {
                let x = map([p, -t, q]);
                Point3::new(x[0], x[1], x[2])
            },
            fixtures::FaceGeometry::Certified,
            tol(),
        );
        fixtures::describe_as_intersections(&mut w2, tol());
        let (w1, w2) = (finished("roof x", w1), finished("roof y", w2));
        let r = topo::union_with(&w1, &w2, &BooleanDeclarations::default(), tol())
            .expect("the valley builds");
        r.body()
            .expect("the valley is not empty")
            .body
            .clone()
            .into_body()
    };
    Local {
        name: "valley4",
        build: Box::new(build),
        pieces: vec![(1.0, hx), (1.0, hy), (-1.0, both)],
    }
}

fn finished(what: &str, body: Body<f64>) -> AtRestBody<f64> {
    AtRestBody::validate(body, tol()).unwrap_or_else(|e| panic!("{what}: {e:?}"))
}

fn local(name: &str) -> Local {
    match name {
        "n343" => notch("n343", 343.0, 0.0, 0.0),
        // The pin's notch343 corner: its gap about +x.
        "p343" => notch_turned(
            "p343",
            360.0 - 2.0 * 0.15f64.atan().to_degrees(),
            0.15f64.atan().to_degrees(),
            0.0,
            0.0,
        ),
        "n345" => notch("n345", 345.0, 0.0, 0.0),
        "n330" => notch("n330", 330.0, 0.0, 0.0),
        "n350" => notch("n350", 350.0, 0.0, 0.0),
        "n357" => notch("n357", 357.0, 0.0, 0.0),
        "n300" => notch("n300", 300.0, 0.0, 0.0),
        "L270" => notch("L270", 270.0, 0.0, 0.0),
        "s343" => notch("s343", 343.0, 0.35, -0.25),
        "s320" => notch("s320", 320.0, -0.4, 0.5),
        "sh200" => notch("sh200", 200.0, 0.2, 0.1),
        "valley4" => valley4(),
        "cubeC" => cube("cubeC", [0.0, 0.0, 0.0]),
        "cubeE" => cube("cubeE", [0.0, -2.0, 0.0]),
        _ => panic!("unknown local {name}"),
    }
}

type Op = fn(
    &AtRestBody<f64>,
    &AtRestBody<f64>,
    &BooleanDeclarations,
    Tol,
) -> Result<BooleanResult<f64>, BooleanError>;

/// The six runs of A (at rest, identity) and B turned by `r` about the
/// shared corner `v`.
fn pose_lines(a: &Local, b: &Local, r: &M3, what: &str) -> Vec<String> {
    let v = [1.0, 1.0, 1.0];
    let id: M3 = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let ab = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let ab = (a.build)(&|x| [x[0] + v[0], x[1] + v[1], x[2] + v[2]]);
        let bb = (b.build)(&|x| {
            let y = apply(r, x);
            [y[0] + v[0], y[1] + v[1], y[2] + v[2]]
        });
        (ab, bb)
    }));
    let Ok((ab, bb)) = ab else {
        return vec![format!("{} {} {what}: OPERAND PANIC", a.name, b.name)];
    };
    let (Ok(ab), Ok(bb)) = (
        AtRestBody::validate(ab, tol()),
        AtRestBody::validate(bb, tol()),
    ) else {
        return vec![format!("{} {} {what}: OPERAND UNFINISHED", a.name, b.name)];
    };
    let pa = moved(&a.pieces, &id, v);
    let pb = moved(&b.pieces, r, v);
    let (va, vb) = (self_volume(&pa), self_volume(&pb));
    let common = pair_volume(&pa, &pb);
    let d = BooleanDeclarations::default();
    let mut out = Vec::new();
    for (order, x, y, vx) in [("ab", &ab, &bb, va), ("ba", &bb, &ab, vb)] {
        let ops: [(&str, Op, f64); 3] = [
            ("U", topo::union_with, va + vb - common),
            ("I", topo::intersect_with, common),
            ("S", topo::subtract_with, vx - common),
        ];
        for (op, run, want) in ops {
            let tag = format!("{} {} {what} {order} {op}", a.name, b.name);
            eprintln!("R1XRUN {tag}");
            let res =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run(x, y, &d, tol())));
            let line = match res {
                Ok(res) => outcome(res, want, tol()),
                Err(_) => "PANIC".into(),
            };
            out.push(format!("{tag}: {line}"));
        }
    }
    out
}

/// A seeded uniform rotation (Shoemake), as the images of the axes.
fn rand_rot(seed: &mut u64) -> M3 {
    let mut next = || {
        *seed = seed
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((*seed >> 11) as f64) / ((1u64 << 53) as f64)
    };
    let (u1, u2, u3) = (next(), next(), next());
    let tau = std::f64::consts::TAU;
    let (a, b) = ((1.0 - u1).sqrt(), u1.sqrt());
    let (x, y, z, w) = (
        a * (tau * u2).sin(),
        a * (tau * u2).cos(),
        b * (tau * u3).sin(),
        b * (tau * u3).cos(),
    );
    // Rotation matrix rows; its columns are the axes' images.
    let m = [
        [
            1.0 - 2.0 * (y * y + z * z),
            2.0 * (x * y - z * w),
            2.0 * (x * z + y * w),
        ],
        [
            2.0 * (x * y + z * w),
            1.0 - 2.0 * (x * x + z * z),
            2.0 * (y * z - x * w),
        ],
        [
            2.0 * (x * z - y * w),
            2.0 * (y * z + x * w),
            1.0 - 2.0 * (x * x + y * y),
        ],
    ];
    [0, 1, 2].map(|c| [m[0][c], m[1][c], m[2][c]])
}

/// The pierce sweep's grid direction.
fn direction(i: u32, j: u32) -> V3 {
    let theta = std::f64::consts::TAU * (f64::from(i) + 0.37) / 12.0;
    let phi = (f64::from(j) - 3.0) * 0.4 + 0.05;
    [theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()]
}
fn r2dir(i: u32, j: u32) -> V3 {
    let theta = std::f64::consts::TAU * (f64::from(i) + 0.11) / 12.0;
    let phi = (f64::from(j) - 3.0) * 0.43 + 0.02;
    [theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()]
}

/// The work of one set: `(a, b, frame, label)`.
fn work(set: &str) -> Vec<(&'static str, &'static str, M3, String)> {
    let mut out = Vec::new();
    match set {
        "rand" => {
            let pairs: [(&str, &str, usize); 16] = [
                ("n343", "cubeC", 400),
                ("n343", "cubeE", 400),
                ("n330", "cubeC", 300),
                ("n330", "cubeE", 300),
                ("n350", "cubeE", 300),
                ("n357", "cubeE", 300),
                ("n300", "cubeE", 300),
                ("s343", "cubeC", 400),
                ("s343", "cubeE", 400),
                ("s320", "cubeE", 300),
                ("valley4", "cubeC", 300),
                ("valley4", "cubeE", 300),
                ("n343", "n330", 500),
                ("n343", "L270", 500),
                ("s343", "sh200", 500),
                ("valley4", "n343", 500),
            ];
            let mut seed = 0x5eed_0f_4050u64;
            for (a, b, count) in pairs {
                for k in 0..count {
                    out.push((a, b, rand_rot(&mut seed), format!("k={k}")));
                }
            }
        }
        "tilt" => {
            // The pin's three poses and a few more six-crossing ones,
            // each tilted by eps about six seeded axes. The pierce
            // sweep's frame `(u, w, m)` maps the cube's local axes.
            let base: [(&str, &str, V3, f64); 6] = [
                ("p343", "cubeE", direction(3, 0), 1.0),
                ("p343", "cubeC", direction(0, 0), 2.2),
                ("p343", "cubeE", direction(3, 0), 0.0),
                ("p343", "cubeE", direction(2, 0), 2.2),
                ("p343", "cubeC", direction(3, 1), 1.0),
                ("n345", "cubeE", r2dir(3, 0), 0.3),
            ];
            let mut seed = 0x7117u64;
            for (t, (a, b, m, psi)) in base.into_iter().enumerate() {
                let f = frame(m, psi);
                out.push((a, b, f, format!("base={t} eps=0")));
                for e in [1e-3, 1e-5, 1e-7, 1e-9, 1e-11] {
                    for x in 0..6 {
                        let ax = rand_rot(&mut seed)[0];
                        let g = f.map(|c| rot(c, ax, e));
                        out.push((a, b, g, format!("base={t} eps={e:e} ax={x}")));
                    }
                }
            }
        }
        "turn" => {
            // Every grid direction, fine turns: notch angles the PR's
            // batteries did not use.
            for a in ["n330", "n350", "s343"] {
                for b in ["cubeE", "cubeC"] {
                    for i in 0..12 {
                        for j in 0..7 {
                            for k in 0..6 {
                                let psi = f64::from(k) * 1.05 + 0.1;
                                out.push((
                                    a,
                                    b,
                                    frame(direction(i, j), psi),
                                    format!("i={i} j={j} k={k}"),
                                ));
                            }
                        }
                    }
                }
            }
        }
        _ => panic!("unknown set {set}"),
    }
    out
}

#[test]
#[ignore = "review probe; run with --ignored --nocapture"]
fn r1x_sweep() {
    let set = std::env::var("R1X_SET").unwrap_or_else(|_| "tilt".into());
    let shard: Option<(usize, usize)> = std::env::var("R1X_SHARD").ok().map(|s| {
        let (k, n) = s.split_once('/').unwrap();
        (k.parse().unwrap(), n.parse().unwrap())
    });
    let mut lines = Vec::new();
    let mut cache: Vec<Local> = Vec::new();
    for (k, (a, b, f, what)) in work(&set).into_iter().enumerate() {
        if shard.is_some_and(|(s, n)| k % n != s) {
            continue;
        }
        for name in [a, b] {
            if !cache.iter().any(|l| l.name == name) {
                cache.push(local(name));
            }
        }
        let la = cache.iter().find(|l| l.name == a).unwrap();
        let lb = cache.iter().find(|l| l.name == b).unwrap();
        lines.extend(pose_lines(la, lb, &f, &what));
    }
    let text = lines.join("\n") + "\n";
    match std::env::var("R1X_OUT") {
        Ok(p) => std::fs::write(p, text).unwrap(),
        Err(_) => print!("{text}"),
    }
}

/// The oracle reads each local operand's own volume as the kernel does.
#[test]
fn r1x_oracle_matches_the_kernel_volumes() {
    for name in [
        "n343", "p343", "s343", "s320", "sh200", "valley4", "cubeE", "L270",
    ] {
        let l = local(name);
        let body = finished(name, (l.build)(&|x| x));
        let kv = topo::mass_properties(&body, tol()).unwrap().volume;
        let ov = self_volume(&l.pieces);
        assert!((kv - ov).abs() < 1e-9, "{name}: oracle {ov} kernel {kv}");
    }
}

/// **A nested plan at a shared vertex.** A pinch operand: two cubes
/// touching only at their corners `v`, opposite octants of one frame,
/// united undeclared; the 343° notch's corner at `v`. Where the notch
/// nests its pairing against one cube's corner and also crosses the
/// other's, the notch's vertex is shared by two crossing plans. Prints
/// one line per run; the pinch's own records are not carried, so its
/// 3′ column is read as information only.
#[test]
#[ignore = "review probe; run with --ignored --nocapture"]
fn r1x_shared_vertex() {
    let a = local("p343");
    let v = [1.0, 1.0, 1.0];
    let id: M3 = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let pa = moved(&a.pieces, &id, v);
    let ab = finished(
        "notch",
        (a.build)(&|x| [x[0] + v[0], x[1] + v[1], x[2] + v[2]]),
    );
    let d = BooleanDeclarations::default();
    let mut tally = std::collections::BTreeMap::new();
    for i in 0..12 {
        for j in 0..7 {
            for k in 0..6 {
                let psi = f64::from(k) * 1.05 + 0.1;
                let f = frame(direction(i, j), psi);
                let (c1, c2) = (cube("c1", [0.0; 3]), cube("c2", [-SIDE; 3]));
                let at = |x: V3| {
                    let y = apply(&f, x);
                    [y[0] + v[0], y[1] + v[1], y[2] + v[2]]
                };
                let (b1, b2) = (
                    finished("c1", (c1.build)(&at)),
                    finished("c2", (c2.build)(&at)),
                );
                let pinch = match topo::union_with(&b1, &b2, &d, tol()) {
                    Ok(r) => match r.body() {
                        Some(bb) => bb.body.clone(),
                        None => continue,
                    },
                    Err(e) => {
                        println!("i={i} j={j} k={k}: PINCH UNBUILT {e:?}");
                        continue;
                    }
                };
                let pb: Vec<(f64, Vec<Half>)> = moved(&c1.pieces, &f, v)
                    .into_iter()
                    .chain(moved(&c2.pieces, &f, v))
                    .collect();
                let (va, vb) = (self_volume(&pa), self_volume(&pb));
                let common = pair_volume(&pa, &pb);
                for (order, x, y, vx) in [("ab", &ab, &pinch, va), ("ba", &pinch, &ab, vb)] {
                    let ops: [(&str, Op, f64); 3] = [
                        ("U", topo::union_with, va + vb - common),
                        ("I", topo::intersect_with, common),
                        ("S", topo::subtract_with, vx - common),
                    ];
                    for (op, run, want) in ops {
                        let tag = format!("p343 pinch i={i} j={j} k={k} {order} {op}");
                        eprintln!("R1XRUN {tag}");
                        let line = outcome(run(x, y, &d, tol()), want, tol());
                        let key: String =
                            line.split([' ', '{']).take(2).collect::<Vec<_>>().join(" ");
                        *tally.entry(key).or_insert(0) += 1;
                        println!("{tag}: {line}");
                    }
                }
            }
        }
    }
    println!("TALLY {tally:?}");
}
