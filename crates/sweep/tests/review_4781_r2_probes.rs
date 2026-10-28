//! **Review rows, R2 of PR 4781: same-side tangent touches against an
//! independent oracle.** Each pose is two analytic operands (upright
//! discs and boxes, optionally scaled and turned rigidly), and every op
//! (both operand orders) is judged three ways that share nothing with
//! the Boolean: volume against the closed form, solid count, and
//! `point_in_solid` at steady sample points against the analytic
//! membership. Tier 3 and 3′ on the result's own records are reported
//! beside them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::approx::band;
use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop};
use sweep::test_support::{finished, sketch_at};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, Body, BooleanResult, SolidContainment, mass_properties};

fn tol() -> Tol {
    Tol::witness()
}

#[derive(Clone, Copy, Debug)]
enum Shape {
    /// Upright disc: centre, radius, z range.
    Cyl(f64, f64, f64, f64, f64),
    /// Box: x, y, z ranges.
    Box((f64, f64), (f64, f64), (f64, f64)),
    /// A nonconvex hook over z ∈ [0, 1]: the unit circle's arc from
    /// (0, −1) through (1, 0) to (½, √3/2), then the band
    /// y ∈ [√3/2, 3/2] out to x = 7/2 (over the disc about (2, 0)), back
    /// along x = −1.
    Hook,
}

impl Shape {
    fn scaled(self, s: f64) -> Self {
        match self {
            Self::Cyl(cx, cy, r, z0, z1) => Self::Cyl(cx * s, cy * s, r * s, z0 * s, z1 * s),
            Self::Box(x, y, z) => {
                Self::Box((x.0 * s, x.1 * s), (y.0 * s, y.1 * s), (z.0 * s, z.1 * s))
            }
            Self::Hook => Self::Hook,
        }
    }
    fn volume(self) -> f64 {
        match self {
            Self::Cyl(_, _, r, z0, z1) => PI * r * r * (z1 - z0),
            Self::Box(x, y, z) => (x.1 - x.0) * (y.1 - y.0) * (z.1 - z.0),
            // 1 (x ≤ 0, y ∈ [−1, √3/2]) · (1 + √3/2) + quarter disc π/4
            // + ∫₀^{√3/2} √(1 − y²) dy (π/6 + √3/8) + the band 4.5 · (1.5 − √3/2).
            Self::Hook => {
                let h = 3f64.sqrt() / 2.0;
                (1.0 + h) + PI / 4.0 + PI / 6.0 + h / 4.0 + 4.5 * (1.5 - h)
            }
        }
    }
    fn has(self, p: Point3<f64>) -> bool {
        match self {
            Self::Cyl(cx, cy, r, z0, z1) => {
                (p.x - cx).powi(2) + (p.y - cy).powi(2) <= r * r && (z0..=z1).contains(&p.z)
            }
            Self::Box(x, y, z) => {
                (x.0..=x.1).contains(&p.x)
                    && (y.0..=y.1).contains(&p.y)
                    && (z.0..=z.1).contains(&p.z)
            }
            Self::Hook => {
                let h = 3f64.sqrt() / 2.0;
                (0.0..=1.0).contains(&p.z)
                    && p.x >= -1.0
                    && (((-1.0..=h).contains(&p.y) && (p.x <= 0.0 || p.x * p.x + p.y * p.y <= 1.0))
                        || ((h..=1.5).contains(&p.y) && p.x <= 3.5))
            }
        }
    }
    fn bbox(self) -> (Point3<f64>, Point3<f64>) {
        match self {
            Self::Cyl(cx, cy, r, z0, z1) => (
                Point3::new(cx - r, cy - r, z0),
                Point3::new(cx + r, cy + r, z1),
            ),
            Self::Box(x, y, z) => (Point3::new(x.0, y.0, z.0), Point3::new(x.1, y.1, z.1)),
            Self::Hook => (Point3::new(-1.0, -1.0, 0.0), Point3::new(3.5, 1.5, 1.0)),
        }
    }
    /// The bounding box's centre, which lies inside a disc or a box.
    fn centre(self) -> Point3<f64> {
        let (lo, hi) = self.bbox();
        let c = lo + (hi - lo) * 0.5;
        assert!(self.has(c), "{self:?}: the centre lies inside");
        c
    }
    fn body(self) -> Body<f64> {
        let (lp, z0, h): (ProfileLoop<f64>, f64, f64) = match self {
            Self::Cyl(cx, cy, r, z0, z1) => (
                bulge_loop(vec![
                    (Point2::new(cx + r, cy), 1.0),
                    (Point2::new(cx - r, cy), 1.0),
                ]),
                z0,
                z1 - z0,
            ),
            Self::Box(x, y, z) => (
                bulge_loop(vec![
                    (Point2::new(x.0, y.0), 0.0),
                    (Point2::new(x.1, y.0), 0.0),
                    (Point2::new(x.1, y.1), 0.0),
                    (Point2::new(x.0, y.1), 0.0),
                ]),
                z.0,
                z.1 - z.0,
            ),
            Self::Hook => {
                let h = 3f64.sqrt() / 2.0;
                (
                    bulge_loop(vec![
                        (Point2::new(0.0, -1.0), (PI / 8.0).tan()),
                        (Point2::new(1.0, 0.0), (PI / 12.0).tan()),
                        (Point2::new(0.5, h), 0.0),
                        (Point2::new(3.5, h), 0.0),
                        (Point2::new(3.5, 1.5), 0.0),
                        (Point2::new(-1.0, 1.5), 0.0),
                        (Point2::new(-1.0, -1.0), 0.0),
                    ]),
                    0.0,
                    1.0,
                )
            }
        };
        let profile = Profile::new(sketch_at(z0), vec![lp])
            .validate(tol())
            .unwrap();
        extrude(
            &profile,
            Extrusion::Distance {
                depth: h,
                side: ExtrudeSide::Along,
            },
            tol(),
        )
        .unwrap()
        .body
    }
}

