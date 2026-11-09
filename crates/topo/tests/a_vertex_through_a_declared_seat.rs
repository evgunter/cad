//! **A vertex through a declared seat reads the seat's two faces as one
//! wall.** A bolt standing on a plate, one body of two solids whose seat
//! is a recorded face–face contact, is cut and met by blocks through the
//! seat: each face holds its piece's material on its own side, so a
//! vertex of a block piercing both is read once, each face's crossing
//! minted at it, and every operation runs on the whole body, the seat's
//! record carried (`vtxfac::Wall`).
//!
//! - **Built**: the seat cut by a post, a pin, a bead or a cap keeps both
//!   pieces, and the seat's record where both still meet over it; a post,
//!   a bead or a cap met with the seat is two pieces on the seat; a bead
//!   on the bolt alone unites with it and leaves the plate a piece of its
//!   own; a cap over the whole seat unites the pieces into one solid with
//!   no seat left. Each holds tier 3 and 3′, its volume and material the
//!   op's, at rest and turned.
//! - **A bridged union touches itself**: a post, a bead, a ledge or a
//!   slab joining the bolt and the plate leaves them lying on each other
//!   over the rest of the seat inside one solid, `TouchesItself` naming
//!   the record.
//! - **A cut across its tool's seat** refuses `ToolSeatCrossed` naming
//!   the record: the cut would leave a sliver of the cut part between the
//!   seat's two sections
//!   (`work/seal/a-cut-across-its-tools-seat-leaves-a-sliver-between-the-seats-two-sections.md`).
//! - **No record, no wall**: the same body carried in without its record
//!   is two solids touching, and a vertex piercing both faces refuses
//!   `VertexReadTwice` in every op; so does a vertex that only touches
//!   the seat, or pairs on it, with the record.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::{Pose, posed_boxes, poses};
use geom_core::{Band, Point3, Tol};
use std::collections::BTreeMap;
use topo::{
    AtRestBody, BooleanError, BooleanInputs, BooleanOp, BooleanResult, ContactRecords, FaceKey,
    Operand, SectorRead, SolidContainment, SolidKey,
};

fn t() -> Tol {
    Tol::witness()
}

/// The bolt `[-0.2, 0.2]² × [0.2, 1.2]` standing on the plate
/// `[-1, 1]² × [0, 0.2]`, one body of two solids, posed, with its seat
/// recorded: one face–face record, the bolt's foot and the plate's top.
struct Seat {
    body: AtRestBody<f64>,
    records: ContactRecords,
    pose: Pose,
}

/// An operand: the seat, or a named block.
enum Shape<'a> {
    Seat,
    Block(&'static str, &'a AtRestBody<f64>),
}

impl Seat {
    fn at(pose: Pose) -> Self {
        let body = posed_boxes(
            "a bolt on a plate",
            &[
                [(-0.2, 0.2), (-0.2, 0.2), (0.2, 1.2)],
                [(-1.0, 1.0), (-1.0, 1.0), (0.0, 0.2)],
            ],
            &pose,
            t(),
        );
        let groups: BTreeMap<SolidKey, u32> = body.solids().map(|(s, _)| s).zip(0..).collect();
        let recorded = topo::record_at_rest(
            &body,
            &ContactRecords::default(),
            |records| {
                topo::validate_pseudomanifold(&body, records, t())
                    .err()
                    .unwrap_or_default()
            },
            &|s| groups.get(&s).copied(),
            0,
        );
        assert!(recorded.findings.is_empty(), "{:?}", recorded.findings);
        let records = recorded.records;
        assert!(
            records.len() == 1 && records.face_face().count() == 1,
            "the premise, {}: the seat is one face–face record: {records:?}",
            pose.label
        );
        Self {
            body,
            records,
            pose,
        }
    }

    /// The seat's two faces.
    fn faces(&self) -> [FaceKey; 2] {
        let (x, y) = self.records.face_face().next().unwrap();
        [x, y]
    }

    fn block(&self, x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
        posed_boxes("a block", &[[x, y, z]], &self.pose, t())
    }

