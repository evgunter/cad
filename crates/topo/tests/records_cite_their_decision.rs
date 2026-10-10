//! **Every contact record a boolean ships cites the decision that backs
//! it** (D1 (ii), D10 Coincidence): a vertex identity the reduction
//! decided by a margin is recorded as a `VertexFusion` row when a record
//! citing it survives into the result, and a record carried in from an
//! operand cites that operand's record.
#![allow(clippy::panic, clippy::expect_used)]

use crate::common;
use common::{brick, finished};
use geom_core::Tol;
use topo::{
    AtRestBody, Backing, BooleanBody, BooleanDeclarations, BooleanResult, Cell, Coincidence,
    ContactClass, DecisionSite, Operand, Relation, RowCell, union_with,
};

fn tol() -> Tol {
    Tol::witness()
}

fn box_of(x: (f64, f64), y: (f64, f64), z: (f64, f64), what: &str) -> AtRestBody<f64> {
    finished(what, brick(x, y, z, tol()), tol())
}

fn union(
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    decls: &BooleanDeclarations,
) -> BooleanBody<f64> {
    match union_with(a, b, decls, tol()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("the union builds: {other:?}"),
    }
}

/// The vertex of `body` at `at`.
fn vertex_at(body: &topo::Body<f64>, at: [f64; 3]) -> topo::VertexKey {
    body.vertex_points()
        .find(|(_, p)| (p.x - at[0]).abs() + (p.y - at[1]).abs() + (p.z - at[2]).abs() < 1e-12)
        .map(|(v, _)| v)
        .unwrap_or_else(|| panic!("a vertex at {at:?}"))
}

/// Every citation of `r`'s records, as the rows they name; panics on a
/// citation that names no row or another operation's record.
fn cited_rows(r: &BooleanBody<f64>) -> Vec<Vec<&Coincidence>> {
    r.contacts
        .cites()
        .map(|cites| {
            cites
                .iter()
                .map(|b| match b {
                    Backing::Decided(k) => r
                        .coincidences
                        .get(k as usize)
                        .unwrap_or_else(|| panic!("row {k} of {}", r.coincidences.len())),
                    Backing::Carried { .. } => panic!("no record is carried here: {b:?}"),
                })
                .collect()
        })
        .collect()
}

/// **Two boxes kissing at a corner**: the one record is the vertex pair
/// at (1, 1, 1), and it cites one `VertexFusion` row naming the two
/// operands' corner vertices, in their own keys, decided Zero. Red if
/// the reduction's decision is not emitted (no row), or if the record
/// cites a row of the wrong cells.
#[test]
fn a_corner_kiss_cites_the_vertex_fusion_that_decided_it() {
    let a = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), "a");
    let b = box_of((1.0, 2.0), (1.0, 2.0), (1.0, 2.0), "b");
    let r = union(&a, &b, &BooleanDeclarations::none());
    assert_eq!(r.contacts.vv.len(), 1, "one record: {:?}", r.contacts);
    let rows = cited_rows(&r);
    assert_eq!(rows.len(), 1, "one record, one citation set");
    let [row] = rows[0][..] else {
        panic!("the kiss cites one decision: {:?}", rows[0]);
    };
    assert_eq!(row.site, DecisionSite::VertexFusion);
    assert_eq!(row.relation, Relation::OnCarrier);
    let (va, vb) = (
        vertex_at(&a, [1.0, 1.0, 1.0]),
        vertex_at(&b, [1.0, 1.0, 1.0]),
    );
    let mut cells = row.cells;
    cells.sort();
    let mut want = [
        RowCell::Input {
            input: Operand::A,
            cell: Cell::Vertex(va),
        },
        RowCell::Input {
            input: Operand::B,
            cell: Cell::Vertex(vb),
        },
    ];
    want.sort();
    assert_eq!(cells, want, "the row names the two operands' corners");
    let vertex_rows = r
        .coincidences
        .iter()
        .filter(|c| c.site == DecisionSite::VertexFusion)
        .count();
    assert_eq!(
        vertex_rows, 1,
        "no decision is emitted that no record cites"
    );
}

/// **A transverse overlap records no vertex identity**: every vertex the
/// reduction placed on the other operand is a pierce or lies on the seam,
/// so no record survives and no `VertexFusion` row is emitted (D1: an ON
/// verdict that only places topology is not a coincidence). Red if every
/// push of the sweep is emitted as a row.
#[test]
fn a_crossing_union_emits_no_vertex_fusion() {
    let a = box_of((0.0, 2.0), (0.0, 2.0), (0.0, 2.0), "a");
    let b = box_of((1.0, 3.0), (1.0, 3.0), (1.0, 3.0), "b");
    let r = union(&a, &b, &BooleanDeclarations::none());
    assert!(r.contacts.cell_pairs().next().is_none(), "{:?}", r.contacts);
    assert!(
        !r.coincidences
            .iter()
            .any(|c| c.site == DecisionSite::VertexFusion),
        "{:?}",
        r.coincidences
    );
}

