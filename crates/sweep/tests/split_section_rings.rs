//! **A plane split through a holed body hands back halves that pass
//! the at-rest gate, with each holed section ONE face whose rings are
//! the holes' sections.** Every fixture is built through the public
//! doors (`topo::subtract`, the sweep fixtures) and cut by `split`.
//!
//! Two layers of the split meet here. The finish nests each section
//! polygon that is a hole into the section face around it, so a bored
//! body's section is an annulus rather than a face over the whole
//! outline plus a coplanar disc cancelling it. The join's order reads
//! crossings on one line of a tilted plane as one column, so a ringed
//! cap the plane crosses is chorded beside its hole, not across it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use sweep::test_support::{bored_cylinder, brick, prism_at};
use topo::splitting::{SplitPart, SplitPlane, split};
use topo::{Body, FaceKey};

fn tol() -> Tol {
    Tol::witness()
}

/// The brick `[−2, 2]² × [0, 2.5]` less a unit rod parallel to `z`
/// about `(cx, cy)`, through the subtract door.
fn cavity_at(cx: f64, cy: f64) -> Body<f64> {
    let block: Body<f64> = brick((-2.0, 2.0), (-2.0, 2.0), (0.0, 2.5), tol());
    let rod: Body<f64> = prism_at(
        vec![
            (Point2::new(cx - 1.0, cy), 1.0),
            (Point2::new(cx + 1.0, cy), 1.0),
        ],
        -0.5,
        3.5,
        tol(),
    );
    match topo::subtract(&block, &rod, tol()) {
        Ok(topo::BooleanResult::Body(b)) => b.body,
        other => panic!("the rod subtracts: {:?}", other.err()),
    }
}

/// The 4³ block less the U-cutter whose prongs, `y ∈ [1, 1.5]` and
/// `[2.5, 3]`, run from `x = 2` out through the `x = 4` wall, `z ∈
/// [1, 3]` — editor-core's `gather_placed_under_two_roots` cutter,
/// through the subtract door.
fn u_cut() -> Body<f64> {
    let block: Body<f64> = brick((0.0, 4.0), (0.0, 4.0), (0.0, 4.0), tol());
    let outline = [
        (2.0, 1.0),
        (6.0, 1.0),
        (6.0, 3.0),
        (2.0, 3.0),
        (2.0, 2.5),
        (5.0, 2.5),
        (5.0, 1.5),
        (2.0, 1.5),
    ];
    let cutter: Body<f64> = prism_at(
        outline
            .iter()
            .map(|&(x, y)| (Point2::new(x, y), 0.0))
            .collect(),
        1.0,
        2.0,
        tol(),
    );
    match topo::subtract(&block, &cutter, tol()) {
        Ok(topo::BooleanResult::Body(b)) => b.body,
        other => panic!("the cutter subtracts: {:?}", other.err()),
    }
}

/// Through `(0, 0, z)`, tilted `t` rad about `y`, flipped when `flip`.
fn tilted(z: f64, t: f64, flip: bool) -> SplitPlane<f64> {
    let s = if flip { -1.0 } else { 1.0 };
    SplitPlane {
        origin: Point3::new(0.0, 0.0, z),
        normal: Vec3::new(s * t.sin(), 0.0, s * t.cos()),
    }
}

