//! PR 4249 review r1's probes (review branch only): every built body is
//! read through `outcome`, the exact cone count at the shared point, the
//! vertices there, their point key, the faces through two of them, and
//! the mesh check. Printed, for a diff between two trees.

use super::*;
use crate::common::pinch_cones::cones_at;

type Ps = Vec<Vec<([f64; 3], f64)>>;

/// The cone reading of one run's body at `at`, empty for a refusal.
fn detail(r: &Result<BooleanResult<f64>, BooleanError>, at: [f64; 3], tag: &str, x: &Ps, y: &Ps) -> String {
    let Some(bb) = r.as_ref().ok().and_then(BooleanResult::body) else {
        return String::new();
    };
    let (p, q, op) = tag_cones(tag, "ab", (x, y));
    let held = vertices_at(&bb.body, at).len();
    let c = cones_at(at, p, q, op);
    let key = shared_point_finding(&bb.body, at).is_none();
    let f2 = faces_through_two_vertices_at(&bb.body, at);
    let mesh = mesh::tessellate(&bb.body, 0.05, tol())
        .map(|m| mesh::validate::check_mesh(&m).is_ok())
        .unwrap_or(false);
    let ok = c.is_ok_and(|(c, free)| (c - free..=c).contains(&held)) && key && mesh;
    format!(
        " | {} vat={held} cones={c:?} onekey={key} faces2={f2} mesh={mesh}",
        if ok { "CONEOK" } else { "CONEBAD" }
    )
}

/// Every op in both orders between `x` and `y`, printed with [`detail`].
fn print_runs(name: &str, at: [f64; 3], (x, xp): (&AtRestBody<f64>, &Ps), (y, yp): (&AtRestBody<f64>, &Ps)) {
    let vol = |ps: &Ps| ps.iter().map(|p| convex_volume(p)).sum::<f64>();
    let (vx, vy) = (vol(xp), vol(yp));
    let common: f64 = xp
        .iter()
        .flat_map(|p| {
            yp.iter().map(move |q| {
                let mut all = p.clone();
                all.extend_from_slice(q);
                convex_volume(&all)
            })
        })
        .sum();
    let decls = BooleanDeclarations::default();
    let _ = shared_point_spread_finding();
    for (order, l, r, vl) in [("ab", x, y, vx), ("ba", y, x, vy)] {
        let ops: [(&str, Op, f64); 3] = [
            ("U", topo::union_with, vx + vy - common),
            ("I", topo::intersect_with, common),
            ("S", topo::subtract_with, vl - common),
        ];
        for (op, run, want) in ops {
            let tag = format!("{order} {op}");
            let res = run(l, r, &decls, tol());
            let d = detail(&res, at, &tag, xp, yp);
            println!("{name} {tag}: {}{d}", outcome(res, want, tol()));
        }
    }
    if let (_, Some(f)) = shared_point_spread_finding() {
        println!("{name} SPREAD {f}");
    }
}

fn notch_pieces() -> Ps {
    notch343().pieces.iter().map(|p| polygon_prism(p)).collect()
}

fn notch_body() -> AtRestBody<f64> {
    finished("the notch", fixtures::prism::<f64>(&notch343().profile, 1.0, tol()).body)
}

/// A right-handed frame whose cube corner diagonal runs along `d`,
/// twisted by `psi` about it.
fn diag_frame(d: [f64; 3], psi: f64) -> [[f64; 3]; 3] {
    let [p, q, a] = frame(d, 0.0);
    let s = (2.0f64 / 3.0).sqrt();
    let e = |k: f64| {
        let t = psi + k * std::f64::consts::TAU / 3.0;
        [0, 1, 2].map(|i| a[i] / 3f64.sqrt() + s * (t.cos() * p[i] + t.sin() * q[i]))
    };
    let (e0, e1, e2) = (e(0.0), e(1.0), e(2.0));
    if dot(cross(e0, e1), e2) > 0.0 { [e0, e1, e2] } else { [e1, e0, e2] }
}