    /// The prism over the triangle `(0.1, 0.35)`, `(-0.1, 0.35)`,
    /// `(0, 0.2)` in `(y, z)`, from `x = -0.1` to `0.1`: its apex edge
    /// lies in the seat, inside the bolt's foot.
    fn ridge(&self) -> AtRestBody<f64> {
        let mut body = topo::Body::<f64>::new();
        common::prism_ops(
            &mut body,
            &[(0.1, 0.35), (-0.1, 0.35), (0.0, 0.2)],
            (-0.1, 0.1),
            |x, y, z| self.pose.at([z, x, y]),
            common::FaceGeometry::Certified,
            t(),
        );
        common::describe_as_intersections(&mut body, t());
        common::finished("a ridge", body, t())
    }

    /// `x op y`, the seat's records carried in where it is an operand
    /// and `carried`.
    fn run(
        &self,
        op: BooleanOp,
        [x, y]: [&Shape<'_>; 2],
        carried: bool,
    ) -> Result<BooleanResult<f64>, BooleanError> {
        let of = |s: &Shape<'_>| match (s, carried) {
            (Shape::Seat, true) => self.records.carried(),
            _ => Default::default(),
        };

        let inputs = BooleanInputs {
            records_a: of(x),
            records_b: of(y),
        };
        topo::boolean_op_with(
            op,
            self.body_of(x),
            self.body_of(y),
            &inputs,
            topo::SweepStrategy::Realized,
            t(),
        )
    }

    fn body_of<'b>(&'b self, s: &Shape<'b>) -> &'b AtRestBody<f64> {
        match *s {
            Shape::Seat => &self.body,
            Shape::Block(_, b) => b,
        }
    }

    fn label(&self, op: BooleanOp, [x, y]: [&Shape<'_>; 2]) -> String {
        let name = |s: &Shape<'_>| match *s {
            Shape::Seat => "seat",
            Shape::Block(name, _) => name,
        };
        let sign = match op {
            BooleanOp::Union => "∪",
            BooleanOp::Subtract => "−",
            BooleanOp::Intersect => "∩",
        };
        format!("{} {sign} {}, {}", name(x), name(y), self.pose.label)
    }
}

fn volume(b: &AtRestBody<f64>) -> f64 {
    topo::mass_properties(b, t(), topo::test_support::pinned_resolution(t()))
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume
}

fn inside(b: &AtRestBody<f64>, q: Point3<f64>) -> Option<bool> {
    match topo::point_in_solid(b, q, Band::linear(t()).unwrap(), t()).ok()? {
        SolidContainment::In => Some(true),
        SolidContainment::Out => Some(false),
        SolidContainment::OnBoundary => None,
    }
}

/// The blocks the rows read, in the seat's frame.
struct Blocks {
    /// Through the seat inside the bolt's foot.
    post: AtRestBody<f64>,
    /// Through the plate's bottom and the seat, up into the bolt.
    pin: AtRestBody<f64>,
    /// Through the seat across the bolt's side.
    bead: AtRestBody<f64>,
    /// Against the bolt alone, above the seat.
    side: AtRestBody<f64>,
    /// Flush on the bolt's side, sunk into the plate.
    ledge: AtRestBody<f64>,
    /// Through the seat and out past the bolt and the plate, crossing
    /// the bolt's foot by a face alone.
    slab: AtRestBody<f64>,
    /// Over the whole of the bolt's foot, sunk into the plate.
    cap: AtRestBody<f64>,
    /// Over the bolt's foot corner `(0.2, 0.2)`, through the seat.
    over: AtRestBody<f64>,
    /// Beside that corner, an edge through it: the edge's vertex on the
    /// seat pairs with the corner and pierces the plate's top.
    corner: AtRestBody<f64>,
    /// A ridge inside the bolt, its apex edge lying in the seat.
    ridge: AtRestBody<f64>,
}

impl Blocks {
    fn of(seat: &Seat) -> Self {
        Self {
            post: seat.block((-0.1, 0.1), (-0.1, 0.1), (0.1, 0.4)),
            pin: seat.block((-0.1, 0.1), (-0.1, 0.1), (-0.5, 0.5)),
            bead: seat.block((0.15, 0.35), (-0.1, 0.1), (0.1, 0.4)),
            side: seat.block((0.15, 0.35), (-0.1, 0.1), (0.6, 0.8)),
            ledge: seat.block((0.2, 0.5), (-0.5, 0.5), (0.1, 0.4)),
            slab: seat.block((-0.1, 2.0), (-2.0, 2.0), (0.1, 0.4)),
            cap: seat.block((-0.5, 0.5), (-0.5, 0.5), (0.1, 0.4)),
            over: seat.block((0.1, 0.5), (0.1, 0.5), (0.1, 0.4)),
            corner: seat.block((0.2, 0.5), (0.2, 0.5), (0.1, 0.4)),
            ridge: seat.ridge(),
        }
    }
}

