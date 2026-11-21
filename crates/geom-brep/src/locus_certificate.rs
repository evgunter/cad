//! **An edge's locus certificate**: C2's limbs 2 and 3 over an
//! `Intersection` carrier between two analytic surfaces, held with the
//! support it was derived over; and the one rule every certificate read
//! by its support follows ([`read_or_derive`]).
//!
//! The edge certificate reads two kinds of data. Its locus part (the
//! carrier over its interval lies within the band of both surfaces, and
//! is the only branch of their section there) reads the carrier, the
//! interval, the surface pair and the band, and nothing else.
//! Its placement part (the ends on their vertices, the witness at
//! mid-parameter, the wedge's reach, the description) reads the
//! topology around the edge. A door that restates, copies, rebases or
//! grafts an edge changes the second and keeps the first, so the locus
//! part is held apart, as a fact about its support (`[topology.tiers]`):
//! it is read wherever that support is the same and derived wherever it
//! is not.
//!
//! "The same" is decided here, never on a door's word: the carrier and
//! each surface by content ([`Curve3::same_content_by`],
//! [`Surface::same_content_by`]), and the interval, each value through
//! [`Real::same_value`]; the band exactly. That one method is the knob:
//! a scalar whose values are never the same (a dual, `Sym`, `Probe`)
//! reads no certificate and derives each where it is asked.
//!
//! Only [`crate::EdgeCurve`]'s certification constructs one, right
//! after the limbs it records returned on that support; its fields are
//! private, so a certificate no derivation produced is unrepresentable.
//! Under D9 the limbs are a pure function of the support, so reading a
//! certificate whose support compares equal returns what deriving it
//! again would.
//!
//! **The agreement pin.** In every test build — this crate's, and every
//! dependent's, whose dev-dependencies turn on this crate's
//! `read-pins-testing` feature — [`read_or_derive`] derives a certificate
//! it reads again and asserts the two agree. A release build (the
//! viewer's, debug assertions and all) carries no pin, so a read costs
//! the comparison alone there. The pin's derivation runs detached from
//! the decision record (`k_stats::detached`, `sym::aside`), so a run
//! with it and one without record the same decisions.
//!
//! **A read records what deriving would.** A certificate keeps the
//! decisions its derivation recorded, and a read replays them
//! ([`k_stats::replay`]), so every lane's verdict and escalation logs
//! are the same whether a door read or derived: the subdivision driver
//! certifies a leaf on that equality.

use std::sync::Arc;

use geom::{Curve3, Surface};
use geom_core::Real;
use geom_core::k_stats::{self, Replayable};
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
    /// The decisions the limbs recorded, which a read replays.
    recorded: Replayable,
}

impl<T: Real> LocusCertificate<T> {
    /// The certificate of the limbs that just returned on this support,
    /// with the decisions they `recorded`.
    pub(crate) fn derived(
        carrier: &Curve3<T>,
        params: (T, T),
        pair: [&Surface<T>; 2],
        band: Band,
        recorded: Replayable,
    ) -> Self {
        Self(Arc::new(Support {
            carrier: carrier.clone(),
            params,
            pair: pair.map(Surface::clone),
            band,
            recorded,
        }))
    }

    /// The decisions its derivation recorded.
    pub(crate) fn recorded(&self) -> &Replayable {
        &self.0.recorded
    }

    /// Whether this certificate's support is `carrier` over `params`
    /// between `pair`, in its order, at `band`. The order is part of the
    /// support: the limbs decide operand by operand in the order asked,
    /// so a certificate derived over the other order records its
    /// decisions in another order, and is no read of this one.
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
            recorded: _,
        } = &*self.0;
        let surface = |a: &Surface<T>, b: &Surface<T>| a.same_content_by(b, T::same_value);
        *held_band == band
            && p0.same_value(params.0)
            && p1.same_value(params.1)
            && held.same_content_by(carrier, T::same_value)
            && surface(h1, s1)
            && surface(h2, s2)
    }
}

/// Which certificate [`read_or_derive`] is asked for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Certified {
    /// An edge's locus certificate (check 6).
    Locus,
    /// A pcurve row's certificate.
    Row,
}

