//! **The fitted-pcurve derivations, as an injected door** — the three
//! operations a fitted (rung-3) or general chart image needs that only
//! a scalar with certification rights can perform.
//!
//! A fitted chart image's between-samples obligation is a C2
//! certificate — hull sup-norm and uniqueness tube, reached through
//! `geom_brep::ssi::certify` — and the image itself and its chart feet
//! are produced by the `edge_nurbs` foot schedule over certification
//! arithmetic (C9). Building any of the three IS certification: `f64`,
//! the telemetry probe, the interval scalar and `Sym` over any of those
//! may do it, and a [`geom_core::Dual`] may not (DL1 — a dual carries a
//! bracket since D1 and still has no
//! [`geom_core::CertifiedEnclosure`] impl).
//!
//! [`FittedLane`] is that door: the three operations in one value,
//! with one constructor ([`FittedLane::certified`]) whose `impl` block
//! is bounded on the right, so holding one IS the statement that the
//! scalar it is parameterised by may certify. A scalar that may not
//! cannot write the value at all:
//!
//! ```compile_fail,E0599
//! use geom_brep::FittedLane;
//! use geom_core::Dual64;
//! let _ = FittedLane::<Dual64>::certified();
//! ```
//!
//! The code is `E0599` because `certified` EXISTS on
//! `FittedLane<Dual64>` and its `impl` block's bounds are not met,
//! which `rustc` reports as "the associated function exists … but its
//! trait bounds were not satisfied". Stable rustdoc verifies only that
//! the block fails to build, so the code beside the fence is a
//! statement and not a check.
//!
//! **Where the `Some` comes from.** One seam answers it —
//! `topo::AtRestPolicy::fitted_lane`, DL3's per-scalar policy home
//! (`docs/DUAL-DESIGN.md`) — and every consumer reads that seam: the
//! pcurve mint (`topo::mint_pcurves`, its general-image derivation,
//! its chart-foot rim arms and its `certify_general` call) and the
//! tier-3 pcurve pass (`topo::pcurves::validate_pcurves`, through
//! [`crate::PcurveCache::recertify`]). A certifying scalar answers
//! `Some(FittedLane::certified())` and a dual answers `None`, and a
//! consumer holding `None` refuses typed with
//! [`crate::PcurveCertifyError::FittedLaneUnsupported`], naming the
//! scalar by the name the same seam hands it.
//!
//! **A `None` door never meets a fitted cache.** The two fitted
//! constructors ([`crate::PcurveCache::certify_fitted`] and
//! [`crate::PcurveCache::certify_general`]) take the door itself, not
//! an `Option`, so no `Fitted` or `General` cache exists at a scalar
//! whose seam answers `None`, and there is no certificate-less fitted
//! cache for any reader to meet. [`crate::PcurveCache::recertify`] is
//! the one door that takes the `Option`, because it dispatches every
//! variant and only its fitted arm needs the lane.
//!
//! The shape is [`crate::OffsetFitLane`]'s, and the absence is a
//! different fact from that one's: the offset fit's `None` is a
//! derivation written at one scalar, this one's is certification
//! rights. Under `work/scalar/H5.md` §RATIFIED ruling 3 the door is the
//! parameter a mixed pass takes and the scalar seam that produces it
//! is the per-scalar policy that cut leaves standing.

use geom::{Curve3, NurbsCurve2, NurbsCurve3, NurbsSurface, Surface};
use geom_core::{Band, Decide, Point2, Point3, Real};

use crate::PcurveCertifyError;
use crate::ssi::SsiCertificate;

/// **The fitted-pcurve door**: the three fitted-pcurve derivations the
/// mint and the tier-3 pass reach, in one value.
///
/// Its one constructor is [`FittedLane::certified`], bounded on
/// [`geom_core::CertifiedBounds`], so holding a value of this type IS
/// the statement that the scalar it is parameterised by may certify
/// (module docs). There is no other way to make one: the fields are
/// private and no other constructor exists.
#[derive(Clone, Copy)]
#[allow(clippy::type_complexity)]
pub struct FittedLane<T: Real> {
    /// [`FittedLane::fitted_certificate`]'s body.
    fitted_lane: fn(
        &Curve3<T>,
        T,
        T,
        &NurbsCurve2<T>,
        &Surface<T>,
        &Surface<T>,
        Band,
    ) -> Result<SsiCertificate<T>, PcurveCertifyError>,
    /// [`FittedLane::general_image`]'s body.
    general_image_lane:
        fn(&NurbsCurve3<T>, &NurbsSurface<T>) -> Result<NurbsCurve2<T>, PcurveCertifyError>,
    /// [`FittedLane::chart_foot`]'s body.
    chart_foot_lane: fn(Point3<T>, &NurbsSurface<T>) -> Result<Point2<f64>, PcurveCertifyError>,
}

