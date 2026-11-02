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

fn assert_at_tier_3_prime(what: &str, body: &Body<f64>, tol: Tol) {
    assert_eq!(
        topo::validate_pseudomanifold(body, &topo::ContactRecords::default(), tol),
        Ok(()),
        "{what}: tier 3′ with no contacts"
    );
}

fn shells_per_solid(body: &Body<f64>) -> Vec<usize> {
    body.solids().map(|(_, s)| s.shells.len()).collect()
}

fn roles(what: &str, body: &Body<f64>, tol: Tol) -> Vec<ShellRole> {
    classify_shells(
        &topo::test_support::finished("the measured body", (body).clone(), tol),
        tol,
    )
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

/// How the needle channel's operands are placed.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Pose {
    /// Along the axes, every coordinate exact.
    Aligned,
    /// Each operand's corners turned about `(1, 2, 3)` by 1.1 rad.
    Turned,
    /// The aligned operands moved by `transform_rigid` through the same
    /// turn: carriers moved, not fitted from turned corners.
    Moved,
}

/// The turn about `(1, 2, 3)` by 1.1 rad, as a matrix.
fn turn() -> [[f64; 3]; 3] {
    let (s, c) = 1.1_f64.sin_cos();
    let n = 14.0_f64.sqrt();
    let (x, y, z) = (1.0 / n, 2.0 / n, 3.0 / n);
    let k = 1.0 - c;
    [
        [c + x * x * k, x * y * k - z * s, x * z * k + y * s],
        [y * x * k + z * s, c + y * y * k, y * z * k - x * s],
        [z * x * k - y * s, z * y * k + x * s, c + z * z * k],
    ]
}

/// The needle channel's operands at width `w`: the brick `(−1,2)³`, the
/// slab `(0,1)×(0.5,0.8)×(0.5,0.5+w)`, and the fill, flush with the slab
/// on its other five faces, whose face toward the channel runs from
/// `y = 0.5 + w` on the floor to `0.5 + w(1 + s)` on the ceiling. Also
/// the channel's eight corners and its cross-section's area.
struct Channel {
    big: AtRestBody<f64>,
    slab: AtRestBody<f64>,
    fill: AtRestBody<f64>,
    corners: Vec<[f64; 3]>,
    area: f64,
}

fn channel(pose: Pose, w: f64, s: f64, tol: Tol) -> Channel {
    let r = if pose == Pose::Turned {
        turn()
    } else {
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
    };
    let place = move |p: [f64; 3]| {
        let at = |i: usize| r[i][0] * p[0] + r[i][1] * p[1] + r[i][2] * p[2];
        [at(0), at(1), at(2)]
    };
    let motion = geom_core::RigidMotion::rotation(
        geom_core::UnitVec3::new(
            geom_core::Vec3::new(1.0, 2.0, 3.0),
            "the needle channel's turn",
            geom_core::Band::linear(tol).expect("the run's band"),
        )
        .expect("a unit axis"),
        1.1,
    )
    .expect("a finite angle");
    let block = |f: Box<dyn Fn(f64, f64, f64) -> [f64; 3]>| {
        let body = common::mapped_cube::<f64>(
            move |u, v, z| {
                let q = place(f(u, v, z));
                Point3::new(q[0], q[1], q[2])
            },
            tol,
        );
        let body = if pose == Pose::Moved {
            topo::transform_rigid(&body, &motion, tol).expect("the turn moves the operand")
        } else {
            body
        };
        AtRestBody::validate(body, tol).expect("an operand at rest")
    };
    let corner = |p: [f64; 3]| {
        if pose == Pose::Aligned {
            p
        } else {
            let t = turn();
            [0, 1, 2].map(|i| t[i][0] * p[0] + t[i][1] * p[1] + t[i][2] * p[2])
        }
    };
    let corners = (0..8)
        .map(|i| {
            let (x, z) = (f64::from(i & 1), f64::from(i >> 2 & 1));
            let y = if i >> 1 & 1 == 0 {
                0.5
            } else {
                0.5 + w + s * w * z
            };
            corner([x, y, 0.5 + w * z])
        })
        .collect();
    Channel {
        big: block(Box::new(|u, v, z| {
            [3.0 * u - 1.0, 3.0 * v - 1.0, 3.0 * z - 1.0]
        })),
        slab: block(Box::new(move |u, v, z| [u, 0.5 + 0.3 * v, 0.5 + w * z])),
        fill: block(Box::new(move |u, v, z| {
            let y0 = 0.5 + w + s * w * z;
            [u, y0 + v * (0.8 - y0), 0.5 + w * z]
        })),
        corners,
        area: w * w * (1.0 + s / 2.0),
    }
}

