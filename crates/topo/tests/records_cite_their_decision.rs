//! **Every contact record a boolean ships cites the decision that backs
//! it** (D1 (ii), D10 Coincidence): a vertex identity the reduction
//! decided by a margin is recorded as a `VertexFusion` row when a record
//! citing it survives into the result or the result pinches at it, not
//! where every copy on the pinch's point key is one an operand already
//! held there, and a record carried in from an operand cites that
//! operand's record.
#![allow(clippy::panic, clippy::expect_used)]

use crate::common;
use common::{brick, finished};
use geom_core::Tol;
use topo::{
    AtRestBody, Backing, Body, BooleanBody, BooleanInputs, BooleanResult, Cell, Coincidence,
    DecisionSite, Operand, Relation, RowCell, union,
};

fn tol() -> Tol {
    Tol::witness()
}

fn box_of(x: (f64, f64), y: (f64, f64), z: (f64, f64), what: &str) -> AtRestBody<f64> {
    finished(what, brick(x, y, z, tol()), tol())
}

/// `a ∪ b` under `inputs`, which must build a body.
fn united(a: &AtRestBody<f64>, b: &AtRestBody<f64>, inputs: &BooleanInputs) -> BooleanBody<f64> {
    let built = topo::boolean_op_with(
        topo::BooleanOp::Union,
        a,
        b,
        inputs,
        topo::SweepStrategy::Realized,
        tol(),
    );
    match built {
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
    let r = united(&a, &b, &BooleanInputs::default());
    assert_eq!(
        r.contacts.vertex_vertex().count(),
        1,
        "one record: {:?}",
        r.contacts
    );
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
    let r = united(&a, &b, &BooleanInputs::default());
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
    let kiss = united(&a, &b, &BooleanInputs::default());
    let far = box_of((5.0, 6.0), (5.0, 6.0), (5.0, 6.0), "far");
    let carried = BooleanInputs {
        records_a: kiss.contacts.carried(),
        ..BooleanInputs::default()
    };
    let r = united(&kiss.body, &far, &carried);
    assert_eq!(r.contacts.vertex_vertex().count(), 1, "{:?}", r.contacts);
    assert_eq!(
        r.contacts.rows()[0].cites,
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
        let r = union(&a, &b, tol()).unwrap_or_else(|e| panic!("the union at turn {turn}: {e:?}"));
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
        let r = union(&a, &b, tol()).unwrap_or_else(|e| panic!("the union at turn {turn}: {e:?}"));
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
        let r = union(&l, &block, tol()).unwrap_or_else(|e| panic!("the union at {z:?}: {e:?}"));
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
/// vertex to `C`'s lies only on a longer chain. `B` and `C` also touch
/// along `B`'s edge over x ∈ (1, 2), whose row cites the decision at
/// each of its two ends. Red if a result cell is read once per side
/// and the pair's citations are their union, or if the touch along the
/// edge goes unrecorded.
#[test]
fn a_cell_reached_from_both_sides_cites_its_shortest_chain() {
    let a = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), "a");
    let b = box_of((1.0, 2.0), (1.0, 2.0), (1.0, 2.0), "b");
    let kiss = united(&a, &b, &BooleanInputs::default());
    let c = box_of((1.0, 2.0), (0.0, 1.0), (0.0, 1.0), "c");
    let r = united(&kiss.body, &c, &BooleanInputs::default());
    let cited = cited_rows(&r);
    assert_eq!(
        (cited.len(), r.contacts.edge_edge().count()),
        (3, 1),
        "the two vertex records and the edge-edge row: {:?}",
        r.contacts
    );
    let kinds: Vec<topo::ContactKind> = r.contacts.kinds().collect();
    let mut ends = Vec::new();
    for (rows, kind) in cited.iter().zip(&kinds) {
        if matches!(kind, topo::ContactKind::EdgeEdge(..)) {
            continue;
        }
        assert_eq!(
            rows.len(),
            1,
            "each vertex record cites its one-hop decision: {rows:?}"
        );
        ends.extend(rows.iter().copied());
    }
    let edge = cited
        .iter()
        .zip(&kinds)
        .find(|(_, k)| matches!(k, topo::ContactKind::EdgeEdge(..)))
        .map(|(rows, _)| rows)
        .expect("an edge-edge row");
    assert!(
        edge.len() == ends.len() && ends.iter().all(|e| edge.contains(e)),
        "the edge-edge row cites the decisions at its two ends: {edge:?}, ends {ends:?}"
    );
    let fusions = r
        .coincidences
        .iter()
        .filter(|c| c.site == DecisionSite::VertexFusion)
        .count();
    // Every vertex identity the union decided is a row, cited or not:
    // A's four corners on C's face, B's kissing vertex on C's corner and
    // the far end of B's edge along C's.
    assert_eq!(
        fusions, 6,
        "every vertex identity decided: {:?}",
        r.coincidences
    );
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
    use geom_core::{Band, Point3, RigidMotion, UnitVec3, Vec3};
    let a = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), "a");
    let cube = brick((-0.5, 0.5), (-0.5, 0.5), (-0.5, 0.5), tol());
    // The corner (−½, −½, −½) turned to point straight down.
    let band = Band::linear(tol()).expect("the run's band");
    let axis =
        UnitVec3::new(Vec3::new(1.0, -1.0, 0.0), "test_turn_axis", band).expect("a definite axis");
    let turn = RigidMotion::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        axis,
        (1.0_f64 / 3.0_f64.sqrt()).acos(),
    )
    .expect("a finite turn");
    let turned = topo::transform_rigid(&cube, &turn, tol()).expect("a rigid turn");
    let low = turned
        .vertex_points()
        .map(|(_, p)| p)
        .min_by(|p, q| p.z.total_cmp(&q.z))
        .expect("a vertex");
    let lift = RigidMotion::translation(Vec3::new(0.5 - low.x, 0.5 - low.y, 1.0 - low.z))
        .expect("a finite lift");
    let on_corner = finished(
        "the cube on its corner",
        topo::transform_rigid(&turned, &lift, tol()).expect("a rigid lift"),
        tol(),
    );
    let kiss = united(&a, &on_corner, &BooleanInputs::default());
    assert_eq!(
        (
            kiss.contacts.vertex_face().count(),
            kiss.contacts.vertex_vertex().count()
        ),
        (1, 0),
        "one vertex-on-face record: {:?}",
        kiss.contacts
    );
    let k = kiss
        .contacts
        .kinds()
        .position(|k| matches!(k, topo::ContactKind::VertexFace(..)))
        .expect("the vertex-on-face record");
    let far = box_of((5.0, 6.0), (5.0, 6.0), (5.0, 6.0), "far");
    let carried = BooleanInputs {
        records_a: kiss.contacts.carried(),
        ..BooleanInputs::default()
    };
    let r = united(&kiss.body, &far, &carried);
    let carried: Vec<_> = r.contacts.cites().cloned().collect();
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

