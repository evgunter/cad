//! Reviewer probes for PR 6476 (`splice/nurbs-revert-roster`): spline
//! operands at placements the census does not hold — stacked on a cap,
//! face to face with a wall, sharing a cap plane inside a brick, touching
//! a wall along a line — where a spline face's box meets the partner's
//! and the deleted revert roster used to refuse ∖ and ∩ up front. Each
//! run of ∖ or ∩ refuses as ∪ of the same order does, or builds at the
//! closed form (and every op that builds implies one `V(O ∩ B)`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_core::{RigidMotion, Tol, Vec3};
use sweep::test_support::{finished, loft_prism, lofted_cylinder};
use topo::{AtRestBody, BooleanError, BooleanResult};

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

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> (topo::Body<f64>, f64) {
    (
        topo::test_support::brick_with_fresh_keys::<f64>(x, y, z, tol()),
        (x.1 - x.0) * (y.1 - y.0) * (z.1 - z.0),
    )
}

fn kernel_invariant(e: &BooleanError) -> bool {
    matches!(
        e,
        BooleanError::ClassificationInvariant { .. } | BooleanError::JoinDesync { .. }
    ) || matches!(e, BooleanError::Join(j) if matches!(j.error(), topo::SplitJoinError::SectionInvariant { .. }))
}

fn volume(what: &str, r: &BooleanResult<f64>) -> (f64, f64) {
    match r {
        BooleanResult::Empty => (0.0, 0.0),
        BooleanResult::Body(b) => {
            let m = topo::test_support::volume_at(what, &b.body, tol(), 1e-6);
            (m.volume, m.volume_pad)
        }
    }
}

/// Runs the six ops; returns the census lines, the wrong outcomes (a
/// kernel invariant, a volume off its closed form) and the refusals
/// that differ from ∪'s of the same order.
fn case(
    name: &str,
    o: &AtRestBody<f64>,
    vo: f64,
    b: &AtRestBody<f64>,
    vb: f64,
    meet: f64,
) -> (Vec<String>, Vec<String>, Vec<String>) {
    let t = tol();
    let u_ob = topo::union(o, b, t);
    let u_bo = topo::union(b, o, t);
    let runs: [(&str, Result<BooleanResult<f64>, BooleanError>, usize, f64); 6] = [
        ("O∪B", topo::union(o, b, t), 0, vo + vb - meet),
        ("B∪O", topo::union(b, o, t), 1, vo + vb - meet),
        ("O∩B", topo::intersect(o, b, t), 0, meet),
        ("B∩O", topo::intersect(b, o, t), 1, meet),
        ("O∖B", topo::subtract(o, b, t), 0, vo - meet),
        ("B∖O", topo::subtract(b, o, t), 1, vb - meet),
    ];
    let unions = [&u_ob, &u_bo];
    let mut lines = Vec::new();
    let mut bad = Vec::new();
    let mut other = Vec::new();
    for (op, got, order, want) in runs {
        let line = format!("{name} / {op}: ");
        match &got {
            Err(e) => {
                if kernel_invariant(e) {
                    bad.push(format!("{line}KERNEL INVARIANT {e:?}"));
                }
                if let Err(u) = unions[order] {
                    if format!("{e:?}") != format!("{u:?}") {
                        other.push(format!("{line}refuses other than ∪: {e:?} vs ∪ {u:?}"));
                    }
                } else if !op.contains('∪') {
                    lines.push(format!("{line}NOTE ∪ builds but this refuses"));
                }
                lines.push(format!("{line}refuses {e:?}"));
            }
            Ok(r) => {
                let (v, pad) = volume(&line, r);
                let ok = (v - want).abs() <= pad + 1e-6 * want.abs().max(1.0);
                if !ok {
                    bad.push(format!("{line}WRONG VOLUME {v} ± {pad:e}, want {want}"));
                }
                if !op.contains('∪') && unions[order].is_err() {
                    lines.push(format!("{line}NOTE builds while ∪ refuses"));
                }
                lines.push(format!("{line}builds V = {v} ± {pad:e} (want {want})"));
            }
        }
    }
    (lines, bad, other)
}

