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
        let inverted = body.revert().expect("the slab reverts");
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
        let inverted = raw.revert().expect("the slab reverts");
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
// A body thin in two directions
// (`work/flush/the-side-reader-refuses-a-body-thin-in-two-directions-whose-pairs-are-definite.md`,
// Ev's ruling on PR 5119). Every pair of a needle `w × w` across is `w`
// apart, definite above `Kε`; no aimed ray reads it (no face is wide
// enough to offer a point more than `Kε` inside it, or every ray lands
// within `Kε` of a face's boundary or crosses a sharp corner), so its
// volume reads, and the volume's sign is certified by its enclosure
// alone, never held against the band: a needle validates, and an
// inside-out one is `NegativeVolume`.
// ---------------------------------------------------------------------

/// The needle widths, in ε.
const NEEDLE: [f64; 5] = [10.5, 12.0, 15.0, 21.0, 30.0];

fn needle(w: f64, tol: Tol) -> Body<f64> {
    common::brick::<f64>((0.0, 1.0), (0.0, w), (0.0, w), tol)
}

/// The needle `1 × w × w` turned about `(37, −21, 13)` by 0.7 rad and
/// moved off the origin: every coordinate rounds, and a re-derivation
/// that fans each face from one corner reads the body's length squared
/// in every face's vector area.
fn turned_needle(w: f64, tol: Tol) -> Body<f64> {
    let (x, y, z) = {
        let n = (37.0_f64 * 37.0 + 21.0 * 21.0 + 13.0 * 13.0).sqrt();
        (37.0 / n, -21.0 / n, 13.0 / n)
    };
    let (s, c) = 0.7_f64.sin_cos();
    let k = 1.0 - c;
    let r = [
        [c + x * x * k, x * y * k - z * s, x * z * k + y * s],
        [y * x * k + z * s, c + y * y * k, y * z * k - x * s],
        [z * x * k - y * s, z * y * k + x * s, c + z * z * k],
    ];
    let t = [0.3, -0.2, 0.5];
    topo::test_support::mapped_cube::<f64>(
        move |a, b, c| {
            let p = [a, b * w, c * w];
            let at = |i: usize| r[i][0] * p[0] + r[i][1] * p[1] + r[i][2] * p[2] + t[i];
            Point3::new(at(0), at(1), at(2))
        },
        tol,
    )
}

/// `raw` at rest: it validates, or, inside out, refuses `NegativeVolume`
/// alone.
fn reads_its_orientation(what: &str, raw: Body<f64>, inverted: bool, tol: Tol) {
    let raw = if inverted {
        raw.revert().expect("the needle reverts")
    } else {
        raw
    };
    let solid = raw.solids().next().expect("one solid").0;
    match (AtRestBody::validate(raw, tol), inverted) {
        (Ok(_), false) => {}
        (Err(errors), true) => assert_eq!(
            errors,
            [ValidationError::NegativeVolume { solid }],
            "inside-out {what}: inside out, and nothing else"
        ),
        (other, _) => panic!(
            "{}{what}: reads its orientation: {:?}",
            if inverted { "inside-out " } else { "" },
            other.map(|_| ())
        ),
    }
}

/// **A needle reads its orientation at rest**, either way out, along the
/// axes and turned off them: its volume's certified enclosure decides
/// its sign.
#[test]
fn a_needle_reads_its_orientation_at_rest() {
    let tol = Tol::witness();
    if stood_down(tol) {
        return;
    }
    for m in NEEDLE {
        let w = m * tol.eps();
        for inverted in [false, true] {
            reads_its_orientation(&format!("needle {m}ε"), needle(w, tol), inverted, tol);
            reads_its_orientation(
                &format!("turned needle {m}ε"),
                turned_needle(w, tol),
                inverted,
                tol,
            );
        }
    }
}