/// The L-prism's reflex top corner, where the pinch fixture's cube meets
/// it.
const PINCH_AT: [f64; 3] = [1.0, 1.0, 1.0];

/// The cube of side `side` whose bottom face lies on the plane through
/// [`PINCH_AT`] with normal `m`, on `m`'s side, centred on it there. At `m = (1, 1.3, 0.7)`
/// the L-prism's corner there has two runs out of the cube (its reflex
/// top sector and its −z edge), so the prism's part inside the cube is
/// two lobes touching at the corner.
fn cube_beyond(m: [f64; 3], side: f64) -> AtRestBody<f64> {
    let unit = |v: [f64; 3]| {
        let n = v.iter().map(|c| c * c).sum::<f64>().sqrt();
        v.map(|c| c / n)
    };
    let m = unit(m);
    let u = unit([m[1], -m[0], 0.0]);
    let w = [
        m[1] * u[2] - m[2] * u[1],
        m[2] * u[0] - m[0] * u[2],
        m[0] * u[1] - m[1] * u[0],
    ];
    let v = PINCH_AT;
    let body = topo::test_support::mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (side * (x - 0.5), side * (y - 0.5), side * z);
            geom_core::Point3::new(
                v[0] + a * u[0] + b * w[0] + c * m[0],
                v[1] + a * u[1] + b * w[1] + c * m[1],
                v[2] + a * u[2] + b * w[2] + c * m[2],
            )
        },
        tol(),
    );
    finished("the cube", body, tol())
}

