//! **An edge's locus certificate**: C2's limbs 2 and 3 over an
//! `Intersection` carrier between two analytic surfaces, held with the
//! support it was derived over.
//!
//! The edge certificate reads two kinds of data. Its locus part (the
//! carrier over its interval lies within the band of both surfaces, and
//! is the only branch of their section there) reads the carrier, the
//! interval, the unordered surface pair and the band, and nothing else.
//! Its placement part (the ends on their vertices, the witness at
//! mid-parameter, the wedge's reach, the description) reads the
//! topology around the edge. A door that restates, copies, rebases or
//! grafts an edge changes the second and keeps the first, so the locus
//! part is held apart, as a fact about its support (`[topology.tiers]`):
//! it is read wherever that support is the same and derived wherever it
//! is not.
//!
//! "The same" is decided here, never on a door's word. A spline carrier
//! is the same when it is the same allocation (`Arc::ptr_eq`): a door
//! that keeps an edge's carrier clones the handle, and one that builds
//! another net, even a bitwise twin, derives again. A closed-form
//! carrier and every surface compare kind and scalar by scalar through
//! [`CertifiedEnclosure::same_value`]: bits at a value scalar, the
//! enclosure and its decoration at `Interval`, the expression node at
//! `Sym`. The interval and the band compare exactly.
//!
//! Only [`crate::EdgeCurve`]'s certification constructs one, right
//! after the limbs it records returned on that support; its fields are
//! private, so a certificate no derivation produced is unrepresentable.
//! Under D9 the limbs are a pure function of the support, so reading a
//! certificate whose support compares equal returns what deriving it
//! again would. The test profile derives it again at every read and
//! asserts the two agree (`certify::run_checks`).

use std::sync::Arc;

use geom::{Curve3, Surface};
use geom_core::predicate::Band;
use geom_core::{CertifiedEnclosure, Real};

/// C2's limbs 2–3 over one support (module docs). Cheap to clone: the
/// support is shared.
#[derive(Clone, Debug)]
pub struct LocusCertificate<T: Real>(Arc<Support<T>>);

/// What the limbs read.
#[derive(Debug)]
struct Support<T: Real> {
    /// The carrier as the edge stores it; a periodic carrier's view over
    /// the interval is a function of it and the interval.
    carrier: Curve3<T>,
    params: (T, T),
    pair: [Surface<T>; 2],
    band: Band,
}

impl<T: Real + CertifiedEnclosure> LocusCertificate<T> {
    /// The certificate of the limbs that just returned on this support.
    pub(crate) fn derived(
        carrier: &Curve3<T>,
        params: (T, T),
        pair: [&Surface<T>; 2],
        band: Band,
    ) -> Self {
        Self(Arc::new(Support {
            carrier: carrier.clone(),
            params,
            pair: pair.map(Surface::clone),
            band,
        }))
    }

    /// Whether this certificate's support is `carrier` over `params`
    /// between `pair`, in either order, at `band`.
    #[must_use]
    pub fn reads(
        &self,
        carrier: &Curve3<T>,
        params: (T, T),
        [s1, s2]: [&Surface<T>; 2],
        band: Band,
    ) -> bool {
        let Support {
            carrier: held,
            params: (p0, p1),
            pair: [h1, h2],
            band: held_band,
        } = &*self.0;
        *held_band == band
            && p0.same_value(params.0)
            && p1.same_value(params.1)
            && same_curve(held, carrier)
            && ((same_surface(h1, s1) && same_surface(h2, s2))
                || (same_surface(h1, s2) && same_surface(h2, s1)))
    }
}

/// A curve's scalars, in its lift's order.
fn curve_scalars<T: Real>(c: &Curve3<T>) -> Vec<T> {
    let out = core::cell::RefCell::new(Vec::new());
    let _ = c.map_scalar(|x| {
        out.borrow_mut().push(x);
        x
    });
    out.into_inner()
}

/// A surface's scalars, in its lift's order.
fn surface_scalars<T: Real>(s: &Surface<T>) -> Vec<T> {
    let out = core::cell::RefCell::new(Vec::new());
    let _ = s.map_scalar(|x| {
        out.borrow_mut().push(x);
        x
    });
    out.into_inner()
}

fn same_scalars<T: CertifiedEnclosure>(a: &[T], b: &[T]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.same_value(*y))
}

/// One carrier: a spline by its allocation, a closed form by kind and
/// scalars.
fn same_curve<T: Real + CertifiedEnclosure>(a: &Curve3<T>, b: &Curve3<T>) -> bool {
    match (a, b) {
        (Curve3::Nurbs(x), Curve3::Nurbs(y)) => Arc::ptr_eq(x, y),
        (Curve3::Periodic(x), Curve3::Periodic(y)) => Arc::ptr_eq(x, y),
        (Curve3::Nurbs(_) | Curve3::Periodic(_), _) | (_, Curve3::Nurbs(_) | Curve3::Periodic(_)) => {
            false
        }
        _ => {
            core::mem::discriminant(a) == core::mem::discriminant(b)
                && same_scalars(&curve_scalars(a), &curve_scalars(b))
        }
    }
}

/// One surface: a spline or approximating one by its allocation, an
/// analytic one by kind and scalars.
fn same_surface<T: Real + CertifiedEnclosure>(a: &Surface<T>, b: &Surface<T>) -> bool {
    match (a, b) {
        (Surface::Nurbs(x), Surface::Nurbs(y)) => Arc::ptr_eq(x, y),
        (Surface::Approx(x), Surface::Approx(y)) => Arc::ptr_eq(x, y),
        (Surface::Nurbs(_) | Surface::Approx(_), _) | (_, Surface::Nurbs(_) | Surface::Approx(_)) => {
            false
        }
        _ => {
            core::mem::discriminant(a) == core::mem::discriminant(b)
                && same_scalars(&surface_scalars(a), &surface_scalars(b))
        }
    }
}
