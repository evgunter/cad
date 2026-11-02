//! **Spline and spiric operand edges reach the Boolean, and none builds
//! wrong.** No gate refuses an operand for its edge carriers: each lane
//! that reads one refuses a kind it has no row for at its own site,
//! naming the edge and the face. This census runs every population
//! carrying such edges through every op, against bricks in several
//! placements, in both operand orders, and holds each outcome to an
//! oracle:
//!
//! - an `Ok` carries the volume the oracle gives: the closed form where
//!   the placement has one, and otherwise inclusion–exclusion, every
//!   op that builds implying one shared `V(O ∩ B)`;
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
//!   elliptic integral, so its own measure is the closed form's stand-in
//!   and inclusion–exclusion holds every cut: a poke through the neck
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
    /// `V(O ∪ B)`, `V(O ∩ B)`, `V(O ∖ B)`, `V(B ∖ O)`.
    Known([f64; 4]),
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

fn known(
    name: &'static str,
    brick: Box3,
    v: f64,
    builds: bool,
    of: impl Fn(f64, f64) -> [f64; 4],
) -> Placement {
    Placement {
        name,
        brick,
        oracle: Oracle::Known(of(v, volume_of_box(brick))),
        builds,
    }
}

/// `brick` disjoint from an operand of volume `v`.
fn far(brick: Box3, v: f64, builds: bool) -> Placement {
    known("far", brick, v, builds, |v, b| [v + b, 0.0, v, b])
}

/// `brick` enclosing it.
fn around(brick: Box3, v: f64, builds: bool) -> Placement {
    known("around", brick, v, builds, |v, b| [b, v, 0.0, b - v])
}

/// `brick` within it.
fn inside(brick: Box3, v: f64, builds: bool) -> Placement {
    known("inside", brick, v, builds, |v, b| [v, b, v - b, 0.0])
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

/// Runs the four ops in both orders against each placement and holds
/// each outcome to its oracle; returns the census lines.
fn census(name: &str, operand: &AtRestBody<f64>, placements: &[Placement]) -> Vec<String> {
    let tol = Tol::witness();
    let v = topo::test_support::pinned_volume("the operand measures", operand, tol).volume;
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
                    let got = match &result {
                        BooleanResult::Empty => 0.0,
                        BooleanResult::Body(b) => {
                            topo::test_support::pinned_volume("the measured body", &b.body, tol)
                                .volume
                        }
                    };
                    match p.oracle {
                        Oracle::Known(want) => assert!(
                            (got - want[slot]).abs() <= 1e-6 * want[slot].max(1.0),
                            "{line}builds V = {got}, the closed form gives {}",
                            want[slot]
                        ),
                        Oracle::Cut => implied.push((op, meet(v, vb, got))),
                    }
                    lines.push(format!("{line}builds V = {got}"));
                }
            }
        }
        if let Some(&(_, first)) = implied.first() {
            let scale = v.max(vb);
            assert!(
                (-1e-9 * scale..=v.min(vb) + 1e-9 * scale).contains(&first),
                "{name} / {}: V(O ∩ B) = {first} lies outside [0, min(V(O), V(B))]",
                p.name
            );
            for &(op, meet) in &implied {
                assert!(
                    (meet - first).abs() <= 1e-9 * scale,
                    "{name} / {}: {op} implies V(O ∩ B) = {meet}, the others {first}",
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
        &[
            far(((5.0, 6.0), (5.0, 6.0), (5.0, 6.0)), 9.0, true),
            around(((-3.0, 3.0), (-3.0, 3.0), (-1.0, 3.0)), 9.0, true),
            inside(((-0.5, 0.5), (-0.5, 0.5), (0.5, 1.5)), 9.0, false),
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
        &[
            far(((5.0, 6.0), (5.0, 6.0), (5.0, 6.0)), v, true),
            around(((-2.0, 2.0), (-2.0, 2.0), (-1.0, 3.0)), v, true),
            inside(((-0.5, 0.5), (-0.5, 0.5), (0.5, 1.5)), v, false),
            cut(
                "slab through the wall",
                ((-2.0, 2.0), (-0.25, 0.25), (0.75, 1.25)),
                false,
            ),
        ],
    )
}

/// The cavity, at rest, and its volume, its own measure.
fn cavity() -> (AtRestBody<f64>, f64) {
    let tol = Tol::witness();
    let (_, cavity) = vessel_cavity(1.0 / 128.0);
    let cavity = AtRestBody::validate(cavity, tol).expect("the cavity passes the at-rest gate");
    let v = topo::test_support::pinned_volume("the cavity measures", &cavity, tol).volume;
    (cavity, v)
}

/// The cavity's placements, in its own frame: the quarter `x ≥ 1/128`,
/// `z ≤ −1/128` of the vessel about `ŷ`, `y ∈ [1/128, 0.3672]`; the neck
/// cylinder of radius 0.1016 over `y ∈ [0.1797, 0.3672]` under the mouth
/// plane `y = 0.3672`; the torus band (centre `y = 0.125`, radii 0.0938
/// and 0.0703) over `y ∈ [0.0703, 0.1797]`, whose rims on the cut planes
/// are the two spirics. Every cut stands clear of the band.
fn cavity_census() -> Vec<String> {
    let (cavity, v) = cavity();
    census(
        "vessel cavity",
        &cavity,
        &[
            far(((5.0, 6.0), (5.0, 6.0), (5.0, 6.0)), v, true),
            around(((-1.0, 1.0), (-1.0, 1.0), (-1.0, 1.0)), v, true),
            inside(((0.03, 0.05), (0.25, 0.30), (-0.05, -0.03)), v, true),
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
    // Pinned red, two ways. At ε = 1e-6 the lofted cylinder's far union
    // builds a volume 3.2e-5 off its closed form
    // (`work/spliceseam/main-red-the-lofted-cylinders-far-union-builds-the-wrong-volume-at-1e-6`); at
    // 1e-12 the bare operand's measurement stops at its wall
    // (`work/spliceseam/the-lofted-cylinder-operand-runs-out-of-quadrature-budget-at-1e-12`).
    let eps = geom_core::Tol::witness().eps();
    let says: &[&str] = if test_utils::kernel_limit::at_row(eps, &[1e-6]) {
        &[
            "lofted cylinder / far / O ∪ B: builds V = 7.283217721708709, the closed form gives 7.283185307179586",
        ]
    } else {
        &["the operand measures: the measurement did not meet its resolution: it stopped at `wall`"]
    };
    test_utils::pinned_red::row(
        eps,
        &[1e-6, 1e-12],
        "the spline and spiric operands",
        says,
        no_spline_or_spiric_operand_builds_wrong_row,
    );
}

/// The row [`no_spline_or_spiric_operand_builds_wrong`] runs, under its pins.
fn no_spline_or_spiric_operand_builds_wrong_row() {
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
    let (cavity, _) = cavity();
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
