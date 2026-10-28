//! **Review rows, R1 of PR 4781: same-side tangent touches at every op,
//! read against an analytic oracle.** Pairs beyond the PR's fixtures:
//! a box whose edge lies on a cylinder's seam ruling (plane × cylinder),
//! two cylinders of unequal radii kissing (cylinder × cylinder), both
//! operand orders, three scales, and near-tangent poses inside and
//! outside the band. Each op is judged three ways that share nothing
//! with the Boolean: volume against the closed form, solid count, and
//! `point_in_solid` at steady sample points against analytic
//! membership.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::approx::band;
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop};
use sweep::test_support::{brick, finished, sketch_at};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, BooleanResult, SolidContainment, mass_properties};

fn tol() -> Tol {
    Tol::witness()
}

fn circle(cx: f64, cy: f64, r: f64) -> ProfileLoop<f64> {
    bulge_loop(vec![
        (Point2::new(cx + r, cy), 1.0),
        (Point2::new(cx - r, cy), 1.0),
    ])
}

fn cyl(cx: f64, cy: f64, r: f64, z0: f64, z1: f64) -> AtRestBody<f64> {
    let profile = Profile::new(sketch_at(z0), vec![circle(cx, cy, r)])
        .validate(tol())
        .unwrap();
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth: z1 - z0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    finished("cyl", body, tol())
}

fn boxed(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    finished("box", brick::<f64>(x, y, z, tol()), tol())
}

/// An analytic solid: a z-cylinder or an axis box.
#[derive(Clone, Copy, Debug)]
enum Shape {
    Cyl {
        c: (f64, f64),
        r: f64,
        z: (f64, f64),
    },
    Box {
        x: (f64, f64),
        y: (f64, f64),
        z: (f64, f64),
    },
}

impl Shape {
    fn body(self) -> AtRestBody<f64> {
        match self {
            Self::Cyl { c, r, z } => cyl(c.0, c.1, r, z.0, z.1),
            Self::Box { x, y, z } => boxed(x, y, z),
        }
    }
    fn has(self, p: Point3<f64>) -> bool {
        match self {
            Self::Cyl { c, r, z } => {
                (p.x - c.0).powi(2) + (p.y - c.1).powi(2) <= r * r && (z.0..=z.1).contains(&p.z)
            }
            Self::Box { x, y, z } => {
                (x.0..=x.1).contains(&p.x)
                    && (y.0..=y.1).contains(&p.y)
                    && (z.0..=z.1).contains(&p.z)
            }
        }
    }
    fn bbox(self) -> ([f64; 3], [f64; 3]) {
        match self {
            Self::Cyl { c, r, z } => ([c.0 - r, c.1 - r, z.0], [c.0 + r, c.1 + r, z.1]),
            Self::Box { x, y, z } => ([x.0, y.0, z.0], [x.1, y.1, z.1]),
        }
    }
    fn scaled(self, s: f64) -> Self {
        match self {
            Self::Cyl { c, r, z } => Self::Cyl {
                c: (c.0 * s, c.1 * s),
                r: r * s,
                z: (z.0 * s, z.1 * s),
            },
            Self::Box { x, y, z } => Self::Box {
                x: (x.0 * s, x.1 * s),
                y: (y.0 * s, y.1 * s),
                z: (z.0 * s, z.1 * s),
            },
        }
    }
    fn shifted_x(self, d: f64) -> Self {
        match self {
            Self::Cyl { c, r, z } => Self::Cyl {
                c: (c.0 + d, c.1),
                r,
                z,
            },
            Self::Box { x, y, z } => Self::Box {
                x: (x.0 + d, x.1 + d),
                y,
                z,
            },
        }
    }
}

