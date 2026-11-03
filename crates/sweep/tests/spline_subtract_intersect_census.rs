//! **∖ and ∩ of a spline operand refuse where ∪ does, or build right.**
//! One roster serves every op (`boolean::reduce::boolean_arm_exists`):
//! ∖ and ∩ route regions through `Body::revert`, which flips each face's
//! `sense` and keeps its carrier, so a spline face meets the doors under
//! ∖ and ∩ that it meets under ∪. This census holds that fact over the
//! spline operands against a partner of every kind, in both operand
//! orders, so the day a ∪ lane starts building a spline body its ∖ and ∩
//! are checked rather than carried along unseen. Each run of ∖ or ∩:
//!
//! - **refuses**: never a kernel invariant, and where ∪ of the same
//!   operand order refuses, with ∪'s refusal exactly, variant and payload;
//! - **or builds**, and its body is held to the volume oracle — the
//!   closed form where the partner lies inside the operand, and
//!   otherwise inclusion–exclusion, every op that builds implying one
//!   shared `V(O ∩ B)` inside `[0, min(V(O), V(B))]` — and to a point
//!   sample against both operands, at every point `point_in_solid`
//!   decides in all three bodies.
//!
//! The operands: the loft prism (`V = 9`), the lofted cylinder
//! (`V = 2π`), the wavy loft (2 × 2 squares at `z = 0, 1, 2, 3` offset
//! `∓0.3` in x, `V = 12`: every section is a unit-area translate), and the
//! saddle block against its tangent half-space (`V = 8`,
//! `V(O ∩ B) = 1/2`). The partners: a brick, a cylinder rod, a ball, a
//! cone frustum and a ring torus, each wholly inside the operand and
//! across its wall; a lofted cylinder across the wall; a peg through the
//! top cap; a slab through every wall.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_core::{Band, Point2, Point3, RigidMotion, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, RawLoop};
use sweep::test_support::{
    ball_poled_z, cylinder_of_arcs_at, finished, loft_prism, lofted_cylinder, revolved_about_y,
    stacked_at,
};
use topo::{AtRestBody, BooleanError, BooleanResult, SolidContainment};

fn tol() -> Tol {
    Tol::witness()
}

fn moved(b: topo::Body<f64>, d: [f64; 3]) -> topo::Body<f64> {
    topo::transform_rigid(
        &b,
        &RigidMotion::translation(Vec3::new(d[0], d[1], d[2])).unwrap(),
        tol(),
    )
    .unwrap()
}

