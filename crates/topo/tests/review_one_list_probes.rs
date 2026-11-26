//! Review probes for PR 7782 (one list of contact records). Not for merge.

use topo::{
    CarriedRecord, Cell, Cited, Cites, Contact, ContactKind, EdgeKey, EfContact, FaceKey, Operand,
    OperandRecords, ReductionContacts, TangentFaces, VertexKey, VfContact, VvContact,
};

fn keys<K: Default + Copy>() -> K {
    K::default()
}

/// P1: a public constructor panics on a shape no writer writes.
#[test]
#[should_panic(expected = "the reduction's working set holds no row")]
fn p1_from_iter_with_a_one_operand_vertex_pair_panics() {
    let v: VertexKey = keys();
    let _ = ReductionContacts::from_iter([Cited::new(
        [(Operand::A, Cell::Vertex(v)), (Operand::A, Cell::Vertex(v))],
        Cites::decided(0),
    )]);
}

/// P2: OperandRecords filed out of kind order reads back in carry order,
/// each kind in filing order.
#[test]
fn p2_operand_records_file_by_kind() {
    let body = topo::Body::<f64>::new();
    let mut vs = topo::Body::<f64>::new();
    let _ = &mut vs;
    let f0: FaceKey = keys();
    let e0: EdgeKey = keys();
    let v0: VertexKey = keys();
    // Distinct cells of distinct kinds (default keys of different arenas).
    let ff = Contact::new(&body, Cell::Face(f0), Cell::Edge(e0)).unwrap(); // edge–face
    let vf = Contact::new(&body, Cell::Vertex(v0), Cell::Face(f0)).unwrap();
    let ve = Contact::new(&body, Cell::Edge(e0), Cell::Vertex(v0)).unwrap();
    let rec = |c, r| CarriedRecord {
        contact: c,
        record: r,
    };
    let r: OperandRecords = [rec(ff, 0), rec(ve, 1), rec(vf, 2), rec(ve, 3)]
        .into_iter()
        .collect();
    let order: Vec<u32> = r.rows().iter().map(|c| c.record).collect();
    assert_eq!(order, [2, 1, 3, 0], "vf, ve, ve (filing order), ef");
    assert!(matches!(
        r.rows()[0].contact.kind(),
        ContactKind::VertexFace(..)
    ));
}

/// P3: each A×B shape round-trips through `cells` into its own read
/// only, and `rows()` reads in fixed kind order whatever the push order.
#[test]
fn p3_rows_round_trip_and_kind_order() {
    let (v, e, f): (VertexKey, EdgeKey, FaceKey) = (keys(), keys(), keys());
    let c = |p| Cited::new(p, Cites::decided(0));
    let rc = ReductionContacts::from_iter([
        c(EfContact { edge: e, face: f }.cells(Operand::B)),
        c(TangentFaces { a: f, b: f }.cells()),
        c(VfContact { vertex: v, face: f }.cells(Operand::B)),
        c(VvContact { a: v, b: v }.cells()),
        c(EfContact { edge: e, face: f }.cells(Operand::A)),
        c(VfContact { vertex: v, face: f }.cells(Operand::A)),
    ]);
    assert_eq!(rc.vv().count(), 1);
    assert_eq!(rc.on(Operand::A).count(), 1);
    assert_eq!(rc.on(Operand::B).count(), 1);
    assert_eq!(rc.ff().count(), 1);
    assert_eq!(rc.lying_in(Operand::A).count(), 1);
    assert_eq!(rc.lying_in(Operand::B).count(), 1);
    let shapes: Vec<_> = rc.rows().map(|(p, _)| p).collect();
    assert_eq!(
        shapes,
        [
            VvContact { a: v, b: v }.cells(),
            VfContact { vertex: v, face: f }.cells(Operand::A),
            VfContact { vertex: v, face: f }.cells(Operand::B),
            TangentFaces { a: f, b: f }.cells(),
            EfContact { edge: e, face: f }.cells(Operand::A),
            EfContact { edge: e, face: f }.cells(Operand::B),
        ]
    );
}

/// P4/P5: two working sets equal under `==` but filed in other orders:
/// equal, and their Debug differs.
#[test]
fn p4_equal_sets_print_differently() {
    let (v, f): (VertexKey, FaceKey) = (keys(), keys());
    let c = |p| Cited::new(p, Cites::decided(0));
    let x = ReductionContacts::from_iter([
        c(VvContact { a: v, b: v }.cells()),
        c(VfContact { vertex: v, face: f }.cells(Operand::A)),
    ]);
    let y = ReductionContacts::from_iter([
        c(VfContact { vertex: v, face: f }.cells(Operand::A)),
        c(VvContact { a: v, b: v }.cells()),
    ]);
    assert_eq!(x, y);
    let (dx, dy) = (format!("{x:?}"), format!("{y:?}"));
    eprintln!("x = {dx}\ny = {dy}");
    assert_ne!(dx, dy, "P5: equal values, different Debug");
}