/// **A sharp corner is never a pair.** At ε 1e-6 the inside-out needle
/// 30ε across aims a ray at a face whose two closest crossings, on two
/// faces that meet, lie in band of each other; the ray is set aside at
/// that corner and reads nothing, and the volume reads the needle inside
/// out. It was once refused as their pair.
#[test]
fn an_inside_out_needles_corner_is_set_aside_not_a_pair() {
    let tol = Tol::witness();
    if stood_down(tol) || (tol.eps() - 1e-6).abs() > 1e-9 {
        return;
    }
    reads_its_orientation("needle 30ε", needle(30.0 * tol.eps(), tol), true, tol);
}

/// **A boolean whose result is a needle builds it.** `brick ∩ brick`
/// leaving a needle `2Kε` across reads no side of it by ray (no face
/// offers an aim), and its volume's certified enclosure reads it `Out`.
#[test]
fn a_boolean_leaving_a_needle_builds_it() {
    let tol = Tol::witness();
    if stood_down(tol) {
        return;
    }
    let w = 2.0 * tol.k() * tol.eps();
    let a = brick((0.0, 2.0), (0.0, 1.0), (0.0, 1.0), tol);
    let b = brick((-1.0, 3.0), (1.0 - w, 2.0), (1.0 - w, 2.0), tol);
    let body = built(
        "brick ∩ brick leaving a 2Kε needle",
        topo::intersect(&a, &b, tol),
    );
    assert_eq!(shells_per_solid(&body), [1], "one needle");
}

/// **A boolean leaving a needle turned off the axes builds it**, at every
/// ε a run takes (designer B's four poses, 2Kε and 3Kε across). The
/// re-derivation takes the volume about the mean of the body's points,
/// within its thickness of every long face; about the least corner of
/// its points, which a turned needle stands off by up to its length, the
/// long faces' terms cancelled in pairs inside an enclosure that held
/// zero at ε 1e-12.
#[test]
fn a_boolean_leaving_a_turned_needle_builds_it_in_every_pose() {
    let tol = Tol::witness();
    let rot = |axis: [f64; 3], ang: f64| {
        let n = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
        let (x, y, z) = (axis[0] / n, axis[1] / n, axis[2] / n);
        let (s, c) = ang.sin_cos();
        let k = 1.0 - c;
        [
            [c + x * x * k, x * y * k - z * s, x * z * k + y * s],
            [y * x * k + z * s, c + y * y * k, y * z * k - x * s],
            [z * x * k - y * s, z * y * k + x * s, c + z * z * k],
        ]
    };
    let poses: [([f64; 3], f64, [f64; 3]); 4] = [
        ([0.0, 0.0, 1.0], 0.0, [0.0; 3]),
        ([1.0, 2.0, 3.0], 1.1, [0.3, -0.2, 0.5]),
        ([0.3, -0.7, 0.2], 2.4, [1.7, 0.4, -0.9]),
        ([1.0, 1.0, 0.0], core::f64::consts::FRAC_PI_4, [0.0; 3]),
    ];
    for across in [2.0, 3.0] {
        let w = across * tol.k() * tol.eps();
        for (i, &(axis, ang, c)) in poses.iter().enumerate() {
            let r = rot(axis, ang);
            let place = move |p: [f64; 3]| {
                let at = |k: usize| c[k] + r[k][0] * p[0] + r[k][1] * p[1] + r[k][2] * p[2];
                Point3::new(at(0), at(1), at(2))
            };
            let at_rest = |body| AtRestBody::validate(body, tol).expect("an operand at rest");
            let a = at_rest(common::mapped_cube::<f64>(
                move |x, y, z| place([x, y, z]),
                tol,
            ));
            let b = at_rest(common::mapped_cube::<f64>(
                move |x, y, z| place([x, 1.0 - w + y * (1.0 + w), 1.0 - w + z * (1.0 + w)]),
                tol,
            ));
            let what = format!("brick ∩ brick leaving a {across}Kε needle, pose {i}");
            let body = built(&what, topo::intersect(&a, &b, tol));
            assert_eq!(shells_per_solid(&body), [1], "{what}: one needle");
        }
    }
}

