//! **Every op of a spline operand ends in a typed refusal or a checked
//! body.** One face-kind roster serves every op
//! (`boolean::reduce::boolean_arm_exists`), so a loft or sweep body
//! reaches ∖ and ∩ wherever it reaches ∪. The doors are not the same
//! under each op: ∖ and ∩ classify the operands unreverted through
//! op-dependent tables (an on-sector's Eq. 15.3 lump,
//! `boolean::tables::eq15_3_lump`; the kept side of a coincident pair;
//! the subtrahend's flipped normals in the join's `stands_on`), and ∖
//! reverts its kept subtrahend pieces only after classification, so a
//! placement can stop at another door under ∖ or ∩ than under ∪ (a cap
//! shared with or stacked on the partner reaches the crossings path
//! under ∪ and the extent re-gate under ∩ and ∖). This census holds
//! what every op does owe, over the spline operands against a partner
//! of every kind, in both operand orders, so the day a lane starts
//! building a spline body its ∖ and ∩ are checked rather than carried
//! along unseen. Each run:
//!
//! - **refuses** typed, at whatever door, never a kernel invariant;
//! - **or builds**, and its body is held to the volume oracle — the
//!   closed form where the placement has one, and inclusion–exclusion
//!   always, every op that builds implying one shared `V(O ∩ B)` inside
//!   `[0, min(V(O), V(B))]` ([`crate::common::op_census`]) — and to a
//!   point sample against both operands, at every point `point_in_solid`
//!   decides in all three bodies.
//!
//! **What it cannot see.** `point_in_solid` decides no point against a
//! NURBS face, so the point sample is silent against a spline operand,
//! and a volume reads the same whatever `sense` a closed spline shell's
//! faces carry; a wrong `sense` on a built spline face is the void row's
//! below (`a_subtracted_spline_void_carries_its_walls_reverted`) and the
//! at-rest gate's gap
//! (`finding_a_cavity_with_its_spline_walls_unreversed_passes_the_gate_at_the_true_volume`).
//!
//! The operands: the loft prism (`V = 9`), the lofted cylinder
//! (`V = 2π`), the wavy loft (2 × 2 squares at `z = 0, 1, 2, 3` offset
//! `∓0.3` in x, `V = 12`: every section is a unit-area translate), and the
//! saddle block against its tangent half-space (`V = 8`,
//! `V(O ∩ B) = 1/2`). The partners: a brick, a cylinder rod, a ball, a
//! cone frustum and a ring torus, each wholly inside the operand and
//! across its wall; a lofted cylinder across the wall; a peg through the
//! top cap; a slab through every wall; and the reviewers' placements
//! (PR 6476) against the prism and the lofted cylinder — stacked on a
//! cap, around it sharing a cap plane, face to face with a wall, inside
//! on a cap or a wall, touching a wall along a ruling.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_core::{Band, Point2, Point3, RigidMotion, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, RawLoop};
use sweep::test_support::{
    ball_poled_z, cylinder_of_arcs_at, finished, loft_prism, lofted_cylinder, revolved_about_y,
    stacked_at,
};
use topo::{AtRestBody, BooleanResult, SolidContainment};