/// What a run of the needle channel does.
#[derive(Debug, PartialEq)]
enum Want {
    Builds,
    /// The reduction's sector question refuses (the GERMCERT row).
    Sector,
}

/// Every distinct point of `shell`'s loops.
fn shell_points(body: &Body<f64>, shell: topo::ShellKey) -> Vec<[f64; 3]> {
    let mut out: Vec<[f64; 3]> = Vec::new();
    for &f in &body.get_shell(shell).expect("the shell resolves").faces {
        let face = body.get_face(f).expect("its face resolves");
        for l in std::iter::once(face.outer).chain(face.rings.iter().copied()) {
            let topo::LoopBoundary::Cycle { first } = body.get_loop(l).expect("a loop").boundary
            else {
                continue;
            };
            for he in body.loop_cycle(first).expect("the loop walks") {
                let v = body.get_half_edge(he).expect("a half-edge").start;
                let p = body
                    .get_point(body.get_vertex(v).expect("a vertex").point)
                    .expect("a point");
                if !out.contains(&[p.x, p.y, p.z]) {
                    out.push([p.x, p.y, p.z]);
                }
            }
        }
    }
    out
}

/// `points` are `corners`, each within `w/1000`.
fn assert_corners(what: &str, points: &[[f64; 3]], corners: &[[f64; 3]], w: f64) {
    assert_eq!(
        points.len(),
        corners.len(),
        "{what}: the channel's corners, got {points:?}"
    );
    for q in corners {
        assert!(
            points
                .iter()
                .any(|p| (0..3).all(|i| (p[i] - q[i]).abs() < 1e-3 * w)),
            "{what}: a corner at {q:?}, got {points:?}"
        );
    }
}

/// The needle channel at width `w = m·ε` (fill face tilt `s`), each of
/// its five runs held to `want`: the union in both orders reads one
/// solid whose cavity is the channel, its signed volume `−area` and its
/// corners the channel's; the bar through `∖` and `∩` in both orders is
/// one shell, volume `area`, corners the channel's. Every operand and
/// every built body passes tier 3′ with no contacts. The volumes are
/// read to `1e-6` at ε 1e-6 and to 5% at ε 1e-9, where a turned bar's
/// sums round by up to 2%.
fn channel_rows(pose: Pose, m: f64, s: f64, tol: Tol, want: impl Fn(&str) -> Want) {
    let e = tol.eps();
    let w = m * e;
    let rel = if (e - 1e-9).abs() < 1e-18 { 5e-2 } else { 1e-6 };
    let what = |op: &str| format!("{op} leaving a {m}ε channel, tilt {s}, {pose:?}, at ε {e:e}");
    let c = channel(pose, w, s, tol);
    let hollow = built(&what("brick ∖ slab"), topo::subtract(&c.big, &c.slab, tol));
    let complement = built(&what("brick ∖ fill"), topo::subtract(&c.big, &c.fill, tol));
    for (name, body) in [
        ("brick", &c.big),
        ("slab", &c.slab),
        ("fill", &c.fill),
        ("brick ∖ slab", &hollow),
        ("brick ∖ fill", &complement),
    ] {
        assert_at_tier_3_prime(&what(name), body, tol);
    }
    for (op, run) in [
        ("brick ∖ slab ∪ fill", topo::union(&hollow, &c.fill, tol)),
        ("fill ∪ brick ∖ slab", topo::union(&c.fill, &hollow, tol)),
        ("slab ∖ fill", topo::subtract(&c.slab, &c.fill, tol)),
        (
            "slab ∩ (brick ∖ fill)",
            topo::intersect(&c.slab, &complement, tol),
        ),
        (
            "(brick ∖ fill) ∩ slab",
            topo::intersect(&complement, &c.slab, tol),
        ),
    ] {
        let what = what(op);
        match want(op) {
            Want::Sector => {
                assert!(
                    matches!(
                        &run,
                        Err(BooleanError::Escalated {
                            decision: BooleanDecision::Coincidence(Coincide::Sectors, _),
                            diag,
                        }) if matches!(diag.predicate, Some("bool_sector_within" | "bool_sector_coplanar"))
                    ),
                    "{what}: refuses on the sector question, got {:?}",
                    run.map(|_| ())
                );
                continue;
            }
            Want::Builds => {}
        }
        let body = built(&what, run);
        assert_at_tier_3_prime(&what, &body, tol);
        let shells = classify_shells(
            &topo::test_support::finished("the needle channel", (*body).clone(), tol),
            tol,
        )
        .unwrap_or_else(|err| panic!("{what}: every shell classifies, got {err:?}"));
        let (volume, cavity) = if op.contains('∪') {
            assert_eq!(
                shells.iter().map(|c| c.role).collect::<Vec<_>>(),
                [ShellRole::Outer, ShellRole::Void],
                "{what}: one solid, its channel a cavity"
            );
            (-c.area, shells[1].shell)
        } else {
            assert_eq!(shells_per_solid(&body), [1], "{what}: one bar");
            (c.area, shells[0].shell)
        };
        let got = shells
            .iter()
            .find(|r| r.shell == cavity)
            .expect("the shell")
            .enclosure
            .volume;
        assert!(
            ((got - volume) / volume).abs() < rel,
            "{what}: the channel's shell holds {volume:e}, got {got:e}"
        );
        assert_corners(&what, &shell_points(&body, cavity), &c.corners, w);
    }
}

