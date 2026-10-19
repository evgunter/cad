//! **Curved parts of two solids that no direction separates**, read by
//! the census's depth gap (`topo::census::depth_gap`): nested thin walls
//! a shell leaves, a rod in a bore, a tube in a tube. Each is two solids
//! whose reach boxes nest, so before the depth gap every curved pair
//! refused `CensusUndecidable` "a curved face of one is within reach of
//! the other"
//! (`work/restread/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`).
//! Apart, the empty-record tier 3′ passes; touching, or apart by less
//! than the band, it still refuses with that finding, which the census
//! keeps until it can read a curved touch.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, RigidMotion, Tol, Vec3};
use sweep::test_support::finished;
use topo::{AtRestBody, Body, ContactRecords, EntityId, ValidationError};

use super::shell7_common::tube_torus_hollow;

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

/// The census backstop's reason for a curved pair it cannot separate.
const CURVED_WITHIN_REACH: &str = "a curved face of one is within reach of the other";

/// A full revolve about `y` of the rectangle `x ∈ [inner, outer]`,
/// `y ∈ [0, h]`: a rod where `inner` is zero, a tube otherwise.
fn revolved(inner: f64, outer: f64, h: f64) -> Body<f64> {
    sweep::test_support::revolved_about_y(
        vec![
            (Point2::new(inner, 0.0), 0.0),
            (Point2::new(outer, 0.0), 0.0),
            (Point2::new(outer, h), 0.0),
            (Point2::new(inner, h), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    )
}

fn shelled(what: &str, body: Body<f64>, t: f64) -> AtRestBody<f64> {
    let s = topo::shell(&finished(what, body, tol()), t, tol()).expect("the shell builds");
    finished(what, s.body, tol())
}

/// `inner` grafted into `outer`'s body, moved by `by`: two solids of one
/// body, at rest.
fn nested(outer: Body<f64>, inner: &Body<f64>, by: Vec3<f64>) -> AtRestBody<f64> {
    let mut out = outer;
    let placed =
        topo::transform_rigid(inner, &RigidMotion::translation(by).unwrap(), tol()).unwrap();
    topo::graft_disjoint(&mut out, &placed).expect("the placed copy grafts");
    finished("the nested pair", out, tol())
}

fn census(body: &AtRestBody<f64>) -> Result<(), Vec<ValidationError>> {
    topo::validate_pseudomanifold(body, &ContactRecords::default(), tol())
}

/// Whether `errs` holds the backstop's curved-pair refusal on two faces
/// of different solids.
fn refuses_curved_within_reach(body: &AtRestBody<f64>, errs: &[ValidationError]) -> bool {
    errs.iter().any(|e| match e {
        ValidationError::CensusUndecidable {
            a: EntityId::Face(a),
            b: EntityId::Face(b),
            what,
        } => what.contains(CURVED_WITHIN_REACH) && body.solid_of_face(*a) != body.solid_of_face(*b),
        _ => false,
    })
}

/// **The row's minimal repro**: a 1 × 2 revolved cylinder shelled at
/// 0.2, then shelled again at 0.05, is two nested thin solids (4
/// shells, 16 faces); a hollow torus shelled again is two nested thin
/// tubes. Both pass the census on no records. Red on main, where the
/// cylinder's 32 face pairs and the torus's 16 refuse.
#[test]
fn a_hollow_shelled_again_passes_the_census() {
    let twice = shelled(
        "the cylinder shelled twice",
        shelled("the hollow cylinder", revolved(0.0, 1.0, 2.0), 0.2).into_body(),
        0.05,
    );
    assert_eq!(twice.solids().count(), 2, "two thin solids");
    assert_eq!(twice.faces().count(), 16, "sixteen faces");
    assert_eq!(census(&twice), Ok(()), "the nested cylinders");
    let tori = shelled("the hollow torus", tube_torus_hollow(2.0, 0.5, 0.125), 0.05);
    assert_eq!(tori.solids().count(), 2, "two thin tubes");
    assert_eq!(census(&tori), Ok(()), "the nested tori");
}

/// **A rod in a bore**: a rod of radius 0.4 over `y ∈ [0.5, 1.5]` in
/// a tube `r ∈ [0.6, 1]` over `y ∈ [0, 2]`, on the axis and 0.1 off it,
/// passes; its wall and caps lie below the bore's depth about either
/// axis. Red on main.
#[test]
fn a_rod_in_a_bore_passes_the_census() {
    let rod = revolved(0.0, 0.4, 1.0);
    for (what, by) in [
        ("on the axis", Vec3::new(0.0, 0.5, 0.0)),
        ("off the axis", Vec3::new(0.1, 0.5, 0.0)),
    ] {
        let pair = nested(revolved(0.6, 1.0, 2.0), &rod, by);
        assert_eq!(census(&pair), Ok(()), "the rod {what}");
    }
}

/// **A rod wider than the bore it sits in refuses, cap and wall**: the
/// rod `r ≤ 0.8` crosses the bore wall `r = 0.6`, its caps cut by it
/// along a circle no edge holds. The caps' least depth about the axis is
/// zero, at their centres, so no gap clears a cap against the wall, and
/// the material test finds the rod's rim in the tube. Red where a planar
/// face's least depth is read off its boundary without first placing
/// the axis's pierce outside it.
#[test]
fn a_rod_crossing_its_bore_refuses_its_caps() {
    let pair = nested(
        revolved(0.6, 1.0, 2.0),
        &revolved(0.0, 0.8, 1.0),
        Vec3::new(0.0, 0.5, 0.0),
    );
    let errs = census(&pair).expect_err("the rod crosses the bore");
    let plane = |f| {
        matches!(
            pair.get_face(f).and_then(|d| pair.get_surface(d.surface)),
            Some(geom::Surface::Plane { .. })
        )
    };
    let cylinder = |f| {
        matches!(
            pair.get_face(f).and_then(|d| pair.get_surface(d.surface)),
            Some(geom::Surface::Cylinder { .. })
        )
    };
    let cap_on_wall = errs.iter().any(|e| match e {
        ValidationError::CensusUndecidable {
            a: EntityId::Face(a),
            b: EntityId::Face(b),
            what,
        } => {
            what.contains(CURVED_WITHIN_REACH)
                && ((plane(*a) && cylinder(*b)) || (cylinder(*a) && plane(*b)))
                && pair.solid_of_face(*a) != pair.solid_of_face(*b)
        }
        _ => false,
    });
    assert!(cap_on_wall, "a cap against the bore wall refuses: {errs:?}");
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::InstanceInterference { .. })),
        "the rim lies in the tube: {errs:?}"
    );
}

