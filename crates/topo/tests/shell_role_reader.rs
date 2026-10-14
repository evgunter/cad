//! **A shell's role is the side at infinity of its faces** (D10,
//! Booleans), read by a ray aimed at one of them from beyond the shell,
//! and a solid's orientation (D1 tier 3, check 7) by the same reader. A
//! slab `t` thick whose two faces are a definite pair (`t > Kε`) reads
//! its role and orientation off that pair, however thin its volume over
//! its area: `brick ∖ slab` holds the slab as a cavity, `slab ∪ brick`
//! far apart is two solids, a lone slab classifies `Outer` by arity, and
//! an inside-out slab refuses at its own door. Below `Kε` each refuses on
//! its pair.
//!
//! The thicknesses are in ε and the poses scale with it; the rows were
//! measured at ε 1e-9 and 1e-6, and `Kε` itself is a knife-edge for
//! every instrument, so the pins sit at 10.5ε, never at `Kε`.

#![allow(clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol};
use topo::{
    AtRestBody, Body, BooleanDecision, BooleanError, BooleanResult, Coincide, ShellRole,
    ValidationError, classify_shells,
};

use crate::common;

/// The definite thicknesses, in ε.
const ABOVE_K: [f64; 4] = [10.5, 12.0, 15.0, 19.0];

/// A thickness just under `Kε`.
const BELOW_K: f64 = 9.9;

fn stood_down(tol: Tol) -> bool {
    if ![1e-9, 1e-6]
        .iter()
        .any(|e| (tol.eps() - e).abs() < 1e-3 * e)
    {
        test_utils::vacuity::stood_down(
            "eps other than 1e-9 or 1e-6",
            "the slab rows were measured at 1e-9 and 1e-6",
        );
        return true;
    }
    false
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64), tol: Tol) -> AtRestBody<f64> {
    AtRestBody::validate(common::brick::<f64>(x, y, z, tol), tol).expect("the brick is at rest")
}

fn slab(t: f64, tol: Tol) -> Body<f64> {
    common::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, t), tol)
}

fn built(what: &str, result: Result<BooleanResult<f64>, BooleanError>) -> AtRestBody<f64> {
    let result = result.unwrap_or_else(|e| panic!("{what}: builds, got {e:?}"));
    result
        .body()
        .unwrap_or_else(|| panic!("{what}: is not empty"))
        .body
        .clone()
}

fn shells_per_solid(body: &Body<f64>) -> Vec<usize> {
    body.solids().map(|(_, s)| s.shells.len()).collect()
}

fn roles(what: &str, body: &Body<f64>, tol: Tol) -> Vec<ShellRole> {
    classify_shells(body, tol)
        .unwrap_or_else(|e| panic!("{what}: every shell classifies, got {e:?}"))
        .iter()
        .map(|c| c.role)
        .collect()
}

/// `refusal` is the near-coincidence of a pair of cells `t` apart.
fn assert_refuses_on_a_pair(what: &str, t: f64, refusal: Result<BooleanResult<f64>, BooleanError>) {
    let Err(BooleanError::Escalated { decision, diag }) = &refusal.map(|_| ()) else {
        panic!("{what}: refuses Escalated");
    };
    assert!(
        matches!(
            decision,
            BooleanDecision::Coincidence(Coincide::VertexOnFace, _)
        ),
        "{what}: the refusal's subject is a vertex and a face: {decision:?}"
    );
    let gap = diag.margin.diagnostic_f64_for_error_text().value();
    assert!(
        gap.is_some_and(|g| (g.abs() - t).abs() < 1e-3 * t),
        "{what}: the pair is the slab's two sides, {t:e} apart: {diag:?}"
    );
}

/// **A thin cavity is a void**: `brick (−1,2)³ ∖ slab(t)` builds one
/// solid of two shells, the slab's a `Void`, at every definite `t`.
#[test]
fn a_brick_less_a_thin_slab_holds_it_as_a_cavity() {
    let tol = Tol::witness();
    if stood_down(tol) {
        return;
    }
    let big = brick((-1.0, 2.0), (-1.0, 2.0), (-1.0, 2.0), tol);
    for m in ABOVE_K {
        let what = format!("brick ∖ slab {m}ε");
        let s = AtRestBody::validate(slab(m * tol.eps(), tol), tol).expect("the slab is at rest");
        let body = built(&what, topo::subtract(&big, &s, tol));
        assert_eq!(shells_per_solid(&body), [2], "{what}: one solid");
        assert_eq!(
            roles(&what, &body, tol),
            [ShellRole::Outer, ShellRole::Void],
            "{what}: the cavity is a void"
        );
    }
}