/// The poses each row reads: at rest, and turned in general.
fn two_poses() -> [Pose; 2] {
    let mut all = poses().into_iter();
    let rest = all.next().unwrap();
    let turned = all.find(|p| p.label == "turned").unwrap();
    [rest, turned]
}

/// **Each cut and meet of the seat by a block through it builds**, both
/// pieces kept and the seat's record carried where both still meet over
/// it, at rest and turned: tier 3 and 3′ hold, the volume is the op's,
/// and so is the material at a point the result holds and one it does
/// not. Red where a vertex through both faces refuses `VertexReadTwice`,
/// as it did while no reader took the record, or where a result is not
/// the op's.
#[test]
fn a_cut_or_meet_through_a_seat_builds_on_the_whole_body() {
    for pose in two_poses() {
        let seat = Seat::at(pose);
        let b = Blocks::of(&seat);
        let s = Shape::Seat;
        let post = Shape::Block("post", &b.post);
        let pin = Shape::Block("pin", &b.pin);
        let bead = Shape::Block("bead", &b.bead);
        let cap = Shape::Block("cap", &b.cap);
        let over = Shape::Block("over", &b.over);
        // The op, its operands, the result's solids and volume, whether
        // the seat's record is carried, and a point the result holds and
        // one it does not, in the seat's frame.
        type Row<'a> = (
            BooleanOp,
            [&'a Shape<'a>; 2],
            usize,
            f64,
            bool,
            [[f64; 3]; 2],
        );
        let rows: [Row<'_>; 12] = [
            (
                BooleanOp::Subtract,
                [&s, &post],
                2,
                0.948,
                true,
                [[0.5, 0.5, 0.1], [0.0, 0.0, 0.15]],
            ),
            (
                BooleanOp::Subtract,
                [&s, &pin],
                2,
                0.94,
                true,
                [[0.0, 0.0, 0.6], [0.0, 0.0, 0.05]],
            ),
            (
                BooleanOp::Subtract,
                [&s, &bead],
                2,
                0.954,
                true,
                [[0.1, 0.0, 0.3], [0.25, 0.0, 0.15]],
            ),
            (
                BooleanOp::Subtract,
                [&s, &cap],
                2,
                0.828,
                false,
                [[0.0, 0.0, 0.6], [0.0, 0.0, 0.3]],
            ),
            (
                BooleanOp::Intersect,
                [&post, &s],
                2,
                0.012,
                true,
                [[0.0, 0.0, 0.3], [0.0, 0.0, 0.05]],
            ),
            (
                BooleanOp::Intersect,
                [&s, &post],
                2,
                0.012,
                true,
                [[0.0, 0.0, 0.15], [0.0, 0.0, 0.45]],
            ),
            (
                BooleanOp::Intersect,
                [&bead, &s],
                2,
                0.006,
                true,
                [[0.25, 0.0, 0.15], [0.3, 0.0, 0.3]],
            ),
            (
                BooleanOp::Intersect,
                [&cap, &s],
                2,
                0.132,
                true,
                [[0.0, 0.0, 0.3], [0.4, 0.0, 0.3]],
            ),
            (
                BooleanOp::Union,
                [&s, &cap],
                1,
                1.128,
                false,
                [[0.4, 0.0, 0.3], [0.6, 0.0, 0.6]],
            ),
            (
                BooleanOp::Subtract,
                [&s, &over],
                2,
                0.942,
                true,
                [[0.0, 0.0, 0.3], [0.15, 0.15, 0.3]],
            ),
            (
                BooleanOp::Intersect,
                [&over, &s],
                2,
                0.018,
                true,
                [[0.15, 0.15, 0.3], [0.3, 0.3, 0.3]],
            ),
            (
                BooleanOp::Intersect,
                [&s, &over],
                2,
                0.018,
                true,
                [[0.3, 0.3, 0.15], [0.3, 0.3, 0.3]],
            ),
        ];
        for (op, operands, solids, want, carried, [held, not]) in rows {
            let what = seat.label(op, operands);
            let r = match seat.run(op, operands, true) {
                Ok(BooleanResult::Body(r)) => r,
                other => panic!("{what}: builds, got {:?}", other.map(|_| ())),
            };
            assert_eq!(r.body.solids().count(), solids, "{what}: its solids");
            assert_eq!(
                topo::validate_geometric(&r.body, t()),
                Ok(()),
                "{what}: tier 3"
            );
            assert_eq!(
                topo::validate_pseudomanifold(&r.body, &r.contacts, t()),
                Ok(()),
                "{what}: tier 3′ under its records"
            );
            let v = volume(&r.body);
            assert!((v - want).abs() < 1e-9, "{what}: volume {v}, want {want}");
            assert_eq!(
                r.contacts.face_face().count(),
                usize::from(carried),
                "{what}: the seat's record {}: {:?}",
                if carried {
                    "carried"
                } else {
                    "gone with a face"
                },
                r.contacts
            );
            assert_eq!(
                inside(&r.body, seat.pose.at(held)),
                Some(true),
                "{what}: holds {held:?}"
            );
            assert_eq!(
                inside(&r.body, seat.pose.at(not)),
                Some(false),
                "{what}: does not hold {not:?}"
            );
        }
    }
}