/// **A rod that touches its bore, or misses it by less than the band,
/// still refuses**, with the curved-pair finding and nothing that calls
/// it apart: the gap that clears has to be definite past both faces'
/// pads. A rod flush in the bore, and one a half band inside it, at
/// every ε row.
#[test]
fn a_rod_touching_its_bore_still_refuses() {
    let zero = band().zero();
    for (what, gap) in [("flush", 0.0), ("half a band in", 0.5 * zero)] {
        let pair = nested(
            revolved(0.6, 1.0, 2.0),
            &revolved(0.0, 0.6 - gap, 1.0),
            Vec3::new(0.0, 0.5, 0.0),
        );
        let errs = census(&pair).expect_err(what);
        assert!(
            refuses_curved_within_reach(&pair, &errs),
            "the rod {what}: {errs:?}"
        );
    }
}

/// **A rod crossed by a thinner rod refuses.** A rod of radius 0.2
/// along `y` over `[0.5, 1.5]`, and a rod of radius 0.1 along `x` over
/// `[-1, 1]` through `(·, 1, 0.25)`: the two walls cross through each
/// other, and no edge of either meets the other (the thin rod's seam
/// runs at `z = 0.35`, outside the thick rod). The thin rod's wall's
/// BOUNDARY (its two rims at `x = ±1`) lies ~0.9 from the thick rod's
/// axis, but its rulings pass through the axis. Red where a cylinder
/// face's least depth about a foreign axis is read off its boundary.
#[test]
fn a_rod_crossed_by_a_thinner_rod_refuses() {
    let mut out = topo::transform_rigid(
        &revolved(0.0, 0.2, 1.0),
        &RigidMotion::translation(Vec3::new(0.0, 0.5, 0.0)).unwrap(),
        tol(),
    )
    .unwrap();
    let cross = topo::transform_rigid(
        &revolved(0.0, 0.1, 2.0),
        &RigidMotion::axes_zx(geom_core::Point3::new(-1.0, 1.0, 0.25)),
        tol(),
    )
    .unwrap();
    topo::graft_disjoint(&mut out, &cross).expect("grafts");
    let pair = finished("the crossed rods", out, tol());
    let errs = census(&pair).expect_err("the rods cross");
    assert!(refuses_curved_within_reach(&pair, &errs), "{errs:?}");
}