/// **A thin cavity cut rather than placed**: the brick less a unit cube,
/// then ∪ a block whose floor stands `t` above the cube's, leaves a
/// cavity `t` thick. It builds at every definite `t`, and below `Kε` it
/// refuses on the cavity's floor and the block's.
#[test]
fn a_thin_cavity_cut_by_a_union_builds_above_k_eps_and_refuses_on_its_pair_below() {
    let tol = Tol::witness();
    if stood_down(tol) {
        return;
    }
    let big = brick((-1.0, 2.0), (-1.0, 2.0), (-1.0, 2.0), tol);
    let core = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let hollow = built("the hollow brick", topo::subtract(&big, &core, tol));
    let floor = |t: f64| brick((-0.5, 1.5), (-0.5, 1.5), (t, 1.5), tol);
    for m in ABOVE_K {
        let what = format!("cut cavity {m}ε");
        let body = built(&what, topo::union(&hollow, &floor(m * tol.eps()), tol));
        assert_eq!(shells_per_solid(&body), [2], "{what}: one solid");
        assert_eq!(
            roles(&what, &body, tol),
            [ShellRole::Outer, ShellRole::Void],
            "{what}: the cavity is a void"
        );
    }
    let t = BELOW_K * tol.eps();
    assert_refuses_on_a_pair(
        &format!("cut cavity {BELOW_K}ε"),
        t,
        topo::union(&hollow, &floor(t), tol),
    );
}

/// **A thin slab and a brick far apart are two solids**, in either order
/// of the union, and the slab cut out of a unit brick beside the far one
/// is too; below `Kε` the cut refuses on the slab's pair.
#[test]
fn a_thin_slab_and_a_far_brick_are_two_solids() {
    let tol = Tol::witness();
    if stood_down(tol) {
        return;
    }
    let far = brick((5.0, 6.0), (5.0, 6.0), (5.0, 6.0), tol);
    let unit = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let cutter = |t: f64| brick((-1.0, 2.0), (-1.0, 2.0), (t, 2.0), tol);
    for (order, unit_first) in [("unit first", true), ("far first", false)] {
        let pair = if unit_first {
            topo::union(&unit, &far, tol)
        } else {
            topo::union(&far, &unit, tol)
        };
        let pair = built(order, pair);
        for m in ABOVE_K {
            let what = format!("({order}) ∖ cutter, {m}ε");
            let body = built(&what, topo::subtract(&pair, &cutter(m * tol.eps()), tol));
            assert_eq!(shells_per_solid(&body), [1, 1], "{what}: two solids");
        }
        let t = BELOW_K * tol.eps();
        assert_refuses_on_a_pair(
            &format!("({order}) ∖ cutter, {BELOW_K}ε"),
            t,
            topo::subtract(&pair, &cutter(t), tol),
        );
    }
    for m in ABOVE_K {
        let s = AtRestBody::validate(slab(m * tol.eps(), tol), tol).expect("the slab is at rest");
        for (order, a, b) in [("slab ∪ far", &s, &far), ("far ∪ slab", &far, &s)] {
            let what = format!("{order} {m}ε");
            let body = built(&what, topo::union(a, b, tol));
            assert_eq!(shells_per_solid(&body), [1, 1], "{what}: two solids");
            assert_eq!(
                roles(&what, &body, tol),
                [ShellRole::Outer, ShellRole::Outer],
                "{what}"
            );
        }
    }
}

