//! **Two edges pinched along one line pierce a face at one point, and
//! every op on the pair builds, whichever operand holds the face.**
//!
//! Each pierce mints a ring in the pierced face, and the ring is a
//! strut: one null edge whose two vertices are both the pierce point.
//! The first polygon through the point divides the face along an outline
//! the second pierce's strut lies on at every vertex, so no point of
//! the strut says which side it is on. Its own polygon does: the join
//! leaves the strut pending and places it when the polygon's next
//! chord reaches it.
//!
//! The rows, each in all six ops (both ops, both operand orders):
//! - the 3N staircase: a slab less three cuts, each consecutive pair of
//!   them touching along a line that pierces a plate's top;
//! - a bare pinch: two bricks touching along the z-axis, which pierces
//!   a brick's top face at the origin;
//! - a wedge in an L's reflex corner, the same way: an interior angle
//!   above π at the pinch;
//! - the pinch standing on a face: its two lower vertices land on the
//!   face at one point, each minting a ring there.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, finished, flush_declarations};
use geom_core::Tol;
use topo::{
    AtRestBody, Body, BooleanDeclarations, BooleanResult, intersect, intersect_with, subtract,
    subtract_with, union, union_with, validate_geometric, validate_pseudomanifold,
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
/// pinch line, which no boolean carries into its result
/// (`work/wire/a-boolean-drops-its-operands-own-contact-records.md`).
type Row = (&'static str, Kind, bool, [usize; 3], f64, bool);

fn t() -> Tol {
    Tol::witness()
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
            topo::joinable_vertices(&r.body),
            vec![],
            "{what}: maximal edges"
        );
        assert_eq!(validate_geometric(&r.body, t()), Ok(()), "{what}: tier 3");
        let v = topo::mass_properties(&r.body, t()).unwrap().volume;
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

/// `base` less `cut`, the two flush where they share a plane.
fn less(base: &AtRestBody<f64>, cut: &AtRestBody<f64>) -> AtRestBody<f64> {
    match subtract_with(base, cut, &flush_declarations(base, cut, t()), t()).unwrap() {
        BooleanResult::Body(r) => r.body,
        BooleanResult::Empty => panic!("the cut leaves material"),
    }
}

/// **The 3N staircase against the plate, every op both ways round.**
#[test]
fn the_staircase_and_the_plate_build_in_every_op() {
    let cuts = [
        ((1.0, 1.5), (-2.0, 1.0), (0.3, 2.3)),
        ((1.5, 2.0), (1.0, 1.5), (0.27, 2.0)),
        ((2.0, 2.5), (1.5, 4.0), (0.31, 1.81)),
    ];
    let x = cuts.iter().fold(
        block((-1.0, 4.0), (-1.0, 3.0), (0.5, 3.0)),
        |x, &(cx, cy, cz)| less(&x, &block(cx, cy, cz)),
    );
    let p = block((0.0, 3.0), (0.0, 2.0), (0.0, 1.0));
    // Each cut's part inside the slab, and inside the plate's z ∈ [0.5, 1].
    let x_volume = 5.0 * 4.0 * 2.5 - 0.5 * 2.0 * 1.8 - 0.25 * 1.5 - 0.5 * 1.5 * 1.31;
    let both = 3.0 * 2.0 * 0.5 - (0.5 * 1.0 + 0.25 + 0.25) * 0.5;
    let p_volume = 6.0;
    let rows: [Row; 6] = [
        (
            "X − P",
            Kind::Subtract,
            false,
            [28, 72, 46],
            x_volume - both,
            false,
        ),
        (
            "X ∪ P",
            Kind::Union,
            false,
            [28, 72, 48],
            x_volume + p_volume - both,
            false,
        ),
        ("X ∩ P", Kind::Intersect, false, [20, 48, 28], both, true),
        (
            "P − X",
            Kind::Subtract,
            true,
            [20, 48, 30],
            p_volume - both,
            false,
        ),
        (
            "P ∪ X",
            Kind::Union,
            true,
            [28, 72, 48],
            x_volume + p_volume - both,
            false,
        ),
        ("P ∩ X", Kind::Intersect, true, [20, 48, 28], both, true),
    ];
    every_op("staircase", &x, &p, &rows);
}

/// **A bare pinch against a face its line pierces, every op both ways
/// round.**
#[test]
fn a_pinch_through_a_face_builds_in_every_op() {
    let q1 = block((0.0, 1.0), (0.0, 1.0), (0.5, 1.5));
    let q3 = block((-1.0, 0.0), (-1.0, 0.0), (0.5, 1.5));
    let pinch = match union_with(&q1, &q3, &BooleanDeclarations::default(), t()).unwrap() {
        BooleanResult::Body(r) => r.body,
        BooleanResult::Empty => panic!("the pinch is two bricks"),
    };
    let b = block((-0.5, 0.5), (-0.5, 0.5), (0.0, 1.0));
    // B holds a quarter of each brick's lower half.
    let both = 2.0 * 0.25 * 0.5;
    let rows: [Row; 6] = [
        (
            "pinch ∪ B",
            Kind::Union,
            false,
            [19, 48, 31],
            2.0 + 1.0 - both,
            false,
        ),
        (
            "pinch − B",
            Kind::Subtract,
            false,
            [18, 42, 28],
            2.0 - both,
            false,
        ),
        (
            "pinch ∩ B",
            Kind::Intersect,
            false,
            [12, 24, 16],
            both,
            false,
        ),
        (
            "B ∪ pinch",
            Kind::Union,
            true,
            [19, 48, 31],
            2.0 + 1.0 - both,
            false,
        ),
        (
            "B − pinch",
            Kind::Subtract,
            true,
            [13, 30, 19],
            1.0 - both,
            false,
        ),
        (
            "B ∩ pinch",
            Kind::Intersect,
            true,
            [12, 24, 16],
            both,
            false,
        ),
    ];
    every_op("pinch", &pinch, &b, &rows);
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
    let pinch = match union_with(&l, &w, &BooleanDeclarations::default(), t()).unwrap() {
        BooleanResult::Body(r) => r.body,
        BooleanResult::Empty => panic!("the pinch is two prisms"),
    };
    let b = block((-0.5, 0.5), (-0.5, 0.5), (0.0, 1.0));
    let wedge_area = 0.5 * (0.32 * 0.32 - 0.12 * 0.12);
    let pinch_volume = 3.0 + wedge_area;
    // B holds three quarters of its top's square of the L and all of the
    // wedge, over z ∈ [0.5, 1].
    let both = (0.75 + wedge_area) * 0.5;
    let rows: [Row; 6] = [
        (
            "pinch ∪ B",
            Kind::Union,
            false,
            [18, 45, 29],
            pinch_volume + 1.0 - both,
            false,
        ),
        (
            "pinch − B",
            Kind::Subtract,
            false,
            [18, 42, 28],
            pinch_volume - both,
            false,
        ),
        (
            "pinch ∩ B",
            Kind::Intersect,
            false,
            [13, 27, 18],
            both,
            false,
        ),
        (
            "B ∪ pinch",
            Kind::Union,
            true,
            [18, 45, 29],
            pinch_volume + 1.0 - both,
            false,
        ),
        (
            "B − pinch",
            Kind::Subtract,
            true,
            [13, 30, 19],
            1.0 - both,
            false,
        ),
        (
            "B ∩ pinch",
            Kind::Intersect,
            true,
            [13, 27, 18],
            both,
            false,
        ),
    ];
    every_op("reflex", &pinch, &b, &rows);
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
        &BooleanDeclarations,
        Tol,
    ) -> Result<BooleanResult<f64>, topo::BooleanError>;
    let q1 = block((0.0, 1.0), (0.0, 1.0), (0.5, 1.5));
    let q3 = block((-1.0, 0.0), (-1.0, 0.0), (0.5, 1.5));
    let pinch = match union_with(&q1, &q3, &BooleanDeclarations::default(), t()).unwrap() {
        BooleanResult::Body(r) => r.body,
        BooleanResult::Empty => panic!("the pinch is two bricks"),
    };
    let b = block((-1.0, 1.0), (-1.0, 1.0), (0.0, 0.5));
    // Each op's counts and volume, or `None` where it is empty.
    type Want = Option<([usize; 3], f64)>;
    let rows: [(&str, DeclaredOp, bool, Want); 6] = [
        ("pinch ∪ B", union_with, false, Some(([13, 30, 19], 4.0))),
        ("pinch − B", subtract_with, false, Some(([12, 24, 16], 2.0))),
        ("pinch ∩ B", intersect_with, false, None),
        ("B ∪ pinch", union_with, true, Some(([13, 30, 19], 4.0))),
        ("B − pinch", subtract_with, true, Some(([6, 12, 8], 2.0))),
        ("B ∩ pinch", intersect_with, true, None),
    ];
    for (name, op, swap, want) in rows {
        let (x, y) = if swap { (&b, &pinch) } else { (&pinch, &b) };
        let got = op(x, y, &flush_declarations(x, y, t()), t())
            .unwrap_or_else(|e| panic!("{name}: refused: {e:?}"));
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
                    topo::joinable_vertices(&r.body),
                    vec![],
                    "{name}: maximal edges"
                );
                assert_eq!(validate_geometric(&r.body, t()), Ok(()), "{name}: tier 3");
                let v = topo::mass_properties(&r.body, t()).unwrap().volume;
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
