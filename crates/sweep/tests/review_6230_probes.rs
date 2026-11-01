//! **Reviewer probes for PR 6230** (`OnDeferred::answered` without
//! `sided`'s in-face-edge condition). These are poses where a deferred
//! one-carrier pair may reach the no-crossings fallback with an edge of
//! each operand inside the other's face. Each op's body is checked
//! against an oracle that does not go through the kernel's
//! classification. The operands are revolved profiles with analytic
//! membership, a point-in-polygon in `(r, y)`. The volume oracle is
//! Pappus's integral of that membership over the half-plane. The
//! membership oracle is a grid sampled against a ray-parity read of the
//! result's tessellation. A typed refusal passes; a wrong body fails.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::test_support::{finished, revolved_about_y};
use topo::{Body, BooleanResult};

fn tol() -> Tol {
    Tol::witness()
}

type Poly = Vec<(f64, f64)>;

/// An operand: its body, and its analytic membership. It is the union
/// of the `solids` (profiles in `(r, y)`, disjoint), minus the `voids`,
/// minus box `drills`.
struct Probe {
    body: Body<f64>,
    solids: Vec<Poly>,
    voids: Vec<Poly>,
    drills: Vec<([f64; 3], [f64; 3])>,
}

fn in_poly(p: &Poly, r: f64, y: f64) -> bool {
    let mut c = false;
    let n = p.len();
    for i in 0..n {
        let (a, b) = (p[i], p[(i + 1) % n]);
        if (a.1 > y) != (b.1 > y) && r < a.0 + (y - a.1) / (b.1 - a.1) * (b.0 - a.0) {
            c = !c;
        }
    }
    c
}

fn member_ry(p: &Probe, r: f64, y: f64) -> bool {
    p.solids.iter().any(|s| in_poly(s, r, y)) && !p.voids.iter().any(|s| in_poly(s, r, y))
}

fn member(p: &Probe, q: [f64; 3]) -> bool {
    let r = (q[0] * q[0] + q[2] * q[2]).sqrt();
    member_ry(p, r, q[1])
        && !p
            .drills
            .iter()
            .any(|&(lo, hi)| (0..3).all(|k| q[k] > lo[k] && q[k] < hi[k]))
}

fn pt(p: (f64, f64)) -> Point2<f64> {
    Point2::new(p.0, p.1)
}

fn prof(pts: &[(f64, f64)]) -> Body<f64> {
    revolved_about_y(
        pts.iter().map(|&p| (pt(p), 0.0)).collect(),
        sweep::Revolution::Full,
        tol(),
    )
}

fn turned(b: &Body<f64>, angle: f64) -> Body<f64> {
    if angle == 0.0 {
        return b.clone();
    }
    let map =
        Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), angle);
    topo::transform_rigid(b, &geom_core::test_support::rigid(&map, tol()), tol()).unwrap()
}

fn result_body(r: Result<BooleanResult<f64>, topo::BooleanError>, what: &str) -> Body<f64> {
    match r {
        Ok(BooleanResult::Body(b)) => b.body.into_body(),
        other => panic!("{what}: the fixture builds: {other:?}"),
    }
}

/// The union of disjoint profiles (several shells), minus `voids`.
fn shells(solids: &[&[(f64, f64)]], voids: &[&[(f64, f64)]], turn: f64) -> Probe {
    let mut body = prof(solids[0]);
    for s in &solids[1..] {
        let (a, b) = (
            finished("A", body.clone(), tol()),
            finished("B", prof(s), tol()),
        );
        body = result_body(topo::union(&a, &b, tol()), "disjoint shells");
    }
    for v in voids {
        let (a, b) = (
            finished("A", body.clone(), tol()),
            finished("B", prof(v), tol()),
        );
        body = result_body(topo::subtract(&a, &b, tol()), "void");
    }
    Probe {
        body: turned(&body, turn),
        solids: solids.iter().map(|s| s.to_vec()).collect(),
        voids: voids.iter().map(|s| s.to_vec()).collect(),
        drills: vec![],
    }
}

fn one(p: &[(f64, f64)], turn: f64) -> Probe {
    shells(&[p], &[], turn)
}

fn rod(r: f64, y0: f64, y1: f64) -> Poly {
    vec![(0.0, y0), (r, y0), (r, y1), (0.0, y1)]
}