/// **A lone slab classifies `Outer`, and inside out it refuses at its own
/// door.** A solid's only shell is its outer shell by arity; its
/// orientation is check 7's, read by a ray aimed at one of its faces,
/// which crosses into the inside-out slab's material first.
#[test]
fn a_lone_slab_is_outer_and_inside_out_refuses_at_rest() {
    let tol = Tol::witness();
    if stood_down(tol) {
        return;
    }
    for m in ABOVE_K {
        let body = slab(m * tol.eps(), tol);
        assert_eq!(
            roles("lone slab", &body, tol),
            [ShellRole::Outer],
            "{m}ε: a lone slab"
        );
        let inverted = body.revert();
        let solid = inverted.solids().next().expect("one solid").0;
        assert_eq!(
            AtRestBody::validate(inverted, tol).map(|_| ()),
            Err(vec![ValidationError::NegativeVolume { solid }]),
            "{m}ε: the inside-out slab refuses at rest"
        );
    }
}

/// **A slab whose big faces lie in the plane `x = y`** reads the same: its
/// roles beside a far brick and as a cavity, and inside out it refuses.
/// A ray from a point beyond the slab's box along its diagonal would lie
/// in the slab's plane; a ray aimed at 60° to the face it aims at cannot
/// graze it.
#[test]
fn a_slab_in_the_plane_x_equals_y_reads_its_role() {
    let tol = Tol::witness();
    if stood_down(tol) {
        return;
    }
    let r = std::f64::consts::FRAC_1_SQRT_2;
    let big = brick((-1.0, 2.0), (-1.0, 2.0), (-1.0, 2.0), tol);
    let far = brick((5.0, 6.0), (5.0, 6.0), (5.0, 6.0), tol);
    for m in ABOVE_K {
        let t = m * tol.eps();
        let raw = common::mapped_cube::<f64>(
            move |x, y, z| Point3::new(0.2 + r * x + r * t * z, 0.2 + r * x - r * t * z, y),
            tol,
        );
        let inverted = raw.revert();
        let solid = inverted.solids().next().expect("one solid").0;
        assert_eq!(
            AtRestBody::validate(inverted, tol).map(|_| ()),
            Err(vec![ValidationError::NegativeVolume { solid }]),
            "{m}ε: the inside-out slab refuses at rest"
        );
        let s = AtRestBody::validate(raw, tol).expect("the slab is at rest");
        let what = format!("brick ∖ x=y slab {m}ε");
        let body = built(&what, topo::subtract(&big, &s, tol));
        assert_eq!(
            roles(&what, &body, tol),
            [ShellRole::Outer, ShellRole::Void],
            "{what}"
        );
        let what = format!("x=y slab ∪ far {m}ε");
        let body = built(&what, topo::union(&s, &far, tol));
        assert_eq!(shells_per_solid(&body), [1, 1], "{what}: two solids");
    }
}

// ---------------------------------------------------------------------
// The open class: a body thin in two directions
// (`work/flush/the-side-reader-refuses-a-body-thin-in-two-directions-whose-pairs-are-definite.md`).
// Every pair of a needle `w × w` across is `w` apart, definite above
// `Kε`, yet check 7 reads no side of it: no face is wide enough to offer
// a point more than `Kε` inside it, or every aimed ray lands within `Kε`
// of a face's boundary or crosses a sharp corner, and its volume over
// area, `≈ w/4`, lies in band. These rows pin today's refusal so that a
// fix to the class shows up here; they are not its answer.
// ---------------------------------------------------------------------

/// The needle widths, in ε: no face offers an aim at the first; every
/// face does, and every ray is set aside, at the other two.
const NEEDLE: [(f64, usize); 3] = [(10.5, 0), (21.0, 6), (30.0, 6)];

fn needle(w: f64, tol: Tol) -> Body<f64> {
    common::brick::<f64>((0.0, 1.0), (0.0, w), (0.0, w), tol)
}

