//! **A curved keel resting on the floor builds its slit form.** A 270°
//! partial revolve about `y` of the slab `0.5..3 × 1..2`, less a 150°
//! partial revolve of a V whose tip lies on its floor `y = 1`, turned
//! about `y`. The keel is a circular arc of radius 2 lying in the floor,
//! and the material about it is two wedges, floor | wall a and wall b |
//! floor (`crates/topo/README.md`, tier 3), so the floor carries it as
//! two coincident arcs: turned 60°, the V lies inside the slab's 270° and
//! the arcs are a zero-area ring of the floor; turned −20°, the keel
//! leaves through the slab's end face and they are a slit of its
//! outline. The split's twin is `a_curved_keel_in_the_plane`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{BooleanResult, subtract, transform_rigid};

fn tol() -> Tol {
    Tol::witness()
}

/// **Each turn builds at its closed form and passes tier 3′ on its
/// records**: tiers 1, 3 and 3′, and one edge–edge record of the two
/// arcs, citing the parting's row of the arc lying in the floor
/// (`DecisionSite::ArcOnFace`). The volume is the slab's,
/// `0.75 π (3² − 0.5²)`, less the V's triangle in the slab (area 0.2,
/// centroid at radius 2) over the turn it holds inside the slab's: all
/// 150° turned 60°, 130° turned −20°. *Breaks if* the parting ships the
/// arcs unrecorded (3′ refuses an `EdgeEdgeOverlap`), or parts them on a
/// locus that is not the keel's (the volume moves, or tier 3 refuses).
#[test]
fn a_curved_keel_on_the_floor_builds_its_slit_form() {
    let loop_of = |pts: &[(f64, f64)]| pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect();
    let slab = finished(
        "the slab",
        revolved_about_y(
            loop_of(&[(0.5, 1.0), (3.0, 1.0), (3.0, 2.0), (0.5, 2.0)]),
            Revolution::Partial(1.5 * PI),
            tol(),
        ),
        tol(),
    );
    for (turn, inside) in [(60.0, 150.0f64), (-20.0, 130.0)] {
        let what = format!("the V turned {turn}°");
        let v = revolved_about_y(
            loop_of(&[(1.7, 2.5), (2.0, 1.0), (2.3, 2.5)]),
            Revolution::Partial(150f64.to_radians()),
            tol(),
        );
        let turned =
            Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), f64::to_radians(turn));
        let v = transform_rigid(&v, &geom_core::test_support::rigid(&turned, tol())).unwrap();
        let r = match subtract(&slab, &finished("the V", v, tol()), tol()) {
            Ok(BooleanResult::Body(r)) => r,
            other => panic!("{what}: {:?}, not a body", other.map(drop)),
        };
        assert_eq!(topo::validate(&r.body), Ok(()), "{what}: tier 1");
        assert_eq!(
            topo::validate_geometric(&r.body, tol()),
            Ok(()),
            "{what}: tier 3"
        );
        assert_eq!(
            topo::validate_pseudomanifold(&r.body, r.contacts(), tol()),
            Ok(()),
            "{what}: tier 3′ on {:?}",
            r.contacts()
        );
        let pairs: Vec<_> = r
            .contacts()
            .rows()
            .iter()
            .filter(|c| matches!(c.kind(), topo::ContactKind::EdgeEdge(..)))
            .collect();
        let [pair] = pairs.as_slice() else {
            panic!("{what}: the arcs' record, {:?}", r.contacts());
        };
        // The record cites the parting's reading of the arc in the floor,
        // which covers its interior, not its ends' placements.
        let rows: Vec<_> = pair
            .cites
            .iter()
            .filter_map(|b| match b {
                topo::Backing::Decided(k) => Some(r.coincidences[k as usize]),
                topo::Backing::Carried { .. } => None,
            })
            .filter(|row| row.site == topo::DecisionSite::ArcOnFace)
            .collect();
        let [row] = rows.as_slice() else {
            panic!("{what}: {pair:?} cites no one row of the arc on the floor");
        };
        // The row names the arc and the floor as the result holds them,
        // after the parting: the edge bounds the face.
        let [
            topo::RowCell::Result {
                cell: topo::Cell::Edge(arc),
            },
            topo::RowCell::Result {
                cell: topo::Cell::Face(floor),
            },
        ] = row.cells
        else {
            panic!("{what}: {row:?} is not the result's arc and face");
        };
        let edge = r
            .body
            .get_edge(arc)
            .unwrap_or_else(|| panic!("{what}: {arc:?} is dead"));
        assert!(
            [edge.he_plus, edge.he_minus]
                .into_iter()
                .any(|he| r.body.face_of_half_edge(he) == Some(floor)),
            "{what}: the row's arc {arc:?} bounds no face {floor:?}"
        );
        assert!(
            [edge_of(pair, 0), edge_of(pair, 1)].contains(&arc),
            "{what}: the row's arc is one of the record's two edges"
        );
        let volume = 0.75 * PI * (9.0 - 0.25) - 0.2 * 2.0 * inside.to_radians();
        let got = topo::test_support::pinned_volume(
            &what,
            &finished(&what, r.body.clone(), tol()),
            tol(),
        )
        .volume;
        assert!(
            (got - volume).abs() < 1e-9 * volume,
            "{what}: volume {got}, closed form {volume}"
        );
    }
}

