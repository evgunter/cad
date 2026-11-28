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
//!   `z = 0` holds both poles, its section far from the cut;
//! - a cap box and nine thin boxes, one in each plane of the meridian
//!   ladder or a hair off it;
//! - a cap box and a rod (a cylinder) crossing the face, or a dome (a
//!   trimmed ball) read by its radical plane;
//! - a rod stub's loop beside the corner box, alone and with the side
//!   box's circle on the same face.
//!
//! Where a row reads volumes in both orders against the operands' own,
//! it is inclusion–exclusion: `V(∪) + V(∩) = V(ball) + V(other)`, and
//! each difference the operand less the intersection.
//!
//! `work/lathe/close-cuts-is-not-total-on-crossings-path-cut-ins.md`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
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

/// A rod along `z`: the disc of radius `r` about `(x0, y0)` from `z0` up
/// by `h`.
fn rod(x0: f64, y0: f64, r: f64, (z0, h): (f64, f64)) -> AtRestBody<f64> {
    finished(
        "a rod",
        sweep::test_support::prism_at(
            vec![
                (Point2::new(x0 - r, y0), 1.0),
                (Point2::new(x0 + r, y0), 1.0),
            ],
            z0,
            h,
            tol(),
        ),
        tol(),
    )
}

/// A box turned about the ball's axis `y` by `phi`.
fn turned(x: (f64, f64), y: (f64, f64), z: (f64, f64), phi: f64) -> AtRestBody<f64> {
    let m = geom_core::test_support::rigid(
        &Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), phi),
        tol(),
    );
    finished(
        "a turned box",
        topo::transform_rigid(&brick::<f64>(x, y, z, tol()), &m).unwrap(),
        tol(),
    )
}

/// The cap box with a thin box in each of the meridian ladder's nine
/// planes, a hair `delta` (radians) off it: each is the plane `x = 0`
/// turned by `atan(t) + delta` about the axis, beside the cap circle.
/// The ladder is `interior_cuts::meridian_turns`' `t`.
fn cap_and_turned_boxes(delta: f64) -> AtRestBody<f64> {
    [0.0, 0.125, -0.125, 0.25, -0.25, 0.375, -0.375, 0.5, -0.5]
        .into_iter()
        .fold(cap_box(), |other, t: f64| {
            joined(
                &other,
                &turned((-0.01, 0.0), (0.5, 2.0), (0.15, 0.25), t.atan() + delta),
            )
        })
}