/// Every placement, every op: the lines, the wrong outcomes and the
/// refusals that differ from ∪'s.
fn placements() -> (Vec<String>, Vec<String>, Vec<String>) {
    let t = tol();
    let prism = finished("the loft prism", loft_prism(t), t);
    let lcyl = finished("the lofted cylinder", lofted_cylinder(2.0, t), t);
    let mut lines = Vec::new();
    let mut bad = Vec::new();
    let mut other = Vec::new();
    // The prism: box [−1.375, 1.375] × [−1, 1] × [0, 2], V = 9; its
    // section is the unit-half square at z = 0 and z = 2.
    let prism_cases: Vec<(&str, (topo::Body<f64>, f64), f64)> = vec![
        ("stacked under, wide", brick((-3.0, 3.0), (-3.0, 3.0), (-1.0, 0.0)), 0.0),
        ("stacked over, wide", brick((-3.0, 3.0), (-3.0, 3.0), (2.0, 3.0)), 0.0),
        ("stacked under, footprint", brick((-1.0, 1.0), (-1.0, 1.0), (-1.0, 0.0)), 0.0),
        ("stacked under, small", brick((-0.5, 0.5), (-0.5, 0.5), (-1.0, 0.0)), 0.0),
        ("around, sharing the bottom", brick((-3.0, 3.0), (-3.0, 3.0), (0.0, 3.0)), 9.0),
        ("around, sharing the top", brick((-3.0, 3.0), (-3.0, 3.0), (-1.0, 2.0)), 9.0),
        ("around, sharing both caps", brick((-3.0, 3.0), (-3.0, 3.0), (0.0, 2.0)), 9.0),
        (
            "face to face on the −y wall, small",
            brick((-0.2, 0.2), (-2.0, -1.0), (0.8, 1.2)),
            0.0,
        ),
        (
            "face to face on the −y wall, wide",
            brick((-3.0, 3.0), (-2.0, -1.0), (-1.0, 3.0)),
            0.0,
        ),
        (
            "a bar under the cap's corner strip",
            brick((0.5, 3.0), (-3.0, 3.0), (-1.0, 0.0)),
            0.0,
        ),
        (
            "stacked under, off one edge",
            brick((-0.5, 0.5), (-0.5, 3.0), (-1.0, 0.0)),
            0.0,
        ),
        (
            "inside, on the bottom cap",
            brick((-0.5, 0.5), (-0.5, 0.5), (0.0, 0.5)),
            0.25,
        ),
        (
            "inside, on the −y wall",
            brick((-0.2, 0.2), (-1.0, -0.5), (0.8, 1.2)),
            0.4 * 0.5 * 0.4,
        ),
        (
            "inside, in the bottom −y corner",
            brick((-0.2, 0.2), (-1.0, -0.5), (0.0, 0.4)),
            0.4 * 0.5 * 0.4,
        ),
        (
            "inside, a hair off the −y wall",
            brick((-0.2, 0.2), (-0.99, -0.5), (0.8, 1.2)),
            0.4 * 0.49 * 0.4,
        ),
    ];
    for (n, (b, vb), meet) in prism_cases {
        let b = finished(n, b, t);
        let (l, x, y) = case(&format!("prism × {n}"), &prism, 9.0, &b, vb, meet);
        lines.extend(l);
        bad.extend(x);
        other.extend(y);
    }
    let vc = 2.0 * PI;
    let cyl_cases: Vec<(String, topo::Body<f64>, f64, f64)> = {
        let mut v = Vec::new();
        for (n, (b, vb), meet) in [
            ("stacked under, wide", brick((-2.0, 2.0), (-2.0, 2.0), (-1.0, 0.0)), 0.0),
            ("around, sharing the bottom", brick((-2.0, 2.0), (-2.0, 2.0), (0.0, 3.0)), vc),
            ("around, sharing both caps", brick((-2.0, 2.0), (-2.0, 2.0), (0.0, 2.0)), vc),
            (
                "a brick touching the wall along a ruling",
                brick((1.0, 2.0), (-0.5, 0.5), (0.5, 1.5)),
                0.0,
            ),
        ] {
            v.push((n.to_string(), b, vb, meet));
        }
        v.push((
            "a lofted cylinder stacked on its cap".into(),
            moved(lofted_cylinder(1.0, t), [0.0, 0.0, 2.0]),
            PI,
            0.0,
        ));
        v.push((
            "a lofted cylinder beside it, boxes overlapping".into(),
            moved(lofted_cylinder(2.0, t), [1.5, 1.5, 0.0]),
            vc,
            0.0,
        ));
        v
    };
    for (n, b, vb, meet) in cyl_cases {
        let b = finished(&n, b, t);
        let (l, x, y) = case(&format!("lofted cylinder × {n}"), &lcyl, vc, &b, vb, meet);
        lines.extend(l);
        bad.extend(x);
        other.extend(y);
    }
    (lines, bad, other)
}