/// **A record carried in cites the operand's record**: the kiss's vertex
/// pair, carried into a union with a far box, comes out citing record 0
/// of operand A's list, and no decision of the second union's. Red if a
/// carried record is re-cited as this op's own decision, or loses its
/// index.
#[test]
fn a_carried_record_cites_its_operands_record() {
    let a = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), "a");
    let b = box_of((1.0, 2.0), (1.0, 2.0), (1.0, 2.0), "b");
    let kiss = union(&a, &b, &BooleanDeclarations::none());
    let far = box_of((5.0, 6.0), (5.0, 6.0), (5.0, 6.0), "far");
    let decls = BooleanDeclarations {
        carried_a: kiss.contacts.carried(ContactClass::Rest),
        ..BooleanDeclarations::none()
    };
    let r = union(&kiss.body, &far, &decls);
    assert_eq!(r.contacts.vv.len(), 1, "{:?}", r.contacts);
    assert_eq!(
        r.contacts.vv[0].cites,
        topo::Cites::one(Backing::Carried {
            input: 0,
            record: 0,
        }),
        "the carried pair cites operand A's record 0"
    );
    assert!(
        !r.coincidences
            .iter()
            .any(|c| c.site == DecisionSite::VertexFusion),
        "the carry decides nothing: {:?}",
        r.coincidences
    );
}

/// A unit-square prism over z ∈ [0, 1], its square turned `turn`
/// radians about the z axis through the origin and its centre at
/// `centre` before the turn.
fn turned_square(centre: (f64, f64), turn: f64, what: &str) -> AtRestBody<f64> {
    let (c, s) = (turn.cos(), turn.sin());
    let corners = [(-0.5, -0.5), (0.5, -0.5), (0.5, 0.5), (-0.5, 0.5)]
        .map(|(x, y)| (centre.0 + x, centre.1 + y))
        .map(|(x, y)| (c * x - s * y, s * x + c * y));
    finished(what, common::prism_z(&corners, 0.0, 1.0, tol()).body, tol())
}

/// The face-pair rows of `r`: the rows naming a face of each operand.
fn face_pair_rows(r: &BooleanResult<f64>) -> usize {
    let BooleanResult::Body(r) = r else {
        panic!("the op builds a body: {r:?}");
    };
    r.coincidences
        .iter()
        .filter(|row| {
            matches!(
                row.cells,
                [
                    RowCell::Input {
                        cell: Cell::Face(_),
                        ..
                    },
                    RowCell::Input {
                        cell: Cell::Face(_),
                        ..
                    }
                ]
            )
        })
        .count()
}

/// **A face pair whose faces never meet records no row, in any frame**:
/// two unit blocks 0.2 apart with coplanar tops and bottoms. Turned 45°
/// about z, their faces' boxes overlap and the glue door decides the
/// tops one carrier and the bottoms one carrier; the faces never meet,
/// so the glue takes no effect and no row is kept. Red if the rows
/// follow the boxes (0 axis-aligned, 2 turned).
#[test]
fn blocks_apart_record_no_row_axis_aligned_or_turned() {
    for turn in [0.0, core::f64::consts::FRAC_PI_4] {
        let a = turned_square((0.0, 0.0), turn, "a");
        let b = turned_square((1.2, 0.0), turn, "b");
        let r = union_with(&a, &b, &BooleanDeclarations::none(), tol())
            .unwrap_or_else(|e| panic!("the union at turn {turn}: {e:?}"));
        assert_eq!(face_pair_rows(&r), 0, "turn {turn}: {r:?}");
    }
}

/// **Blocks side by side record the same rows in any frame**: two unit
/// blocks sharing a side face, their tops, bottoms, fronts and backs
/// continuing across it. Every face pair that meets is recorded, and the same number of
/// them axis-aligned and turned 45°, though the turned faces' boxes
/// offer more pairs.
#[test]
fn blocks_side_by_side_record_equal_rows_axis_aligned_and_turned() {
    let rows = [0.0, core::f64::consts::FRAC_PI_4].map(|turn| {
        let a = turned_square((0.0, 0.0), turn, "a");
        let b = turned_square((1.0, 0.0), turn, "b");
        let r = union_with(&a, &b, &BooleanDeclarations::none(), tol())
            .unwrap_or_else(|e| panic!("the union at turn {turn}: {e:?}"));
        face_pair_rows(&r)
    });
    assert_eq!(rows[0], rows[1], "axis-aligned vs turned: {rows:?}");
    assert_eq!(
        rows[0], 5,
        "the side rest, and the tops, bottoms, fronts and backs continuing"
    );
}

