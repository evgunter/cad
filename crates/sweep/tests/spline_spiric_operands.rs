//! **Spline and spiric operand edges reach the Boolean, and none builds
//! wrong.** No gate refuses an operand for its edge carriers: each lane
//! that reads one refuses a kind it has no row for at its own site,
//! naming the edge and the face. This census runs every population
//! carrying such edges through every op, against bricks in several
//! placements, in both operand orders, and holds each outcome to an
//! oracle:
//!
//! - an `Ok` carries the volume the oracle gives;
//! - an `Err` is a typed refusal, never a kernel invariant, and a
//!   refusal naming an operand's face names a face of that operand
//!   whose kind it states.
//!
//! The populations:
//!
//! - the loft prism (`sweep::test_support::loft_prism`): four NURBS
//!   walls, two planar caps, NURBS seams, `V = 9 m³` exactly;
//! - the lofted cylinder (`lofted_cylinder`): rational NURBS walls and
//!   seams, `V = π·h`;
//! - the vessel's cavity (`common::torus_walls::vessel_cavity`):
//!   analytic faces, spiric rims. Its volume is an elliptic integral,
//!   so its oracle is additivity over its own measure: a far brick
//!   unions to the sum, an enclosing one to itself.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use topo::{AtRestBody, BooleanError, BooleanResult, Operand};

use crate::common::torus_walls::vessel_cavity;
use sweep::test_support::{brick, finished, loft_prism, lofted_cylinder};

type Box3 = ((f64, f64), (f64, f64), (f64, f64));

/// A brick's placement, and the oracle volumes of `O ∪ B`, `O ∩ B`,
/// `O ∖ B` and `B ∖ O` where one is known.
struct Placement {
    name: &'static str,
    brick: Box3,
    oracle: Option<[f64; 4]>,
}

fn volume_of_box(((x0, x1), (y0, y1), (z0, z1)): Box3) -> f64 {
    (x1 - x0) * (y1 - y0) * (z1 - z0)
}

