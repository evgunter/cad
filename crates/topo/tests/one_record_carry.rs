//! **One lineage, one carry** (D1, Ev's rulings on `[ev]` PRs 4749 and
//! 4951): every op states what it made of each cell it touched, and one
//! carry lands each record on its cells' images by lineage where the op
//! kept or substituted both, and otherwise on every pair of its two
//! cells' images the census confirms. Two cells a cut mints on a recorded touch, meeting at a
//! point, land citing the record alone where one plane made both.
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common;
use geom_core::{Band, Point3, Tol, Vec3};
use topo::construction::Construct;
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

/// **A record on cells the op kept lands unread; one on a cell the op
/// divided that no pair confirms, with nothing removed, refuses
/// `Stale`** (D1): the carry reads geometry only where an op divided a
/// cell, and there a record is never dropped because no witness turned
/// up. Two cubes kissing at (1, 1, 1) carry their kiss and a second
/// record (bookkeeping: the first cube's far corner against the second
/// cube's far face, no touch). A merge that merges nothing keeps both
/// cells, and lands it; a split across the far face divides it, and
/// refuses. Red on the carry that confirms kept pairs: the merge
/// refused `Stale`.
#[test]
fn a_record_lands_by_lineage_and_is_stale_only_on_a_divided_cell() {
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
    let kept = outcome
        .carry_contacts(
            &records,
            &merged,
            band(),
            Some(topo::RegionLane::certified()),
        )
        .expect("the merge keeps both cells");
    assert!(
        kept.cell_pairs()
            .any(|p| p == (Cell::Vertex(corner), Cell::Face(far))),
        "the merge lands the record on its keys: {kept:?}"
    );
    let plane = common::split_plane(Point3::new(0.0, 0.0, 1.5), Vec3::new(0.0, 0.0, 1.0), tol());
    let split = topo::split(&base.body, &plane, tol()).expect("the kiss splits");
    assert!(
        matches!(
            split.carry_contacts(&records, band(), Some(topo::RegionLane::certified())),
            Err(topo::CarryRefusal::Stale { record: 1 })
        ),
        "the split divides the far face: the record is stale"
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
        restated = restated.with(c, cut.fate(c)).expect("one fate per cell");
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

/// `op` of `a` and `b`, carrying `a`'s records in.
fn carried_op(
    op: topo::BooleanOp,
    a: &BooleanBody<f64>,
    b: &AtRestBody<f64>,
) -> Result<BooleanResult<f64>, topo::BooleanError> {
    let inputs = topo::BooleanInputs {
        records_a: a.contacts.carried(),
        ..topo::BooleanInputs::default()
    };
    topo::boolean_op_with(
        op,
        &a.body,
        b,
        &inputs,
        topo::SweepStrategy::Realized,
        tol(),
    )
}

/// A unit cube turned off every axis, balanced on its corner at
/// `(2, 2, 1)` on a slab's top: one vertex–face record.
fn corner_on_slab() -> BooleanBody<f64> {
    let (e1, e2, e3) = (
        Vec3::new(0.9, 0.1, 0.3),
        Vec3::new(-0.2, 0.8, 0.45),
        Vec3::new(-0.3, -0.4, 0.85),
    );
    let tilted = common::finished(
        "the tilted cube",
        common::mapped_cube(
            |x, y, z| {
                Point3::new(
                    2.0 + x * e1.x + y * e2.x + z * e3.x,
                    2.0 + x * e1.y + y * e2.y + z * e3.y,
                    1.0 + x * e1.z + y * e2.z + z * e3.z,
                )
            },
            tol(),
        ),
        tol(),
    );
    let slab = box_of((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), "slab");
    let base = union_of(&slab, &tilted);
    assert_eq!(base.contacts.len(), 1, "{:?}", base.contacts);
    assert_eq!(
        base.contacts.vertex_face().count(),
        1,
        "{:?}",
        base.contacts
    );
    base
}

/// **A carried record on cells the boolean keeps lands unread** (D1):
/// the carry reads geometry only where an op divided a cell. The
/// corner-on-slab body carries its rest and a second record
/// (bookkeeping: the cube's corner against the slab's bottom, no touch)
/// into a union with a box far from both, which keeps every cell. Red
/// on the boolean's carry that confirms kept pairs: it refused `Stale`.
#[test]
fn a_carried_record_on_cells_the_boolean_keeps_lands_unread() {
    let mut base = corner_on_slab();
    let [corner, _] = base.contacts.rows()[0].cells();
    let bottom = base
        .body
        .faces()
        .find(|(_, f)| {
            matches!(
                base.body.get_surface(f.surface),
                Some(topo::Surface::Plane { origin, normal, .. })
                    if normal.z.abs() == 1.0 && origin.z == 0.0
            )
        })
        .map(|(k, _)| k)
        .expect("the slab's bottom");
    base.contacts.extend([topo::Cited::new(
        topo::Contact::new(&base.body, corner, Cell::Face(bottom)).unwrap(),
        topo::Cites::decided(0),
    )]);
    let far = box_of((10.0, 11.0), (10.0, 11.0), (0.0, 1.0), "far");
    match carried_op(topo::BooleanOp::Union, &base, &far) {
        Ok(BooleanResult::Body(out)) => assert_eq!(
            out.contacts.vertex_face().count(),
            2,
            "both records land: {:?}",
            out.contacts
        ),
        other => panic!("the union builds: {:?}", other.map(|_| ())),
    }
}

/// **A lineage gap on one cell refuses, whatever the other cell lost.**
/// The edge kiss's edge–edge record through a hand lineage: one edge
/// divided with its remainder removed (its one piece itself), the other
/// substituted by a cell the result does not hold, so the lineage does
/// not cover it. D1: "a cell the lineage does not cover refuses". Red
/// where the first cell's removal consumes the record.
#[test]
fn a_lineage_gap_beside_a_removal_refuses_unmapped() {
    let a = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), "a");
    let b = box_of((1.0, 2.0), (1.0, 2.0), (0.0, 1.0), "b");
    let base = union_of(&a, &b);
    let ee = record_of(&base.contacts, |k| matches!(k, ContactKind::EdgeEdge(..)));
    let [x, y] = base.contacts.rows()[ee as usize].cells();
    let one: ContactRecords = [base.contacts.rows()[ee as usize].clone()]
        .into_iter()
        .collect();
    let gap = Cell::Edge(topo::EdgeKey::default());
    let lineage = topo::Lineage::new()
        .with(
            x,
            topo::Fate::Divided(topo::Division {
                pieces: vec![x],
                minted: vec![],
                remainder: topo::Remainder::Removed,
            }),
        )
        .and_then(|l| l.with(y, topo::Fate::Substituted(gap)))
        .unwrap();
    assert_eq!(
        one.carried_through(
            0,
            &lineage,
            &base.body,
            band(),
            Some(topo::RegionLane::certified()),
        ),
        Err(topo::CarryRefusal::Unmapped { record: 0, cell: y }),
        "the uncovered cell refuses"
    );
}