/// A pose: two shapes, a rigid map applied to both, the closed-form
/// meet volume, and the union's expected solid count.
struct Pose {
    name: String,
    a: Shape,
    b: Shape,
    map: Option<Affine3<f64>>,
    meet: f64,
    union_solids: usize,
}

fn body_of(s: Shape, map: Option<Affine3<f64>>) -> AtRestBody<f64> {
    let b = s.body();
    let b = match map {
        None => b,
        Some(map) => {
            topo::transform_rigid(&b, &geom_core::test_support::rigid(&map, tol()), tol()).unwrap()
        }
    };
    finished("r2-4781 operand", b, tol())
}

#[derive(Clone, Copy)]
enum Op {
    Union,
    AMinusB,
    BMinusA,
    Meet,
}

fn samples(p: &Pose) -> Vec<(Point3<f64>, bool, bool)> {
    let (la, ha) = p.a.bbox();
    let (lb, hb) = p.b.bbox();
    let lo = Point3::new(la.x.min(lb.x), la.y.min(lb.y), la.z.min(lb.z));
    let hi = Point3::new(ha.x.max(hb.x), ha.y.max(hb.y), ha.z.max(hb.z));
    let span = (hi - lo).norm();
    // Steady past the band's escalation width, at any ε.
    let d = (1e-5 * span).max(band().escalate());
    let mut s: u64 = 0x9e37_79b9_7f4a_7c15;
    let mut next = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        (s >> 11) as f64 / (1u64 << 53) as f64
    };
    let mut out = Vec::new();
    let mut tries = 0;
    while out.len() < 160 && tries < 100_000 {
        tries += 1;
        let q = Point3::new(
            lo.x - 0.1 * span + (hi.x - lo.x + 0.2 * span) * next(),
            lo.y - 0.1 * span + (hi.y - lo.y + 0.2 * span) * next(),
            lo.z - 0.1 * span + (hi.z - lo.z + 0.2 * span) * next(),
        );
        let (a, b) = (p.a.has(q), p.b.has(q));
        let steady = [
            Vec3::new(d, 0.0, 0.0),
            Vec3::new(-d, 0.0, 0.0),
            Vec3::new(0.0, d, 0.0),
            Vec3::new(0.0, -d, 0.0),
            Vec3::new(0.0, 0.0, d),
            Vec3::new(0.0, 0.0, -d),
        ]
        .iter()
        .all(|&o| p.a.has(q + o) == a && p.b.has(q + o) == b);
        if steady {
            out.push((p.map.map_or(q, |m| m.transform_point(q)), a, b));
        }
    }
    out
}

