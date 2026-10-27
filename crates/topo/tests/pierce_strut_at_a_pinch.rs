//! **Two edges pinched along one line pierce a face at one point, and
//! every op on the pair builds, whichever operand holds the face.**
//!
//! The two pierces mint one ring in the pierced face, a strut per run:
//! null edges whose vertices are all the pierce point, on one point key.
//! The first polygon through the point divides the face along an outline
//! the second pierce's strut lies on at every vertex, so no point of
//! the strut says which side it is on. Its own polygon does: the join
//! leaves the strut pending and places it when the polygon's next
//! chord reaches it.
//!
//! The rows, each in all six ops (both ops, both operand orders):
//! - a bare pinch: two bricks touching along the z-axis, which pierces
//!   a brick's top face at the origin;
//! - a wedge in an L's reflex corner, the same way: an interior angle
//!   above π at the pinch;
//! - the pinch standing on a face: its two lower vertices land on the
//!   face at one point, each minting a ring there.
//!
//! In the bare pinch and the reflex corner, `B − pinch` keeps the pinch
//! line inside `B`, where the complement of the pinch's two wedges pairs
//! its sheets across the two solids. ∖ does not build that re-pairing,
//! so the result's two edges there overlap (270° each, or 311.1° and
//! 90°) and the result gate refuses `BooleanError::LineWedges`. The
//! staircase refuses at its own second cut, the same way.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, finished};
use geom_core::Tol;
use topo::{
    AtRestBody, Body, BooleanError, BooleanResult, LineWedgeFault, intersect, subtract, union,
    validate_geometric, validate_pseudomanifold,
};

/// An op on the pair.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Union,
    Subtract,
    Intersect,
}

