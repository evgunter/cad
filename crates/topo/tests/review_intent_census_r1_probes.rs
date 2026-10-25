//! Review probes (INTENT census, R1): a confirmed face pair answers for
//! a touch of its boundary cells only in the closure of its overlap
//! (`[assertions]`), so a touch of the same two faces' boundaries
//! elsewhere on a non-convex face is decided as its own pair.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::Tol;
use std::collections::BTreeMap;
use topo::{Body, ContactRecords, SolidKey};

fn prism(profile: &[(f64, f64)], z0: f64, z1: f64) -> Body<f64> {
    common::prism_z::<f64>(profile, z0, z1, Tol::witness()).body
}

/// An L-shaped base (its void the quadrant x, y in [1, 2]) and a block
/// on it, grafted as two solids.
fn on_l_base(block: &[(f64, f64)]) -> Body<f64> {
    let l = [
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ];
    let mut body = prism(&l, 0.0, 1.0);
    topo::graft_disjoint_all_keyed(&mut body, &prism(block, 1.0, 2.0)).unwrap();
    body
}

fn recorded(body: &Body<f64>) -> topo::RecordedAtRest {
    let groups: BTreeMap<SolidKey, u32> = body.solids().map(|(s, _)| s).zip(0..).collect();
    topo::record_at_rest(
        body,
        &ContactRecords::default(),
        |records| {
            topo::validate_pseudomanifold(body, records, Tol::witness())
                .err()
                .unwrap_or_default()
        },
        &|s| groups.get(&s).copied(),
        0,
    )
}

fn is_face_pair(r: &topo::Coincidence) -> bool {
    r.cells.iter().all(|c| {
        matches!(
            c,
            topo::RowCell::Result {
                cell: topo::Cell::Face(_)
            }
        )
    })
}

fn kinds(r: &topo::Coincidence) -> [&'static str; 2] {
    r.cells.map(|c| match c {
        topo::RowCell::Result { cell } => match cell {
            topo::Cell::Face(_) => "face",
            topo::Cell::Edge(_) => "edge",
            topo::Cell::Vertex(_) => "vertex",
        },
        _ => "other",
    })
}

/// **A collinear run outside the overlap is its own pair.** The block
/// [1, 2] x [0.5, 1.5] overlaps the base's lower arm over
/// [1, 2] x [0.5, 1]; its x = 1 edge runs along the base's re-entrant
/// edge for y in [1, 1.5] with the two faces' materials on opposite
/// sides, outside the overlap's closure but the point (1, 1). Red if
/// that run is absorbed by the seat's pair (no edge row).
#[test]
fn r1_a_collinear_run_outside_a_non_convex_overlap_is_its_own_pair() {
    let body = on_l_base(&[(1.0, 0.5), (2.0, 0.5), (2.0, 1.5), (1.0, 1.5)]);
    let rec = recorded(&body);
    let rows: Vec<_> = rec.rows.iter().map(kinds).collect();
    assert!(rec.findings.is_empty(), "{:?}", rec.findings);
    assert_eq!(
        rec.rows.iter().filter(|r| is_face_pair(r)).count(),
        1,
        "{rows:?}"
    );
    assert!(
        rows.contains(&["edge", "edge"]),
        "the run along x = 1 is an edge row: {rows:?}"
    );
    assert_eq!(
        topo::validate_pseudomanifold(&body, &rec.records, Tol::witness()),
        Ok(()),
    );
}

/// **A spike's tip outside the overlap is its own pair.** The block
/// overlaps the base's lower arm, and a spike of it reaches over the
/// void to rest its tip on the base's re-entrant edge x = 1 at
/// y = 1.5, its material at x > 1, the base's at x < 1. Red if the tip
/// is absorbed by the seat's pair (no vertex-on-edge row).
#[test]
fn r1_a_spike_tip_outside_a_non_convex_overlap_is_its_own_pair() {
    let body = on_l_base(&[(1.5, 0.5), (2.0, 0.5), (2.0, 1.8), (1.0, 1.5), (1.5, 1.2)]);
    let rec = recorded(&body);
    let rows: Vec<_> = rec.rows.iter().map(kinds).collect();
    assert!(rec.findings.is_empty(), "{:?}", rec.findings);
    assert_eq!(
        rec.rows.iter().filter(|r| is_face_pair(r)).count(),
        1,
        "{rows:?}"
    );
    assert!(
        rows.iter()
            .any(|k| k.contains(&"vertex") && k.contains(&"edge")),
        "the tip on x = 1 is a vertex-on-edge row: {rows:?}"
    );
}

