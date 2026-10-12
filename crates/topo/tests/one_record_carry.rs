//! **One lineage, one carry** (D1, Ev's ruling on `[ev]` PR 4749): every
//! op states what it made of each cell it touched, and one carry lands
//! each record on every pair of its two cells' images the census
//! confirms. Two cells a cut mints on a recorded touch, meeting at a
//! point, land citing the record alone where one plane made both.
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common;
use geom_core::{Band, Point3, Tol, Vec3};
use topo::{
    AtRestBody, Backing, BooleanBody, BooleanResult, Cell, ContactKind, ContactRecords, union,
    validate_pseudomanifold,
};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

fn box_of(x: (f64, f64), y: (f64, f64), z: (f64, f64), what: &str) -> AtRestBody<f64> {
    common::finished(what, common::brick(x, y, z, tol()), tol())
}

fn union_of(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> BooleanBody<f64> {
    match union(a, b, tol()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("the union builds: {other:?}"),
    }
}

/// The index of `base`'s one record of `kind`'s shape.
fn record_of(base: &ContactRecords, shape: fn(&ContactKind) -> bool) -> u32 {
    let at: Vec<usize> = base
        .kinds()
        .enumerate()
        .filter(|(_, k)| shape(k))
        .map(|(i, _)| i)
        .collect();
    assert_eq!(at.len(), 1, "one record of the shape: {base:?}");
    u32::try_from(at[0]).unwrap()
}

/// Splits `base` by the plane through `origin` across `normal`, carries
/// its records, and asserts each side passes at rest with what it got.
fn split_passes(
    what: &str,
    base: &BooleanBody<f64>,
    origin: Point3<f64>,
    normal: Vec3<f64>,
) -> topo::SplitContacts {
    let plane = common::split_plane(origin, normal, tol());
    let split = topo::split(&base.body, &plane, tol()).expect("the body splits");
    let sides = split
        .carry_contacts(&base.contacts, band(), Some(topo::RegionLane::certified()))
        .unwrap_or_else(|refusal| panic!("{what}: the records land: {refusal:?}"));
    for (side, body, records) in [
        ("above", split.above.body(), &sides.above),
        ("below", split.below.body(), &sides.below),
    ] {
        assert_eq!(
            validate_pseudomanifold(body.expect("material"), records, tol()),
            Ok(()),
            "{what} {side}: the carried records pass at rest: {records:?}"
        );
    }
    sides
}

/// The records of `side` whose two cells are both vertices, each with
/// what it cites.
fn vertex_pairs(side: &ContactRecords) -> Vec<Vec<Backing>> {
    side.rows()
        .iter()
        .filter(|r| matches!(r.cells(), [Cell::Vertex(_), Cell::Vertex(_)]))
        .map(|r| r.cites.iter().collect())
        .collect()
}

/// **A split across an edge kiss lands the pair it mints on the kiss.**
/// Two unit cubes share the edge x = y = 1, recorded edge–edge and
/// bounded by a vertex–vertex record at each end. The plane z = ½ cuts
/// each kissing edge, minting one vertex on each at (1, 1, ½). On each
/// side the edge–edge record lands on the two edge pieces and on the
/// two minted vertices, which one plane made, so that landing cites the
/// edge–edge record alone; each side keeps its end record, and both pass
/// at rest. Red on the carry that places only pieces: each side refuses
/// the minted pair undeclared, and the half overlap it bounds.
#[test]
fn a_split_across_an_edge_kiss_lands_the_minted_pair_on_the_kiss() {
    let a = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), "a");
    let b = box_of((1.0, 2.0), (1.0, 2.0), (0.0, 1.0), "b");
    let base = union_of(&a, &b);
    let ee = record_of(&base.contacts, |k| matches!(k, ContactKind::EdgeEdge(..)));
    let sides = split_passes(
        "edge kiss",
        &base,
        Point3::new(0.0, 0.0, 0.5),
        Vec3::new(0.0, 0.0, 1.0),
    );
    for (what, side) in [("above", &sides.above), ("below", &sides.below)] {
        assert_eq!(
            side.edge_edge().count(),
            1,
            "{what}: one piece pair {side:?}"
        );
        let pairs = vertex_pairs(side);
        assert_eq!(
            pairs.len(),
            2,
            "{what}: the end and the minted pair {side:?}"
        );
        assert!(
            pairs.contains(&vec![Backing::Carried {
                input: 0,
                record: ee
            }]),
            "{what}: the minted pair cites the edge–edge record alone: {pairs:?}"
        );
    }
}

/// A cube turned 45° about x resting its bottom edge on a slab's top,
/// from x = 1 to 3 along y = 2: an edge–face record, bounded by a
/// vertex–face record at each end.
fn ridge_on_slab() -> BooleanBody<f64> {
    let slab = box_of((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), "slab");
    let s = core::f64::consts::FRAC_1_SQRT_2;
    let ridge = common::finished(
        "ridge",
        common::mapped_cube(
            |u, v, w| Point3::new(1.0 + 2.0 * u, 2.0 + (v - w) * s, 1.0 + (v + w) * s),
            tol(),
        ),
        tol(),
    );
    union_of(&slab, &ridge)
}