impl<T: Decide + geom_core::CertifiedBounds> FittedLane<T> {
    /// The certified fitted-pcurve door — the whole inventory of this
    /// door, and the only constructor there is. Every field holds the
    /// one shared body the tree has for its derivation, instantiated at
    /// `T`.
    #[must_use]
    pub const fn certified() -> Self {
        Self {
            fitted_lane: crate::pcurve_cache::fitted_lane::<T>,
            general_image_lane: crate::pcurve_cache::general_image_lane::<T>,
            chart_foot_lane: crate::pcurve_cache::chart_foot_lane::<T>,
        }
    }
}

impl<T: Real> FittedLane<T> {
    /// The full C2 certificate of a fitted chart image against its
    /// operand pair — check 4 of the fitted lane's five checks, reached
    /// only from inside this crate's fitted doors.
    ///
    /// The carrier arrives as the edge's own [`Curve3`] (M6-3): a
    /// rung-3 `Curve3::Nurbs` feeds the SSI door directly; an exact
    /// `Curve3::Circle` (the sphere chart's GENERAL-circle class,
    /// walk row 4) is converted to its locus-exact rational-quadratic
    /// chain for the certificate limbs — every limb consulted is a
    /// statement about the LOCUS (on-locus hull, uniqueness tube), so
    /// the chain's own parameter never enters the certified claim;
    /// `t0`/`t1` name the traversed angular arc.
    ///
    /// # Errors
    ///
    /// [`PcurveCertifyError::FittedCertificate`] when the SSI
    /// certificate itself refuses, or for a (Circle carrier, NURBS
    /// operand) pairing — the NURBS limbs are parameter-coupled to a
    /// traced pcurve a synthetic arc chain does not have.
    #[allow(clippy::too_many_arguments)] // one parameter per named quantity
    pub(crate) fn fitted_certificate(
        self,
        carrier: &Curve3<T>,
        t0: T,
        t1: T,
        image: &NurbsCurve2<T>,
        surface: &Surface<T>,
        mate: &Surface<T>,
        band: Band,
    ) -> Result<SsiCertificate<T>, PcurveCertifyError> {
        (self.fitted_lane)(carrier, t0, t1, image, surface, mate, band)
    }

    /// **The chart image of a spline carrier on a NURBS wall.**
    ///
    /// The producer is `edge_nurbs`'s — the one derivation of this
    /// object in the tree (`edge_nurbs::chart_image`): foot points at
    /// the D9-fixed schedule, interpolated on the carrier's own
    /// parameter. It is EVIDENCE and certifies nothing by itself; the
    /// caller's next move is [`crate::PcurveCache::certify_general`],
    /// which bounds `sup_t |S(P(t)) − C(t)|` over the whole span
    /// against the operand pair.
    ///
    /// It sits on THIS door rather than beside its producer because the
    /// derivation and the certificate are the same split — both need
    /// certification arithmetic (C9), both are absent at a
    /// [`geom_core::Dual`] — and a mint that had to hold two doors for
    /// one image would carry the split twice. The plane × NURBS lane
    /// ([`crate::plane_nurbs_limbs`]) keeps its own door for the ADOPT
    /// path, which certifies the same image with the plane operand's
    /// limbs beside it.
    ///
    /// # Errors
    ///
    /// [`PcurveCertifyError::FittedCertificate`] when a foot point of
    /// the schedule will not converge or the interpolation is
    /// degenerate.
    pub fn general_image(
        self,
        carrier: &NurbsCurve3<T>,
        wall: &NurbsSurface<T>,
    ) -> Result<NurbsCurve2<T>, PcurveCertifyError> {
        (self.general_image_lane)(carrier, wall)
    }

