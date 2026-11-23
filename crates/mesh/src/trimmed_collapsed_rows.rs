//! **Rows for a spline chart's collapsed side** (`super::CollapsedSides`,
//! `super::SidePoints`): the walls a loft builds beside a pinned corner, a
//! hinge or a lune, each with a side of its chart that is one point.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp
)]

use std::collections::HashSet;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::test_support::{pinned_square, rigid_places, turn_about};
use topo::Body;

use super::{Along, CollapsedSide, CollapsedSides, PolygonOf, SidePoints};
use crate::chart_table::ChartRow;

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
/// and a hinge in the first, middle or last slab, whose rim is a cap's
/// in the first and last), a pinned corner (in a lone slab, in the first
/// or last slab of two, through three at v-degree 1, and through three at
/// v-degree 2, where one wall spans the three), and a lune, collapsed at
/// both ends of its chart's `u`. Every side is a `u` column: the loft
/// builds no collapsed `v` row, and no import fixture holds a collapsed
/// side, so a `v` side is covered by the unit rows alone.
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
            "one corner, first slab of two",
            loft(4, &[id(), corner(0.5), lift(2.0) * corner(0.5)], 1),
        ),
        (
            "one corner, last slab of two",
            loft(4, &[lift(-2.0), id(), corner(0.5)], 1),
        ),
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

