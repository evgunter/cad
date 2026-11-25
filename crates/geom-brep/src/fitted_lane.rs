//! **The certification-arithmetic pcurve derivations, as an injected
//! door** — the operations a fitted, general or projected chart image
//! needs that only a scalar with certification rights can perform.
//!
//! A fitted chart image's between-samples obligation is C2's limb 2
//! against its own chart — the hull bound on `|S(P(t)) − C(t)|`,
//! reached through `geom_brep::ssi::certify` — with the image and its
//! chart feet produced by the one image function
//! (`geom_brep::carrier_image`) over certification arithmetic (C9); a spline carrier's projected image on
//! an analytic chart reads its hull terms (its sector hulls and the
//! chart's implicit form composed along the carrier) the same way.
//! Building any of them IS certification: `f64`,
//! the telemetry probe, the interval scalar and `Sym` over any of those
//! may do it, and a [`geom_core::Dual`] may not (DL1 — a dual carries a
//! bracket since D1 and still has no
//! [`geom_core::CertifiedEnclosure`] impl).
//!
//! [`FittedLane`] is that door: the operations in one value,
//! with one constructor ([`FittedLane::certified`]), declared over
//! [`geom_core::Certifies`], so holding one IS the statement that the
//! scalar it is parameterised by certifies. A scalar that does not has
//! no such type. A pass that runs at every scalar holds a
//! [`FittedLaneOf`], a sealed trait whose two implementors are the lane
//! and [`geom_core::NoLane`]: of that holder the statement is that it is
//! the lane or it has no values.
//!
//! ```compile_fail,E0277
//! use geom_brep::FittedLane;
//! use geom_core::Dual64;
//! let _ = FittedLane::<Dual64>::certified();
//! ```
//!
//! Stable rustdoc verifies only that the block fails to build, so the
//! code beside the fence is a statement and not a check.
//!
//! **Where the `Some` comes from.** One seam answers it —
//! `topo::AtRestPolicy::fitted_lane`, DL3's per-scalar policy home
//! (`docs/DUAL-DESIGN.md`) — and every consumer reads that seam: the
//! pcurve mint (`topo::mint_pcurves`, its general-image derivation,
//! its chart-foot rim arms and its `certify_general` and
//! `certify_projected` calls) and the
//! tier-3 pcurve pass (`topo::pcurves::validate_pcurves`, through
//! [`crate::PcurveCache::recertify`]). A certifying scalar answers
//! `Some(FittedLane::certified())` and a dual answers `None`, and a
//! consumer holding `None` refuses typed with
//! [`crate::PcurveCertifyError::FittedLaneUnsupported`], naming the
//! scalar by its [`geom_core::Real::NAME`].
//!
//! **A `None` door refuses where the door is first needed.** For an
//! image the caller already holds, that is check 4 of the fitted lane's
//! four checks, where every fitted cache's certificate is derived — in
//! [`crate::PcurveCache::certify_general`] and
//! [`crate::PcurveCache::recertify`], the two doors that take the
//! `Option` because a caller reaches them at every scalar (the mint
//! with a construction's stated `General` image, the tier-3 pass with
//! every row). Checks 1–3 read no door, so an image they refuse draws
//! the same verdict at every scalar, and no `Fitted` or `General`
//! cache is built without the door; a spline carrier's projected image
//! on an analytic chart refuses at the same check 4
//! ([`crate::PcurveCache::certify_projected`]). For an image the mint
//! must DERIVE (a general image on a spline chart) it is the
//! derivation, which refuses before any check runs; the mint leaves
//! such a face rowless at a scalar with no door, since that scalar is
//! owed no fitted rows (`topo::pcurves`' `mint_faces`).
//! [`crate::PcurveCache::certify_fitted`] takes the door itself.
//!
//! The shape is [`crate::OffsetFitLane`]'s, and the absence is a
//! different fact from that one's: the offset fit's `None` is a
//! derivation written at one scalar, this one's is certification
//! rights. Under H5's ratified ruling 3 (PR 2701) the door is the
//! parameter a mixed pass takes and the scalar seam that produces it
//! is the per-scalar policy that cut leaves standing.

use geom::{Curve3, NurbsCurve2, NurbsCurve3, NurbsSurface, Surface};
use geom_core::{Band, Point2, Point3, Real};

use crate::PcurveCertifyError;
use crate::pcurve_cache::projected::ProjectedHull;

