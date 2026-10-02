//! JOIN-1 fix-pass delta review: probes of the `AlongEdge` lane, the
//! minted edge-edge record and the restated scaffold. Every building
//! pose is read at tiers 2, 3′ and the at-rest certificate, against a
//! polygonised closed-form volume, AND for the seams a restated
//! scaffold could hide: an edge between two faces of ONE planar
//! surface key (a face cut in two), an edge between two faces whose
//! planes coincide (a smooth planar seam), and a zero-length open
//! edge. `#[ignore]`d batteries print one line per pose so two trees
//! can be diffed.
//!
//! Measured (release) on 21b7f289 against its merge parent da396111f:
//! the arc battery (7350 ops) moves 50 refusals to sound bodies, one
//! sound body to a refusal (the lens row below), no BAD on either; the
//! brick battery (6000 ops) moves 8 `RestZipUnsupported` to sound, no
//! other move, no BAD.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::type_complexity
)]

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{Extrusion, extrude};
use topo::Body;

fn tol() -> Tol {
    Tol::witness()
}

/// A z-prism over a bulge loop `(x, y, bulge)`, shifted by `(dx, dy)`,
/// spanning `z`.
fn zprism(pts: &[(f64, f64, f64)], d: (f64, f64), z: (f64, f64)) -> Body<f64> {
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z.0)));
    let lp = bulge_loop(
        pts.iter()
            .map(|&(x, y, b)| (Point2::new(x + d.0, y + d.1), b))
            .collect(),
    );
    let p = Profile::new(plane, vec![lp]).validate(tol()).unwrap();
    extrude(&p, Extrusion::Distance(z.1 - z.0), tol())
        .unwrap()
        .body
}

/// The loop polygonised (each bulge arc at 4096 chords).
fn polygon(pts: &[(f64, f64, f64)], d: (f64, f64)) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for i in 0..pts.len() {
        let (x0, y0, b) = pts[i];
        let (x1, y1, _) = pts[(i + 1) % pts.len()];
        out.push((x0 + d.0, y0 + d.1));
        if b == 0.0 {
            continue;
        }
        // Bulge b: included angle θ = 4·atan(b), CCW for b > 0.
        let theta = 4.0 * b.atan();
        let (mx, my) = ((x0 + x1) / 2.0, (y0 + y1) / 2.0);
        let (cx_, cy_) = (x1 - x0, y1 - y0);
        let chord = (cx_ * cx_ + cy_ * cy_).sqrt();
        let r = chord / (2.0 * (theta / 2.0).sin());
        // centre: left of the chord for a CCW arc of < π
        let h = r * (theta / 2.0).cos();
        let (nx, ny) = (-cy_ / chord, cx_ / chord);
        let (ccx, ccy) = (mx + nx * h, my + ny * h);
        let a0 = (y0 - ccy).atan2(x0 - ccx);
        let n = 4096;
        for k in 1..n {
            let t = a0 + theta * (k as f64) / (n as f64);
            out.push((ccx + r.abs() * t.cos() + d.0, ccy + r.abs() * t.sin() + d.1));
        }
    }
    out
}

fn area(p: &[(f64, f64)]) -> f64 {
    let n = p.len();
    0.5 * (0..n)
        .map(|i| p[i].0 * p[(i + 1) % n].1 - p[(i + 1) % n].0 * p[i].1)
        .sum::<f64>()
}

fn clip_convex(subject: &[(f64, f64)], clipper: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut out = subject.to_vec();
    for i in 0..clipper.len() {
        let (a, b) = (clipper[i], clipper[(i + 1) % clipper.len()]);
        let side = |p: (f64, f64)| (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0);
        let inp = out.clone();
        out.clear();
        for j in 0..inp.len() {
            let (p, q) = (inp[(j + inp.len() - 1) % inp.len()], inp[j]);
            let (sp, sq) = (side(p), side(q));
            if sq >= 0.0 {
                if sp < 0.0 {
                    let t = sp / (sp - sq);
                    out.push((p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1)));
                }
                out.push(q);
            } else if sp >= 0.0 {
                let t = sp / (sp - sq);
                out.push((p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1)));
            }
        }
        if out.is_empty() {
            return out;
        }
    }
    out
}

