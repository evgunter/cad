//! **A saddle cut by its tangent plane refuses at its side wall's chord.** The
//! block `x, z ∈ [−1, 1]`, `−2 ≤ y ≤ x·z`, lofted along `z`, has the
//! hyperbolic paraboloid `y = x·z` for its top wall; the half-space
//! `y ≥ 0` is tangent to it at the origin and cuts it along the lines
//! `x = 0` and `z = 0`, a straight section of two lines crossing at a
//! point. Two smooth carriers meet in two crossing lines only at a
//! tangency, so the pair is a spline × plane section with a node at the
//! origin. The join carries its four arcs to the node, and each op
//! refuses typed at the at-rest certificate of the plane's chord across
//! the block's side wall `x = −1`: a lofted wall whose chart is not
//! affine along the section `y = 0` (its `u` reads `y` across a side
//! whose far end moves with `z`), which the plane × spline lane's
//! degree-1 image misses between its feet
//! (`work/sectarms/a-planes-carried-arc-across-a-flared-spline-wall-misses-its-own-image-at-rest.md`).
//! No chord to the node is reached. Its closed
//! forms (the block `8`, its part above the plane `1/2`) are the row
//! `work/splice/a-saddle-cut-by-its-tangent-plane-builds-at-its-closed-form.md`
//! asks for.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::test_support::bulge_loop;
use sweep::test_support::{finished, stacked_at};
use topo::{AtRestBody, BooleanError, intersect, subtract, union};

fn tol() -> Tol {
    Tol::witness()
}

/// The saddle block (module docs).
fn saddle_block() -> AtRestBody<f64> {
    let section = |s: f64| {
        let p = |x: f64, y: f64| (Point2::new(x, y), 0.0);
        vec![bulge_loop(vec![
            p(-1.0, -2.0),
            p(1.0, -2.0),
            p(1.0, s),
            p(-1.0, -s),
        ])]
    };
    let lofted = sweep::loft_body(
        &[section(-1.0), section(1.0)],
        &stacked_at(&[-1.0, 1.0]),
        1,
        tol(),
    )
    .expect("the saddle block lofts");
    finished("the saddle block", lofted.body, tol())
}

/// The half-space `y ≥ 0`, as a brick with a face on its plane.
fn above() -> AtRestBody<f64> {
    finished(
        "the half-space",
        topo::test_support::brick::<f64>((-4.0, 4.0), (0.0, 4.0), (-4.0, 4.0), tol()),
        tol(),
    )
}

/// **Every op, either way round, refuses at the side wall's chord at
/// rest**: the join carries the saddle's four arcs to the node, and the
/// edge certificate of the plane's chord along `x = −1` refuses its
/// chart image, limb 2 reading the image's miss of the carrier past the
/// band at a break of its composite (`SsiLimb::ImageMiss`). The
/// refusal names no face.
/// Its held pierce reads first: the block's corner edges pierce the
/// plane at `(±1, 0, ±1)`, a unit off the wall, which reads them off
/// its carrier.
#[test]
fn a_saddle_cut_by_its_tangent_plane_refuses_at_its_node_arcs_under_every_op() {
    let (s, h) = (saddle_block(), above());
    let frame = |r: &Result<topo::BooleanResult<f64>, BooleanError>| {
        let Err(BooleanError::Join(join)) = r else {
            return false;
        };
        matches!(
            join.error(),
            topo::JoinError::Euler(topo::EulerOpError::Certification {
                error: geom_brep::CertifyError::PlaneNurbs(geom_brep::PlaneNurbsRefusal::Limb {
                    limb: geom_brep::SsiLimb::ImageMiss,
                    ..
                }),
            })
        )
    };
    for (op, r) in [
        ("s ∩ h", intersect(&s, &h, tol())),
        ("h ∩ s", intersect(&h, &s, tol())),
        ("s ∖ h", subtract(&s, &h, tol())),
        ("h ∖ s", subtract(&h, &s, tol())),
        ("s ∪ h", union(&s, &h, tol())),
        ("h ∪ s", union(&h, &s, tol())),
    ] {
        assert!(
            frame(&r),
            "{op}: refuses at the side wall's chord at rest, not {r:?}"
        );
    }
}