/// Pappus: `2π ∫∫ r [op(a, b)] dr dy` over the profiles' box.
fn pappus(a: &Probe, b: &Probe, f: fn(bool, bool) -> bool, bx: [f64; 4]) -> f64 {
    let [r1, y0, y1] = [bx[1], bx[2], bx[3]];
    // Cells of 1/1000 aligned to the profiles' round coordinates.
    let d = 1e-3;
    let (nr, ny) = ((r1 / d).ceil() as usize, ((y1 - y0) / d).ceil() as usize);
    let (dr, dy) = (d, d);
    let mut s = 0.0;
    for i in 0..nr {
        let r = (i as f64 + 0.5) * dr;
        for j in 0..ny {
            let y = y0 + (j as f64 + 0.5) * dy;
            if f(member_ry(a, r, y), member_ry(b, r, y)) {
                s += r;
            }
        }
    }
    2.0 * core::f64::consts::PI * s * dr * dy
}

/// Ray parity through the triangles, along an irrational direction.
fn mesh_inside(pos: &[[f64; 3]], tris: &[[u32; 3]], q: [f64; 3]) -> bool {
    let d = [0.5773, 0.6123, 0.5401];
    let mut n = 0;
    for t in tris {
        let [a, b, c] = t.map(|i| pos[i as usize]);
        let e1 = sub(b, a);
        let e2 = sub(c, a);
        let h = cross(d, e2);
        let det = dot(e1, h);
        if det.abs() < 1e-14 {
            continue;
        }
        let s = sub(q, a);
        let u = dot(s, h) / det;
        if !(0.0..=1.0).contains(&u) {
            continue;
        }
        let qq = cross(s, e1);
        let v = dot(d, qq) / det;
        if v < 0.0 || u + v > 1.0 {
            continue;
        }
        if dot(e2, qq) / det > 0.0 {
            n += 1;
        }
    }
    n % 2 == 1
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

type Mesh = (Vec<[f64; 3]>, Vec<[u32; 3]>, f64);

fn mesh_of(body: &Body<f64>) -> Option<Mesh> {
    let m = match mesh::tessellate(body, 2e-3, tol()) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("  no mesh oracle: {e:?}");
            return None;
        }
    };
    let pos: Vec<[f64; 3]> = m.positions.iter().map(|p| [p.x, p.y, p.z]).collect();
    let tris: Vec<[u32; 3]> = m
        .patches
        .iter()
        .flat_map(|p| p.triangles.iter().copied())
        .collect();
    let vol = tris
        .iter()
        .map(|t| {
            let [a, b, c] = t.map(|i| pos[i as usize]);
            dot(a, cross(b, c)) / 6.0
        })
        .sum();
    Some((pos, tris, vol))
}

/// The `(r, y)` box of both operands' profiles: `[0, rmax, ymin, ymax]`.
fn bbox(a: &Probe, b: &Probe) -> [f64; 4] {
    let mut bx = [0.0, 0.0, f64::INFINITY, f64::NEG_INFINITY];
    for s in a.solids.iter().chain(&b.solids) {
        for &(r, y) in s {
            bx[1] = f64::max(bx[1], r);
            bx[2] = f64::min(bx[2], y);
            bx[3] = f64::max(bx[3], y);
        }
    }
    bx[1] += 0.05;
    bx[2] = (bx[2] / 0.05).floor() * 0.05 - 0.05;
    bx[3] += 0.05;
    bx
}

