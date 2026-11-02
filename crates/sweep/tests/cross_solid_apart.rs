//! **Curved parts of two solids that no direction separates**, read by
//! the census's depth gap (`topo::census::depth_gap`): nested thin walls
//! a shell leaves, a rod in a bore, a tube in a tube. Each is two solids
//! whose reach boxes nest, so before the depth gap every curved pair
//! refused `CensusUndecidable` "a curved face of one is within reach of
//! the other"
//! (`work/restread/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`).
//! Apart, the empty-record tier 3′ passes; touching, or apart by less
//! than the band, it still refuses with that finding, which the census
//! keeps until it can read a curved touch. The same holds for parts a
//! direction does separate but the census's world axes and anchors do
//! not find: a box beside a rod's wall, read along the offset from the
//! rod's axis, and two lumps of a ball's cut, whose sphere patches reach
//! no further than their rims away from a pole they do not hold.

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
            ..
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
            ..
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
/// and only the walls' crossing says they meet: the material test
/// cannot see it, so the curved pair has to refuse. Red where the depth
/// gap clears the rod's wall against the ball. (Reading the wall's
/// least depth off its boundary alone does not: the seam chord passes
/// 0.2 from the centre.)
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
            ..
        } => {
            what.contains(CURVED_WITHIN_REACH)
                && (top(*a) || top(*b))
                && pair.solid_of_face(*a) != pair.solid_of_face(*b)
        }
        _ => false,
    });
    assert!(top_on_rod, "the top face against the rod refuses: {errs:?}");
}

/// **A ball flush in a bore refuses; one clear of it passes.** A ball of
/// radius 0.6 centred on the axis of the bore `r = 0.6` touches its wall
/// along the equator, and its poles lie in the bore, outside the tube's
/// material, so the material test cannot see the touch: only the curved
/// pair can refuse it. A ball of radius 0.5 is clear by 0.1 and passes.
/// Red where a sphere face's cover holds less than the ball.
#[test]
fn a_ball_flush_in_a_bore_refuses() {
    let in_bore = |r: f64| {
        let mut out = revolved(0.6, 1.0, 2.0);
        let ball = sweep::test_support::ball_poled_y(r, Vec3::new(0.0, 1.0, 0.0), tol());
        topo::graft_disjoint(&mut out, &ball).expect("grafts");
        finished("the ball in the bore", out, tol())
    };
    let flush = in_bore(0.6);
    let errs = census(&flush).expect_err("the flush ball touches the bore");
    assert!(refuses_curved_within_reach(&flush, &errs), "{errs:?}");
    assert_eq!(census(&in_bore(0.5)), Ok(()), "a ball 0.1 clear");
}

/// **A ring torus flush in a bore refuses; one clear of it passes.** A
/// torus of radii 0.35 and 0.25 about the axis of the bore `r = 0.6`
/// touches its wall along its outer equator; its profile's vertices sit
/// at the top and bottom of the tube, off the wall and outside the
/// tube's material, so only the curved pair can refuse it. Radii 0.3
/// and 0.2 leave 0.1 and pass. Red where a torus face's cover holds less
/// than the tube.
#[test]
fn a_torus_flush_in_a_bore_refuses() {
    let in_bore = |major: f64, minor: f64| {
        let mut out = revolved(0.6, 1.0, 2.0);
        let torus = sweep::test_support::revolved_about_y(
            vec![
                (Point2::new(major, 1.0 - minor), 1.0),
                (Point2::new(major, 1.0 + minor), 1.0),
            ],
            sweep::Revolution::Full,
            tol(),
        );
        topo::graft_disjoint(&mut out, &torus).expect("grafts");
        finished("the torus in the bore", out, tol())
    };
    let flush = in_bore(0.35, 0.25);
    let errs = census(&flush).expect_err("the flush torus touches the bore");
    assert!(refuses_curved_within_reach(&flush, &errs), "{errs:?}");
    assert_eq!(census(&in_bore(0.3, 0.2)), Ok(()), "a torus 0.1 clear");
}