/// The seams a restated scaffold could hide: (edges between two faces
/// of one planar surface key, edges between two faces on one plane,
/// zero-length open edges).
pub(crate) fn seams(body: &Body<f64>) -> (usize, usize, usize) {
    let (mut same_key, mut coplanar, mut zero) = (0, 0, 0);
    for (_, e) in body.edges() {
        let ends = (
            body.get_half_edge(e.he_plus).map(|h| h.start),
            body.half_edge_end(e.he_plus),
        );
        if let (Some(a), Some(b)) = ends
            && a != b
        {
            let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            if (p(a) - p(b)).norm() < 1e-9 {
                zero += 1;
            }
        }
        let (Some(f1), Some(f2)) = (
            body.face_of_half_edge(e.he_plus),
            body.face_of_half_edge(e.he_minus),
        ) else {
            continue;
        };
        if f1 == f2 {
            continue;
        }
        let (k1, k2) = (
            body.get_face(f1).unwrap().surface,
            body.get_face(f2).unwrap().surface,
        );
        let plane = |k| match body.get_surface(k) {
            Some(geom::Surface::Plane { origin, normal, .. }) => Some((*origin, *normal)),
            _ => None,
        };
        if let (Some((o1, n1)), Some((o2, n2))) = (plane(k1), plane(k2)) {
            if k1 == k2 {
                same_key += 1;
            } else if n1.cross(n2).norm() < 1e-9 && (o2 - o1).dot(n1).abs() < 1e-9 {
                coplanar += 1;
            }
        }
    }
    (same_key, coplanar, zero)
}

pub(crate) fn outcome(
    r: Result<topo::BooleanResult<f64>, topo::BooleanError>,
    want: f64,
    vtol: f64,
) -> String {
    match r {
        Err(e) => {
            let s = format!("{:?}", e.kind());
            let cut: String = s.chars().take(90).collect();
            format!("ERR {cut}")
        }
        Ok(r) => match r.body() {
            None => {
                if want.abs() < vtol {
                    "EMPTY ok".into()
                } else {
                    format!("EMPTY WRONG want={want}")
                }
            }
            Some(bb) => {
                let t2 = topo::validate_closed(&bb.body).is_ok();
                let t3 = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol());
                let t3s = match &t3 {
                    Ok(()) => "ok".to_string(),
                    Err(e) => format!("{:?}", e).chars().take(60).collect(),
                };
                let cert = topo::validate_geometric_certificate(&bb.body, tol()).is_ok();
                let (sk, cp, z) = seams(&bb.body);
                match topo::mass_properties(&bb.body, tol()).map(|m| m.volume) {
                    Ok(v) => {
                        let good = (v - want).abs() < vtol;
                        format!(
                            "OK {} t2={t2} t3p={t3s} cert={cert} samekey={sk} coplanar={cp} \
                             zero={z} v={v:.7} want={want:.7}",
                            if good && t2 && t3.is_ok() && sk == 0 && z == 0 {
                                "SOUND"
                            } else {
                                "BAD"
                            }
                        )
                    }
                    Err(e) => format!(
                        "OK-UNMEASURED t2={t2} t3p={t3s} cert={cert} samekey={sk} \
                         coplanar={cp} zero={z} {:?}",
                        e
                    )
                    .chars()
                    .take(200)
                    .collect(),
                }
            }
        },
    }
}

/// Bulge of a CCW arc of included angle `deg`.
fn bulge(deg: f64) -> f64 {
    (deg.to_radians() / 4.0).tan()
}

/// Convex arc-bounded profiles on and around the circle of radius 0.5
/// at the origin: shared arcs, arcs of other radii through the same
/// two points, an arc against the straight chord through its ends.
fn arc_shapes() -> Vec<(&'static str, Vec<(f64, f64, f64)>)> {
    vec![
        // upper half disc: arc (0.5,0) → (−0.5,0) CCW, chord back.
        ("half", vec![(0.5, 0.0, bulge(180.0)), (-0.5, 0.0, 0.0)]),
        // lower half disc.
        ("lower", vec![(-0.5, 0.0, bulge(180.0)), (0.5, 0.0, 0.0)]),
        // a shallower arc (radius 0.707) through the same two points.
        ("shallow", vec![(0.5, 0.0, bulge(90.0)), (-0.5, 0.0, 0.0)]),
        // a quarter sector of the same circle.
        (
            "quarter",
            vec![(0.0, 0.0, 0.0), (0.5, 0.0, bulge(90.0)), (0.0, 0.5, 0.0)],
        ),
        // a square holding the half disc's chord.
        (
            "square",
            vec![
                (-0.5, -1.0, 0.0),
                (0.5, -1.0, 0.0),
                (0.5, 0.0, 0.0),
                (-0.5, 0.0, 0.0),
            ],
        ),
        // the triangle over the chord.
        (
            "tri",
            vec![(0.5, 0.0, 0.0), (0.0, 0.5, 0.0), (-0.5, 0.0, 0.0)],
        ),
        // a lens: two arcs of the radius-0.707 circle family.
        (
            "lens",
            vec![(0.5, 0.0, bulge(90.0)), (-0.5, 0.0, bulge(90.0))],
        ),
    ]
}