fn judge(
    op: Op,
    r: &Result<BooleanResult<f64>, topo::BooleanError>,
    want_vol: f64,
    want_solids: Option<usize>,
    pts: &[(Point3<f64>, bool, bool)],
) -> String {
    let want_in = |a: bool, b: bool| match op {
        Op::Union => a || b,
        Op::AMinusB => a && !b,
        Op::BMinusA => b && !a,
        Op::Meet => a && b,
    };
    match r {
        Err(e) => format!("REFUSED {:?} ({e})", e.kind()),
        Ok(BooleanResult::Empty) => {
            if want_vol > 1e-9 {
                format!("WRONG Empty, want volume {want_vol}")
            } else {
                "OK Empty".into()
            }
        }
        Ok(BooleanResult::Body(bb)) => {
            let mut bad = Vec::new();
            let v = mass_properties(
                &bb.body,
                tol(),
                topo::test_support::pinned_resolution(tol()),
            )
            .unwrap()
            .met()
            .expect("the pinned resolution is met")
            .volume;
            if (v - want_vol).abs() > 1e-9_f64.max(1e-7 * want_vol) {
                bad.push(format!("volume {v} vs {want_vol}"));
            }
            let solids = bb.body.solids().count();
            if want_solids.is_some_and(|w| w != solids) {
                bad.push(format!("solids {solids} vs {want_solids:?}"));
            }
            if let Err(e) = topo::validate_geometric(&bb.body, tol()) {
                bad.push(format!("tier3 {e:?}"));
            }
            let mut miss = 0;
            for &(p, a, b) in pts {
                let got = topo::point_in_solid(&bb.body, p, band(), tol());
                let ok = matches!(
                    (&got, want_in(a, b)),
                    (Ok(SolidContainment::In), true) | (Ok(SolidContainment::Out), false)
                );
                if !ok {
                    miss += 1;
                    if miss <= 2 {
                        bad.push(format!("pis {p:?}: {got:?} want in={}", want_in(a, b)));
                    }
                }
            }
            if miss > 0 {
                bad.push(format!("{miss} of {} samples disagree", pts.len()));
            }
            let ee = bb.contacts.edge_edge().count();
            let v3 = match topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()) {
                Ok(()) => "3'=Ok".to_string(),
                Err(errs) => {
                    let undec = errs
                        .iter()
                        .filter(|e| matches!(e, topo::ValidationError::CensusUndecidable { .. }))
                        .count();
                    if undec == errs.len() {
                        let mut whats: Vec<&str> = errs
                            .iter()
                            .filter_map(|e| match e {
                                topo::ValidationError::CensusUndecidable { what, .. } => {
                                    Some(*what)
                                }
                                _ => None,
                            })
                            .collect();
                        whats.sort_unstable();
                        whats.dedup();
                        format!("3'=Undecidable×{undec} {whats:?}")
                    } else {
                        format!("3'=FINDINGS {errs:?}")
                    }
                }
            };
            if bad.is_empty() {
                format!(
                    "OK v={v:.9} solids={solids} kind={:?} ee={ee} {v3}",
                    bb.kind
                )
            } else {
                format!(
                    "WRONG solids={solids} kind={:?} ee={ee} {v3}: {}",
                    bb.kind,
                    bad.join("; ")
                )
            }
        }
    }
}