/// The needle's refusal at rest: check 7 alone, its volume in band with
/// the sign its orientation gives, `aimed` faces offering an aim, every
/// one of their rays set aside.
fn needle_at_rest(m: f64, inverted: bool, tol: Tol) -> Vec<topo::SetAside> {
    let raw = needle(m * tol.eps(), tol);
    let raw = if inverted { raw.revert() } else { raw };
    let solid = raw.solids().next().expect("one solid").0;
    let what = format!("{}needle {m}ε", if inverted { "inside-out " } else { "" });
    let errors = AtRestBody::validate(raw, tol)
        .map(|_| ())
        .expect_err(&format!("{what}: refuses at rest (the open class)"));
    let [
        ValidationError::OrientationUndecided {
            solid: s,
            reading:
                topo::OrientationUnread::Volume {
                    margin: Some(margin),
                    aimed,
                    set_aside,
                },
        },
    ] = errors.as_slice()
    else {
        panic!("{what}: refuses its orientation alone, its volume in band: {errors:?}");
    };
    assert_eq!(*s, solid, "{what}");
    let sign = margin
        .margin
        .diagnostic_f64_for_error_text()
        .value()
        .expect("a value");
    assert_eq!(
        sign < 0.0,
        inverted,
        "{what}: the volume's sign is its orientation's"
    );
    let want = NEEDLE.iter().find(|n| n.0 == m).expect("a needle row").1;
    assert_eq!(*aimed, want, "{what}: faces offering an aim");
    assert_eq!(
        set_aside.len(),
        *aimed,
        "{what}: every ray aimed is set aside"
    );
    set_aside.clone()
}

/// **A needle refuses its orientation at rest**, either way out, and no
/// aimed ray reads it: today's answer to the open class.
#[test]
fn a_needle_refuses_its_orientation_at_rest() {
    let tol = Tol::witness();
    if stood_down(tol) {
        return;
    }
    for (m, _) in NEEDLE {
        for inverted in [false, true] {
            let set_aside = needle_at_rest(m, inverted, tol);
            if m == 21.0 {
                // Every ray at 21ε lands within `Kε` of a face's
                // boundary: the face-boundary half of the class.
                assert!(
                    set_aside
                        .iter()
                        .all(|s| matches!(s.why, topo::SetAsideWhy::Crossing(_))),
                    "needle 21ε: each ray's crossing is in band of a boundary: {set_aside:?}"
                );
            }
        }
    }
}

/// **A sharp corner is never a pair.** At ε 1e-6 the inside-out needle
/// 30ε across aims a ray at face 4v1 whose two closest crossings, on
/// faces 1v1 and 5v1, lie in band of each other; the two faces meet at
/// vertices 7v1 and 8v1, so the ray is set aside at a corner and the
/// refusal is the volume's, naming it. It was once refused as their pair.
#[test]
fn an_inside_out_needles_corner_is_set_aside_not_a_pair() {
    let tol = Tol::witness();
    if stood_down(tol) || (tol.eps() - 1e-6).abs() > 1e-9 {
        return;
    }
    let set_aside = needle_at_rest(30.0, true, tol);
    let corners: Vec<String> = set_aside
        .iter()
        .filter_map(|s| match &s.why {
            topo::SetAsideWhy::Corner { near, far, .. } => {
                Some(format!("{:?} {near:?} {far:?}", s.aimed))
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        corners,
        ["FaceKey(4v1) FaceKey(1v1) FaceKey(5v1)"],
        "the ray aimed at 4v1 is set aside at the corner of 1v1 and 5v1"
    );
    let raw = needle(30.0 * tol.eps(), tol).revert();
    let text = AtRestBody::validate(raw, tol)
        .map(|_| ())
        .expect_err("refuses")[0]
        .to_string();
    assert!(
        text.contains("sharp corner") && !text.contains("clearly apart"),
        "the refusal names the corner and offers no lever to part its faces: {text}"
    );
}

/// **A boolean whose result is a needle escalates its side.** `brick ∩
/// brick` leaving a needle `2Kε` across reads no side of it (no face
/// offers an aim), and its volume lies in band: the operands'
/// ill-conditioning, escalated on the solid's side.
#[test]
fn a_boolean_leaving_a_needle_escalates_its_side() {
    let tol = Tol::witness();
    if stood_down(tol) {
        return;
    }
    let w = 2.0 * tol.k() * tol.eps();
    let a = brick((0.0, 2.0), (0.0, 1.0), (0.0, 1.0), tol);
    let b = brick((-1.0, 3.0), (1.0 - w, 2.0), (1.0 - w, 2.0), tol);
    match topo::intersect(&a, &b, tol) {
        Err(BooleanError::Escalated {
            decision: BooleanDecision::SolidSide { aimed: 0, .. },
            ..
        }) => {}
        other => panic!("brick ∩ brick leaving a 2Kε needle escalates its side: {other:?}"),
    }
}