/// **How often each certificate was read and derived on this thread**,
/// at [`read_or_derive`], in a test build: the witness a row reads to
/// tell that the doors read what they keep (`(reads, derivations)` per
/// [`Certified`]).
#[cfg(any(test, feature = "read-pins-testing"))]
pub mod counts {
    use super::Certified;
    use core::cell::Cell;

    std::thread_local! {
        static COUNTS: Cell<[(u64, u64); 2]> = const { Cell::new([(0, 0); 2]) };
    }

    pub(super) fn tally(kind: Certified, read: bool) {
        COUNTS.with(|c| {
            let mut all = c.get();
            let slot = &mut all[kind as usize];
            if read {
                slot.0 += 1;
            } else {
                slot.1 += 1;
            }
            c.set(all);
        });
    }

    /// This thread's `(reads, derivations)` of `kind` so far.
    #[must_use]
    pub fn of(kind: Certified) -> (u64, u64) {
        COUNTS.with(|c| c.get()[kind as usize])
    }
}

/// What [`read_or_derive`] gave: a held certificate it read, or one it
/// derived with the record its derivation left in the decision frame.
pub(crate) enum Got<R> {
    /// The held certificate, its record replayed.
    Read(R),
    /// A derivation's certificate and its record, for the caller to keep
    /// beside it.
    Derived(R, Replayable),
}

impl<R> Got<R> {
    /// The certificate, read or derived.
    pub(crate) fn certificate(self) -> R {
        match self {
            Self::Read(r) | Self::Derived(r, _) => r,
        }
    }
}

