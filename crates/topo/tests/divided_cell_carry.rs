//! **A divided cell's record lands on the piece that holds the touch**
//! (D1): a surgery verb that divides a recorded cell keeps the cell's
//! key on one piece, and the record follows its touch onto the piece
//! (or the pairs of pieces) whose interiors still meet the other cell,
//! not onto the piece that kept the key.
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common;
use geom_core::{Band, Point3, Tol, Vec3};
use topo::{
    AtRestBody, Backing, Body, BooleanBody, BooleanResult, Cell, ContactRecords, FaceKey, Surface,
    union, validate_pseudomanifold,
};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

fn box_of(x: (f64, f64), y: (f64, f64), z: (f64, f64), what: &str) -> AtRestBody<f64> {
    common::finished(what, common::brick(x, y, z, tol()), tol())
}

fn union_of(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> BooleanBody<f64> {
    match union(a, b, tol()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("the union builds: {other:?}"),
    }
}

/// A unit cube turned off every axis, balanced on its corner at
/// `(2, 2, 1)`.
fn tilted_cube() -> AtRestBody<f64> {
    let (e1, e2, e3) = (
        Vec3::new(0.9, 0.1, 0.3),
        Vec3::new(-0.2, 0.8, 0.45),
        Vec3::new(-0.3, -0.4, 0.85),
    );
    let cube = common::mapped_cube(
        |x, y, z| {
            Point3::new(
                2.0 + x * e1.x + y * e2.x + z * e3.x,
                2.0 + x * e1.y + y * e2.y + z * e3.y,
                1.0 + x * e1.z + y * e2.z + z * e3.z,
            )
        },
        tol(),
    );
    common::finished("the tilted cube", cube, tol())
}

/// The plane face of `body` on z = `z`.
fn face_at_z(body: &Body<f64>, z: f64) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(Surface::Plane { origin, normal, .. })
                    if normal.z.abs() == 1.0 && origin.z == z
            )
        })
        .map(|(k, _)| k)
        .expect("a face on z")
}

/// Each of `carried` cites the operand's record it is.
fn cites_its_record(what: &str, carried: &ContactRecords, record: u32) {
    for row in carried.rows() {
        assert_eq!(
            row.cites.iter().collect::<Vec<_>>(),
            [Backing::Carried { input: 0, record }],
            "{what}: the record cites the operand's"
        );
    }
}

/// Splits `base` by the plane through `origin` across `normal`, carries
/// its records, and asserts each side passes at rest with what it got.
fn split_passes(
    what: &str,
    base: &BooleanBody<f64>,
    origin: Point3<f64>,
    normal: Vec3<f64>,
) -> topo::SplitContacts {
    let plane = common::split_plane(origin, normal, tol());
    let split = topo::split(&base.body, &plane, tol()).expect("the body splits");
    let sides = split
        .carry_contacts(&base.contacts, band(), Some(topo::RegionLane::certified()))
        .unwrap_or_else(|refusal| panic!("{what}: the records land: {refusal:?}"));
    for (side, body, records) in [
        ("above", split.above.body(), &sides.above),
        ("below", split.below.body(), &sides.below),
    ] {
        assert_eq!(
            validate_pseudomanifold(body.expect("material"), records, tol()),
            Ok(()),
            "{what} {side}: the carried records pass at rest: {records:?}"
        );
    }
    sides
}

/// **A cut across the face a corner rests on lands the record on the
/// fragment holding the corner.** The tilted cube's corner rests on the
/// slab's top at (2, 2, 1); the plane x = 1 cuts the slab alone, and the
/// top's key stays on the fragment x < 1, below the cut. The record
/// lands above, on the fragment x > 1, and both sides pass at rest. Red
/// on the carry that reads the face by its key: the corner is above and
/// the key below, so it refuses the record `Parted`.
#[test]
fn a_cut_across_a_rest_face_lands_the_record_on_the_fragment_holding_the_corner() {
    let base = union_of(
        &tilted_cube(),
        &box_of((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), "slab"),
    );
    assert_eq!(
        base.contacts.vertex_face().count(),
        1,
        "{:?}",
        base.contacts
    );
    let (_, top) = base.contacts.vertex_face().next().unwrap();
    let sides = split_passes(
        "x = 1",
        &base,
        Point3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
    );
    assert!(sides.below.is_empty(), "{:?}", sides.below);
    let landed: Vec<_> = sides.above.vertex_face().collect();
    assert!(
        matches!(landed[..], [(_, face)] if face != top),
        "the record lands on a fragment other than the key's: {landed:?}"
    );
    cites_its_record("x = 1", &sides.above, 0);
}