/// A ball of radius `0.3` about `(0.9, 0, 0)` above `z = 0`: its sphere
/// face, trimmed by the plane, crosses the unit ball's face.
fn dome() -> AtRestBody<f64> {
    let small = finished(
        "the small ball",
        sweep::test_support::ball_poled(
            0.3,
            Vec3::new(0.9, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            tol(),
        ),
        tol(),
    );
    match topo::boolean::intersect(&small, &boxed((-2.0, 2.0), (-2.0, 2.0), (0.0, 2.0)), tol()) {
        Ok(BooleanResult::Body(b)) => finished("the dome", b.body.into_body(), tol()),
        other => panic!("the dome builds: {:?}", other.map(|_| ())),
    }
}

/// A rod about `(0.5, 0)` from `z = 0` up, its cap inside the ball: its
/// face meets the ball's in one loop.
fn rod_stub() -> AtRestBody<f64> {
    rod(0.5, 0.0, 0.2, (0.0, 2.0))
}

/// The corner box: it crosses the ball's face, a first-pass section.
fn corner_box() -> AtRestBody<f64> {
    boxed((-2.0, -0.2), (-2.0, -0.7), (-2.0, 0.0))
}

/// What a run yields.
#[derive(Debug)]
enum Got {
    /// A body: its volume, and tier 3′'s reading of it.
    Body(f64, Result<(), Vec<topo::ValidationError>>),
    /// The interior-loop guard's refusal.
    Guard,
    /// An escalation: a reading in band.
    Escalated,
}

const OPS: [BooleanOp; 3] = [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract];

/// `op` on `x` and `y`: a body passing tier 2 and the certificate, with
/// no cut traced into it ([`sweep::test_support::cut_trace`]), the
/// interior-loop guard's refusal, or an escalation. Anything else fails
/// the row.
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
        Err(BooleanError::Escalated { .. }) => Got::Escalated,
        other => panic!("{label}: a body, the interior-loop guard or an escalation, not {other:?}"),
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

/// A body's volume where tier 3′ passes, or refuses only with the
/// census's undecidable pair of curved parts
/// (`work/reachhold/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`).
fn built(label: &str, got: &Got) -> f64 {
    match got {
        Got::Body(v, Ok(())) => *v,
        Got::Body(v, Err(findings))
            if findings.iter().all(|f| {
                matches!(
                    f,
                    topo::ValidationError::CensusUndecidable {
                        reason: topo::Undecidable::MayOverlap,
                        ..
                    }
                )
            }) =>
        {
            *v
        }
        other => panic!("{label}: a body passing tier 3′ or its census alone, not {other:?}"),
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
/// ball's volume, or where `marched` (a face carried on a marched
/// section), to the resolution the measurement meets: `1024ε` of mean
/// boundary displacement over the operands' area
/// (`topo::test_support::pinned_len`).
fn inclusion_exclusion(
    name: &str,
    other: &AtRestBody<f64>,
    runs: &[(BooleanOp, [f64; 2])],
    marched: bool,
) {
    let (ball, other) = (
        topo::test_support::pinned_volume("the ball", &ball(), tol()),
        topo::test_support::pinned_volume(name, other, tol()),
    );
    let bound = if marched {
        topo::test_support::pinned_len(tol()) * (ball.surface_area + other.surface_area)
    } else {
        1e-9 * ball.volume
    };
    let (ball, other) = (ball.volume, other.volume);
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
        assert!(
            (v - w).abs() <= bound,
            "{name}, {what}: {v} against {w}, past {bound:e}"
        );
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
            inclusion_exclusion(name, &other, &runs, false);
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
    inclusion_exclusion(name, &other, &runs, false);
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
    let whole = 4.0 / 3.0 * PI;
    for (op, [ab, ba]) in both_orders("the die's slab", &slab()) {
        let label = format!("the die's slab, {op:?}");
        match op {
            BooleanOp::Union => {
                for got in [&ab, &ba] {
                    same_volume(&label, sound(&label, got), 11.2 + whole - band);
                }
            }
            BooleanOp::Intersect => {
                assert!(
                    matches!((&ab, &ba), (Got::Guard, Got::Guard)),
                    "{label}: the guard's refusal in either order: {ab:?} {ba:?}"
                );
            }
            BooleanOp::Subtract => {
                same_volume(&label, sound(&label, &ab), whole - band);
                assert!(
                    matches!(ba, Got::Guard),
                    "{label}, the slab less the ball: the guard's refusal: {ba:?}"
                );
            }
        }
    }
    // The built bodies hold the circle `z = 0` through both poles, and
    // carry each pole as a vertex (DESIGN `[topology]`: a chart
    // singularity is "a vertex of every face whose closure holds it"),
    // the form tier 3′'s chart-singularity arm reads.
    for (label, r) in [
        (
            "the slab ∪ the ball",
            topo::boolean::union(&slab(), &ball(), tol()),
        ),
        (
            "the ball ∪ the slab",
            topo::boolean::union(&ball(), &slab(), tol()),
        ),
        (
            "the ball ∖ the slab",
            topo::boolean::subtract(&ball(), &slab(), tol()),
        ),
    ] {
        let Ok(BooleanResult::Body(out)) = r else {
            panic!("{label} builds")
        };
        for pole in [1.0, -1.0] {
            let at = out
                .body
                .vertex_points()
                .filter(|(_, p)| (p.x.powi(2) + (p.y - pole).powi(2) + p.z.powi(2)).sqrt() < 1e-9)
                .count();
            assert_eq!(at, 1, "{label}: one vertex at the pole (0, {pole}, 0)");
        }
    }
}

/// **A first-pass section on the cut face that is no plane or sphere
/// stops the cut-in, and the guard's refusal stands.** R1's rod along `z`
/// about `(0, 0.9)`, from `z = 0.05` to `0.25`, crosses the ball's face
/// beside the cap box's circle `z = 0.6`. A cylinder's section has no
/// plane to turn the cut's meridian off (`interior_cuts::closing_meridians`),
/// so the op does not cut in, in every op and either order.
#[test]
fn a_rod_crossing_the_cut_face_is_no_section_a_meridian_turns_off() {
    let name = "a cap box and a rod";
    for (op, [ab, ba]) in both_orders(name, &joined(&cap_box(), &rod(0.0, 0.9, 0.12, (0.05, 0.2))))
    {
        assert!(
            matches!((&ab, &ba), (Got::Guard, Got::Guard)),
            "{name}, {op:?}: the guard's refusal in either order: {ab:?} {ba:?}"
        );
    }
}

/// **A trimmed ball crossing the cut face is read by its radical plane,
/// and the op builds.** The dome (a ball of radius `0.3` about
/// `(0.9, 0, 0)`, above `z = 0`) crosses the ball's face beside the cap
/// box's circle `z = 0.6`. Its sphere face is a first-pass section read
/// as the two spheres' radical plane `x ≈ 0.956`, off the cut's meridian
/// `x = 0`, and every op builds sound, a union or an intersection at one
/// volume in either order, and every volume as the operands' own
/// (inclusion–exclusion).
#[test]
fn a_trimmed_ball_crossing_the_cut_face_is_read_by_its_radical_plane() {
    let name = "a cap box and a dome";
    let other = joined(&cap_box(), &dome());
    let mut runs = Vec::new();
    for (op, [ab, ba]) in both_orders(name, &other) {
        let label = format!("{name}, {op:?}");
        let (v, w) = (sound(&label, &ab), sound(&label, &ba));
        if op != BooleanOp::Subtract {
            same_volume(&label, v, w);
        }
        runs.push((op, [v, w]));
    }
    inclusion_exclusion(name, &other, &runs, false);
}

/// Whether a row on the rod stub's marched loop runs at this ε: from
/// `1e-10` up. Finer, a ball's boolean with a cylinder marches for tens of
/// minutes (`work/sect/a-ball-and-cylinder-boolean-costs-seconds-per-op-in-an-optimized-build.md`:
/// the loop and the circle took 3 330 s to reach the guard at `1e-12`),
/// and its volume does not meet [`topo::test_support::pinned_len`]'s
/// resolution.
fn marched_runs() -> bool {
    let runs = tol().eps() >= 1e-10;
    if !runs {
        println!(
            "[closing] the rod stub's rows run from ε 1e-10 up, not at {:e}",
            tol().eps()
        );
    }
    runs
}

/// **A loop's cut beside a first-pass section builds.** The rod stub
/// about `(0.5, 0)` meets the ball's face in one loop, which the cut-in
/// cuts along its own meridian; the corner box crosses the face away from
/// it. Every op builds sound, a union or an intersection at one volume in
/// either order, and every volume as the operands' own, to the
/// resolution its measurement meets. From ε `1e-10` up ([`marched_runs`]).
#[test]
fn a_loops_cut_beside_a_first_pass_section_builds() {
    if !marched_runs() {
        return;
    }
    let name = "a corner box and a rod stub";
    let other = joined(&corner_box(), &rod_stub());
    let mut runs = Vec::new();
    for (op, [ab, ba]) in both_orders(name, &other) {
        let label = format!("{name}, {op:?}");
        let (v, w) = (sound(&label, &ab), sound(&label, &ba));
        if op != BooleanOp::Subtract {
            same_volume(&label, v, w);
        }
        runs.push((op, [v, w]));
    }
    inclusion_exclusion(name, &other, &runs, true);
}

/// **A loop's cut-in and a circle's on one face, beside a first-pass
/// section, refuse at the guard.** The side box's circle `x = −0.9` and
/// the rod stub's loop both lie on the ball's one face, and the corner
/// box crosses it. The gate this unit replaced refused any face holding a
/// loop's cut-in and another; it no longer does, and the pose is not
/// built: the guard's refusal stands, in every op and either order.
/// Red when it builds (then pin it sound), or when it refuses otherwise.
/// From ε `1e-10` up ([`marched_runs`]).
#[test]
fn a_loop_and_a_circle_cut_in_on_one_face_refuse_at_the_guard() {
    if !marched_runs() {
        return;
    }
    let name = "a side box, a corner box and a rod stub";
    for (op, [ab, ba]) in both_orders(name, &joined(&side_and_corner(), &rod_stub())) {
        assert!(
            matches!((&ab, &ba), (Got::Guard, Got::Guard)),
            "{name}, {op:?}: the guard's refusal in either order: {ab:?} {ba:?}"
        );
    }
}

/// **A box in every turn of the meridian ladder: on the planes the
/// guard's refusal stands, a hair off them in band the op escalates, and
/// clear of them it builds.** R1's nine thin boxes, each in a plane of
/// the ladder turned by `delta`, beside the cap box's circle `z = 0.6`,
/// whose own meridian is the ladder's first plane:
/// - `delta = 0`: every turn is decidedly blocked, and the op goes on
///   uncut, to the guard's refusal;
/// - `delta = 10ε`: each turn's offset is read in band (about `2.5ε`),
///   none clears, and the op escalates as that reading
///   (`interior_cuts::closing_meridians`);
/// - `delta = 100ε`: the first turn clears, the op builds, and every
///   volume is the operands' own (inclusion–exclusion), a body tier 3′
///   passes or reads only as the census's undecidable pair of parts.
///
/// In every op and either order.
#[test]
fn a_box_in_every_turn_blocks_escalates_or_builds_as_its_offset_reads() {
    let eps = tol().eps();
    for (delta, want) in [(0.0, "guard"), (10.0 * eps, "escalated")] {
        let name = format!("nine turned boxes, δ {delta:e}");
        for (op, [ab, ba]) in both_orders(&name, &cap_and_turned_boxes(delta)) {
            let got = match (&ab, &ba) {
                (Got::Guard, Got::Guard) => "guard",
                (Got::Escalated, Got::Escalated) => "escalated",
                _ => "other",
            };
            assert_eq!(got, want, "{name}, {op:?}: {ab:?} {ba:?}");
        }
    }
    let name = format!("nine turned boxes, δ {:e}", 100.0 * eps);
    let other = cap_and_turned_boxes(100.0 * eps);
    let mut runs = Vec::new();
    for (op, [ab, ba]) in both_orders(&name, &other) {
        let label = format!("{name}, {op:?}");
        let (v, w) = (built(&label, &ab), built(&label, &ba));
        if op != BooleanOp::Subtract {
            same_volume(&label, v, w);
        }
        runs.push((op, [v, w]));
    }
    inclusion_exclusion(&name, &other, &runs, false);
}
