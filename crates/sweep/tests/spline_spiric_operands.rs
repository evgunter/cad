//! **Spline and spiric operand edges reach the Boolean, and none builds
//! wrong.** No gate refuses an operand for its edge carriers: each lane
//! that reads one refuses a kind it has no row for at its own site,
//! naming the edge and the face. This census runs every population
//! carrying such edges through every op, against bricks in several
//! placements, in both operand orders, and holds each outcome to an
//! oracle:
//!
//! - an `Ok` carries the volume the oracle gives: the closed form in
//!   `V(O)` and `V(B)` where the placement has one, and otherwise
//!   inclusion–exclusion, every op that builds implying one shared
//!   `V(O ∩ B)`. `V(O)` is the operand's closed form where it has one
//!   (its measure holds it), and otherwise its measure. Every body is
//!   measured at [`RESOLUTION`], its bracket held within the band, and
//!   each reading is held to its oracle within the band beyond the
//!   brackets on both sides;
//! - a placement marked to build builds every op;
//! - an `Err` is a typed refusal, never a kernel invariant, and a
//!   refusal naming a face names one of the operand its site says owns
//!   it, of the kind it states. The bricks are
//!   [`topo::test_support::brick_with_fresh_keys`], whose face keys no
//!   population shares (checked), so a face key read in the wrong body
//!   resolves nowhere.
//!
//! The populations:
//!
//! - the loft prism (`sweep::test_support::loft_prism`): four NURBS
//!   walls, two planar caps, NURBS seams, `V = 9 m³` exactly;
//! - the lofted cylinder (`lofted_cylinder`): rational NURBS walls and
//!   seams, `V = π·h`.
//!
//!   Each builds every op against a brick far off and a brick around it.
//!   Its spline edges are boxed by their control nets and cross no face
//!   of the brick, its NURBS faces' boxes meet none of the brick's, and
//!   each vertex of the brick stands past every face of it, so the
//!   containment fallback reads each side. A brick inside it, or cutting
//!   it, refuses typed at a later site;
//! - the vessel's cavity (`common::torus_walls::vessel_cavity`), a
//!   quarter of the revolved vessel about `ŷ`: a neck cylinder under a
//!   mouth plane, a torus band whose two rims on the quarter's cut
//!   planes are spirics, a lower cylinder and a base. Its volume is an
//!   elliptic integral, so it has no closed form to hold its measure to;
//!   inclusion–exclusion holds every cut: a poke through the neck
//!   wall, a peg through the mouth, a brick inside, each away from the
//!   spirics, all build; a poke into the torus band refuses typed
//!   (`a_poke_into_the_cavitys_torus_band_refuses_typed`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::SurfaceKind;
use geom_core::Tol;
use topo::{AtRestBody, BooleanError, BooleanResult, Operand};

use crate::common::torus_walls::vessel_cavity;
use sweep::test_support::{finished, loft_prism, lofted_cylinder};
use topo::test_support::brick_with_fresh_keys;

type Box3 = ((f64, f64), (f64, f64), (f64, f64));

/// What an `Ok` is held to.
#[derive(Clone, Copy)]
enum Oracle {
    /// `V(O ∪ B)`, `V(O ∩ B)`, `V(O ∖ B)`, `V(B ∖ O)` from `V(O)` and
    /// `V(B)`.
    Known(fn(f64, f64) -> [f64; 4]),
    /// Inclusion–exclusion: each op that builds implies `V(O ∩ B)`, and
    /// they agree, inside `[0, min(V(O), V(B))]`.
    Cut,
}

/// A brick's placement, its oracle, and whether every op must build.
struct Placement {
    name: &'static str,
    brick: Box3,
    oracle: Oracle,
    builds: bool,
}

fn volume_of_box(((x0, x1), (y0, y1), (z0, z1)): Box3) -> f64 {
    (x1 - x0) * (y1 - y0) * (z1 - z0)
}

/// `brick` disjoint from the operand.
fn far(brick: Box3, builds: bool) -> Placement {
    let of: fn(f64, f64) -> [f64; 4] = |v, b| [v + b, 0.0, v, b];
    Placement {
        name: "far",
        brick,
        oracle: Oracle::Known(of),
        builds,
    }
}

/// `brick` enclosing it.
fn around(brick: Box3, builds: bool) -> Placement {
    let of: fn(f64, f64) -> [f64; 4] = |v, b| [b, v, 0.0, b - v];
    Placement {
        name: "around",
        brick,
        oracle: Oracle::Known(of),
        builds,
    }
}

/// `brick` within it.
fn inside(brick: Box3, builds: bool) -> Placement {
    let of: fn(f64, f64) -> [f64; 4] = |v, b| [v, b, v - b, 0.0];
    Placement {
        name: "inside",
        brick,
        oracle: Oracle::Known(of),
        builds,
    }
}

