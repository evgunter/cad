//! Review probes (R1) for PRs 4809 + 4820: the one lineage and the one
//! carry. Each row states what it shows; a row that pins a defect says so.
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common;
use geom_core::{Band, Point3, Tol};
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

fn edge_kiss() -> (BooleanBody<f64>, u32) {
    let a = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), "a");
    let b = box_of((1.0, 2.0), (1.0, 2.0), (0.0, 1.0), "b");
    let base = union_of(&a, &b);
    let ee = base
        .contacts
        .kinds()
        .position(|k| matches!(k, ContactKind::EdgeEdge(..)))
        .expect("an edge-edge record");
    (base, u32::try_from(ee).unwrap())
}

/// **The boolean's minted-pair door, reached.** The edge kiss minus a
/// box that takes everything above z = 1/2: one tool face cuts both
/// kissing edges, minting a vertex on each at (1, 1, 1/2). The landing is
/// minted, and the door's decision runs; but `reduce::pend` dedupes it
/// into the ring tie's row the reduction already holds (relation and
/// cells equal), so the pair cites `Decided(0)` (`VertexFusion`,
/// `OnCarrier`, named by the two input edges) with the decision skipped
/// too: this row does NOT catch skipping the door's decision (executed),
/// and the same output holds on 7dc1049.
#[test]
fn r1_a_difference_across_an_edge_kiss_decides_the_minted_pair() {
    let (base, ee) = edge_kiss();
    let tool = box_of((-1.0, 3.0), (-1.0, 3.0), (0.5, 2.0), "tool");
    let inputs = topo::BooleanInputs {
        records_a: base.contacts.carried(),
        ..topo::BooleanInputs::default()
    };
    let got = topo::boolean_op_with(
        topo::BooleanOp::Subtract,
        &base.body,
        &tool,
        &inputs,
        topo::SweepStrategy::Realized,
        tol(),
    );
    let out = match got {
        Ok(BooleanResult::Body(out)) => out,
        other => panic!("the difference builds: {other:?}"),
    };
    let at_cut = |v: topo::VertexKey| {
        let p = out
            .body
            .get_point(out.body.get_vertex(v).unwrap().point)
            .copied()
            .unwrap();
        (p - Point3::new(1.0, 1.0, 0.5)).norm() < 1e-9
    };
    let minted: Vec<Vec<Backing>> = out
        .contacts
        .rows()
        .iter()
        .filter(
            |r| matches!(r.cells(), [Cell::Vertex(a), Cell::Vertex(b)] if at_cut(a) && at_cut(b)),
        )
        .map(|r| r.cites.iter().collect())
        .collect();
    eprintln!("records: {:?}", out.contacts);
    eprintln!("minted pair cites: {minted:?}");
    eprintln!("coincidences: {:?}", out.coincidences);
    eprintln!(
        "base edge-edge cells: {:?}",
        base.contacts.rows()[ee as usize].cells()
    );
    assert_eq!(minted.len(), 1, "one minted pair: {:?}", out.contacts);
    assert!(
        minted[0].contains(&Backing::Carried {
            input: 0,
            record: ee
        }),
        "cites the edge-edge record: {minted:?}"
    );
    assert!(
        minted[0].iter().any(|b| matches!(b, Backing::Decided(_))),
        "cites the boolean's own decision beside it: {minted:?}"
    );
    assert_eq!(
        validate_pseudomanifold(&out.body, &out.contacts, tol()),
        Ok(()),
        "the difference passes at rest"
    );
}