use crate::common::op_census::{
    Run, assert_one_meet, band, kernel_invariant, result_volume, six_runs,
};

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
    let (lo, hi) = sample_box([o, b]);
    let pts = samples(lo, hi);
    let mut lines = Vec::new();
    let mut implied: Vec<(&str, f64, f64)> = Vec::new();
    for Run { op, keep, got, .. } in six_runs(o, b) {
        let line = format!("{name} / {op}: ");
        match got {
            Err(e) => {
                assert!(
                    !kernel_invariant(&e),
                    "{line}a kernel invariant, not a refusal: {e:?}"
                );
                lines.push(format!("{line}refuses {e:?}"));
            }
            Ok(r) => {
                let (v, pad) = result_volume(&format!("{line}the result"), &r);
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
                let body = match &r {
                    BooleanResult::Empty => None,
                    BooleanResult::Body(bb) => Some(&bb.body),
                };
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
    assert_one_meet(name, &implied, vo, 0.0, vb);
    lines
}

type Box3 = ((f64, f64), (f64, f64), (f64, f64));

/// A spline operand: its name, the body, its volume, a point inside it
/// and a point on its wall.
type Operand<'a> = (&'a str, &'a AtRestBody<f64>, f64, [f64; 3], [f64; 3]);

#[test]
fn every_op_of_a_spline_operand_refuses_typed_or_builds_right() {
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
    // The reviewers' placements (PR 6476): the prism's box is
    // `[−1.375, 1.375] × [−1, 1] × [0, 2]`, its caps at `z = 0, 2`, its
    // `±y` walls flat at `y = ±1`.
    let prism_placements: [(&str, Box3, f64); 15] = [
        (
            "stacked under, wide",
            ((-3.0, 3.0), (-3.0, 3.0), (-1.0, 0.0)),
            0.0,
        ),
        (
            "stacked over, wide (a flush slab on the cap)",
            ((-3.0, 3.0), (-3.0, 3.0), (2.0, 3.0)),
            0.0,
        ),
        (
            "stacked under, the footprint",
            ((-1.0, 1.0), (-1.0, 1.0), (-1.0, 0.0)),
            0.0,
        ),
        (
            "stacked under, small",
            ((-0.5, 0.5), (-0.5, 0.5), (-1.0, 0.0)),
            0.0,
        ),
        (
            "around, sharing the bottom",
            ((-3.0, 3.0), (-3.0, 3.0), (0.0, 3.0)),
            9.0,
        ),
        (
            "around, sharing the top",
            ((-3.0, 3.0), (-3.0, 3.0), (-1.0, 2.0)),
            9.0,
        ),
        (
            "around, sharing both caps",
            ((-3.0, 3.0), (-3.0, 3.0), (0.0, 2.0)),
            9.0,
        ),
        (
            "face to face on the −y wall, small",
            ((-0.2, 0.2), (-2.0, -1.0), (0.8, 1.2)),
            0.0,
        ),
        (
            "face to face on the −y wall, wide",
            ((-3.0, 3.0), (-2.0, -1.0), (-1.0, 3.0)),
            0.0,
        ),
        (
            "a bar under the cap's corner strip",
            ((0.5, 3.0), (-3.0, 3.0), (-1.0, 0.0)),
            0.0,
        ),
        (
            "stacked under, off one edge",
            ((-0.5, 0.5), (-0.5, 3.0), (-1.0, 0.0)),
            0.0,
        ),
        (
            "inside, on the bottom cap",
            ((-0.5, 0.5), (-0.5, 0.5), (0.0, 0.5)),
            0.25,
        ),
        (
            "inside, on the −y wall",
            ((-0.2, 0.2), (-1.0, -0.5), (0.8, 1.2)),
            0.08,
        ),
        (
            "inside, in the bottom −y corner",
            ((-0.2, 0.2), (-1.0, -0.5), (0.0, 0.4)),
            0.08,
        ),
        (
            "inside, a hair off the −y wall",
            ((-0.2, 0.2), (-0.99, -0.5), (0.8, 1.2)),
            0.4 * 0.49 * 0.4,
        ),
    ];
    for (pn, (x, y, z), meet) in prism_placements {
        let (bk, vb) = brick(x, y, z);
        let bk = finished(pn, bk, t);
        lines.extend(case(
            &format!("loft prism × {pn}"),
            &prism,
            9.0,
            &bk,
            vb,
            Some(meet),
        ));
    }
    let vc = 2.0 * PI;
    let mut cylinder_placements: Vec<(String, topo::Body<f64>, f64, f64)> = Vec::new();
    for (pn, (x, y, z), meet) in [
        (
            "stacked under, wide",
            ((-2.0, 2.0), (-2.0, 2.0), (-1.0, 0.0)),
            0.0,
        ),
        (
            "around, sharing the bottom",
            ((-2.0, 2.0), (-2.0, 2.0), (0.0, 3.0)),
            vc,
        ),
        (
            "around, sharing both caps",
            ((-2.0, 2.0), (-2.0, 2.0), (0.0, 2.0)),
            vc,
        ),
        (
            "a brick touching the wall along a ruling",
            ((1.0, 2.0), (-0.5, 0.5), (0.5, 1.5)),
            0.0,
        ),
    ] {
        let (bk, vb) = brick(x, y, z);
        cylinder_placements.push((pn.to_string(), bk, vb, meet));
    }
    cylinder_placements.push((
        "a lofted cylinder stacked on its cap".into(),
        moved(lofted_cylinder(1.0, t), [0.0, 0.0, 2.0]),
        PI,
        0.0,
    ));
    cylinder_placements.push((
        "a lofted cylinder beside it, boxes overlapping".into(),
        moved(lofted_cylinder(2.0, t), [1.5, 1.5, 0.0]),
        vc,
        0.0,
    ));
    for (pn, pb, vb, meet) in cylinder_placements {
        let pb = finished(&pn, pb, t);
        lines.extend(case(
            &format!("lofted cylinder × {pn}"),
            &lcyl,
            vc,
            &pb,
            vb,
            Some(meet),
        ));
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

/// The brick `[−3, 3]² × [−1, 3]` around the loft prism, and the
/// prism.
fn prism_in_a_brick() -> (AtRestBody<f64>, AtRestBody<f64>) {
    let t = tol();
    let prism = finished("the loft prism", loft_prism(t), t);
    let (b, _) = brick((-3.0, 3.0), (-3.0, 3.0), (-1.0, 3.0));
    (finished("the brick around", b, t), prism)
}

/// The NURBS faces of `body`, each with its `sense`.
fn spline_walls(body: &topo::Body<f64>) -> Vec<(topo::FaceKey, bool)> {
    body.faces()
        .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(geom::Surface::Nurbs(_))))
        .map(|(k, f)| (k, f.sense))
        .collect()
}

/// **The void a ∖ leaves of a spline operand carries its NURBS walls
/// reverted** (adopted from PR 6476's review). `brick ∖ prism`, the brick
/// around the prism, builds: the brick with the prism as a void, whose
/// four NURBS walls the void door hands through `Body::revert`
/// (`boolean::voids`). Each must read `sense = false`. Nothing else at
/// the op level reads it: with `Body::revert` skipping the flip on NURBS
/// faces, the census and `spline_spiric_operands` stay green and the
/// volume still reads 135 m³ (the finding below).
#[test]
fn a_subtracted_spline_void_carries_its_walls_reverted() {
    let (around, prism) = prism_in_a_brick();
    let BooleanResult::Body(out) =
        topo::subtract(&around, &prism, tol()).expect("brick ∖ prism builds")
    else {
        panic!("brick ∖ prism is a body");
    };
    let walls = spline_walls(&out.body);
    assert_eq!(walls.len(), 4, "the void's four NURBS walls: {walls:?}");
    assert!(
        walls.iter().all(|&(_, sense)| !sense),
        "every wall of the void is reverted: {walls:?}"
    );
}

/// **FINDING: a cavity whose spline walls carry the wrong `sense` passes
/// the at-rest gate at the true volume** (adopted from PR 6476's
/// review; `work/verdict/m6-sense-gate-recorded-residuals.md`, residual 3).
/// `brick ∖ prism` builds a cavity with sense-false NURBS walls; flipping
/// each back — what a reversal that ignored `sense` on spline faces would
/// build — is admitted by `AtRestBody::validate`, and its volume reads
/// the same 135 m³. So neither the gate nor a volume oracle sees a wrong
/// `sense` on a spline face. Pinned as found: red the day the gate
/// refuses it.
#[test]
fn finding_a_cavity_with_its_spline_walls_unreversed_passes_the_gate_at_the_true_volume() {
    let t = tol();
    let (around, prism) = prism_in_a_brick();
    let Ok(BooleanResult::Body(cavity)) = topo::subtract(&around, &prism, t) else {
        panic!("brick ∖ prism builds");
    };
    let v_right = topo::test_support::volume_at("the cavity", &cavity.body, t, 1e-6).volume;
    let cavity: topo::Body<f64> = (*cavity.body).clone();
    let walls = spline_walls(&cavity);
    assert_eq!(walls.len(), 4, "the cavity's four NURBS walls");
    assert!(
        walls.iter().all(|&(_, sense)| !sense),
        "the cavity's walls are sense-false"
    );
    let mut wrong = cavity;
    for &(k, _) in &walls {
        wrong = wrong.flipped_face_sense_for_tests(k).unwrap();
    }
    let admitted = AtRestBody::validate(wrong, t).expect("the gate admits the wrong-sense cavity");
    let v_wrong =
        topo::test_support::volume_at("the wrong-sense cavity", &admitted, t, 1e-6).volume;
    assert!((v_right - 135.0).abs() < 1e-6, "the cavity: {v_right}");
    assert!(
        (v_wrong - 135.0).abs() < 1e-6,
        "the wrong-sense cavity reads the true volume: {v_wrong}"
    );
}
