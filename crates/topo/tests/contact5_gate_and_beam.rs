//! **The instance arm clears a meeting pair only through the touch
//! analysis.** The box gate answers containment: when neither solid's
//! vertex hull sits inside the other's reach, the pair is cleared at the
//! gate only if nothing on record says the two boundaries meet. A pair
//! whose boundaries meet — any finding with one entity on each side, or
//! a declared record naming one of each — clears only when every touch
//! between them reads as a rest, and a crossing of two edges at one
//! point is such a touch, read through the two edges' dihedral wedges.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use geom_core::{Band, Point3, Tol};
use topo::{
    Body, CensusContact, ContactRecords, EntityId, ValidationError, validate_pseudomanifold,
};

fn block(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    common::brick::<f64>(x, y, z, Tol::witness())
}

/// The parallelepiped `p + u·a + v·b + w·c` over the unit cube
/// (`det[a, b, c] > 0`, so an ordinary solid).
fn parallelepiped(p: [f64; 3], a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> Body<f64> {
    common::mapped_cube(
        move |u, v, w| {
            Point3::new(
                p[0] + u * a[0] + v * b[0] + w * c[0],
                p[1] + u * a[1] + v * b[1] + w * c[1],
                p[2] + u * a[2] + v * b[2] + w * c[2],
            )
        },
        Tol::witness(),
    )
}

fn assembly(parts: &[Body<f64>]) -> Body<f64> {
    let mut out = parts[0].clone();
    for part in &parts[1..] {
        topo::graft_disjoint(&mut out, part, Tol::witness()).unwrap();
    }
    out
}

fn errors_of(body: &Body<f64>) -> Vec<ValidationError> {
    validate_pseudomanifold(body, &ContactRecords::default(), Tol::witness())
        .err()
        .unwrap_or_default()
}

/// The findings arm 2 raises on a solid pair.
fn placement_findings(errors: &[ValidationError]) -> Vec<&ValidationError> {
    errors
        .iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::CensusUndecidable {
                    a: EntityId::Solid(_),
                    b: EntityId::Solid(_),
                    ..
                } | ValidationError::InstanceInterference { .. }
            )
        })
        .collect()
}

fn crosses(errors: &[ValidationError]) -> usize {
    errors
        .iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::UndeclaredContact {
                    contact: CensusContact::EdgeEdgeCross { .. },
                    ..
                }
            )
        })
        .count()
}

fn pierces(errors: &[ValidationError]) -> usize {
    errors
        .iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::UndeclaredContact {
                    contact: CensusContact::EdgeFacePierce { .. },
                    ..
                }
            )
        })
        .count()
}

fn escalations(errors: &[ValidationError]) -> usize {
    errors
        .iter()
        .filter(|e| matches!(e, ValidationError::CensusEscalated { .. }))
        .count()
}

/// Every placement refusal's `what`, cut to its reason: the text before
/// the recourse.
fn refusals(errors: &[ValidationError]) -> Vec<&'static str> {
    placement_findings(errors)
        .into_iter()
        .map(|e| match e {
            ValidationError::CensusUndecidable { what, .. } => {
                what.split(", and this one").next().unwrap_or(what)
            }
            other => panic!("a typed refusal, not a decided interference: {other:?}"),
        })
        .collect()
}

const CROSSING: &str = "another finding reports their boundaries crossing";
const UNEXAMINED: &str = "another finding left their boundaries unchecked";

/// **Two cubes side by side, face to face.** `[0, 2]³` and `[2, 4] ×
/// [0, 2]²` share the face `x = 2` with opposite normals, every edge of
/// it collinear with the other's and every corner coincident. The gate
/// separates both orderings and the boundaries meet, so the touch
/// analysis reads every finding: each is a rest across the plane
/// `x = 2`, and the pair clears.
#[test]
fn two_cubes_side_by_side_clear() {
    let body = assembly(&[
        block((0.0, 2.0), (0.0, 2.0), (0.0, 2.0)),
        block((2.0, 4.0), (0.0, 2.0), (0.0, 2.0)),
    ]);
    let errors = errors_of(&body);
    assert!(!errors.is_empty(), "the undeclared touches stand");
    assert!(placement_findings(&errors).is_empty(), "{errors:?}");
}

/// The pose of the beam across two supports: two blocks `[0, 0.1] ×
/// [0, 0.5] × [0, 1]` and `[3, 3.1] × [0, 0.5] × [0, 1]`, and a beam
/// over `x ∈ [−0.2, 3.3]`, `y ∈ [0.1, 0.15]`, lifted by `lift` from
/// resting on their tops at `z = 1`.
fn beam_across_two_supports(lift: f64) -> Body<f64> {
    let beam = parallelepiped(
        [-0.2, 0.1, 1.0 + lift],
        [3.5, 0.0, 0.0],
        [0.0, 0.05, 0.0],
        [0.0, 0.0, 0.05],
    );
    assembly(&[
        block((0.0, 0.1), (0.0, 0.5), (0.0, 1.0)),
        block((3.0, 3.1), (0.0, 0.5), (0.0, 1.0)),
        beam,
    ])
}

/// **A beam resting across two supports clears.** The beam's two
/// bottom edges cross each support's two top edges in the plane `z = 1`, so the
/// sweeps report edge crosses; each is read at the crossing point
/// through the two edges' wedges, which the plane `z = 1` separates.
/// Together with the edges lying in the other's faces, every finding
/// is a rest.
#[test]
fn a_beam_across_two_supports_clears() {
    let errors = errors_of(&beam_across_two_supports(0.0));
    assert_eq!(crosses(&errors), 8, "{errors:?}");
    assert_eq!(pierces(&errors), 0, "{errors:?}");
    assert!(placement_findings(&errors).is_empty(), "{errors:?}");
}