/// **A box off a rod's side passes until it touches.** The rod `r ≤ 0.5`
/// about `y` over `y ∈ [0, 1]`, and the unit box over the same `y` whose
/// edge nearest the axis runs along `x = z = s`, `s = (0.5 + gap)/√2`:
/// that edge is `gap` from the wall, and the box's bottom and top lie in
/// the rod's cap planes, `gap` from the caps' rims. No world axis, and
/// not the axis between the faces' anchors, parts the box from the
/// wall or a cap at these gaps; the offset from the rod's axis to the
/// box's nearest edge does ([`topo`]'s `depth_gap::toward`). At gap 0
/// the edge lies along a ruling of the wall, and the pair refuses. Red
/// where the census proposes no direction from a face's axis.
#[test]
fn a_box_off_a_rod_passes_until_it_touches() {
    let beside = |gap: f64| {
        let s = (0.5 + gap) * core::f64::consts::FRAC_1_SQRT_2;
        let mut out = revolved(0.0, 0.5, 1.0);
        let block = sweep::test_support::brick((s, s + 1.0), (0.0, 1.0), (s, s + 1.0), tol());
        topo::graft_disjoint(&mut out, &block).expect("grafts");
        finished("the box beside the rod", out, tol())
    };
    for gap in [0.04, 1e-3] {
        assert_eq!(census(&beside(gap)), Ok(()), "a box {gap} clear");
    }
    let touching = beside(0.0);
    let errs = census(&touching).expect_err("the box's edge lies on the wall");
    assert!(refuses_curved_within_reach(&touching, &errs), "{errs:?}");
}

/// **A bar cut through a ball leaves two lumps the census parts.** The
/// bar `[−2, 2] × [−0.2, 0.25] × [0.1, 0.4]` less the unit ball is a lump
/// either side of the ball, each holding a patch of the sphere whose
/// carrier's box is the whole ball's. Along `x`, each patch reaches no
/// further than its own boundary, since neither holds the sphere's pole
/// there, so the two lumps part along it. Red where a sphere face is
/// read only by its carrier's box.
#[test]
fn a_bar_through_a_ball_leaves_two_lumps_the_census_parts() {
    let ball = finished(
        "the ball",
        sweep::test_support::ball_poled_y(1.0, Vec3::new(0.0, 0.0, 0.0), tol()),
        tol(),
    );
    let bar = finished(
        "the bar",
        sweep::test_support::brick((-2.0, 2.0), (-0.2, 0.25), (0.1, 0.4), tol()),
        tol(),
    );
    let Ok(topo::BooleanResult::Body(lumps)) = topo::subtract(&bar, &ball, tol()) else {
        panic!("bar ∖ ball builds");
    };
    assert_eq!(lumps.body.solids().count(), 2, "two lumps");
    assert_eq!(census(&lumps.body), Ok(()), "the two lumps");
}

/// **A slab through a ball's side refuses.** The unit ball poled along
/// `z`, its one face bounded by the seam meridian in `y = 0`, and the
/// slab `[−2, 2] × [0.9, 0.95] × [−2, 2]` across its side: the slab cuts
/// the ball, yet every vertex of each lies outside the other (the slab's
/// corners outside the ball, the ball's at its poles `(0, 0, ±1)`), so
/// only the curved pair can refuse it. The face's boundary reaches no
/// further along `y` than `y = 0`, but the face holds the sphere's pole
/// `(0, 1, 0)` there, so its reach along `y` is the sphere's. Red where
/// a sphere face's reach is narrowed to its boundary at an end whose pole
/// it holds.
#[test]
fn a_slab_through_a_balls_side_refuses() {
    let mut out = sweep::test_support::ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol());
    let slab = sweep::test_support::brick((-2.0, 2.0), (0.9, 0.95), (-2.0, 2.0), tol());
    topo::graft_disjoint(&mut out, &slab).expect("grafts");
    let crossed = finished("the slab through the ball", out, tol());
    let errs = census(&crossed).expect_err("the slab cuts the ball");
    assert!(refuses_curved_within_reach(&crossed, &errs), "{errs:?}");
}

