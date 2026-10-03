//! **Spline and spiric operand edges reach the Boolean, and no body
//! that the deleted operand gate refused now builds wrong.**
//!
//! `topo`'s body-scoped edge gate refused every operand carrying a
//! spiric or NURBS edge. With it gone, each lane that reads an edge's
//! carrier refuses one it has no row for at its own site. This census
//! runs every population that carries such edges through every op,
//! against a brick in five placements, in both operand orders, and
//! holds each outcome to the oracle: an `Ok` must carry the volume the
//! closed form gives (the prism's `V = 9 m³` exactly,
//! `test_support::loft_prism`'s derivation, and the brick's product of
//! sides); an `Err` must be a typed refusal, never a kernel invariant.
//!
//! - the loft prism: four NURBS walls, two planar caps, NURBS seams;
//! - the vessel's cavity (`common::torus_walls::vessel_cavity`):
//!   analytic faces, spiric rims. Its volume is an elliptic integral,
//!   so an `Ok` on it has no oracle here and fails the row.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use topo::{Body, BooleanError, BooleanResult};

use crate::common::torus_walls::vessel_cavity;

/// A brick's placement against the operand, and the oracle volumes of
/// `operand ∪ brick`, `operand ∩ brick`, `operand ∖ brick` and
/// `brick ∖ operand` where a closed form gives them.
struct Placement {
    name: &'static str,
    brick: ((f64, f64), (f64, f64), (f64, f64)),
    oracle: Option<[f64; 4]>,
}

fn brick_volume(((x0, x1), (y0, y1), (z0, z1)): ((f64, f64), (f64, f64), (f64, f64))) -> f64 {
    (x1 - x0) * (y1 - y0) * (z1 - z0)
}

/// The prism's bounding box is `[−1.375, 1.375] × [−1, 1] × [0, 2]`; its
/// section at `z ∈ [0.5, 1.5]` holds `[−1, 1]²`.
fn prism_placements() -> Vec<Placement> {
    const V: f64 = 9.0;
    let far = ((5.0, 6.0), (5.0, 6.0), (5.0, 6.0));
    let around = ((-3.0, 3.0), (-3.0, 3.0), (-1.0, 3.0));
    let inside = ((-0.5, 0.5), (-0.5, 0.5), (0.5, 1.5));
    let (vf, va, vi) = (
        brick_volume(far),
        brick_volume(around),
        brick_volume(inside),
    );
    vec![
        Placement {
            name: "far",
            brick: far,
            oracle: Some([V + vf, 0.0, V, vf]),
        },
        Placement {
            name: "around",
            brick: around,
            oracle: Some([va, V, 0.0, va - V]),
        },
        Placement {
            name: "inside",
            brick: inside,
            oracle: Some([V, vi, V - vi, 0.0]),
        },
        Placement {
            name: "slab through the walls",
            brick: ((-3.0, 3.0), (-0.25, 0.25), (0.75, 1.25)),
            oracle: None,
        },
        Placement {
            name: "peg through the top cap",
            brick: ((-0.25, 0.25), (-0.25, 0.25), (1.75, 2.5)),
            oracle: None,
        },
    ]
}

/// The cavity's placements: far from it, and around it.
fn cavity_placements(cavity: &Body<f64>) -> Vec<Placement> {
    let (lo, hi) = bbox(cavity);
    let pad = 1.0;
    vec![
        Placement {
            name: "far",
            brick: (
                (hi.x + 5.0, hi.x + 6.0),
                (hi.y + 5.0, hi.y + 6.0),
                (hi.z + 5.0, hi.z + 6.0),
            ),
            oracle: None,
        },
        Placement {
            name: "around",
            brick: (
                (lo.x - pad, hi.x + pad),
                (lo.y - pad, hi.y + pad),
                (lo.z - pad, hi.z + pad),
            ),
            oracle: None,
        },
    ]
}