fn run(p: &Pose) -> Vec<String> {
    let a = body_of(p.a, p.map);
    let b = body_of(p.b, p.map);
    let (va, vb) = (p.a.volume(), p.b.volume());
    let pts = samples(p);
    let us = Some(p.union_solids);
    [
        (
            "a∪b",
            Op::Union,
            topo::union(&a, &b, tol()),
            va + vb - p.meet,
            us,
        ),
        (
            "b∪a",
            Op::Union,
            topo::union(&b, &a, tol()),
            va + vb - p.meet,
            us,
        ),
        (
            "a∖b",
            Op::AMinusB,
            topo::subtract(&a, &b, tol()),
            va - p.meet,
            None,
        ),
        (
            "b∖a",
            Op::BMinusA,
            topo::subtract(&b, &a, tol()),
            vb - p.meet,
            None,
        ),
        (
            "a∩b",
            Op::Meet,
            topo::intersect(&a, &b, tol()),
            p.meet,
            None,
        ),
        (
            "b∩a",
            Op::Meet,
            topo::intersect(&b, &a, tol()),
            p.meet,
            None,
        ),
    ]
    .into_iter()
    .map(|(n, op, got, want, ws)| format!("{n}: {}", judge(op, &got, want, ws, &pts)))
    .collect()
}

fn id() -> Option<Affine3<f64>> {
    None
}

/// Two-disc lens area for radii r1, r2 at centre distance d.
fn lens(r1: f64, r2: f64, d: f64) -> f64 {
    if d >= r1 + r2 {
        return 0.0;
    }
    let a1 = ((d * d + r1 * r1 - r2 * r2) / (2.0 * d * r1)).acos();
    let a2 = ((d * d + r2 * r2 - r1 * r1) / (2.0 * d * r2)).acos();
    r1 * r1 * (a1 - a1.sin() * a1.cos()) + r2 * r2 * (a2 - a2.sin() * a2.cos())
}