/// **A pit dug under a carried rest consumes it.** The corner-on-slab
/// body minus the pit [1.9, 2.1]² × [0.5, 1] under the corner: the
/// slab's top is divided, the fragment under the corner removed with the
/// pit, so the rest lands on no pair and is consumed, and the result
/// passes at rest with no record. Red where a route that removes a face
/// does not state it: the carry then cannot tell the removed fragment
/// from one the lineage lost, and refuses `JoinDesync`.
#[test]
fn a_pit_under_a_carried_rest_consumes_it() {
    let base = corner_on_slab();
    let pit = box_of((1.9, 2.1), (1.9, 2.1), (0.5, 1.0), "pit");
    let out = match carried_op(topo::BooleanOp::Subtract, &base, &pit) {
        Ok(BooleanResult::Body(out)) => out,
        other => panic!("the pit builds: {other:?}"),
    };
    assert!(
        out.contacts.is_empty(),
        "the rest is consumed: {:?}",
        out.contacts
    );
    assert_eq!(
        validate_pseudomanifold(&out.body, &out.contacts, tol()),
        Ok(()),
        "the corner hangs over the pit"
    );
}

/// Two unit cubes kissing along the vertical edge x = y = 1: two
/// vertex–vertex records and the edge–edge record, whose index is
/// returned.
fn edge_kiss() -> (BooleanBody<f64>, u32) {
    let a = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), "a");
    let b = box_of((1.0, 2.0), (1.0, 2.0), (0.0, 1.0), "b");
    let base = union_of(&a, &b);
    let ee = record_of(&base.contacts, |k| matches!(k, ContactKind::EdgeEdge(..)));
    (base, ee)
}