    /// **The chart foot of one point** on a NURBS wall —
    /// [`FittedLane::general_image`]'s single-sample sibling, same
    /// producer (`edge_nurbs::chart_foot`).
    ///
    /// Its consumer is the pcurve mint's rim arms: they know the SHAPE
    /// of their image and are only missing its position, which on a
    /// chart wider than the face it trims is not a knot-domain end.
    /// Evidence, not a certificate — the caller offers it to its own
    /// metre-valued check.
    ///
    /// # Errors
    ///
    /// [`PcurveCertifyError::FittedCertificate`] when the projection
    /// will not converge.
    pub fn chart_foot(
        self,
        point: Point3<T>,
        wall: &NurbsSurface<T>,
    ) -> Result<Point2<f64>, PcurveCertifyError> {
        (self.chart_foot_lane)(point, wall)
    }
}

/// **The door's WIRING, field by field** — the rows that say which
/// free function each field of [`FittedLane::certified`] holds, rather
/// than what that function answered.
///
/// A row that compares outputs cannot see a door re-pointed at a
/// routine that agrees on the fixture in front of it; the helper
/// compares the stored function pointers instead, so a re-point is a
/// failure no matter what it computes. Function-pointer identity is
/// what `std::ptr::fn_addr_eq` compares and is not a language guarantee
/// (identical function bodies may be merged), which costs nothing here:
/// the three bodies differ, and a false PASS from a merge would need
/// the re-pointed routine to be instruction-identical to the one it
/// replaced.
///
/// The helper is instantiated once per certifying scalar.
/// `topo`'s `certified_enclosure_impl_census` counts those
/// instantiations against the `CertifiedEnclosure` impls in the tree,
/// both directions, and counts the tree's door values against its
/// roster of helpers, this one included.
#[cfg(test)]
mod wiring_rows {
    use super::FittedLane;
    use crate::pcurve_cache::{chart_foot_lane, fitted_lane, general_image_lane};

    /// `Ok(())` when every field holds its shared body; otherwise the
    /// name of the first field that does not.
    fn holds_the_certified_fitted_lane<T: geom_core::Decide + geom_core::CertifiedBounds>()
    -> Result<(), &'static str> {
        let lane = FittedLane::<T>::certified();
        if !std::ptr::fn_addr_eq(
            lane.fitted_lane,
            fitted_lane::<T> as fn(_, _, _, _, _, _, _) -> _,
        ) {
            return Err("fitted_lane is not `pcurve_cache::fitted_lane`");
        }
        if !std::ptr::fn_addr_eq(
            lane.general_image_lane,
            general_image_lane::<T> as fn(_, _) -> _,
        ) {
            return Err("general_image_lane is not `pcurve_cache::general_image_lane`");
        }
        if !std::ptr::fn_addr_eq(lane.chart_foot_lane, chart_foot_lane::<T> as fn(_, _) -> _) {
            return Err("chart_foot_lane is not `pcurve_cache::chart_foot_lane`");
        }
        Ok(())
    }

    #[test]
    fn f64_is_wired_to_the_certified_fitted_lane() {
        assert_eq!(
            holds_the_certified_fitted_lane::<f64>(),
            Ok(()),
            "`FittedLane::<f64>::certified()` holds something other than its three bodies"
        );
    }

    /// The symbolic tier holds the base scalar's door: the same
    /// bodies, instantiated at `Sym<f64>`.
    #[test]
    fn sym_over_f64_is_wired_to_the_certified_fitted_lane() {
        assert_eq!(
            holds_the_certified_fitted_lane::<geom_core::Sym<f64>>(),
            Ok(()),
            "`FittedLane::<Sym<f64>>::certified()` holds something other than its three bodies"
        );
    }

    #[cfg(feature = "probe")]
    #[test]
    fn probe_is_wired_to_the_certified_fitted_lane() {
        assert_eq!(
            holds_the_certified_fitted_lane::<geom_core::Probe>(),
            Ok(()),
            "`FittedLane::<Probe>::certified()` holds something other than its three bodies"
        );
    }

    #[test]
    fn interval_is_wired_to_the_certified_fitted_lane() {
        assert_eq!(
            holds_the_certified_fitted_lane::<geom_core::interval::Interval>(),
            Ok(()),
            "`FittedLane::<Interval>::certified()` holds something other than its three bodies"
        );
    }
}
