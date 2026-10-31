//! **Whether a loft at `Interval` encloses a member of its family**:
//! the box a row lofts over, and the check of the enclosing body
//! against one member's `f64` loft, wall by wall and vertex by vertex.
//! A check of a body, so beside [`super::orient`] and [`super::census`].
//!
//! **Deliberately not absorbed**: [`super::interval`]'s literals, which
//! author sections rather than check bodies.

use geom::Surface;
use geom_core::spline::SpanLocate;
use geom_core::{Bounds, Interval, Tol};
use sweep::Lofted;

/// The ends of a box `ε/256` either side of `c`, narrow enough for the
/// assembly's certificates to resolve at every ε.
pub fn box_about(c: f64) -> (f64, f64) {
    let w = Tol::witness().eps() / 256.0;
    (c - w, c + w)
}

/// Every wall's poles, weights and v knots, in face order, through `f`.
fn wall_values<T: SpanLocate, R>(lofted: &Lofted<T>, f: impl Fn(T) -> R) -> Vec<R> {
    let mut out = Vec::new();
    for face in lofted.side_faces.iter().flatten().flatten() {
        let face = lofted.body.get_face(*face).expect("a built face");
        match lofted
            .body
            .get_surface(face.surface)
            .expect("a described face")
        {
            Surface::Nurbs(n) => {
                for p in n.control() {
                    out.extend([f(p.x), f(p.y), f(p.z)]);
                }
                out.extend(n.weights().iter().map(|w| f(*w)));
                out.extend(n.knot_values_v().values().iter().map(|k| f(*k)));
            }
            other => panic!("a lofted wall is a NURBS surface, got {other:?}"),
        }
    }
    out
}

/// Asserts every wall value of `enclosing` (poles, weights, v knots)
/// and every vertex holds the matching one of `sample`, the `f64` loft
/// of one member of the family `enclosing` was built over. One
/// construction builds both bodies, so they list both in one order.
pub fn assert_encloses(what: &str, enclosing: &Lofted<Interval>, sample: &Lofted<f64>) {
    let brackets = wall_values(enclosing, |x| (x.lo(), x.hi()));
    let values = wall_values(sample, |x| x);
    assert_eq!(
        brackets.len(),
        values.len(),
        "{what}: the wall shapes agree"
    );
    for (i, ((lo, hi), v)) in brackets.iter().zip(&values).enumerate() {
        assert!(
            lo <= v && v <= hi,
            "{what}: wall value {i} is {v}, outside its enclosure [{lo}, {hi}]"
        );
    }
    let corners: Vec<[(f64, f64); 3]> = enclosing
        .body
        .points()
        .map(|(_, p)| [p.x, p.y, p.z].map(|c| (c.lo(), c.hi())))
        .collect();
    let members: Vec<[f64; 3]> = sample.body.points().map(|(_, p)| [p.x, p.y, p.z]).collect();
    assert_eq!(
        corners.len(),
        members.len(),
        "{what}: the vertex counts agree"
    );
    for (i, (b, v)) in corners.iter().zip(&members).enumerate() {
        assert!(
            b.iter().zip(v).all(|((lo, hi), x)| lo <= x && x <= hi),
            "{what}: vertex {i} at {v:?} lies outside its enclosure {b:?}"
        );
    }
}