/// **A pinch records the vertex identity that made it** (DESIGN, the
/// reduction's coincidence discipline: an ON verdict that makes pieces
/// of one result touch is a coincidence). The cube's bottom plane passes
/// through the L-prism's reflex corner, decided Zero, and the
/// intersection and the union keep one vertex per cone there: two
/// vertices on one point key, which no record states. Each carries one
/// `VertexFusion` row, the prism's corner on the cube's bottom face, in
/// either operand order. The prism minus the cube keeps one vertex there
/// and records the same decision: a row is the decision, whatever the
/// result made of the vertex. Red if a vertex identity no surviving
/// record cites is dropped.
#[test]
fn a_pinch_records_the_vertex_identity_that_made_it() {
    let prism = topo::test_support::prism::<f64>(
        &[
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (0.0, 2.0),
        ],
        1.0,
        tol(),
    );
    let corner = prism.top[3];
    let prism = finished("the prism", prism.body, tol());
    let cube = cube_beyond([1.0, 1.3, 0.7], 4.0);
    let bottom = cube
        .faces()
        .find(|(_, f)| match cube.get_surface(f.surface) {
            Some(topo::Surface::Plane { origin, normal, .. }) => {
                let p = geom_core::Point3::new(PINCH_AT[0], PINCH_AT[1], PINCH_AT[2]);
                (p - *origin).dot(*normal).abs() < 1e-12
            }
            _ => false,
        })
        .map(|(f, _)| f)
        .expect("the cube's face through the corner");
    let vertex_rows = |r: &BooleanBody<f64>| -> Vec<Coincidence> {
        r.coincidences
            .iter()
            .filter(|c| c.site == DecisionSite::VertexFusion)
            .copied()
            .collect()
    };
    let at_corner = |r: &BooleanBody<f64>| -> Vec<topo::VertexKey> {
        r.body
            .vertex_points()
            .filter(|(_, p)| (p.x, p.y, p.z) == (PINCH_AT[0], PINCH_AT[1], PINCH_AT[2]))
            .map(|(v, _)| v)
            .collect()
    };
    for (order, x, y, prism_is) in [
        ("prism-cube", &prism, &cube, Operand::A),
        ("cube-prism", &cube, &prism, Operand::B),
    ] {
        let cube_is = match prism_is {
            Operand::A => Operand::B,
            Operand::B => Operand::A,
        };
        for (op, run) in [
            ("intersect", topo::intersect::<f64> as fn(_, _, _) -> _),
            ("union", topo::union::<f64>),
        ] {
            let what = format!("{order} {op}");
            let r = match run(x, y, tol()) {
                Ok(BooleanResult::Body(r)) => r,
                other => panic!("{what} builds: {other:?}"),
            };
            let copies = at_corner(&r);
            assert_eq!(copies.len(), 2, "{what}: one vertex per cone at the corner");
            let point = |v| r.body.get_vertex(v).expect("a live vertex").point;
            assert_eq!(
                point(copies[0]),
                point(copies[1]),
                "{what}: on one point key"
            );
            assert!(r.contacts.rows().is_empty(), "{what}: {:?}", r.contacts);
            let rows = vertex_rows(&r);
            assert_eq!(rows.len(), 1, "{what}: one vertex identity: {rows:?}");
            assert_eq!(
                (rows[0].cells, rows[0].relation),
                (
                    [
                        RowCell::Input {
                            input: prism_is,
                            cell: Cell::Vertex(corner),
                        },
                        RowCell::Input {
                            input: cube_is,
                            cell: Cell::Face(bottom),
                        },
                    ],
                    Relation::OnCarrier,
                ),
                "{what}: the prism's corner on the cube's bottom"
            );
            assert_eq!(
                rows[0].margin.kind(),
                geom_core::MarginKind::Value,
                "{what}: the margin the reduction decided Zero"
            );
        }
    }
    let r = match topo::subtract(&prism, &cube, tol()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("the subtraction builds: {other:?}"),
    };
    assert_eq!(
        at_corner(&r).len(),
        1,
        "the subtraction keeps one vertex there"
    );
    let rows = vertex_rows(&r);
    assert!(
        matches!(
            rows.as_slice(),
            [c] if c.cells == [
                RowCell::Input { input: Operand::A, cell: Cell::Vertex(corner) },
                RowCell::Input { input: Operand::B, cell: Cell::Face(bottom) },
            ]
        ),
        "the subtraction records the corner on the cube's bottom: {rows:?}"
    );
}