/// **A trimmed sphere face holding its pole clears only past it.** The
/// unit ball poled along `y`, spun about `y` by 1 and by π/2, less the
/// slab `y > 0.1`: one sphere face bounded by the circle `y = 0.1`,
/// which holds the ball's `±x` poles, and a box whose face `x = 1 + dx`
/// faces the `+x` pole. The pair is apart only at `dx > 0`, so it clears
/// there and refuses at `dx ≤ 0`, where the box reaches the pole. At
/// these spins a direction arm 1 reads has one of its poles outside
/// the face and the other inside. Red where the face is narrowed to its
/// boundary at an end whose pole it holds: at every end, or with the
/// two ends' poles swapped.
#[test]
fn a_trimmed_sphere_face_holding_its_pole_clears_only_past_it() {
    use geom_core::{Affine3, Point3};
    let knife = finished(
        "the slab",
        sweep::test_support::brick((-2.0, 2.0), (0.1, 2.0), (-2.0, 2.0), tol()),
        tol(),
    );
    for spin in [1.0, core::f64::consts::FRAC_PI_2] {
        let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), spin);
        let ball = topo::transform_rigid(
            &sweep::test_support::ball_poled_y(1.0, Vec3::new(0.0, 0.0, 0.0), tol()),
            &geom_core::test_support::rigid(&turn, tol()),
            tol(),
        )
        .unwrap();
        let ball = finished("the ball", ball, tol());
        let Ok(topo::BooleanResult::Body(patch)) = topo::subtract(&ball, &knife, tol()) else {
            panic!("the ball less the slab builds at spin {spin}");
        };
        let patch = patch.body.into_body();
        for dx in [1e-3, 0.0, -1e-3] {
            let mut out = patch.clone();
            let x0 = 1.0 + dx;
            let block = sweep::test_support::brick((x0, x0 + 1.0), (-0.5, 0.5), (-0.5, 0.5), tol());
            topo::graft_disjoint(&mut out, &block).expect("grafts");
            let pair = finished("the box by the pole", out, tol());
            let what = format!("spin {spin}, the box {dx} past the pole");
            if dx > 0.0 {
                assert_eq!(census(&pair), Ok(()), "{what}");
            } else {
                let errs = census(&pair).expect_err(&what);
                assert!(
                    refuses_curved_within_reach(&pair, &errs),
                    "{what}: {errs:?}"
                );
            }
        }
    }
}

/// An upright prism over `[z0, z0 + h]` on the bulge loop `shape`,
/// turned by `phi` about `z` and moved by `by` in plan.
fn prism(shape: &[((f64, f64), f64)], phi: f64, by: (f64, f64), z0: f64, h: f64) -> Body<f64> {
    let (sin, cos) = phi.sin_cos();
    let lp = profile::test_support::bulge_loop(
        shape
            .iter()
            .map(|&((x, y), b)| {
                (
                    Point2::new(cos * x - sin * y + by.0, sin * x + cos * y + by.1),
                    b,
                )
            })
            .collect(),
    );
    let plane = profile::SketchPlane::xy_at(geom_core::Point3::new(0.0, 0.0, z0));
    let p = profile::Profile::new(plane, vec![lp])
        .validate(tol())
        .unwrap();
    let along = sweep::Extrusion::Distance {
        depth: h,
        side: sweep::ExtrudeSide::Along,
    };
    sweep::extrude(&p, along, tol()).unwrap().body
}

/// **A sliver off a block's corner parts along its own flat until it
/// touches.** The block `[−1, 1]² × [0, 1]`, and the rod's segment past
/// its chord at `x = −0.45`, turned 4 rad and moved by `(0.75, 0.75)`:
/// the sliver stands off the corner `(1, 1)` with its flat facing it,
/// `0.45 − (0.25, 0.25)·u` clear along the flat's normal `u`. Its arc
/// wall's axis lies inside the block, so the offset from it aims across
/// the block; the arc's chord normal, which is the flat's, parts the
/// wall from the block's faces. Moved in along `u`, it clears 1e-3
/// short of the corner's edge and refuses on it and past it. Red where
/// the census proposes no direction from an arc's chord.
#[test]
fn a_sliver_off_a_blocks_corner_parts_along_its_flat_until_it_touches() {
    let (sin, cos) = 4.0f64.sin_cos();
    let u = (-cos, -sin);
    let clear = 0.45 - 0.25 * (u.0 + u.1);
    let c = sweep::test_support::rod_chord_at(-0.45);
    let sliver = [((-0.45, c.half), c.wall_bulge), ((-0.45, -c.half), 0.0)];
    let off = |s: f64| {
        let mut out = sweep::test_support::brick((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.0), tol());
        let by = (0.75 + s * u.0, 0.75 + s * u.1);
        topo::graft_disjoint(&mut out, &prism(&sliver, 4.0, by, 0.0, 1.0)).expect("grafts");
        finished("the sliver off the corner", out, tol())
    };
    for s in [0.0, 1e-3 - clear] {
        assert_eq!(census(&off(s)), Ok(()), "the sliver {} clear", clear + s);
    }
    for (s, what) in [(-clear, "on the corner's edge"), (-clear - 1e-3, "past it")] {
        let pair = off(s);
        let errs = census(&pair).expect_err(what);
        assert!(
            refuses_curved_within_reach(&pair, &errs),
            "{what}: {errs:?}"
        );
    }
}

