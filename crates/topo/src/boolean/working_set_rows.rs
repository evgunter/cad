//! **The reduction's working set reads each row back as the kind its
//! writer wrote**: every typed pair's `cells` encoder lands in its own
//! per-kind read and in no other, and `rows()` keeps the fixed kind order
//! whatever order the rows were pushed in.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::{
    Cell, EeContact, EfContact, Operand, ReductionContacts, RowKind, TangentFaces, VfContact,
    VvContact, row_kind,
};
use crate::coincidence::{Cited, Cites};
use crate::entity::{EdgeKey, FaceKey, VertexKey};

/// **Each encoder round-trips through [`row_kind`].** Red where an
/// encoder writes a shape the reading takes for another kind, or swaps
/// an operand's vertex and face, or its edge and face.
#[test]
fn each_encoder_reads_back_as_its_own_kind() {
    let (v, e, f) = (VertexKey::default(), EdgeKey::default(), FaceKey::default());
    let vv = VvContact { a: v, b: v };
    let vf = VfContact { vertex: v, face: f };
    let ef = EfContact { edge: e, face: f };
    for (what, cells, want) in [
        ("vertex pair", vv.cells(), RowKind::Vv(vv)),
        (
            "A's vertex on B's face",
            vf.cells(Operand::A),
            RowKind::On(Operand::A, vf),
        ),
        (
            "B's vertex on A's face",
            vf.cells(Operand::B),
            RowKind::On(Operand::B, vf),
        ),
        ("edge pair", EeContact { a: e, b: e }.cells(), RowKind::Ee),
        (
            "face pair",
            TangentFaces { a: f, b: f }.cells(),
            RowKind::Ff,
        ),
        (
            "A's edge in B's face",
            ef.cells(Operand::A),
            RowKind::In(Operand::A, ef),
        ),
        (
            "B's edge in A's face",
            ef.cells(Operand::B),
            RowKind::In(Operand::B, ef),
        ),
    ] {
        assert_eq!(row_kind(cells), want, "{what}");
    }
}

/// **`rows()` reads in the fixed kind order, each kind in push order**,
/// and each per-kind read holds its own rows alone. The rows are pushed
/// in the reverse of that order. Red where `rows()` reads in push order,
/// or a read takes another operand's rows.
#[test]
fn rows_read_in_kind_order_whatever_the_push_order() {
    let (v, e, f) = (VertexKey::default(), EdgeKey::default(), FaceKey::default());
    let vv = VvContact { a: v, b: v }.cells();
    let on_a = VfContact { vertex: v, face: f }.cells(Operand::A);
    let on_b = VfContact { vertex: v, face: f }.cells(Operand::B);
    let ee = EeContact { a: e, b: e }.cells();
    let ff = TangentFaces { a: f, b: f }.cells();
    let in_a = EfContact { edge: e, face: f }.cells(Operand::A);
    let in_b = EfContact { edge: e, face: f }.cells(Operand::B);
    let order = [vv, on_a, on_b, ee, ff, in_a, in_b];
    let set = ReductionContacts::of_rows(
        order
            .iter()
            .rev()
            .enumerate()
            .map(|(k, &p)| Cited::new(p, Cites::decided(u32::try_from(k).unwrap()))),
    );
    assert_eq!(
        set.rows().map(|(p, _)| p).collect::<Vec<_>>(),
        order,
        "the fixed kind order"
    );
    assert_eq!(set.vv().count(), 1, "one vertex pair");
    for o in Operand::BOTH {
        assert_eq!(set.on(o).count(), 1, "{o:?}'s one vertex on a face");
        assert_eq!(set.lying_in(o).count(), 1, "{o:?}'s one edge in a face");
    }
}

/// **A shape no writer writes is a defect, read loudly.** Two vertices
/// of one operand are no row of the working set. Red where the set
/// takes such a row.
#[test]
#[should_panic(expected = "the reduction's working set holds no row")]
fn a_shape_no_writer_writes_panics() {
    let v = VertexKey::default();
    let _ = ReductionContacts::of_rows([Cited::new(
        [(Operand::A, Cell::Vertex(v)), (Operand::A, Cell::Vertex(v))],
        Cites::decided(0),
    )]);
}