/// The wavy loft (module docs).
fn wavy() -> topo::Body<f64> {
    let a = 0.3;
    let sections: Vec<Vec<ProfileLoop<f64>>> = [-a, a, -a, a]
        .into_iter()
        .map(|dx| {
            vec![ProfileLoop::polygon(
                [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
                    .into_iter()
                    .map(|(x, y)| Point2::new(dx + x, y)),
            )]
        })
        .collect();
    sweep::loft_body::<f64>(&sections, &stacked_at(&[0.0, 1.0, 2.0, 3.0]), 2, tol())
        .expect("the wavy loft skins")
        .body
}

/// The block `x, z ∈ [−1, 1]`, `−2 ≤ y ≤ x·z`, lofted along `z`.
fn saddle() -> topo::Body<f64> {
    let section = |s: f64| {
        let p = |x: f64, y: f64| (Point2::new(x, y), 0.0);
        vec![bulge_loop(vec![
            p(-1.0, -2.0),
            p(1.0, -2.0),
            p(1.0, s),
            p(-1.0, -s),
        ])]
    };
    sweep::loft_body(
        &[section(-1.0), section(1.0)],
        &stacked_at(&[-1.0, 1.0]),
        1,
        tol(),
    )
    .expect("the saddle block lofts")
    .body
}

/// A frustum about `y`, radius `r0` at `y0` and `r1` at `y1`, and its
/// volume.
fn frustum(r0: f64, r1: f64, y0: f64, y1: f64) -> (topo::Body<f64>, f64) {
    let b = revolved_about_y(
        vec![
            (Point2::new(0.0, y0), 0.0),
            (Point2::new(r0, y0), 0.0),
            (Point2::new(r1, y1), 0.0),
            (Point2::new(0.0, y1), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    );
    (b, PI * (y1 - y0) / 3.0 * (r0 * r0 + r0 * r1 + r1 * r1))
}

/// A ring torus about `y`, major radius `big`, minor `r`, and its volume.
fn torus(big: f64, r: f64) -> (topo::Body<f64>, f64) {
    let b = revolved_about_y(
        vec![
            (Point2::new(big - r, 0.0), 1.0),
            (Point2::new(big + r, 0.0), 1.0),
        ],
        sweep::Revolution::Full,
        tol(),
    );
    (b, 2.0 * PI * PI * big * r * r)
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> (topo::Body<f64>, f64) {
    (
        topo::test_support::brick_with_fresh_keys::<f64>(x, y, z, tol()),
        (x.1 - x.0) * (y.1 - y.0) * (z.1 - z.0),
    )
}

/// The refusals that name a broken kernel rather than an input or a
/// frontier.
fn kernel_invariant(e: &BooleanError) -> bool {
    matches!(
        e,
        BooleanError::ClassificationInvariant { .. } | BooleanError::JoinDesync { .. }
    ) || matches!(e, BooleanError::Join(j) if matches!(j.error(), topo::SplitJoinError::SectionInvariant { .. }))
}

/// How far a volume `v` may read from its oracle beyond the brackets.
fn band(v: f64) -> f64 {
    1e-6 * v.abs().max(1.0)
}

/// `body`'s volume at 1e-6 m and its bracket's half-width.
fn measure(what: &str, body: &AtRestBody<f64>) -> (f64, f64) {
    let m = topo::test_support::volume_at(what, body, tol(), 1e-6);
    (m.volume, m.volume_pad)
}

/// 400 points of the box `lo..hi`, a fixed xorshift sequence.
fn samples(lo: Point3<f64>, hi: Point3<f64>) -> Vec<Point3<f64>> {
    let mut s = 0x2545_f491_4f6c_dd1d_u64;
    let mut r = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        (s >> 11) as f64 / (1_u64 << 53) as f64
    };
    (0..400)
        .map(|_| {
            Point3::new(
                lo.x + (hi.x - lo.x) * r(),
                lo.y + (hi.y - lo.y) * r(),
                lo.z + (hi.z - lo.z) * r(),
            )
        })
        .collect()
}

/// `q` strictly in (`Some(true)`) or out of `b`; `None` on the boundary
/// or where the door does not decide.
fn inside(b: &topo::Body<f64>, q: Point3<f64>) -> Option<bool> {
    let band = Band::linear(tol()).unwrap();
    match topo::point_in_solid(b, q, band, tol()) {
        Ok(SolidContainment::In) => Some(true),
        Ok(SolidContainment::Out) => Some(false),
        _ => None,
    }
}

/// The two operands' vertex box, padded.
fn sample_box(bodies: [&topo::Body<f64>; 2]) -> (Point3<f64>, Point3<f64>) {
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    for b in bodies {
        for (_, v) in b.vertices() {
            let p = b.get_point(v.point).unwrap();
            for (k, c) in [p.x, p.y, p.z].into_iter().enumerate() {
                lo[k] = lo[k].min(c);
                hi[k] = hi[k].max(c);
            }
        }
    }
    let pad = 0.3;
    (
        Point3::new(lo[0] - pad, lo[1] - pad, lo[2] - pad),
        Point3::new(hi[0] + pad, hi[1] + pad, hi[2] + pad),
    )
}

/// Which region of the two operands an op keeps.
#[derive(Clone, Copy)]
enum Keep {
    Union,
    Meet,
    OLessB,
    BLessO,
}

impl Keep {
    fn holds(self, in_o: bool, in_b: bool) -> bool {
        match self {
            Keep::Union => in_o || in_b,
            Keep::Meet => in_o && in_b,
            Keep::OLessB => in_o && !in_b,
            Keep::BLessO => in_b && !in_o,
        }
    }

    /// `V(O ∩ B)` from the op's volume.
    fn meet(self, vo: f64, vb: f64, got: f64) -> f64 {
        match self {
            Keep::Union => vo + vb - got,
            Keep::Meet => got,
            Keep::OLessB => vo - got,
            Keep::BLessO => vb - got,
        }
    }
}

/// One case: spline operand `o` of closed-form volume `vo`, partner `b`
/// of `vb`, `meet` the closed-form `V(O ∩ B)` where there is one.
/// Returns the census lines.
fn case(
    name: &str,
    o: &AtRestBody<f64>,
    vo: f64,
    b: &AtRestBody<f64>,
    vb: f64,
    meet: Option<f64>,
) -> Vec<String> {
    let t = tol();
    let union_ob = topo::union(o, b, t);
    let union_bo = topo::union(b, o, t);
    let (meet_ob, meet_bo) = (topo::intersect(o, b, t), topo::intersect(b, o, t));
    let (less_ob, less_bo) = (topo::subtract(o, b, t), topo::subtract(b, o, t));
    let runs = [
        ("O∪B", &union_ob, None, Keep::Union),
        ("B∪O", &union_bo, None, Keep::Union),
        ("O∩B", &meet_ob, Some(&union_ob), Keep::Meet),
        ("B∩O", &meet_bo, Some(&union_bo), Keep::Meet),
        ("O∖B", &less_ob, Some(&union_ob), Keep::OLessB),
        ("B∖O", &less_bo, Some(&union_bo), Keep::BLessO),
    ];
    let (lo, hi) = sample_box([o, b]);
    let pts = samples(lo, hi);
    let mut lines = Vec::new();
    let mut implied: Vec<(&str, f64, f64)> = Vec::new();
    for (op, got, union_of_order, keep) in runs {
        let line = format!("{name} / {op}: ");
        match got {
            Err(e) => {
                assert!(
                    !kernel_invariant(e),
                    "{line}a kernel invariant, not a refusal: {e:?}"
                );
                if let Some(Err(u)) = union_of_order {
                    assert_eq!(
                        format!("{e:?}"),
                        format!("{u:?}"),
                        "{line}refuses other than ∪ of the same order"
                    );
                }
                lines.push(format!("{line}refuses {e:?}"));
            }
            Ok(r) => {
                let (v, pad, body) = match r {
                    BooleanResult::Empty => (0.0, 0.0, None),
                    BooleanResult::Body(bb) => {
                        let (v, pad) = measure(&format!("{line}the result"), &bb.body);
                        (v, pad, Some(&bb.body))
                    }
                };
                let m = keep.meet(vo, vb, v);
                if let Some(want) = meet {
                    topo::test_support::assert_volume_near(
                        &format!("{line}V(O ∩ B) against the closed form"),
                        m,
                        pad,
                        want,
                        band(want),
                    );
                }
                implied.push((op, m, pad));
                let mut decided = 0;
                for &q in &pts {
                    let (Some(in_o), Some(in_b)) = (inside(o, q), inside(b, q)) else {
                        continue;
                    };
                    let have = match body {
                        None => Some(false),
                        Some(bb) => inside(bb, q),
                    };
                    if let Some(have) = have {
                        decided += 1;
                        assert_eq!(
                            have,
                            keep.holds(in_o, in_b),
                            "{line}the result at {q:?} against the operands"
                        );
                    }
                }
                lines.push(format!(
                    "{line}builds V = {v} ± {pad:e}, {decided} points decided"
                ));
            }
        }
    }
    if let Some(&(_, first, first_pad)) = implied.first() {
        let slack = 1e-9 * vo.max(vb) + first_pad;
        assert!(
            (-slack..=vo.min(vb) + slack).contains(&first),
            "{name}: V(O ∩ B) = {first} lies outside [0, min(V(O), V(B))]"
        );
        for &(op, m, pad) in &implied {
            assert!(
                (m - first).abs() <= 1e-9 * vo.max(vb) + pad + first_pad,
                "{name}: {op} implies V(O ∩ B) = {m}, the others {first}"
            );
        }
    }
    lines
}

/// A spline operand: its name, the body, its volume, a point inside it
/// and a point on its wall.
type Operand<'a> = (&'a str, &'a AtRestBody<f64>, f64, [f64; 3], [f64; 3]);

#[test]
fn subtract_and_intersect_of_a_spline_operand_refuse_where_union_does_or_build_right() {
    let t = tol();
    let prism = finished("the loft prism", loft_prism(t), t);
    let lcyl = finished("the lofted cylinder", lofted_cylinder(2.0, t), t);
    let wavy = finished("the wavy loft", wavy(), t);
    let saddle = finished("the saddle block", saddle(), t);
    let mut lines = Vec::new();
    // Each operand, its volume, a point inside it and a point on its wall.
    let operands: [Operand<'_>; 3] = [
        ("loft prism", &prism, 9.0, [0.0, 0.0, 1.0], [0.0, 1.0, 1.0]),
        (
            "lofted cylinder",
            &lcyl,
            2.0 * PI,
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 1.0],
        ),
        ("wavy loft", &wavy, 12.0, [0.0, 0.0, 1.5], [0.0, 1.0, 1.5]),
    ];
    for (on, o, vo, c, w) in operands {
        let top = 2.0 * c[2];
        // Each partner, its volume, and `V(O ∩ B)` where it lies inside.
        let mut partners: Vec<(String, topo::Body<f64>, f64, Option<f64>)> = Vec::new();
        for (at, p, contained) in [("inside", c, true), ("across the wall", w, false)] {
            let within = |v: f64| contained.then_some(v);
            let (bk, vb) = brick(
                (p[0] - 0.25, p[0] + 0.25),
                (p[1] - 0.25, p[1] + 0.25),
                (p[2] - 0.25, p[2] + 0.25),
            );
            partners.push((format!("a brick {at}"), bk, vb, within(vb)));
            let rod = cylinder_of_arcs_at(4, 0.2, Point2::new(p[0], p[1]), p[2] - 0.3, 0.6, t);
            let vr = PI * 0.04 * 0.6;
            partners.push((format!("a cylinder rod {at}"), rod, vr, within(vr)));
            let ball = ball_poled_z(0.25, Vec3::new(p[0], p[1], p[2]), t);
            let vball = 4.0 / 3.0 * PI * 0.25_f64.powi(3);
            partners.push((format!("a ball {at}"), ball, vball, within(vball)));
            let (fr, vf) = frustum(0.2, 0.1, -0.25, 0.25);
            partners.push((format!("a cone frustum {at}"), moved(fr, p), vf, within(vf)));
            let (to, vt) = torus(0.15, 0.06);
            partners.push((format!("a torus {at}"), moved(to, p), vt, within(vt)));
        }
        let lc = moved(lofted_cylinder(0.5, t), [w[0], w[1], w[2] - 0.25]);
        partners.push((
            "a lofted cylinder across the wall".into(),
            lc,
            PI * 0.5,
            None,
        ));
        let (peg, vpeg) = brick((-0.25, 0.25), (-0.25, 0.25), (top - 0.25, top + 0.5));
        partners.push(("a peg through the top cap".into(), peg, vpeg, None));
        let (slab, vs) = brick((-3.0, 3.0), (-0.25, 0.25), (c[2] - 0.25, c[2] + 0.25));
        partners.push(("a slab through every wall".into(), slab, vs, None));
        for (pn, pb, vb, meet) in partners {
            let pb = finished(&pn, pb, t);
            lines.extend(case(&format!("{on} × {pn}"), o, vo, &pb, vb, meet));
        }
    }
    let (h, vh) = brick((-4.0, 4.0), (0.0, 4.0), (-4.0, 4.0));
    let h = finished("the half-space", h, t);
    lines.extend(case(
        "saddle × its tangent half-space",
        &saddle,
        8.0,
        &h,
        vh,
        Some(0.5),
    ));
    for line in &lines {
        println!("{line}");
    }
}