/// Unites bodies one by one, `None` (printed) where one refuses.
fn unite(name: &str, bodies: Vec<Body<f64>>) -> Option<AtRestBody<f64>> {
    let decls = BooleanDeclarations::default();
    let mut it = bodies.into_iter().map(|b| finished("a part", b));
    let mut acc = it.next()?;
    for b in it {
        match topo::union_with(&acc, &b, &decls, tol()) {
            Ok(BooleanResult::Body(bb)) => acc = bb.body,
            other => {
                println!("{name} OPERAND {:?}", other.map(|_| "not a body").err());
                return None;
            }
        }
    }
    Some(acc)
}

/// Three pairs at one vertex: the notch's corner against three cubes
/// whose corners touch there only, diagonals 120° apart in the plane
/// normal to the grid direction.
#[test]
#[ignore = "review probe"]
fn r1_three_cubes_at_one_vertex() {
    let v = notch343().v;
    for i in 0..12 {
        for j in 0..7 {
            for k in 0..2 {
                let n = direction(i, j);
                let [p, q, _] = frame(n, 0.3 + f64::from(k) * 0.9);
                let mut cubes = Vec::new();
                let mut planes = Vec::new();
                for t in 0..3 {
                    let th = f64::from(t) * std::f64::consts::TAU / 3.0;
                    let d = [0, 1, 2].map(|c| th.cos() * p[c] + th.sin() * q[c]);
                    let f = diag_frame(d, 0.2 + f64::from(t));
                    cubes.push(cube_sized(v, f, [0.0; 3], 2.0));
                    planes.push(cube_planes_sized(v, f, [0.0; 3], 2.0));
                }
                let name = format!("three i={i} j={j} k={k}");
                if let Some(b) = unite(&name, cubes) {
                    print_runs(&name, v, (&notch_body(), &notch_pieces()), (&b, &planes));
                }
            }
        }
    }
}

/// Both operands pinched: the corner pinch of two cubes against another
/// such pinch in another frame, at one point.
#[test]
#[ignore = "review probe"]
fn r1_two_pinches_at_one_point() {
    let v = notch343().v;
    let pinch = |f: [[f64; 3]; 3]| -> (Vec<Body<f64>>, Ps) {
        (
            vec![cube_at(v, f, [0.0; 3]), cube_at(v, f, [-SIDE; 3])],
            vec![cube_planes_at(v, f, [0.0; 3]), cube_planes_at(v, f, [-SIDE; 3])],
        )
    };
    for i in 0..12 {
        for j in 0..7 {
            for k in 0..2 {
                let f1 = frame(direction(i, j), 0.1 + 2.1 * f64::from(k));
                let f2 = frame(direction((i + 5) % 12, (j + 3) % 7), 1.3 + f64::from(k));
                let name = format!("two-pinch i={i} j={j} k={k}");
                let (b1, p1) = pinch(f1);
                let (b2, p2) = pinch(f2);
                if let (Some(x), Some(y)) = (unite(&name, b1), unite(&name, b2)) {
                    print_runs(&name, v, (&x, &p1), (&y, &p2));
                }
            }
        }
    }
}