/// Edge `i` of edge–edge record `pair`.
fn edge_of(pair: &topo::Cited<topo::Contact>, i: usize) -> topo::EdgeKey {
    match pair.cells()[i] {
        topo::Cell::Edge(e) => e,
        other => panic!("{pair:?}: cell {i} is {other:?}, not an edge"),
    }
}

/// **A keel across the whole floor divides it, and the arcs' row names
/// the piece the arc bounds.** A 90° slab less the V turned −30°, whose
/// 150° spans past both of the slab's end faces: the keel runs across the
/// floor from end face to end face, and the parting divides the floor
/// into the inner and outer piece, each in a solid of its own touching
/// the other along the keel. The arcs' one edge–edge record cites one
/// `ArcOnFace` row, whose edge is one of the record's two and whose face,
/// live in the result, is the piece that edge bounds. *Breaks if* the row
/// is minted before the parting, on the floor's key, which the division
/// leaves on the other piece. Tier 3′ is not read: two curved solids in
/// reach are undecidable to the census
/// (`work/reachhold/boolean-door-runs-the-census-over-its-result.md`).
#[test]
fn a_keel_across_the_floor_names_the_piece_its_arc_bounds() {
    let loop_of = |pts: &[(f64, f64)]| pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect();
    let slab = finished(
        "the slab",
        revolved_about_y(
            loop_of(&[(0.5, 1.0), (3.0, 1.0), (3.0, 2.0), (0.5, 2.0)]),
            Revolution::Partial(90f64.to_radians()),
            tol(),
        ),
        tol(),
    );
    let v = revolved_about_y(
        loop_of(&[(1.7, 2.5), (2.0, 1.0), (2.3, 2.5)]),
        Revolution::Partial(150f64.to_radians()),
        tol(),
    );
    let turned =
        Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), (-30f64).to_radians());
    let v = transform_rigid(&v, &geom_core::test_support::rigid(&turned, tol())).unwrap();
    let r = match subtract(&slab, &finished("the V", v, tol()), tol()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("{:?}, not a body", other.map(drop)),
    };
    assert_eq!(r.body.solids().count(), 2, "the floor's two pieces' solids");
    assert_eq!(topo::validate(&r.body), Ok(()), "tier 1");
    assert_eq!(topo::validate_geometric(&r.body, tol()), Ok(()), "tier 3");
    let pairs: Vec<_> = r
        .contacts()
        .rows()
        .iter()
        .filter(|c| matches!(c.kind(), topo::ContactKind::EdgeEdge(..)))
        .collect();
    let [pair] = pairs.as_slice() else {
        panic!("the arcs' record, {:?}", r.contacts());
    };
    let rows: Vec<_> = pair
        .cites
        .iter()
        .filter_map(|b| match b {
            topo::Backing::Decided(k) => Some(r.coincidences[k as usize]),
            topo::Backing::Carried { .. } => None,
        })
        .filter(|row| row.site == topo::DecisionSite::ArcOnFace)
        .collect();
    let [row] = rows.as_slice() else {
        panic!("{pair:?} cites no one row of the arc on the floor");
    };
    let [
        topo::RowCell::Result {
            cell: topo::Cell::Edge(arc),
        },
        topo::RowCell::Result {
            cell: topo::Cell::Face(floor),
        },
    ] = row.cells
    else {
        panic!("{row:?} is not the result's arc and face");
    };
    assert!(
        [edge_of(pair, 0), edge_of(pair, 1)].contains(&arc),
        "the row's arc {arc:?} is one of the record's edges, {pair:?}"
    );
    assert!(
        r.body.get_face(floor).is_some(),
        "the row's face {floor:?} is live"
    );
    let (from, to) = r
        .body
        .edge_vertices(arc)
        .unwrap_or_else(|| panic!("{arc:?} is dead"));
    let edge = r.body.get_edge(arc).unwrap();
    assert!(
        [edge.he_plus, edge.he_minus]
            .into_iter()
            .any(|he| r.body.face_of_half_edge(he) == Some(floor)),
        "the row's arc {arc:?} ({from:?}–{to:?}) bounds no face {floor:?}"
    );
}
