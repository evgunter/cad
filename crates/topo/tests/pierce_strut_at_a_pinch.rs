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

use common::{brick, flush_declarations};
use geom_core::Tol;
use topo::{
    Body, BooleanDeclarations, BooleanResult, intersect, intersect_with, subtract, subtract_with,
    union, union_with, validate_geometric, validate_pseudomanifold,
};

type Op = fn(&Body<f64>, &Body<f64>, Tol) -> Result<BooleanResult<f64>, topo::BooleanError>;

/// The six ops on `(x, y)` with each one's name, faces/edges/vertices
/// and closed-form volume, in that order.
type Row = (&'static str, Op, bool, [usize; 3], f64);

fn t() -> Tol {
    Tol::witness()
}

/// Runs every row on `(x, y)` (`swap` puts `y` first) and asserts the
/// result's counts, volume and tier-3 validity, and that the two orders
/// of a symmetric op agree. Returns each result's tier-3′ verdict.
fn every_op(label: &str, x: &Body<f64>, y: &Body<f64>, rows: &[Row]) -> Vec<bool> {
    let mut verdicts = Vec::new();
    for &(name, op, swap, counts, volume) in rows {
        let what = format!("{label}: {name}");
        let (a, b) = if swap { (y, x) } else { (x, y) };
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
        assert_eq!(validate_geometric(&r.body, t()), Ok(()), "{what}: tier 3");
        let v = topo::mass_properties(&r.body, t()).unwrap().volume;
        assert!(
            (v - volume).abs() < 1e-9,
            "{what}: volume {v}, closed form {volume}"
        );
        verdicts.push(validate_pseudomanifold(&r.body, &r.contacts, t()).is_ok());
    }
    verdicts
}

fn block(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    brick::<f64>(x, y, z, t())
}

/// `base` less `cut`, the two flush where they share a plane.
fn less(base: &Body<f64>, cut: &Body<f64>) -> Body<f64> {
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
        ("X − P", subtract, false, [28, 74, 48], x_volume - both),
        (
            "X ∪ P",
            union,
            false,
            [28, 74, 50],
            x_volume + p_volume - both,
        ),
        ("X ∩ P", intersect, false, [20, 48, 28], both),
        ("P − X", subtract, true, [20, 48, 30], p_volume - both),
        (
            "P ∪ X",
            union,
            true,
            [28, 74, 50],
            x_volume + p_volume - both,
        ),
        ("P ∩ X", intersect, true, [20, 48, 28], both),
    ];
    let verdicts = every_op("staircase", &x, &p, &rows);
    assert!(verdicts[2] && verdicts[5], "the intersections pass 3′");
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
        ("pinch ∪ B", union, false, [19, 48, 31], 2.0 + 1.0 - both),
        ("pinch − B", subtract, false, [18, 42, 28], 2.0 - both),
        ("pinch ∩ B", intersect, false, [12, 24, 16], both),
        ("B ∪ pinch", union, true, [19, 48, 31], 2.0 + 1.0 - both),
        ("B − pinch", subtract, true, [13, 30, 19], 1.0 - both),
        ("B ∩ pinch", intersect, true, [12, 24, 16], both),
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
    let l = common::prism_z::<f64>(&ell, 0.5, 1.5, t()).body;
    let w = common::prism_z::<f64>(&wedge, 0.5, 1.5, t()).body;
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
            union,
            false,
            [18, 45, 29],
            pinch_volume + 1.0 - both,
        ),
        (
            "pinch − B",
            subtract,
            false,
            [18, 42, 28],
            pinch_volume - both,
        ),
        ("pinch ∩ B", intersect, false, [13, 27, 18], both),
        (
            "B ∪ pinch",
            union,
            true,
            [18, 45, 29],
            pinch_volume + 1.0 - both,
        ),
        ("B − pinch", subtract, true, [13, 30, 19], 1.0 - both),
        ("B ∩ pinch", intersect, true, [13, 27, 18], both),
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
        &Body<f64>,
        &Body<f64>,
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
        ("pinch ∪ B", union_with, false, Some(([13, 32, 21], 4.0))),
        ("pinch − B", subtract_with, false, Some(([12, 24, 16], 2.0))),
        ("pinch ∩ B", intersect_with, false, None),
        ("B ∪ pinch", union_with, true, Some(([13, 32, 21], 4.0))),
        ("B − pinch", subtract_with, true, Some(([6, 16, 12], 2.0))),
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