/// **A cut across an edge a corner rests on lands the record on the
/// piece holding the corner.** In `a ∪ c` (bricks touching along an
/// edge), `c`'s corner (0.5, 0, 0) rests on `a`'s rim x ∈ (0, 1), and
/// `a`'s corner (1, 0, 0) on `c`'s rim x ∈ (0.5, 1.5). The plane x = ¼
/// cuts `a`'s rim short of `c`'s corner and x = 1¼ cuts `c`'s past
/// `a`'s: each corner is on the piece that did not keep its edge's key.
/// Both records land on one side, on the corner's piece, and both sides
/// pass at rest. Red on the carry that reads the edge by its key: the
/// cut parts each corner from the key, and the record is refused
/// `Parted`.
#[test]
fn a_cut_across_a_rest_edge_lands_the_record_on_the_piece_holding_the_corner() {
    let a = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), "a");
    let c = box_of((0.5, 1.5), (-1.0, 0.0), (-1.0, 0.0), "c");
    let base = union_of(&a, &c);
    assert_eq!(
        base.contacts.vertex_edge().count(),
        2,
        "{:?}",
        base.contacts
    );
    let before: Vec<_> = base.contacts.vertex_edge().collect();
    for (what, x, moved) in [("x = 1/4", 0.25, 1), ("x = 5/4", 1.25, 0)] {
        let sides = split_passes(
            what,
            &base,
            Point3::new(x, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        );
        let landed: Vec<_> = sides
            .above
            .vertex_edge()
            .chain(sides.below.vertex_edge())
            .collect();
        assert_eq!(landed.len(), 2, "{what}: {sides:?}");
        assert_eq!(
            landed.iter().filter(|r| before.contains(r)).count(),
            1,
            "{what}: one record moves off its key: {landed:?}"
        );
        assert!(
            !before.contains(&landed[moved]) && landed[moved].0 == before[moved].0,
            "{what}: record {moved} keeps its corner and leaves its key: {landed:?}"
        );
    }
}

/// **A cut across one of two crossing edges lands their record on the
/// piece that still crosses.** The cube's top-front edge and a turned
/// box's edge cross at (0.5, 0, 1) and nowhere else, one edge–edge
/// record. The planes x = 0.1 and x = 0.9 each cut the cube's edge clear
/// of the box, on either side of the crossing: once the key's piece
/// still crosses and once the other piece does. The record lands on the
/// crossing piece each time, and both sides pass at rest. Red on the
/// carry that reads the edge by its key, at whichever cut parts the
/// crossing from the key.
#[test]
fn a_cut_across_a_crossing_edge_lands_the_record_on_the_piece_that_still_crosses() {
    let cube = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), "cube");
    let r = std::f64::consts::FRAC_1_SQRT_2;
    let (u, v, w) = ([-r, -0.5, 0.5], [r, -0.5, 0.5], [0.0, r, r]);
    let turned = common::mapped_cube::<f64>(
        |x, y, z| {
            let at = |k: usize| 0.5 * (x * u[k] + y * v[k] + (z - 0.5) * w[k]);
            Point3::new(0.5 + at(0), at(1), 1.0 + at(2))
        },
        tol(),
    );
    let base = union_of(&cube, &common::finished("box", turned, tol()));
    assert_eq!(base.contacts.edge_edge().count(), 1, "{:?}", base.contacts);
    assert_eq!(base.contacts.len(), 1, "{:?}", base.contacts);
    let [before] = base.contacts.edge_edge().collect::<Vec<_>>()[..] else {
        unreachable!()
    };
    let mut off_key = 0;
    for (what, x) in [("x = 0.1", 0.1), ("x = 0.9", 0.9)] {
        let sides = split_passes(
            what,
            &base,
            Point3::new(x, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        );
        let landed: Vec<_> = sides
            .above
            .edge_edge()
            .chain(sides.below.edge_edge())
            .collect();
        assert_eq!(
            landed.len(),
            1,
            "{what}: one side holds the crossing: {sides:?}"
        );
        off_key += usize::from(landed[0] != before);
        cites_its_record(what, &sides.above, 0);
        cites_its_record(what, &sides.below, 0);
    }
    assert_eq!(off_key, 1, "one cut parts the crossing from the key");
}

