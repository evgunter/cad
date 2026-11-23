//! **Rows for a spline chart's collapsed side** (`super::CollapsedSides`,
//! `super::SideWalk`): the walls a loft builds beside a pinned corner, a
//! hinge or a lune, each with a side of its chart that is one point.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp
)]

use std::collections::{HashMap, HashSet};

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::test_support::{pinned_square, rigid_places, turn_about};
use topo::Body;

use super::{CollapsedSides, SideWalk};
use crate::chart_table::ChartRow;
use crate::chords::ChordPass;

/// The square turned about the diagonal through its corner (−1, −1),
/// then lifted `off` along z: at `off = 0` that corner is pinned.
fn about_corner(angle: f64, off: f64) -> Affine3<f64> {
    Affine3::translation(Vec3::new(0.0, 0.0, off))
        * turn_about([-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], angle)
}

fn loft(sections: usize, ms: &[Affine3<f64>], v_degree: usize) -> Body<f64> {
    let tol = Tol::witness();
    let section = if sections == 4 {
        pinned_square()
    } else {
        // A half disc on the chord from (−1, 0) to (1, 0).
        vec![profile::test_support::bulge_loop(vec![
            (Point2::new(-1.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 1.0),
        ])]
    };
    sweep::loft_body::<f64>(
        &vec![section; ms.len()],
        &rigid_places(ms, tol),
        v_degree,
        tol,
    )
    .unwrap_or_else(|e| panic!("the loft builds: {e}"))
    .body
    .into_body()
}

/// **Every collapsed side the loft builds**, by name: a hinge (the wedge,
/// and a hinge in the first, middle or last slab), a pinned corner (in
/// one slab, through three at v-degree 1, and through three at v-degree
/// 2, where one wall spans the three), and a lune, collapsed at both ends
/// of its chart's `u`. Every side is a `u` column: no door builds a
/// collapsed `v` row.
fn the_class() -> Vec<(&'static str, Body<f64>)> {
    let hinge = |a| turn_about([-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], a);
    let far = || turn_about([0.0, 1.0, 0.0], [1.0, 0.0, 0.0], -0.3);
    let lift = |z| Affine3::translation(Vec3::new(0.0, 0.0, z));
    let lune = || turn_about([0.0; 3], [1.0, 0.0, 0.0], 0.8);
    let id = Affine3::identity;
    let corner = |a| about_corner(a, 0.0);
    vec![
        ("wedge", loft(4, &[id(), hinge(0.5)], 1)),
        ("one corner", loft(4, &[id(), corner(0.5)], 1)),
        (
            "corner through three, v-degree 1",
            loft(4, &[id(), corner(0.3), corner(0.6)], 1),
        ),
        (
            "corner through three, v-degree 2",
            loft(4, &[id(), corner(0.3), corner(0.6)], 2),
        ),
        ("hinge, first slab", loft(4, &[id(), far(), lift(2.0)], 1)),
        (
            "hinge, middle slab",
            loft(4, &[lift(-2.0), id(), far(), lift(2.0) * far()], 1),
        ),
        ("hinge, last slab", loft(4, &[lift(-2.0), id(), far()], 1)),
        ("lune", loft(2, &[id(), lune()], 1)),
        ("lune, last slab", loft(2, &[lift(-2.0), id(), lune()], 1)),
    ]
}

/// The faces of `body` with a collapsed side.
fn collapsed(body: &Body<f64>) -> Vec<topo::FaceKey> {
    body.faces()
        .map(|(fk, _)| fk)
        .filter(|&fk| !CollapsedSides::of(body.face_surface(fk).unwrap()).is_empty())
        .collect()
}

/// **The class meshes, closed and wound out, at every δ down to 10⁻³**,
/// and the wedge encloses its prism `4·sin 0.5` within `δ` times its
/// area. A wall whose collapsed side the polygon cuts across, or holds
/// once, refuses `CertificateExceeded` here: on every body at 10⁻³, and
/// on the v-degree-2 wall at every δ.
#[test]
fn every_collapsed_side_the_loft_builds_meshes_at_a_fine_chordal() {
    for (what, body) in the_class() {
        assert!(
            !collapsed(&body).is_empty(),
            "{what}: a wall with a collapsed side"
        );
        for delta in [1e-1, 1e-2, 1e-3] {
            let mesh = crate::tessellate(&body, delta, Tol::witness())
                .unwrap_or_else(|e| panic!("{what} at δ = {delta}: meshes, got {e:?}"));
            crate::validate::check_mesh(&mesh)
                .unwrap_or_else(|e| panic!("{what} at δ = {delta}: closed, got {e:?}"));
            crate::validate::check_winding(&mesh)
                .unwrap_or_else(|e| panic!("{what} at δ = {delta}: resolved, got {e:?}"));
            if what == "wedge" {
                let area = 4.0 * 0.5f64.sin() + 8.0 + 8.0 * 0.25f64.sin();
                crate::validate::check_volume(&mesh, 4.0 * 0.5f64.sin(), area, delta)
                    .unwrap_or_else(|e| panic!("wedge at δ = {delta}: encloses its prism, {e:?}"));
            }
        }
    }
}

/// **Each sample of a collapsed-side wall's triangles lies within its own
/// triangle's certificate** (`crate::budget`'s deviation pass, the
/// falsifier of every spline face), at the finest δ the class row meshes.
#[cfg(feature = "budget")]
#[test]
fn a_collapsed_side_walls_samples_stay_under_their_certificates() {
    use crate::budget::{self, Mode};
    for (what, body) in the_class() {
        let walls = collapsed(&body);
        budget::arm(Mode::Deviation {
            samples_per_edge: 6,
        });
        let run = crate::tessellate(&body, 1e-3, Tol::witness());
        let measures = budget::take();
        run.unwrap_or_else(|e| panic!("{what}: meshes, got {e:?}"));
        for fk in &walls {
            let m = measures
                .iter()
                .find(|m| m.face == *fk)
                .unwrap_or_else(|| panic!("{what}: face {fk:?} is measured"));
            assert!(
                m.dev_samples > 0 && m.worst_ratio <= 1.0,
                "{what}: face {fk:?}'s samples exceed their own certificates: {m:?}"
            );
        }
    }
}

/// The run's chord pass on `body` at `delta`, with the shared positions.
fn chord_pass(
    body: &Body<f64>,
    delta: f64,
) -> (ChordPass, Vec<Point3<f64>>, crate::nurbs_cert::FaceBounds) {
    let mut positions = Vec::new();
    let mut vids = HashMap::new();
    for (vk, p) in body.vertex_points() {
        #[allow(clippy::cast_possible_truncation)]
        vids.insert(vk, positions.len() as u32);
        positions.push(p);
    }
    let mut bounds = crate::nurbs_cert::FaceBounds::new();
    let routes = crate::tessellate::Routes::witness(body);
    let chords =
        crate::chords::compute_chords(body, delta, &routes, &vids, &mut positions, &mut bounds)
            .unwrap();
    (chords, positions, bounds)
}

/// **A collapsed side is walked as its unpinned twin's edge is chorded.**
/// The wall beside a corner pinned in one slab, and the same wall with
/// the corner moved off by a hair, which builds a strut edge where the
/// side was: the side holds as many pieces as the strut's chords, the
/// larger of the two walls' asks (the chord pass sizes an edge for both
/// faces it bounds). Each copy of the vertex lies on the side, exactly
/// where its stored chart column is.
#[test]
fn a_collapsed_side_is_walked_as_its_unpinned_twins_strut_is_chorded() {
    let band = geom_core::predicate::Band::linear(Tol::witness()).unwrap();
    for delta in [1e-1, 1e-2, 1e-3] {
        let pinned = loft(4, &[Affine3::identity(), about_corner(0.5, 0.0)], 1);
        let (chords, shared, bounds) = chord_pass(&pinned, delta);
        let mut pieces = Vec::new();
        for fk in collapsed(&pinned) {
            let surface = pinned.face_surface(fk).unwrap();
            let row = ChartRow::of(&pinned, fk, surface, &bounds).unwrap();
            let lift = pinned.face_lift(fk, band).unwrap();
            let (singular, polygon) = super::face_polygon(
                &pinned,
                fk,
                (surface, &row),
                &lift,
                &chords,
                &shared,
                band,
                delta,
                None,
            )
            .unwrap();
            assert_eq!(singular.len(), 1, "δ = {delta}, {fk:?}: one pinned vertex");
            let copies: Vec<(f64, f64)> = polygon
                .points
                .iter()
                .filter(|p| singular.contains(&p.2))
                .map(|&(u, v, _)| (u, v))
                .collect();
            let column = copies[0].0;
            assert!(
                copies.iter().all(|&(u, _)| u == column),
                "δ = {delta}, {fk:?}: every copy on the side's column, {copies:?}"
            );
            pieces.push(copies.len() - 1);
        }
        let twin = loft(4, &[Affine3::identity(), about_corner(0.5, 1e-4)], 1);
        let (chords, shared, _) = chord_pass(&twin, delta);
        // The strut: the one edge a hair long.
        let struts: Vec<usize> = chords
            .ids
            .values()
            .filter(|ids| {
                let (a, b) = (shared[ids[0] as usize], shared[ids[ids.len() - 1] as usize]);
                (a - b).norm() < 1e-3
            })
            .map(|ids| ids.len() - 1)
            .collect();
        assert_eq!(struts.len(), 1, "δ = {delta}: the twin has one strut");
        assert_eq!(
            pieces.iter().max(),
            Some(&struts[0]),
            "δ = {delta}: the walls' sides {pieces:?} against the twin's strut"
        );
    }
}

/// **A side is a vertex's only where its stored point is the side's**: no
/// band reads it, so a vertex an ulp off the side keeps its polygon's
/// one-point shape, and the mesh stays a function of (body, δ).
#[test]
fn a_collapsed_sides_vertex_is_its_stored_point_and_nothing_near_it() {
    let body = loft(4, &[Affine3::identity(), about_corner(0.5, 0.0)], 1);
    let fk = collapsed(&body)[0];
    let sides = CollapsedSides::of(body.face_surface(fk).unwrap());
    let p = sides.0[0].1;
    assert!(sides.holds(p), "the side's own point");
    let off = Point3::new(p.x.next_up(), p.y, p.z);
    assert!(!sides.holds(off), "a point an ulp off the side's");
}

/// **A side is walked along its own axis, at that axis's step, by a
/// vertex on it alone.** A `u` side runs along `v`, a `v` side along `u`;
/// a vertex on two sides is their corner and is walked by neither.
#[test]
fn a_side_is_walked_along_its_axis_and_a_corner_is_not() {
    use geom_brep::{ChartAxis, ChartEnd, ChartSide};
    let side = |fixed, end| ChartSide { fixed, end };
    let (p, q) = (Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    let shared = [p, q];
    let singular: HashSet<u32> = [0, 1].into_iter().collect();
    let sides = CollapsedSides(vec![
        (side(ChartAxis::U, ChartEnd::Low), p),
        (side(ChartAxis::V, ChartEnd::High), q),
        (side(ChartAxis::U, ChartEnd::High), q),
    ]);
    let walk = sides.walk(&shared, &singular, (0.25, 0.1));
    assert_eq!(walk.steps.get(&0), Some(&(1, 0.1)), "a u side runs along v");
    assert_eq!(
        walk.steps.get(&1),
        None,
        "a corner of two sides is not walked"
    );
    let sides = CollapsedSides(vec![(side(ChartAxis::V, ChartEnd::Low), q)]);
    let walk = sides.walk(&shared, &singular, (0.25, 0.1));
    assert_eq!(
        walk.steps.get(&1),
        Some(&(0, 0.25)),
        "a v side runs along u"
    );
}

/// **A walked side holds `⌈span/h⌉` pieces between its copies**, each
/// point on the segment between them and beside the edge of the copy it
/// follows, the loop's closing pair included; a copy pair of another
/// vertex, and two points of the walked vertex at one place on the axis,
/// are left as they are.
#[test]
fn a_walked_side_holds_its_pieces_between_its_copies() {
    let keys = super::tests::twelve_edge_keys();
    let e = |k: usize| keys[k];
    let walk = SideWalk {
        steps: [(7, (1, 0.3))].into_iter().collect(),
    };
    // The side runs from (0, 1) down to (0, 0) and closes the loop.
    let points = vec![(0.0, 0.0, 7), (1.0, 0.0, 2), (1.0, 1.0, 3), (0.0, 1.0, 7)];
    let edges = vec![e(1), e(2), e(3), e(4)];
    let (got, got_edges) = walk.along((points, edges)).unwrap();
    let side: Vec<(f64, f64)> = got[4..].iter().map(|&(u, v, _)| (u, v)).collect();
    assert_eq!(
        got.len(),
        4 + 3,
        "⌈1/0.3⌉ = 4 pieces, three points: {got:?}"
    );
    assert_eq!(
        side,
        [(0.0, 0.75), (0.0, 0.5), (0.0, 0.25)],
        "on the side, in walk order"
    );
    assert!(
        got[4..].iter().all(|p| p.2 == 7),
        "each a copy of the vertex"
    );
    assert!(
        got_edges[4..].iter().all(|&k| k == e(4)),
        "beside the edge they follow"
    );
    // A copy pair at one `v`, and a pair of a vertex no side holds.
    let points = vec![(0.0, 0.0, 7), (0.5, 0.0, 7), (1.0, 1.0, 3), (2.0, 1.0, 3)];
    let edges = vec![e(1), e(2), e(3), e(4)];
    let (got, _) = walk.along((points.clone(), edges)).unwrap();
    assert_eq!(got, points, "nothing to walk");
}
