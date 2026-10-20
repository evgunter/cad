//! Review probes (PR 5368, reviewer R1): the fate cut over box, L,
//! ringed and curved coincident fixtures, every op, both orders. Run with
//! `--nocapture`; each row prints one line, and the order-symmetry test
//! asserts ∪ and ∩ agree up to the swap.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use geom_core::{Point2, Point3, Tol};
use profile::{Profile, SketchPlane};
use sweep::test_support::{brick, finished};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, Body, BooleanResult, FaceKey, Operand};

fn tol() -> Tol {
    Tol::witness()
}

fn cyl(cx: f64, cy: f64, r: f64, z0: f64, z1: f64) -> Body<f64> {
    let lp = profile::circle(Point2::new(cx, cy), r, tol()).unwrap();
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, z0));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol()).unwrap();
    extrude(
        &profile,
        Extrusion::Distance {
            depth: z1 - z0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body
}

fn fin(b: Body<f64>) -> AtRestBody<f64> {
    finished("probe", b, tol())
}

fn bx(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    fin(brick(x, y, z, tol()))
}

fn op_of(k: usize, x: &AtRestBody<f64>, y: &AtRestBody<f64>) -> Result<BooleanResult<f64>, topo::BooleanError> {
    match k {
        0 => topo::union(x, y, tol()),
        1 => topo::intersect(x, y, tol()),
        _ => topo::subtract(x, y, tol()),
    }
}

fn combine(k: usize, x: &AtRestBody<f64>, y: &AtRestBody<f64>) -> AtRestBody<f64> {
    match op_of(k, x, y).expect("fixture builds") {
        BooleanResult::Body(b) => b.body,
        BooleanResult::Empty => panic!("fixture empty"),
    }
}

/// Rounds every float in a Debug string to 3 decimals (and -0 to 0).
fn round_str(s: &str) -> String {
    let mut out = String::new();
    let mut num = String::new();
    let flush = |num: &mut String, out: &mut String| {
        if !num.is_empty() {
            match num.parse::<f64>() {
                Ok(v) if num.contains('.') || num.contains('e') => {
                    let r = (v * 1000.0).round() / 1000.0;
                    let r = if r == 0.0 { 0.0 } else { r };
                    out.push_str(&format!("{r}"));
                }
                _ => out.push_str(num),
            }
            num.clear();
        }
    };
    for ch in s.chars() {
        if ch.is_ascii_digit() || ch == '.' || ch == '-' || ch == 'e' && !num.is_empty() {
            num.push(ch);
        } else {
            flush(&mut num, &mut out);
            out.push(ch);
        }
    }
    flush(&mut num, &mut out);
    out
}

fn desc(body: &Body<f64>, f: FaceKey) -> String {
    match topo::face_carrier(body, f) {
        Some(c) => round_str(&format!("{c:?}")),
        None => "?".into(),
    }
}

fn pt(p: Point3<f64>) -> [i64; 3] {
    [p.x, p.y, p.z].map(|c| {
        let r = (c * 1000.0).round() as i64;
        if r == 0 { 0 } else { r }
    })
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Summary {
    volume: i64,
    faces: usize,
    tier3: bool,
    tier3p: bool,
    /// (dropped role, dropped root carrier, holder root carrier)
    subs: BTreeSet<(char, String, String)>,
    /// substituted rows that are a fragment (cut), by role
    cut_rows: BTreeSet<(char, String)>,
    /// (role, root carrier, settled bordered stretches)
    lost: BTreeSet<(char, String, Vec<[[i64; 3]; 2]>)>,
    /// covered pairs (A root, B root) of the reduction with no substituted row
    unrowed: usize,
    covered: usize,
}

/// `x op y`; `roles` names operand A then B.
fn summary(
    k: usize,
    x: &AtRestBody<f64>,
    y: &AtRestBody<f64>,
    roles: [char; 2],
) -> Result<Option<(Summary, String)>, String> {
    let r = op_of(k, x, y).map_err(|e| round_str(&format!("{e:?}")))?;
    let BooleanResult::Body(out) = r else {
        return Ok(None);
    };
    let ops = [x, y];
    let role = |o: Operand| roles[usize::from(o == Operand::B)];
    let root = |o: Operand, f: FaceKey| match o {
        Operand::A => out.naming.face_fragments_a.root(f),
        Operand::B => out.naming.face_fragments_b.root(f),
    };
    let fdesc = |o: Operand, f: FaceKey| desc(ops[usize::from(o == Operand::B)], root(o, f));
    let subs = out
        .naming
        .substituted
        .iter()
        .map(|s| (role(s.operand), fdesc(s.operand, s.face), fdesc(s.operand.other(), s.by)))
        .collect();
    let cut_rows = out
        .naming
        .substituted
        .iter()
        .filter(|s| root(s.operand, s.face) != s.face)
        .map(|s| (role(s.operand), fdesc(s.operand, s.face)))
        .collect();
    let settler = out.naming.settler(&out.body);
    let at = |v| match settler.settle(v) {
        Ok(topo::Cell::Vertex(w)) => topo::readback::vertex_point(&out.body, w).map(pt),
        _ => None,
    };
    let lost = out
        .naming
        .discards
        .iter()
        .filter(|r| !r.bordered.is_empty())
        .map(|r| {
            let mut s: Vec<[[i64; 3]; 2]> = r
                .bordered
                .iter()
                .filter_map(|&(u, w)| {
                    let mut e = [at(u)?, at(w)?];
                    e.sort();
                    Some(e)
                })
                .collect();
            s.sort();
            (role(r.operand), fdesc(r.operand, r.face), s)
        })
        .collect();
    let opk = [topo::BooleanOp::Union, topo::BooleanOp::Intersect, topo::BooleanOp::Subtract][k];
    let covered: BTreeSet<(FaceKey, FaceKey)> = topo::boolean_reduce(opk, x, y, tol())
        .map(|red| red.covered.iter().copied().collect())
        .unwrap_or_default();
    let rowed: BTreeSet<(FaceKey, FaceKey)> = out
        .naming
        .substituted
        .iter()
        .map(|s| {
            let (fa, fb) = match s.operand {
                Operand::A => (s.face, s.by),
                Operand::B => (s.by, s.face),
            };
            (root(Operand::A, fa), root(Operand::B, fb))
        })
        .collect();
    let unrowed = covered.iter().filter(|p| !rowed.contains(p)).count();
    if unrowed > 0 {
        println!(
            "UNROWED {} pairs: {:?}",
            unrowed,
            covered
                .iter()
                .filter(|p| !rowed.contains(p))
                .map(|&(fa, fb)| (desc(x, fa), desc(y, fb)))
                .collect::<Vec<_>>()
        );
    }
    let s = Summary {
        unrowed,
        covered: covered.len(),
        volume: (topo::mass_properties(&out.body, tol()).unwrap().volume * 1e6).round() as i64,
        faces: out.body.faces().count(),
        tier3: topo::validate_geometric(&out.body, tol()).is_ok(),
        tier3p: topo::validate_pseudomanifold(&out.body, &out.contacts, tol()).is_ok(),
        subs,
        cut_rows,
        lost,
    };
    Ok(Some((s, format!("{:?}", out.kind))))
}

fn fixtures() -> Vec<(&'static str, AtRestBody<f64>, AtRestBody<f64>)> {
    let mut v = Vec::new();
    v.push((
        "partial-overlap",
        bx((0.0, 2.0), (0.0, 2.0), (0.0, 1.0)),
        bx((1.0, 3.0), (1.0, 3.0), (0.0, 1.0)),
    ));
    v.push((
        "sunk-box",
        bx((0.0, 3.0), (0.0, 3.0), (0.0, 1.0)),
        bx((1.0, 2.0), (1.0, 2.0), (0.5, 1.0)),
    ));
    // An L of equal height and a box over its inner corner.
    let l = combine(
        0,
        &bx((0.0, 3.0), (0.0, 1.0), (0.0, 1.0)),
        &bx((0.0, 1.0), (0.0, 3.0), (0.0, 1.0)),
    );
    v.push(("L-and-box", l.clone(), bx((0.5, 2.0), (0.5, 2.0), (0.0, 1.0))));
    // A box whose top overlaps the L's top in an L, sunk (shorter).
    v.push(("L-and-sunk", l, bx((0.5, 2.0), (0.5, 2.0), (0.5, 1.0))));
    // A plate with a through hole; a box over the hole, equal height:
    // the plate's top holds a ring.
    let plate = combine(
        2,
        &bx((0.0, 4.0), (0.0, 4.0), (0.0, 1.0)),
        &bx((1.5, 2.5), (1.5, 2.5), (-1.0, 2.0)),
    );
    v.push(("holed-plate-and-box", plate.clone(), bx((1.0, 3.0), (1.0, 3.0), (0.0, 1.0))));
    // ... and one overlapping the hole's edge only.
    v.push(("holed-plate-and-offset", plate.clone(), bx((2.0, 3.0), (2.0, 3.0), (0.5, 1.0))));
    // A plate with a pocket (top ring around a sunk floor) and a box whose
    // top is flush with the plate top and spans the pocket.
    v.push((
        "big-and-flush-small",
        bx((-1.0, 3.0), (-1.0, 2.0), (-1.0, 1.0)),
        bx((0.0, 2.0), (0.0, 1.0), (0.0, 1.0)),
    ));
    v.push((
        "coaxial-cyl-overlap",
        fin(cyl(0.0, 0.0, 1.0, 0.0, 2.0)),
        fin(cyl(0.0, 0.0, 1.0, 1.0, 3.0)),
    ));
    v.push((
        "cyl-sunk-in-box",
        bx((0.0, 2.0), (0.0, 2.0), (0.0, 1.0)),
        fin(cyl(1.0, 1.0, 0.5, 0.5, 1.0)),
    ));
    v.push((
        "cyl-half-out-of-box-top",
        bx((0.0, 2.0), (0.0, 2.0), (0.0, 1.0)),
        fin(cyl(2.0, 1.0, 0.5, 0.5, 1.0)),
    ));
    v.push((
        "parallel-cyl-lens",
        fin(cyl(0.0, 0.0, 1.0, 0.0, 2.0)),
        fin(cyl(1.2, 0.0, 1.0, 0.0, 2.0)),
    ));
    v.push((
        "box-sunk-in-cyl",
        fin(cyl(0.0, 0.0, 1.0, 0.0, 2.0)),
        bx((-0.3, 0.3), (-0.3, 0.3), (1.0, 2.0)),
    ));
    v.push((
        "box-across-cyl-top",
        fin(cyl(0.0, 0.0, 1.0, 0.0, 2.0)),
        bx((-0.3, 0.3), (-2.0, 2.0), (1.0, 2.0)),
    ));
    v
}

const OPS: [&str; 3] = ["∪", "∩", "−"];

#[test]
#[ignore = "review probe"]
fn review_r1_fate_probe_rows() {
    let eps = tol().eps();
    let mut asym = Vec::new();
    for (name, x, y) in fixtures() {
        for k in 0..3 {
            let xy = summary(k, &x, &y, ['x', 'y']);
            let yx = summary(k, &y, &x, ['y', 'x']);
            for (ord, r) in [("x?y", &xy), ("y?x", &yx)] {
                match r {
                    Ok(Some((s, kind))) => println!(
                        "ROW eps={eps:e} {name} {} {ord} {kind} vol={} faces={} t3={} t3p={} subs={} cut={} lost={} covered={} unrowed={} | {:?} | {:?}",
                        OPS[k], s.volume, s.faces, s.tier3, s.tier3p, s.subs.len(), s.cut_rows.len(), s.lost.len(), s.covered, s.unrowed, s.subs, s.lost
                    ),
                    Ok(None) => println!("ROW eps={eps:e} {name} {} {ord} EMPTY", OPS[k]),
                    Err(e) => println!("ROW eps={eps:e} {name} {} {ord} ERR {e}", OPS[k]),
                }
            }
            if k < 2 {
                match (&xy, &yx) {
                    (Ok(Some((a, _))), Ok(Some((b, _)))) => {
                        let mut d = Vec::new();
                        if a.volume != b.volume { d.push("volume"); }
                        if a.faces != b.faces { d.push("faces"); }
                        if a.subs != b.subs { d.push("subs"); }
                        if a.lost != b.lost { d.push("lost"); }
                        if !d.is_empty() {
                            println!("ASYM eps={eps:e} {name} {}: {d:?}\n   xy subs {:?}\n   yx subs {:?}\n   xy lost {:?}\n   yx lost {:?}", OPS[k], a.subs, b.subs, a.lost, b.lost);
                            asym.push(format!("{name} {} {d:?}", OPS[k]));
                        }
                    }
                    (Ok(None), Ok(None)) => {}
                    _ => {
                        println!("ASYM eps={eps:e} {name} {}: build/refuse differs", OPS[k]);
                        asym.push(format!("{name} {} build", OPS[k]));
                    }
                }
            }
        }
    }
    println!("ASYMMETRIES {asym:?}");
}
