//! **A crossings-path cut-in is made only where its closing is total.**
//! The output stage closes a sphere cut-in's cut
//! (`topo::boolean::slit`, closing a cut) on two premises: no join chord
//! runs along the cut, and where a cut's piece parts a face bounded by
//! section circles alone, the chart's pole lies beyond one of them. The
//! no-crossings path holds both by construction. On the crossings path
//! (PR 7440's interior-loop cut-in) the first is held by the cut's
//! meridian, chosen off every first-pass section on its face
//! (`interior_cuts::closing_meridians`); the second is read by the
//! closing itself, and where it reads no pole the re-entry answers
//! `CutOutlineUndecided`, the op goes on uncut, and the interior-loop
//! guard's refusal stands. It is never a `JoinDesync`.
//!
//! In each pose the ball's one face carries a first-pass crossing beside
//! the interior loop, each in all six ops at the run's ε:
//! - the die's slab cuts the `y`-poled ball in the circle `z = 0.7` and,
//!   by its bottom, the circle `z = 0` through both poles;
//! - a cap box beside a pole box: the cap box cuts the ball in the
//!   interior circle `z = 0.6`, and the pole box crosses its face in a
//!   plane `x = 0` that holds the circle's own meridian;
//! - a side box and a corner box: the side box cuts the ball in the
//!   interior circle `x = −0.9`, and the corner box crosses its face in a
//!   plane `z = 0` that holds that circle's own meridian;
//! - a cap box and a far box under the equator: the far box's plane
//!   `z = 0` holds both poles, its section far from the cut.
//!
//! `work/lathe/close-cuts-is-not-total-on-crossings-path-cut-ins.md`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::PI;

use geom_core::{Point2, Tol};
use sweep::Revolution;
use sweep::test_support::{brick, finished, revolved_about_y};
use topo::{AtRestBody, BooleanError, BooleanOp, BooleanResult};

fn tol() -> Tol {
    Tol::witness()
}

/// The unit ball revolved about `y`: one face, poles at `(0, ±1, 0)`.
fn ball() -> AtRestBody<f64> {
    let b = revolved_about_y(
        vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
        Revolution::Full,
        tol(),
    );
    finished("the ball", b, tol())
}

fn boxed(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    finished("a box", brick::<f64>(x, y, z, tol()), tol())
}

/// `p ∪ q`, at rest.
fn joined(p: &AtRestBody<f64>, q: &AtRestBody<f64>) -> AtRestBody<f64> {
    match topo::boolean::union(p, q, tol()) {
        Ok(BooleanResult::Body(b)) => finished("the union", b.body.into_body(), tol()),
        other => panic!("the box union builds: {:?}", other.map(|_| ())),
    }
}

/// The die's slab, `[−2, 2]² × [0, 0.7]`.
fn slab() -> AtRestBody<f64> {
    boxed((-2.0, 2.0), (-2.0, 2.0), (0.0, 0.7))
}

/// The box `z ≥ 0.6` over the ball's cap.
fn cap_box() -> AtRestBody<f64> {
    boxed((-2.0, 2.0), (-2.0, 2.0), (0.6, 2.0))
}

/// The box beside the `+y` pole, its face `x = 0` in the plane of the
/// cap box circle's own meridian.
fn pole_box() -> AtRestBody<f64> {
    boxed((-0.2, 0.0), (0.5, 2.0), (0.05, 0.25))
}

/// The cap box and the pole box: two disjoint boxes.
fn cap_beside_pole() -> AtRestBody<f64> {
    joined(&cap_box(), &pole_box())
}

/// The side box `x ≤ −0.9` and the corner box, whose face `z = 0` lies in
/// the plane of the side box circle's own meridian.
fn side_and_corner() -> AtRestBody<f64> {
    joined(
        &boxed((-2.0, -0.9), (-1.5, 1.5), (-1.5, 1.5)),
        &boxed((-2.0, -0.2), (-2.0, -0.7), (-2.0, 0.0)),
    )
}

/// The cap box and a box under the equator, whose face `z = 0` holds
/// both of the ball's poles, its section far from the cut.
fn cap_and_far() -> AtRestBody<f64> {
    joined(&cap_box(), &boxed((0.3, 0.5), (0.5, 2.0), (-0.1, 0.0)))
}

