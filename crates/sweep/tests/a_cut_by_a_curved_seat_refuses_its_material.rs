//! **A cut by a curved seat refuses to read it as material**
//! (`crates/topo/README.md`, `[topo.cut-material]`): a cut reads a tool
//! whose solids rest on each other as their material, fused on the
//! face–face records between them, and a record of two planar faces is
//! an area, one plane with the materials on its two sides. A record of
//! curved faces may be an area or a tangency along a line, which the
//! record does not tell apart, so the cut refuses `ToolSeatUnread`
//! naming it, never glued on a class the record does not state and never
//! decided again.
//!
//! The seats are a shaft in a bore: the plate `[0, 2]² × [0, 0.5]` bored
//! at `(1, 1)` with radius 0.4, the shaft of that radius through it
//! (`z` −0.5 to 1.0) or flush with it (`z` 0 to 0.5), turned so its arcs'
//! ends lie off the bore's, the two one body of two solids. Their walls
//! are handed in as records, as no census records a curved seat yet
//! (`work/atrest/the-census-cannot-record-a-curved-seat-between-two-solids.md`).
//! The block `[0.4, 1.6]² × [−0.2, 0.7]` cut by either covers the
//! whole seat with no vertex on it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use sweep::test_support::{bored_block_of_arcs, cylinder_of_arcs_at, finished};
use topo::{
    AtRestBody, BooleanError, BooleanInputs, BooleanOp, Cell, FaceKey, OperandRecords, SolidKey,
};

fn t() -> Tol {
    Tol::witness()
}

/// The plate and the shaft from `z0`, `h` tall, one body of two solids,
/// and a record of every pair of their walls, one of each solid.
fn seat(z0: f64, h: f64) -> (AtRestBody<f64>, OperandRecords) {
    let mut body = bored_block_of_arcs(2, 2.0, 0.5, 0.4, t());
    let shaft = cylinder_of_arcs_at(2, 0.4, Point2::new(1.0, 1.0), z0, h, t());
    let turn = geom_core::RigidMotion::rotation_then_translation(
        geom_core::UnitVec3::exact_z(),
        core::f64::consts::FRAC_PI_2,
        geom_core::Vec3::new(2.0, 0.0, 0.0),
    )
    .unwrap();
    let shaft = topo::transform_rigid(&shaft, &turn).unwrap();
    topo::graft_disjoint_all_keyed(&mut body, &shaft).unwrap();
    let body = finished("the seat", body, t());
    let walls: Vec<(FaceKey, SolidKey)> = body
        .faces()
        .map(|(f, _)| f)
        .filter(|&f| body.face_surface(f).unwrap().kind() == geom::SurfaceKind::Cylinder)
        .map(|f| (f, body.solid_of_face(f).unwrap()))
        .collect();
    let mut records = OperandRecords::default();
    for &(x, sx) in &walls {
        for &(y, sy) in &walls {
            if x < y && sx != sy {
                records.ff.push(topo::CarriedRecord {
                    contact: topo::FfContact { a: x, b: y },
                    record: u32::try_from(records.ff.len()).unwrap(),
                });
            }
        }
    }
    assert!(!records.ff.is_empty(), "the premise: the walls meet");
    (body, records)
}

/// **A cut by a curved seat refuses `ToolSeatUnread` naming its first
/// record**, the shaft through the bore and the shaft flush with it.
/// Red where a curved record is glued as `Rest` or as any other class,
/// or where the cut runs as though the tool's solids did not rest on
/// each other (one solid where two belong, its walls held apart inside
/// one shell).
#[test]
fn a_cut_by_a_curved_seat_refuses_its_material() {
    let block = finished(
        "block",
        topo::test_support::brick::<f64>((0.4, 1.6), (0.4, 1.6), (-0.2, 0.7), t()),
        t(),
    );
    for (what, z0, h) in [("through", -0.5, 1.5), ("flush", 0.0, 0.5)] {
        let (seat, records) = seat(z0, h);
        let first = records.ff[0].contact;
        let r = topo::boolean_op_with(
            BooleanOp::Subtract,
            &block,
            &seat,
            &BooleanInputs {
                records_a: OperandRecords::default(),
                records_b: records,
            },
            topo::SweepStrategy::Realized,
            t(),
        );
        match r {
            Err(BooleanError::ToolSeatUnread { record, cells }) => {
                assert_eq!(record, 0, "{what}: the first record");
                assert_eq!(
                    cells,
                    [Cell::Face(first.a), Cell::Face(first.b)],
                    "{what}: its two walls"
                );
            }
            other => panic!(
                "{what}: refuses ToolSeatUnread, got {:?}",
                other.map(|_| ())
            ),
        }
    }
}