/// **A split across a ridge resting on a slab lands the ridge's minted
/// vertex on the slab's minted chord.** The plane x = 2 cuts the ridge
/// edge and the slab's top at (2, 2, 1). On each side the edge–face
/// record lands on the ridge's piece and the top's fragment, and on the
/// vertex the cut minted on the ridge against the chord it minted on
/// the top, a pair one plane made, citing the edge–face record alone;
/// both sides pass at rest. Red on the carry that places only pieces:
/// each side refuses the vertex on the chord undeclared, and the half
/// overlap it bounds.
#[test]
fn a_split_across_a_resting_ridge_lands_its_minted_vertex_on_the_minted_chord() {
    let base = ridge_on_slab();
    let ef = record_of(&base.contacts, |k| matches!(k, ContactKind::EdgeFace(..)));
    assert_eq!(
        base.contacts.vertex_face().count(),
        2,
        "{:?}",
        base.contacts
    );
    let sides = split_passes(
        "ridge",
        &base,
        Point3::new(2.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
    );
    for (what, side) in [("above", &sides.above), ("below", &sides.below)] {
        assert_eq!(side.edge_face().count(), 1, "{what}: {side:?}");
        let on_chord: Vec<Vec<Backing>> = side
            .rows()
            .iter()
            .filter(|r| matches!(r.cells(), [Cell::Vertex(_), Cell::Edge(_)]))
            .map(|r| r.cites.iter().collect())
            .collect();
        assert_eq!(
            on_chord,
            [vec![Backing::Carried {
                input: 0,
                record: ef
            }]],
            "{what}: the minted vertex on the minted chord cites the edge–face record alone"
        );
    }
}

/// The vertex of `body` at `p`.
fn vertex_at(body: &topo::Body<f64>, p: Point3<f64>) -> topo::VertexKey {
    body.vertices()
        .find(|(_, v)| {
            body.get_point(v.point)
                .is_some_and(|q| q.x == p.x && q.y == p.y && q.z == p.z)
        })
        .map(|(k, _)| k)
        .expect("a vertex there")
}

/// **A record no pair confirms, through an op that removed nothing,
/// refuses `Stale`**: it is never dropped because no witness turned up
/// (D1). Two cubes kissing at (1, 1, 1) carry their kiss; a second record
/// (bookkeeping: the first cube's far corner against the second cube's
/// far face, no touch) is carried through a merge that merges nothing.
/// Red on the carry that keeps a record on its keys unread.
#[test]
fn a_record_that_lands_nowhere_with_nothing_removed_refuses_stale() {
    let a = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), "a");
    let b = box_of((1.0, 2.0), (1.0, 2.0), (1.0, 2.0), "b");
    let base = union_of(&a, &b);
    let corner = vertex_at(&base.body, Point3::new(0.0, 0.0, 0.0));
    let far = base
        .body
        .faces()
        .find(|(_, f)| {
            matches!(
                base.body.get_surface(f.surface),
                Some(topo::Surface::Plane { origin, normal, .. })
                    if normal.x.abs() == 1.0 && origin.x == 2.0
            )
        })
        .map(|(k, _)| k)
        .expect("the second cube's far face");
    let mut records = base.contacts.clone();
    records.extend([topo::Cited::new(
        topo::Contact::new(&base.body, Cell::Vertex(corner), Cell::Face(far)).unwrap(),
        topo::Cites::decided(0),
    )]);
    let mut merged = topo::Body::clone(&base.body);
    let outcome = merged.merge_coplanar_faces(tol()).expect("the merge");
    assert!(outcome.groups.is_empty(), "nothing merges: {outcome:?}");
    assert_eq!(
        outcome.carry_contacts(
            &records,
            &merged,
            band(),
            Some(topo::RegionLane::certified())
        ),
        Err(topo::CarryRefusal::Stale { record: 1 })
    );
}

/// **An op that is not one cut refuses two cells it minted on a recorded
/// touch meeting at a point** (D1: only the split's one plane, or an op
/// that decides the meeting at its door, backs one). The edge kiss's
/// split lineage, restated as an op's that declares no one cut, carries
/// the edge–edge record onto the side above: its minted pair refuses.
/// Red on the carry that lands every confirmed pair whatever the op.
#[test]
fn a_minted_pair_an_op_cannot_back_refuses() {
    let a = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), "a");
    let b = box_of((1.0, 2.0), (1.0, 2.0), (0.0, 1.0), "b");
    let base = union_of(&a, &b);
    let ee = record_of(&base.contacts, |k| matches!(k, ContactKind::EdgeEdge(..)));
    let plane = common::split_plane(Point3::new(0.0, 0.0, 0.5), Vec3::new(0.0, 0.0, 1.0), tol());
    let split = topo::split(&base.body, &plane, tol()).expect("the edge kiss splits");
    assert!(
        split.naming.edge_joins.is_empty(),
        "the restated lineage reads no join"
    );
    let cut = split.lineage();
    let mut restated = topo::Lineage::new();
    for c in base.contacts.cell_pairs().flat_map(|(x, y)| [x, y]) {
        restated.set(c, cut.fate(c));
    }
    let one: ContactRecords = [base.contacts.rows()[ee as usize].clone()]
        .into_iter()
        .collect();
    let refused = one.carried_through(
        0,
        &restated,
        split.above.body().expect("material above"),
        band(),
        Some(topo::RegionLane::certified()),
    );
    assert!(
        matches!(
            refused,
            Err(topo::CarryRefusal::MintedMeeting {
                record: 0,
                cells: [Cell::Vertex(_), Cell::Vertex(_)]
            })
        ),
        "{refused:?}"
    );
}
