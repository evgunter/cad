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
//! is the same allocation (`Arc::ptr_eq`), the cheap and
//! common case, since a door that keeps an edge's carrier clones the
//! handle; failing that, it is the same data, read whole through its
//! `Debug` form, which prints every field that defines the net (knots,
//! control points, weights; a periodic loop's data and sense) and none
//! of the memos derived from them. A closed-form carrier and an
//! analytic surface compare kind and scalar by scalar through
//! [`Real::same_value`]. Either way a scalar is the same
//! when it has the same bits at a value scalar, the same enclosure and
//! decoration at `Interval`, and the same expression node at `Sym`
//! (whose `Debug` prints the node). The interval and the band compare
//! exactly.
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
use geom_core::Real;
use geom_core::predicate::Band;

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

impl<T: Real> LocusCertificate<T> {
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

fn same_scalars<T: Real>(a: &[T], b: &[T]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.same_value(*y))
}

/// One spline: the same allocation, or the same data.
fn same_spline<S: core::fmt::Debug>(x: &Arc<S>, y: &Arc<S>) -> bool {
    Arc::ptr_eq(x, y) || format!("{x:?}") == format!("{y:?}")
}

/// One carrier: a spline by [`same_spline`], a closed form by kind and
/// scalars.
pub(crate) fn same_curve<T: Real>(a: &Curve3<T>, b: &Curve3<T>) -> bool {
    match (a, b) {
        (Curve3::Nurbs(x), Curve3::Nurbs(y)) => same_spline(x, y),
        (Curve3::Periodic(x), Curve3::Periodic(y)) => same_spline(x, y),
        (Curve3::Nurbs(_) | Curve3::Periodic(_), _)
        | (_, Curve3::Nurbs(_) | Curve3::Periodic(_)) => false,
        _ => {
            core::mem::discriminant(a) == core::mem::discriminant(b)
                && same_scalars(&curve_scalars(a), &curve_scalars(b))
        }
    }
}

/// One surface: an analytic one by kind and scalars; a spline or
/// approximating one, which the limbs never read (they take two
/// analytic operands), by its allocation alone.
pub(crate) fn same_surface<T: Real>(a: &Surface<T>, b: &Surface<T>) -> bool {
    match (a, b) {
        (Surface::Nurbs(x), Surface::Nurbs(y)) => Arc::ptr_eq(x, y),
        (Surface::Approx(x), Surface::Approx(y)) => Arc::ptr_eq(x, y),
        (Surface::Nurbs(_) | Surface::Approx(_), _)
        | (_, Surface::Nurbs(_) | Surface::Approx(_)) => false,
        _ => {
            core::mem::discriminant(a) == core::mem::discriminant(b)
                && same_scalars(&surface_scalars(a), &surface_scalars(b))
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use std::sync::Arc;

    use geom::{Curve3, NurbsCurve3, Surface};
    use geom_core::spline::KnotVector;
    use geom_core::{Band, Point3, Real, Sym, Vec3};

    use super::LocusCertificate;

    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    fn plane(z: f64) -> Surface<f64> {
        Surface::Plane {
            origin: Point3::new(0.0, 0.0, z),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        }
    }

    fn cylinder(radius: f64) -> Surface<f64> {
        Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius,
            u_ref: Vec3::unit_x(),
        }
    }

    fn net() -> Curve3<f64> {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let points = vec![Point3::new(1.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0)];
        Curve3::Nurbs(Arc::new(NurbsCurve3::new(kv, points, vec![1.0, 1.0]).unwrap()))
    }

    /// **A certificate reads exactly its own support**: the same
    /// support reads, the pair in either order reads, and a spline
    /// carrier read through another allocation of the same data reads;
    /// each input changed alone misses — the carrier's data, a closed
    /// form's radius, an end of the interval a ulp off, a surface's
    /// scalar, the band. Red under a comparison that leaves out an input
    /// the limbs read, or one that takes a twin allocation for another
    /// curve.
    #[test]
    fn a_certificate_reads_its_own_support_and_misses_each_changed_input() {
        let (p, c) = (plane(0.0), cylinder(1.0));
        let params = (0.0, 1.0);
        let spline = net();
        let held = LocusCertificate::derived(&spline, params, [&p, &c], band());
        assert!(held.reads(&spline, params, [&p, &c], band()), "its own support");
        assert!(held.reads(&spline, params, [&c, &p], band()), "the pair in the other order");
        let twin = net();
        assert!(
            held.reads(&twin, params, [&p, &c], band()),
            "a twin allocation of the same net"
        );
        let other = {
            let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
            let points = vec![Point3::new(1.0, 0.0, 0.0), Point3::new(0.0, 1.0, 1e-12)];
            Curve3::Nurbs(Arc::new(NurbsCurve3::new(kv, points, vec![1.0, 1.0]).unwrap()))
        };
        assert!(!held.reads(&other, params, [&p, &c], band()), "another net");
        let ulp = f64::from_bits(1.0_f64.to_bits() - 1);
        assert!(!held.reads(&spline, (0.0, ulp), [&p, &c], band()), "an end a ulp off");
        assert!(
            !held.reads(&spline, params, [&plane(1e-15), &c], band()),
            "a surface's scalar"
        );
        assert!(
            !held.reads(&spline, params, [&p, &c], Band::new(1e-9, 2e-8).unwrap()),
            "the band"
        );
        let circle = |r| Curve3::circle(Point3::origin(), Vec3::unit_z(), r, Vec3::unit_x());
        let round = LocusCertificate::derived(&circle(1.0), params, [&p, &c], band());
        assert!(round.reads(&circle(1.0), params, [&p, &c], band()), "a closed form's own");
        assert!(
            !round.reads(&circle(1.0 + f64::EPSILON), params, [&p, &c], band()),
            "a closed form's radius"
        );
    }

    /// **At `Sym` one value is one expression**: a literal read twice is
    /// one node and the same value, while two unknowns with equal
    /// numeric channels are not. At `f64` the bits decide, so `0.0` and
    /// `-0.0` differ. Red under a comparison that reads `Sym`'s numeric
    /// channel alone.
    #[test]
    fn a_symbolic_value_is_the_same_only_as_its_own_expression() {
        let budget = geom_core::SymBudget {
            max_terms: 64,
            max_degree: 8,
        };
        let (verdicts, _) = geom_core::sym::with_session(budget, || {
            let one = Sym::<f64>::from_f64(1.0);
            (
                one.same_value(Sym::from_f64(1.0)),
                Sym::opaque(1.0).same_value(Sym::opaque(1.0)),
            )
        });
        assert_eq!(verdicts, (true, false), "(a literal twice, two unknowns)");
        assert!(!0.0_f64.same_value(-0.0), "f64 compares bits");
    }
}
