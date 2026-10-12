//! **A blend carries its operand's contact records** through the cells
//! it keeps, and refuses a record on a cell it consumes rather than
//! dropping it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::cavity::{brick, edges_with_corners};
use geom_core::{Point3, Tol};
use sweep::blend::build::{Blended, fillet_edges};
use sweep::chamfer::chamfer_edges;
use sweep::test_support::finished;
use topo::{
    Backing, BooleanBody, BooleanResult, ContactRecords, ValidationError, union,
    validate_pseudomanifold,
};

/// The blend size, meters.
const R: f64 = 0.1;

fn tol() -> Tol {
    Tol::witness()
}

/// Two unit cubes kissing at the corner `(1, 1, 1)`: one vertex–vertex
/// record.
fn kiss() -> BooleanBody<f64> {
    let a = finished(
        "box a",
        brick(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0)),
        tol(),
    );
    let b = finished(
        "box b",
        brick(Point3::new(1.0, 1.0, 1.0), Point3::new(2.0, 2.0, 2.0)),
        tol(),
    );
    match union(&a, &b, tol()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("the union builds: {other:?}"),
    }
}

/// The one edge of `body` between `p` and `q`.
fn edge_between(body: &topo::Body<f64>, p: Point3<f64>, q: Point3<f64>) -> topo::EdgeKey {
    let near = |a: Point3<f64>, b: Point3<f64>| (a - b).norm() < 1e-9;
    let found = edges_with_corners(body, |c| near(c, p) || near(c, q));
    let [e] = found[..] else {
        panic!("one edge between {p:?} and {q:?}: {found:?}")
    };
    e
}

/// The kiss's record carried onto `out`, citing the operand's record 0.
fn carried_kiss(what: &str, base: &BooleanBody<f64>, out: &Blended<f64>) -> ContactRecords {
    let carried = out
        .carry_contacts(
            &base.contacts,
            geom_core::Band::linear(tol()).unwrap(),
            Some(topo::RegionLane::certified()),
        )
        .unwrap();
    assert_eq!(carried.vertex_vertex().count(), 1, "{what}: {carried:?}");
    assert_eq!(carried.len(), 1, "{what}: {carried:?}");
    let cites: Vec<Backing> = carried.rows()[0].cites.iter().collect();
    assert_eq!(
        cites,
        [Backing::Carried {
            input: 0,
            record: 0
        }],
        "{what}: the record cites the operand's"
    );
    carried
}

/// **A fillet and a chamfer away from the kiss carry it.** Red if the
/// blend's carry drops the record or cites anything but the operand's.
/// The chamfer's result passes at rest with the carried record and
/// refuses the kiss as undeclared without it. The fillet's is not read
/// at rest: its curved band lies within reach of the other cube, which
/// the census cannot yet decide
/// (`work/restread/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`).
#[test]
fn a_blend_away_from_the_kiss_carries_it() {
    let base = kiss();
    let edge = edge_between(
        &base.body,
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
    );
    let filleted = fillet_edges(&base.body, &[edge], R, tol()).expect("the fillet builds");
    carried_kiss("fillet", &base, &filleted);

    let chamfered = chamfer_edges(&base.body, &[edge], R, tol()).expect("the chamfer builds");
    let carried = carried_kiss("chamfer", &base, &chamfered);
    assert_eq!(
        validate_pseudomanifold(&chamfered.body, &carried, tol()),
        Ok(()),
        "chamfer: the carried record passes at rest"
    );
    let withheld = validate_pseudomanifold(&chamfered.body, &ContactRecords::default(), tol())
        .expect_err("the kiss is a touch");
    assert!(
        withheld
            .iter()
            .all(|e| matches!(e, ValidationError::UnrecordedContact { .. })),
        "chamfer: withheld, the kiss is undeclared: {withheld:?}"
    );
}