/// What a run yields.
#[derive(Debug)]
enum Got {
    /// A body: its volume, and tier 3′'s reading of it.
    Body(f64, Result<(), Vec<topo::ValidationError>>),
    /// The interior-loop guard's refusal.
    Guard,
}

const OPS: [BooleanOp; 3] = [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract];

/// `op` on `x` and `y`: a body passing tier 2 and the certificate, with
/// no cut traced into it ([`sweep::test_support::cut_trace`]), or the
/// interior-loop guard's refusal. Anything else fails the row.
fn run(label: &str, op: BooleanOp, x: &AtRestBody<f64>, y: &AtRestBody<f64>) -> Got {
    let r = match op {
        BooleanOp::Union => topo::boolean::union(x, y, tol()),
        BooleanOp::Intersect => topo::boolean::intersect(x, y, tol()),
        BooleanOp::Subtract => topo::boolean::subtract(x, y, tol()),
    };
    let got = match r {
        Ok(r @ BooleanResult::Body(_)) => {
            sweep::test_support::cut_trace::assert_no_cut_trace(label, &r, [x, y]);
            let BooleanResult::Body(out) = r else {
                unreachable!("matched a body")
            };
            assert_eq!(topo::validate_closed(&out.body), Ok(()), "{label}: tier 2");
            assert!(
                topo::validate_geometric_certificate(&out.body, tol()).is_ok(),
                "{label}: the certificate"
            );
            let v = topo::test_support::pinned_volume(label, &out.body, tol()).volume;
            Got::Body(
                v,
                topo::validate_pseudomanifold(&out.body, &out.contacts, tol()),
            )
        }
        Err(BooleanError::CurvedPairUnsupported {
            site: topo::PairRefusalSite::InteriorLoopGuard,
            ..
        }) => Got::Guard,
        other => panic!("{label}: a body or the interior-loop guard, not {other:?}"),
    };
    println!("[closing] {label}: {got:?}");
    got
}

/// Each op on the ball and `other`, ball first and ball second.
fn both_orders(name: &str, other: &AtRestBody<f64>) -> Vec<(BooleanOp, [Got; 2])> {
    let ball = ball();
    OPS.into_iter()
        .map(|op| {
            let label = |order: &str| format!("{name}, {op:?}, {order}, ε {}", tol().eps());
            (
                op,
                [
                    run(&label("ball first"), op, &ball, other),
                    run(&label("ball second"), op, other, &ball),
                ],
            )
        })
        .collect()
}

/// A sound body's volume: tier 3′ passes.
fn sound(label: &str, got: &Got) -> f64 {
    match got {
        Got::Body(v, Ok(())) => *v,
        other => panic!("{label}: a body passing tier 3′, not {other:?}"),
    }
}

/// Two volumes of one body, built in either order or composed of parts,
/// agree to `1e-12` of the larger.
fn same_volume(label: &str, v: f64, w: f64) {
    assert!(
        (v - w).abs() <= 1e-12 * v.abs().max(w.abs()),
        "{label}: {v} and {w}"
    );
}

/// **The six runs' volumes against the operands' own**, an oracle
/// independent of the op: `V(∪) + V(∩) = V(ball) + V(other)`,
/// `V(ball ∖ other) = V(ball) − V(∩)` and `V(other ∖ ball) = V(other) −
/// V(∩)`, each in either order, the operands' volumes pinned alone
/// (`topo::test_support::pinned_volume`). Each holds to `1e-9` of the
/// ball's volume.
fn inclusion_exclusion(name: &str, other: &AtRestBody<f64>, runs: &[(BooleanOp, [f64; 2])]) {
    let (ball, other) = (
        topo::test_support::pinned_volume("the ball", &ball(), tol()).volume,
        topo::test_support::pinned_volume(name, other, tol()).volume,
    );
    let of = |op: BooleanOp| {
        runs.iter()
            .find(|(o, _)| *o == op)
            .map(|&(_, v)| v)
            .expect("each op ran")
    };
    let (union, meet, less) = (
        of(BooleanOp::Union),
        of(BooleanOp::Intersect),
        of(BooleanOp::Subtract),
    );
    let close = |what: &str, v: f64, w: f64| {
        println!("[oracle] {name}, {what}: {v} against {w}, off {:e}", v - w);
        assert!((v - w).abs() <= 1e-9 * ball, "{name}, {what}: {v} against {w}");
    };
    for i in 0..2 {
        close("V(∪) + V(∩)", union[i] + meet[i], ball + other);
    }
    close("V(ball ∖ other)", less[0], ball - meet[0]);
    close("V(other ∖ ball)", less[1], other - meet[1]);
}