/// **A needle cavity at the door** (review R2's
/// `r2_needle_cavity_and_solid_at_the_door`): `brick (−1,2)³ ∖ slab`,
/// then `∪` a fill that leaves of the slab's cavity a channel `w × w`
/// across. No face of the channel offers an aim that reads, and its
/// volume's certified enclosure reads it a cavity: one solid, its
/// channel a `Void`. Measured at these widths; at 10.5ε to 15ε the union
/// refuses earlier, at the join's own section loop
/// (`SectionLoopUndecided`), before any result is read.
#[test]
fn a_union_leaving_a_needle_cavity_holds_it_as_a_void() {
    let tol = Tol::witness();
    if stood_down(tol) {
        return;
    }
    let e = tol.eps();
    let big = brick((-1.0, 2.0), (-1.0, 2.0), (-1.0, 2.0), tol);
    for m in [21.0, 30.0] {
        let w = m * e;
        let what = format!("brick ∖ slab ∪ fill leaving a {m}ε channel at ε {e:e}");
        let slab = brick((0.0, 1.0), (0.5, 0.8), (0.5, 0.5 + w), tol);
        let fill = brick((0.0, 1.0), (0.5 + w, 0.8), (0.5, 0.5 + w), tol);
        let hollow = built(&what, topo::subtract(&big, &slab, tol));
        let body = built(&what, topo::union(&hollow, &fill, tol));
        assert_eq!(
            shells_per_solid(&body),
            [2],
            "{what}: one solid, its channel inside"
        );
        assert_eq!(
            roles(&what, &body, tol),
            [ShellRole::Outer, ShellRole::Void],
            "{what}: the channel is a cavity"
        );
    }
}

/// **A needle's orientation through the structural doors**, which hold
/// no quadrature lane and read the volume walk's own sums: the sign of
/// the sum against a stated bound on its rounding
/// (`props::rounding_bound`), never its `V/A` against the band
/// (`work/tally/lane-free-volume-sign-reads-decide-on-a-rounded-sum.md`).
/// A needle `w` across, a unit long, holds `V = w²`, and its sums round
/// by about `u` per coordinate product of its unit length: at ε 1e-6
/// (`w ≥ 1.05e-5`, `V ≥ 1.1e-10`) the sign clears that bound, so the
/// needle validates and inside out is `NegativeVolume`; at ε 1e-9
/// (`V ≤ 9e-16`) it lies within it, and the needle refuses as a
/// capability limit of the scalar, `VolumeSignUnresolved { Scalar }`,
/// never as a thinness gate or a defect — except where an aimed ray reads
/// its side first. At ε 1e-9 one reads every needle 21ε and 30ε across;
/// below `2Kε` every ray through a needle meets a wedge (a face it passes
/// within the band of two edges of) and reads nothing, except one aimed
/// at an end of the inside-out needle along the axes.
#[test]
fn a_needles_orientation_through_the_structural_doors() {
    let tol = Tol::witness();
    if stood_down(tol) {
        return;
    }
    let at_1e6 = (tol.eps() - 1e-6).abs() < 1e-12;
    for m in NEEDLE {
        let w = m * tol.eps();
        for (turned, what, body) in [
            (false, format!("needle {m}ε"), needle(w, tol)),
            (true, format!("turned needle {m}ε"), turned_needle(w, tol)),
        ] {
            let reads = at_1e6 || m > 2.0 * tol.k();
            let solid = body.solids().next().expect("one solid").0;
            let inside_out = body.revert().expect("the needle reverts");
            let unresolved = Err(vec![ValidationError::VolumeSignUnresolved {
                solid,
                limit: topo::SignLimit::Scalar,
            }]);
            let plain = if reads { Ok(()) } else { unresolved.clone() };
            let inverted = if reads || !turned {
                Err(vec![ValidationError::NegativeVolume { solid }])
            } else {
                unresolved
            };
            assert_eq!(
                topo::validate_geometric_structural(&body, tol),
                plain,
                "{what} through the structural door"
            );
            assert_eq!(
                topo::validate_geometric_structural(&inside_out, tol),
                inverted,
                "inside-out {what} through the structural door"
            );
        }
    }
}