/// **Far from the origin, a flush rod still refuses** at every ε
/// row: the rod-in-bore pair of `a_rod_touching_its_bore_still_refuses`
/// moved 1e3 and 1e4 along each axis.
#[test]
fn a_flush_rod_far_from_the_origin_still_refuses() {
    for far in [1.0e3, 1.0e4] {
        let pair = nested(
            revolved(0.6, 1.0, 2.0),
            &revolved(0.0, 0.6, 1.0),
            Vec3::new(0.0, 0.5, 0.0),
        );
        let moved = topo::transform_rigid(
            &pair.into_body(),
            &RigidMotion::translation(Vec3::new(far, far, far)).unwrap(),
            tol(),
        )
        .unwrap();
        let moved = finished("the far pair", moved, tol());
        let errs = census(&moved).expect_err("the flush rod must not clear");
        assert!(
            refuses_curved_within_reach(&moved, &errs),
            "the flush rod at {far}: {errs:?}"
        );
    }
}

/// **A rod 1.5 box pads inside its bore refuses** — between one box
/// pad and the depth gap's two, the curved pairs stay undecided.
#[test]
fn a_rod_between_the_pads_refuses() {
    let b = band();
    let gap = 1.5 * topo::test_support::sweep_pad(b);
    let pair = nested(
        revolved(0.6, 1.0, 2.0),
        &revolved(0.0, 0.6 - gap, 1.0),
        Vec3::new(0.0, 0.5, 0.0),
    );
    let errs = census(&pair).expect_err("1.5 pads must not clear");
    assert!(refuses_curved_within_reach(&pair, &errs), "{errs:?}");
}

/// **A skewer through a ball, every vertex of each outside the other,
/// refuses**: a rod of radius 0.2 along `x` over `[-2, 2]` through a
/// unit ball poled on `y`. The rod's rims lie 2 from the ball's centre
/// and the ball's poles 1 from the rod's axis, so no vertex is inside
/// and only the walls' crossing says they meet. Red where a cylinder
/// face's least depth about a foreign centre is read off its boundary.
#[test]
fn a_skewer_through_a_ball_with_every_vertex_outside_refuses() {
    let mut out = sweep::test_support::ball_poled_y(1.0, Vec3::new(0.0, 0.0, 0.0), tol());
    let skewer = topo::transform_rigid(
        &revolved(0.0, 0.2, 4.0),
        &RigidMotion::axes_zx(geom_core::Point3::new(-2.0, 0.0, 0.0)),
        tol(),
    )
    .unwrap();
    topo::graft_disjoint(&mut out, &skewer).expect("grafts");
    let pair = finished("the skewered ball", out, tol());
    let errs = census(&pair).expect_err("the skewer crosses the ball");
    assert!(refuses_curved_within_reach(&pair, &errs), "{errs:?}");
}