/// The box of `body`'s points (its vertices); a far or enclosing brick
/// is placed with a metre of margin, which no rim bulges past.
fn bbox(body: &Body<f64>) -> (geom_core::Point3<f64>, geom_core::Point3<f64>) {
    let mut lo = geom_core::Point3::new(f64::MAX, f64::MAX, f64::MAX);
    let mut hi = geom_core::Point3::new(f64::MIN, f64::MIN, f64::MIN);
    for (_, v) in body.vertices() {
        let p = *body.get_point(v.point).expect("a live point");
        lo = geom_core::Point3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z));
        hi = geom_core::Point3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z));
    }
    (lo, hi)
}

/// One op's label, outcome and oracle volume.
type Run = (
    &'static str,
    Result<BooleanResult<f64>, BooleanError>,
    Option<f64>,
);

/// A refusal that names an input or a frontier, not a broken kernel.
fn typed(e: &BooleanError) -> bool {
    !matches!(
        e,
        BooleanError::ClassificationInvariant { .. } | BooleanError::JoinDesync { .. }
    )
}

/// Runs the four ops in both orders and holds each outcome to the
/// oracle; returns the census lines.
fn census(name: &str, operand: &Body<f64>, placements: &[Placement]) -> Vec<String> {
    let tol = Tol::witness();
    let mut lines = Vec::new();
    for p in placements {
        let (x, y, z) = p.brick;
        let brick: Body<f64> = sweep::test_support::brick(x, y, z, tol);
        let runs: [Run; 6] = [
            (
                "O ∪ B",
                topo::union(operand, &brick, tol),
                p.oracle.map(|o| o[0]),
            ),
            (
                "B ∪ O",
                topo::union(&brick, operand, tol),
                p.oracle.map(|o| o[0]),
            ),
            (
                "O ∩ B",
                topo::intersect(operand, &brick, tol),
                p.oracle.map(|o| o[1]),
            ),
            (
                "B ∩ O",
                topo::intersect(&brick, operand, tol),
                p.oracle.map(|o| o[1]),
            ),
            (
                "O ∖ B",
                topo::subtract(operand, &brick, tol),
                p.oracle.map(|o| o[2]),
            ),
            (
                "B ∖ O",
                topo::subtract(&brick, operand, tol),
                p.oracle.map(|o| o[3]),
            ),
        ];
        for (op, got, want) in runs {
            let line = format!("{name} / {} / {op}: ", p.name);
            match got {
                Err(e) => {
                    assert!(typed(&e), "{line}a kernel invariant, not a refusal: {e:?}");
                    lines.push(format!("{line}refuses {:?}", e.kind()));
                }
                Ok(result) => {
                    let Some(want) = want else {
                        panic!(
                            "{line}builds with no oracle to hold it to: {:?}",
                            result.body().map(|b| b.kind)
                        );
                    };
                    let volume = match result.body() {
                        None => 0.0,
                        Some(b) => {
                            topo::mass_properties(&b.body, tol)
                                .unwrap_or_else(|e| {
                                    panic!("{line}builds a body with no volume: {e:?}")
                                })
                                .volume
                        }
                    };
                    assert!(
                        (volume - want).abs() <= 1e-6 * want.max(1.0),
                        "{line}builds V = {volume}, the closed form gives {want}"
                    );
                    lines.push(format!("{line}builds V = {volume} (oracle {want})"));
                }
            }
        }
    }
    lines
}

#[test]
fn no_spline_or_spiric_operand_builds_wrong_without_the_edge_gate() {
    let prism = sweep::test_support::loft_prism(Tol::witness());
    let (_, cavity) = vessel_cavity(1.0 / 128.0);
    let mut lines = census("loft prism", &prism, &prism_placements());
    lines.extend(census(
        "vessel cavity",
        &cavity,
        &cavity_placements(&cavity),
    ));
    for line in &lines {
        println!("{line}");
    }
}
