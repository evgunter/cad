//! Reviewer r2's local probes for PR 6613 (not for merge).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use crate::common;

use common::meeting::{Pose, poses};
use geom_core::{Band, Point3, Tol};
use std::collections::BTreeMap;
use topo::{
    AtRestBody, BooleanError, BooleanInputs, BooleanOp, BooleanResult, ContactRecords,
    SolidContainment, SolidKey,
};

fn t() -> Tol {
    Tol::witness()
}

/// A prism: a profile in (x, y), z range, sheared by (sx, sy) per unit z.
#[derive(Clone)]
struct Piece {
    profile: Vec<(f64, f64)>,
    z: (f64, f64),
    shear: (f64, f64),
    /// Profile in (Y, Z), extruded along X.
    perm: bool,
}

fn bx(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Piece {
    Piece {
        profile: vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)],
        z,
        shear: (0.0, 0.0),
        perm: false,
    }
}

fn build(pieces: &[Piece], pose: &Pose) -> AtRestBody<f64> {
    let mut body = topo::Body::<f64>::new();
    for p in pieces {
        let (sx, sy) = p.shear;
        common::prism_ops(
            &mut body,
            &p.profile,
            p.z,
            |x, y, z| {
                if p.perm {
                    pose.at([z, x, y])
                } else {
                    pose.at([x + sx * z, y + sy * z, z])
                }
            },
            common::FaceGeometry::Certified,
            t(),
        );
    }
    common::describe_as_intersections(&mut body, t());
    common::finished("probe", body, t())
}

struct Opd {
    name: &'static str,
    body: AtRestBody<f64>,
    records: ContactRecords,
    pieces: Vec<AtRestBody<f64>>,
}

fn opd(name: &'static str, pieces: &[Piece], pose: &Pose, record: bool) -> Opd {
    let body = build(pieces, pose);
    let records = if record && pieces.len() > 1 {
        let groups: BTreeMap<SolidKey, u32> = body.solids().map(|(s, _)| s).zip(0..).collect();
        let recorded = topo::record_at_rest(
            &body,
            &ContactRecords::default(),
            |records| {
                topo::validate_pseudomanifold(&body, records, t())
                    .err()
                    .unwrap_or_default()
            },
            &|s| groups.get(&s).copied(),
            0,
        );
        assert!(recorded.findings.is_empty(), "{name}: {:?}", recorded.findings);
        recorded.records
    } else {
        ContactRecords::default()
    };
    let singles = pieces.iter().map(|p| build(std::slice::from_ref(p), pose)).collect();
    Opd {
        name,
        body,
        records,
        pieces: singles,
    }
}

fn run(op: BooleanOp, x: &Opd, y: &Opd) -> Result<BooleanResult<f64>, BooleanError> {
    let inputs = BooleanInputs {
        records_a: x.records.carried(),
        records_b: y.records.carried(),
    };
    topo::boolean_op_with(
        op,
        &x.body,
        &y.body,
        &inputs,
        topo::SweepStrategy::Realized,
        t(),
    )
}

