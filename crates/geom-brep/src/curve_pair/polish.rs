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
        let ((pa, da), (pb, db)) = (a(s), b(t));
        let d = pa - pb;
        let (aa, bb, ab) = (da.dot(da), db.dot(db), da.dot(db));
        let (ad, bd) = (da.dot(d), db.dot(d));
        let det = aa * bb - ab.powi(2);
        let (ds, dt) = ((ab * bd - bb * ad) / det, (aa * bd - ab * ad) / det);
        moved = da.norm() * ds.abs() + db.norm() * dt.abs();
        if keep(moved) {
            s = s + ds;
            t = t + dt;
        }
    }
    (s, t, moved)
}
