//! **The offset fit, as an injected door** — the one derivation in the
//! certification machinery that is written at a single scalar.
//!
//! [`crate::offset_fit`] derives an offset surface's fit and its
//! two-limb certificate at `f64`. Everything that consumes such a
//! surface is scalar-generic: [`geom::ApproxSurface::certify`] takes
//! its certifier as an argument, so an `ApproxSurface<T>` is
//! representable at every scalar and a body carrying one reaches the
//! validator, the offset mint and the transform door at whatever
//! scalar its owner holds. So the absence here is **not** a
//! certification right and cannot be spelled as a bound: it is the
//! absence of a DERIVATION, and the passes carry it as an [`Option`].
//!
//! [`OffsetFitLane`] is that door — the three operations in one value,
//! constructed only at `f64` ([`OffsetFitLane::fit`]) and handed to the
//! three passes that need it. [`OffsetFitScalar`] is the per-scalar
//! seam that answers whether a scalar has one; `None` is *"the fit is
//! not written here"*, never *"a value of this scalar may not
//! certify"* — that one is the missing [`geom_core::CertifiedEnclosure`]
//! impl (`docs/DUAL-DESIGN.md` DL1), and the two stopped sharing a
//! `None` when this module was written.
//!
//! The shape is the injected plane × NURBS lane's ([`crate::NurbsLane`]),
//! for the same reason and with the same discipline: a caller that can
//! derive hands the door in, a caller that cannot hands `None` and gets
//! the typed refusal its pass already had. It is the first instance of
//! `work/scalar/H5.md` §RATIFIED ruling 3 — every certified
//! sub-operation a plain function, the mixed passes taking their door
//! as a parameter.

use geom::surfaces::{NurbsSurface, Surface};
use geom_core::{Band, Real, Tol};

use crate::OffsetFitError;

/// **The offset-fit door**: the three offset-fit operations the
/// certification passes reach, in one value.
///
/// Its one constructor is [`OffsetFitLane::fit`], at `f64`, so holding
/// a value of this type IS the statement that the fit is derivable at
/// the scalar it is parameterised by. There is no other way to make one
/// and there is no third outcome: a pass holding `None` refuses typed.
#[derive(Clone, Copy)]
#[allow(clippy::type_complexity)]
pub struct OffsetFitLane<T: Real> {
    /// [`OffsetFitLane::recertify`]'s body.
    recertify:
        fn(&geom::ApproxSurface<T>, Tol, Band) -> Result<geom::OffsetCertificate, OffsetFitError>,
    /// [`OffsetFitLane::mint`]'s body.
    mint: fn(std::sync::Arc<NurbsSurface<T>>, T, Tol, Band) -> Result<Surface<T>, OffsetFitError>,
    /// [`OffsetFitLane::remap`]'s body.
    remap: fn(
        &geom::SurfaceDescription<T>,
        &NurbsSurface<T>,
        geom::ApproxWindow,
        f64,
        Band,
    ) -> Result<geom::OffsetCertificate, OffsetFitError>,
}

impl<T: Real> core::fmt::Debug for OffsetFitLane<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("OffsetFitLane")
    }
}

impl OffsetFitLane<f64> {
    /// The `f64` fit — the whole inventory of this door, and the only
    /// constructor there is.
    #[must_use]
    pub const fn fit() -> Self {
        Self {
            recertify: crate::offset_fit::recertify_approx,
            mint: crate::offset_fit::approx_offset_surface,
            remap: remap_offset_certificate,
        }
    }
}

impl<T: Real> OffsetFitLane<T> {
    /// Re-derives an approximating surface's certificate against its
    /// own stored description and fit, classified against `tol` — the
    /// tier-3 never-trust posture (O5), one dimension up from
    /// `EdgeCurve::recertify`.
    ///
    /// **The tolerance is the RUN's, not the surface's.** The edge
    /// machinery re-certifies every carrier against the run's band and
    /// never against a stored bound, and the surface claim is the same
    /// shape: O3 ratifies `sup ‖S_fit − (S + d·n)‖ ≤ ε_precision`, so
    /// verifying it means measuring against the ε this validation call
    /// runs at. A surface minted at a loose tolerance validating
    /// forever afterwards would be the stored bound quietly replacing
    /// the ratified one. The stored tolerance stays what it always was:
    /// the MINT's parameter, and the fit door's own gate.
    ///
    /// # Errors
    ///
    /// The fit door's typed refusal, when the re-derivation fails.
    pub fn recertify(
        self,
        approx: &geom::ApproxSurface<T>,
        tol: Tol,
        band: Band,
    ) -> Result<geom::OffsetCertificate, OffsetFitError> {
        (self.recertify)(approx, tol, band)
    }