#[test]
#[ignore = "differential battery; run with --ignored"]
fn join1_delta_arc_battery() {
    use topo::flush::{declare_all, find_flush_candidates};
    let shapes = arc_shapes();
    let offsets = [(0.0, 0.0), (0.5, 0.0), (-0.5, 0.0), (0.0, 0.5), (1.0, 0.0)];
    let za = (0.0, 2.0);
    let zbs = [(0.0, 2.0), (1.0, 3.0), (2.0, 4.0), (-1.0, 1.0), (0.5, 1.5)];
    let only = std::env::var("J1D_SHAPE").ok();
    for (na, pa) in &shapes {
        if only.as_deref().is_some_and(|o| o != *na) {
            continue;
        }
        let a = zprism(pa, (0.0, 0.0), za);
        let polya = polygon(pa, (0.0, 0.0));
        let va = area(&polya) * (za.1 - za.0);
        for (nb, pb) in &shapes {
            for &d in &offsets {
                let polyb = polygon(pb, d);
                let ab = area(&clip_convex(&polya, &polyb)).max(0.0);
                for &zb in &zbs {
                    let b = zprism(pb, d, zb);
                    let vb = area(&polyb) * (zb.1 - zb.0);
                    let vi = ab * (za.1.min(zb.1) - za.0.max(zb.0)).max(0.0);
                    for decl in [false, true] {
                        let d_ = if decl {
                            match find_flush_candidates(&a, &b, tol()) {
                                Ok(f) => Some(declare_all(&f)),
                                Err(e) => {
                                    println!(
                                        "ARC {na} {nb} d={d:?} zb={zb:?} decl FLUSHERR {:?}",
                                        e
                                    );
                                    continue;
                                }
                            }
                        } else {
                            None
                        };
                        for (op, want) in [("U", va + vb - vi), ("S", va - vi), ("I", vi)] {
                            let res = match (&d_, op) {
                                (None, "U") => topo::union(&a, &b, tol()),
                                (None, "S") => topo::subtract(&a, &b, tol()),
                                (None, _) => topo::intersect(&a, &b, tol()),
                                (Some(dd), "U") => topo::union_with(&a, &b, dd, tol()),
                                (Some(dd), "S") => topo::subtract_with(&a, &b, dd, tol()),
                                (Some(dd), _) => topo::intersect_with(&a, &b, dd, tol()),
                            };
                            println!(
                                "ARC {na} {nb} d={d:?} zb={zb:?} decl={decl} {op} => {}",
                                outcome(res, want, 1e-6)
                            );
                        }
                    }
                }
            }
        }
    }
}

