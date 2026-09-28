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
//! [`OffsetFitLane`] is that door: the three operations in one value,
//! with one constructor ([`OffsetFitLane::fit`]) which exists at `f64`
//! alone. Holding one IS the statement that the fit is derivable at
//! the scalar it is parameterised by, and a pass holding `None`
//! refuses typed.
//!
//! **Where the `Some` comes from.** One seam answers it —
//! `topo::AtRestPolicy::offset_fit_lane`, DL3's per-scalar policy home
//! (`docs/DUAL-DESIGN.md`) — and five sites read that seam: check 1's
//! tier-3 battery, the two certified validation doors it serves, the
//! offset mint (`topo::replace_face`) and the transform's surface map
//! (`topo::transform`). `f64` answers `Some`, every other scalar
//! answers `None`, and `None` means *"the fit is derived at `f64`
//! only"* — never *"a value of this scalar may not certify"*, which is
//! the missing [`geom_core::CertifiedEnclosure`] impl (DL1) and a
//! different fact about a different thing.
//!
//! The shape is the injected plane × NURBS lane's ([`crate::NurbsLane`]),
//! for the same reason and with the same discipline: a caller that can
//! derive hands the door in, a caller that cannot hands `None` and gets
//! the typed refusal its pass already had. Under `work/scalar/H5.md`
//! §RATIFIED ruling 3 the door is the parameter a mixed pass takes;
//! the scalar seam that produces it is the per-scalar policy that cut
//! leaves standing, folded into `topo::AtRestPolicy` rather than
//! carried on a trait of its own.
//!
//! # This file is on the shell's offset chain
//!
//! All three doors take the run's ε as the [`Tol`] witness and no
//! `f64` epsilon, and the SHELL-TOLERANCE-CHAIN census
//! (`crates/topo/tests/shell_tolerance_chain.rs`) carries this file
//! with none declared: an `f64` tolerance parameter or an `.eps()`
//! read here reds that census and has to be said what it is for.

use geom::surfaces::{NurbsSurface, Surface};
use geom_core::{Band, Real, Tol};

use crate::OffsetFitError;

/// **The offset-fit door**: the three offset-fit operations the
/// certification passes reach, in one value — over two bodies, because
/// [`OffsetFitLane::recertify`] and [`OffsetFitLane::remap`] are one
/// certifier reached from a surface and from an unpacked triple.
///
/// Its one constructor is [`OffsetFitLane::fit`], at `f64`, so holding
/// a value of this type IS the statement that the fit is derivable at
/// the scalar it is parameterised by. There is no other way to make one
/// and there is no third outcome: a pass holding `None` refuses typed.
#[derive(Clone, Copy)]
#[allow(clippy::type_complexity)]
pub struct OffsetFitLane<T: Real> {
    /// [`OffsetFitLane::mint`]'s body.
    mint: fn(std::sync::Arc<NurbsSurface<T>>, T, Tol, Band) -> Result<Surface<T>, OffsetFitError>,
    /// The certifier over a `(description, fit, window)` triple:
    /// [`OffsetFitLane::remap`]'s body, and [`OffsetFitLane::recertify`]'s
    /// on a surface's own triple.
    certify: fn(
        &geom::SurfaceDescription<T>,
        &NurbsSurface<T>,
        geom::ApproxWindow,
        Tol,
        Band,
    ) -> Result<geom::OffsetCertificate, OffsetFitError>,
}

impl OffsetFitLane<f64> {
    /// The `f64` fit — the whole inventory of this door, and the only
    /// constructor there is.
    #[must_use]
    pub const fn fit() -> Self {
        Self {
            mint: crate::offset_fit::approx_offset_surface,
            certify: crate::offset_fit::certify_offset_over,
        }
    }
}

impl<T: Real> OffsetFitLane<T> {
    /// Re-derives an approximating surface's certificate against its
    /// own stored description and fit, classified against `tol` — the
    /// tier-3 never-trust posture (O5), one dimension up from
    /// `EdgeCurve::recertify`.
    ///
    /// **The tolerance is the RUN's.** The edge machinery re-certifies
    /// every carrier against the run's band and never against a stored
    /// bound, and the surface claim is the same shape: O3 ratifies
    /// `sup ‖S_fit − (S + d·n)‖ ≤ ε_precision`, so verifying it means
    /// measuring against the ε this validation call runs at. The
    /// surface stores no tolerance, so there is no per-surface bound
    /// that could stand in for the ratified one.
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
        (self.certify)(
            approx.description(),
            approx.fit(),
            approx.window(),
            tol,
            band,
        )
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
    /// The classification tolerance is the run's ε and arrives as the
    /// witness, for [`OffsetFitLane::recertify`]'s reason: the map and
    /// the validator classify a given surface against the same number.
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
        tol: Tol,
        band: Band,
    ) -> Result<geom::OffsetCertificate, OffsetFitError> {
        (self.certify)(description, fit, window, tol, band)
    }
}

/// **The door's WIRING, field by field** — the row that says which
/// free function each limb of [`OffsetFitLane::fit`] is, rather than
/// what that function answered.
///
/// A row that compares outputs cannot see a door re-pointed at a
/// routine that agrees on the fixture in front of it — the neighbouring
/// `_at` instrument at the fixture's own tolerance agrees exactly on
/// the certify limb, and a same-signature closure can agree by
/// construction. The helper compares the stored function pointers
/// instead, so a re-point is a failure no matter what it computes.
///
/// Function-pointer identity is what `std::ptr::fn_addr_eq` compares
/// and is not a language guarantee (identical function bodies may be
/// merged), which costs nothing here: the two bodies differ, and a
/// false PASS from a merge would need the re-pointed routine to be
/// instruction-identical to the one it replaced.
///
/// The door is formed at `f64` alone — its one constructor is concrete
/// — so its helper is not generic and its roster is one row.
/// `topo`'s `certified_enclosure_impl_census` counts the door values
/// in the tree against its roster of helpers, this one included.
#[cfg(test)]
mod wiring_rows {
    use super::OffsetFitLane;

    /// `Ok(())` when every limb holds its routine; otherwise the name
    /// of the first field that does not.
    fn holds_the_offset_fit() -> Result<(), &'static str> {
        let lane = OffsetFitLane::fit();
        if !std::ptr::fn_addr_eq(
            lane.mint,
            crate::offset_fit::approx_offset_surface as fn(_, _, _, _) -> _,
        ) {
            return Err("mint is not `offset_fit::approx_offset_surface`");
        }
        if !std::ptr::fn_addr_eq(
            lane.certify,
            crate::offset_fit::certify_offset_over as fn(_, _, _, _, _) -> _,
        ) {
            return Err("certify is not `offset_fit::certify_offset_over`");
        }
        Ok(())
    }

    #[test]
    fn f64_is_wired_to_the_offset_fit() {
        assert_eq!(
            holds_the_offset_fit(),
            Ok(()),
            "`OffsetFitLane::<f64>::fit()` holds something other than its two routines"
        );
    }
}