/// Every op of `(a, b)` against the oracle; returns the failures and
/// prints every verdict, including refusals.
fn probe(label: &str, a: &Probe, b: &Probe) -> Vec<String> {
    let (fa, fb) = (
        finished("A", a.body.clone(), tol()),
        finished("B", b.body.clone(), tol()),
    );
    let ops: [(&str, fn(bool, bool) -> bool, _); 4] = [
        ("A ∪ B", |x, y| x || y, topo::union(&fa, &fb, tol())),
        ("A ∩ B", |x, y| x && y, topo::intersect(&fa, &fb, tol())),
        ("A ∖ B", |x, y| x && !y, topo::subtract(&fa, &fb, tol())),
        ("B ∖ A", |x, y| y && !x, topo::subtract(&fb, &fa, tol())),
    ];
    let bx = bbox(a, b);
    let mut wrong = Vec::new();
    for (op, f, r) in ops {
        let want = pappus(a, b, f, bx);
        let body = match r {
            Err(e) => {
                eprintln!("{label} {op}: refused {e:?}");
                continue;
            }
            Ok(BooleanResult::Empty) => {
                if want > 1e-3 {
                    wrong.push(format!("{label} {op}: Empty, want volume {want}"));
                }
                eprintln!("{label} {op}: Empty, want {want:.6}");
                continue;
            }
            Ok(BooleanResult::Body(bb)) => {
                let t3 = topo::validate_geometric(&bb.body, tol());
                let t3p = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol());
                if t3.is_err() || t3p.is_err() {
                    wrong.push(format!("{label} {op}: unsound {t3:?} {t3p:?}"));
                }
                let kv = topo::test_support::pinned_volume("probe", &bb.body, tol()).volume;
                if (kv - want).abs() > 5e-3 * want.max(1.0) {
                    wrong.push(format!("{label} {op}: kernel volume {kv}, Pappus {want}"));
                }
                bb.body
            }
        };
        let Some((pos, tris, mv)) = mesh_of(&body) else {
            eprintln!("{label} {op}: built, membership not probed");
            continue;
        };
        let (r1, y0, y1) = (bx[1], bx[2], bx[3]);
        let n = 16;
        let ny = ((y1 - y0) / (2.0 * r1) * n as f64).ceil() as usize;
        let h = 0.01;
        let (mut bad, mut seen) = (0, 0);
        for i in 0..n {
            for j in 0..ny {
                for l in 0..n {
                    let q = [
                        -r1 + 2.0 * r1 * (i as f64 + 0.37) / n as f64,
                        y0 + (y1 - y0) * (j as f64 + 0.41) / ny as f64,
                        -r1 + 2.0 * r1 * (l as f64 + 0.29) / n as f64,
                    ];
                    let truth = |q: [f64; 3]| f(member(a, q), member(b, q));
                    let t = truth(q);
                    let near = [
                        [h, 0., 0.],
                        [-h, 0., 0.],
                        [0., h, 0.],
                        [0., -h, 0.],
                        [0., 0., h],
                        [0., 0., -h],
                    ]
                    .iter()
                    .any(|d| truth([q[0] + d[0], q[1] + d[1], q[2] + d[2]]) != t);
                    if near {
                        continue;
                    }
                    seen += 1;
                    if mesh_inside(&pos, &tris, q) != t {
                        bad += 1;
                    }
                }
            }
        }
        eprintln!(
            "{label} {op}: built, mesh volume {mv:.6}, Pappus {want:.6}, mismatches {bad}/{seen}"
        );
        if bad > 0 {
            wrong.push(format!("{label} {op}: {bad}/{seen} membership mismatches"));
        }
        if (mv - want).abs() > 1e-2 * want.max(1.0) {
            wrong.push(format!("{label} {op}: mesh volume {mv}, Pappus {want}"));
        }
    }
    wrong
}

fn run(poses: Vec<(String, Probe, Probe)>) {
    let mut wrong = Vec::new();
    for (label, a, b) in &poses {
        wrong.extend(probe(label, a, b));
        wrong.extend(probe(&format!("{label}, swapped"), b, a));
    }
    assert!(wrong.is_empty(), "wrong bodies: {wrong:#?}");
}

const STEP: &[(f64, f64)] = &[
    (0.0, -1.0),
    (1.0, -1.0),
    (1.0, 0.5),
    (0.5, 0.5),
    (0.5, 2.0),
    (0.0, 2.0),
];

/// Nested shells: a rod on the step, the step with a cavity; the step
/// as two shells (a far rod beside it); the rod as two shells (a second
/// rod inside the narrow wall, and one clear of everything).
#[test]
fn rods_on_a_step_nested_and_several_shells() {
    let mut poses = Vec::new();
    for turn in [0.0, 0.3] {
        poses.push((
            format!("rod on step, turn {turn}"),
            one(&rod(0.5, 0.0, 1.0), turn),
            one(STEP, 0.0),
        ));
        poses.push((
            format!("rod on cavitied step, turn {turn}"),
            one(&rod(0.5, 0.0, 1.0), turn),
            shells(&[STEP], &[&rod(0.3, -0.8, -0.3)], 0.0),
        ));
        poses.push((
            format!("rod around the step's cavity, turn {turn}"),
            one(&rod(0.5, -0.9, 1.0), turn),
            shells(&[STEP], &[&rod(0.3, -0.8, -0.3)], 0.0),
        ));
        poses.push((
            format!("two-shell rod on step, turn {turn}"),
            shells(
                &[
                    &rod(0.5, 0.0, 1.0),
                    &rod(0.5, 1.2, 1.8),
                    &rod(0.5, 3.0, 3.5),
                ],
                &[],
                turn,
            ),
            one(STEP, 0.0),
        ));
        poses.push((
            format!("rod on two-shell step, turn {turn}"),
            one(&rod(0.5, 0.0, 1.0), turn),
            shells(&[STEP, &rod(0.5, 3.0, 4.0)], &[], 0.0),
        ));
        poses.push((
            format!("two shells on two shells, turn {turn}"),
            shells(&[&rod(0.5, 0.0, 1.0), &rod(0.5, 3.5, 4.5)], &[], turn),
            shells(&[STEP, &rod(0.5, 3.0, 4.0)], &[], 0.0),
        ));
    }
    run(poses);
}