fn cut(name: &'static str, brick: Box3, builds: bool) -> Placement {
    Placement {
        name,
        brick,
        oracle: Oracle::Cut,
        builds,
    }
}

/// The refusals that name a broken kernel rather than an input or a
/// frontier.
fn kernel_invariant(e: &BooleanError) -> bool {
    matches!(
        e,
        BooleanError::ClassificationInvariant { .. } | BooleanError::JoinDesync { .. }
    ) || matches!(e, BooleanError::Join(j) if matches!(j.error(), topo::SplitJoinError::SectionInvariant { .. }))
}

/// Where a refusal names a face, that it is a face of the operand its
/// site says owns it — resolving there and not in the other — of the
/// kind it states; and where it names an edge, a spiric or spline edge
/// of its operand.
fn names_its_own_face(e: &BooleanError, [a, b]: [&AtRestBody<f64>; 2]) -> Result<(), String> {
    let body = |operand: Operand| match operand {
        Operand::A => a,
        Operand::B => b,
    };
    let owned = |operand: Operand, face| {
        body(operand).get_face(face).is_some() && body(operand.other()).get_face(face).is_none()
    };
    let kind_of = |operand: Operand, face| {
        body(operand)
            .get_face(face)
            .and_then(|f| body(operand).get_surface(f.surface))
            .map(geom::Surface::kind)
    };
    let check = |ok: bool, why: String| ok.then_some(()).ok_or(why);
    match *e {
        BooleanError::CurvedBooleanUnsupported {
            operand,
            face,
            kind,
        } => check(
            owned(operand, face) && kind_of(operand, face) == Some(kind),
            format!("{face:?} is no {kind:?} face of {operand:?} alone"),
        ),
        BooleanError::CurvedPairUnsupported {
            operand,
            face,
            kind,
            other_face,
            other_kind,
            ..
        } => check(
            owned(operand, face)
                && kind_of(operand, face) == Some(kind)
                && owned(operand.other(), other_face)
                && kind_of(operand.other(), other_face) == Some(other_kind),
            format!(
                "{face:?}/{other_face:?} are not {kind:?}/{other_kind:?} faces of {operand:?} and its partner"
            ),
        ),
        BooleanError::EdgeCarrierUnsupported {
            operand,
            edge,
            face,
            face_of,
            ..
        } => check(
            matches!(
                topo::query::edge_carrier_kind(body(operand), edge),
                Some(topo::query::CurveKind::Nurbs | topo::query::CurveKind::Spiric)
            ) && owned(face_of, face),
            format!("{edge:?} of {operand:?} against {face:?} of {face_of:?}"),
        ),
        BooleanError::SplineCrossingUnsettled {
            operand,
            edge,
            face,
            face_of,
            ..
        } => check(
            topo::query::edge_carrier_kind(body(operand), edge)
                == Some(topo::query::CurveKind::Nurbs)
                && owned(face_of, face),
            format!("{edge:?} of {operand:?} against {face:?} of {face_of:?}"),
        ),
        _ => Ok(()),
    }
}

/// One op of a placement: its name, its two operands in order, its
/// outcome, its slot in [`Oracle::Known`], and `V(O ∩ B)` from its
/// volume, given `V(O)` and `V(B)`.
type Run<'a> = (
    &'static str,
    [&'a AtRestBody<f64>; 2],
    Result<BooleanResult<f64>, BooleanError>,
    usize,
    fn(f64, f64, f64) -> f64,
);

/// The resolution (mean boundary displacement, m) every body here is
/// measured at: the width at which a reading's bracket lies within
/// [`band`] of the volumes here, at every ε row. It is no finer than the
/// coarsest row's ε, 1e-6.
const RESOLUTION: f64 = 1e-6;

/// How far a volume `v` may read from its oracle beyond the brackets.
fn band(v: f64) -> f64 {
    1e-6 * v.abs().max(1.0)
}

/// `body`'s volume at [`RESOLUTION`] and its bracket's half-width, which
/// lies within [`band`].
fn measure(what: &str, body: &AtRestBody<f64>) -> (f64, f64) {
    let m = topo::test_support::volume_at(what, body, Tol::witness(), RESOLUTION);
    assert!(
        m.volume_pad <= band(m.volume),
        "{what}: V = {} ± {:e}, a bracket wider than its band {:e}",
        m.volume,
        m.volume_pad,
        band(m.volume)
    );
    (m.volume, m.volume_pad)
}