fn volume(b: &AtRestBody<f64>) -> f64 {
    topo::mass_properties(b, t(), topo::test_support::pinned_resolution(t()))
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum M {
    In,
    Out,
    On,
}

fn contain(b: &AtRestBody<f64>, q: Point3<f64>) -> Option<M> {
    Some(
        match topo::point_in_solid(b, q, Band::linear(t()).unwrap(), t()).ok()? {
            SolidContainment::In => M::In,
            SolidContainment::Out => M::Out,
            SolidContainment::OnBoundary => M::On,
        },
    )
}

fn of_pieces(o: &Opd, q: Point3<f64>) -> Option<M> {
    let mut all_out = true;
    for p in &o.pieces {
        match contain(p, q)? {
            M::In => return Some(M::In),
            M::On => all_out = false,
            M::Out => {}
        }
    }
    Some(if all_out { M::Out } else { M::On })
}

fn want(op: BooleanOp, a: M, b: M) -> M {
    match op {
        BooleanOp::Subtract => match (a, b) {
            (_, M::In) | (M::Out, _) => M::Out,
            (M::In, M::Out) => M::In,
            _ => M::On,
        },
        BooleanOp::Intersect => match (a, b) {
            (M::Out, _) | (_, M::Out) => M::Out,
            (M::In, M::In) => M::In,
            _ => M::On,
        },
        BooleanOp::Union => match (a, b) {
            (M::In, _) | (_, M::In) => M::In,
            (M::Out, M::Out) => M::Out,
            _ => M::On,
        },
    }
}

fn meet_volume(x: &Opd, y: &Opd) -> Option<f64> {
    let mut v = 0.0;
    for a in &x.pieces {
        for b in &y.pieces {
            match topo::boolean_op_with(
                BooleanOp::Intersect,
                a,
                b,
                &BooleanInputs::default(),
                topo::SweepStrategy::Realized,
                t(),
            ) {
                Ok(BooleanResult::Body(r)) => v += volume(&r.body),
                Ok(BooleanResult::Empty) => {}
                Err(_) => return None,
            }
        }
    }
    Some(v)
}

fn total(o: &Opd) -> f64 {
    o.pieces.iter().map(volume).sum()
}

/// Deterministic sample points in [lo, hi]^3 in the scene frame, plus a
/// layer on each given z plane.
fn samples(lo: [f64; 3], hi: [f64; 3], planes: &[f64], n: usize) -> Vec<[f64; 3]> {
    let mut s: u64 = 0x9e3779b97f4a7c15;
    let mut r = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        (s >> 11) as f64 / (1u64 << 53) as f64
    };
    let mut out = Vec::new();
    for _ in 0..n {
        out.push([0, 1, 2].map(|i| lo[i] + (hi[i] - lo[i]) * r()));
    }
    for &z in planes {
        for _ in 0..n / 2 {
            out.push([lo[0] + (hi[0] - lo[0]) * r(), lo[1] + (hi[1] - lo[1]) * r(), z]);
        }
    }
    out
}

/// Runs `x op y`, and where it builds checks it against the per-piece
/// oracle. Returns a one-line verdict and pushes violations.
fn probe(
    op: BooleanOp,
    x: &Opd,
    y: &Opd,
    pose: &Pose,
    pts: &[[f64; 3]],
    bad: &mut Vec<String>,
) -> String {
    let sign = match op {
        BooleanOp::Union => "∪",
        BooleanOp::Subtract => "−",
        BooleanOp::Intersect => "∩",
    };
    let what = format!("{} {sign} {} [{}]", x.name, y.name, pose.label);
    let r = match run(op, x, y) {
        Ok(BooleanResult::Body(r)) => r,
        Ok(BooleanResult::Empty) => {
            // Check emptiness against the oracle.
            let m = meet_volume(x, y);
            let wantv = m.map(|m| match op {
                BooleanOp::Intersect => m,
                BooleanOp::Subtract => total(x) - m,
                BooleanOp::Union => total(x) + total(y) - m,
            });
            if let Some(w) = wantv
                && w.abs() > 1e-9
            {
                bad.push(format!("{what}: EMPTY but oracle volume {w}"));
            }
            return format!("{what}: Empty (oracle {wantv:?})");
        }
        Err(e) => return format!("{what}: Err {:?}", e.kind()),
    };
    let mut notes = Vec::new();
    let g = topo::validate_geometric(&r.body, t());
    if g.is_err() {
        bad.push(format!("{what}: tier 3 {g:?}"));
        notes.push("T3-FAIL".to_string());
    }
    let p = topo::validate_pseudomanifold(&r.body, &r.contacts, t());
    if p.is_err() {
        bad.push(format!("{what}: tier 3' {p:?}"));
        notes.push("T3'-FAIL".to_string());
    }
    let v = volume(&r.body);
    let m = meet_volume(x, y);
    let wantv = m.map(|m| match op {
        BooleanOp::Intersect => m,
        BooleanOp::Subtract => total(x) - m,
        BooleanOp::Union => total(x) + total(y) - m,
    });
    if let Some(w) = wantv
        && (v - w).abs() > 1e-7
    {
        bad.push(format!("{what}: volume {v} want {w}"));
        notes.push(format!("VOL {v} != {w}"));
    }
    let mut mism = 0;
    let mut on_where_definite = 0;
    for q in pts {
        let q = pose.at(*q);
        let (Some(a), Some(b)) = (of_pieces(x, q), of_pieces(y, q)) else {
            continue;
        };
        let w = want(op, a, b);
        let Some(got) = contain(&r.body, q) else {
            continue;
        };
        if w != M::On && got != w {
            if got == M::On {
                on_where_definite += 1;
            } else {
                mism += 1;
            }
            if mism + on_where_definite <= 3 {
                bad.push(format!("{what}: at {q:?} want {w:?} got {got:?} (a {a:?} b {b:?})"));
            }
        }
    }
    if mism + on_where_definite > 0 {
        notes.push(format!("PTS mism {mism} on {on_where_definite}"));
    }
    let faces = r.body.faces().count();
    format!(
        "{what}: Body solids {} faces {faces} ff {} vol {v:.6} (oracle {wantv:?}) {}",
        r.body.solids().count(),
        r.contacts.face_face().count(),
        notes.join(" ")
    )
}