/// A rod sharing the narrow wall from above the step and running out of
/// its top, and one through the step's top and down into its wide part
/// (one rim in each wall, a partial overlap). Then collars hugging a rod
/// or the step's narrow wall from outside, one sitting on the annulus,
/// and a collar beside a nested rod as two shells (opposed senses).
#[test]
fn partial_overlaps_and_opposed_collars() {
    let mut poses = Vec::new();
    for turn in [0.0, 0.3] {
        poses.push((
            format!("rod out of the step's top, turn {turn}"),
            one(&rod(0.5, 1.0, 2.5), turn),
            one(STEP, 0.0),
        ));
        poses.push((
            format!("rod through the step's top and into the wide part, turn {turn}"),
            one(&rod(0.5, 0.0, 2.5), turn),
            one(STEP, 0.0),
        ));
        poses.push((
            format!("collar on a rod, turn {turn}"),
            one(&rod(0.5, 0.0, 1.0), turn),
            one(&[(0.5, 0.3), (1.0, 0.3), (1.0, 0.7), (0.5, 0.7)], 0.0),
        ));
        poses.push((
            format!("collar on the step's narrow wall, turn {turn}"),
            one(&[(0.5, 1.0), (0.8, 1.0), (0.8, 1.5), (0.5, 1.5)], turn),
            one(STEP, 0.0),
        ));
        poses.push((
            format!("collar sitting on the step's annulus, turn {turn}"),
            one(&[(0.5, 0.5), (0.8, 0.5), (0.8, 1.5), (0.5, 1.5)], turn),
            one(STEP, 0.0),
        ));
        poses.push((
            format!("collar and nested rod, turn {turn}"),
            shells(
                &[
                    &rod(0.5, 0.6, 1.4),
                    &[(0.5, 1.6), (0.8, 1.6), (0.8, 1.9), (0.5, 1.9)],
                ],
                &[],
                turn,
            ),
            one(STEP, 0.0),
        ));
    }
    run(poses);
}

/// Cones and a sphere for carriers: a frustum on a conical step, a
/// frustum on a cavitied conical step, and a ball segment on a stepped
/// ball's cap.
#[test]
fn curved_partners_on_a_step() {
    let cone_step: &[(f64, f64)] = &[
        (0.0, -1.0),
        (1.5, -1.0),
        (1.5, 0.5),
        (0.75, 0.5),
        (0.0, 2.0),
    ];
    let frustum: &[(f64, f64)] = &[(0.0, 0.0), (1.0, 0.0), (0.5, 1.0), (0.0, 1.0)];
    let high: &[(f64, f64)] = &[(0.0, 0.8), (0.6, 0.8), (0.4, 1.2), (0.0, 1.2)];
    let mut poses = Vec::new();
    for turn in [0.0, 0.3] {
        poses.push((
            format!("frustum on conical step, turn {turn}"),
            one(frustum, turn),
            one(cone_step, 0.0),
        ));
        poses.push((
            format!("frustum inside the conical part, turn {turn}"),
            one(high, turn),
            one(cone_step, 0.0),
        ));
        poses.push((
            format!("frustum on cavitied conical step, turn {turn}"),
            one(frustum, turn),
            shells(&[cone_step], &[&rod(0.3, -0.8, -0.3)], 0.0),
        ));
        poses.push((
            format!("two frusta on conical step, turn {turn}"),
            shells(&[frustum, &rod(0.3, 1.4, 1.6)], &[], turn),
            one(cone_step, 0.0),
        ));
    }
    run(poses);
}