/// **A record on the edge a blend consumes is consumed with it** (D1).
/// The blend removes the edge it replaces with a band, and the band face
/// holds none of the edge's points, so it takes no record from it: the
/// record (bookkeeping against the far cube's top, no touch) lands on no
/// pair, and the lineage's stated removal consumes it, while the kiss
/// carries. Red on the carry that refuses the record `Unmapped`, or keeps
/// it on the excised edge or the band face.
#[test]
fn a_record_on_the_blended_edge_is_consumed() {
    let base = kiss();
    let edge = edge_between(
        &base.body,
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
    );
    let far = base
        .body
        .faces()
        .find(|(_, f)| {
            matches!(
                base.body.get_surface(f.surface),
                Some(topo::Surface::Plane { origin, normal, .. })
                    if normal.z.abs() == 1.0 && origin.z == 2.0
            )
        })
        .map(|(k, _)| k)
        .expect("the far cube's top");
    let mut records = base.contacts.clone();
    records.extend([topo::Cited::new(
        topo::Contact::new(&base.body, topo::Cell::Edge(edge), topo::Cell::Face(far)).unwrap(),
        topo::Cites::decided(7),
    )]);
    for (what, out) in [
        (
            "fillet",
            fillet_edges(&base.body, &[edge], R, tol()).expect("the fillet builds"),
        ),
        (
            "chamfer",
            chamfer_edges(&base.body, &[edge], R, tol()).expect("the chamfer builds"),
        ),
    ] {
        assert_eq!(
            out.lineage().fate(topo::Cell::Edge(edge)),
            topo::Fate::Removed,
            "{what}: the band removes the edge"
        );
        let carried = out
            .carry_contacts(
                &records,
                geom_core::Band::linear(tol()).unwrap(),
                Some(topo::RegionLane::certified()),
            )
            .unwrap_or_else(|refusal| panic!("{what}: {refusal:?}"));
        assert_eq!(
            carried
                .cites()
                .map(|c| c.iter().collect())
                .collect::<Vec<Vec<_>>>(),
            [vec![topo::Backing::Carried {
                input: 0,
                record: 0
            }]],
            "{what}: the kiss alone carries"
        );
    }
}

/// **A turn's third edge lands a resting corner's record on the piece
/// the blend left.** In `a ∪ c` (bricks touching along an edge), `c`'s
/// corner (0.5, 0, 0) rests on `a`'s rim x ∈ (0, 1). Blending `a`'s two
/// edges at the origin along y and z turns there, and the band splits
/// the third edge, `a`'s rim, at the turn's foot: the piece at the turn is
/// carved away under the rim's key, and the surviving piece is a fresh
/// key (`BlendNaming::meridian_remnants`). The record lands on that
/// piece, and the result passes at rest. Red on the carry that reads the
/// rim by its key: the key is dead, and the record is refused
/// `Unmapped`.
#[test]
fn a_turns_third_edge_lands_a_resting_corners_record_on_the_surviving_piece() {
    let a = finished(
        "a",
        brick(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0)),
        tol(),
    );
    let c = finished(
        "c",
        brick(Point3::new(0.5, -1.0, -1.0), Point3::new(1.5, 0.0, 0.0)),
        tol(),
    );
    let base = match union(&a, &c, tol()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("the union builds: {other:?}"),
    };
    assert_eq!(
        base.contacts.vertex_edge().count(),
        2,
        "{:?}",
        base.contacts
    );
    let corner = Point3::new(0.0, 0.0, 0.0);
    let y = edge_between(&base.body, corner, Point3::new(0.0, 1.0, 0.0));
    let z = edge_between(&base.body, corner, Point3::new(0.0, 0.0, 1.0));
    let band = geom_core::Band::linear(tol()).unwrap();
    for (what, out) in [
        (
            "fillet",
            fillet_edges(&base.body, &[y, z], R, tol()).expect("the fillet builds"),
        ),
        (
            "chamfer",
            chamfer_edges(&base.body, &[y, z], R, tol()).expect("the chamfer builds"),
        ),
    ] {
        let remnants = &out
            .naming
            .as_ref()
            .expect("birth records")
            .meridian_remnants;
        let carried = out
            .carry_contacts(&base.contacts, band, Some(topo::RegionLane::certified()))
            .unwrap_or_else(|refusal| panic!("{what}: the records land: {refusal:?}"));
        let before: Vec<_> = base.contacts.vertex_edge().collect();
        let after: Vec<_> = carried.vertex_edge().collect();
        assert_eq!(after.len(), 2, "{what}: {carried:?}");
        let moved: Vec<_> = after.iter().filter(|r| !before.contains(r)).collect();
        assert!(
            matches!(moved[..], [&(_, piece)] if remnants
                .iter()
                .any(|&(p, source)| p == piece && p != source)),
            "{what}: one record moves onto a fresh surviving piece: {after:?} {remnants:?}"
        );
        assert_eq!(
            validate_pseudomanifold(&out.body, &carried, tol()),
            Ok(()),
            "{what}: the carried records pass at rest"
        );
    }
}