/// **Touches inside the overlap's closure stay the pair's.** A block on
/// the base's lower arm with an edge exactly through the re-entrant
/// corner: only the face pair. Red if a touch at the overlap's corner
/// or along its boundary is recorded separately.
#[test]
fn r1_touches_in_the_closure_of_a_non_convex_overlap_are_the_pairs() {
    let body = on_l_base(&[(0.5, 0.25), (2.0, 0.25), (2.0, 1.0), (0.5, 1.0)]);
    let rec = recorded(&body);
    let rows: Vec<_> = rec.rows.iter().map(kinds).collect();
    assert!(rec.findings.is_empty(), "{:?}", rec.findings);
    assert_eq!(rows, vec![["face", "face"]], "{rows:?}");
}

/// Two blocks one unit wide in x and y, grafted as two solids.
fn two_blocks(az: (f64, f64), bz: (f64, f64)) -> Body<f64> {
    let sq = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let mut body = prism(&sq, az.0, az.1);
    topo::graft_disjoint_all_keyed(&mut body, &prism(&sq, bz.0, bz.1)).unwrap();
    body
}

fn flush(body: &Body<f64>) -> topo::FlushAtRest {
    let groups: BTreeMap<SolidKey, u32> = body.solids().map(|(s, _)| s).zip(0..).collect();
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    <f64 as topo::AtRestPolicy>::flush_at_rest(body, &|s| groups.get(&s).copied(), band)
}

/// **Coincident side walls are no flush pair.** Two blocks of two
/// groups whose side walls lie on one plane facing one way and share a
/// positive area (the second block overlaps the first's upper half, or
/// sits wholly inside it, the heat sink's fin instance inside its
/// union): the walls' interiors share area, so Door 2 reads
/// `PositiveArea` and no flush row is written. Red if a wall pair
/// sharing area is reported flush.
#[test]
fn r1_side_walls_sharing_area_are_no_flush_rows() {
    let overlapping = flush(&two_blocks((0.0, 1.0), (0.5, 1.5)));
    let inside = flush(&two_blocks((0.0, 1.0), (0.25, 1.0)));
    let stacked = flush(&two_blocks((0.0, 1.0), (1.0, 2.0)));
    assert_eq!(
        stacked.rows.len(),
        4,
        "the control: stacked walls meet along a line"
    );
    assert!(
        overlapping.rows.is_empty(),
        "overlapping: {}",
        overlapping.rows.len()
    );
    assert!(inside.rows.is_empty(), "inside: {}", inside.rows.len());
}

/// **The heat sink's shape, both graft orders.** A block z in
/// [0.25, 1] and one z in [0.1875, 1] on the same footprint: their four
/// side walls share a 0.75-tall area, so none is a flush pair. Red if
/// either graft order reports one.
#[test]
fn r1_the_heat_sinks_fin_walls_are_no_flush_rows_in_either_order() {
    let fin = [
        (0.25, 0.125),
        (0.4375, 0.125),
        (0.4375, 0.875),
        (0.25, 0.875),
    ];
    for (a, b) in [((0.25, 1.0), (0.1875, 1.0)), ((0.1875, 1.0), (0.25, 1.0))] {
        let mut body = prism(&fin, a.0, a.1);
        topo::graft_disjoint_all_keyed(&mut body, &prism(&fin, b.0, b.1)).unwrap();
        let f = flush(&body);
        assert!(f.rows.is_empty(), "{a:?} then {b:?}: {} rows", f.rows.len());
    }
}

/// **The heat sink itself**: a base plate united with a fin sunk into
/// it, and the same fin placed again as its own solid. The instance's
/// four side walls share a 0.75-tall area with the union's: no flush
/// pair. Red if one is reported (`perf12_census_goldens`' heat_sink
/// rows).
#[test]
fn r1_a_fin_instance_over_its_united_copy_is_no_flush_row() {
    let tol = Tol::witness();
    let base = prism(&[(0.0, 0.0), (3.0, 0.0), (3.0, 1.0), (0.0, 1.0)], 0.0, 0.25);
    let fin = [
        (0.25, 0.125),
        (0.4375, 0.125),
        (0.4375, 0.875),
        (0.25, 0.875),
    ];
    let fin_at = |z0| common::finished("fin", prism(&fin, z0, 1.0), tol);
    let base = common::finished("base", base, tol);
    let topo::BooleanResult::Body(out) = topo::union(&base, &fin_at(0.1875), tol).unwrap() else {
        panic!("one body");
    };
    let mut body = out.body.into_body();
    topo::graft_disjoint_all_keyed(&mut body, &prism(&fin, 0.1875, 1.0)).unwrap();
    let f = flush(&body);
    assert!(f.rows.is_empty(), "{} rows", f.rows.len());
}