/// The witness run's band.
fn band() -> geom_core::predicate::Band {
    geom_core::predicate::Band::linear(Tol::witness()).unwrap()
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

/// **A collapsed side is walked as its unpinned twin's edge is chorded.**
/// The wall beside a corner pinned in one slab, and the same wall with
/// the corner moved off by a hair, which builds a strut edge where the
/// side was: the side holds as many pieces as the strut's chords, the
/// larger of the two walls' asks (the chord pass sizes an edge for both
/// faces it bounds). Each copy of the vertex lies on the side, exactly
/// where its stored chart column is.
#[test]
fn a_collapsed_side_is_walked_as_its_unpinned_twins_strut_is_chorded() {
    let band = band();
    for delta in [1e-1, 1e-2, 1e-3] {
        let pinned = loft(4, &[Affine3::identity(), about_corner(0.5, 0.0)], 1);
        let (chords, shared, bounds) = crate::chords::tests::witness_pass(&pinned, delta);
        let mut pieces = Vec::new();
        for fk in collapsed(&pinned) {
            let surface = pinned.face_surface(fk).unwrap();
            let row = ChartRow::of(&pinned, fk, surface, &bounds).unwrap();
            let lift = pinned.face_lift(fk, band).unwrap();
            let (singular, polygon) = super::face_polygon(&PolygonOf {
                body: &pinned,
                fk,
                surface,
                row: &row,
                lift: &lift,
                creases: None,
                chords: &chords,
                shared: &shared,
                band,
                delta,
            })
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
        let (chords, shared, _) = crate::chords::tests::witness_pass(&twin, delta);
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

/// **A vertex is on a collapsed side where the band decides it there**,
/// as every singular point is decided (`chart_table`'s
/// `at_a_singular_point`): the side's own point and one an ulp off are
/// on it, one past the band's escalation is off it, and one between is
/// undecided, which `singular_vertices` refuses typed.
#[test]
fn a_collapsed_sides_vertex_is_decided_at_the_band() {
    let band = band();
    let body = loft(4, &[Affine3::identity(), about_corner(0.5, 0.0)], 1);
    let fk = collapsed(&body)[0];
    let sides = CollapsedSides::of(body.face_surface(fk).unwrap());
    let p = sides.0[0].point;
    let along = |d: f64| Point3::new(p.x + d, p.y, p.z);
    assert_eq!(sides.holds(p, band), Ok(true), "the side's own point");
    assert_eq!(
        sides.holds(Point3::new(p.x.next_up(), p.y, p.z), band),
        Ok(true),
        "a point an ulp off"
    );
    assert_eq!(
        sides.holds(along(0.5 * band.zero()), band),
        Ok(true),
        "a point inside the band's zero"
    );
    assert_eq!(
        sides.holds(along(2.0 * band.escalate()), band),
        Ok(false),
        "a point past its escalation"
    );
    assert!(
        sides
            .holds(along(0.5 * (band.zero() + band.escalate())), band)
            .is_err(),
        "a point between is undecided"
    );
}

/// **A side is walked along its own axis, at that axis's step and held
/// value, by a vertex on it alone.** A `u` side runs along `v`, a `v` side
/// along `u`; a vertex on two sides is their corner and is walked by
/// neither.
#[test]
fn a_side_is_walked_along_its_axis_and_a_corner_is_not() {
    use geom_brep::{ChartAxis, ChartEnd, ChartSide};
    let side = |fixed, end, point, held| CollapsedSide {
        side: ChartSide { fixed, end },
        point,
        held,
    };
    let (p, q) = (Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    let shared = [p, q];
    let singular: HashSet<u32> = [0, 1].into_iter().collect();
    let sides = CollapsedSides(vec![
        side(ChartAxis::U, ChartEnd::Low, p, 0.0),
        side(ChartAxis::V, ChartEnd::High, q, 1.0),
        side(ChartAxis::U, ChartEnd::High, q, 1.0),
    ]);
    let walk = sides.points(&shared, &singular, (0.25, 0.1), band());
    assert_eq!(
        walk.along.get(&0),
        Some(&Along {
            fixed: ChartAxis::U,
            held: 0.0,
            step: 0.1
        }),
        "a u side runs along v"
    );
    assert_eq!(
        walk.along.get(&1),
        None,
        "a corner of two sides is not walked"
    );
    let sides = CollapsedSides(vec![side(ChartAxis::V, ChartEnd::Low, q, 2.0)]);
    let walk = sides.points(&shared, &singular, (0.25, 0.1), band());
    assert_eq!(
        walk.along.get(&1),
        Some(&Along {
            fixed: ChartAxis::V,
            held: 2.0,
            step: 0.25
        }),
        "a v side runs along u"
    );
}

/// **A walked side holds `⌈span/h⌉` pieces between its copies**, each
/// point at the side's held value on its fixed axis (whatever the copies'
/// rounding there) and spaced evenly along the other, beside the edge of
/// the copy it follows, the loop's closing pair included; a copy pair of
/// another vertex, and two points of the walked vertex at one place on
/// the axis, are left as they are.
#[test]
fn a_walked_side_holds_its_pieces_between_its_copies() {
    use geom_brep::ChartAxis;
    let keys = super::tests::twelve_edge_keys();
    let e = |k: usize| keys[k];
    let walk = SidePoints {
        along: [(
            7,
            Along {
                fixed: ChartAxis::U,
                held: 0.0,
                step: 0.3,
            },
        )]
        .into_iter()
        .collect(),
    };
    // The side runs from the copy an ulp off it at (0⁺, 1) down to (0, 0),
    // and closes the loop.
    let off = 0.0f64.next_up();
    let points = vec![(0.0, 0.0, 7), (1.0, 0.0, 2), (1.0, 1.0, 3), (off, 1.0, 7)];
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
        "on the side's held u, in walk order"
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

/// **The witness of why the side is walked**: the corner pinned through
/// three sections at v-degree 2, one wall over the three, collapsed at
/// the corner over its whole span. Holding the vertex's copy alone (the
/// analytic arms' pole shape) leaves a triangle beside the side whose
/// chart box spans the column, and that bound does not fall with δ: it
/// sits near 0.06 from δ = 3·10⁻² down, so the face refuses there. Walked,
/// it meshes.
#[test]
fn the_wall_pinned_through_three_sections_meshes_only_with_its_side_walked() {
    let body = loft(
        4,
        &[
            Affine3::identity(),
            about_corner(0.3, 0.0),
            about_corner(0.6, 0.0),
        ],
        2,
    );
    for delta in [3e-2, 1e-2] {
        let mesh = crate::tessellate(&body, delta, Tol::witness())
            .unwrap_or_else(|e| panic!("δ = {delta}: the v-degree-2 wall meshes, got {e:?}"));
        crate::validate::check_mesh(&mesh).unwrap();
    }
}

/// **Two collapsed sides on one face, each walked**: the lune, the half
/// disc's arc wall turned about its chord, is collapsed at both ends of
/// its chart's `u`, each at its own vertex. The polygon holds each
/// vertex's copies down its own column, ⌈1/h_v⌉ pieces on each.
#[test]
fn a_lune_walks_both_its_collapsed_sides() {
    let band = band();
    let body = loft(
        2,
        &[
            Affine3::identity(),
            turn_about([0.0; 3], [1.0, 0.0, 0.0], 0.8),
        ],
        1,
    );
    let delta = 1e-3;
    let (chords, shared, bounds) = crate::chords::tests::witness_pass(&body, delta);
    let walls = collapsed(&body);
    assert_eq!(walls.len(), 1, "the lune is one wall");
    let fk = walls[0];
    let surface = body.face_surface(fk).unwrap();
    let row = ChartRow::of(&body, fk, surface, &bounds).unwrap();
    let lift = body.face_lift(fk, band).unwrap();
    let (singular, polygon) = super::face_polygon(&PolygonOf {
        body: &body,
        fk,
        surface,
        row: &row,
        lift: &lift,
        creases: None,
        chords: &chords,
        shared: &shared,
        band,
        delta,
    })
    .unwrap();
    assert_eq!(singular.len(), 2, "one vertex per side");
    let (_, hv) = row.steps(super::face_target(
        surface,
        crate::tessellate::Lane::Trimmed,
        delta,
    ));
    let want = crate::sizing::ceil_count(1.0, hv).unwrap() + 1;
    for id in &singular {
        let copies: Vec<(f64, f64)> = polygon
            .points
            .iter()
            .filter(|p| p.2 == *id)
            .map(|&(u, v, _)| (u, v))
            .collect();
        assert!(
            copies.len() == want && copies.iter().all(|&(u, _)| u == copies[0].0),
            "vertex {id}: {want} copies down one column, got {copies:?}"
        );
    }
    let mut columns: Vec<u64> = polygon
        .points
        .iter()
        .filter(|p| singular.contains(&p.2))
        .map(|p| p.0.to_bits())
        .collect();
    columns.sort_unstable();
    columns.dedup();
    assert_eq!(
        columns.len(),
        2,
        "the two sides are the chart's two columns"
    );
}

/// **A vertex in the band of a collapsed side but not its stored point is
/// on it**, as the lift decides it: the wedge, its vertex points restated
/// an ulp up (no door builds one; an import could), keeps the vertex's
/// copies down the collapsed column where the lift turns the face, and
/// meshes at every δ the wedge does, triangle for triangle.
#[test]
fn a_vertex_in_a_sides_band_is_on_it_and_meshes_as_the_vertex_on_its_point() {
    let band = band();
    let hinge = turn_about([-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], 0.5);
    let on = loft(4, &[Affine3::identity(), hinge], 1);
    let mut off = on.clone();
    topo::test_support::restate_geometry(&mut off, |at, x| match at {
        topo::test_support::Stored::Point => x.next_up(),
        _ => x,
    });
    let fk = collapsed(&off)[0];
    let surface = off.face_surface(fk).unwrap();
    let sides = CollapsedSides::of(surface);
    let side_point = sides.0[0].point;
    let (vertex, gap) = off
        .vertex_points()
        .map(|(_, p)| (p, (p - side_point).norm()))
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .unwrap();
    assert!(
        gap > 0.0 && sides.holds(vertex, band) == Ok(true),
        "the vertex is {gap} off the side's point and decided on it"
    );
    let (chords, shared, bounds) = crate::chords::tests::witness_pass(&off, 1e-2);
    let row = ChartRow::of(&off, fk, surface, &bounds).unwrap();
    let lift = off.face_lift(fk, band).unwrap();
    let (singular, polygon) = super::face_polygon(&PolygonOf {
        body: &off,
        fk,
        surface,
        row: &row,
        lift: &lift,
        creases: None,
        chords: &chords,
        shared: &shared,
        band,
        delta: 1e-2,
    })
    .unwrap();
    assert_eq!(singular.len(), 1, "the vertex is read singular");
    assert!(
        polygon
            .points
            .iter()
            .filter(|p| singular.contains(&p.2))
            .count()
            > 2,
        "its side is walked"
    );
    let counts = |body: &Body<f64>, delta: f64| -> Vec<usize> {
        let mesh = crate::tessellate(body, delta, Tol::witness())
            .unwrap_or_else(|e| panic!("δ = {delta}: meshes, got {e:?}"));
        mesh.patches.iter().map(|p| p.triangles.len()).collect()
    };
    for delta in [1e-1, 1e-2, 1e-3] {
        assert_eq!(
            counts(&off, delta),
            counts(&on, delta),
            "δ = {delta}: the restated wedge meshes face for face as the wedge"
        );
    }
}

/// One FNV-1a step over `x`'s bytes.
fn fnv(h: &mut u64, x: u64) {
    for b in x.to_le_bytes() {
        *h ^= u64::from(b);
        *h = h.wrapping_mul(0x0100_0000_01b3);
    }
}

/// **The class's meshes are one byte stream at every ε.** The singular
/// vertex a spline face walks its side through is decided at the band
/// ([`CollapsedSides::holds`]), so ε could choose the polygon where a
/// vertex sits inside the band at one ε and outside it at another. On
/// every body the loft builds the vertex is the side's own point, decided
/// on at any band, so the mesh is the same bytes at every ε: each body's
/// FNV-1a over its positions' bits and its triangles at δ = 10⁻² is
/// pinned, and the suite's ε rows (10⁻⁹, 10⁻⁶, 10⁻¹²) each re-derive it
/// in their own process. The guard the classification owes on a vertex
/// the band can flip is
/// `work/tesslane/pole-identification-lets-eps-choose-between-two-meshes-of-a-cone-face.md`'s.
#[test]
fn the_class_meshes_to_one_digest_at_every_eps() {
    let pinned: [u64; 11] = [
        0x4a35_f95f_f08f_36d8,
        0xa9ff_f939_5870_71d4,
        0x8dae_5c35_b89e_6abb,
        0xc4aa_f59e_712f_29fa,
        0xa812_21f7_18fc_e776,
        0x12c1_ce7a_18ff_f50d,
        0x2c6f_7c0d_2fd5_66bf,
        0x4e72_791f_c1bb_da04,
        0x8025_9ad5_84ed_5012,
        0x519c_93fe_1c05_8ee4,
        0x3c56_afa7_98dd_e63e,
    ];
    let got: Vec<u64> = the_class()
        .iter()
        .map(|(what, body)| {
            let mesh = crate::tessellate(body, 1e-2, Tol::witness())
                .unwrap_or_else(|e| panic!("{what}: meshes, got {e:?}"));
            let mut h: u64 = 0xcbf2_9ce4_8422_2325;
            for p in &mesh.positions {
                fnv(&mut h, p.x.to_bits());
                fnv(&mut h, p.y.to_bits());
                fnv(&mut h, p.z.to_bits());
            }
            for t in mesh.patches.iter().flat_map(|p| &p.triangles) {
                for &i in t {
                    fnv(&mut h, u64::from(i));
                }
            }
            h
        })
        .collect();
    assert_eq!(
        got, pinned,
        "the class's digests at δ = 1e-2, in `the_class` order: {got:#x?}"
    );
}