/// **A rod off the axis of a plate's hole clears only past both pads**:
/// a rod 0.1 off the axis of a hole of radius 0.5 through an annular
/// plate, standing through it, `gap` from the hole wall at its nearest.
/// Half a band, one and a half pads and exactly two pads refuse on the
/// curved pairs; 0.05 passes. Red where the pads are dropped from the
/// margin.
#[test]
fn a_rod_off_axis_in_a_plate_hole_clears_only_past_the_pads() {
    let b = band();
    let pad = topo::test_support::sweep_pad(b);
    let rod_in_hole = |gap: f64| {
        nested(
            revolved(0.5, 1.5, 0.4),
            &revolved(0.0, 0.4 - gap, 1.4),
            Vec3::new(0.1, -0.5, 0.0),
        )
    };
    for (what, gap) in [
        ("half a band", 0.5 * b.zero()),
        ("one and a half pads", 1.5 * pad),
        ("two pads", 2.0 * pad),
    ] {
        let pair = rod_in_hole(gap);
        let errs = census(&pair).expect_err(what);
        assert!(
            refuses_curved_within_reach(&pair, &errs),
            "{what}: {errs:?}"
        );
    }
    assert_eq!(census(&rod_in_hole(0.05)), Ok(()), "0.05 clears");
}

/// **A rod through an elliptical rim, wider than its minor semi-axis,
/// refuses on the plate's face**: a 2 × 2 × 1 block bored by a cylinder
/// of radius 0.3 tilted 30° through its top's centre leaves an
/// elliptical rim of semi-axes 0.3 and 0.3/cos 30° ≈ 0.346 there, and a
/// coaxial upright rod of radius 0.32 standing through that rim crosses
/// the top face at the minor vertices. The top's least depth about the
/// rim's axis is the minor semi-axis, so no gap parts that pair. Red
/// where an ellipse's ring is read as its major circle alone, which
/// puts that least depth past the rod.
#[test]
fn a_rod_wider_than_an_elliptical_rim_refuses_on_the_plate() {
    use geom_core::{Affine3, Mat3, Point3};
    let block = finished(
        "the block",
        sweep::test_support::brick((0.0, 2.0), (0.0, 2.0), (0.0, 1.0), tol()),
        tol(),
    );
    let upright =
        sweep::test_support::cylinder_of_arcs_at(3, 0.3, Point2::new(1.0, 1.0), -2.0, 5.0, tol());
    let (s, c) = 30f64.to_radians().sin_cos();
    let tilt = Mat3::from_cols(
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, c, s),
        Vec3::new(0.0, -s, c),
    );
    let pivot = Point3::new(1.0, 1.0, 1.0) - Point3::origin();
    let map = Affine3::from_parts(tilt, pivot - tilt * pivot);
    let bore = finished(
        "the tilted bore",
        topo::transform_rigid(
            &upright,
            &geom_core::test_support::rigid(&map, tol()),
            tol(),
        )
        .expect("the cylinder tilts"),
        tol(),
    );
    let topo::BooleanResult::Body(bored) = topo::subtract(&block, &bore, tol()).expect("bores")
    else {
        panic!("a body")
    };
    let mut out = bored.body.into_body();
    let rod =
        sweep::test_support::cylinder_of_arcs_at(3, 0.32, Point2::new(1.0, 1.0), 0.9, 1.0, tol());
    topo::graft_disjoint(&mut out, &rod).expect("grafts");
    let pair = finished("the rod through the rim", out, tol());
    let errs = census(&pair).expect_err("the rod crosses the rim");
    let top = |f| {
        matches!(
            pair.get_face(f).and_then(|d| pair.get_surface(d.surface)),
            Some(geom::Surface::Plane { origin, normal, .. })
                if (origin.z - 1.0).abs() < 1e-9 && normal.x == 0.0 && normal.y == 0.0
        )
    };
    let top_on_rod = errs.iter().any(|e| match e {
        ValidationError::CensusUndecidable {
            a: EntityId::Face(a),
            b: EntityId::Face(b),
            what,
        } => {
            what.contains(CURVED_WITHIN_REACH)
                && (top(*a) || top(*b))
                && pair.solid_of_face(*a) != pair.solid_of_face(*b)
        }
        _ => false,
    });
    assert!(top_on_rod, "the top face against the rod refuses: {errs:?}");
}