/// The records of `out` on two vertices at `p`, each as its citations.
fn vertex_pairs_at(out: &BooleanBody<f64>, p: Point3<f64>) -> Vec<Vec<Backing>> {
    let at = |v: topo::VertexKey| {
        let q = out
            .body
            .get_point(out.body.get_vertex(v).unwrap().point)
            .copied()
            .unwrap();
        (q - p).norm() < 1e-9
    };
    out.contacts
        .rows()
        .iter()
        .filter(|r| matches!(r.cells(), [Cell::Vertex(x), Cell::Vertex(y)] if at(x) && at(y)))
        .map(|r| r.cites.iter().collect())
        .collect()
}

/// **Two tool faces cutting a recorded touch's two edges at one height
/// meet by the boolean's own decision** (Ev's ruling on `[ev]` PR 4749,
/// his example). The edge kiss minus a tool of two boxes, kissing along
/// the same line: one takes `a` below z = 1/2, the other `b` below
/// z = 1/2. Each box's top cuts one kissing edge there, minting a vertex
/// on each; they meet only because the two heights agree. The minted pair
/// cites the edge–edge record and the door's decision that the two
/// minted vertices are one point, which names two cells of the carried
/// body; the reduction's own decisions name a tool corner on each edge,
/// and say nothing of the heights agreeing. Red when the door's decision
/// is skipped (`bool_carried_minted_meeting`): the pair then cites the
/// record and the two corner decisions alone.
#[test]
fn two_cutters_meeting_on_a_recorded_touch_cite_the_doors_decision() {
    let (base, ee) = edge_kiss();
    let t1 = box_of((-1.0, 1.0), (-1.0, 1.0), (-1.0, 0.5), "t1");
    let t2 = box_of((1.0, 3.0), (1.0, 3.0), (-1.0, 0.5), "t2");
    let tool = union_of(&t1, &t2);
    let out = match carried_op(topo::BooleanOp::Subtract, &base, &tool.body) {
        Ok(BooleanResult::Body(out)) => out,
        other => panic!("the difference builds: {other:?}"),
    };
    let minted = vertex_pairs_at(&out, Point3::new(1.0, 1.0, 0.5));
    assert_eq!(minted.len(), 1, "one minted pair: {:?}", out.contacts);
    assert!(
        minted[0].contains(&Backing::Carried {
            input: 0,
            record: ee
        }),
        "the pair cites the edge-edge record: {minted:?}"
    );
    let on_the_carried_body = |k: u32| {
        out.coincidences[k as usize].cells.iter().all(|c| {
            matches!(
                c,
                topo::RowCell::Input {
                    input: topo::Operand::A,
                    ..
                }
            )
        })
    };
    assert!(
        minted[0]
            .iter()
            .any(|b| matches!(*b, Backing::Decided(k) if on_the_carried_body(k))),
        "the pair cites the door's decision on the two minted vertices: {minted:?} {:?}",
        out.coincidences
    );
    assert_eq!(
        validate_pseudomanifold(&out.body, &out.contacts, tol()),
        Ok(()),
        "the difference passes at rest"
    );
}

/// **One tool face cutting both edges of a recorded touch**: the edge
/// kiss minus a box above z = 1/2 mints a vertex on each kissing edge at
/// (1, 1, 1/2). The minted pair cites the edge–edge record and a
/// decision; here the door's decision is the reduction's own row on the
/// same two cells (`reduce::pend` keeps one), which already decides the
/// meeting. The difference passes at rest.
#[test]
fn one_cutter_on_a_recorded_touch_cites_the_record_and_a_decision() {
    let (base, ee) = edge_kiss();
    let tool = box_of((-1.0, 3.0), (-1.0, 3.0), (0.5, 2.0), "tool");
    let out = match carried_op(topo::BooleanOp::Subtract, &base, &tool) {
        Ok(BooleanResult::Body(out)) => out,
        other => panic!("the difference builds: {other:?}"),
    };
    let minted = vertex_pairs_at(&out, Point3::new(1.0, 1.0, 0.5));
    assert_eq!(minted.len(), 1, "one minted pair: {:?}", out.contacts);
    assert!(
        minted[0].contains(&Backing::Carried {
            input: 0,
            record: ee
        }) && minted[0].iter().any(|b| matches!(b, Backing::Decided(_))),
        "the pair cites the record and a decision: {minted:?}"
    );
    assert_eq!(
        validate_pseudomanifold(&out.body, &out.contacts, tol()),
        Ok(()),
        "the difference passes at rest"
    );
}

