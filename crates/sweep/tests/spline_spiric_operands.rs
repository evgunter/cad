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
//!   The loft prism builds every op against a brick far off and a brick
//!   around it; the lofted cylinder builds them but for the far brick's
//!   union and the brick around it less the cylinder, which refuse at its
//!   own rims (`the_lofted_cylinders_unions_refuse_at_its_own_rims`).
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
use topo::{AtRestBody, BooleanError};

use crate::common::op_census::{
    Run, assert_one_meet, band, kernel_invariant, measure, names_its_own_face, result_volume,
    six_runs,
};
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
        let mut implied = Vec::new();
        for Run {
            op,
            pair,
            keep,
            got,
        } in six_runs(operand, &brick)
        {
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
                    let (got, got_pad) = result_volume(&format!("{line}the result"), &result);
                    match p.oracle {
                        Oracle::Known(of) => {
                            let want = of(v, vb)[keep.slot()];
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
                        Oracle::Cut => implied.push((op, keep.meet(v, vb, got), got_pad)),
                    }
                    lines.push(format!("{line}builds V = {got}"));
                }
            }
        }
        assert_one_meet(&format!("{name} / {}", p.name), &implied, v, v_pad, vb);
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
            // Its union refuses at its own rims
            // (`the_lofted_cylinders_unions_refuse_at_its_own_rims`).
            far(((5.0, 6.0), (5.0, 6.0), (5.0, 6.0)), false),
            // Its `B ∖ O` refuses at its own rims too.
            around(((-2.0, 2.0), (-2.0, 2.0), (-1.0, 3.0)), false),
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

/// **The lofted cylinder's union with a far brick, in both orders, and a
/// brick around it less the cylinder, refuse at the rider reading**
/// (`splice/an-unread-rider-related-only-by-its-own-vertices-refuses`).
/// The slit pass asks each circle rim arc against the lofted cylinder's
/// OTHER spline wall, which shares the arc's two vertices in the
/// cylinder's own topology and nothing else: the arc's place on a spline
/// face is not read, so nothing says it lies along no piece of that wall
/// (`EdgeInFace { SplineFace }`). Pinned in the far `O ∪ B` by identity: the edge is
/// a circle arc of the cylinder bounding a plane cap and a spline wall,
/// the face a spline wall of the cylinder that is not one of the arc's
/// faces and holds both its vertices. In the far `B ∪ O` and the around
/// `B ∖ O` the keys are the result's, so the variant and gap are pinned.
/// The other ops build (the census row above).
#[test]
fn the_lofted_cylinders_unions_refuse_at_its_own_rims() {
    let tol = Tol::witness();
    let cyl = finished("the lofted cylinder", lofted_cylinder(2.0, tol), tol);
    let brick = finished(
        "the far brick",
        brick_with_fresh_keys::<f64>((5.0, 6.0), (5.0, 6.0), (5.0, 6.0), tol),
        tol,
    );
    let gap = topo::EdgeInFaceGap::SplineFace;
    match topo::union(&cyl, &brick, tol) {
        Err(BooleanError::EdgeInFace { edge, face, gap: g }) if g == gap => {
            let kind = |f: topo::FaceKey| {
                cyl.get_face(f)
                    .and_then(|d| cyl.get_surface(d.surface))
                    .map(geom::Surface::kind)
            };
            let sides = topo::readback::edge_sides(&cyl, edge).expect("the edge is the cylinder's");
            let (fa, fb) = sides.faces();
            let mut own = [kind(fa), kind(fb)];
            own.sort_by_key(|k| format!("{k:?}"));
            assert_eq!(
                topo::query::edge_carrier_kind(&cyl, edge),
                Some(topo::query::CurveKind::Circle),
                "the edge is a rim arc"
            );
            assert_eq!(
                own,
                [Some(SurfaceKind::Nurbs), Some(SurfaceKind::Plane)],
                "the arc bounds a spline wall and a cap"
            );
            assert!(
                kind(face) == Some(SurfaceKind::Nurbs) && face != fa && face != fb,
                "the face is the cylinder's other spline wall"
            );
            let e = cyl.get_edge(edge).expect("the arc");
            let start = |h| cyl.get_half_edge(h).expect("its half-edge").start;
            let (a, b) = (start(e.he_plus), start(e.he_minus));
            let of = |v| cyl.faces_of_vertex(v).unwrap_or_default();
            assert!(
                of(a).contains(&face) && of(b).contains(&face),
                "the wall holds both the arc's vertices"
            );
        }
        other => panic!(
            "O ∪ B: the rim arc against the other wall, got {:?}",
            other.err()
        ),
    }
    let around = finished(
        "the brick around it",
        brick_with_fresh_keys::<f64>((-2.0, 2.0), (-2.0, 2.0), (-1.0, 3.0), tol),
        tol,
    );
    for (what, got) in [
        ("far B ∪ O", topo::union(&brick, &cyl, tol)),
        ("around B ∖ O", topo::subtract(&around, &cyl, tol)),
    ] {
        assert!(
            matches!(&got, Err(BooleanError::EdgeInFace { gap: g, .. }) if *g == gap),
            "{what}: {:?}",
            got.err()
        );
    }
}