fn all_ops(x: &Opd, y: &Opd, pose: &Pose, pts: &[[f64; 3]], bad: &mut Vec<String>) {
    for op in [BooleanOp::Subtract, BooleanOp::Intersect, BooleanOp::Union] {
        eprintln!("{}", probe(op, x, y, pose, pts, bad));
        eprintln!("{}", probe(op, y, x, pose, pts, bad));
    }
}

fn the_poses() -> Vec<Pose> {
    poses()
}

fn report(bad: Vec<String>) {
    for b in &bad {
        eprintln!("VIOLATION {b}");
    }
    assert!(bad.is_empty(), "{} violations", bad.len());
}

fn bolt() -> Piece {
    bx((-0.2, 0.2), (-0.2, 0.2), (0.2, 1.2))
}
fn plate() -> Piece {
    bx((-1.0, 1.0), (-1.0, 1.0), (0.0, 0.2))
}

/// P1: the PR's own seat against blocks the suite does not use, all
/// ops both orders, every pose.
#[test]
fn p1_seat_against_new_blocks() {
    let mut bad = Vec::new();
    let pts = samples([-0.6, -0.6, -0.1], [0.6, 0.6, 0.6], &[0.2], 160);
    for pose in the_poses() {
        let seat = opd("seat", &[bolt(), plate()], &pose, true);
        let blocks: Vec<(&'static str, Piece)> = vec![
            // a hexagonal post inside the foot: six vertices on the wall
            (
                "hexpost",
                Piece {
                    profile: (0..6)
                        .map(|k| {
                            let a = std::f64::consts::PI / 3.0 * k as f64 + 0.1;
                            (0.12 * a.cos(), 0.12 * a.sin())
                        })
                        .collect(),
                    z: (0.1, 0.4),
                    shear: (0.0, 0.0),
                    perm: false,
                },
            ),
            // a slanted post through the seat
            (
                "slant",
                Piece {
                    profile: vec![(-0.1, -0.1), (0.05, -0.1), (0.05, 0.05), (-0.1, 0.05)],
                    z: (0.1, 0.4),
                    shear: (0.3, 0.15),
                    perm: false,
                },
            ),
            // a diamond post whose vertical edge crosses the foot's edge x = 0.2
            (
                "diamond",
                Piece {
                    profile: vec![(0.2, 0.0), (0.3, -0.1), (0.4, 0.0), (0.3, 0.1)],
                    z: (0.1, 0.4),
                    shear: (0.0, 0.0),
                    perm: false,
                },
            ),
            // a diamond straddling the foot edge
            (
                "diamond2",
                Piece {
                    profile: vec![(0.1, 0.0), (0.25, -0.12), (0.4, 0.0), (0.25, 0.12)],
                    z: (0.1, 0.4),
                    shear: (0.0, 0.0),
                    perm: false,
                },
            ),
            // a post whose vertices land exactly on the seat plane
            ("onplane", bx((-0.1, 0.1), (-0.1, 0.1), (0.2, 0.4))),
            ("onplane_below", bx((-0.1, 0.1), (-0.1, 0.1), (0.0, 0.2))),
            // a tall post through the bolt and plate entirely
            ("through", bx((-0.1, 0.1), (-0.1, 0.1), (-0.5, 1.5))),
            // a triangle post, a tilted face crossing
            (
                "tri",
                Piece {
                    profile: vec![(-0.15, -0.1), (0.15, -0.05), (0.0, 0.15)],
                    z: (0.05, 0.5),
                    shear: (0.0, 0.0),
                    perm: false,
                },
            ),
        ];
        for (name, piece) in blocks {
            let b = opd(name, &[piece], &pose, false);
            all_ops(&seat, &b, &pose, &pts, &mut bad);
        }
    }
    report(bad);
}