/// **A certificate read by its support, or derived** (module docs).
///
/// Where nothing is `held`, `derive` runs under its own decision frame
/// ([`k_stats::detached`]), whose record is spliced where the caller's
/// decisions go and handed back with the certificate. Where a reader's
/// comparison found one, the held certificate is returned and its
/// derivation's record replayed in the derivation's place
/// ([`k_stats::replay`]) as this scope would record it
/// ([`Replayable::here`]), so the frame holds exactly what deriving it
/// would have left (Ev's ruling on PR 7182: a read returns what deriving
/// would, and the decisions a derivation records are part of that).
///
/// In a test build a held certificate is derived again, detached from
/// the decision record, and must `agree` with it and record the same
/// decisions.
///
/// # Errors
///
/// `derive`'s, where nothing is held.
///
/// # Panics
///
/// In a test build, where the certificate derived again refuses, does
/// not agree with the held one, or records other decisions: a door read
/// a certificate its support does not give.
#[cfg_attr(
    not(any(test, feature = "read-pins-testing")),
    allow(unused_variables, clippy::needless_pass_by_value)
)]
pub(crate) fn read_or_derive<R, E>(
    kind: Certified,
    held: Option<(R, &Replayable)>,
    derive: impl FnOnce() -> Result<R, E>,
    agree: impl FnOnce(&R, &R) -> bool,
) -> Result<Got<R>, E> {
    // A record that does not state here what its derivation would (one
    // taken in a check scope, read in a choice scope) is no read.
    let held = held.and_then(|(held, record)| record.here().map(|here| (held, here)));
    #[cfg(any(test, feature = "read-pins-testing"))]
    counts::tally(kind, held.is_some());
    let Some((held, recorded)) = held else {
        let (out, record) = k_stats::recorded(derive);
        return out.map(|derived| Got::Derived(derived, record));
    };
    #[cfg(any(test, feature = "read-pins-testing"))]
    {
        let ((again, recording), decisions) = geom_core::sym::aside(|| k_stats::detached(derive));
        decisions.discard();
        let again_record = recording.recorded();
        assert!(
            *again_record == recorded,
            "a certificate read by its support records other decisions than one derived again: \
             {} verdicts and {} escalations held, {} and {} derived; first verdicts apart: {:?}",
            recorded.verdicts.len(),
            recorded.escalations.len(),
            again_record.verdicts.len(),
            again_record.escalations.len(),
            recorded
                .verdicts
                .iter()
                .zip(&again_record.verdicts)
                .find(|(a, b)| a != b)
        );
        assert!(
            again.is_ok_and(|again| agree(&held, &again)),
            "a certificate read by its support differs from the one derived again"
        );
    }
    k_stats::replay(&recorded);
    Ok(Got::Read(held))
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
        Curve3::Nurbs(Arc::new(
            NurbsCurve3::new(kv, points, vec![1.0, 1.0]).unwrap(),
        ))
    }

    /// **A certificate reads exactly its own support**: the same
    /// support reads, and a spline
    /// carrier read through another allocation of the same data reads;
    /// each input changed alone misses — the carrier's data, a closed
    /// form's radius, an end of the interval a ulp off, a surface's
    /// scalar, the band, and the pair in the other order (whose limbs
    /// decide in the other order). Red under a comparison that leaves out
    /// an input the limbs read, or one that takes a twin allocation for
    /// another curve.
    #[test]
    fn a_certificate_reads_its_own_support_and_misses_each_changed_input() {
        let (p, c) = (plane(0.0), cylinder(1.0));
        let params = (0.0, 1.0);
        let spline = net();
        let held = LocusCertificate::derived(&spline, params, [&p, &c], band(), Default::default());
        assert!(
            held.reads(&spline, params, [&p, &c], band()),
            "its own support"
        );
        assert!(
            !held.reads(&spline, params, [&c, &p], band()),
            "the pair in the other order"
        );
        let twin = net();
        assert!(
            held.reads(&twin, params, [&p, &c], band()),
            "a twin allocation of the same net"
        );
        let other = {
            let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
            let points = vec![Point3::new(1.0, 0.0, 0.0), Point3::new(0.0, 1.0, 1e-12)];
            Curve3::Nurbs(Arc::new(
                NurbsCurve3::new(kv, points, vec![1.0, 1.0]).unwrap(),
            ))
        };
        assert!(!held.reads(&other, params, [&p, &c], band()), "another net");
        let ulp = f64::from_bits(1.0_f64.to_bits() - 1);
        assert!(
            !held.reads(&spline, (0.0, ulp), [&p, &c], band()),
            "an end a ulp off"
        );
        assert!(
            !held.reads(&spline, params, [&plane(1e-15), &c], band()),
            "a surface's scalar"
        );
        assert!(
            !held.reads(&spline, params, [&p, &c], Band::new(1e-9, 2e-8).unwrap()),
            "the band"
        );
        let circle = |r| Curve3::circle(Point3::origin(), Vec3::unit_z(), r, Vec3::unit_x());
        let round =
            LocusCertificate::derived(&circle(1.0), params, [&p, &c], band(), Default::default());
        assert!(
            round.reads(&circle(1.0), params, [&p, &c], band()),
            "a closed form's own"
        );
        assert!(
            !round.reads(
                &circle(f64::from_bits(1.0_f64.to_bits() + 1)),
                params,
                [&p, &c],
                band()
            ),
            "a closed form's radius"
        );
    }

    /// **Nothing is read at `Sym`**: the same support, the same `Arc`
    /// included, does not read there, while at `f64` it does; and at
    /// `f64` the bits decide, so `0.0` and `-0.0` differ. Red under a
    /// read at a scalar whose derivations mint fresh unknowns.
    #[test]
    fn nothing_is_read_at_the_symbolic_tier() {
        let lift = |c: &Curve3<f64>| c.map_scalar(Sym::<f64>::from_f64);
        let lift_s = |s: &Surface<f64>| s.map_scalar(Sym::<f64>::from_f64);
        let budget = geom_core::SymBudget {
            max_terms: 64,
            max_degree: 8,
        };
        let (read, _) = geom_core::sym::with_session(budget, || {
            let (carrier, p, c) = (lift(&net()), lift_s(&plane(0.0)), lift_s(&cylinder(1.0)));
            let params = (Sym::from_f64(0.0), Sym::from_f64(1.0));
            let held =
                LocusCertificate::derived(&carrier, params, [&p, &c], band(), Default::default());
            held.reads(&carrier, params, [&p, &c], band())
        });
        assert!(!read, "Sym reads its own support");
        let (p, c, carrier) = (plane(0.0), cylinder(1.0), net());
        let held =
            LocusCertificate::derived(&carrier, (0.0, 1.0), [&p, &c], band(), Default::default());
        assert!(
            held.reads(&carrier, (0.0, 1.0), [&p, &c], band()),
            "f64 reads its own"
        );
        assert!(!0.0_f64.same_value(-0.0), "f64 compares bits");
    }
}