/// One op on `(x, y)`: its name, its kind, whether `y` goes first, its
/// faces/edges/vertices, its closed-form volume and its tier-3′ verdict.
///
/// A result 3′ refuses is refused by an operand's own contacts along its
/// pinch line, which a kernel op called without `BooleanInputs` records
/// does not carry into its result. A pinch line whose ends the union
/// that made it put on shared point keys reads as structure, and
/// passes ([`keyed_pinch`]): the shared ends are ratified on PR 4793,
/// whose records are the lint's evidence, not the census's.
type Row = (&'static str, Kind, bool, [usize; 3], f64, bool);

/// The pinch `r` a union built, with each of its vertex-vertex records'
/// two vertices on one point key: what lets a later op's result pass at
/// rest without those records (the row [`Row`] names).
fn keyed_pinch(r: BooleanResult<f64>) -> AtRestBody<f64> {
    let BooleanResult::Body(r) = r else {
        panic!("the pinch is two solids");
    };
    for (x, y) in r.contacts.vertex_vertex() {
        let key = |v| r.body.get_vertex(v).unwrap().point;
        assert_eq!(key(x), key(y), "the pinch line's ends share their keys");
    }
    r.body
}

fn t() -> Tol {
    Tol::witness()
}

/// `b − pinch` refuses the pinch's revert (module docs).
fn b_less_pinch_refuses(label: &str, b: &AtRestBody<f64>, pinch: &AtRestBody<f64>) {
    refuses_overlap(&format!("{label}: B − pinch"), subtract(b, pinch, t()));
}

/// Asserts `got` is the result gate's refusal of overlapping wedges on
/// the result's coincident edges.
fn refuses_overlap(what: &str, got: Result<BooleanResult<f64>, BooleanError>) {
    assert!(
        matches!(
            got,
            Err(BooleanError::LineWedges {
                fault: LineWedgeFault::Overlap { .. },
                ..
            })
        ),
        "{what}: {got:?}, not the overlap's refusal"
    );
}

type Point = (i64, i64, i64);

/// A body's geometry, key-free: each face by the points of its loops'
/// vertices, each edge by its ends' points, rounded to a micron, as
/// sorted multisets.
fn shape(body: &Body<f64>) -> (Vec<Vec<Point>>, Vec<[Point; 2]>) {
    let at = |p: geom_core::Point3<f64>| {
        let n = |x: f64| (x * 1e6).round() as i64;
        (n(p.x), n(p.y), n(p.z))
    };
    let mut faces: Vec<Vec<Point>> = body
        .faces()
        .map(|(_, f)| {
            let mut ps: Vec<Point> = core::iter::once(f.outer)
                .chain(f.rings.iter().copied())
                .flat_map(|l| {
                    let first = match body.get_loop(l).unwrap().boundary {
                        topo::LoopBoundary::Cycle { first } => first,
                        topo::LoopBoundary::Empty { .. } => {
                            panic!("a closed body has no empty loop")
                        }
                    };
                    body.loop_cycle(first)
                        .unwrap()
                        .into_iter()
                        .map(|he| at(body.half_edge_start_point(he).unwrap()))
                        .collect::<Vec<_>>()
                })
                .collect();
            ps.sort_unstable();
            ps
        })
        .collect();
    faces.sort_unstable();
    let mut edges: Vec<[Point; 2]> = body
        .edges()
        .map(|(_, e)| {
            let mut ps =
                [e.he_plus, e.he_minus].map(|he| at(body.half_edge_start_point(he).unwrap()));
            ps.sort_unstable();
            ps
        })
        .collect();
    edges.sort_unstable();
    (faces, edges)
}

/// Runs every row on `(x, y)` (`swap` puts `y` first) and asserts the
/// result's counts, volume, tier-3 validity and tier-3′ verdict, and
/// that the two orders of a union, and of an intersection, are one body.
fn every_op(label: &str, x: &AtRestBody<f64>, y: &AtRestBody<f64>, rows: &[Row]) {
    type Shape = (Vec<Vec<Point>>, Vec<[Point; 2]>);
    let mut by_kind: Vec<(Kind, bool, Shape)> = Vec::new();
    for &(name, kind, swap, counts, volume, verdict) in rows {
        let what = format!("{label}: {name}");
        let (a, b) = if swap { (y, x) } else { (x, y) };
        let op = match kind {
            Kind::Union => union,
            Kind::Subtract => subtract,
            Kind::Intersect => intersect,
        };
        let r = match op(a, b, t()) {
            Ok(BooleanResult::Body(r)) => r,
            Ok(BooleanResult::Empty) => panic!("{what}: empty"),
            Err(e) => panic!("{what}: refused: {e:?}"),
        };
        let got = [
            r.body.faces().count(),
            r.body.edges().count(),
            r.body.vertices().count(),
        ];
        assert_eq!(got, counts, "{what}: faces, edges, vertices");
        assert_eq!(
            topo::joinable_vertices(
                &r.body,
                geom_core::Band::linear(geom_core::Tol::witness()).unwrap()
            )
            .unwrap(),
            vec![],
            "{what}: maximal edges"
        );
        assert_eq!(validate_geometric(&r.body, t()), Ok(()), "{what}: tier 3");
        let v = topo::test_support::pinned_volume("the measured body", &r.body, t()).volume;
        assert!(
            (v - volume).abs() < 1e-9,
            "{what}: volume {v}, closed form {volume}"
        );
        assert_eq!(
            validate_pseudomanifold(&r.body, &r.contacts, t()).is_ok(),
            verdict,
            "{what}: tier 3′"
        );
        by_kind.push((kind, swap, shape(&r.body)));
    }
    for kind in [Kind::Union, Kind::Intersect] {
        let both: Vec<&Shape> = by_kind
            .iter()
            .filter(|(k, _, _)| *k == kind)
            .map(|(_, _, s)| s)
            .collect();
        assert_eq!(both.len(), 2, "{label}: {kind:?} runs both ways round");
        assert!(
            both[0] == both[1],
            "{label}: {kind:?} differs by operand order"
        );
    }
}

fn block(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    finished("a brick", brick::<f64>(x, y, z, t()), t())
}

/// **The 3N staircase refuses at its second cut.** The slab
/// `[−1, 4] × [−1, 3] × [0.5, 3]` less the cut `[1, 1.5] × [−2, 1]`
/// builds; less the next cut, `[1.5, 2] × [1, 1.5]`, which touches the
/// first along `x = 1.5, y = 1`, the material there is two 90° wedges,
/// one each side. The join pairs each cut's two faces into an edge, so
/// the result carries the line from `z = 0.5` to `z = 2` as two edges of
/// 270° each, overlapping by 180° (every tier passed it), and the result
/// gate refuses. Its six ops against the plate, `P − X` among them,
/// return with the re-pairing
/// (`work/topo/revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet.md`).
#[test]
fn the_staircase_refuses_at_its_second_cut() {
    let first = match subtract(
        &block((-1.0, 4.0), (-1.0, 3.0), (0.5, 3.0)),
        &block((1.0, 1.5), (-2.0, 1.0), (0.3, 2.3)),
        t(),
    )
    .unwrap()
    {
        BooleanResult::Body(r) => r.body,
        BooleanResult::Empty => panic!("the first cut leaves material"),
    };
    let got = subtract(&first, &block((1.5, 2.0), (1.0, 1.5), (0.27, 2.0)), t());
    if let Err(BooleanError::LineWedges { from, to, .. }) = &got {
        let mut ends = [from.as_str(), to.as_str()];
        ends.sort_unstable();
        assert_eq!(
            ends,
            ["(1.5, 1.0, 0.5)", "(1.5, 1.0, 2.0)"],
            "the staircase's second cut: the line"
        );
    }
    refuses_overlap("the staircase's second cut", got);
}

/// **A bare pinch against a face its line pierces, every op both ways
/// round.**
#[test]
fn a_pinch_through_a_face_builds_in_every_op() {
    let q1 = block((0.0, 1.0), (0.0, 1.0), (0.5, 1.5));
    let q3 = block((-1.0, 0.0), (-1.0, 0.0), (0.5, 1.5));
    let pinch = keyed_pinch(union(&q1, &q3, t()).unwrap());
    let b = block((-0.5, 0.5), (-0.5, 0.5), (0.0, 1.0));
    // B holds a quarter of each brick's lower half.
    let both = 2.0 * 0.25 * 0.5;
    let rows: [Row; 5] = [
        (
            "pinch ∪ B",
            Kind::Union,
            false,
            [19, 48, 31],
            2.0 + 1.0 - both,
            true,
        ),
        (
            "pinch − B",
            Kind::Subtract,
            false,
            [18, 42, 28],
            2.0 - both,
            true,
        ),
        (
            "pinch ∩ B",
            Kind::Intersect,
            false,
            [12, 24, 16],
            both,
            true,
        ),
        (
            "B ∪ pinch",
            Kind::Union,
            true,
            [19, 48, 31],
            2.0 + 1.0 - both,
            true,
        ),
        ("B ∩ pinch", Kind::Intersect, true, [12, 24, 16], both, true),
    ];
    every_op("pinch", &pinch, &b, &rows);
    b_less_pinch_refuses("pinch", &b, &pinch);
}

/// **A wedge in an L's reflex corner, against a face their shared line
/// pierces, every op both ways round.** The L's section of the face
/// turns through 3π/2 at the pinch, so the side a strut's two germs
/// bound there is not the side their bisector points to; the join never
/// reads it.
#[test]
fn a_wedge_in_a_reflex_corner_through_a_face_builds_in_every_op() {
    let ell = [
        (0.0, 0.0),
        (0.0, -1.0),
        (1.0, -1.0),
        (1.0, 1.0),
        (-1.0, 1.0),
        (-1.0, 0.0),
    ];
    let wedge = [(0.0, 0.0), (-0.32, -0.12), (-0.12, -0.32)];
    let l = finished(
        "the L",
        common::prism_z::<f64>(&ell, 0.5, 1.5, t()).body,
        t(),
    );
    let w = finished(
        "the wedge",
        common::prism_z::<f64>(&wedge, 0.5, 1.5, t()).body,
        t(),
    );
    let pinch = keyed_pinch(union(&l, &w, t()).unwrap());
    let b = block((-0.5, 0.5), (-0.5, 0.5), (0.0, 1.0));
    let wedge_area = 0.5 * (0.32 * 0.32 - 0.12 * 0.12);
    let pinch_volume = 3.0 + wedge_area;
    // B holds three quarters of its top's square of the L and all of the
    // wedge, over z ∈ [0.5, 1].
    let both = (0.75 + wedge_area) * 0.5;
    let rows: [Row; 5] = [
        (
            "pinch ∪ B",
            Kind::Union,
            false,
            [18, 45, 29],
            pinch_volume + 1.0 - both,
            true,
        ),
        (
            "pinch − B",
            Kind::Subtract,
            false,
            [18, 42, 28],
            pinch_volume - both,
            true,
        ),
        (
            "pinch ∩ B",
            Kind::Intersect,
            false,
            [13, 27, 18],
            both,
            true,
        ),
        (
            "B ∪ pinch",
            Kind::Union,
            true,
            [18, 45, 29],
            pinch_volume + 1.0 - both,
            true,
        ),
        ("B ∩ pinch", Kind::Intersect, true, [13, 27, 18], both, true),
    ];
    every_op("reflex", &pinch, &b, &rows);
    b_less_pinch_refuses("reflex", &b, &pinch);
}

/// **A pinch standing on a face, every op both ways round.** The two
/// bricks' lower ends meet at one point of the face, a vertex of each
/// on it: two rings at one point, each placed with its own polygon.
/// The solids only touch, so the intersections are empty.
#[test]
fn a_pinch_standing_on_a_face_builds_in_every_op() {
    type DeclaredOp = fn(
        &AtRestBody<f64>,
        &AtRestBody<f64>,
        Tol,
    ) -> Result<BooleanResult<f64>, topo::BooleanError>;
    let q1 = block((0.0, 1.0), (0.0, 1.0), (0.5, 1.5));
    let q3 = block((-1.0, 0.0), (-1.0, 0.0), (0.5, 1.5));
    let pinch = match union(&q1, &q3, t()).unwrap() {
        BooleanResult::Body(r) => r.body,
        BooleanResult::Empty => panic!("the pinch is two bricks"),
    };
    let b = block((-1.0, 1.0), (-1.0, 1.0), (0.0, 0.5));
    // Each op's counts and volume, or `None` where it is empty.
    type Want = Option<([usize; 3], f64)>;
    let rows: [(&str, DeclaredOp, bool, Want); 6] = [
        ("pinch ∪ B", union, false, Some(([13, 30, 19], 4.0))),
        ("pinch − B", subtract, false, Some(([12, 24, 16], 2.0))),
        ("pinch ∩ B", intersect, false, None),
        ("B ∪ pinch", union, true, Some(([13, 30, 19], 4.0))),
        ("B − pinch", subtract, true, Some(([6, 12, 8], 2.0))),
        ("B ∩ pinch", intersect, true, None),
    ];
    for (name, op, swap, want) in rows {
        let (x, y) = if swap { (&b, &pinch) } else { (&pinch, &b) };
        let got = op(x, y, t()).unwrap_or_else(|e| panic!("{name}: refused: {e:?}"));
        match (got, want) {
            (BooleanResult::Empty, None) => {}
            (BooleanResult::Body(r), Some((counts, volume))) => {
                let got = [
                    r.body.faces().count(),
                    r.body.edges().count(),
                    r.body.vertices().count(),
                ];
                assert_eq!(got, counts, "{name}: faces, edges, vertices");
                assert_eq!(
                    topo::joinable_vertices(
                        &r.body,
                        geom_core::Band::linear(geom_core::Tol::witness()).unwrap()
                    )
                    .unwrap(),
                    vec![],
                    "{name}: maximal edges"
                );
                assert_eq!(validate_geometric(&r.body, t()), Ok(()), "{name}: tier 3");
                let v = topo::test_support::pinned_volume("the measured body", &r.body, t()).volume;
                assert!(
                    (v - volume).abs() < 1e-9,
                    "{name}: volume {v}, closed form {volume}"
                );
            }
            (BooleanResult::Empty, Some(_)) => panic!("{name}: empty"),
            (BooleanResult::Body(_), None) => panic!("{name}: the touching solids share material"),
        }
    }
}