/// Runs the four ops in both orders against each placement and holds
/// each outcome to its oracle; returns the census lines. `V(O)` is the
/// operand's closed form where it has one, which its measure holds, and
/// otherwise its measure.
fn census(
    name: &str,
    operand: &AtRestBody<f64>,
    closed_form: Option<f64>,
    placements: &[Placement],
) -> Vec<String> {
    let tol = Tol::witness();
    let (measured, measured_pad) = measure(&format!("{name}: the operand"), operand);
    // `V(O)` and its bracket's half-width.
    let (v, v_pad) = match closed_form {
        Some(want) => {
            topo::test_support::assert_volume_near(
                &format!("{name}: the operand against its closed form"),
                measured,
                measured_pad,
                want,
                band(want),
            );
            (want, 0.0)
        }
        None => (measured, measured_pad),
    };
    let mut lines = Vec::new();
    for p in placements {
        let (x, y, z) = p.brick;
        let brick = finished("the brick", brick_with_fresh_keys::<f64>(x, y, z, tol), tol);
        assert!(
            brick.faces().all(|(f, _)| operand.get_face(f).is_none()),
            "{name} / {}: the brick's face keys are disjoint from the operand's",
            p.name
        );
        let vb = volume_of_box(p.brick);
        // Each op's volume as a function of `V(O ∩ B)`, inverted.
        let runs: [Run<'_>; 6] = [
            (
                "O ∪ B",
                [operand, &brick],
                topo::union(operand, &brick, tol),
                0,
                |v, vb, got| v + vb - got,
            ),
            (
                "B ∪ O",
                [&brick, operand],
                topo::union(&brick, operand, tol),
                0,
                |v, vb, got| v + vb - got,
            ),
            (
                "O ∩ B",
                [operand, &brick],
                topo::intersect(operand, &brick, tol),
                1,
                |_, _, got| got,
            ),
            (
                "B ∩ O",
                [&brick, operand],
                topo::intersect(&brick, operand, tol),
                1,
                |_, _, got| got,
            ),
            (
                "O ∖ B",
                [operand, &brick],
                topo::subtract(operand, &brick, tol),
                2,
                |v, _, got| v - got,
            ),
            (
                "B ∖ O",
                [&brick, operand],
                topo::subtract(&brick, operand, tol),
                3,
                |_, vb, got| vb - got,
            ),
        ];
        let mut implied = Vec::new();
        for (op, pair, got, slot, meet) in runs {
            let line = format!("{name} / {} / {op}: ", p.name);
            match got {
                Err(e) => {
                    assert!(
                        !kernel_invariant(&e),
                        "{line}a kernel invariant, not a refusal: {e:?}"
                    );
                    assert!(!p.builds, "{line}must build, refuses {e:?}");
                    if let Err(why) = names_its_own_face(&e, pair) {
                        panic!("{line}the refusal misnames its subject ({why}): {e:?}");
                    }
                    lines.push(format!("{line}refuses {e:?}"));
                }
                Ok(result) => {
                    let (got, got_pad) = match &result {
                        BooleanResult::Empty => (0.0, 0.0),
                        BooleanResult::Body(b) => measure(&format!("{line}the result"), &b.body),
                    };
                    match p.oracle {
                        Oracle::Known(of) => {
                            let want = of(v, vb)[slot];
                            topo::test_support::assert_volume_near(
                                &format!("{line}against the closed form"),
                                got,
                                got_pad + v_pad,
                                want,
                                band(want),
                            );
                        }
                        // `V(O)` enters every op's implied `V(O ∩ B)` alike,
                        // so their agreement reads only the results' brackets.
                        Oracle::Cut => implied.push((op, meet(v, vb, got), got_pad)),
                    }
                    lines.push(format!("{line}builds V = {got}"));
                }
            }
        }
        if let Some(&(_, first, first_pad)) = implied.first() {
            let scale = v.max(vb);
            let slack = 1e-9 * scale + first_pad + v_pad;
            assert!(
                (-slack..=v.min(vb) + slack).contains(&first),
                "{name} / {}: V(O ∩ B) = {first} ± {:e} lies outside [0, min(V(O), V(B))]",
                p.name,
                first_pad + v_pad
            );
            for &(op, meet, pad) in &implied {
                assert!(
                    (meet - first).abs() <= 1e-9 * scale + pad + first_pad,
                    "{name} / {}: {op} implies V(O ∩ B) = {meet} ± {pad:e}, the others {first} ± {first_pad:e}",
                    p.name
                );
            }
        }
    }
    lines
}

fn loft_prism_census() -> Vec<String> {
    // The prism's box is `[−1.375, 1.375] × [−1, 1] × [0, 2]`; its
    // section at `z ∈ [0.5, 1.5]` holds `[−1, 1]²`.
    let tol = Tol::witness();
    let prism = finished("the loft prism", loft_prism(tol), tol);
    census(
        "loft prism",
        &prism,
        Some(9.0),
        &[
            far(((5.0, 6.0), (5.0, 6.0), (5.0, 6.0)), true),
            around(((-3.0, 3.0), (-3.0, 3.0), (-1.0, 3.0)), true),
            inside(((-0.5, 0.5), (-0.5, 0.5), (0.5, 1.5)), false),
            cut(
                "slab through the walls",
                ((-3.0, 3.0), (-0.25, 0.25), (0.75, 1.25)),
                false,
            ),
            cut(
                "peg through the top cap",
                ((-0.25, 0.25), (-0.25, 0.25), (1.75, 2.5)),
                false,
            ),
        ],
    )
}