/// **A lineage gap on one cell is masked by a removal on the other.**
/// The edge kiss's edge-edge record through a hand lineage: one edge
/// divided with its remainder removed (its one piece itself), the other
/// substituted by a cell the result does not hold (an op bug: the
/// lineage does not cover it). D1: "a cell the lineage does not cover
/// refuses". Fixed in fde80e8b85: it refuses `Unmapped`.
#[test]
fn r1_a_lineage_gap_beside_a_removal_refuses_unmapped() {
    let (base, ee) = edge_kiss();
    let [x, y] = base.contacts.rows()[ee as usize].cells();
    let lineage = topo::Lineage::new()
        .with(
            x,
            topo::Fate::Divided(topo::Division {
                pieces: vec![x],
                minted: vec![],
                remainder: topo::Remainder::Removed,
            }),
        )
        .unwrap()
        .with(
            y,
            topo::Fate::Substituted(Cell::Edge(topo::EdgeKey::default())),
        )
        .unwrap();
    let one: ContactRecords = [base.contacts.rows()[ee as usize].clone()]
        .into_iter()
        .collect();
    let got = one.carried_through(
        0,
        &lineage,
        &base.body,
        band(),
        Some(topo::RegionLane::certified()),
    );
    eprintln!("got: {got:?}");
    // The same gap with nothing removed refuses Unmapped.
    let bare = topo::Lineage::new()
        .with(
            y,
            topo::Fate::Substituted(Cell::Edge(topo::EdgeKey::default())),
        )
        .unwrap();
    let bare_got = one.carried_through(
        0,
        &bare,
        &base.body,
        band(),
        Some(topo::RegionLane::certified()),
    );
    assert!(
        matches!(bare_got, Err(topo::CarryRefusal::Unmapped { .. })),
        "{bare_got:?}"
    );
    assert!(
        matches!(got, Err(topo::CarryRefusal::Unmapped { .. })),
        "the gap refuses beside a removal: {got:?}"
    );
}

fn corner_on_slab() -> BooleanBody<f64> {
    use geom_core::Vec3;
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
    assert_eq!(
        base.contacts.vertex_face().count(),
        1,
        "{:?}",
        base.contacts
    );
    base
}

/// `base` minus `tool`, carrying `base`'s records; the result's records
/// and whether it passes 3', or the refusal.
fn minus(
    base: &BooleanBody<f64>,
    tool: &AtRestBody<f64>,
) -> Result<(ContactRecords, bool), String> {
    let inputs = topo::BooleanInputs {
        records_a: base.contacts.carried(),
        ..topo::BooleanInputs::default()
    };
    match topo::boolean_op_with(
        topo::BooleanOp::Subtract,
        &base.body,
        tool,
        &inputs,
        topo::SweepStrategy::Realized,
        tol(),
    ) {
        Ok(BooleanResult::Body(out)) => {
            let ok = validate_pseudomanifold(&out.body, &out.contacts, tol()).is_ok();
            Ok((out.contacts, ok))
        }
        other => Err(format!("{other:?}")),
    }
}

fn carried_left(r: &ContactRecords) -> usize {
    r.cites()
        .filter(|c| c.iter().any(|b| matches!(b, Backing::Carried { .. })))
        .count()
}

/// **Pits under a carried rest** (the delta's MAJOR): centred, deeper,
/// offset (corner still over the opening), and one whose wall passes
/// through the corner's foot (the rest lands on the rim edge). Each
/// builds and passes 3'; the first three consume the rest.
#[test]
fn r1_pits_under_a_carried_rest() {
    let base = corner_on_slab();
    for (what, x, y, z, consumed) in [
        ("centred", (1.9, 2.1), (1.9, 2.1), (0.5, 1.0), true),
        ("deeper", (1.9, 2.1), (1.9, 2.1), (0.1, 1.0), true),
        ("offset", (1.95, 2.3), (1.7, 2.02), (0.5, 1.0), true),
        (
            "wall through the foot",
            (2.0, 2.2),
            (1.8, 2.2),
            (0.5, 1.0),
            false,
        ),
    ] {
        let got = minus(&base, &box_of(x, y, z, what));
        eprintln!("pit {what}: {got:?}");
        if !consumed && got.as_ref().is_err_and(|e| e.contains("VertexReadTwice")) {
            // The boolean's own frontier (the cutter's wall meets the
            // resting corner), before any carry.
            continue;
        }
        let (records, ok) = got.unwrap_or_else(|e| panic!("pit {what} builds: {e}"));
        assert!(ok, "pit {what}: passes 3': {records:?}");
        if consumed {
            assert_eq!(
                carried_left(&records),
                0,
                "pit {what}: consumed {records:?}"
            );
        }
    }
}
