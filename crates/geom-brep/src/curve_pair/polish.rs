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

/// **The place a step is read from**: an enclosure's middle
/// ([`geom::bracket_mid`]), and any other value itself. A step read from
/// a whole enclosure is a step from every point of it, which grows the
/// iterate by the box's spread each step rather than closing on the
/// pair; a step from one point is that point's, and the pair it ends on
/// is one place the readings after it read.
///
/// **A bracket with an infinite end has no middle**, and is kept whole:
/// the step is then read from the whole enclosure, its metres are as
/// wide, and the meters after the polish refuse on them. Poison has no
/// bracket and is kept too.
///
/// **A selection that moves with the inputs**, so at a `Dual` over an
/// enclosure the place is the value channel's middle with the tangent
/// dropped, and at `Sym` over one a literal: #874's class
/// (`geom::projection_policy`'s `mid`). A point scalar, `Dual<f64>`
/// included, is its own middle and keeps everything it carries.
fn place<T: Decide>(x: T) -> T {
    match x.structure_bracket() {
        Some((lo, hi)) if lo != hi && lo.is_finite() && hi.is_finite() => {
            T::from_computed(geom::bracket_mid((lo, hi)))
        }
        _ => x,
    }
}

#[cfg(test)]
mod tests {
    use geom_core::Interval;

    /// The middle of an enclosure, a point kept, and an enclosure with an
    /// infinite end kept whole.
    #[test]
    fn a_place_is_an_enclosures_middle_and_an_unbounded_one_is_kept() {
        use geom_core::Bounds;
        let ends = |x: Interval| (x.lo(), x.hi());
        assert_eq!(
            ends(super::place(Interval::from_bounds(1.0, 3.0))),
            (2.0, 2.0)
        );
        assert_eq!(super::place(0.25_f64).to_bits(), 0.25_f64.to_bits());
        let unbounded = Interval::from_bounds(1.0, f64::INFINITY);
        assert_eq!(ends(super::place(unbounded)), (1.0, f64::INFINITY));
    }
}