/// `far` disjoint from an operand of volume `v`, `around` enclosing it,
/// `inside` within it where it holds one; then the placements that cut
/// it, with no oracle.
fn placements(
    v: f64,
    far: Box3,
    around: Box3,
    inside: Option<Box3>,
    cuts: &[(&'static str, Box3)],
) -> Vec<Placement> {
    let (vf, va) = (volume_of_box(far), volume_of_box(around));
    let mut out = vec![
        Placement {
            name: "far",
            brick: far,
            oracle: Some([v + vf, 0.0, v, vf]),
        },
        Placement {
            name: "around",
            brick: around,
            oracle: Some([va, v, 0.0, va - v]),
        },
    ];
    if let Some(inside) = inside {
        let vi = volume_of_box(inside);
        out.push(Placement {
            name: "inside",
            brick: inside,
            oracle: Some([v, vi, v - vi, 0.0]),
        });
    }
    out.extend(cuts.iter().map(|&(name, brick)| Placement {
        name,
        brick,
        oracle: None,
    }));
    out
}

/// The refusals that name a broken kernel rather than an input or a
/// frontier.
fn kernel_invariant(e: &BooleanError) -> bool {
    matches!(
        e,
        BooleanError::ClassificationInvariant { .. } | BooleanError::JoinDesync { .. }
    ) || matches!(e, BooleanError::Join(j) if matches!(j.error(), topo::SplitJoinError::SectionInvariant { .. }))
}

/// Where a refusal names an operand's face, that it is a face of that
/// operand, of the kind it states.
fn names_its_own_face(e: &BooleanError, [a, b]: [&AtRestBody<f64>; 2]) -> Result<(), String> {
    let body = |operand: &Operand| match operand {
        Operand::A => a,
        Operand::B => b,
    };
    let kind_of = |operand: &Operand, face| {
        body(operand)
            .get_face(face)
            .and_then(|f| body(operand).get_surface(f.surface))
            .map(geom::Surface::kind)
    };
    match e {
        BooleanError::CurvedBooleanUnsupported {
            operand,
            face,
            kind,
        } => (kind_of(operand, *face) == Some(*kind))
            .then_some(())
            .ok_or(format!("{face:?} is no {kind:?} face of {operand:?}")),
        BooleanError::EdgeCarrierUnsupported {
            operand,
            edge,
            face,
            site,
        } => {
            let curved = matches!(
                topo::query::edge_carrier_kind(body(operand), *edge),
                Some(topo::query::CurveKind::Nurbs | topo::query::CurveKind::Spiric)
            );
            let other = match site {
                topo::EdgeCarrierSite::PlanarCrossing | topo::EdgeCarrierSite::CurvedCrossing => {
                    operand.other()
                }
                _ => *operand,
            };
            (curved && kind_of(&other, *face).is_some())
                .then_some(())
                .ok_or(format!(
                    "{edge:?} of {operand:?} against {face:?} at {site:?}"
                ))
        }
        _ => Ok(()),
    }
}

/// Runs the four ops in both orders and holds each outcome to the
/// oracle; returns the census lines.
fn census(name: &str, operand: &AtRestBody<f64>, placements: &[Placement]) -> Vec<String> {
    let tol = Tol::witness();
    let mut lines = Vec::new();
    for p in placements {
        let (x, y, z) = p.brick;
        let brick = finished("the brick", brick::<f64>(x, y, z, tol), tol);
        let o = |i: usize| p.oracle.map(|v| v[i]);
        let runs = [
            (
                "O ∪ B",
                [operand, &brick],
                topo::union(operand, &brick, tol),
                o(0),
            ),
            (
                "B ∪ O",
                [&brick, operand],
                topo::union(&brick, operand, tol),
                o(0),
            ),
            (
                "O ∩ B",
                [operand, &brick],
                topo::intersect(operand, &brick, tol),
                o(1),
            ),
            (
                "B ∩ O",
                [&brick, operand],
                topo::intersect(&brick, operand, tol),
                o(1),
            ),
            (
                "O ∖ B",
                [operand, &brick],
                topo::subtract(operand, &brick, tol),
                o(2),
            ),
            (
                "B ∖ O",
                [&brick, operand],
                topo::subtract(&brick, operand, tol),
                o(3),
            ),
        ];
        for (op, pair, got, want) in runs {
            let line = format!("{name} / {} / {op}: ", p.name);
            match got {
                Err(e) => {
                    assert!(
                        !kernel_invariant(&e),
                        "{line}a kernel invariant, not a refusal: {e:?}"
                    );
                    if let Err(why) = names_its_own_face(&e, pair) {
                        panic!("{line}the refusal misnames its subject ({why}): {e:?}");
                    }
                    lines.push(format!("{line}refuses {e:?}"));
                }
                Ok(result) => {
                    let Some(want) = want else {
                        panic!("{line}builds with no oracle to hold it to");
                    };
                    let volume = match &result {
                        BooleanResult::Empty => 0.0,
                        BooleanResult::Body(b) => {
                            topo::mass_properties(&b.body, tol)
                                .unwrap_or_else(|e| {
                                    panic!("{line}builds a body with no volume: {e:?}")
                                })
                                .volume
                        }
                    };
                    assert!(
                        (volume - want).abs() <= 1e-6 * want.max(1.0),
                        "{line}builds V = {volume}, the oracle gives {want}"
                    );
                    lines.push(format!("{line}builds V = {volume} (oracle {want})"));
                }
            }
        }
    }
    lines
}

#[test]
fn no_spline_or_spiric_operand_builds_wrong() {
    let tol = Tol::witness();
    let far = ((5.0, 6.0), (5.0, 6.0), (5.0, 6.0));
    let mut lines = Vec::new();

    // The prism's box is `[−1.375, 1.375] × [−1, 1] × [0, 2]`; its
    // section at `z ∈ [0.5, 1.5]` holds `[−1, 1]²`.
    let prism = finished("the loft prism", loft_prism(tol), tol);
    lines.extend(census(
        "loft prism",
        &prism,
        &placements(
            9.0,
            far,
            ((-3.0, 3.0), (-3.0, 3.0), (-1.0, 3.0)),
            Some(((-0.5, 0.5), (-0.5, 0.5), (0.5, 1.5))),
            &[
                (
                    "slab through the walls",
                    ((-3.0, 3.0), (-0.25, 0.25), (0.75, 1.25)),
                ),
                (
                    "peg through the top cap",
                    ((-0.25, 0.25), (-0.25, 0.25), (1.75, 2.5)),
                ),
            ],
        ),
    ));

    // The unit-radius lofted cylinder, `z ∈ [0, 2]`.
    let h = 2.0;
    let cylinder = finished("the lofted cylinder", lofted_cylinder(h, tol), tol);
    lines.extend(census(
        "lofted cylinder",
        &cylinder,
        &placements(
            core::f64::consts::PI * h,
            far,
            ((-2.0, 2.0), (-2.0, 2.0), (-1.0, 3.0)),
            Some(((-0.5, 0.5), (-0.5, 0.5), (0.5, 1.5))),
            &[(
                "slab through the wall",
                ((-2.0, 2.0), (-0.25, 0.25), (0.75, 1.25)),
            )],
        ),
    ));

    // The cavity, placed off its own vertices' box (no rim bulges a
    // metre past it), and its volume its own measure.
    let (_, cavity) = vessel_cavity(1.0 / 128.0);
    let cavity = AtRestBody::validate(cavity, tol).expect("the cavity passes the at-rest gate");
    let v = topo::mass_properties(&cavity, tol)
        .expect("the cavity measures")
        .volume;
    let (lo, hi) = vertex_box(&cavity);
    let pad = 1.0;
    let mid = |a: f64, b: f64| (a + b) / 2.0;
    lines.extend(census(
        "vessel cavity",
        &cavity,
        &placements(
            v,
            (
                (hi.x + 5.0, hi.x + 6.0),
                (hi.y + 5.0, hi.y + 6.0),
                (hi.z + 5.0, hi.z + 6.0),
            ),
            (
                (lo.x - pad, hi.x + pad),
                (lo.y - pad, hi.y + pad),
                (lo.z - pad, hi.z + pad),
            ),
            None,
            &[(
                "slab through its middle",
                (
                    (lo.x - pad, hi.x + pad),
                    (lo.y - pad, hi.y + pad),
                    (mid(lo.z, hi.z) - 0.01, mid(lo.z, hi.z) + 0.01),
                ),
            )],
        ),
    ));
    for line in &lines {
        println!("{line}");
    }
}

/// The box of `body`'s vertices.
fn vertex_box(body: &topo::Body<f64>) -> (geom_core::Point3<f64>, geom_core::Point3<f64>) {
    let mut lo = geom_core::Point3::new(f64::MAX, f64::MAX, f64::MAX);
    let mut hi = geom_core::Point3::new(f64::MIN, f64::MIN, f64::MIN);
    for (_, v) in body.vertices() {
        let p = *body.get_point(v.point).expect("a live point");
        lo = geom_core::Point3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z));
        hi = geom_core::Point3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z));
    }
    (lo, hi)
}