/// The notch (and reflex wedges) against a pinch of two thin wedges whose
/// apexes touch at the corner: more crossings per pair, so a turned run
/// may hold two of its pair's runs.
#[test]
#[ignore = "review probe"]
fn r1_wedge_pinches() {
    let corners: [(&str, fn() -> Corner); 3] = [
        ("n343", notch343),
        ("w300", || wedge(300.0)),
        ("w345", wedge345),
    ];
    for (cname, corner) in corners {
        let c = corner();
        let x = finished("a corner", fixtures::prism::<f64>(&c.profile, 1.0, tol()).body);
        let xp: Ps = c.pieces.iter().map(|p| polygon_prism(p)).collect();
        for alpha in [50.0, 80.0] {
            for i in 0..12 {
                for j in 0..7 {
                    let f = frame(direction(i, j), 0.7 + 0.3 * f64::from(j));
                    // The second wedge: the first turned by π about its own y.
                    let g = [f[0].map(|t| -t), f[1], f[2].map(|t| -t)];
                    let w = wedge(alpha);
                    let (b1, p1) = posed(&w, f, c.v);
                    let (b2, p2) = posed(&w, g, c.v);
                    let name = format!("wpinch {cname} a={alpha} i={i} j={j}");
                    let decls = BooleanDeclarations::default();
                    let y = match topo::union_with(&b1, &b2, &decls, tol()) {
                        Ok(BooleanResult::Body(bb)) => bb.body,
                        other => {
                            println!("{name} OPERAND {:?}", other.err());
                            continue;
                        }
                    };
                    let yp: Ps = p1.into_iter().chain(p2).collect();
                    print_runs(&name, c.v, (&x, &xp), (&y, &yp));
                }
            }
        }
    }
}

/// Near-tangent tilts of the pinch battery at given poses: `psi` nudged
/// by tiny turns and the direction tilted.
#[test]
#[ignore = "review probe"]
fn r1_pinch_nudged() {
    let v = notch343().v;
    let poses: Vec<(u32, u32, u32)> = std::env::var("R1_POSES")
        .unwrap_or_default()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let n: Vec<u32> = s.split(':').map(|x| x.parse().unwrap()).collect();
            (n[0], n[1], n[2])
        })
        .collect();
    for (i, j, k) in poses {
        for d in [0.0, 1e-12, -1e-12, 1e-9, -1e-9, 1e-6, -1e-6, 1e-3, -1e-3] {
            for tilt in [0.0, 1e-7, 1e-4] {
                let psi = f64::from(k) * 1.05 + 0.1 + d;
                let m0 = direction(i, j);
                let m = [m0[0] + tilt, m0[1] - tilt, m0[2] + 0.5 * tilt];
                let f = frame(m, psi);
                let c = notch343();
                let pinch = unite(
                    "pinch",
                    vec![cube_at(c.v, f, [0.0; 3]), cube_at(c.v, f, [-SIDE; 3])],
                );
                let Some(b) = pinch else { continue };
                let bp = vec![cube_planes_at(c.v, f, [0.0; 3]), cube_planes_at(c.v, f, [-SIDE; 3])];
                let name = format!("nudge i={i} j={j} k={k} d={d:e} tilt={tilt:e}");
                print_runs(&name, v, (&notch_body(), &notch_pieces()), (&b, &bp));
            }
        }
    }
}

/// The pinch battery with the cone reading on every built body.
#[test]
#[ignore = "review probe"]
fn r1_pinch_battery_detail() {
    let v = notch343().v;
    for i in 0..12 {
        for j in 0..7 {
            for k in 0..6 {
                let psi = f64::from(k) * 1.05 + 0.1;
                let f = frame(direction(i, j), psi);
                let c = notch343();
                let Some(b) = unite("pinch", vec![cube_at(c.v, f, [0.0; 3]), cube_at(c.v, f, [-SIDE; 3])]) else {
                    continue;
                };
                let bp = vec![cube_planes_at(c.v, f, [0.0; 3]), cube_planes_at(c.v, f, [-SIDE; 3])];
                print_runs(&format!("i={i} j={j} k={k}"), v, (&notch_body(), &notch_pieces()), (&b, &bp));
            }
        }
    }
}