/// **A notch that removes the whole overlap of a recorded line touch
/// consumes the record.** `a` (z ∈ [0, 2]) and `b` (z ∈ [1, 3]) kiss
/// along x = y = 1 over z ∈ [1, 2]; a notch [0.5, 1.5]² × [0.9, 2.1]
/// takes that whole span from both. Each kissing edge is divided, its
/// remainder removed, so the edge–edge record lands on no pair and is
/// consumed, and the result passes at rest. Red where the boolean's
/// division is read as exhaustive: the record then refuses stale.
#[test]
fn a_notch_that_removes_a_recorded_overlap_consumes_it() {
    let a = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 2.0), "a");
    let b = box_of((1.0, 2.0), (1.0, 2.0), (1.0, 3.0), "b");
    let base = union_of(&a, &b);
    let ee = record_of(&base.contacts, |k| matches!(k, ContactKind::EdgeEdge(..)));
    let notch = box_of((0.5, 1.5), (0.5, 1.5), (0.9, 2.1), "notch");
    let out = match carried_op(topo::BooleanOp::Subtract, &base, &notch) {
        Ok(BooleanResult::Body(out)) => out,
        other => panic!("the notch builds: {other:?}"),
    };
    assert!(
        out.contacts.cites().all(|c| !c.iter().any(|b| b
            == Backing::Carried {
                input: 0,
                record: ee
            })),
        "the edge-edge record is consumed: {:?}",
        out.contacts
    );
    assert_eq!(
        validate_pseudomanifold(&out.body, &out.contacts, tol()),
        Ok(()),
        "nothing of the touch is left"
    );
}

/// The face of `body` on the plane `axis = at` (0 = x, 2 = z) whose
/// normal runs along that axis.
fn face_on(body: &topo::Body<f64>, axis: usize, at: f64) -> topo::FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(topo::Surface::Plane { origin, normal, .. })
                    if [normal.x, normal.y, normal.z][axis].abs() == 1.0
                        && [origin.x, origin.y, origin.z][axis] == at
            )
        })
        .map(|(k, _)| k)
        .expect("the face")
}

/// **A record on a cell an offset trims is re-read.** The
/// corner-on-slab rest, through an offset of the slab's side face
/// x = 4:
///
/// - by −0.5, which trims the slab's top on its carrier (its region shrinks to
///   x ≤ 3.5, the corner at x = 2 still inside it): the rest goes to the
///   census's confirm, reads met, and lands;
/// - the same offset, the record restated on the slab's bottom
///   (bookkeeping, no touch): the confirm reads it apart, and it is
///   consumed, never landed unread.
///
/// Red on the carry that reads a trimmed cell as kept: the bottom record
/// then landed.
#[test]
fn a_record_on_a_cell_an_offset_trims_is_re_read() {
    let base = corner_on_slab();
    let [corner, top] = base.contacts.rows()[0].cells();
    let bottom = Cell::Face(face_on(&base.body, 2, 0.0));
    let side = face_on(&base.body, 0, 4.0);
    let mut moved = topo::Body::clone(&base.body);
    let outcome =
        topo::replace_face_offset(&mut moved, side, -0.5, tol()).expect("the side moves in");
    let rest = outcome
        .carry_contacts(
            &base.contacts,
            &moved,
            band(),
            Some(topo::RegionLane::certified()),
        )
        .expect("the rest is confirmed");
    assert_eq!(
        rest.cell_pairs().collect::<Vec<_>>(),
        [(corner, top)],
        "the rest lands on the reshaped top"
    );
    let false_rest: ContactRecords = [topo::Cited::new(
        topo::Contact::new(&base.body, corner, bottom).unwrap(),
        topo::Cites::decided(0),
    )]
    .into_iter()
    .collect();
    assert_eq!(
        outcome.carry_contacts(
            &false_rest,
            &moved,
            band(),
            Some(topo::RegionLane::certified())
        ),
        Ok(ContactRecords::default()),
        "the bookkeeping record on the reshaped bottom is read apart and consumed"
    );
}

