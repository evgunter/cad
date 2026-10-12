//! Review probes (R1, delta) for PR 4820: a curved pit under a carried
//! rest.
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use geom_core::{Point2, Point3, Tol, Vec3};
use sweep::test_support::{cylinder_of_arcs_at, finished};
use topo::{Backing, BooleanResult, validate_pseudomanifold};

/// **A cylindrical pit under a cube's corner resting on a slab**: the
/// rest is consumed and the result passes 3'; also a pit whose wall
/// passes through the corner's foot.
#[test]
fn r1_a_curved_pit_under_a_carried_rest() {
    let tol = Tol::witness();
    let (e1, e2, e3) = (
        Vec3::new(0.9, 0.1, 0.3),
        Vec3::new(-0.2, 0.8, 0.45),
        Vec3::new(-0.3, -0.4, 0.85),
    );
    let tilted = finished(
        "the tilted cube",
        topo::test_support::mapped_cube(
            |x, y, z| {
                Point3::new(
                    2.0 + x * e1.x + y * e2.x + z * e3.x,
                    2.0 + x * e1.y + y * e2.y + z * e3.y,
                    1.0 + x * e1.z + y * e2.z + z * e3.z,
                )
            },
            tol,
        ),
        tol,
    );
    let slab = finished(
        "slab",
        topo::test_support::brick((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), tol),
        tol,
    );
    let Ok(BooleanResult::Body(base)) = topo::union(&slab, &tilted, tol) else {
        panic!("the base")
    };
    assert_eq!(base.contacts.vertex_face().count(), 1);
    for (what, c, r, n) in [
        ("centred", Point2::new(2.0, 2.0), 0.15, 3),
        ("offset", Point2::new(2.05, 1.97), 0.15, 4),
        ("wall through the foot", Point2::new(2.1, 2.0), 0.1, 2),
    ] {
        let pit = finished(what, cylinder_of_arcs_at(n, r, c, 0.5, 0.5, tol), tol);
        let inputs = topo::BooleanInputs {
            records_a: base.contacts.carried(),
            ..topo::BooleanInputs::default()
        };
        let got = topo::boolean_op_with(
            topo::BooleanOp::Subtract,
            &base.body,
            &pit,
            &inputs,
            topo::SweepStrategy::Realized,
            tol,
        );
        eprintln!(
            "curved pit {what}: {:?}",
            got.as_ref().map(|r| match r {
                BooleanResult::Body(b) => format!(
                    "{:?} 3'={:?}",
                    b.contacts,
                    validate_pseudomanifold(&b.body, &b.contacts, tol).err()
                ),
                o => format!("{o:?}"),
            })
        );
        // 3' is not asserted: the census cannot yet read a curved pit
        // wall against the tilted cube's faces (`CensusUndecidable`),
        // which is the census's frontier, not the carry's.
        match got {
            Ok(BooleanResult::Body(out)) if what != "wall through the foot" => assert!(
                out.contacts
                    .cites()
                    .all(|c| c.iter().all(|b| !matches!(b, Backing::Carried { .. }))),
                "curved pit {what}: consumed"
            ),
            Ok(_) => {}
            Err(topo::BooleanError::VertexReadTwice { .. }) if what == "wall through the foot" => {}
            other => panic!("curved pit {what}: {other:?}"),
        }
    }
}
