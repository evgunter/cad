//! Review 4730 probes: the divided-cell carry's confirm, read at the
//! caller's band, decides, refuses `Unread`, or finds no meeting pair.
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common;
use geom_core::{Band, Point3, Tol, Vec3};
use topo::{AtRestBody, BooleanResult, CarryRefusal, Cell, union, validate_pseudomanifold};

fn tol() -> Tol {
    Tol::witness()
}

fn box_of(x: (f64, f64), y: (f64, f64), z: (f64, f64), what: &str) -> AtRestBody<f64> {
    common::finished(what, common::brick(x, y, z, tol()), tol())
}

/// **A corner 1e-4 past a cut through the edge it rests on.** In `a ∪ c`
/// `c`'s corner (0.5, 0, 0) rests on `a`'s rim x ∈ (0, 1); the plane
/// x = 0.5 − 1e-4 divides that rim, leaving the key on the piece below
/// and the corner 1e-4 inside the piece above. At the default band the
/// record lands on the piece above, beside the far end's record and the
/// touch's edge–edge record, and both sides pass at rest. At a
/// band where 1e-4 is inside the escalation zone the confirm cannot
/// read the pair, and the carry refuses `Unread` (the only row that
/// reaches that refusal). At a band where 1e-4 is Zero the corner reads
/// as at the vertex the cut minted on the rim, and the record lands on
/// that pair: the minted vertex is the rim's image too (D1).
#[test]
fn a_corner_just_past_a_cut_lands_on_the_piece_refuses_unread_or_lands_on_the_minted_vertex_by_band()
 {
    let a = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), "a");
    let c = box_of((0.5, 1.5), (-1.0, 0.0), (-1.0, 0.0), "c");
    let base = match union(&a, &c, tol()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("the union builds: {other:?}"),
    };
    let plane = common::split_plane(
        Point3::new(0.5 - 1e-4, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        tol(),
    );
    let split = topo::split(&base.body, &plane, tol()).expect("splits");
    let region = || Some(topo::RegionLane::certified());
    let sides = split
        .carry_contacts(&base.contacts, Band::linear(tol()).unwrap(), region())
        .expect("the records land at the default band");
    for (side, body, records) in [
        ("above", split.above.body(), &sides.above),
        ("below", split.below.body(), &sides.below),
    ] {
        assert_eq!(
            validate_pseudomanifold(body.expect("material"), records, tol()),
            Ok(()),
            "{side}: {records:?}"
        );
    }
    assert_eq!((sides.above.len(), sides.below.len()), (3, 0), "{sides:?}");
    let corner = base.contacts.rows()[1].cells()[0];
    assert!(matches!(corner, Cell::Vertex(_)), "{:?}", base.contacts);
    let at = |eps: f64| {
        split.carry_contacts(
            &base.contacts,
            Band::linear_at(tol(), eps).unwrap(),
            region(),
        )
    };
    assert!(
        matches!(
            at(1e-5),
            Err(CarryRefusal::Unread { record: 1, cells: [c, Cell::Edge(_)] }) if c == corner
        ),
        "{:?}",
        at(1e-5)
    );
    let coarse = at(1e-3).expect("the record lands on the minted vertex");
    let on_minted: Vec<_> = coarse
        .above
        .rows()
        .iter()
        .filter(|r| {
            matches!(r.cells(), [a, Cell::Vertex(_)] if a == corner)
                && r.cites.iter().eq([topo::Backing::Carried {
                    input: 0,
                    record: 1,
                }])
        })
        .collect();
    assert_eq!(on_minted.len(), 1, "{coarse:?}");
}