/// **Where a first-pass section on the cut face lies in the plane of the
/// circle's own meridian, the cut runs in another meridian, and the op
/// builds sound in either order.** The side box's circle `x = −0.9` has
/// its own meridian in the plane `z = 0`, where the corner box crosses
/// the ball's face; the cap box's circle `z = 0.6` has its own in `x = 0`,
/// where the pole box does. Each cut turns off that plane
/// (`interior_cuts::closing_meridians`), and every op but the cap box's
/// intersection (two parts,
/// [`the_cap_and_pole_boxes_meet_the_ball_in_two_parts_the_census_cannot_tell_apart`])
/// builds a body passing tier 3′, a union or an intersection at one
/// volume in either order. Before the turn, the join's chord along the
/// cut left a piece between faces on two surfaces (`JoinDesync`), so the
/// op did not cut in.
#[test]
fn a_cut_turns_off_a_first_pass_section_in_its_own_meridian_plane_and_builds() {
    for (name, other) in [
        ("a side box and a corner box", side_and_corner()),
        ("a cap box beside a pole box", cap_beside_pole()),
    ] {
        let mut runs = Vec::new();
        for (op, [ab, ba]) in both_orders(name, &other) {
            let label = format!("{name}, {op:?}");
            if name.starts_with("a cap box") && op == BooleanOp::Intersect {
                continue;
            }
            let (v, w) = (sound(&label, &ab), sound(&label, &ba));
            if op != BooleanOp::Subtract {
                same_volume(&label, v, w);
            }
            runs.push((op, [v, w]));
        }
        if name.starts_with("a side box") {
            inclusion_exclusion(name, &other, &runs);
        }
    }
}

/// **The cap box and the pole box meet the ball in two parts, which tier
/// 3′'s census cannot tell apart, and the body is right.** The boxes are
/// disjoint (`z ≥ 0.6` and `z ∈ [0.05, 0.25]`), so the ball's
/// intersection with their union is the ball's cap and a sliver beside
/// its pole. In either order it builds, passing tier 2 and the
/// certificate, at the volume of the ball's intersection with each box
/// alone, summed, each of which builds sound (the cap at its closed
/// form). Tier 3′ refuses only with the census's
/// `CensusUndecidable { MayOverlap }`: a curved face of one part within
/// reach of the other, and the pair it leaves unchecked
/// (`work/reachhold/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`).
/// Red when the census tells the two parts apart (flip it to sound), or
/// when either order refuses or builds another body. Before the cut
/// turned off the pole box's plane `x = 0`, the ball-first order desynced
/// at the closing and the other built this body.
#[test]
fn the_cap_and_pole_boxes_meet_the_ball_in_two_parts_the_census_cannot_tell_apart() {
    let ball = ball();
    let alone =
        |b: &AtRestBody<f64>, what: &str| sound(what, &run(what, BooleanOp::Intersect, &ball, b));
    // The ball's cap above `z = 0.6`, at its closed form `πh²(3 − h)/3`
    // with `h = 0.4`, and the sliver beside the pole, each built alone
    // and sound.
    let cap = alone(&cap_box(), "the ball ∩ the cap box");
    same_volume(
        "the ball's cap",
        cap,
        PI * 0.4f64.powi(2) * (3.0 - 0.4) / 3.0,
    );
    let parts = cap + alone(&pole_box(), "the ball ∩ the pole box");
    let other = cap_beside_pole();
    for (order, got) in [
        (
            "ball first",
            run("ball first", BooleanOp::Intersect, &ball, &other),
        ),
        (
            "ball second",
            run("ball second", BooleanOp::Intersect, &other, &ball),
        ),
    ] {
        let Got::Body(v, Err(findings)) = got else {
            panic!("{order}: a body tier 3′'s census leaves undecided, not {got:?}")
        };
        same_volume(&format!("{order}: the two parts"), v, parts);
        assert!(
            !findings.is_empty()
                && findings.iter().all(|f| matches!(
                    f,
                    topo::ValidationError::CensusUndecidable {
                        reason: topo::Undecidable::MayOverlap,
                        what,
                        ..
                    } if what.starts_with("a curved face of one is within reach of the other")
                        || what.starts_with("another finding left their boundaries unchecked")
                )),
            "{order}: only the census's two curved parts: {findings:?}"
        );
    }
}