/// **A needle cavity at the door** (review R2's
/// `r2_needle_cavity_and_solid_at_the_door`): `brick (−1,2)³ ∖ slab`,
/// then `∪` a fill that leaves of the slab's cavity a channel `w × w`
/// across, and the channel's bar alone through `∖` and `∩`
/// ([`channel_rows`]). Every pair is definite. The fill's face toward
/// the channel is `w` wide and its rim lies on the slab's walls, so no
/// point of it is off the band of those walls; the join reads its side
/// off its germ at the seam, and builds at every width.
///
/// Turned at ε 1e-9 at 12ε, the same operands refuse earlier, in the
/// reduction's sector question (`bool_sector_coplanar`)
/// (`work/germcert/a-sector-direction-read-off-a-needles-short-run-carries-its-rounding-over-1-w.md`).
#[test]
fn a_needle_cavity_builds_at_every_definite_width_and_holds_it_as_a_void() {
    let tol = Tol::witness();
    if stood_down(tol) {
        return;
    }
    let at_1e9 = (tol.eps() - 1e-9).abs() < 1e-18;
    for pose in [Pose::Aligned, Pose::Turned, Pose::Moved] {
        for m in [10.5, 12.0, 15.0, 21.0, 30.0] {
            channel_rows(pose, m, 0.0, tol, |_| match pose {
                Pose::Turned if at_1e9 && m == 12.0 => Want::Sector,
                _ => Want::Builds,
            });
        }
    }
}

/// **A channel whose fill face leans** (review of the seam read): the
/// fill's face toward the channel runs from `w` off the far wall at the
/// floor to `1.5w` at the ceiling, 30ε across, so the cavity's cross
/// section is a trapezoid. Its rim lies on the cavity's walls as the
/// upright face's does, and its germ there reads it. Both poses build
/// every op.
#[test]
fn a_needle_channel_whose_fill_face_leans_reads_its_side_off_its_seam() {
    let tol = Tol::witness();
    if stood_down(tol) {
        return;
    }
    for pose in [Pose::Aligned, Pose::Turned] {
        channel_rows(pose, 30.0, 0.5, tol, |_| Want::Builds);
    }
}

/// `result` with each unresolved sign's bracket checked to be one (its
/// ends ordered and finite) and then cleared, so the rows compare
/// the verdict and not the walk's last enclosure, which they do not pin.
fn bracket_read(result: Result<(), Vec<ValidationError>>) -> Result<(), Vec<ValidationError>> {
    result.map_err(|errors| {
        errors
            .into_iter()
            .map(|error| match error {
                ValidationError::VolumeSignUnresolved {
                    solid,
                    limit,
                    volume,
                } => {
                    let (lo, hi) = volume.expect("the structural door reads a bracket end");
                    assert!(
                        lo.is_finite() && hi.is_finite() && lo <= hi,
                        "an unresolved sign carries a bracket: [{lo:e}, {hi:e}]"
                    );
                    ValidationError::VolumeSignUnresolved {
                        solid,
                        limit,
                        volume: None,
                    }
                }
                other => other,
            })
            .collect()
    })
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
/// at an end of the inside-out needle along the axes, and on the
/// inside-out turned needles 12ε and 15ε across, the ray aimed at a long
/// face's point stepped in from its end (`side_reader`'s planar aim, which
/// hangs on the face's first half-edge, so the needle unreverted aims
/// elsewhere).
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
                volume: None,
            }]);
            let plain = if reads { Ok(()) } else { unresolved.clone() };
            let end_aim_reads = turned && !at_1e6 && (m == 12.0 || m == 15.0);
            let inverted = if reads || !turned || end_aim_reads {
                Err(vec![ValidationError::NegativeVolume { solid }])
            } else {
                unresolved
            };
            assert_eq!(
                bracket_read(topo::validate_geometric_structural(&body, tol)),
                plain,
                "{what} through the structural door"
            );
            assert_eq!(
                bracket_read(topo::validate_geometric_structural(&inside_out, tol)),
                inverted,
                "inside-out {what} through the structural door"
            );
        }
    }
}