/// **A bead on one piece unites with it; a block joining both pieces
/// touches itself.** A bead against the bolt alone leaves the plate a
/// piece of its own and the seat a record between them. A post, a bead,
/// a ledge or a slab joining the bolt and the plate leaves the rest of
/// the seat inside one solid, its two faces on each other, which refuses
/// `TouchesItself` naming the seat's record and faces, in either operand
/// order. Red where a bridge builds or refuses anything else.
#[test]
fn a_union_bridging_a_seat_touches_itself_and_one_on_a_piece_builds() {
    for pose in two_poses() {
        let seat = Seat::at(pose);
        let b = Blocks::of(&seat);
        let s = Shape::Seat;
        let side = Shape::Block("side", &b.side);
        let what = seat.label(BooleanOp::Union, [&s, &side]);
        match seat.run(BooleanOp::Union, [&s, &side], true) {
            Ok(BooleanResult::Body(r)) => {
                assert_eq!(
                    r.body.solids().count(),
                    2,
                    "{what}: bolt and bead, and plate"
                );
                assert_eq!(
                    r.contacts.face_face().count(),
                    1,
                    "{what}: {:?}",
                    r.contacts
                );
                let v = volume(&r.body);
                assert!((v - 0.966).abs() < 1e-9, "{what}: volume {v}");
            }
            other => panic!("{what}: builds, got {:?}", other.map(|_| ())),
        }
        for bridge in [
            Shape::Block("post", &b.post),
            Shape::Block("bead", &b.bead),
            Shape::Block("ledge", &b.ledge),
            Shape::Block("slab", &b.slab),
            Shape::Block("over", &b.over),
        ] {
            for (operands, operand) in [([&s, &bridge], Operand::A), ([&bridge, &s], Operand::B)] {
                let what = seat.label(BooleanOp::Union, operands);
                match seat.run(BooleanOp::Union, operands, true) {
                    Err(BooleanError::TouchesItself {
                        operand: o,
                        record,
                        faces,
                    }) => {
                        assert_eq!(o, operand, "{what}: the seat's operand");
                        assert_eq!(record, 0, "{what}: the seat's record");
                        assert_eq!(faces, seat.faces(), "{what}: the seat's faces");
                    }
                    other => panic!("{what}: refuses TouchesItself, got {:?}", other.map(|_| ())),
                }
            }
        }
    }
}

