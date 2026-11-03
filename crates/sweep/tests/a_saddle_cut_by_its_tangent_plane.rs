//! **A saddle cut by its tangent plane refuses before the join.** The
//! block `x, z ∈ [−1, 1]`, `−2 ≤ y ≤ x·z`, lofted along `z`, has the
//! hyperbolic paraboloid `y = x·z` for its top wall; the half-space
//! `y ≥ 0` is tangent to it at the origin and cuts it along the lines
//! `x = 0` and `z = 0`, a straight section of two lines crossing at a
//! point. Two smooth carriers meet in two crossing lines only at a
//! tangency, so the pair is a spline × plane tangent section, which no
//! lane builds yet: each op refuses typed at the held pierce of the
//! saddle's wall on the plane, long before the join pairs germs across
//! the crossing. Its closed
//! forms (the block `8`, its part above the plane `1/2`) are the row
//! `work/splice/a-saddle-cut-by-its-tangent-plane-builds-at-its-closed-form.md`
//! asks for.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::SurfaceKind::{Nurbs, Plane};
use geom_core::{Point2, Point3, Tol};
use profile::test_support::bulge_loop;
use sweep::test_support::{finished, stacked_at};
use topo::{AtRestBody, BooleanError, FaceKey, Operand, intersect, subtract, union};

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

/// The saddle's top wall: its NURBS face whose chart's middle is the
/// tangency point, the origin.
fn top_wall(s: &AtRestBody<f64>) -> FaceKey {
    let walls: Vec<FaceKey> = s
        .faces()
        .filter(|(_, fd)| match s.get_surface(fd.surface) {
            Some(geom::Surface::Nurbs(n)) => {
                let (u0, u1) = n.knot_values_u().domain();
                let (v0, v1) = n.knot_values_v().domain();
                n.eval(0.5 * (u0 + u1), 0.5 * (v0 + v1)).distance(Point3::origin()) < 1e-12
            }
            _ => false,
        })
        .map(|(f, _)| f)
        .collect();
    assert_eq!(walls.len(), 1, "one wall holds the origin at its middle");
    walls[0]
}

/// The half-space's face on the plane `y = 0`.
fn cutting_face(h: &AtRestBody<f64>) -> FaceKey {
    let faces: Vec<FaceKey> = h
        .faces()
        .filter(|(_, fd)| {
            matches!(
                h.get_surface(fd.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if normal.y.abs() == 1.0 && origin.y == 0.0
            )
        })
        .map(|(f, _)| f)
        .collect();
    assert_eq!(faces.len(), 1, "one face of the half-space lies on y = 0");
    faces[0]
}

/// **Every op, either way round, refuses at the held pierce of the
/// saddle's top wall on the cutting plane**: the spline × plane pair is
/// read by ∪'s doors under every op, so ∩ and ∖ stop where ∪ does,
/// naming the same two faces by identity, the wall on the saddle's side.
#[test]
fn a_saddle_cut_by_its_tangent_plane_refuses_at_the_held_pierce_under_every_op() {
    let (s, h) = (saddle_block(), above());
    let (wall, cut) = (top_wall(&s), cutting_face(&h));
    for (op, r, saddle) in [
        ("s ∩ h", intersect(&s, &h, tol()), Operand::A),
        ("h ∩ s", intersect(&h, &s, tol()), Operand::B),
        ("s ∖ h", subtract(&s, &h, tol()), Operand::A),
        ("h ∖ s", subtract(&h, &s, tol()), Operand::B),
        ("s ∪ h", union(&s, &h, tol()), Operand::A),
        ("h ∪ s", union(&h, &s, tol()), Operand::B),
    ] {
        let at_pierce = matches!(
            &r,
            Err(BooleanError::HeldPierceUnbuilt {
                operand,
                face,
                pierced,
                kind: Nurbs,
                pierced_kind: Plane,
                placed: false,
            }) if *operand == saddle && *face == wall && *pierced == cut
        );
        assert!(
            at_pierce,
            "{op}: refuses at the held pierce of the top wall {wall:?} on {cut:?}, not {r:?}"
        );
    }
}
