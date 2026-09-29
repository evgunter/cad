//! **A curved face in a touch's star is read through its certified
//! reach box**: a box on its side of the candidate plane certifies the
//! face, and a box across it refuses, never deciding a crossing. The
//! rows are a log whose lower arc dips into a wall, declared as four
//! v-on-f records that cleared on their word, and its flat-bottomed
//! twin, a true rest the box reading certifies.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{ArcSweep, bulge_from_center, test_support::bulge_loop};
use sweep::test_support::{brick, extruded, sketch_at};
use topo::{Body, ContactRecords, EntityId, ValidationError, VfContact, validate_pseudomanifold};

/// The log's circle: radius 0.5 about `(2, 1.3)`, through its three
/// joints `(1.6, 1)`, `(2.4, 1)` and `(2, 1.8)`.
const CENTRE: (f64, f64) = (2.0, 1.3);

/// The log along `z ∈ [1, 3]`: the circle through its three joints, or
/// with its lower arc replaced by the chord `y = 1` when `flat`.
fn log(flat: bool) -> Body<f64> {
    let joints = [(1.6, 1.0), (2.4, 1.0), (2.0, 1.8)];
    let centre = Point2::new(CENTRE.0, CENTRE.1);
    let chain = (0..3)
        .map(|i| {
            let (a, b) = (joints[i], joints[(i + 1) % 3]);
            let (a, b) = (Point2::new(a.0, a.1), Point2::new(b.0, b.1));
            let bulge = if flat && i == 0 {
                0.0
            } else {
                bulge_from_center(a, b, centre, ArcSweep::Ccw)
            };
            (a, bulge)
        })
        .collect();
    extruded(sketch_at(1.0), vec![bulge_loop(chain)], 2.0, Tol::witness())
}

/// The wall `[0, 4] × [0, 1] × [0, 4]` with `part` grafted beside it.
fn wall_with(part: &Body<f64>) -> Body<f64> {
    let mut body = brick::<f64>((0.0, 4.0), (0.0, 1.0), (0.0, 4.0), Tol::witness());
    topo::graft_disjoint(&mut body, part, Tol::witness()).unwrap();
    body
}

/// The log's four joints at `y = 1`, declared on the wall's top.
fn joints_on_the_wall_top(body: &Body<f64>) -> ContactRecords {
    let top = body
        .faces()
        .find_map(|(k, f)| match body.get_surface(f.surface) {
            Some(geom::Surface::Plane { origin, normal, .. })
                if (origin.y - 1.0).abs() < 1e-9 && normal.y.abs() > 0.5 =>
            {
                // The wall's top, not the flat log's bottom: its
                // corners reach `x = 0`.
                body.vertices()
                    .filter(|(vk, _)| body.faces_of_vertex(*vk).unwrap().contains(&k))
                    .any(|(_, v)| body.get_point(v.point).unwrap().x.abs() < 1e-9)
                    .then_some(k)
            }
            _ => None,
        })
        .expect("the wall's top");
    let mut records = ContactRecords::default();
    for (vk, v) in body.vertices() {
        let p = body.get_point(v.point).unwrap();
        let joint = (p.y - 1.0).abs() < 1e-9
            && ((p.x - 1.6).abs() < 1e-9 || (p.x - 2.4).abs() < 1e-9)
            && ((p.z - 1.0).abs() < 1e-9 || (p.z - 3.0).abs() < 1e-9);
        if joint {
            records.a_on_b.push(VfContact {
                vertex: vk,
                face: top,
            });
        }
    }
    assert_eq!(records.a_on_b.len(), 4, "the log's four lower joints");
    records
}

/// **A log dipping 0.2 m into a wall, declared by its four joints on
/// the wall's top**, refuses: the lower arc's face and the caps enter
/// each joint's star as their reach boxes, which cross the wall's top
/// plane. It cleared when one point's record excused the whole curved
/// face.
#[test]
fn a_log_dipping_into_a_wall_declared_by_its_joints_refuses() {
    let body = wall_with(&log(false));
    let records = joints_on_the_wall_top(&body);
    let errors = validate_pseudomanifold(&body, &records, Tol::witness())
        .expect_err("the dipping log refuses");
    assert!(
        errors.iter().any(|e| matches!(
            e,
            ValidationError::CensusUndecidable {
                a: EntityId::Face(_) | EntityId::Solid(_),
                ..
            }
        )),
        "{errors:?}"
    );
}

/// **The control: the flat-bottomed log on the wall**, declared the
/// same way, certifies: every curved face's box lies on its side of
/// the wall's top plane.
#[test]
fn a_flat_bottomed_log_on_a_wall_declared_by_its_joints_certifies() {
    let body = wall_with(&log(true));
    let records = joints_on_the_wall_top(&body);
    assert_eq!(
        validate_pseudomanifold(&body, &records, Tol::witness()),
        Ok(())
    );
}
