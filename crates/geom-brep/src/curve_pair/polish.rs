//! **Gauss–Newton polish** of two carriers' closest pair, each carrier
//! read through its point and first derivative at a parameter (a point's
//! foot on one carrier is [`geom::Curve3::foot`]). A polished parameter is a place to read,
//! never a decision: every point it yields is read by
//! [`super::Span::contact`].

use geom_core::{Decide, Point3, Vec3};

/// The steps each polish takes.
const STEPS: usize = 4;

/// Gauss–Newton from `(s, t)` toward the closest pair of the carriers
/// `a` and `b`, a step taken where `keep` accepts the metres it moves the
/// pair: the pair, and the metres its last step moved it.
pub(super) fn pair<T: Decide>(
    a: impl Fn(T) -> (Point3<T>, Vec3<T>),
    b: impl Fn(T) -> (Point3<T>, Vec3<T>),
    (mut s, mut t): (T, T),
    keep: impl Fn(T) -> bool,
) -> (T, T, T) {
    let mut moved = T::zero();
    for _ in 0..STEPS {
        (s, t) = (place(s), place(t));
        let ((pa, da), (pb, db)) = (a(s), b(t));
        let (w, n) = (pb - pa, da.cross(db));
        let nn = n.dot(n);
        let (ds, dt) = (w.cross(db).dot(n) / nn, w.cross(da).dot(n) / nn);
        moved = da.norm() * ds.abs() + db.norm() * dt.abs();
        if keep(moved) {
            s = s + ds;
            t = t + dt;
        }
    }
    (s, t, moved)
}

/// **The place a step is read from**: an enclosure's middle, and any
/// other value itself. A step read from a whole enclosure is a step from
/// every point of it, which grows the iterate by the box's spread each
/// step rather than closing on the pair; a step from one point is that
/// point's, and the pair it ends on is one place the readings after it
/// read.
fn place<T: Decide>(x: T) -> T {
    match x.structure_bracket() {
        Some((lo, hi)) if lo != hi => T::from_computed(lo + 0.5 * (hi - lo)),
        _ => x,
    }
}