/// Brick against brick: corners, edges and faces shared, caps resting
/// (the declared-REST zip's ground) and overlapping, undeclared and
/// flush-declared, every op in both orders.
#[test]
#[ignore = "differential battery; run with --ignored"]
fn join1_delta_brick_battery() {
    use sweep::test_support::brick;
    use topo::flush::{declare_all, find_flush_candidates};
    let coords = [-0.5, 0.0, 0.5, 1.0, 1.5];
    let mut ranges = Vec::new();
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            ranges.push((coords[i], coords[j]));
        }
    }
    let a = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol());
    let ov = |p: (f64, f64), q: (f64, f64)| (p.1.min(q.1) - p.0.max(q.0)).max(0.0);
    let zs = [(1.0, 2.0), (0.0, 1.0), (0.5, 1.5), (-1.0, 0.0), (0.0, 0.5)];
    for &x in &ranges {
        for &y in &ranges {
            for &z in &zs {
                let b = brick(x, y, z, tol());
                let vb = (x.1 - x.0) * (y.1 - y.0) * (z.1 - z.0);
                let vi = ov((0.0, 1.0), x) * ov((0.0, 1.0), y) * ov((0.0, 1.0), z);
                for decl in [false, true] {
                    for (order, l, r) in [("AB", &a, &b), ("BA", &b, &a)] {
                        let d_ = if decl {
                            match find_flush_candidates(l, r, tol()) {
                                Ok(f) => Some(declare_all(&f)),
                                Err(e) => {
                                    println!(
                                        "BRICK x={x:?} y={y:?} z={z:?} {order} FLUSHERR {e:?}"
                                    );
                                    continue;
                                }
                            }
                        } else {
                            None
                        };
                        let (vl, vr) = if order == "AB" { (1.0, vb) } else { (vb, 1.0) };
                        for (op, want) in [("U", vl + vr - vi), ("S", vl - vi), ("I", vi)] {
                            let res = match (&d_, op) {
                                (None, "U") => topo::union(l, r, tol()),
                                (None, "S") => topo::subtract(l, r, tol()),
                                (None, _) => topo::intersect(l, r, tol()),
                                (Some(dd), "U") => topo::union_with(l, r, dd, tol()),
                                (Some(dd), "S") => topo::subtract_with(l, r, dd, tol()),
                                (Some(dd), _) => topo::intersect_with(l, r, dd, tol()),
                            };
                            println!(
                                "BRICK x={x:?} y={y:?} z={z:?} decl={decl} {order} {op} => {}",
                                outcome(res, want, 1e-9)
                            );
                        }
                    }
                }
            }
        }
    }
}

/// Two lens prisms on one profile (two 90° arcs of radius 0.707 through
/// (±0.5, 0)), overlapping in `z ∈ [0, 1]`, every flush pair declared:
/// the union is the lens prism over `z ∈ [−1, 2]`. **RED on 21b7f289**:
/// the merge parent of JOIN-1's fix-pass head (`da396111f`, main)
/// builds it sound; the head refuses `Join(SectionLoopMixed)`, the
/// guard `boolean::join::loop_roles` documents as a kernel defect that
/// "no row reaches". The head with the fix-pass delta reverted refuses
/// the same way: the move predates the fix pass.
#[test]
fn overlapping_lens_prisms_declared_union_builds() {
    use topo::flush::{declare_all, find_flush_candidates};
    let lens = [(0.5, 0.0, bulge(90.0)), (-0.5, 0.0, bulge(90.0))];
    let a = zprism(&lens, (0.0, 0.0), (0.0, 2.0));
    let b = zprism(&lens, (0.0, 0.0), (-1.0, 1.0));
    let want = area(&polygon(&lens, (0.0, 0.0))) * 3.0;
    let d = declare_all(&find_flush_candidates(&a, &b, tol()).unwrap());
    let r = topo::union_with(&a, &b, &d, tol());
    if let Err(e) = &r {
        println!("[lens] {e:?}");
    }
    let line = outcome(r, want, 1e-6);
    println!("[lens] {line}");
    assert!(line.starts_with("OK SOUND"), "{line}");
    // A legal operand: the union of the result with a far brick runs.
    let r = topo::union_with(&a, &b, &d, tol()).unwrap();
    let bb = r.body().unwrap();
    let far = sweep::test_support::brick((50.0, 51.0), (50.0, 51.0), (0.0, 1.0), tol());
    topo::union(&bb.body, &far, tol())
        .unwrap_or_else(|e| panic!("the lens union is a legal operand: {e:?}"));
}