/// **A cut across its tool's seat refuses typed**: a post, a pin, a
/// bead, a slab or a cap cut by the seat, its material across the seat,
/// refuses `ToolSeatCrossed` naming the seat's record and faces, never a
/// sliver of the cut part between the seat's two sections (a lamina at
/// the gate, or an edge the finish leaves in a face). Red where the cut
/// builds or refuses anything else.
#[test]
fn a_cut_across_its_tools_seat_refuses_typed() {
    for pose in two_poses() {
        let seat = Seat::at(pose);
        let b = Blocks::of(&seat);
        let s = Shape::Seat;
        for cut in [
            Shape::Block("post", &b.post),
            Shape::Block("pin", &b.pin),
            Shape::Block("bead", &b.bead),
            Shape::Block("slab", &b.slab),
            Shape::Block("cap", &b.cap),
            Shape::Block("over", &b.over),
            Shape::Block("ridge", &b.ridge),
        ] {
            let what = seat.label(BooleanOp::Subtract, [&cut, &s]);
            match seat.run(BooleanOp::Subtract, [&cut, &s], true) {
                Err(BooleanError::ToolSeatCrossed { record, faces }) => {
                    assert_eq!(record, 0, "{what}: the seat's record");
                    assert_eq!(faces, seat.faces(), "{what}: the seat's faces");
                }
                other => panic!(
                    "{what}: refuses ToolSeatCrossed, got {:?}",
                    other.map(|_| ())
                ),
            }
        }
    }
}

/// **Two pieces touching with no record are no wall**: the seat carried
/// in without its record, a vertex of the post piercing both faces
/// refuses `VertexReadTwice`, its two reads the two faces, in every op
/// and both operand orders. Red where a reader takes two faces as a wall
/// that no record declares.
#[test]
fn a_vertex_through_two_faces_no_record_declares_reads_twice() {
    let seat = Seat::at(Pose::rest());
    let b = Blocks::of(&seat);
    let s = Shape::Seat;
    let post = Shape::Block("post", &b.post);
    for op in [BooleanOp::Union, BooleanOp::Subtract, BooleanOp::Intersect] {
        for (operands, operand) in [([&post, &s], Operand::A), ([&s, &post], Operand::B)] {
            let what = seat.label(op, operands);
            match seat.run(op, operands, false) {
                Err(BooleanError::VertexReadTwice {
                    operand: o,
                    reads: [SectorRead::Pierce(x), SectorRead::Pierce(y)],
                    ..
                }) => {
                    assert_eq!(o, operand, "{what}: the post's vertex");
                    let mut read = [x, y];
                    read.sort();
                    let mut faces = seat.faces();
                    faces.sort();
                    assert_eq!(read, faces, "{what}: it pierces the seat's two faces");
                }
                other => panic!(
                    "{what}: refuses VertexReadTwice, got {:?}",
                    other.map(|_| ())
                ),
            }
        }
    }
}

/// **A vertex that only touches the seat, or pairs on it, is still read
/// twice**, the seat's record carried in: a ridge whose apex edge lies
/// in the seat touches both faces at its ends, and its edge lies in the
/// wall, which no line reader reads as one; a block beside the bolt's
/// foot corner, its edge through it, pierces the plate's top and pairs
/// with the corner. Each refuses `VertexReadTwice` in every op but the
/// ridge cut by the seat (the cut's tool rule), in both operand orders.
/// Red where either is read as a wall's crossing: admitted, the ridge's
/// cut built a body whose records tier 3′ refuses.
#[test]
fn a_vertex_touching_a_seat_or_pairing_on_it_reads_twice() {
    let seat = Seat::at(Pose::rest());
    let b = Blocks::of(&seat);
    let s = Shape::Seat;
    let ridge = Shape::Block("ridge", &b.ridge);
    let corner = Shape::Block("corner", &b.corner);
    for (x, pair) in [(&ridge, false), (&corner, true)] {
        for op in [BooleanOp::Union, BooleanOp::Subtract, BooleanOp::Intersect] {
            for (operands, cut_by_seat) in [([&s, x], false), ([x, &s], op == BooleanOp::Subtract)]
            {
                if cut_by_seat && !pair {
                    continue;
                }
                let what = seat.label(op, operands);
                match seat.run(op, operands, true) {
                    Err(BooleanError::VertexReadTwice {
                        reads: [SectorRead::Pierce(_), next],
                        ..
                    }) => assert_eq!(
                        matches!(next, SectorRead::Pair(_)),
                        pair,
                        "{what}: its second read, got {next:?}"
                    ),
                    other => panic!(
                        "{what}: refuses VertexReadTwice, got {:?}",
                        other.map(|_| ())
                    ),
                }
            }
        }
    }
}