/// Steady samples: membership unchanged a step `h` away on each axis.
fn samples(a: Shape, b: Shape, h: f64) -> Vec<(Point3<f64>, bool, bool)> {
    let (la, ua) = a.bbox();
    let (lb, ub) = b.bbox();
    let lo: Vec<f64> = (0..3).map(|i| la[i].min(lb[i])).collect();
    let hi: Vec<f64> = (0..3).map(|i| ua[i].max(ub[i])).collect();
    let mut s: u64 = 0x9e37_79b9_7f4a_7c15;
    let mut next = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        (s >> 11) as f64 / (1u64 << 53) as f64
    };
    let mut out = Vec::new();
    let mut tries = 0;
    while out.len() < 150 && tries < 100_000 {
        tries += 1;
        let p = Point3::new(
            lo[0] + (hi[0] - lo[0]) * (1.2 * next() - 0.1),
            lo[1] + (hi[1] - lo[1]) * (1.2 * next() - 0.1),
            lo[2] + (hi[2] - lo[2]) * (1.2 * next() - 0.1),
        );
        let (ia, ib) = (a.has(p), b.has(p));
        let steady = [
            Vec3::new(h, 0.0, 0.0),
            Vec3::new(-h, 0.0, 0.0),
            Vec3::new(0.0, h, 0.0),
            Vec3::new(0.0, -h, 0.0),
            Vec3::new(0.0, 0.0, h),
            Vec3::new(0.0, 0.0, -h),
        ]
        .iter()
        .all(|&o| a.has(p + o) == ia && b.has(p + o) == ib);
        if steady {
            out.push((p, ia, ib));
        }
    }
    // Probe points straddling the touch line region densely too.
    out
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Union,
    AMinusB,
    BMinusA,
    Meet,
}