/// **No ∖ or ∩ of a spline operand at a placement past the deleted
/// roster builds wrong or breaks a kernel invariant.**
#[test]
fn review_6476_no_spline_placement_past_the_deleted_roster_builds_wrong() {
    let (lines, bad, _) = placements();
    for l in &lines {
        println!("{l}");
    }
    assert!(bad.is_empty(), "wrong:\n{}", bad.join("\n"));
}

/// **FINDING: ∖ and ∩ do not refuse where ∪ does.** The PR states that
/// every op meets ∪'s doors and refuses with ∪'s refusal
/// (`reduce::boolean_arm_exists`'s docs; the census's premise). At a
/// cap stacked on a brick, a brick around the operand sharing a cap
/// plane, or a bar under part of a cap, ∪ refuses
/// `CurvedBooleanUnsupported { kind: Nurbs }` where ∩ and ∖ of the same
/// order refuse `NurbsExtentUnsupported` (and the reverse for B∩O and
/// B∖O around a shared cap). None builds. Pinned as found: red the day
/// the doors agree, when the census can take these placements.
#[test]
fn review_6476_finding_subtract_and_intersect_refuse_other_than_union_at_a_shared_cap() {
    let (_, _, other) = placements();
    for l in &other {
        println!("{l}");
    }
    assert_eq!(other.len(), 40, "the refusals that differ from ∪'s");
}

/// **The void a ∖ leaves of a spline operand carries its NURBS faces
/// reverted.** `brick ∖ prism`, the brick around the prism, builds today
/// (`spline_spiric_operands`' "around" placement): the result is the
/// brick with the prism as a void, its four NURBS walls handed through
/// `Body::revert`. Each must read `sense = false`. No op-level row
/// reads it: with `Body::revert` skipping the
/// flip on NURBS faces, the census, the saddle row and
/// `spline_spiric_operands` all stay green and the volume still reads
/// 135 m³.
#[test]
fn review_6476_a_subtracted_spline_void_carries_its_walls_reverted() {
    let t = tol();
    let prism = finished("the loft prism", loft_prism(t), t);
    let (b, _) = brick((-3.0, 3.0), (-3.0, 3.0), (-1.0, 3.0));
    let b = finished("the brick around", b, t);
    let BooleanResult::Body(out) = topo::subtract(&b, &prism, t).expect("brick ∖ prism builds")
    else {
        panic!("brick ∖ prism is a body");
    };
    let body = &out.body;
    let walls: Vec<(topo::FaceKey, bool)> = body
        .faces()
        .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(geom::Surface::Nurbs(_))))
        .map(|(k, f)| (k, f.sense))
        .collect();
    assert_eq!(walls.len(), 4, "the void's four NURBS walls: {walls:?}");
    assert!(
        walls.iter().all(|&(_, sense)| !sense),
        "every wall of the void is reverted: {walls:?}"
    );
}