    /// Mints the certified approximating surface for a NURBS operand's
    /// offset — the fit door, reached through the hook so the doors
    /// above it stay scalar-generic.
    ///
    /// The fit target is the run's ε and arrives as the witness, for
    /// [`OffsetFitLane::recertify`]'s reason: the mint and the
    /// re-derivation that must later re-establish its claim classify
    /// against the same number by construction, not because two callers
    /// passed the same one.
    ///
    /// # Errors
    ///
    /// The fit door's typed refusal (the meters, a rational operand,
    /// the refinement budget, a certificate limb).
    pub fn mint(
        self,
        base: std::sync::Arc<NurbsSurface<T>>,
        d: T,
        tol: Tol,
        band: Band,
    ) -> Result<Surface<T>, OffsetFitError> {
        (self.mint)(base, d, tol, band)
    }

    /// **The certificate of an offset description's fit, re-derived on
    /// the given `(description, fit)` pair.**
    ///
    /// The pair is handed in rather than read off a
    /// [`geom::ApproxSurface`] because the caller that needs this does
    /// not have one yet: it is building the surface, and
    /// [`geom::ApproxSurface::certify`] is the only door into the type.
    /// So this is the certifier that door takes.
    /// ([`OffsetFitLane::recertify`] is the same derivation reached the
    /// other way round — from a surface that already exists, for the
    /// validator that re-derives its claim.)
    ///
    /// **The classification tolerance is the CALLER's** and so is the
    /// band: `tolerance` is what the mapped surface will store and
    /// therefore what it must be shown to honour.
    ///
    /// # Errors
    ///
    /// The fit door's typed refusal, verbatim — a certificate limb
    /// above tolerance, a door meter, a window this derivation does not
    /// cover.
    pub fn remap(
        self,
        description: &geom::SurfaceDescription<T>,
        fit: &NurbsSurface<T>,
        window: geom::ApproxWindow,
        tolerance: f64,
        band: Band,
    ) -> Result<geom::OffsetCertificate, OffsetFitError> {
        (self.remap)(description, fit, window, tolerance, band)
    }
}

/// The remap door's `f64` body.
///
/// The window rule and the derivation behind it live in one place, so
/// this door, the storage mint and the validator's re-derivation cannot
/// disagree about the same surface. The `_at` form, deliberately: this
/// door classifies against the tolerance the SURFACE's claim was made
/// at — a stored datum, not the run's ε — which is what keeps the map
/// and the validator agreeing about a given surface (`topo::transform`'s
/// `map_approx` argues it). It is the one production caller of a
/// numeric-target routine, named at that routine's own door.
fn remap_offset_certificate(
    description: &geom::SurfaceDescription<f64>,
    fit: &NurbsSurface<f64>,
    window: geom::ApproxWindow,
    tolerance: f64,
    band: Band,
) -> Result<geom::OffsetCertificate, OffsetFitError> {
    let geom::SurfaceDescription::Offset { base, d } = description;
    crate::offset_fit::certify_offset_over_at(base, fit, *d, window, tolerance, band)
}

/// **Whether this scalar has an offset fit** — the per-scalar seam
/// behind [`OffsetFitLane`], and the one place each scalar's answer is
/// written.
///
/// `Some` is `f64`'s alone, because [`crate::offset_fit`] is written at
/// `f64`. `None` is a statement about the DERIVATION and never about
/// which values can arrive: an `ApproxSurface<T>` is representable at
/// every scalar, so a face carrying one does reach the passes at a
/// scalar with no lane, and each of them refuses typed rather than
/// passing.
///
/// It is a seam of its own rather than a method on a lane trait
/// (`work/scalar/H5.md` §RATIFIED ruling 3): the absence it carries is
/// *"not derived here"*, which is a different fact from a lane trait's
/// *"this scalar does not certify"*.
pub trait OffsetFitScalar: Real {
    /// This scalar's offset-fit door, or `None` where the fit is not
    /// derived.
    fn offset_fit_lane() -> Option<OffsetFitLane<Self>>;
}

impl OffsetFitScalar for f64 {
    fn offset_fit_lane() -> Option<OffsetFitLane<Self>> {
        Some(OffsetFitLane::fit())
    }
}

#[cfg(feature = "probe")]
impl OffsetFitScalar for geom_core::Probe {
    fn offset_fit_lane() -> Option<OffsetFitLane<Self>> {
        None
    }
}

#[cfg(feature = "interval")]
impl OffsetFitScalar for geom_core::interval::Interval {
    fn offset_fit_lane() -> Option<OffsetFitLane<Self>> {
        None
    }
}

/// **The symbolic tier over a base scalar**: no fit lane, for the BASE
/// scalar's reason. The tier changes how a margin DECIDES, not which
/// derivations exist, so wrapping a scalar cannot add one.
impl<T> OffsetFitScalar for geom_core::Sym<T>
where
    geom_core::Sym<T>: Real,
{
    fn offset_fit_lane() -> Option<OffsetFitLane<Self>> {
        None
    }
}

/// The dual tier: no fit lane, for the same reason as every other
/// non-`f64` scalar — the fit is not written here.
impl<T> OffsetFitScalar for geom_core::Dual<T>
where
    geom_core::Dual<T>: Real,
{
    fn offset_fit_lane() -> Option<OffsetFitLane<Self>> {
        None
    }
}