/// One op judged; `want_solids` is checked when the op answers a body.
fn judge(
    op: Op,
    r: &Result<BooleanResult<f64>, topo::BooleanError>,
    want_vol: f64,
    want_solids: Option<usize>,
    pts: &[(Point3<f64>, bool, bool)],
    vol_scale: f64,
) -> String {
    let want_in = |a: bool, b: bool| match op {
        Op::Union => a || b,
        Op::AMinusB => a && !b,
        Op::BMinusA => b && !a,
        Op::Meet => a && b,
    };
    match r {
        Err(e) => format!("REFUSED {:?} {e:?}", e.kind()),
        Ok(BooleanResult::Empty) => {
            if want_vol > 1e-9 * vol_scale {
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
            if (v - want_vol).abs() > 1e-9 * vol_scale {
                bad.push(format!("volume {v} vs {want_vol}"));
            }
            let solids = bb.body.solids().count();
            if let Some(n) = want_solids
                && solids != n
            {
                bad.push(format!("solids {solids} vs {n}"));
            }
            if let Err(e) = topo::validate_geometric(&bb.body, tol()) {
                bad.push(format!("tier3 {e:?}"));
            }
            let mut miss = 0;
            let mut unread = 0;
            for &(p, a, b) in pts {
                let got = topo::point_in_solid(&bb.body, p, band(), tol());
                if matches!(
                    got,
                    Err(topo::PointInSolidError::Escalated { .. }
                        | topo::PointInSolidError::RayExhausted
                        | topo::PointInSolidError::Loop(topo::PointInLoopError::Escalated { .. }))
                ) {
                    unread += 1;
                    continue;
                }
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
            if bad.is_empty() && unread > 0 {
                format!(
                    "UNPROBED body solids={solids} kind={:?} ee={ee}: the membership \
                     oracle escalates on {unread} of {} samples",
                    bb.kind,
                    pts.len()
                )
            } else if bad.is_empty() {
                format!("OK body solids={solids} kind={:?} ee={ee}", bb.kind)
            } else {
                format!(
                    "WRONG solids={solids} kind={:?} ee={ee}: {}",
                    bb.kind,
                    bad.join("; ")
                )
            }
        }
    }
}

/// All four ops; `meet` the closed-form shared volume, `touch_only`
/// whether a union owes two solids.
fn pose(a: Shape, b: Shape, meet: f64, union_solids: Option<usize>) -> Vec<String> {
    let (ba, bb) = (a.body(), b.body());
    let vol = |x: &AtRestBody<f64>| {
        mass_properties(x, tol(), topo::test_support::pinned_resolution(tol()))
            .unwrap()
            .met()
            .expect("the pinned resolution is met")
            .volume
    };
    let (va, vb) = (vol(&ba), vol(&bb));
    let scale = va.max(vb);
    let (lo, hi) = a.bbox();
    // Steady past the band's escalation width: a sample nearer a face
    // than that honestly reads on the boundary.
    let h = (1e-4 * (hi[0] - lo[0])).max(band().escalate());
    let pts = samples(a, b, h);
    let solo = if union_solids == Some(2) {
        Some(1)
    } else {
        None
    };
    [
        (
            Op::Union,
            topo::union(&ba, &bb, tol()),
            va + vb - meet,
            union_solids,
        ),
        (
            Op::AMinusB,
            topo::subtract(&ba, &bb, tol()),
            va - meet,
            solo,
        ),
        (
            Op::BMinusA,
            topo::subtract(&bb, &ba, tol()),
            vb - meet,
            solo,
        ),
        (Op::Meet, topo::intersect(&ba, &bb, tol()), meet, None),
    ]
    .into_iter()
    .map(|(op, got, want, n)| format!("{op:?}: {}", judge(op, &got, want, n, &pts, scale)))
    .collect()
}

/// The touch-only pairs, `(name, a, b)`: each pair shares no volume and
/// touches along a straight line on which both have an edge.
fn touch_pairs() -> Vec<(&'static str, Shape, Shape)> {
    let unit = Shape::Cyl {
        c: (0.0, 0.0),
        r: 1.0,
        z: (0.0, 2.0),
    };
    vec![
        // Box face x = 1 tangent to the wall at the seam (1, 0); the box's
        // edge x = 1, y = 0 lies on the seam ruling. Same z extent.
        (
            "box on seam, full height",
            unit,
            Shape::Box {
                x: (1.0, 3.0),
                y: (0.0, 2.0),
                z: (0.0, 2.0),
            },
        ),
        // The box's edge shorter than the seam: a partial overlap.
        (
            "box on seam, short box",
            unit,
            Shape::Box {
                x: (1.0, 3.0),
                y: (0.0, 2.0),
                z: (0.5, 1.5),
            },
        ),
        // The box's edge longer than the seam.
        (
            "box on seam, tall box",
            unit,
            Shape::Box {
                x: (1.0, 3.0),
                y: (0.0, 2.0),
                z: (-1.0, 3.0),
            },
        ),
        // The box on the other side of the seam (y ≤ 0).
        (
            "box on seam, -y",
            unit,
            Shape::Box {
                x: (1.0, 3.0),
                y: (-2.0, 0.0),
                z: (0.0, 2.0),
            },
        ),
        // Unequal radii kissing at (1, 0), staggered heights.
        (
            "cyl r1 / r0.5 staggered",
            unit,
            Shape::Cyl {
                c: (1.5, 0.0),
                r: 0.5,
                z: (0.5, 3.0),
            },
        ),
        // Equal radii, same heights (the PR's disc kiss, as cylinders).
        (
            "cyl r1 / r1 equal",
            unit,
            Shape::Cyl {
                c: (2.0, 0.0),
                r: 1.0,
                z: (0.0, 2.0),
            },
        ),
    ]
}

/// **Every touch-only pair answers every op right, both orders, three
/// scales**, edges of unequal extent along the line included, read
/// against the running ε through [`coarse_at`] and
/// [`past_f64_headroom`]. Prints each line; red on any other outcome.
#[test]
fn review_4781_touch_pairs_every_op() {
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
        review_4781_touch_pairs_every_op_row,
    );
}

/// The row [`review_4781_touch_pairs_every_op`] runs, under its pin.
fn review_4781_touch_pairs_every_op_row() {
    let eps = tol().get().eps;
    let mut bad = Vec::new();
    for (name, a, b) in touch_pairs() {
        for s in [1e-3, 1.0, 1e3] {
            for (order, x, y) in [("ab", a, b), ("ba", b, a)] {
                let (x, y) = (x.scaled(s), y.scaled(s));
                let (coarse, past_headroom) = (coarse_at(x, y, eps), past_f64_headroom(x, y, eps));
                for line in pose(x, y, 0.0, Some(2)) {
                    println!("{name} s={s:e} {order} | {line}");
                    let op = line.split(':').next().unwrap_or_default();
                    let ok = line.contains(": OK ")
                        || (coarse && line.contains(": UNPROBED "))
                        || (past_headroom
                            && name.starts_with("cyl ")
                            && ["Escalated", "CurvedPierceUnsupported"]
                                .iter()
                                .any(|k| line.starts_with(&format!("{op}: REFUSED {k} "))));
                    if !ok {
                        bad.push(format!(
                            "eps {eps:e} {name} s={s:e} {order} coarse={coarse} \
                             past_headroom={past_headroom} | {line}"
                        ));
                    }
                }
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// Whether the pose's smallest feature (a radius, a box half-extent)
/// spans under 1e4 ε. Rays of the membership oracle then graze a face
/// inside the band often enough that some samples escalate; the body
/// is still judged whole, and every sample that reads must agree.
fn coarse_at(a: Shape, b: Shape, eps: f64) -> bool {
    let feature = [a, b]
        .iter()
        .map(|&sh| {
            let (lo, hi) = sh.bbox();
            (0..3)
                .map(|i| 0.5 * (hi[i] - lo[i]))
                .fold(f64::MAX, f64::min)
        })
        .fold(f64::MAX, f64::min);
    feature < 1e4 * eps
}

/// Whether ε sits within two orders of the pose's own resolution floor,
/// its reach times 2⁻⁵² (`geom_core::EPS_MIN` is the same product at
/// the session box's edge). A cylinder kiss's clearance there reads
/// tens of ulps of rounding, which spans the band, so the kiss may
/// refuse with either half of its two-tolerance pair — `Escalated` in
/// band, `CurvedPierceUnsupported` past it, which half depending on
/// where ε falls against the rounding — and never answers wrong.
fn past_f64_headroom(a: Shape, b: Shape, eps: f64) -> bool {
    let reach = [a.bbox(), b.bbox()]
        .iter()
        .flat_map(|(lo, hi)| lo.iter().chain(hi))
        .fold(0.0_f64, |m, c| m.max(c.abs()));
    100.0 * reach * f64::EPSILON > eps
}

/// Area of the half circular segment of depth `h` of a radius-`r`
/// circle (the half on one side of the chord's perpendicular through
/// the centre).
fn half_segment(r: f64, h: f64) -> f64 {
    let d = r - h;
    0.5 * (r * r * (d / r).acos() - d * (2.0 * r * h - h * h).sqrt())
}

/// The slides a near-tangent row runs, as multiples of the band's
/// zero: inside it, between it and the escalation width, and past both.
fn near_slides() -> Vec<f64> {
    let zero = band().zero();
    [1e-2, 3.0, 1e2, 1e4]
        .iter()
        .flat_map(|k| [k * zero, -k * zero])
        .collect()
}

/// One near-tangent pose slid by `d`, both orders, against the true
/// shared volume `meet`. Short of the escalation width the touch
/// reading (no shared volume) is right too; past it only the truth is.
/// Red lines go to `bad`: `WRONG` or `ClassificationInvariant`.
fn near_pose(a: Shape, b: Shape, d: f64, meet: f64, bad: &mut Vec<String>) {
    let in_band = d.abs() < band().escalate();
    let red = |l: &str| l.contains("WRONG") || l.contains("ClassificationInvariant");
    for (order, x, y) in [("ab", a, b), ("ba", b, a)] {
        let touch = if in_band && meet > 0.0 {
            pose(x, y, 0.0, None)
        } else {
            Vec::new()
        };
        for (i, line) in pose(x, y, meet, None).into_iter().enumerate() {
            println!("d={d:e} {order} | {line}");
            if red(&line) && touch.get(i).is_none_or(|t| red(t)) {
                bad.push(format!("d={d:e} in_band={in_band} {order} | {line}"));
            }
        }
    }
}

/// **Near-tangent poses**: the box slid by `d` along x (positive apart,
/// negative into the wall), `d` read against the running band. Short of
/// the escalation width it may read as the touch; past it, as disjoint
/// or a typed refusal or a correct crossing answer — never an untyped
/// invariant or a wrong body. Prints each outcome; red on `WRONG` or
/// `ClassificationInvariant`.
#[test]
fn review_4781_near_tangent_box_on_seam() {
    let unit = Shape::Cyl {
        c: (0.0, 0.0),
        r: 1.0,
        z: (0.0, 2.0),
    };
    let bx = Shape::Box {
        x: (1.0, 3.0),
        y: (0.0, 2.0),
        z: (0.0, 2.0),
    };
    let mut bad = Vec::new();
    for d in near_slides().into_iter().chain([1e-3, -1e-3]) {
        let meet = if d < 0.0 {
            2.0 * half_segment(1.0, -d)
        } else {
            0.0
        };
        near_pose(unit, bx.shifted_x(d), d, meet, &mut bad);
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **Near-tangent cylinders**: r 1 at the origin and r 0.5 slid by `d`,
/// read as [`review_4781_near_tangent_box_on_seam`] reads the box.
#[test]
fn review_4781_near_tangent_cylinders() {
    let unit = Shape::Cyl {
        c: (0.0, 0.0),
        r: 1.0,
        z: (0.0, 2.0),
    };
    let small = Shape::Cyl {
        c: (1.5, 0.0),
        r: 0.5,
        z: (0.5, 3.0),
    };
    let mut bad = Vec::new();
    for d in near_slides() {
        // The lens of two circles at centre distance 1.5 + d, over the
        // shared height 1.5.
        let meet = if d < 0.0 {
            let (r1, r2, c): (f64, f64, f64) = (1.0, 0.5, 1.5 + d);
            let a1 = r1 * r1 * ((c * c + r1 * r1 - r2 * r2) / (2.0 * c * r1)).acos();
            let a2 = r2 * r2 * ((c * c + r2 * r2 - r1 * r1) / (2.0 * c * r2)).acos();
            let k = 0.5 * ((-c + r1 + r2) * (c + r1 - r2) * (c - r1 + r2) * (c + r1 + r2)).sqrt();
            1.5 * (a1 + a2 - k)
        } else {
            0.0
        };
        near_pose(unit, small.shifted_x(d), d, meet, &mut bad);
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **A kiss that also overlaps elsewhere** (same-side tangent flank,
/// materials on ONE side): box `x ∈ [-3, 1], y ∈ [0, 3]` whose face
/// `x = 1` is tangent to the unit cylinder at the seam `(1, 0)`, box
/// edge on the seam ruling, and the cylinder's `y ≥ 0` half inside the
/// box. ∩ is the half cylinder. Every op answers, both orders.
#[test]
fn review_4781_kiss_with_overlap() {
    let unit = Shape::Cyl {
        c: (0.0, 0.0),
        r: 1.0,
        z: (0.0, 2.0),
    };
    let bx = Shape::Box {
        x: (-3.0, 1.0),
        y: (0.0, 3.0),
        z: (-0.5, 2.5),
    };
    let meet = core::f64::consts::PI;
    let mut bad = Vec::new();
    for (order, x, y) in [("ab", unit, bx), ("ba", bx, unit)] {
        for line in pose(x, y, meet, None) {
            println!("{order} | {line}");
            if !line.contains(": OK ") {
                bad.push(format!("{order} | {line}"));
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **The edge–edge record survives a later op**: union the touch pair,
/// then, with the union's records carried as operand A's, union a
/// far-away brick and subtract a notch that does not reach the line,
/// and a notch that cuts through the line's top end. Red when the
/// later op drops the edge–edge row, or lands it off the line, or when
/// the result does not stay 3′ up to the census's curved-pair refusal.
#[test]
fn review_4781_ee_record_after_a_later_op() {
    let a = cyl(0.0, 0.0, 1.0, 0.0, 2.0);
    let b = cyl(2.0, 0.0, 1.0, 0.0, 2.0);
    let u = match topo::union(&a, &b, tol()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("{other:?}"),
    };
    let first: Vec<_> = u.contacts.edge_edge().collect();
    println!("first union ee: {first:?}");
    let inputs = topo::BooleanInputs {
        records_a: u.contacts.carried(),
        ..topo::BooleanInputs::default()
    };
    let far = boxed((10.0, 11.0), (0.0, 1.0), (0.0, 1.0));
    let notch = boxed((-0.5, 0.5), (-0.5, 0.5), (1.5, 3.0));
    let top = boxed((0.5, 1.5), (-0.2, 0.2), (1.5, 3.0));
    let on_line = |r: &topo::BooleanBody<f64>, e: topo::EdgeKey| {
        let edge = r.body.get_edge(e).unwrap();
        [edge.he_plus, edge.he_minus].iter().all(|&he| {
            let p = r.body.half_edge_start_point(he).unwrap();
            (p.x - 1.0).abs() < 1e-9 && p.y.abs() < 1e-9
        })
    };
    let mut bad = Vec::new();
    for (what, op, cutter, want_v) in [
        (
            "∪ far brick",
            topo::BooleanOp::Union,
            &far,
            4.0 * core::f64::consts::PI + 1.0,
        ),
        (
            "∖ notch off the line",
            topo::BooleanOp::Subtract,
            &notch,
            4.0 * core::f64::consts::PI - 0.5,
        ),
        (
            "∖ notch across the line's top",
            topo::BooleanOp::Subtract,
            &top,
            f64::NAN,
        ),
    ] {
        match topo::boolean_op_with(
            op,
            &u.body,
            cutter,
            &inputs,
            topo::SweepStrategy::Realized,
            tol(),
        ) {
            Ok(BooleanResult::Body(r)) => {
                let ee: Vec<_> = r.contacts.edge_edge().collect();
                let v =
                    mass_properties(&r.body, tol(), topo::test_support::pinned_resolution(tol()))
                        .unwrap()
                        .met()
                        .expect("the pinned resolution is met")
                        .volume;
                let verdict = topo::validate_pseudomanifold(&r.body, &r.contacts, tol());
                let short = verdict.as_ref().map_err(|e| {
                    e.iter()
                        .map(|x| format!("{x:?}").chars().take(200).collect::<String>())
                        .collect::<Vec<_>>()
                });
                let line = format!(
                    "{what}: kind={:?} solids={} v={v} ee={ee:?} on_line={:?} 3'={short:?}",
                    r.kind,
                    r.body.solids().count(),
                    ee.iter()
                        .map(|&(x, y)| on_line(&r, x) && on_line(&r, y))
                        .collect::<Vec<_>>()
                );
                println!("{line}");
                let undecidable = |e: &topo::ValidationError| {
                    matches!(e, topo::ValidationError::CensusUndecidable { .. })
                };
                let three_ok = verdict
                    .as_ref()
                    .err()
                    .is_none_or(|e| e.iter().all(undecidable));
                if ee.is_empty()
                    || !ee.iter().all(|&(x, y)| on_line(&r, x) && on_line(&r, y))
                    || !three_ok
                    || (!want_v.is_nan() && (v - want_v).abs() > 1e-9 * want_v)
                {
                    bad.push(line);
                }
            }
            // A typed refusal of the later op is not a record defect;
            // an untyped invariant is.
            Err(e) if !matches!(e, topo::BooleanError::ClassificationInvariant { .. }) => {
                println!("{what}: refused typed {e:?}");
            }
            other => {
                println!("{what}: {other:?}");
                bad.push(format!("{what}: {other:?}"));
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