/// **A lens through a block's corner leaves two pieces the census
/// parts.** The block `[−1, 1]² × [0, 1]` less the lens on the tips
/// `(0, ±0.4)` (bulge 0.3 each way), turned by φ and moved by
/// `(0.75, 0.75)`, through the block: the corner is cut off, and the two
/// pieces' walls are arcs of the lens's two circles, cut short by the
/// block's sides. No face either piece has, and neither wall's axis
/// toward the other's rulings, parts them; the line between the two
/// circles' centres does, and so does either arc's chord. Red where the
/// census proposes neither.
#[test]
fn a_lens_through_a_blocks_corner_leaves_two_pieces_the_census_parts() {
    let block = finished(
        "the block",
        sweep::test_support::brick((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.0), tol()),
        tol(),
    );
    let lens = [((0.0, -0.4), 0.3), ((0.0, 0.4), 0.3)];
    for phi in [0.7, 4.0] {
        let cutter = finished(
            "the lens",
            prism(&lens, phi, (0.75, 0.75), -0.5, 2.0),
            tol(),
        );
        let Ok(topo::BooleanResult::Body(pieces)) = topo::subtract(&block, &cutter, tol()) else {
            panic!("block ∖ lens builds at φ {phi}");
        };
        assert_eq!(pieces.body.solids().count(), 2, "two pieces at φ {phi}");
        assert_eq!(census(&pieces.body), Ok(()), "the two pieces at φ {phi}");
    }
}

/// **Two parallel rods part along the line between their axes until
/// they touch.** The rod `r ≤ 0.5` about `y` over `y ∈ [0, 1]`, and a
/// rod of radius 0.05 beside it on the diagonal of `x` and `z`, its
/// axis `0.55 + gap` from the first's. The walls' nearest points lie
/// off both walls' seam rulings, so the offset from either axis to the
/// other wall's segments aims past them, and neither wall has a least
/// depth about the other's axis; the line between the axes parts them
/// by the gap. At gap 0 the walls meet along a ruling, and inside it
/// they cross: both refuse. Red where the census proposes no direction
/// between two carriers' axes.
#[test]
fn two_parallel_rods_part_along_the_line_of_centres_until_they_touch() {
    let beside = |gap: f64| {
        let s = (0.55 + gap) * core::f64::consts::FRAC_1_SQRT_2;
        let mut out = revolved(0.0, 0.5, 1.0);
        let pin = topo::transform_rigid(
            &revolved(0.0, 0.05, 1.0),
            &RigidMotion::translation(Vec3::new(s, 0.0, s)).unwrap(),
            tol(),
        )
        .unwrap();
        topo::graft_disjoint(&mut out, &pin).expect("grafts");
        finished("the rods side by side", out, tol())
    };
    for gap in [0.04, 5e-4] {
        assert_eq!(census(&beside(gap)), Ok(()), "the rods {gap} clear");
    }
    for gap in [0.0, -1e-3] {
        let pair = beside(gap);
        let errs = census(&pair).expect_err("the walls meet");
        assert!(
            refuses_curved_within_reach(&pair, &errs),
            "gap {gap}: {errs:?}"
        );
    }
}

/// **A half-disc off a block's corner parts along its diameter until it
/// touches.** The rod's half past `x = 0`, turned 4 rad, its flat
/// facing the block's corner `(1, 1)` `clear` away along the flat's
/// normal, as the sliver above. The arc's chord is a diameter, through
/// the arc's centre, so only the chord's own normal in the arc's plane
/// parts the wall from the block. Red where the chord direction is read
/// from the centre to the chord's midpoint.
#[test]
fn a_half_disc_off_a_blocks_corner_parts_along_its_diameter_until_it_touches() {
    let (sin, cos) = 4.0f64.sin_cos();
    let u = (-cos, -sin);
    let k = u.0 + u.1;
    let c = sweep::test_support::rod_chord_at(0.0);
    let half = [((0.0, c.half), c.wall_bulge), ((0.0, -c.half), 0.0)];
    let off = |clear: f64| {
        // The flat's u-level is by·u; the corner's is k.
        let s = k + clear - 0.75 * k;
        let by = (0.75 + s * u.0, 0.75 + s * u.1);
        let mut out = sweep::test_support::brick((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.0), tol());
        topo::graft_disjoint(&mut out, &prism(&half, 4.0, by, 0.0, 1.0)).expect("grafts");
        finished("the half-disc off the corner", out, tol())
    };
    for clear in [0.1, 1e-3] {
        assert_eq!(census(&off(clear)), Ok(()), "the half-disc {clear} clear");
    }
    for clear in [0.0, -1e-3] {
        let pair = off(clear);
        let errs = census(&pair).expect_err("the half-disc meets the corner");
        assert!(
            refuses_curved_within_reach(&pair, &errs),
            "{clear}: {errs:?}"
        );
    }
}

