//! The tour's scalar abstraction (M4 PR 8b, D3): every scene builds
//! its geometry generically over [`Scalar`] so the SAME construction
//! code runs both the f64 tour and the K-telemetry Probe sweep
//! (`crate::probe`) — no duplicated scene data to drift.
//!
//! `f()` is the narration escape hatch: demo code asserts volume
//! oracles and formats captions in plain f64, which generic kernel
//! code must never do (the evaluation-code discipline routes every
//! decision through `Decide`). Demos are narration, not evaluation
//! code — and for `f64` and `Probe` the extraction is EXACT (`Probe`
//! is a transparent f64 wrapper; its arithmetic delegates verbatim),
//! so every oracle asserted at f64 holds bit-identically at Probe.
//! `Interval` builds scenes only for the rows that ask them for a
//! certificate, and no oracle reads its `f`.

use pncad::document::ContentBits;
use pncad::geom_core::Decide;
// Only the `Scalar for Probe` impl below names it; the recording scalar
// rides the `probe` feature (see this crate's manifest).
#[cfg(feature = "probe")]
use pncad::geom_core::Probe;

/// The K funnel name the tour's decided frame axes and cut directions
/// are decided under.
///
/// A frame whose axes a scene COMPUTED — a plane rolled about a stem,
/// a blade's section along its spine — or a split plane across a
/// computed direction has to be decided, and the mint asks for a funnel
/// name, which a demo, as an outside consumer, has to invent. One name
/// for the whole tour, because which scene asked is not what a K reader
/// is after. An axis-aligned sketch plane is not decided: it is a
/// written turn (`RigidMotion::turned`). The tube scenes' [`axis_frame`]
/// still decides an exact (axis, reference) pair under this name
/// (`work/frame/exact-axis-and-reference-frames-still-decide.md`).
pub const TOUR_FRAME_AXIS: &str = "tour_frame_axis";

/// **A sketch plane from two computed directions**, orthonormalized at
/// the run's band — a plane rolled about a stem, a blade's section
/// frame. The Gram–Schmidt mint IS a decision, so it wants a band and
/// [`TOUR_FRAME_AXIS`]; this is the tour writing that out once instead
/// of at every call site.
///
/// # Panics
///
/// If the band cannot be formed, or if the two directions span no
/// plane — every scene here computes a pair that does.
pub fn sketch_frame<S: Scalar>(
    origin: pncad::geom_core::Point3<S>,
    u: pncad::geom_core::Vec3<S>,
    v: pncad::geom_core::Vec3<S>,
    tol: pncad::geom_core::Tol,
) -> pncad::profile::SketchPlane<S> {
    pncad::profile::SketchPlane::from_frame(
        pncad::geom_core::RigidMotion::gram_schmidt(
            origin,
            u,
            v,
            TOUR_FRAME_AXIS,
            pncad::geom_core::Band::linear(tol).expect("the run's tolerance forms a band"),
        )
        .expect("the scene's two directions span a plane"),
    )
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
/// helper: one call to `RigidMotion::from_axis_and_reference`, wearing
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
) -> pncad::geom_core::RigidMotion<S> {
    pncad::geom_core::RigidMotion::from_axis_and_reference(
        center,
        axis,
        u_ref,
        TOUR_FRAME_AXIS,
        pncad::geom_core::Band::linear(tol).expect("the run's tolerance forms a band"),
    )
    .expect("the scene's spine axis has a direction and its reference radial is off it")
}

/// **A split plane through `origin` across the authored direction
/// `dir`**, the direction's length decided at the run's band.
///
/// `SplitPlane` takes a `UnitVec3`, and the kernel has no door from a
/// point and a direction, so a scene that cuts writes the mint — band,
/// funnel name and all — itself; this is the tour writing it once. The
/// friction is recorded at `work/issues/kernel-split-plane-has-no-direction-door.md`.
///
/// # Panics
///
/// If the band cannot be formed, or if `dir` has no length — every
/// scene here authors one that does.
pub fn split_plane<S: Scalar>(
    origin: pncad::geom_core::Point3<S>,
    dir: pncad::geom_core::Vec3<S>,
    tol: pncad::geom_core::Tol,
) -> pncad::topo::splitting::SplitPlane<S> {
    let band = pncad::geom_core::Band::linear(tol).expect("the run's tolerance forms a band");
    pncad::topo::splitting::SplitPlane {
        origin,
        normal: pncad::geom_core::UnitVec3::new(dir, TOUR_FRAME_AXIS, band)
            .expect("the scene's cut direction has a length"),
    }
}

/// A scalar the tour can build scenes at: kernel-decidable, certifying
/// (every scene is measured through the certified doors, so the bound
/// is the quadrature's own), document-evaluable (the heat-sink recipe),
/// and exactly f64-extractable for narration.
pub trait Scalar:
    Decide
    + pncad::geom_core::CertifiedBounds
    + ContentBits
    + pncad::topo::AtRestPolicy
    + Send
    + Sync
    + Copy
    + 'static
{
    /// The exact f64 value (narration/oracles only — never decisions).
    fn f(self) -> f64;
}

impl Scalar for f64 {
    fn f(self) -> f64 {
        self
    }
}

/// The certifying scalar, for the rows that ask a certificate of a
/// scene (`tests/wall_jets.rs`): its `f` is the midpoint, which is not
/// exact, so no narration or oracle reads a scene built at it.
impl Scalar for pncad::geom_core::Interval {
    fn f(self) -> f64 {
        use pncad::geom_core::Bounds;
        0.5 * self.lo() + 0.5 * self.hi()
    }
}

#[cfg(feature = "probe")]
impl Scalar for Probe {
    fn f(self) -> f64 {
        self.0
    }
}