fn poses() -> Vec<Pose> {
    let mut v = Vec::new();
    let pose = |name: &str, a: Shape, b: Shape, map, meet, union_solids| Pose {
        name: name.to_string(),
        a,
        b,
        map,
        meet,
        union_solids,
    };
    for (sname, s) in [("", 1.0), (" ×1e-3", 1e-3), (" ×1e3", 1e3)] {
        for mname in ["", " turned"] {
            let m = |name: &str| format!("{name}{sname}{mname}");
            let scale = |sh: Shape| sh.scaled(s);
            let mp = if mname.is_empty() {
                id()
            } else {
                Some(Affine3::rotation_about_axis(
                    Point3::new(0.3 * s, -0.2 * s, 0.1 * s),
                    Vec3::new(1.0, 2.0, 3.0).normalize(),
                    0.7,
                ))
            };
            // cyl × cyl, equal radii (the PR's discs).
            v.push(pose(
                &m("cc equal"),
                scale(Shape::Cyl(0.0, 0.0, 1.0, 0.0, 1.0)),
                scale(Shape::Cyl(2.0, 0.0, 1.0, 0.0, 1.0)),
                mp,
                0.0,
                2,
            ));
            // cyl × cyl, unequal radii, touching at (1, 0).
            v.push(pose(
                &m("cc unequal"),
                scale(Shape::Cyl(0.0, 0.0, 1.0, 0.0, 1.0)),
                scale(Shape::Cyl(3.0, 0.0, 2.0, 0.0, 1.0)),
                mp,
                0.0,
                2,
            ));
            // cyl × cyl, staggered heights: the line touch is z ∈ [1, 2].
            v.push(pose(
                &m("cc staggered"),
                scale(Shape::Cyl(0.0, 0.0, 1.0, 0.0, 2.0)),
                scale(Shape::Cyl(2.0, 0.0, 1.0, 1.0, 3.0)),
                mp,
                0.0,
                2,
            ));
            // plane × cyl, the box's edge on the disc's vertex ruling,
            // the box's face x = 1 tangent to the wall, box going +y.
            v.push(pose(
                &m("pc corner +y"),
                scale(Shape::Cyl(0.0, 0.0, 1.0, 0.0, 1.0)),
                scale(Shape::Box((1.0, 3.0), (0.0, 2.0), (0.0, 1.0))),
                mp,
                0.0,
                2,
            ));
            // and going -y.
            v.push(pose(
                &m("pc corner -y"),
                scale(Shape::Cyl(0.0, 0.0, 1.0, 0.0, 1.0)),
                scale(Shape::Box((1.0, 3.0), (-2.0, 0.0), (0.0, 1.0))),
                mp,
                0.0,
                2,
            ));
            // plane × cyl, box face x = 1 tangent through the face's
            // interior (the slit refusal's row for ∪).
            v.push(pose(
                &m("pc face"),
                scale(Shape::Cyl(0.0, 0.0, 1.0, 0.0, 1.0)),
                scale(Shape::Box((1.0, 3.0), (-1.0, 1.0), (0.0, 1.0))),
                mp,
                0.0,
                2,
            ));
        }
    }
    // Near-tangent discs: centre distance 2 + δ, within and outside the
    // band, both signs.
    let eps = tol().eps();
    for k in [0.01, 0.5, 2.0, 100.0, 1e4] {
        for sign in [1.0, -1.0] {
            let d = 2.0 + sign * k * eps;
            v.push(pose(
                &format!("cc near δ={:+e}", sign * k * eps),
                Shape::Cyl(0.0, 0.0, 1.0, 0.0, 1.0),
                Shape::Cyl(d, 0.0, 1.0, 0.0, 1.0),
                id(),
                lens(1.0, 1.0, d),
                if sign > 0.0 { 2 } else { 1 },
            ));
        }
    }
    // Opposite-side controls: one disc inside a bigger one, touching
    // internally (the crescent), and a box in a disc's hole-free wall.
    v.push(pose(
        "cc internal",
        Shape::Cyl(0.0, 0.0, 2.0, 0.0, 1.0),
        Shape::Cyl(1.0, 0.0, 1.0, 0.0, 1.0),
        id(),
        PI,
        1,
    ));
    v.push(pose(
        "cc internal tall",
        Shape::Cyl(0.0, 0.0, 2.0, 0.0, 1.0),
        Shape::Cyl(1.0, 0.0, 1.0, -1.0, 2.0),
        id(),
        PI,
        1,
    ));
    // A kiss along x = 1, y = 0 that also overlaps elsewhere: the hook's
    // band crosses the disc about (2, 0) above y = √3/2.
    v.push(pose(
        "hook kiss+overlap",
        Shape::Hook,
        Shape::Cyl(2.0, 0.0, 1.0, 0.0, 1.0),
        id(),
        PI / 6.0 - 3f64.sqrt() / 4.0,
        1,
    ));
    // Crossing controls: overlapping parallel discs.
    v.push(pose(
        "cc overlap",
        Shape::Cyl(0.0, 0.0, 1.0, 0.0, 1.0),
        Shape::Cyl(1.5, 0.0, 1.0, 0.0, 1.0),
        id(),
        lens(1.0, 1.0, 1.5),
        1,
    ));
    v
}

/// Every pose, every op, printed against the oracle. Run:
/// `cargo test -p sweep --test all review_4781_r2_every_pose -- --ignored --nocapture`
#[test]
#[ignore = "measurement: prints each pose's outcome"]
fn review_4781_r2_every_pose() {
    let only = std::env::var("R2_POSE").ok();
    for p in poses() {
        if only.as_deref().is_some_and(|o| !p.name.contains(o)) {
            continue;
        }
        let lines = std::panic::catch_unwind(|| run(&p))
            .unwrap_or_else(|e| vec![format!("PANIC {:?}", e.downcast_ref::<String>())]);
        for line in lines {
            eprintln!("R2-4781 eps={:e} {} | {line}", tol().eps(), p.name);
        }
    }
}

