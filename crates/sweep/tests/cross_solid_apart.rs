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
