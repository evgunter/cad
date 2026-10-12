//! **`shell_open`'s lineage reads through the join it ends with.**

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shell_curved_mouth::split_d_section;
use geom_core::Tol;
use sweep::test_support::finished;
use topo::Cell;

/// **A vertex the shell joins away is the joined edge in its lineage.**
/// The D-section's half-cylinder is split along its middle ruling, so
/// each arc carries a vertex at its middle; opening both halves merges
/// them, leaves each such vertex between two pieces of one circle, and
/// the shell's closing join takes it. Red if the lineage skips the
/// shell's joins: the vertex is dead.
#[test]
fn shell_opens_lineage_reads_through_its_join() {
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
    let operand = finished("the operand", body, tol);
    let shelled = topo::shell_open(&operand, 0.05, &wall, tol).expect("the window opens");
    let joins = &shelled.naming.edge_joins;
    let join = joins
        .iter()
        .find(|j| operand.get_vertex(j.vertex).is_some())
        .expect("an operand vertex is joined away");
    assert_eq!(
        shelled.lineage().images(Cell::Vertex(join.vertex)),
        [Cell::Edge(topo::joined_edge(joins, join.kept))]
    );
}