fn ee_lines(r: &topo::BooleanBody<f64>) -> Vec<String> {
    r.contacts
        .edge_edge()
        .map(|(e0, e1)| {
            let ends = |e| {
                let edge = r.body.get_edge(e).expect("a recorded edge is live");
                [edge.he_plus, edge.he_minus]
                    .map(|he| r.body.half_edge_start_point(he).expect("an end"))
            };
            let [p, q] = ends(e0);
            let [s, t] = ends(e1);
            format!(
                "[({:.3},{:.3},{:.3})-({:.3},{:.3},{:.3}) | ({:.3},{:.3},{:.3})-({:.3},{:.3},{:.3})]",
                p.x, p.y, p.z, q.x, q.y, q.z, s.x, s.y, s.z, t.x, t.y, t.z
            )
        })
        .collect()
}

/// **Does the touch's edge–edge record survive a later op?** The discs'
/// union, carried (`ContactRecords::carried`) into a second op. Run:
/// `cargo test -p sweep --test all review_4781_r2_record_survives -- --ignored --nocapture`
#[test]
#[ignore = "measurement: prints the carried records"]
fn review_4781_r2_record_survives() {
    let a = body_of(Shape::Cyl(0.0, 0.0, 1.0, 0.0, 1.0), None);
    let b = body_of(Shape::Cyl(2.0, 0.0, 1.0, 0.0, 1.0), None);
    let u = match topo::union(&a, &b, tol()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("{other:?}"),
    };
    eprintln!("R2-4781 carry | first: ee {:?}", ee_lines(&u));
    let kinds: Vec<String> = u
        .contacts
        .rows()
        .iter()
        .map(|c| format!("{:?}", c.record.kind()).chars().take(40).collect())
        .collect();
    eprintln!("R2-4781 carry-rows | first: {kinds:?}");
    let steps: Vec<(&str, topo::BooleanOp, Shape)> = vec![
        (
            "∖ far box",
            topo::BooleanOp::Subtract,
            Shape::Box((-2.0, -0.5), (-2.0, 2.0), (-1.0, 2.0)),
        ),
        (
            "∪ far box",
            topo::BooleanOp::Union,
            Shape::Box((5.0, 6.0), (5.0, 6.0), (0.0, 1.0)),
        ),
        (
            "∪ third disc kissing at x = 3",
            topo::BooleanOp::Union,
            Shape::Cyl(4.0, 0.0, 1.0, 0.0, 1.0),
        ),
        (
            "∖ slab across the line",
            topo::BooleanOp::Subtract,
            Shape::Box((-2.0, 4.0), (-2.0, 2.0), (0.4, 0.6)),
        ),
        (
            "∩ big box",
            topo::BooleanOp::Intersect,
            Shape::Box((-2.0, 4.0), (-2.0, 2.0), (-1.0, 0.7)),
        ),
    ];
    for (name, op, c) in steps {
        let inputs = topo::BooleanInputs {
            records_a: u.contacts.carried(),
            ..topo::BooleanInputs::default()
        };
        let got = topo::boolean_op_with(
            op,
            &u.body,
            &body_of(c, None),
            &inputs,
            topo::SweepStrategy::Realized,
            tol(),
        );
        match got {
            Ok(BooleanResult::Body(r)) => {
                let v3 = topo::validate_pseudomanifold(&r.body, &r.contacts, tol());
                let kinds: Vec<String> = r
                    .contacts
                    .rows()
                    .iter()
                    .map(|c| format!("{:?}", c.record.kind()).chars().take(40).collect())
                    .collect();
                eprintln!("R2-4781 carry-rows | {name}: {kinds:?}");
                if let Err(errs) = &v3 {
                    for e in errs
                        .iter()
                        .filter(|e| !matches!(e, topo::ValidationError::CensusUndecidable { .. }))
                    {
                        eprintln!("R2-4781 carry-3' | {name}: {e:?}");
                    }
                }
                eprintln!(
                    "R2-4781 carry | {name}: solids={} kind={:?} ee={:?} 3'={:?}",
                    r.body.solids().count(),
                    r.kind,
                    ee_lines(&r),
                    v3.err().map(|e| e
                        .iter()
                        .map(|x| format!("{x:?}").chars().take(90).collect::<String>())
                        .collect::<Vec<_>>())
                );
            }
            other => eprintln!("R2-4781 carry | {name}: {other:?}"),
        }
    }
}