/// One pose of [`r1_three_cubes_at_one_vertex`] (`R1_POSE="i j k"`): the
/// tier 3′ verdict of each op that builds, with its error.
#[test]
#[ignore = "review probe"]
fn r1_three_one() {
    let v = notch343().v;
    let w: Vec<u32> = std::env::var("R1_POSE").unwrap().split(' ').map(|x| x.parse().unwrap()).collect();
    let (i, j, k) = (w[0], w[1], w[2]);
    let n = direction(i, j);
    let [p, q, _] = frame(n, 0.3 + f64::from(k) * 0.9);
    let mut cubes = Vec::new();
    for t in 0..3 {
        let th = f64::from(t) * std::f64::consts::TAU / 3.0;
        let d = [0, 1, 2].map(|c| th.cos() * p[c] + th.sin() * q[c]);
        cubes.push(cube_sized(v, diag_frame(d, 0.2 + f64::from(t)), [0.0; 3], 2.0));
    }
    let b = unite("three", cubes).unwrap();
    let decls = BooleanDeclarations::default();
    for (tag, l, r) in [("ab", &notch_body(), &b), ("ba", &b, &notch_body())] {
        let res = topo::union_with(l, r, &decls, tol());
        if let Ok(BooleanResult::Body(bb)) = &res {
            let at = vertices_at(&bb.body, v);
            let keys: Vec<_> = at.iter().map(|&k| bb.body.get_vertex(k).unwrap().point).collect();
            println!("{tag} U: vertices at v {at:?} points {keys:?}");
            println!("{tag} U: t3' {:?}", topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()));
        } else {
            println!("{tag} U: {:?}", res.err());
        }
    }
}

/// The notch with its face from `v` to `(2, 1.15)` bowed into a
/// cylinder arc (bulge `b`, sign from `R1_BULGE_SIGN`), its body and its
/// second piece's arc approximated by `n` chords.
fn curved_notch(b: f64, n: usize) -> (Ps, Vec<(f64, f64)>) {
    let (p, q) = ((1.0f64, 1.0f64), (2.0f64, 1.15f64));
    let (mx, my) = ((p.0 + q.0) / 2.0, (p.1 + q.1) / 2.0);
    let (dx, dy) = (q.0 - p.0, q.1 - p.1);
    let len = (dx * dx + dy * dy).sqrt();
    let h = len / 2.0;
    let s = h * b.abs();
    let r = (h * h + s * s) / (2.0 * s);
    // Bowed toward the right of travel (down): the centre to the left.
    let (lx, ly) = (-dy / len, dx / len);
    let c = (mx + lx * (r - s), my + ly * (r - s));
    let a0 = (p.1 - c.1).atan2(p.0 - c.0);
    let mut a1 = (q.1 - c.1).atan2(q.0 - c.0);
    // The short way, through the bottom.
    while a1 > a0 + std::f64::consts::PI {
        a1 -= std::f64::consts::TAU;
    }
    while a1 < a0 - std::f64::consts::PI {
        a1 += std::f64::consts::TAU;
    }
    let arc: Vec<(f64, f64)> = (1..n)
        .map(|t| {
            let a = a0 + (a1 - a0) * t as f64 / n as f64;
            (c.0 + r * a.cos(), c.1 + r * a.sin())
        })
        .collect();
    let mut p2 = vec![(0.0, 1.0), (1.0, 1.0)];
    p2.extend(arc.iter().copied());
    p2.extend([(2.0, 1.15), (2.0, 2.0), (0.0, 2.0)]);
    let pieces = vec![
        polygon_prism(&[(0.0, 0.0), (2.0, 0.0), (2.0, 0.85), (1.0, 1.0), (0.0, 1.0)]),
        polygon_prism(&p2),
    ];
    (pieces, arc)
}

fn curved_notch_body(b: f64) -> AtRestBody<f64> {
    use geom_core::{Affine3, Mat3, Point2, Vec3};
    let sign: f64 = std::env::var("R1_BULGE_SIGN").map_or(1.0, |s| s.parse().unwrap());
    let pts = [(0.0, 0.0, 0.0), (2.0, 0.0, 0.0), (2.0, 0.85, 0.0), (1.0, 1.0, sign * b), (2.0, 1.15, 0.0), (2.0, 2.0, 0.0), (0.0, 2.0, 0.0)];
    let lp = profile::test_support::bulge_loop(pts.iter().map(|&(x, y, g)| (Point2::new(x, y), g)).collect());
    let plane = profile::SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()),
        Vec3::new(0.0, 0.0, 0.0),
    ));
    let vp = profile::Profile::new(plane, vec![lp]).validate(tol()).expect("the curved notch validates");
    let body = sweep::extrude(
        &vp,
        sweep::Extrusion::Distance { depth: 1.0, side: sweep::ExtrudeSide::Along },
        tol(),
    )
    .expect("the curved notch extrudes")
    .body;
    sweep::test_support::finished("the curved notch", body, tol())
}