fn at_x(x: f64) -> SplitPlane<f64> {
    SplitPlane {
        origin: Point3::new(x, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// The halves `[below, above]`, each asserted to pass tiers 1 and 3.
fn halves(what: &str, body: &Body<f64>, plane: &SplitPlane<f64>) -> [Body<f64>; 2] {
    let result = split(body, plane, tol()).unwrap_or_else(|e| panic!("{what}: splits: {e:?}"));
    let mut out = Vec::new();
    for (side, part) in [("below", result.below), ("above", result.above)] {
        let SplitPart::Body(half) = part else {
            panic!("{what} {side}: material on both sides");
        };
        assert_eq!(topo::validate(&half), Ok(()), "{what} {side}: tier 1");
        assert_eq!(
            topo::validate_geometric(&half, tol()),
            Ok(()),
            "{what} {side}: tier 3"
        );
        out.push(half);
    }
    [out.remove(0), out.remove(0)]
}

/// The ring count of each face of `half` on the plane, in face order.
fn section_rings(half: &Body<f64>, plane: &SplitPlane<f64>) -> Vec<usize> {
    let on: Vec<FaceKey> = half
        .faces()
        .filter(|(_, f)| {
            matches!(
                half.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if normal.cross(plane.normal).norm() < 1e-12
                        && (*origin - plane.origin).dot(plane.normal).abs() < 1e-12
            )
        })
        .map(|(k, _)| k)
        .collect();
    on.iter()
        .map(|&f| half.get_face(f).unwrap().rings.len())
        .collect()
}

/// Every face's ring count, summed.
fn rings(half: &Body<f64>) -> usize {
    half.faces().map(|(_, f)| f.rings.len()).sum()
}

fn volume(half: &Body<f64>) -> f64 {
    topo::mass_properties(half, tol()).unwrap().volume
}

/// **A bore the plane crosses all round is a ring of the one section
/// face**, on each half, for a centred rod, an off-centre one and the
/// bored cylinder, flat and tilted. Where the plane is flat the halves'
/// volumes are the closed forms: half the brick less half the rod, half
/// the annular cylinder.
#[test]
fn a_split_through_a_bore_makes_one_annular_section_face_per_half() {
    use core::f64::consts::PI;
    let cases = [
        (
            "cavity cut flat",
            cavity_at(0.0, 0.0),
            tilted(1.25, 0.0, false),
            Some(20.0 - 1.25 * PI),
        ),
        (
            "cavity cut at tilt 0.3",
            cavity_at(0.0, 0.0),
            tilted(1.25, 0.3, false),
            None,
        ),
        (
            "off-centre cavity cut flat",
            cavity_at(0.8, 0.0),
            tilted(1.25, 0.0, false),
            Some(20.0 - 1.25 * PI),
        ),
        (
            "bored cylinder cut flat",
            bored_cylinder(0.3, 0.2, 0.37, tol()),
            tilted(0.5, 0.0, false),
            Some(0.5 * PI * (1.0 - 0.09)),
        ),
        (
            "bored cylinder cut at tilt 0.3",
            bored_cylinder(0.3, 0.2, 0.37, tol()),
            tilted(0.5, 0.3, false),
            None,
        ),
    ];
    for (what, body, plane, half_volume) in cases {
        for (side, half) in ["below", "above"]
            .into_iter()
            .zip(halves(what, &body, &plane))
        {
            assert_eq!(
                section_rings(&half, &plane),
                vec![1],
                "{what} {side}: one section face, the bore its one ring"
            );
            if let Some(v) = half_volume {
                assert!(
                    (volume(&half) - v).abs() < 1e-9,
                    "{what} {side}: volume {} against {v}",
                    volume(&half)
                );
            }
        }
    }
}

/// **The U-cutter's pockets are two rings of one section face.** The
/// plane at `x = 3` (and at `x = 3.9`) crosses both prongs, so each
/// half's section is the block's square with the prongs' two
/// rectangles as rings; the volumes are the block's slab less the
/// prongs' part of it (`x ∈ [2, 4]` of each, `0.5 × 2` in section).
#[test]
fn a_split_through_the_u_cutters_pockets_makes_one_face_with_two_rings() {
    let body = u_cut();
    for (x, below, above) in [(3.0, 46.0, 14.0), (3.9, 58.6, 1.4)] {
        let what = format!("U-cutter split at x = {x}");
        let plane = at_x(x);
        for ((side, half), v) in ["below", "above"]
            .into_iter()
            .zip(halves(&what, &body, &plane))
            .zip([below, above])
        {
            assert_eq!(
                section_rings(&half, &plane),
                vec![2],
                "{what} {side}: one section face, each prong a ring"
            );
            assert!(
                (volume(&half) - v).abs() < 1e-9,
                "{what} {side}: volume {} against {v}",
                volume(&half)
            );
        }
    }
}

/// **A ringed cap the plane crosses is chorded beside its hole.** A
/// steep plane meets the `z = 0` cap of the brick (`z = 1` of the
/// bored cylinder) through the bore's rim, and the other cap beside
/// it: the crossed cap's fragments are each one loop with no ring,
/// the other cap keeps its ring whole on the side the rim lies on, and
/// each half's section is one face notched by the bore. Two more rod
/// positions put the plane through the bore on both caps, so the bore
/// parts each half's section in two.
#[test]
fn a_split_through_a_ringed_cap_chords_it_beside_its_hole() {
    let cases = [
        (
            "off-centre cavity cut at tilt 1.4, flipped",
            cavity_at(0.8, 0.0),
            tilted(1.25, 1.4, true),
            [vec![0], vec![0]],
            [1, 0],
        ),
        (
            "bored cylinder cut at tilt -1, flipped",
            bored_cylinder(0.3, 0.2, 0.37, tol()),
            tilted(0.5, -1.0, true),
            [vec![0], vec![0]],
            [0, 1],
        ),
        (
            "cavity about (0.4, -0.3) cut at tilt 1.4, flipped",
            cavity_at(0.4, -0.3),
            tilted(1.25, 1.4, true),
            [vec![0, 0], vec![0, 0]],
            [0, 0],
        ),
        (
            "cavity about (0.5, 0) cut at tilt 1.4, flipped",
            cavity_at(0.5, 0.0),
            tilted(1.25, 1.4, true),
            [vec![0, 0], vec![0, 0]],
            [0, 0],
        ),
    ];
    for (what, body, plane, sections, ringed) in cases {
        for (((side, half), section), ringed) in ["below", "above"]
            .into_iter()
            .zip(halves(what, &body, &plane))
            .zip(sections)
            .zip(ringed)
        {
            assert_eq!(
                section_rings(&half, &plane),
                section,
                "{what} {side}: the section faces' ring counts"
            );
            assert_eq!(
                rings(&half),
                ringed,
                "{what} {side}: the rings any face carries"
            );
        }
    }
}