/// **Two blocks touching along a line cut out of a slab refuse.** The
/// slab would hold the contact as two coincident edges with their
/// edge-edge record; the complement of the two blocks inside the slab
/// pairs one block's faces with the other's, and ∖ does not build that
/// re-pairing, so the result's two edges bound 270° each, overlapping
/// by 180°, and the result gate refuses `LineWedges`. The record's lineage at a
/// plate's pierce through that slab returns with the re-pairing
/// (`work/topo/revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet.md`).
#[test]
fn cutting_a_touching_pair_out_of_a_slab_refuses_its_revert() {
    let slab = box_of((-1.0, 4.0), (-1.0, 3.0), (0.5, 2.5), "the slab");
    let p1 = box_of((1.0, 1.5), (-2.0, 1.0), (0.3, 2.0), "p1");
    let p2 = box_of((1.5, 2.0), (1.0, 4.0), (0.27, 1.73), "p2");
    let blocks = united(&p1, &p2, &BooleanInputs::default());
    let inputs = BooleanInputs {
        records_b: blocks.contacts.carried(),
        ..BooleanInputs::default()
    };
    let got = topo::boolean_op_with(
        topo::BooleanOp::Subtract,
        &slab,
        &blocks.body,
        &inputs,
        topo::SweepStrategy::Realized,
        tol(),
    );
    assert!(
        matches!(
            got,
            Err(topo::BooleanError::LineWedges {
                fault: topo::LineWedgeFault::Overlap { .. },
                ..
            })
        ),
        "the slab less the blocks: {got:?}, not the overlap's refusal"
    );
}

/// The L-prism whose reflex top corner is [`PINCH_AT`].
fn ell() -> AtRestBody<f64> {
    let prism = topo::test_support::prism::<f64>(
        &[
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (0.0, 2.0),
        ],
        1.0,
        tol(),
    );
    finished("the prism", prism.body, tol())
}

/// The point keys of `r`'s vertices at [`PINCH_AT`], and its
/// `VertexFusion` rows.
fn at_pinch(r: &BooleanBody<f64>) -> (Vec<topo::PointKey>, usize) {
    let keys = r
        .body
        .vertex_points()
        .filter(|(_, p)| (p.x, p.y, p.z) == (PINCH_AT[0], PINCH_AT[1], PINCH_AT[2]))
        .map(|(v, _)| r.body.get_vertex(v).expect("a live vertex").point)
        .collect();
    let rows = r
        .coincidences
        .iter()
        .filter(|c| c.site == DecisionSite::VertexFusion)
        .count();
    (keys, rows)
}