/// **The same beam sunk a millimetre into its supports refuses.** Its
/// bottom edges now pierce the supports' sides and the supports' top
/// edges pierce its sides, so the boundaries meet in crossings and
/// both beam × support pairs refuse on them.
#[test]
fn a_beam_sunk_into_its_supports_refuses() {
    let errors = errors_of(&beam_across_two_supports(-0.001));
    assert!(pierces(&errors) > 0, "{errors:?}");
    assert_eq!(refusals(&errors), [CROSSING, CROSSING], "{errors:?}");
}

/// **A beam tilted in band about one bottom edge.** The beam pivots on
/// its bottom edge `y = 0.1` so its other bottom edge rises by a height
/// inside the run band: whether that edge's line meets the supports'
/// top edges is too close to call, so the sweep escalates there, and
/// every beam × support pair — whose boundaries meet at the pivot
/// edge's crosses — refuses on the escalation rather than clearing.
#[test]
fn a_beam_tilted_in_band_escalates() {
    let band = Band::linear(Tol::witness()).unwrap();
    let rise = (band.zero() * band.escalate()).sqrt();
    let beam = parallelepiped(
        [-0.2, 0.1, 1.0],
        [3.5, 0.0, 0.0],
        [0.0, 0.05, rise],
        [0.0, 0.0, 0.05],
    );
    let body = assembly(&[
        block((0.0, 0.1), (0.0, 0.5), (0.0, 1.0)),
        block((3.0, 3.1), (0.0, 0.5), (0.0, 1.0)),
        beam,
    ]);
    let errors = errors_of(&body);
    assert!(escalations(&errors) > 0, "{errors:?}");
    assert!(
        crosses(&errors) > 0,
        "the pivot edge still crosses: {errors:?}"
    );
    assert_eq!(refusals(&errors), [UNEXAMINED, UNEXAMINED], "{errors:?}");
}

/// Two square bars turned 45° about their axes and crossed ridge on
/// ridge: the lower bar's top edge runs along `x` at `y = 0, z = 2`, the
/// upper bar's bottom edge along `y` at `x = 0, z = 2 + lift`.
fn crossed_ridges(lift: f64) -> Body<f64> {
    assembly(&[
        parallelepiped(
            [-1.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [0.0, 1.0, 1.0],
            [0.0, -1.0, 1.0],
        ),
        parallelepiped(
            [0.0, -1.0, 2.0 + lift],
            [0.0, 2.0, 0.0],
            [-1.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
        ),
    ])
}

/// **Two ridges crossed edge on edge clear; sunk, they refuse.** No
/// face of either bar lies in the plane the two ridges span, and the
/// crossing is still a touch: the plane `z = 2` through both ridges
/// separates the two wedges. Lowered by a centimetre, the upper ridge
/// passes through the lower bar's slopes and the pair refuses.
#[test]
fn crossed_ridges_clear_resting_and_refuse_sunk() {
    let errors = errors_of(&crossed_ridges(0.0));
    assert_eq!(crosses(&errors), 1, "{errors:?}");
    assert!(placement_findings(&errors).is_empty(), "{errors:?}");
    let errors = errors_of(&crossed_ridges(-0.01));
    assert!(pierces(&errors) > 0, "{errors:?}");
    assert_eq!(refusals(&errors), [CROSSING], "{errors:?}");
}

/// Whether two closed intervals overlap in a positive length.
fn overlaps(a: (f64, f64), b: (f64, f64)) -> bool {
    a.1.min(b.1) - a.0.max(b.0) > 0.0
}

/// **A sweep of axis-aligned brick pairs on a unit grid.** The cube
/// `[0, 2]³` against every brick whose sides run between the grid
/// values `−1, 0, 1, 2, 3` on each axis — a thousand poses, every one
/// with the two boundaries meeting, and with faces coplanar and edges
/// collinear or crossing in plane wherever the grid lines coincide.
/// Two bricks' materials overlap exactly when their intervals overlap
/// in a positive length on all three axes, which is the ground truth.
/// No pose that overlaps may clear. A pose that does not overlap is a
/// rest, and every one of those clears.
#[test]
fn a_grid_of_brick_pairs_clears_exactly_the_rests() {
    let grid = [-1.0, 0.0, 1.0, 2.0, 3.0];
    let spans: Vec<(f64, f64)> = grid
        .iter()
        .enumerate()
        .flat_map(|(i, &lo)| grid[i + 1..].iter().map(move |&hi| (lo, hi)))
        .collect();
    let a = (0.0, 2.0);
    let (mut wrong_clears, mut false_refusals, mut overlapping, mut rests) =
        (Vec::new(), Vec::new(), 0, 0);
    for &x in &spans {
        for &y in &spans {
            for &z in &spans {
                let body = assembly(&[block(a, a, a), block(x, y, z)]);
                let errors = errors_of(&body);
                let refused = !placement_findings(&errors).is_empty();
                let overlap = overlaps(a, x) && overlaps(a, y) && overlaps(a, z);
                if overlap {
                    overlapping += 1;
                    if !refused {
                        wrong_clears.push((x, y, z));
                    }
                } else {
                    rests += 1;
                    if refused {
                        false_refusals.push(((x, y, z), refusals(&errors)));
                    }
                }
            }
        }
    }
    println!(
        "grid sweep: {overlapping} overlapping, {rests} rests; wrong clears {}, false refusals {}",
        wrong_clears.len(),
        false_refusals.len()
    );
    assert_eq!(overlapping + rests, 1000);
    assert!(wrong_clears.is_empty(), "{wrong_clears:?}");
    assert!(false_refusals.is_empty(), "{false_refusals:?}");
}
