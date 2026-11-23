//! **The datum conventions a stored analytic datum must lie inside**, as
//! quantities: the margin type both representability doors return
//! ([`crate::Surface::representability_margins`],
//! [`crate::Curve3::representability_margins`]). A carrier's frame has
//! no margin: it is a [`crate::Frame`], rigid by its type.
//!
//! Nothing here decides. Each margin is strictly positive exactly when
//! its datum is inside that end of its convention; whether it is, and
//! with what posture, is the consumer's question (`topo`'s tier-3
//! check 1 reads each one's lower bracket end against zero).

use geom_core::{Real, Vec3};

use crate::SurfaceDatum;

/// One representability margin: the datum it bounds, which end of its
/// value's convention, and the margin itself — strictly positive
/// exactly when the datum is inside that end. `D` is [`SurfaceDatum`] for a surface and
/// [`crate::CurveDatum`] for a carrier.
#[derive(Clone, Copy, Debug)]
pub struct RepresentabilityMargin<T, D = SurfaceDatum> {
    /// The datum the margin bounds.
    pub datum: D,
    /// Which end of the datum's convention the margin measures.
    pub end: ConventionEnd,
    /// The margin itself, at the datum's scalar.
    pub margin: T,
}

/// **A stored frame squared** ([`crate::Frame::square`]): the unit axis
/// along a stored `axis`, and the unit seam square to it
/// toward a stored `u_ref`.
/// The one home for a reader that needs the orthonormal frame a datum
/// is only by convention (nothing margins a cone's frame at rest).
///
/// The seam is taken as `a × (u_ref × a)` over the unit `a`, which
/// stands square to `a` to the rounding of its own size wherever the
/// seam leans: `u_ref − a·(a·u_ref)` keeps a component along `a` of
/// `u_ref`'s size times the rounding, which the renormalisation
/// amplifies by `1/width` (98u at `u_ref·a = 0.99`). Here each
/// component of the outer cross product is off by `2u` of its two
/// products, which puts the result's component along `a` under `2√3·u`
/// of `‖u_ref × a‖`; the inner product's own error tips that product
/// off square to `a` by under `2√3·u/(width − 4u)`, which shrinks the
/// result by under 5% where `width ≥ 16u`. That is a tilt under `3.7u`
/// off the unit `a`, which stands under `u` off the stored axis'
/// direction, and normalising the result adds `u`: under `6u` off the
/// stored axis where `width ≥ 16u` (measured: under `2u` at every width
/// down to `u`).
///
/// `width` is `‖u_ref × a‖/‖u_ref‖`, the sine of the stored seam's
/// angle off the axis. A carrier's frame is a `RigidMotion`, whose seam
/// is square to its axis by its mint, so on a stored carrier `width` is
/// one to rounding and the bounds above hold with room; the squaring
/// stays because it is the arithmetic the readers' pins were taken on.
#[derive(Clone, Copy, Debug)]
pub struct SquareFrame<T: Real> {
    /// The unit axis.
    pub axis: Vec3<T>,
    /// The unit seam square to `axis`.
    pub u_ref: Vec3<T>,
    /// The sine of the stored seam's angle off the axis.
    pub width: T,
}

/// Which end of a datum's convention a margin measures — the refusal's
/// way of saying TOO SMALL (a radius of zero, a cone closed to a line)
/// from TOO LARGE (a cone opened to a plane).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// Every value, for `topo`'s samples (this crate's `test-support`
// feature, test builds only).
#[cfg_attr(feature = "test-support", derive(strum::EnumIter))]
pub enum ConventionEnd {
    /// The quantity must lie above this end (`radius > 0`,
    /// `half_angle > 0`).
    Lower,
    /// The quantity must lie below this end (`half_angle < π/2`).
    Upper,
}

/// A value convention's margin at its lower end: the datum itself,
/// positive exactly when the datum is.
pub(crate) fn lower<T, D>(datum: D, margin: T) -> RepresentabilityMargin<T, D> {
    RepresentabilityMargin {
        datum,
        end: ConventionEnd::Lower,
        margin,
    }
}
