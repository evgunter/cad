//! **A saddle cut by its tangent plane refuses before the join.** The
//! block `x, z ∈ [−1, 1]`, `−2 ≤ y ≤ x·z`, lofted along `z`, has the
//! hyperbolic paraboloid `y = x·z` for its top wall; the half-space
//! `y ≥ 0` is tangent to it at the origin and cuts it along the lines
//! `x = 0` and `z = 0`, a straight section of two lines crossing at a
//! point. Two smooth carriers meet in two crossing lines only at a
//! tangency, so the pair is a spline × plane tangent section, which no
//! lane builds yet: each op refuses typed at the spline × plane pair,
//! intersect and subtract at the revert roster, and union in the join,
//! where no frame names the wall × plane germ pair's section. Its closed
//! forms (the block `8`, its part above the plane `1/2`) are the row
//! `work/splice/a-saddle-cut-by-its-tangent-plane-builds-at-its-closed-form.md`
//! asks for.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::SurfaceKind::{Nurbs, Plane};
use geom_core::{Point2, Tol};
use profile::test_support::bulge_loop;
use sweep::test_support::{finished, stacked_at};
use topo::{AtRestBody, BooleanError, PairRefusalSite, intersect, subtract, union};

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
        topo::test_support::brick((-4.0, 4.0), (0.0, 4.0), (-4.0, 4.0), tol()),
        tol(),
    )
}

/// The saddle's top wall: the NURBS face whose net lies on `y = x·z`.
fn saddle_wall(s: &AtRestBody<f64>) -> topo::FaceKey {
    let walls: Vec<_> = s
        .faces()
        .filter(|(_, fd)| match s.get_surface(fd.surface) {
            Some(geom::Surface::Nurbs(n)) => {
                n.control().iter().all(|p| (p.y - p.x * p.z).abs() < 1e-12)
            }
            _ => false,
        })
        .map(|(f, _)| f)
        .collect();
    assert_eq!(walls.len(), 1, "one saddle wall: {walls:?}");
    walls[0]
}

/// The half-space's face on `y = 0`.
fn cut_face(h: &AtRestBody<f64>) -> topo::FaceKey {
    let faces: Vec<_> = h
        .faces()
        .filter(|(_, fd)| {
            matches!(
                h.get_surface(fd.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if origin.y.abs() < 1e-12 && normal.y.abs() > 1.0 - 1e-12
            )
        })
        .map(|(f, _)| f)
        .collect();
    assert_eq!(faces.len(), 1, "one face on y = 0: {faces:?}");
    faces[0]
}

/// **Every op, either way round, refuses at the spline × plane pair**:
/// intersect and subtract at the revert roster, union in the join at
/// the germ frame of the saddle's wall against the half-space's face
/// `y = 0` (`work/sectarms/a-nurbs-wall-against-a-plane-has-no-germ-frame.md`).
/// Its held pierce reads first: the block's corner edges pierce the
/// plane at `(±1, 0, ±1)`, a unit off the wall, which reads them off
/// its carrier.
#[test]
fn a_saddle_cut_by_its_tangent_plane_refuses_at_the_spline_plane_pair() {
    let (s, h) = (saddle_block(), above());
    let (wall, cut) = (saddle_wall(&s), cut_face(&h));
    let roster = |r: &Result<topo::BooleanResult<f64>, BooleanError>| {
        matches!(
            r,
            Err(BooleanError::CurvedPairUnsupported {
                site: PairRefusalSite::RevertRoster,
                kind: Nurbs,
                other_kind: Plane,
                ..
            })
        )
    };
    // The A face first: the saddle's wall in `s ∪ h`, the cut face in
    // `h ∪ s`.
    let frame = |r: &Result<topo::BooleanResult<f64>, BooleanError>, saddle_first: bool| {
        let (a, b) = if saddle_first {
            (wall, cut)
        } else {
            (cut, wall)
        };
        let (a_kind, b_kind) = if saddle_first {
            (Nurbs, Plane)
        } else {
            (Plane, Nurbs)
        };
        matches!(
            r,
            Err(BooleanError::GermFrameUnsupported { a_face, a_kind: ak, b_face, b_kind: bk })
                if (*a_face, *ak, *b_face, *bk) == (a, a_kind, b, b_kind)
        )
    };
    for (op, r, at_frame) in [
        ("s ∩ h", intersect(&s, &h, tol()), None),
        ("h ∩ s", intersect(&h, &s, tol()), None),
        ("s ∖ h", subtract(&s, &h, tol()), None),
        ("h ∖ s", subtract(&h, &s, tol()), None),
        ("s ∪ h", union(&s, &h, tol()), Some(true)),
        ("h ∪ s", union(&h, &s, tol()), Some(false)),
    ] {
        let refused = match at_frame {
            Some(saddle_first) => frame(&r, saddle_first),
            None => roster(&r),
        };
        assert!(
            refused,
            "{op}: refuses typed at the spline × plane pair, not {r:?}"
        );
    }
}
