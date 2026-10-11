//! **`shell_open` carries a record through the join it ends with.**

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shell_curved_mouth::split_d_section;
use geom_core::Tol;
use sweep::test_support::finished;
use topo::{Backing, Cell, Cited, Cites, Contact, ContactRecords};

/// **A record on a vertex the shell joins away lands on the joined
/// edge.** The D-section's half-cylinder is split along its middle
/// ruling, so each arc carries a vertex at its middle; opening both
/// halves merges them, leaves each such vertex between two pieces of
/// one circle, and the shell's closing join takes it. A record naming
/// one (bookkeeping against the flat face, no touch) is carried onto the
/// edge that holds it now. Red if the carry skips the shell's joins: the
/// vertex is dead.
#[test]
fn shell_open_carries_a_record_through_its_join() {
    let tol = Tol::witness();
    let body = split_d_section(0.5, 0.8);
    let wall: Vec<topo::FaceKey> = body
        .faces()
        .filter(|(_, f)| {
            body.get_surface(f.surface).map(geom::Surface::kind)
                == Some(geom::SurfaceKind::Cylinder)
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(wall.len(), 2, "the half-cylinder is split in two");
    let flat = body
        .faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(topo::Surface::Plane { normal, .. }) if normal.x.abs() == 1.0
            )
        })
        .map(|(k, _)| k)
        .expect("the D's flat face");
    let operand = finished("the operand", body, tol);
    let shelled = topo::shell_open(&operand, 0.05, &wall, tol).expect("the window opens");
    let joins = &shelled.naming.edge_joins;
    let join = joins
        .iter()
        .find(|j| operand.get_vertex(j.vertex).is_some())
        .expect("an operand vertex is joined away");
    let records: ContactRecords = [Cited::new(
        Contact::new(&operand, Cell::Vertex(join.vertex), Cell::Face(flat)).unwrap(),
        Cites::decided(0),
    )]
    .into_iter()
    .collect();
    let carried = shelled.carry_contacts(&records).unwrap();
    let joined = Cell::Edge(topo::joined_edge(joins, join.kept));
    assert_eq!(
        carried.cell_pairs().collect::<Vec<_>>(),
        [Contact::new(&shelled.body, joined, Cell::Face(flat))
            .unwrap()
            .cells()
            .into()]
    );
    assert_eq!(
        carried.rows()[0].cites.iter().collect::<Vec<_>>(),
        [Backing::Carried {
            input: 0,
            record: 0
        }]
    );
}