/// The scalars that hold a [`FittedLane`], by their [`Real::NAME`]s —
/// the replay list [`crate::PcurveCertifyError::FittedLaneUnsupported`]
/// renders. Membership is the policy's answer, which this crate cannot
/// read, so topo pins it against `AtRestPolicy::fitted_lane` at every
/// scalar (`stated_general_image_mint.rs`). The symbolic tier holds the
/// door over a certifying base only.
#[cfg(not(feature = "probe"))]
pub const FITTED_DOOR_HOLDERS: &[&str] = &[
    <f64 as Real>::NAME,
    <geom_core::Interval as Real>::NAME,
    <geom_core::Sym<f64> as Real>::NAME,
];

/// The scalars that hold a [`FittedLane`] (the unprobed build's docs).
#[cfg(feature = "probe")]
pub const FITTED_DOOR_HOLDERS: &[&str] = &[
    <f64 as Real>::NAME,
    <geom_core::Probe as Real>::NAME,
    <geom_core::Interval as Real>::NAME,
    <geom_core::Sym<f64> as Real>::NAME,
];

/// **The fitted-pcurve door**: the certification-arithmetic pcurve
/// derivations the mint and the tier-3 pass reach, in one value.
///
/// Its one constructor is [`FittedLane::certified`], bounded on
/// [`geom_core::CertifiedBounds`], so holding a value of this type IS
/// the statement that the scalar it is parameterised by may certify
/// (module docs). There is no other way to make one: the fields are
/// private and no other constructor exists.
#[derive(Clone, Copy)]
#[allow(clippy::type_complexity)]
pub struct FittedLane<T: geom_core::Certifies> {
    /// [`FittedLane::fitted_certificate`]'s body.
    fitted_lane: fn(
        &Curve3<T>,
        &NurbsCurve2<T>,
        crate::pcurve_cache::FittedChart<'_, T>,
        (T, T),
        Band,
    ) -> Result<T, PcurveCertifyError>,
    /// [`FittedLane::general_image`]'s body.
    general_image_lane: fn(
        &Curve3<T>,
        (T, T),
        &NurbsSurface<T>,
        Band,
    ) -> Result<NurbsCurve2<T>, PcurveCertifyError>,
    /// [`FittedLane::chart_foot`]'s body.
    chart_foot_lane: fn(Point3<T>, &NurbsSurface<T>) -> Result<Point2<f64>, PcurveCertifyError>,
    /// [`FittedLane::projected_hull`]'s body.
    projected_lane: fn(
        &crate::ProjectedImage<T>,
        &NurbsCurve3<T>,
        &Surface<T>,
        crate::pcurve_cache::Parts,
        &[f64],
    ) -> Result<ProjectedHull, PcurveCertifyError>,
}

impl<T: geom_core::Certifies + geom_core::CertifiedBounds> FittedLane<T> {
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
            projected_lane: crate::pcurve_cache::projected_hull_lane::<T>,
        }
    }
}

impl<T: geom_core::Certifies> FittedLane<T> {
    /// The hull bound of a fitted chart image against its own chart —
    /// check 4 of the fitted lane's four checks, reached only from
    /// inside this crate's fitted doors: a certified bound on
    /// `sup |S(P(t)) − C(t)|` over the image's domain, which covers the
    /// edge's interval `params`. `chart` is a spline chart, or an
    /// analytic one with the class of the patch that agrees with it along
    /// the image ([`crate::pcurve_cache::FittedChart`]).
    ///
    /// # Errors
    ///
    /// [`PcurveCertifyError::FittedCertificate`] when the bound refuses
    /// or the carrier cannot be enclosed.
    pub(crate) fn fitted_certificate(
        self,
        carrier: &Curve3<T>,
        image: &NurbsCurve2<T>,
        chart: crate::pcurve_cache::FittedChart<'_, T>,
        params: (T, T),
        band: Band,
    ) -> Result<T, PcurveCertifyError> {
        (self.fitted_lane)(carrier, image, chart, params, band)
    }