/// P2: other seat shapes: an overhanging bolt, a triangular bolt, and
/// several bolts on one plate.
#[test]
fn p2_other_seats() {
    let mut bad = Vec::new();
    let pts = samples([-1.1, -0.6, -0.1], [1.6, 0.6, 0.6], &[0.2], 200);
    for pose in the_poses() {
        let over = opd(
            "overhang",
            &[bx((0.6, 1.4), (-0.2, 0.2), (0.2, 1.2)), plate()],
            &pose,
            true,
        );
        let tri = opd(
            "tribolt",
            &[
                Piece {
                    profile: vec![(-0.3, -0.2), (0.3, -0.2), (0.0, 0.3)],
                    z: (0.2, 1.0),
                    shear: (0.0, 0.0),
                    perm: false,
                },
                plate(),
            ],
            &pose,
            true,
        );
        let two = opd(
            "twobolts",
            &[
                bx((-0.6, -0.2), (-0.2, 0.2), (0.2, 1.0)),
                bx((0.2, 0.6), (-0.2, 0.2), (0.2, 1.0)),
                plate(),
            ],
            &pose,
            true,
        );
        // blocks
        let p_in = opd("p_in", &[bx((0.7, 0.9), (-0.1, 0.1), (0.1, 0.4))], &pose, false);
        let p_edge = opd(
            "p_edge",
            &[bx((0.9, 1.1), (-0.1, 0.1), (0.1, 0.4))],
            &pose,
            false,
        );
        let p_tri = opd(
            "p_tri",
            &[bx((-0.05, 0.05), (-0.1, 0.1), (0.1, 0.4))],
            &pose,
            false,
        );
        let p_two = opd(
            "p_two",
            &[bx((-0.5, 0.5), (-0.1, 0.1), (0.1, 0.4))],
            &pose,
            false,
        );
        let p_two_in = opd(
            "p_two_in",
            &[bx((-0.5, -0.3), (-0.1, 0.1), (0.1, 0.4))],
            &pose,
            false,
        );
        all_ops(&over, &p_in, &pose, &pts, &mut bad);
        all_ops(&over, &p_edge, &pose, &pts, &mut bad);
        all_ops(&tri, &p_tri, &pose, &pts, &mut bad);
        all_ops(&two, &p_two, &pose, &pts, &mut bad);
        all_ops(&two, &p_two_in, &pose, &pts, &mut bad);
    }
    report(bad);
}

/// P3: both operands carry seat records.
#[test]
fn p3_both_operands_carry_records() {
    let mut bad = Vec::new();
    let pts = samples([-1.1, -1.1, -0.1], [1.1, 1.1, 0.7], &[0.2, 0.25], 200);
    for pose in the_poses() {
        let a = opd("seatA", &[bolt(), plate()], &pose, true);
        // B: a small bolt on a thinner plate, its seat at z = 0.3, its bolt
        // inside A's bolt foot, its plate crossing A's bolt.
        let b = opd(
            "seatB",
            &[
                bx((-0.1, 0.1), (-0.1, 0.1), (0.05, 0.3)),
                bx((-0.3, 0.3), (-0.3, 0.3), (0.3, 0.4)),
            ],
            &pose,
            true,
        );
        all_ops(&a, &b, &pose, &pts, &mut bad);
        // B2: B's pieces stacked so B's seat lies inside A's plate
        let b2 = opd(
            "seatB2",
            &[
                bx((-0.1, 0.1), (-0.1, 0.1), (0.1, 0.5)),
                bx((-0.15, 0.15), (-0.15, 0.15), (-0.2, 0.1)),
            ],
            &pose,
            true,
        );
        all_ops(&a, &b2, &pose, &pts, &mut bad);
        // C: each operand's vertex pierces the other's wall.
        let c = opd(
            "seatC",
            &[
                bx((0.1, 0.3), (0.1, 0.3), (0.1, 0.25)),
                bx((0.1, 0.3), (0.1, 0.3), (0.25, 0.6)),
            ],
            &pose,
            true,
        );
        all_ops(&a, &c, &pose, &pts, &mut bad);
    }
    report(bad);
}