/// **A section through both of the ball's poles, far from the cut, does
/// not stop it.** The far box's face `z = 0` holds both poles. The
/// closing reads the outline only where a cut's piece parts a face
/// bounded by section circles alone, and here it reads the pole beyond
/// the cap box's circle, so every op builds sound, a union or an
/// intersection at one volume in either order. A gate that read every
/// section circle on the cut face refused it.
#[test]
fn a_section_through_both_poles_far_from_the_cut_does_not_stop_it() {
    let name = "a cap box and a far box";
    let other = cap_and_far();
    let mut runs = Vec::new();
    for (op, [ab, ba]) in both_orders(name, &other) {
        let label = format!("{name}, {op:?}");
        let (v, w) = (sound(&label, &ab), sound(&label, &ba));
        if op != BooleanOp::Subtract {
            same_volume(&label, v, w);
        }
        runs.push((op, [v, w]));
    }
    inclusion_exclusion(name, &other, &runs);
}

/// **The die's slab builds where the closing reads a pole and refuses at
/// the guard where it reads none.** Its bottom cuts the ball's face in
/// the circle `z = 0` through both poles. Where the result keeps the band
/// between that circle and the top's circle `z = 0.7`, bounded by the two
/// alone (the intersection, and the slab less the ball), neither part of
/// the cut face's loop reads a pole beyond it; the closing answers
/// `CutOutlineUndecided`, the op goes on uncut, and the interior-loop
/// guard's refusal stands, in either order
/// (`work/lathe/a-section-circle-through-both-chart-poles-leaves-a-cut-face-with-no-pole-to-read.md`).
/// The union and the ball less the slab build sound at their closed
/// forms: `11.2 + 4π/3 − π(0.7 − 0.7³/3)` and `4π/3 − π(0.7 − 0.7³/3)`.
#[test]
fn the_die_slab_builds_where_its_closing_reads_a_pole_and_refuses_where_none() {
    let band = PI * (0.7 - 0.7f64.powi(3) / 3.0);
    let ball = 4.0 / 3.0 * PI;
    for (op, [ab, ba]) in both_orders("the die's slab", &slab()) {
        let label = format!("the die's slab, {op:?}");
        match op {
            BooleanOp::Union => {
                for got in [&ab, &ba] {
                    same_volume(&label, sound(&label, got), 11.2 + ball - band);
                }
            }
            BooleanOp::Intersect => {
                assert!(
                    matches!((&ab, &ba), (Got::Guard, Got::Guard)),
                    "{label}: the guard's refusal in either order: {ab:?} {ba:?}"
                );
            }
            BooleanOp::Subtract => {
                same_volume(&label, sound(&label, &ab), ball - band);
                assert!(
                    matches!(ba, Got::Guard),
                    "{label}, the slab less the ball: the guard's refusal: {ba:?}"
                );
            }
        }
    }
}

#[test]
fn zz_probe_pole_vertices() {
    for (name, x, y, op) in [
        ("slab ∪ ball", slab(), ball(), BooleanOp::Union),
        ("ball ∪ slab", ball(), slab(), BooleanOp::Union),
        ("ball ∖ slab", ball(), slab(), BooleanOp::Subtract),
    ] {
        let r = match op {
            BooleanOp::Union => topo::boolean::union(&x, &y, tol()),
            _ => topo::boolean::subtract(&x, &y, tol()),
        };
        let Ok(BooleanResult::Body(out)) = r else { panic!("{name}") };
        let pts: Vec<_> = out.body.vertex_points().map(|(_, p)| p).collect();
        let near = |q: [f64; 3]| pts.iter().filter(|p| ((p.x - q[0]).powi(2) + (p.y - q[1]).powi(2) + (p.z - q[2]).powi(2)).sqrt() < 1e-9).count();
        let on_circle = pts.iter().filter(|p| p.z.abs() < 1e-9 && ((p.x.powi(2) + p.y.powi(2)).sqrt() - 1.0).abs() < 1e-9).map(|p| format!("({:.3},{:.3})", p.x, p.y)).collect::<Vec<_>>();
        println!("[probe] {name}: {} vertices; at (0,1,0): {}; at (0,-1,0): {}; on z=0 unit circle: {:?}; tier3' {:?}", pts.len(), near([0.0, 1.0, 0.0]), near([0.0, -1.0, 0.0]), on_circle, topo::validate_pseudomanifold(&out.body, &out.contacts, tol()).map_err(|e| e.len()));
    }
}