    /// **The chart image of a carrier on a NURBS wall.**
    ///
    /// The one image of a carrier on a wall (`crate::carrier_image`,
    /// through `pcurve_cache::general_image_lane`): the wall's foot path
    /// interpolated on the carrier's own knots, which the trace's
    /// certificate and the edge certificate at rest read too. It
    /// is EVIDENCE and certifies nothing by itself; the caller's next
    /// move is [`crate::PcurveCache::certify_general`], which bounds
    /// `sup_t |S(P(t)) − C(t)|` over the whole span against the chart.
    ///
    /// `params` is read only for a carrier with no knot domain. A
    /// spline's image spans its own knot domain, because the composite
    /// pairs carrier and image on one parameter there; whether that
    /// domain holds the edge's interval is check 4's to refuse
    /// (`fitted_lane`), not the producer's. Any other carrier's image
    /// spans `params`' outer ends.
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
    /// the schedule will not converge or the interpolation refuses;
    /// [`PcurveCertifyError::CarrierDomain`] when the image cannot be
    /// re-expressed on the carrier's own domain.
    pub fn general_image(
        self,
        carrier: &Curve3<T>,
        params: (T, T),
        wall: &NurbsSurface<T>,
        band: Band,
    ) -> Result<NurbsCurve2<T>, PcurveCertifyError> {
        (self.general_image_lane)(carrier, params, wall, band)
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

/// **A [`FittedLane`] as a pass that runs at every scalar holds it**:
/// the lane at a scalar that certifies, [`geom_core::NoLane`] (no
/// values) at one that does not (`geom-core` README, DL5.3). Sealed:
/// those two are its only implementors, so a pass that holds one holds
/// the lane's own constructor's value or nothing ([`crate::NurbsLaneOf`]
/// carries the seal's pin; this trait's `projected_hull` names types no
/// other crate can spell, so no doctest can write its methods out).
///
/// Each method is the entry to a routine declared over
/// [`geom_core::Certifies`]: the pass asks whether it holds a lane, and
/// what runs with the lane in hand is not a function at a dual.
pub trait FittedLaneOf<T: Real>: crate::lane_seal::Sealed + Copy + Send + Sync {
    /// The fitted lane's four checks of `image` over `[t0, t1]`
    /// (`pcurve_cache`'s `run_fitted_checks`).
    ///
    /// # Errors
    /// The checks' own.
    fn fitted_checks(
        self,
        image: &std::sync::Arc<NurbsCurve2<T>>,
        span: (T, T),
        carrier: &Curve3<T>,
        surface: &Surface<T>,
        band: Band,
    ) -> Result<crate::PcurveCertificate<T>, PcurveCertifyError>;

    /// [`crate::PcurveCache::certify_fitted`].
    ///
    /// # Errors
    /// The door's own.
    fn certify_fitted(
        self,
        image: std::sync::Arc<NurbsCurve2<T>>,
        span: (T, T),
        carrier: &Curve3<T>,
        surface: &Surface<T>,
        band: Band,
    ) -> Result<crate::PcurveCache<T>, PcurveCertifyError>;

    /// [`FittedLane::general_image`].
    ///
    /// # Errors
    /// The lane's own.
    fn general_image(
        self,
        carrier: &Curve3<T>,
        params: (T, T),
        wall: &NurbsSurface<T>,
        band: Band,
    ) -> Result<NurbsCurve2<T>, PcurveCertifyError>;

    /// The projected image's hull against its twin (`projected_hull_lane`). Its operands are this crate's,
    /// so only this crate's passes can call it.
    #[allow(private_interfaces)]
    fn projected_hull(
        self,
        image: &crate::ProjectedImage<T>,
        twin_net: &NurbsCurve3<T>,
        twin: &Surface<T>,
        parts: crate::pcurve_cache::Parts,
        cuts: &[f64],
    ) -> Result<ProjectedHull, PcurveCertifyError>;

    /// [`FittedLane::chart_foot`].
    ///
    /// # Errors
    /// The lane's own.
    fn chart_foot(
        self,
        point: Point3<T>,
        wall: &NurbsSurface<T>,
    ) -> Result<Point2<f64>, PcurveCertifyError>;
}

impl<T: geom_core::Certifies> FittedLaneOf<T> for FittedLane<T> {
    fn fitted_checks(
        self,
        image: &std::sync::Arc<NurbsCurve2<T>>,
        span: (T, T),
        carrier: &Curve3<T>,
        surface: &Surface<T>,
        band: Band,
    ) -> Result<crate::PcurveCertificate<T>, PcurveCertifyError> {
        crate::pcurve_cache::run_fitted_checks(image, span, carrier, surface, band, self)
    }

    fn certify_fitted(
        self,
        image: std::sync::Arc<NurbsCurve2<T>>,
        (t0, t1): (T, T),
        carrier: &Curve3<T>,
        surface: &Surface<T>,
        band: Band,
    ) -> Result<crate::PcurveCache<T>, PcurveCertifyError> {
        crate::PcurveCache::certify_fitted(image, t0, t1, carrier, surface, band, self)
    }

    fn general_image(
        self,
        carrier: &Curve3<T>,
        params: (T, T),
        wall: &NurbsSurface<T>,
        band: Band,
    ) -> Result<NurbsCurve2<T>, PcurveCertifyError> {
        (self.general_image_lane)(carrier, params, wall, band)
    }

    #[allow(private_interfaces)]
    fn projected_hull(
        self,
        image: &crate::ProjectedImage<T>,
        twin_net: &NurbsCurve3<T>,
        twin: &Surface<T>,
        parts: crate::pcurve_cache::Parts,
        cuts: &[f64],
    ) -> Result<ProjectedHull, PcurveCertifyError> {
        (self.projected_lane)(image, twin_net, twin, parts, cuts)
    }

    fn chart_foot(
        self,
        point: Point3<T>,
        wall: &NurbsSurface<T>,
    ) -> Result<Point2<f64>, PcurveCertifyError> {
        (self.chart_foot_lane)(point, wall)
    }
}

impl<T: Real> FittedLaneOf<T> for geom_core::NoLane {
    fn fitted_checks(
        self,
        _image: &std::sync::Arc<NurbsCurve2<T>>,
        _span: (T, T),
        _carrier: &Curve3<T>,
        _surface: &Surface<T>,
        _band: Band,
    ) -> Result<crate::PcurveCertificate<T>, PcurveCertifyError> {
        match self {}
    }

    fn certify_fitted(
        self,
        _image: std::sync::Arc<NurbsCurve2<T>>,
        _span: (T, T),
        _carrier: &Curve3<T>,
        _surface: &Surface<T>,
        _band: Band,
    ) -> Result<crate::PcurveCache<T>, PcurveCertifyError> {
        match self {}
    }

    fn general_image(
        self,
        _carrier: &Curve3<T>,
        _params: (T, T),
        _wall: &NurbsSurface<T>,
        _band: Band,
    ) -> Result<NurbsCurve2<T>, PcurveCertifyError> {
        match self {}
    }

    #[allow(private_interfaces)]
    fn projected_hull(
        self,
        _image: &crate::ProjectedImage<T>,
        _twin_net: &NurbsCurve3<T>,
        _twin: &Surface<T>,
        _parts: crate::pcurve_cache::Parts,
        _cuts: &[f64],
    ) -> Result<ProjectedHull, PcurveCertifyError> {
        match self {}
    }

    fn chart_foot(
        self,
        _point: Point3<T>,
        _wall: &NurbsSurface<T>,
    ) -> Result<Point2<f64>, PcurveCertifyError> {
        match self {}
    }
}

#[cfg(test)]
mod wiring_rows {
    use super::FittedLane;
    use crate::pcurve_cache::projected_hull_lane;
    use crate::pcurve_cache::{chart_foot_lane, fitted_lane, general_image_lane};

    /// `Ok(())` when every field holds its shared body; otherwise the
    /// name of the first field that does not.
    fn holds_the_certified_fitted_lane<T: geom_core::Certifies + geom_core::CertifiedBounds>()
    -> Result<(), &'static str> {
        let lane = FittedLane::<T>::certified();
        if !std::ptr::fn_addr_eq(lane.fitted_lane, fitted_lane::<T> as fn(_, _, _, _, _) -> _) {
            return Err("fitted_lane is not `pcurve_cache::fitted_lane`");
        }
        if !std::ptr::fn_addr_eq(
            lane.general_image_lane,
            general_image_lane::<T> as fn(_, _, _, _) -> _,
        ) {
            return Err("general_image_lane is not `pcurve_cache::general_image_lane`");
        }
        if !std::ptr::fn_addr_eq(lane.chart_foot_lane, chart_foot_lane::<T> as fn(_, _) -> _) {
            return Err("chart_foot_lane is not `pcurve_cache::chart_foot_lane`");
        }
        if !std::ptr::fn_addr_eq(
            lane.projected_lane,
            projected_hull_lane::<T> as fn(_, _, _, _, _) -> _,
        ) {
            return Err("projected_lane is not `pcurve_cache::projected_hull_lane`");
        }
        Ok(())
    }

    #[test]
    fn f64_is_wired_to_the_certified_fitted_lane() {
        assert_eq!(
            holds_the_certified_fitted_lane::<f64>(),
            Ok(()),
            "`FittedLane::<f64>::certified()` holds something other than its bodies"
        );
    }

    /// The symbolic tier holds the base scalar's door: the same
    /// bodies, instantiated at `Sym<f64>`.
    #[test]
    fn sym_over_f64_is_wired_to_the_certified_fitted_lane() {
        assert_eq!(
            holds_the_certified_fitted_lane::<geom_core::Sym<f64>>(),
            Ok(()),
            "`FittedLane::<Sym<f64>>::certified()` holds something other than its bodies"
        );
    }

    #[cfg(feature = "probe")]
    #[test]
    fn probe_is_wired_to_the_certified_fitted_lane() {
        assert_eq!(
            holds_the_certified_fitted_lane::<geom_core::Probe>(),
            Ok(()),
            "`FittedLane::<Probe>::certified()` holds something other than its bodies"
        );
    }

    #[test]
    fn interval_is_wired_to_the_certified_fitted_lane() {
        assert_eq!(
            holds_the_certified_fitted_lane::<geom_core::interval::Interval>(),
            Ok(()),
            "`FittedLane::<Interval>::certified()` holds something other than its bodies"
        );
    }
}