/// P4: touching-only and crossing ridges: an apex edge lying in the seat
/// from above (inside the bolt, the PR's ridge) and from below (inside the
/// plate), and a ridge whose apex crosses the seat.
#[test]
fn p4_ridges() {
    let mut bad = Vec::new();
    let pts = samples([-0.6, -0.6, -0.1], [0.6, 0.6, 0.6], &[0.2], 120);
    let ridge = |tri: Vec<(f64, f64)>| Piece {
        profile: tri,
        z: (-0.1, 0.1),
        shear: (0.0, 0.0),
        perm: true,
    };
    for pose in the_poses().into_iter().take(3) {
        let seat = opd("seat", &[bolt(), plate()], &pose, true);
        let up = opd("ridge_up", &[ridge(vec![(0.1, 0.35), (-0.1, 0.35), (0.0, 0.2)])], &pose, false);
        let down = opd("ridge_down", &[ridge(vec![(-0.1, 0.05), (0.1, 0.05), (0.0, 0.2)])], &pose, false);
        let cross = opd("ridge_cross", &[ridge(vec![(0.1, 0.35), (-0.1, 0.35), (0.0, 0.15)])], &pose, false);
        all_ops(&seat, &up, &pose, &pts, &mut bad);
        all_ops(&seat, &down, &pose, &pts, &mut bad);
        all_ops(&seat, &cross, &pose, &pts, &mut bad);
    }
    report(bad);
}

/// P0: the PR's own blocks, every op both orders, with the record, at
/// rest: for comparing head with main.
#[test]
fn p0_pr_scenes() {
    let mut bad = Vec::new();
    let pts = samples([-0.6, -0.6, -0.1], [0.6, 0.6, 0.6], &[0.2], 60);
    let pose = Pose::rest();
    let seat = opd("seat", &[bolt(), plate()], &pose, true);
    let bare = opd("bare", &[bolt(), plate()], &pose, false);
    let blocks = [
        ("post", bx((-0.1, 0.1), (-0.1, 0.1), (0.1, 0.4))),
        ("pin", bx((-0.1, 0.1), (-0.1, 0.1), (-0.5, 0.5))),
        ("bead", bx((0.15, 0.35), (-0.1, 0.1), (0.1, 0.4))),
        ("side", bx((0.15, 0.35), (-0.1, 0.1), (0.6, 0.8))),
        ("ledge", bx((0.2, 0.5), (-0.5, 0.5), (0.1, 0.4))),
        ("slab", bx((-0.1, 2.0), (-2.0, 2.0), (0.1, 0.4))),
        ("cap", bx((-0.5, 0.5), (-0.5, 0.5), (0.1, 0.4))),
        ("over", bx((0.1, 0.5), (0.1, 0.5), (0.1, 0.4))),
        ("corner", bx((0.2, 0.5), (0.2, 0.5), (0.1, 0.4))),
    ];
    for (name, piece) in blocks {
        let b = opd(name, &[piece], &pose, false);
        all_ops(&seat, &b, &pose, &pts, &mut bad);
        all_ops(&bare, &b, &pose, &pts, &mut bad);
    }
    report(bad);
}

fn opd_body(name: &'static str, body: AtRestBody<f64>) -> Opd {
    Opd {
        name,
        pieces: vec![body.clone()],
        body,
        records: ContactRecords::default(),
    }
}

fn sub_body(x: &AtRestBody<f64>, y: &AtRestBody<f64>) -> AtRestBody<f64> {
    match topo::boolean_op_with(
        BooleanOp::Subtract,
        x,
        y,
        &BooleanInputs::default(),
        topo::SweepStrategy::Realized,
        t(),
    ) {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("sub: {:?}", other.map(|_| ())),
    }
}

/// P5: a collar bridging the bolt and the plate around the bolt's foot,
/// leaving the inner seat a closed crack.
#[test]
fn p5_collar() {
    let mut bad = Vec::new();
    let pts = samples([-0.6, -0.6, -0.1], [0.6, 0.6, 0.6], &[0.2], 120);
    for pose in the_poses().into_iter().take(3) {
        let seat = opd("seat", &[bolt(), plate()], &pose, true);
        let outer = build(&[bx((-0.4, 0.4), (-0.4, 0.4), (0.1, 0.4))], &pose);
        let hole = build(&[bx((-0.15, 0.15), (-0.15, 0.15), (0.0, 0.5))], &pose);
        let collar = opd_body("collar", sub_body(&outer, &hole));
        all_ops(&seat, &collar, &pose, &pts, &mut bad);
    }
    report(bad);
}

