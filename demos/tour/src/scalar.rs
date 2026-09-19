//! The tour's scalar abstraction (M4 PR 8b, D3): every scene builds
//! its geometry generically over [`Scalar`] so the SAME construction
//! code runs both the f64 tour and the K-telemetry Probe sweep
//! (`crate::probe`) — no duplicated scene data to drift.
//!
//! `f()` is the narration escape hatch: demo code asserts volume
//! oracles and formats captions in plain f64, which generic kernel
//! code must never do (the evaluation-code discipline routes every
//! decision through `Decide`). Demos are narration, not evaluation
//! code — and for both implementors the extraction is EXACT (`Probe`
//! is a transparent f64 wrapper; its arithmetic delegates verbatim),
//! so every oracle asserted at f64 holds bit-identically at Probe.

use pncad::document::ContentBits;
use pncad::geom_core::{Bounds, Decide};
// Only the `Scalar for Probe` impl below names it; the recording scalar
// rides the `probe` feature (see this crate's manifest).
#[cfg(feature = "probe")]
use pncad::geom_core::Probe;

/// The K funnel name the tour's authored frame axes are decided under.
///
/// A scene that wants a sketch plane off the three world frames has to
/// mint the frame witness, and the mint asks for a funnel name — which
/// a demo, as an outside consumer, has to invent. One name for the
/// whole tour, because which scene asked is not what a K reader is
/// after.
pub const TOUR_FRAME_AXIS: &str = "tour_frame_axis";

/// **A sketch plane from two authored directions**, orthonormalized at
/// the run's band.
///
/// The library's decision-free frames are the three world ones
/// (`OrthoFrame::axes_xy` and its siblings); anything else — a plane
/// spanned by x̂ and ẑ, a plane rolled about a stem — goes through the
/// Gram–Schmidt mint, which IS a decision and so wants a band and a
/// funnel name. This is the tour writing that out once instead of at
/// every call site; `memories/demo-purpose.md` is why the friction is
/// recorded rather than hidden.
///
/// # Panics
///
/// If the band cannot be formed, or if the two directions span no
/// plane — every scene here authors a pair that does.
pub fn sketch_frame<S: Scalar>(
    origin: pncad::geom_core::Point3<S>,
    u: pncad::geom_core::Vec3<S>,
    v: pncad::geom_core::Vec3<S>,
    tol: pncad::geom_core::Tol,
) -> pncad::profile::SketchPlane<S> {
    pncad::profile::SketchPlane::from_frame(authored_frame(origin, u, v, tol))
}

/// **The frame witness itself**, for the scenes that place a body
/// rather than draw on a plane — a skin's section placement, a loft's
/// station. Same mint, same funnel name; see [`sketch_frame`].
///
/// # Panics
///
/// As [`sketch_frame`].
pub fn authored_frame<S: Scalar>(
    origin: pncad::geom_core::Point3<S>,
    u: pncad::geom_core::Vec3<S>,
    v: pncad::geom_core::Vec3<S>,
    tol: pncad::geom_core::Tol,
) -> pncad::geom_core::OrthoFrame<S> {
    pncad::geom_core::OrthoFrame::gram_schmidt(
        origin,
        u,
        v,
        TOUR_FRAME_AXIS,
        pncad::geom_core::Band::linear(tol).expect("the run's tolerance forms a band"),
    )
    .expect("the scene's two directions span a plane")
}

/// **A frame from a raw axis and a raw reference**: the tube doors'
/// spine and window radial, and equally a bud segment's lean or a
/// blade's spine — any scene that holds an axis it means and a
/// direction it wants the roll measured from.
///
/// The axis is decided first and KEPT — it is the frame's `w`, stored
/// verbatim — and the reference yields whatever component of it lies
/// along the axis, so a scene states the roll it means rather than a
/// vector it has had to make perpendicular by hand. That is the whole
/// helper: one call to `OrthoFrame::from_axis_and_reference`, wearing
/// the tour's band and funnel name.
///
/// # Panics
///
/// If the band cannot be formed, if the axis has no direction, or if
/// the reference lies along it.
pub fn axis_frame<S: Scalar>(
    center: pncad::geom_core::Point3<S>,
    axis: pncad::geom_core::Vec3<S>,
    u_ref: pncad::geom_core::Vec3<S>,
    tol: pncad::geom_core::Tol,
) -> pncad::geom_core::OrthoFrame<S> {
    pncad::geom_core::OrthoFrame::from_axis_and_reference(
        center,
        axis,
        u_ref,
        TOUR_FRAME_AXIS,
        pncad::geom_core::Band::linear(tol).expect("the run's tolerance forms a band"),
    )
    .expect("the scene's spine axis has a direction and its reference radial is off it")
}

/// A scalar the tour can build scenes at: kernel-decidable, document-
/// evaluable (the heat-sink recipe), and exactly f64-extractable for
/// narration.
pub trait Scalar:
    Decide + Bounds + ContentBits + pncad::topo::AtRestPolicy + Send + Sync + Copy + 'static
{
    /// The exact f64 value (narration/oracles only — never decisions).
    fn f(self) -> f64;
}

impl Scalar for f64 {
    fn f(self) -> f64 {
        self
    }
}

#[cfg(feature = "probe")]
impl Scalar for Probe {
    fn f(self) -> f64 {
        self.0
    }
}