/// **Every same-side touch pose this reviewer authored answers as a
/// touch** through the PR's own check: other carrier pairs (cylinders
/// of unequal radii, a box corner on a cylinder's ruling going either
/// way), both operand orders, turned rigidly, at three scales. Red if
/// any of them refuses, mis-counts, or writes no edge–edge row.
#[test]
fn r2_every_same_side_touch_pose_answers_as_a_touch() {
    // Pinned since PR 5728 (limb 2 for every carrier): at ε = 1e-12 a
    // cylinder fixture (extent 2236) refuses as limb 2's kernel limit,
    // bound 3.67e-12
    // (`work/offset/main-red-two-review-4781-rows-refuse-at-limb-2s-kernel-limit-since-5728`).
    test_utils::pinned_red::row(
        geom_core::Tol::witness().eps(),
        &[1e-12],
        "the review 4781 touch rows",
        &[
            "Op { source: Certification { error: AnalyticRung3(KernelLimit { operand: Cylinder, bound: 3.667082637548463e-12",
            "extent: 2236.067977499794",
        ],
        r2_every_same_side_touch_pose_answers_as_a_touch_row,
    );
}

/// The row [`r2_every_same_side_touch_pose_answers_as_a_touch`] runs, under its pin.
fn r2_every_same_side_touch_pose_answers_as_a_touch_row() {
    for s in [1.0, 1e-3, 1e3] {
        for turned in [false, true] {
            let map = turned.then(|| {
                Affine3::rotation_about_axis(
                    Point3::new(0.3 * s, -0.2 * s, 0.1 * s),
                    Vec3::new(1.0, 2.0, 3.0).normalize(),
                    0.7,
                )
            });
            for (name, a, b) in [
                (
                    "cc unequal",
                    Shape::Cyl(0.0, 0.0, 1.0, 0.0, 1.0),
                    Shape::Cyl(3.0, 0.0, 2.0, 0.0, 1.0),
                ),
                (
                    "pc corner +y",
                    Shape::Cyl(0.0, 0.0, 1.0, 0.0, 1.0),
                    Shape::Box((1.0, 3.0), (0.0, 2.0), (0.0, 1.0)),
                ),
                (
                    "pc corner -y",
                    Shape::Cyl(0.0, 0.0, 1.0, 0.0, 1.0),
                    Shape::Box((1.0, 3.0), (-2.0, 0.0), (0.0, 1.0)),
                ),
            ] {
                // Fixture limits, not the Boolean's: the ×1e3 turned
                // operands do not finish at every ε
                // (`VolumeUncomputable`), and the radius-2000 disc does
                // not extrude at ε = 1e-12 (a pcurve certification), so
                // those corners are left out.
                if s == 1e3 && (turned || name == "cc unequal") {
                    continue;
                }
                let (a, b) = (a.scaled(s), b.scaled(s));
                let line = [Point3::new(s, 0.0, 0.0), Point3::new(s, 0.0, s)];
                let line = map.map_or(line, |m| line.map(|p| m.transform_point(p)));
                let what = format!("{name} ×{s:e}{}", if turned { " turned" } else { "" });
                let inside = [a, b].map(|sh| {
                    let p = sh.centre();
                    map.map_or(p, |m| m.transform_point(p))
                });
                let (ba, bb) = (body_of(a, map), body_of(b, map));
                crate::common::touch_union::assert_touch_at_every_op(&what, &ba, &bb, line, inside);
            }
        }
    }
}
