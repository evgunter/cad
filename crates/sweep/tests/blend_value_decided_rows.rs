//! **The blend battery records every coincidence it decides from
//! values** (D10): beside the isosceles turn (`band_planar_mitre`), a
//! chain junction decided tangent (`fillet3_chain_g1`) and a curved
//! support pair decided to share an axis or a ruling
//! (`fillet3_support_coaxiality`). Each Zero is one row, carried out on
//! `Blended::coincidences`.
//!
//! - `a_disc_rim_records_its_joints_and_its_supports` — a three-arc
//!   disc's rim: each link's two supports share the wall's axis (one
//!   `Coaxial` row per link, its two faces) and each junction is
//!   tangent (one `Tangent { aligned: true }` row per junction, the two
//!   links' edges). Red if either decision stops recording, or records
//!   other cells.
//! - `a_rod_crease_records_its_ruling` — the ruled family: each of a
//!   milled rod's two creases records one `Coaxial` row; the creases
//!   meet at no junction.
//! - `a_box_edge_records_nothing` — a plane–plane link decides no
//!   coaxiality, and a lone link has no junction.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Tol};
use sweep::blend::battery::{BatteryVerdict, BlendRequest, run_battery};
use sweep::blend::build::fillet_edges;
use sweep::test_support::{circle_arcs_at_z, cube, disc_of_arcs, rod_creases, rod_with_flat};
use topo::{Body, Cell, DecisionSite, EdgeKey, Operand, Relation, RowCell};

fn tol() -> Tol {
    Tol::witness()
}

/// The fillet radius, meters.
const RHO: f64 = 0.1;

fn verdict(body: &Body<f64>, edges: &[EdgeKey]) -> BatteryVerdict<f64> {
    run_battery(
        &BlendRequest {
            body,
            edges: edges.to_vec(),
            size: RHO,
        },
        Band::linear(tol()).unwrap(),
    )
    .expect("the battery passes")
}

/// The row's margin read back: decided Zero, so within the band.
fn reading(row: &topo::Coincidence) -> f64 {
    row.margin
        .diagnostic_f64_for_error_text()
        .value()
        .expect("an f64 margin")
}

/// The rows the battery hands on are the ones the blended body carries.
fn carried(body: &Body<f64>, edges: &[EdgeKey], v: &BatteryVerdict<f64>) {
    let at_rest = sweep::test_support::at_rest(body, tol());
    let blended = fillet_edges(&at_rest, edges, RHO, tol()).expect("the request carves");
    assert_eq!(
        blended.coincidences,
        v.coincidences().copied().collect::<Vec<_>>(),
        "the blended body carries the battery's rows, in its order"
    );
}

fn edge(e: EdgeKey) -> RowCell {
    RowCell::Input {
        input: Operand::A,
        cell: Cell::Edge(e),
    }
}

#[test]
fn a_disc_rim_records_its_joints_and_its_supports() {
    let disc = disc_of_arcs(3, 0.5, 1.0, tol());
    let rim = circle_arcs_at_z(&disc, 1.0);
    assert_eq!(rim.len(), 3, "the rim is three arcs");
    let v = verdict(&disc, &rim);
    let rows: Vec<_> = v.coincidences().copied().collect();
    let links: Vec<_> = v.chains.iter().flat_map(|c| c.links()).collect();
    assert_eq!(links.len(), 3);

    let (axes, joints) = rows.split_at(3);
    for (row, link) in axes.iter().zip(&links) {
        assert_eq!(
            (row.relation, row.site),
            (Relation::Coaxial, DecisionSite::BatterySupportAxis),
            "the supports come first, one per link in walk order: {rows:?}"
        );
        assert_eq!(
            row.cells,
            [link.face_a, link.face_b].map(|f| RowCell::face(Operand::A, f)),
            "a coaxiality row names its link's two supports"
        );
        assert!(reading(row).abs() < 1e-12, "decided Zero: {row:?}");
    }

    assert_eq!(joints.len(), 3, "one row per junction of the closed rim");
    assert_eq!(joints, &v.joints[..]);
    for row in joints {
        assert_eq!(
            (row.relation, row.site),
            (
                Relation::Tangent { aligned: true },
                DecisionSite::BatteryJoint
            ),
            "a smooth junction is tangent, heading on: {row:?}"
        );
        let [a, b] = row.cells;
        assert_ne!(a, b, "a junction joins two links");
        assert!(
            [a, b].iter().all(|c| rim.iter().any(|&e| edge(e) == *c)),
            "a joint row names two rim edges: {row:?}"
        );
        assert!(reading(row).abs() < 1e-12, "decided Zero: {row:?}");
    }
    // Every rim edge sits at two of the closed rim's junctions.
    for &e in &rim {
        let n = joints.iter().filter(|r| r.cells.contains(&edge(e))).count();
        assert_eq!(n, 2, "edge {e:?} is in {n} joint rows");
    }

    carried(&disc, &rim, &v);
}

#[test]
fn a_rod_crease_records_its_ruling() {
    let rod = rod_with_flat(tol());
    let creases = rod_creases(&rod);
    assert_eq!(creases.len(), 2);
    let v = verdict(&rod, &creases);
    let rows: Vec<_> = v.coincidences().copied().collect();
    assert_eq!(rows.len(), 2, "one row per crease: {rows:?}");
    assert!(v.joints.is_empty(), "the creases meet at no junction");
    for (row, link) in rows.iter().zip(v.chains.iter().flat_map(|c| c.links())) {
        assert_eq!(
            (row.relation, row.site),
            (Relation::Coaxial, DecisionSite::BatterySupportAxis)
        );
        assert_eq!(
            row.cells,
            [link.face_a, link.face_b].map(|f| RowCell::face(Operand::A, f))
        );
        assert!(reading(row).abs() < 1e-12, "decided Zero: {row:?}");
    }
    carried(&rod, &creases, &v);
}

#[test]
fn a_box_edge_records_nothing() {
    let body = cube(1.0, tol());
    let one = vec![body.edges().next().unwrap().0];
    let v = verdict(&body, &one);
    assert_eq!(v.coincidences().count(), 0);
    carried(&body, &one, &v);
}