fn lofted_cylinder_census() -> Vec<String> {
    // The unit-radius lofted cylinder, `z ∈ [0, 2]`.
    let tol = Tol::witness();
    let v = core::f64::consts::PI * 2.0;
    let cylinder = finished("the lofted cylinder", lofted_cylinder(2.0, tol), tol);
    census(
        "lofted cylinder",
        &cylinder,
        Some(v),
        &[
            far(((5.0, 6.0), (5.0, 6.0), (5.0, 6.0)), true),
            around(((-2.0, 2.0), (-2.0, 2.0), (-1.0, 3.0)), true),
            inside(((-0.5, 0.5), (-0.5, 0.5), (0.5, 1.5)), false),
            cut(
                "slab through the wall",
                ((-2.0, 2.0), (-0.25, 0.25), (0.75, 1.25)),
                false,
            ),
        ],
    )
}

/// The cavity, at rest.
fn cavity() -> AtRestBody<f64> {
    let (_, cavity) = vessel_cavity(1.0 / 128.0);
    AtRestBody::validate(cavity, Tol::witness()).expect("the cavity passes the at-rest gate")
}

/// The cavity's placements, in its own frame: the quarter `x ≥ 1/128`,
/// `z ≤ −1/128` of the vessel about `ŷ`, `y ∈ [1/128, 0.3672]`; the neck
/// cylinder of radius 0.1016 over `y ∈ [0.1797, 0.3672]` under the mouth
/// plane `y = 0.3672`; the torus band (centre `y = 0.125`, radii 0.0938
/// and 0.0703) over `y ∈ [0.0703, 0.1797]`, whose rims on the cut planes
/// are the two spirics. Every cut stands clear of the band.
fn cavity_census() -> Vec<String> {
    let cavity = cavity();
    census(
        "vessel cavity",
        &cavity,
        None,
        &[
            far(((5.0, 6.0), (5.0, 6.0), (5.0, 6.0)), true),
            around(((-1.0, 1.0), (-1.0, 1.0), (-1.0, 1.0)), true),
            inside(((0.03, 0.05), (0.25, 0.30), (-0.05, -0.03)), true),
            cut(
                "poke through the neck wall",
                ((0.08, 0.2), (0.25, 0.30), (-0.05, -0.03)),
                true,
            ),
            cut(
                "peg through the mouth",
                ((0.03, 0.05), (0.33, 0.40), (-0.05, -0.03)),
                true,
            ),
        ],
    )
}

#[test]
fn no_spline_or_spiric_operand_builds_wrong() {
    let mut lines = loft_prism_census();
    lines.extend(lofted_cylinder_census());
    lines.extend(cavity_census());
    for line in &lines {
        println!("{line}");
    }
}

/// **A poke into the cavity's torus band refuses typed**: the brick's
/// wall crosses the band between the two spiric rims, a torus face
/// against a plane, and the section it would read there routes to the
/// general rung the boolean does not run.
#[test]
fn a_poke_into_the_cavitys_torus_band_refuses_typed() {
    let tol = Tol::witness();
    let cavity = cavity();
    let brick = finished(
        "the brick",
        brick_with_fresh_keys::<f64>((0.12, 0.25), (0.11, 0.14), (-0.06, -0.04), tol),
        tol,
    );
    for (op, got) in [
        ("O ∪ B", topo::union(&cavity, &brick, tol)),
        ("B ∪ O", topo::union(&brick, &cavity, tol)),
        ("O ∩ B", topo::intersect(&cavity, &brick, tol)),
        ("B ∩ O", topo::intersect(&brick, &cavity, tol)),
        ("O ∖ B", topo::subtract(&cavity, &brick, tol)),
        ("B ∖ O", topo::subtract(&brick, &cavity, tol)),
    ] {
        let e = got.expect_err("the band poke refuses");
        let BooleanError::GermSectionSpiric {
            a_kind,
            b_kind,
            section: geom_brep::SectionError::RoutesToGeneralRung { .. },
            ..
        } = e
        else {
            panic!("{op}: the plane × torus section routes to the general rung, got {e:?}");
        };
        let mut kinds = [a_kind, b_kind];
        kinds.sort_by_key(|k| format!("{k:?}"));
        assert_eq!(
            kinds,
            [SurfaceKind::Plane, SurfaceKind::Torus],
            "{op}: the pair"
        );
    }
}