/// P6: the two new refusals' recourses, followed.
#[test]
fn p6_recourses() {
    let pose = Pose::rest();
    let bolt_b = build(&[bolt()], &pose);
    let plate_b = build(&[plate()], &pose);
    let none = BooleanInputs::default();
    let go = |op, x: &AtRestBody<f64>, y: &AtRestBody<f64>| {
        topo::boolean_op_with(op, x, y, &none, topo::SweepStrategy::Realized, t())
    };
    for (name, b) in [
        ("post", bx((-0.1, 0.1), (-0.1, 0.1), (0.1, 0.4))),
        ("pin", bx((-0.1, 0.1), (-0.1, 0.1), (-0.5, 0.5))),
        ("bead", bx((0.15, 0.35), (-0.1, 0.1), (0.1, 0.4))),
        ("cap", bx((-0.5, 0.5), (-0.5, 0.5), (0.1, 0.4))),
    ] {
        let x = build(&[b], &pose);
        // ToolSeatCrossed: cut by each piece of the tool in turn.
        let step = go(BooleanOp::Subtract, &x, &bolt_b);
        let out = match &step {
            Ok(BooleanResult::Body(r)) => match go(BooleanOp::Subtract, &r.body, &plate_b) {
                Ok(BooleanResult::Body(r2)) => format!("Body vol {:.6}", volume(&r2.body)),
                Ok(BooleanResult::Empty) => "Empty".to_string(),
                Err(e) => format!("Err {:?}", e.kind()),
            },
            Ok(BooleanResult::Empty) => "first step Empty".to_string(),
            Err(e) => format!("first step Err {:?}", e.kind()),
        };
        eprintln!("RECOURSE ({name} − bolt) − plate: {out}");
        let step = go(BooleanOp::Subtract, &x, &plate_b);
        let out = match &step {
            Ok(BooleanResult::Body(r)) => match go(BooleanOp::Subtract, &r.body, &bolt_b) {
                Ok(BooleanResult::Body(r2)) => format!("Body vol {:.6}", volume(&r2.body)),
                Ok(BooleanResult::Empty) => "Empty".to_string(),
                Err(e) => format!("Err {:?}", e.kind()),
            },
            Ok(BooleanResult::Empty) => "first step Empty".to_string(),
            Err(e) => format!("first step Err {:?}", e.kind()),
        };
        eprintln!("RECOURSE ({name} − plate) − bolt: {out}");
        // TouchesItself: unite the resting pieces first.
        let fused = go(BooleanOp::Union, &bolt_b, &plate_b);
        let out = match &fused {
            Ok(BooleanResult::Body(r)) => match go(BooleanOp::Union, &r.body, &x) {
                Ok(BooleanResult::Body(r2)) => format!(
                    "Body solids {} vol {:.6}",
                    r2.body.solids().count(),
                    volume(&r2.body)
                ),
                Ok(BooleanResult::Empty) => "Empty".to_string(),
                Err(e) => format!("Err {:?}", e.kind()),
            },
            other => format!("first step {:?}", other.as_ref().map(|_| ()).map_err(|e| e.kind())),
        };
        eprintln!("RECOURSE (bolt ∪ plate) ∪ {name}: {out}");
    }
}

/// P7: two stacked seats (bolt on plate on base), a pin through all
/// three: each pin edge has a segment whose both ends are wall vertices.
#[test]
fn p7_stacked_seats() {
    let mut bad = Vec::new();
    let pts = samples([-0.6, -0.6, -0.4], [0.6, 0.6, 0.6], &[0.2, 0.0], 160);
    for pose in the_poses() {
        let stack = opd(
            "stack",
            &[bolt(), plate(), bx((-1.0, 1.0), (-1.0, 1.0), (-0.3, 0.0))],
            &pose,
            true,
        );
        let pin = opd("pin3", &[bx((-0.1, 0.1), (-0.1, 0.1), (-0.2, 0.5))], &pose, false);
        let thru = opd("thru3", &[bx((-0.1, 0.1), (-0.1, 0.1), (-0.5, 1.5))], &pose, false);
        all_ops(&stack, &pin, &pose, &pts, &mut bad);
        all_ops(&stack, &thru, &pose, &pts, &mut bad);
    }
    report(bad);
}
