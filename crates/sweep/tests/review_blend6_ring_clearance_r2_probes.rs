//! **BLEND-6 (ring clearance) R2 review probes** — the two facts the
//! unit's own rows leave unmeasured, both about the CONTAINMENT
//! relation `CircleMargins::other_inside_trim`.
//!
//! - **Every ring the unit's rows meter is CONCENTRIC with the trim
//!   circle**, so the `‖cj − ci‖` term of both containment margins is
//!   multiplied by nothing there. The first row is the one fixture in
//!   the tree that evaluates the containment form with a NON-coaxial
//!   ring — a spherical pip bitten out of a cylinder's top face — and
//!   carries it through. It does NOT red when that term is deleted
//!   (the margin stays positive without it); nothing can, for the
//!   second bullet's reason, and that is the honest statement of what
//!   this row buys.
//! - **Whether that non-coaxial ring reaches the exact backstop**
//!   (`work/blend/containment-margin-backstop-unreachable-behind-the-screen.md`
//!   names "a pip cut into the top of a pole-touching revolve" as the
//!   construction that would). Measured here: it does NOT. The boolean
//!   door mints the off-axis pip only at azimuth 0 and π — every other
//!   azimuth refuses `Join(SectionLoopMixed)` — and at those two the
//!   ring's closest approach to the rim lands ON a sample of predicate
//!   2's lattice, so the screen is exact and answers first exactly as
//!   it does on a coaxial pair. The second row pins that measurement.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Sign, Tol, Vec3};
use profile::ProfileVertex;
use sweep::Revolution;
use sweep::blend::BlendError;
use sweep::blend::build::fillet_edges;
use sweep::test_support::{revolved_about_y, rim_arcs_at};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanDeclarations, FaceKey, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

/// A cylinder of radius 1 and height 1 about the y axis, repaired: its
/// flat top is ONE plane disc whose whole outer cycle is the top rim.
fn cylinder() -> Body<f64> {
    let mut b = revolved_about_y(
        vec![
            ProfileVertex::new(Point2::new(0.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.0, 1.0), 0.0),
            ProfileVertex::new(Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    b.merge_coplanar_faces(tol())
        .expect("the pole-split caps repair");
    b
}

/// The cylinder with a spherical PIP of radius `pr` bitten out of its
/// top face, its centre IN the plane `y = 1` at distance `dc` from the
/// axis and azimuth 0 — so the pip's rim is a circle of radius `pr`
/// centred `dc` off the trim circle's own centre.
///
/// The ball's poles lie on the axis the top face's normal runs along,
/// which is what the boolean's plane×sphere section asks for
/// (`test_support::ball_poled_z`'s note, one axis over).
fn pipped(dc: f64, pr: f64) -> Body<f64> {
    let ball = revolved_about_y(
        vec![
            ProfileVertex::new(Point2::new(0.0, -pr), 1.0),
            ProfileVertex::new(Point2::new(0.0, pr), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    let ball = topo::transform_rigid(&ball, &Affine3::translation(Vec3::new(dc, 1.0, 0.0)), tol())
        .expect("the pip's rigid motion");
    boolean_op_with(
        BooleanOp::Subtract,
        &cylinder(),
        &ball,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        tol(),
    )
    .expect("the pip subtracts")
    .body()
    .expect("a body")
    .body
    .clone()
}

/// The top face of the pipped cylinder: the plane host whose outer
/// cycle is the top rim and which carries the pip rim as its one RING.
fn ringed_top(body: &Body<f64>) -> FaceKey {
    let arcs = rim_arcs_at(body, 1.0, 1.0);
    assert_eq!(arcs.len(), 2, "the top rim is two arcs");
    let ed = body.get_edge(arcs[0]).unwrap();
    [ed.he_plus, ed.he_minus]
        .into_iter()
        .map(|he| {
            body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
                .unwrap()
                .face
        })
        .find(|&f| !body.get_face(f).unwrap().rings.is_empty())
        .expect("the top face carries the pip ring")
}

/// **A hostless ANNULUS rim whose host ring is NOT coaxial with the
/// trim circle carries through.**
///
/// The host trim is the circle of radius `1 − r` on the axis; the ring
/// is the pip rim, radius `pr`, centred `dc` off that axis. The
/// containment margin is `(1 − r) − (dc + pr)`. Every ring the unit's
/// own rows meter is concentric with its trim, so this is the only
/// fixture in the tree that evaluates the relation with `dc ≠ 0` at
/// all — red under a mutant that meters this ring EXTERNALLY, green
/// under one that drops the centre-distance term (see the module
/// docs).
#[test]
fn a_non_coaxial_ring_carries_through_the_hostless_annulus_trim() {
    let (dc, pr, r) = (0.5, 0.30, 0.1);
    let body = pipped(dc, pr);
    let host = ringed_top(&body);
    assert_eq!(
        body.get_face(host).unwrap().rings.len(),
        1,
        "one ring, the pip rim"
    );
    assert!(
        (1.0 - r) - (dc + pr) > 0.09,
        "the containment margin is definitely positive"
    );
    let arcs = rim_arcs_at(&body, 1.0, 1.0);
    let out = fillet_edges(&body, &arcs, r, tol()).expect("the top rim carves over a pip");
    validate_geometric(&out.body, tol()).expect("tier-3 valid");
    assert_eq!(
        out.body.get_face(host).map_or(0, |fd| fd.rings.len()),
        1,
        "the host keeps its pip ring across the annulus surgery"
    );
}

/// **The off-axis pip does NOT reach the exact containment backstop**,
/// which is the construction
/// `work/blend/containment-margin-backstop-unreachable-behind-the-screen.md`
/// names as the one that would.
///
/// With the ring `0.01` outside the trim circle the request refuses —
/// but under `fillet3_face_clearance`, predicate 2's sampled screen,
/// reading the containment margin exactly, as it does on a coaxial
/// pair. The reason is that the boolean door only mints this pip at
/// azimuth 0 (and π); every other azimuth refuses
/// `Join(SectionLoopMixed)`, and at azimuth 0 the ring's closest point
/// to the rim is a sample of both edges' lattices, so the sampled gap
/// is the true one and the screen is not overestimating anything.
///
/// This row is the item's canary: it flips the day a construction
/// reaches `fillet3_ring_clearance` here.
#[test]
fn an_off_axis_pip_does_not_reach_the_exact_containment_backstop() {
    let (dc, pr, r) = (0.5, 0.41, 0.1);
    let want = (1.0 - r) - (dc + pr);
    assert!(
        want < -0.009,
        "the containment margin is definitely negative"
    );
    let body = pipped(dc, pr);
    let arcs = rim_arcs_at(&body, 1.0, 1.0);
    let err = fillet_edges(&body, &arcs, r, tol())
        .expect_err("a ring outside the trim circle refuses")
        .error;
    let BlendError::FaceClearanceUncertified { margin, .. } = err else {
        panic!("the sampled screen answers first on this pair, got {err:?}");
    };
    assert_eq!(margin.predicate, "fillet3_face_clearance");
    assert_eq!(margin.sign, Sign::Negative);
    let read = margin.value().expect("a definite reading");
    assert!(
        (read - want).abs() <= 1e-15,
        "the screen reads the containment margin (read {read}, derived {want})"
    );
}