/// `r`'s `VertexFusion` rows, each as the inputs `x`, `y` hold its
/// cells: whether a vertex cell of it sits at [`PINCH_AT`], and whether
/// its other cell is a face.
fn corner_rows(r: &BooleanBody<f64>, x: &Body<f64>, y: &Body<f64>) -> Vec<(bool, bool)> {
    let at_corner = |input: Operand, v| {
        let body = if input == Operand::A { x } else { y };
        body.vertex_points()
            .any(|(k, p)| k == v && (p.x, p.y, p.z) == (PINCH_AT[0], PINCH_AT[1], PINCH_AT[2]))
    };
    r.coincidences
        .iter()
        .filter(|c| c.site == DecisionSite::VertexFusion)
        .map(|c| {
            let corner = c.cells.iter().any(|end| {
                matches!(*end, RowCell::Input { input, cell: Cell::Vertex(v) } if at_corner(input, v))
            });
            let onto_face = c
                .cells
                .iter()
                .any(|end| matches!(end, RowCell::Input { cell: Cell::Face(_), .. }));
            (corner, onto_face)
        })
        .collect()
}

fn built(what: &str, r: Result<BooleanResult<f64>, topo::BooleanError>) -> BooleanBody<f64> {
    match r {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("{what} builds: {other:?}"),
    }
}

/// **A decision onto an op's face at copies an operand already held is
/// a row** (`boolean::reduce::PendingRow`): the prism's intersection
/// with the cube is two lobes on one point key at the corner. Cut by a
/// halfspace whose face passes through the corner, at three normals
/// where both lobes stay, in either operand order, the result keeps the
/// two copies on one key and records the copies decided onto the
/// halfspace's face; so does the union with the halfspace at
/// `(−1, −1, 0)`, which puts a vertex of its own on that key beside an
/// inherited copy. Red if a decision is dropped because the copies were
/// the operand's.
#[test]
fn a_decision_at_copies_an_operand_already_held_is_a_row() {
    let pinched = built(
        "the two lobes",
        topo::intersect(&ell(), &cube_beyond([1.0, 1.3, 0.7], 4.0), tol()),
    );
    let (keys, _) = at_pinch(&pinched);
    assert!(
        keys.len() == 2 && keys[0] == keys[1],
        "the lobes hold two copies on one key: {keys:?}"
    );
    let lobes = pinched.body;
    // The two copies at the corner, each decided on the halfspace's
    // face, in every case; at `(1, 1, 0)` the face also takes two of the
    // lobes' vertices off the corner.
    for (m, all) in [
        ([1.0, 1.0, 0.0], 4),
        ([-0.3, 1.0, -0.5], 2),
        ([1.0, -0.2, -0.4], 2),
    ] {
        let half = cube_beyond(m, 8.0);
        for (order, x, y) in [("lobes-half", &lobes, &half), ("half-lobes", &half, &lobes)] {
            let what = format!("{m:?} {order} intersect");
            let r = built(&what, topo::intersect(x, y, tol()));
            let (keys, _) = at_pinch(&r);
            assert!(
                keys.len() == 2 && keys[0] == keys[1],
                "{what}: both copies stay on one key: {keys:?}"
            );
            let rows = corner_rows(&r, x, y);
            assert!(
                rows.len() == all
                    && rows.iter().filter(|&&(corner, _)| corner).count() == 2
                    && rows.iter().all(|&(_, onto_face)| onto_face),
                "{what}: both copies decided onto the face, {all} rows in all: {rows:?}"
            );
        }
    }
    let half = cube_beyond([-1.0, -1.0, 0.0], 8.0);
    for (order, x, y) in [("lobes-half", &lobes, &half), ("half-lobes", &half, &lobes)] {
        let what = format!("{order} union");
        let r = built(&what, topo::union(x, y, tol()));
        let (keys, _) = at_pinch(&r);
        assert!(
            keys.len() == 2 && keys[0] == keys[1],
            "{what}: an inherited copy and the op's on one key: {keys:?}"
        );
        let rows = corner_rows(&r, x, y);
        assert!(
            rows.len() == 4
                && rows.iter().filter(|&&(corner, _)| corner).count() == 2
                && rows.iter().all(|&(_, onto_face)| onto_face),
            "{what}: both vertices at the corner decided onto the face, four rows in all: {rows:?}"
        );
    }
}