/// An oblique turn about the origin, so no world axis parts a pair.
fn tilted(body: &Body<f64>) -> Body<f64> {
    use geom_core::UnitVec3;
    let rx = RigidMotion::rotation(UnitVec3::exact_x(), 0.4).unwrap();
    let rz = RigidMotion::rotation(UnitVec3::exact_z(), 0.7).unwrap();
    topo::transform_rigid(body, &rz.compose(rx).unwrap(), tol()).unwrap()
}

/// `a` and `b` turned obliquely together, two solids of one body.
fn tilted_pair(a: &Body<f64>, b: &Body<f64>) -> AtRestBody<f64> {
    let mut out = tilted(a);
    topo::graft_disjoint(&mut out, &tilted(b)).expect("grafts");
    finished("the turned pair", out, tol())
}

/// **Two parallel rods whose origins are staggered along their axes
/// part along the line of centres until they touch.** The rods of the
/// row above, the thin one's base 0.37 up the thick one's axis, the
/// pair turned obliquely. The offset between the axes is the origins'
/// offset projected off the axis; unprojected, it tilts by the stagger
/// and no longer parts the walls. Red where the offset off an axis is
/// not projected off it.
#[test]
fn staggered_parallel_rods_part_along_the_line_of_centres_until_they_touch() {
    let beside = |gap: f64| {
        let s = (0.55 + gap) * core::f64::consts::FRAC_1_SQRT_2;
        let pin = topo::transform_rigid(
            &revolved(0.0, 0.05, 1.0),
            &RigidMotion::translation(Vec3::new(s, 0.37, s)).unwrap(),
            tol(),
        )
        .unwrap();
        tilted_pair(&revolved(0.0, 0.5, 1.0), &pin)
    };
    for gap in [0.04, 5e-4] {
        assert_eq!(census(&beside(gap)), Ok(()), "the rods {gap} clear");
    }
    let touching = beside(0.0);
    let errs = census(&touching).expect_err("the walls meet along a ruling");
    assert!(refuses_curved_within_reach(&touching, &errs), "{errs:?}");
}

/// The L plate `[0, 6] × [0, 3] ∪ [0, 3] × [0, 6]` over `z ∈ [0, 1]`,
/// every corner rounded at 0.5, the reflex one's centre at `(3.5, 3.5)`.
fn rounded_ell() -> Body<f64> {
    use profile::{Open, ProfileLoop, Start};
    let t = tol();
    let mut path = Open.at(Point2::new(3.0, 0.0)).toward(1.0, 0.0, t).unwrap();
    for (corner, (dx, dy)) in [
        (Point2::new(6.0, 1.5), (0.0, 1.0)),
        (Point2::new(4.5, 3.0), (-1.0, 0.0)),
        (Point2::new(3.0, 4.5), (0.0, 1.0)),
        (Point2::new(1.5, 6.0), (-1.0, 0.0)),
        (Point2::new(0.0, 3.0), (0.0, -1.0)),
    ] {
        path = path
            .fillet(0.5, t)
            .unwrap()
            .at(corner, t)
            .unwrap()
            .toward(dx, dy, t)
            .unwrap();
    }
    let outline: ProfileLoop<f64> = path.fillet(0.5, t).unwrap().to(Start, t).unwrap().into();
    sweep::test_support::extruded(sweep::test_support::sketch_at(0.0), vec![outline], 1.0, t)
}

/// The rounded L with the block `x × y × z`, two solids of one body.
fn ell_and_block(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    let mut out = rounded_ell();
    topo::graft_disjoint(&mut out, &sweep::test_support::brick(x, y, z, tol())).expect("grafts");
    finished("the plate and the block", out, tol())
}

/// Whether `errs` holds the curved-pair refusal on a planar face of the
/// plate (the solid with a cylinder face) whose normal `on_plate` holds
/// of, against a planar face of the block whose normal `on_block` holds
/// of.
fn refuses_on(
    body: &AtRestBody<f64>,
    errs: &[ValidationError],
    on_plate: fn(Vec3<f64>) -> bool,
    on_block: fn(Vec3<f64>) -> bool,
) -> bool {
    refusals_on(body, errs, on_plate, on_block) > 0
}