fn zz_rod(x0: f64, y0: f64, r: f64, z0: f64, h: f64) -> AtRestBody<f64> {
    finished(
        "a rod",
        sweep::test_support::prism_at(
            vec![(Point2::new(x0 - r, y0), 1.0), (Point2::new(x0 + r, y0), 1.0)],
            z0,
            h,
            tol(),
        ),
        tol(),
    )
}

fn zz_report(name: &str, other: &AtRestBody<f64>) {
    let ball = ball();
    for op in OPS {
        for (order, x, y) in [("ball first", &ball, other), ("ball second", other, &ball)] {
            let r = match op {
                BooleanOp::Union => topo::boolean::union(x, y, tol()),
                BooleanOp::Intersect => topo::boolean::intersect(x, y, tol()),
                BooleanOp::Subtract => topo::boolean::subtract(x, y, tol()),
            };
            let got = match r {
                Ok(BooleanResult::Body(out)) => {
                    let t2 = topo::validate_closed(&out.body).is_ok();
                    let cert = topo::validate_geometric_certificate(&out.body, tol()).is_ok();
                    let v = topo::test_support::pinned_volume("probe", &out.body, tol()).volume;
                    let t3 = topo::validate_pseudomanifold(&out.body, &out.contacts, tol());
                    format!("BODY v={v} t2={t2} cert={cert} t3'={:?}", t3.map_err(|e| e.iter().map(|f| format!("{f:?}").chars().take(90).collect::<String>()).collect::<Vec<_>>()))
                }
                Ok(other) => format!("OTHER {:?}", std::mem::discriminant(&other)),
                Err(e) => format!("ERR {e:?}").chars().take(300).collect(),
            };
            println!("[probe] {name}, {op:?}, {order}: {got}");
        }
    }
}

#[test]
fn zz_probe_new_arms() {
    let rod_stub = zz_rod(0.5, 0.0, 0.2, 0.0, 2.0);
    zz_report("rod stub alone", &rod_stub);
    zz_report("corner box and rod stub", &joined(&boxed((-2.0, -0.2), (-2.0, -0.7), (-2.0, 0.0)), &rod_stub));
    zz_report("side, corner and rod stub", &joined(&side_and_corner(), &rod_stub));
    // A rod whose cap edge crosses the face beside the cap circle.
    zz_report("cap box and a crossing rod", &joined(&cap_box(), &zz_rod(0.0, -0.5, 0.15, -2.0, 1.2)));
    // A small ball trimmed by z ≥ 0, crossing the face beside the cap circle.
    let small = finished(
        "the small ball",
        sweep::test_support::ball_poled(0.3, geom_core::Vec3::new(0.9, 0.0, 0.0), geom_core::Vec3::new(0.0, 0.0, 1.0), tol()),
        tol(),
    );
    match topo::boolean::intersect(&small, &boxed((-2.0, 2.0), (-2.0, 2.0), (0.0, 2.0)), tol()) {
        Ok(BooleanResult::Body(out)) => {
            let dome = finished("the dome", out.body.into_body(), tol());
            zz_report("dome alone", &dome);
            zz_report("cap box and a dome", &joined(&cap_box(), &dome));
            zz_report("side box and a dome", &joined(&boxed((-2.0, -0.9), (-1.5, 1.5), (-1.5, 1.5)), &dome));
        }
        other => println!("[probe] the dome does not build: {:?}", other.map(|_| ())),
    }
}