/// The hexagon ∪ box corner-edge pose (`join1_r1_rows`) builds on the
/// head with the minted chord restated on a smooth seam between two
/// coplanar faces of different keys (`y = −0.5`). The row reads that
/// seam and then uses the union as an OPERAND, with a brick across the
/// seam: what a caller does next with the body. **RED on 21b7f289**:
/// every op refuses `CoplanarNeighbours` (the maximal-faces gate),
/// against DESIGN.md's "every boolean output is a legal boolean
/// operand" and "a same-sense cosurface adjacency it has no licence for
/// refuses at the op that would create it". Main refuses the union.
#[test]
fn the_declared_seam_body_is_an_operand() {
    use sweep::test_support::brick;
    let hex: [(f64, f64, f64); 6] = [
        (0.5, 0.0, 0.0),
        (0.25, 0.5, 0.0),
        (-0.25, 0.5, 0.0),
        (-0.5, 0.0, 0.0),
        (-0.25, -0.5, 0.0),
        (0.25, -0.5, 0.0),
    ];
    let a = zprism(&hex, (0.0, 0.0), (0.0, 2.0));
    let b = brick((-0.5, -0.25), (-0.5, -0.25), (-1.0, 3.0), tol());
    // Fix pass 2: undeclared, the union refuses the continuation it
    // would keep; declared, the merge stage glues it.
    assert!(
        matches!(
            topo::union(&a, &b, tol()),
            Err(topo::BooleanError::UndeclaredCoincidence { .. })
        ),
        "the undeclared continuation refuses at the op"
    );
    let d = topo::flush::declare_all(&topo::flush::find_flush_candidates(&a, &b, tol()).unwrap());
    let r = match topo::union_with(&a, &b, &d, tol()).unwrap() {
        topo::BooleanResult::Body(bb) => bb,
        topo::BooleanResult::Empty => panic!("empty"),
    };
    let (sk, cp, z) = seams(&r.body);
    println!("[seam] hex ∪ box: samekey={sk} coplanar={cp} zero={z}");
    let vr = topo::mass_properties(&r.body, tol()).unwrap().volume;
    assert!((vr - 1.71875).abs() < 1e-9, "{vr}");
    let c = brick((-0.4, -0.1), (-0.7, -0.3), (0.5, 1.5), tol());
    let lines = [
        (
            "∪",
            outcome(topo::union(&r.body, &c, tol()), vr + 0.12 - 0.06, 1e-9),
        ),
        (
            "∖",
            outcome(topo::subtract(&r.body, &c, tol()), vr - 0.06, 1e-9),
        ),
        (
            "∩",
            outcome(topo::intersect(&r.body, &c, tol()), 0.06, 1e-9),
        ),
    ];
    for (op, l) in &lines {
        println!("[seam] (hex ∪ box) {op} brick: {l}");
    }
    assert!(
        lines.iter().all(|(_, l)| l.starts_with("OK SOUND")),
        "{lines:#?}"
    );
}

/// The mechanism rows' peg ∪ collar bodies (`join1_mechanisms`): the
/// flush bottom caps (peg disk, collar annulus) are coplanar and
/// undeclared. Read the shipped body's seams, and use it as an operand
/// against a disjoint brick (no contact at all: only the operand gates
/// run on it). Red on main (the REST zip's body) and on 21b7f289 (the
/// join's) alike: both refuse `CoplanarNeighbours`. Not JOIN-1's.
#[test]
// Red on main and on 21b7f289 with the walls alone declared; JOIN-1's
// fix pass 2 refuses that union (the caps are an undeclared
// continuation), and `wall_decls` now declares the caps beside the
// walls, which the merge stage glues: green, so no longer ignored.
fn the_peg_collar_unions_are_operands() {
    use crate::mate2_common::{collar_at, peg_at, wall_decls};
    use sweep::test_support::brick;
    let far = brick((5.0, 6.0), (5.0, 6.0), (0.0, 1.0), tol());
    let mut lines = Vec::new();
    for (what, h) in [("proud", 1.5), ("flush", 1.0)] {
        let (c, p) = (collar_at(0.0), peg_at(0.0, 1.0, h));
        let r = match topo::union_with(&c, &p, &wall_decls(&c, &p), tol()).unwrap() {
            topo::BooleanResult::Body(bb) => bb,
            topo::BooleanResult::Empty => panic!("empty"),
        };
        let (sk, cp, z) = seams(&r.body);
        let vr = topo::mass_properties(&r.body, tol()).unwrap().volume;
        let next = outcome(topo::union(&r.body, &far, tol()), vr + 1.0, 1e-9);
        let l = format!("{what}: samekey={sk} coplanar={cp} zero={z}; with a far brick: {next}");
        println!("[peg] {l}");
        lines.push(l);
    }
    assert!(lines.iter().all(|l| l.contains("OK SOUND")), "{lines:#?}");
}