/// How many of [`refuses_on`]'s refusals `errs` holds.
fn refusals_on(
    body: &AtRestBody<f64>,
    errs: &[ValidationError],
    on_plate: fn(Vec3<f64>) -> bool,
    on_block: fn(Vec3<f64>) -> bool,
) -> usize {
    let normal = |f| match body.get_face(f).and_then(|d| body.get_surface(d.surface)) {
        Some(geom::Surface::Plane { normal, .. }) => Some(*normal),
        _ => None,
    };
    let plate = body
        .faces()
        .find(|(_, d)| {
            matches!(
                body.get_surface(d.surface),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .and_then(|(f, _)| body.solid_of_face(f))
        .expect("the plate has a rounded wall");
    errs.iter()
        .filter(|e| match e {
            ValidationError::CensusUndecidable {
                a: EntityId::Face(a),
                b: EntityId::Face(b),
                what,
                ..
            } if what.contains(CURVED_WITHIN_REACH) => {
                [(*a, *b), (*b, *a)].into_iter().any(|(p, q)| {
                    body.solid_of_face(p) == Some(plate)
                        && body.solid_of_face(q).is_some_and(|s| s != plate)
                        && normal(p).is_some_and(on_plate)
                        && normal(q).is_some_and(on_block)
                })
            }
            _ => false,
        })
        .count()
}

/// A normal along `z`: the plate's top, the block's base or top.
fn level(n: Vec3<f64>) -> bool {
    n.x.abs() < 1e-12 && n.y.abs() < 1e-12
}

/// A normal across `z`: a wall.
fn upright(n: Vec3<f64>) -> bool {
    n.z.abs() < 1e-12
}

/// **A block in a plate's reflex notch passes, until it touches.** The
/// block stands on the L's top plane in the notch, 0.5 or less clear of
/// the L in plan: its base lies in the top's plane and its walls stand
/// on it, and the L wraps two sides of it, so no plane parts the top
/// from the base or from a wall, and the block's corner lies on the
/// notch fillet's axis, so no depth does. Each is apart where it meets
/// the top's plane (`census::in_plane`). Moved onto the fillet (its
/// corner on the arc) or onto the L's straight edge, it touches, and
/// the base and a wall refuse with the curved-pair finding. Red where
/// arm 1 has no reading in the plane.
#[test]
fn a_block_in_a_plates_notch_passes_until_it_touches() {
    let m = 0.5 * (1.0 - core::f64::consts::FRAC_1_SQRT_2);
    for (x, y) in [
        ((3.5, 6.0), (3.5, 6.0)),
        ((3.5, 5.5), (3.5, 5.5)),
        ((3.0 + m, 6.0), (3.5, 6.0)),
        ((3.5, 6.0), (3.0 + m, 3.5)),
    ] {
        let pair = ell_and_block(x, y, (1.0, 2.0));
        assert_eq!(census(&pair), Ok(()), "the block {x:?} × {y:?}");
    }
    for (x, y, what) in [
        ((3.0 + m, 6.0), (3.0 + m, 6.0), "its corner on the fillet"),
        ((3.5, 5.5), (3.0, 4.0), "its edge on the plate's"),
    ] {
        let pair = ell_and_block(x, y, (1.0, 2.0));
        let errs = census(&pair).expect_err(what);
        assert!(
            refuses_on(&pair, &errs, level, level),
            "{what}, the base: {errs:?}"
        );
        assert!(
            refuses_on(&pair, &errs, level, upright),
            "{what}, a wall: {errs:?}"
        );
    }
}

/// **A block across a plate's arm, on it, over it or sunk through it
/// still refuses against the plate's top.** Across the arm, every vertex
/// of the block's base and walls lies outside the top, but their edges
/// cross the top's; on the arm, the base lies inside the top and the
/// walls stand in it, no edge near the top's; over the whole plate, the
/// top lies inside the base; sunk through the arm, the walls cross the
/// top's plane with no vertex on it. Red where the reading in the plane
/// drops its edge gaps, a placement either way, or the side a face lies
/// on.
#[test]
fn a_block_over_a_plates_top_still_refuses() {
    let base_and_wall = [
        ((4.0, 5.0), (-1.0, 4.0), "across the arm"),
        ((0.5, 2.5), (3.5, 5.5), "on the arm"),
    ];
    for (x, y, what) in base_and_wall {
        let pair = ell_and_block(x, y, (1.0, 2.0));
        let errs = census(&pair).expect_err(what);
        assert!(
            refuses_on(&pair, &errs, level, level),
            "{what}, the base: {errs:?}"
        );
        assert!(
            refuses_on(&pair, &errs, level, upright),
            "{what}, a wall: {errs:?}"
        );
    }
    let over = ell_and_block((-1.0, 7.0), (-1.0, 7.0), (1.0, 2.0));
    let errs = census(&over).expect_err("over the plate");
    assert!(
        refuses_on(&over, &errs, level, level),
        "over the plate, the base: {errs:?}"
    );
    let sunk = ell_and_block((1.0, 2.0), (1.0, 2.0), (0.5, 1.5));
    let errs = census(&sunk).expect_err("through the arm");
    assert!(
        refuses_on(&sunk, &errs, level, upright),
        "through the arm, a wall: {errs:?}"
    );
}

/// A normal within 1e-6 of `+z`: a top.
fn up(n: Vec3<f64>) -> bool {
    n.z > 0.999_999
}

/// A normal within 1e-6 of `−z`: a base, level or nearly.
fn down(n: Vec3<f64>) -> bool {
    n.z < -0.999_999
}

/// The plate on the bulge loop `pts` over `z ∈ [0, 1]`, and `other`:
/// two solids of one body.
fn plate_and(pts: Vec<(Point2<f64>, f64)>, other: &Body<f64>) -> AtRestBody<f64> {
    let mut out = sweep::test_support::prism_at(pts, 0.0, 1.0, tol());
    topo::graft_disjoint(&mut out, other).expect("grafts");
    finished("the plate and the other", out, tol())
}

/// **A base tilted within the band across a plate's top refuses.** The
/// plate is the island `[0.02, 0.07] × [0.9, 1.1]`, one side an arc, and
/// the block's base, turned 2.09e-8 rad about `y`, lies 0.9e-9 below the
/// top's plane at `x = 0` (decided on it) and 2e-8 above at `x = 1`
/// (decided off it): it crosses the plane at `x ≈ 0.043`, through the
/// top, far in plan from its feet. Red where the reading of a face
/// standing on the plane takes its feet's Zero for nought: so shallow a
/// face's section lies wherever the band allows, and its slope is not
/// one the reading admits.
#[test]
fn a_base_tilted_within_the_band_across_a_plates_top_refuses() {
    use sweep::test_support::{corners, prism_on, sketch_from_axes};
    let t = tol();
    let mut island = corners::<f64>(&[(0.02, 0.9), (0.07, 0.9), (0.07, 1.1), (0.02, 1.1)]);
    island[1].1 = 0.2;
    let (delta, high) = (0.9e-9, 2e-8);
    let plane = sketch_from_axes(
        geom_core::Point3::new(0.0, 1.5, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        t,
    );
    let block = prism_on(
        plane,
        corners::<f64>(&[
            (0.0, 1.0 - delta),
            (1.0, 1.0 + high),
            (1.0, 2.0),
            (0.0, 2.0),
        ]),
        1.0,
        t,
    );
    let pair = plate_and(island, &block);
    let errs = census(&pair).expect_err("the base crosses the top");
    assert!(
        refuses_on(&pair, &errs, up, down),
        "the top and the base: {errs:?}"
    );
}

/// **A wedge footed a hair off a plate's edge, its underside over the
/// top, refuses**: the rounded square `[0, 1]²` (corners 0.25) over
/// `z ∈ [0, 1]`, and a wedge whose lowest edge runs at `x = 1 + 50z`
/// (`z` the band's zero threshold) within the band of the top's plane
/// (0.5z and 5z above, 0.9z below), its underside rising over the top
/// at slope 0.01 or 0.1, so within the band of the plane over the top's
/// edge. Red where the
/// standing reading takes the foot for exactly on the plane.
#[test]
fn a_wedge_footed_beside_a_plates_edge_over_its_top_refuses() {
    use profile::{Open, ProfileLoop, Start};
    use sweep::test_support::{corners, extruded, prism_on, sketch_at, sketch_from_axes};
    let t = tol();
    let mut path = Open.at(Point2::new(0.5, 0.0)).toward(1.0, 0.0, t).unwrap();
    for (corner, (dx, dy)) in [
        (Point2::new(1.0, 0.5), (0.0, 1.0)),
        (Point2::new(0.5, 1.0), (-1.0, 0.0)),
        (Point2::new(0.0, 0.5), (0.0, -1.0)),
    ] {
        path = path
            .fillet(0.25, t)
            .unwrap()
            .at(corner, t)
            .unwrap()
            .toward(dx, dy, t)
            .unwrap();
    }
    let outline: ProfileLoop<f64> = path.fillet(0.25, t).unwrap().to(Start, t).unwrap().into();
    let z = band().zero();
    for (slope, lift) in [(0.01, 0.5 * z), (0.1, 5.0 * z), (0.01, -0.9 * z)] {
        let plane = sketch_from_axes(
            geom_core::Point3::new(0.0, 1.5, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            t,
        );
        let (x0, xl) = (1.0 + 50.0 * z, -0.5);
        let wedge = prism_on(
            plane,
            corners::<f64>(&[
                (x0, 1.0 + lift),
                (xl, 1.6),
                (xl, 1.0 + lift + slope * (x0 - xl)),
            ]),
            2.0,
            t,
        );
        let mut out = extruded(sketch_at(0.0), vec![outline.clone()], 1.0, t);
        topo::graft_disjoint(&mut out, &wedge).expect("grafts");
        let pair = finished("the plate and the wedge", out, t);
        let errs = census(&pair).expect_err("the wedge lies on the top's plane");
        assert!(
            refuses_on(&pair, &errs, up, |n| n.z < -0.99),
            "slope {slope}, lift {lift}: the top and the underside: {errs:?}"
        );
    }
}

/// **A wall across a plate's rounded corner refuses, though every vertex
/// of the top lies on one side of it.** The plate `[0, 2]²` with its
/// `(2, 2)` corner rounded at 0.5 has every vertex of its top at
/// `x + y ≤ 3.5`; a diamond prism over `z ∈ [0.5, 1.5]` has a wall on
/// `x + y = 3.6`, which the corner's arc crosses (it reaches
/// `x + y ≈ 3.71`). Red where the standing reading reads an arc by its
/// ends.
#[test]
fn a_wall_across_a_plates_rounded_corner_refuses() {
    use sweep::test_support::{corners, prism_at};
    let mut plate = corners::<f64>(&[(0.0, 0.0), (2.0, 0.0), (2.0, 1.5), (1.5, 2.0), (0.0, 2.0)]);
    plate[2].1 = (core::f64::consts::PI / 8.0).tan();
    let diamond = prism_at(
        corners::<f64>(&[(2.1, 1.5), (2.6, 2.0), (2.0, 2.6), (1.5, 2.1)]),
        0.5,
        1.0,
        tol(),
    );
    let pair = plate_and(plate, &diamond);
    let errs = census(&pair).expect_err("the diamond crosses the corner");
    assert!(
        refuses_on(&pair, &errs, up, |n| n.z.abs() < 1e-9
            && (n.x - n.y).abs() < 1e-9),
        "the top and the wall: {errs:?}"
    );
}

/// **A cap on another plane across a plate's top refuses, whichever
/// loop vertex either face starts at.** The pentagon top (z = 1, one
/// side a shallow arc) holds `y = 5` over `x ∈ [0, 10]` with a vertex at
/// `(10, 5)`; the chevron prism's caps lie on planes through the line
/// `y = 5, z = 1` at 45°, one with a vertex at `(−1, 5, 1)`, and cross
/// the top along `x ∈ [0, 5]`. Their boundaries miss each other and
/// each face's starting vertex lies outside the other, so the region
/// reading would clear them were it let onto two planes. Every rotation
/// of either loop's start, both caps.
#[test]
fn a_cap_on_another_plane_across_a_plates_top_refuses() {
    use sweep::test_support::{prism_at, prism_on, sketch_from_axes};
    let t = tol();
    for ra in 0..5 {
        for rb in 0..5 {
            let mut ring = [
                (0.0, 2.0, 0.0),
                (6.0, 0.0, 0.0),
                (10.0, 5.0, 0.0),
                (6.0, 10.0, 0.05),
                (0.0, 8.0, 0.0),
            ];
            ring.rotate_left(ra);
            let pts = ring
                .iter()
                .map(|&(x, y, b)| (Point2::new(x, y), b))
                .collect();
            let mut chevron = [
                (0.0, 0.0),
                (-2.0, -3.0),
                (5.5, -0.2),
                (5.5, 0.2),
                (-2.0, 3.0),
            ];
            chevron.rotate_left(rb);
            let plane = sketch_from_axes(
                geom_core::Point3::new(-1.0, 5.0, 1.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 1.0),
                t,
            );
            let block = prism_on(
                plane,
                chevron
                    .iter()
                    .map(|&(x, y)| (Point2::new(x, y), 0.0))
                    .collect(),
                0.5,
                t,
            );
            let mut out = prism_at(pts, 0.0, 1.0, t);
            topo::graft_disjoint(&mut out, &block).expect("grafts");
            let pair = finished("the pentagon and the chevron", out, t);
            let errs = census(&pair).expect_err("the caps cross the top");
            assert_eq!(
                refusals_on(&pair, &errs, up, |n| n.x.abs() < 1e-9
                    && (n.y + n.z).abs() < 1e-9),
                2,
                "starts {ra}, {rb}: both caps against the top: {errs:?}"
            );
        }
    }
}