/// **Every pinch over the poses records a row**: the prism against the
/// side-4 cube through its corner at eight normals, in either operand
/// order, by intersect, union and subtract. Wherever the result keeps
/// two vertices on one point key at the corner, it carries a
/// `VertexFusion` row. Red if a pose's pinch is dropped.
#[test]
fn every_pinch_over_the_poses_records_a_row() {
    let prism = ell();
    let mut pinches = 0;
    for m in [
        [1.0, 1.3, 0.7],
        [1.0, 1.3, 0.4],
        [1.2, 1.0, 0.3],
        [2.0, 0.4, 1.0],
        [1.0, 0.2, 0.5],
        [-1.0, -0.9, -1.3],
        [-1.0, -0.6, -1.2],
        [-0.8, -0.7, -1.0],
    ] {
        let cube = cube_beyond(m, 4.0);
        for (order, x, y) in [("prism-cube", &prism, &cube), ("cube-prism", &cube, &prism)] {
            for (op, run) in [
                ("intersect", topo::intersect::<f64> as fn(_, _, _) -> _),
                ("union", topo::union::<f64>),
                ("subtract", topo::subtract::<f64>),
            ] {
                let what = format!("{m:?} {order} {op}");
                let r = match run(x, y, tol()) {
                    Ok(BooleanResult::Body(r)) => r,
                    Ok(BooleanResult::Empty) => continue,
                    other => panic!("{what} builds: {other:?}"),
                };
                let (keys, rows) = at_pinch(&r);
                let mut distinct = keys.clone();
                distinct.sort();
                distinct.dedup();
                if distinct.len() < keys.len() {
                    pinches += 1;
                    assert!(rows > 0, "{what}: the pinch records its row");
                }
            }
        }
    }
    assert!(pinches > 0, "the poses pinch somewhere");
}

/// **An operand's copies on one key stay on one key** (`zip::point_classes`,
/// `zip::share_points`): the two lobes, the halfspace first, at forty
/// normals by every op. Wherever the result keeps both copies at the
/// corner, they share one point key. They carry no vertex-vertex record
/// (the copies were the operand's structure), so the output stage's
/// record rule does not reach them: the seam correspondence ties them,
/// the zips having fused one onto the halfspace's key. Red at 43 of the
/// 120 runs without the seam tie (the copies on two keys).
#[test]
fn an_operands_copies_on_one_key_stay_on_one_key_at_every_normal() {
    let lobes = built(
        "the two lobes",
        topo::intersect(&ell(), &cube_beyond([1.0, 1.3, 0.7], 4.0), tol()),
    )
    .body;
    let n = 40;
    let ga = geom_core::GOLDEN_ANGLE;
    let mut held = 0;
    for i in 0..n {
        let z = 1.0 - 2.0 * (f64::from(i) + 0.5) / f64::from(n);
        let r = (1.0 - z * z).sqrt();
        let m = [
            r * (ga * f64::from(i)).cos(),
            r * (ga * f64::from(i)).sin(),
            z,
        ];
        let half = cube_beyond(m, 8.0);
        for (op, got) in [
            ("intersect", topo::intersect(&half, &lobes, tol())),
            ("union", topo::union(&half, &lobes, tol())),
            ("subtract", topo::subtract(&half, &lobes, tol())),
        ] {
            let Ok(BooleanResult::Body(r)) = got else {
                continue;
            };
            let (keys, _) = at_pinch(&r);
            if keys.len() == 2 {
                held += 1;
                assert_eq!(keys[0], keys[1], "normal {i} half-lobes {op}: {keys:?}");
            }
        }
    }
    assert!(held >= 43, "{held} runs keep both copies");
}