/// **A rim that opens around a hole lands a resting corner's record on
/// the hole's rim.** The slab's top has a through hole whose wall runs
/// x = 2.05, 0.05 from the tilted cube's resting corner. Opening the top
/// as a 0.1-thick rim divides it: the outer rim keeps its key, and the
/// band around the hole is a face of its own
/// (`RimNaming::holes`), which holds the corner. The record lands
/// there, and the opened body passes at rest. Red on the carry that
/// reads the top by its key: the corner is not on the outer rim, which
/// the census reads stale.
#[test]
fn a_rim_around_a_hole_lands_a_resting_corners_record_on_the_holes_rim() {
    let slab = box_of((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), "slab");
    let hole = box_of((2.05, 3.0), (1.0, 3.0), (-1.0, 2.0), "hole");
    let holed = match topo::subtract(&slab, &hole, tol()) {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("the hole cuts: {other:?}"),
    };
    let base = union_of(&tilted_cube(), &holed);
    assert_eq!(base.contacts.len(), 1, "{:?}", base.contacts);
    let top = face_at_z(&base.body, 1.0);
    let opened = topo::shell_open(&base.body, 0.1, &[top], tol()).expect("the top opens");
    let [rim] = &opened.naming.rims[..] else {
        panic!("one rim: {:?}", opened.naming.rims)
    };
    let [hole_rim] = &rim.holes[..] else {
        panic!("one hole rim: {:?}", rim.holes)
    };
    let carried = opened
        .carry_contacts(&base.contacts, band(), Some(topo::RegionLane::certified()))
        .expect("the record lands");
    let (corner, _) = base.contacts.vertex_face().next().unwrap();
    assert_eq!(
        carried.cell_pairs().collect::<Vec<_>>(),
        [(Cell::Vertex(corner), Cell::Face(hole_rim.face))],
        "the record lands on the hole's rim"
    );
    cites_its_record("shell_open", &carried, 0);
    assert_eq!(
        validate_pseudomanifold(&opened.body, &carried, tol()),
        Ok(()),
        "the carried record passes at rest"
    );
}

/// **A side holding two pieces of the divided face lands the record on
/// the one that holds the corner.** A U-shaped slab (a notch x ∈ (1, 3)
/// over y > 2) has one top face; the plane y = 3 cuts both arms, so the
/// side above holds two fragments of the top, the two arms' tips, and
/// the tilted cube's corner rests on the left tip at (0.5, 3.5, 1). The
/// record lands on that tip alone, and both sides pass at rest. Red on a
/// carry that lands on every piece a side holds: the right tip does not
/// hold the corner.
#[test]
fn a_side_holding_two_fragments_lands_the_record_on_the_one_holding_the_corner() {
    let slab = box_of((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), "slab");
    let notch = box_of((1.0, 3.0), (2.0, 5.0), (-1.0, 2.0), "notch");
    let u = match topo::subtract(&slab, &notch, tol()) {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("the notch cuts: {other:?}"),
    };
    let shift = geom_core::Affine3::translation(Vec3::new(-1.5, 1.5, 0.0));
    let cube = common::finished(
        "the shifted tilted cube",
        topo::transform_rigid(
            &tilted_cube(),
            &geom_core::test_support::rigid(&shift, tol()),
            tol(),
        )
        .expect("a rigid move"),
        tol(),
    );
    let base = union_of(&cube, &u);
    assert_eq!(
        base.contacts.vertex_face().count(),
        1,
        "{:?}",
        base.contacts
    );
    let plane = common::split_plane(Point3::new(0.0, 3.0, 0.0), Vec3::new(0.0, 1.0, 0.0), tol());
    let split = topo::split(&base.body, &plane, tol()).expect("the arms split");
    let (top_above, side) = [(split.above.body(), "above"), (split.below.body(), "below")]
        .into_iter()
        .find_map(|(body, side)| {
            let body = body?;
            let tops = body
                .faces()
                .filter(|(_, f)| {
                    matches!(
                        body.get_surface(f.surface),
                        Some(Surface::Plane { origin, normal, .. })
                            if normal.z.abs() == 1.0 && origin.z == 1.0
                    )
                })
                .count();
            (tops == 2).then_some((tops, side))
        })
        .expect("one side holds two fragments of the top");
    assert_eq!(top_above, 2);
    let sides = split_passes(
        "y = 3",
        &base,
        Point3::new(0.0, 3.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    let holding = if side == "above" {
        &sides.above
    } else {
        &sides.below
    };
    assert_eq!(
        holding.vertex_face().count(),
        1,
        "{side}: the record lands on one fragment: {holding:?}"
    );
    assert_eq!(sides.above.len() + sides.below.len(), 1, "{sides:?}");
    cites_its_record("y = 3", holding, 0);
}