/// The curved notch against the corner pinch over the battery's grid at
/// three turns; volumes extrapolated from 48 and 96 chords.
#[test]
#[ignore = "review probe"]
fn r1_curved_notch_pinch() {
    let b = (2.5f64).to_radians().tan();
    let x = curved_notch_body(b);
    let v = notch343().v;
    let vol = |ps: &Ps| ps.iter().map(|p| convex_volume(p)).sum::<f64>();
    let (p48, _) = curved_notch(b, 48);
    let (p96, _) = curved_notch(b, 96);
    let va = (4.0 * vol(&p96) - vol(&p48)) / 3.0;
    println!("curved notch kernel volume {:.9} oracle {va:.9}", mass_properties(&x, tol()).unwrap().volume);
    let decls = BooleanDeclarations::default();
    for i in 0..12 {
        for j in 0..7 {
            for k in [0u32, 2, 4] {
                let psi = f64::from(k) * 1.05 + 0.1;
                let f = frame(direction(i, j), psi);
                let Some(y) = unite("pinch", vec![cube_at(v, f, [0.0; 3]), cube_at(v, f, [-SIDE; 3])]) else {
                    continue;
                };
                let yp = vec![cube_planes_at(v, f, [0.0; 3]), cube_planes_at(v, f, [-SIDE; 3])];
                let clip = |ps: &Ps| -> f64 {
                    ps.iter()
                        .flat_map(|p| {
                            yp.iter().map(move |q| {
                                let mut all = p.clone();
                                all.extend_from_slice(q);
                                convex_volume(&all)
                            })
                        })
                        .sum()
                };
                let common = (4.0 * clip(&p96) - clip(&p48)) / 3.0;
                let vy = 2.0 * SIDE.powi(3);
                let _ = shared_point_spread_finding();
                for (order, l, r, vl) in [("ab", &x, &y, va), ("ba", &y, &x, vy)] {
                    let ops: [(&str, Op, f64); 3] = [
                        ("U", topo::union_with, va + vy - common),
                        ("I", topo::intersect_with, common),
                        ("S", topo::subtract_with, vl - common),
                    ];
                    for (op, run, want) in ops {
                        let tag = format!("{order} {op}");
                        let res = run(l, r, &decls, tol());
                        let d = detail(&res, v, &tag, &p96, &yp);
                        println!("curved i={i} j={j} k={k} {tag}: {}{d}", outcome(res, want, tol()));
                    }
                }
            }
        }
    }
}

/// One pose of [`r1_curved_notch_pinch`] (`R1_POSE="i j k"`): tier 3′'s
/// error on `ab S`.
#[test]
#[ignore = "review probe"]
fn r1_curved_one() {
    let w: Vec<u32> = std::env::var("R1_POSE").unwrap().split(' ').map(|x| x.parse().unwrap()).collect();
    let v = notch343().v;
    let f = frame(direction(w[0], w[1]), f64::from(w[2]) * 1.05 + 0.1);
    let x = curved_notch_body((2.5f64).to_radians().tan());
    let y = unite("pinch", vec![cube_at(v, f, [0.0; 3]), cube_at(v, f, [-SIDE; 3])]).unwrap();
    let r = topo::subtract_with(&x, &y, &BooleanDeclarations::default(), tol());
    if let Ok(BooleanResult::Body(bb)) = &r {
        println!("t3' {:?}", topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()));
    }
}