/// **A block standing apart in an L prism's notch records no row**: the
/// L's top and bottom faces' boxes cover the notch, so the glue door
/// decides the block's faces one carrier with the L's; no such pair
/// meets.
#[test]
fn a_block_apart_in_an_l_notch_records_no_row() {
    let l = finished(
        "the L-prism",
        common::prism_z(
            &[
                (0.0, 0.0),
                (4.0, 0.0),
                (4.0, 2.0),
                (2.0, 2.0),
                (2.0, 4.0),
                (0.0, 4.0),
            ],
            0.0,
            1.0,
            tol(),
        )
        .body,
        tol(),
    );
    // Level with the L (tops and bottoms one carrier, aligned), and
    // raised onto the plane of the L's top (the block's bottom on it,
    // opposed).
    for z in [(0.0, 1.0), (1.0, 2.0)] {
        let block = box_of((2.5, 3.5), (2.5, 3.5), z, "the block");
        let r = union_with(&l, &block, &BooleanDeclarations::none(), tol())
            .unwrap_or_else(|e| panic!("the union at {z:?}: {e:?}"));
        assert_eq!(face_pair_rows(&r), 0, "{z:?}: {r:?}");
    }
}

/// **A pair a vertex group implies cites only shortest chains, when one
/// result cell is reached from both operands.** The corner kiss `K`
/// unioned with a block `C` standing beside `A` with its corner at the
/// kiss: `C`'s corner is decided one with each kissing vertex, and in
/// the result `A`'s kissing vertex and `C`'s corner are one cell. The
/// pair of `B`'s kissing vertex and that cell is one hop from `C`'s
/// corner, so it cites that one decision; the decision joining `A`'s
/// vertex to `C`'s lies only on a longer chain. Red if a result cell is
/// read once per side and the pair's citations are their union.
#[test]
fn a_cell_reached_from_both_sides_cites_its_shortest_chain() {
    let a = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), "a");
    let b = box_of((1.0, 2.0), (1.0, 2.0), (1.0, 2.0), "b");
    let kiss = union(&a, &b, &BooleanDeclarations::none());
    let c = box_of((1.0, 2.0), (0.0, 1.0), (0.0, 1.0), "c");
    let r = union(&kiss.body, &c, &BooleanDeclarations::none());
    let cited = cited_rows(&r);
    assert_eq!(cited.len(), 2, "two records: {:?}", r.contacts);
    for rows in &cited {
        assert_eq!(
            rows.len(),
            1,
            "each record cites its one-hop decision: {rows:?}"
        );
    }
    let fusions = r
        .coincidences
        .iter()
        .filter(|c| c.site == DecisionSite::VertexFusion)
        .count();
    assert_eq!(fusions, 2, "only the cited decisions: {:?}", r.coincidences);
}

/// **A carried vertex-on-face record cites its operand's record**: a
/// unit cube balanced on one corner on the middle of a block's top
/// face, its corner the one point it touches. The kiss's one record is
/// that vertex on the block's top, citing the decision that placed it;
/// carried into a union with a far box, it comes out citing the kiss's
/// record by index and nothing of the second union's. Red if the
/// carried vertex-on-face row loses its citation or is re-decided.
#[test]
fn a_carried_vertex_on_face_record_cites_its_operands_record() {
    use geom_core::{Affine3, Point3, Vec3};
    let a = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), "a");
    let cube = brick((-0.5, 0.5), (-0.5, 0.5), (-0.5, 0.5), tol());
    // The corner (−½, −½, −½) turned to point straight down.
    let turn = Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, -1.0, 0.0),
        (1.0_f64 / 3.0_f64.sqrt()).acos(),
    );
    let turned = topo::transform_rigid(&cube, &turn, tol()).expect("a rigid turn");
    let low = turned
        .vertex_points()
        .map(|(_, p)| p)
        .min_by(|p, q| p.z.total_cmp(&q.z))
        .expect("a vertex");
    let lift = Affine3::translation(Vec3::new(0.5 - low.x, 0.5 - low.y, 1.0 - low.z));
    let on_corner = finished(
        "the cube on its corner",
        topo::transform_rigid(&turned, &lift, tol()).expect("a rigid lift"),
        tol(),
    );
    let kiss = union(&a, &on_corner, &BooleanDeclarations::none());
    assert_eq!(
        (
            kiss.contacts.a_on_b.len() + kiss.contacts.b_on_a.len(),
            kiss.contacts.vv.len()
        ),
        (1, 0),
        "one vertex-on-face record: {:?}",
        kiss.contacts
    );
    let k = kiss
        .contacts
        .rows()
        .position(|((x, y), _)| matches!((x, y), (Cell::Vertex(_), Cell::Face(_))))
        .expect("the vertex-on-face record");
    let far = box_of((5.0, 6.0), (5.0, 6.0), (5.0, 6.0), "far");
    let decls = BooleanDeclarations {
        carried_a: kiss.contacts.carried(ContactClass::Rest),
        ..BooleanDeclarations::none()
    };
    let r = union(&kiss.body, &far, &decls);
    let carried: Vec<_> = r.contacts.rows().map(|(_, cites)| cites.clone()).collect();
    assert_eq!(
        carried,
        [topo::Cites::one(Backing::Carried {
            input: 0,
            record: u32::try_from(k).expect("small"),
        })],
        "the carried vertex-on-face record cites the kiss's record {k}: {:?}",
        r.contacts
    );
}