/// **A record whose cell an op substituted lands on the substitute.**
/// The corner-on-slab body's top is cut by a chord into two halves, the
/// rest recorded on the half that holds the corner; the merge puts the
/// halves back into one face, absorbing one of them into the other. In
/// whichever of the two cuts (x = 1, x = 3) the corner's half is the
/// absorbed one, its fate is `Substituted` and the rest lands on the
/// kept face. Red where a substituted cell's record is dropped, or
/// left on the absorbed face.
#[test]
fn a_real_touch_on_a_substituted_face_lands_on_its_substitute() {
    let mut substituted = 0;
    for x in [1.0, 3.0] {
        let base = corner_on_slab();
        let [corner, top] = base.contacts.rows()[0].cells();
        let Cell::Face(top) = top else {
            panic!("the rest is on a face")
        };
        let mut body = topo::Body::clone(&base.body);
        let edge_at = |body: &topo::Body<f64>, y: f64| {
            body.half_edges()
                .filter(|(_, h)| body.get_loop(h.parent_loop).unwrap().face == top)
                .map(|(_, h)| h.edge)
                .find(|&e| {
                    body.half_edges()
                        .filter(|(_, h)| h.edge == e)
                        .all(|(_, h)| {
                            let p = body
                                .get_point(body.get_vertex(h.start).unwrap().point)
                                .unwrap();
                            p.y == y
                        })
                })
                .expect("a top edge at y")
        };
        let mut ends = Vec::new();
        for y in [0.0, 4.0] {
            let e = edge_at(&body, y);
            let he = body.get_edge(e).unwrap().he_plus;
            let p0 = *body
                .get_point(
                    body.get_vertex(body.get_half_edge(he).unwrap().start)
                        .unwrap()
                        .point,
                )
                .unwrap();
            let t = if p0.x == 0.0 { x } else { 4.0 - x };
            let made = body.split_edge(e, t, tol()).expect("the top edge splits");
            let p = *body.get_point(made.point).unwrap();
            assert!((p.x - x).abs() < 1e-12, "the split lands at x = {x}: {p:?}");
            ends.push((made.vertex, p));
        }
        let f = body.get_face(top).unwrap();
        let topo::LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary else {
            panic!("the top loop is a cycle");
        };
        let starting = |v| {
            body.loop_cycle(first)
                .unwrap()
                .into_iter()
                .find(|&he| body.get_half_edge(he).unwrap().start == v)
                .unwrap()
        };
        let (he1, he2) = (starting(ends[0].0), starting(ends[1].0));
        let chord = body
            .mef(
                topo::construction::MefSite::Chords { he1, he2 },
                common::line(ends[0].1, ends[1].1),
                topo::construction::FaceSurface::Inherit,
                tol(),
            )
            .expect("the chord cuts the top");
        let holder = [top, chord.face]
            .into_iter()
            .find(|&h| {
                body.half_edges()
                    .filter(|(_, e)| body.get_loop(e.parent_loop).unwrap().face == h)
                    .any(|(_, e)| {
                        let p = body
                            .get_point(body.get_vertex(e.start).unwrap().point)
                            .unwrap();
                        (p.x - 2.0) * (x - 2.0) < 0.0
                    })
            })
            .expect("the half holding the corner");
        let rest: ContactRecords = [topo::Cited::new(
            topo::Contact::new(&body, corner, Cell::Face(holder)).unwrap(),
            topo::Cites::decided(0),
        )]
        .into_iter()
        .collect();
        let outcome = body.merge_coplanar_faces(tol()).expect("the merge");
        let group = outcome
            .groups
            .iter()
            .find(|g| g.kept == holder || g.absorbed.contains(&holder))
            .expect("the halves merge");
        let kept = Cell::Face(group.kept);
        if outcome.lineage().fate(Cell::Face(holder)) == topo::Fate::Substituted(kept) {
            substituted += 1;
        }
        let landed = outcome
            .carry_contacts(&rest, &body, band(), Some(topo::RegionLane::certified()))
            .expect("the rest lands");
        assert_eq!(
            landed.cell_pairs().collect::<Vec<_>>(),
            [(corner, kept)],
            "cut at x = {x}: the rest lands on the merged face"
        );
    }
    assert!(substituted > 0, "one cut absorbs the corner's half");
}
