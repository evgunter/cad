//! The viewport camera: one state value, one typed operation
//! vocabulary, one pure `apply`, and the projection algebra over it.
//!
//! # The state
//!
//! [`Camera`] is a **turntable**: a target point the view orbits, a
//! distance from it, an azimuth and an elevation, a vertical field of
//! view, and the scene radius the framing was taken against. World
//! coordinates are the kernel's, and +Z is up — the sketch-plane
//! convention `SketchPlane::xy()` establishes and every demo scene
//! inherits.
//!
//! The fields are private because they carry invariants that must
//! hold at every reachable state, not merely at construction:
//! everything is finite, the distance lies inside the scene-derived
//! zoom band, and `|pitch| < π/2` strictly, so the view direction is
//! never parallel to world up and the frame is always well defined.
//! [`apply`] is the only way to move a camera and it re-establishes
//! all three.
//!
//! # The operations
//!
//! [`CameraOp`] is the whole vocabulary — orbit, pan, dolly, frame —
//! and [`apply`] the whole implementation. Both are renderer-free:
//! nothing here knows what a widget or a pixel is, which is what lets
//! the invariants above be tested by replaying operation sequences in
//! headless CI (G1's testability rule).
//!
//! Operations carry plain `f64` and are therefore constructible with
//! values that are not navigation moves at all (a NaN drag, a
//! zero-scale dolly). Those are **refused typed** by [`apply`], never
//! clamped and never silently dropped: a caller folding user input
//! gets a [`CameraOpError`] it can show, and the camera it already had.
//!
//! # The one free transform, and the one door that does not refuse
//!
//! [`cursor_projection`] is about no camera state at all — a matrix, a
//! cursor and a viewport in, a matrix out — and it is here because
//! projection algebra is this module's subject.
//!
//! **It takes `f32` where the rest of this module is `f64`, and that
//! is deliberate rather than a gap.** The matrix it transforms has to
//! be the one the GPU is actually rasterizing with, which is the
//! narrowed one ([`Camera::view_projection_f32`]): handing it the
//! `f64` original would have the id pass compute with a matrix the
//! shaded pass does not have, which is precisely the divergence
//! `crate::idpass::compare` exists to report. The narrowing
//! itself is `crate::narrowing`'s and is spelled nowhere here.
//!
//! **It is also the one door here that answers for every input**, and
//! the refusal discipline above is not weakened by it: everything it
//! takes has already been refused or vouched for one step out. A
//! component that is not a finite `f32` cannot reach it, because
//! `crate::narrowing::Narrow` is what produced every one of them; a
//! viewport with no area cannot, because `ViewportSize::ndc_of`
//! answers `None` first and no query is built. What is left for a
//! `Result` to carry is nothing, and allocating one per hovered frame
//! to carry nothing is the cost this declines.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use bvh::{Aabb, Axis};
use pncad::geom_core::{Point3, Vec3};
use pncad::select::Ray;

use crate::narrowing::Narrow;
use crate::readout;

/// Elevation is held strictly inside `±(π/2 − POLE_MARGIN)`.
///
/// At exactly ±π/2 the view direction is world up and the camera
/// frame degenerates. A margin rather than an epsilon-free clamp
/// keeps `cos(pitch)` bounded away from zero, so the derived right
/// vector stays numerically well conditioned at the extremes.
const POLE_MARGIN: f64 = 1e-3;

/// The closest the camera may dolly, as a multiple of the scene
/// radius.
const MIN_DISTANCE_FACTOR: f64 = 0.05;

/// The furthest the camera may dolly, as a multiple of the scene
/// radius.
const MAX_DISTANCE_FACTOR: f64 = 100.0;

/// The near plane's distance, as a multiple of the eye's distance to
/// its target.
///
/// Tied to the eye rather than to the scene, because the scene is
/// not the only thing that may be drawn: a plane running out toward the
/// horizon and back under the eye, and at a grazing view its nearest
/// visible point sits a vanishing fraction of the orbit distance in
/// front of the eye. A near plane at the front of the scene's sphere
/// cut such a plane down to the one band of it inside the sphere.
/// This small is affordable because the depth mapping is reversed
/// ([`Camera::projection_matrix`]), where a float depth buffer's
/// resolution is relative at every depth and does not collapse as the
/// near plane approaches the eye.
const NEAR_FACTOR: f64 = 1e-3;

/// The smallest scene radius a camera holds: below it the zoom band's
/// floor ([`band_floor`]) is not a length.
const MIN_SCENE_RADIUS: f64 = f64::MIN_POSITIVE;

/// The radius a fit frames when the picture gives it none — nothing
/// drawn, or a drawn extent whose radius is below [`MIN_SCENE_RADIUS`]
/// (a lone point). Metres.
pub const DEFAULT_FRAME_RADIUS: f64 = 0.05;

/// The default vertical field of view: 45°, the CAD-conventional
/// middle ground between the foreshortening of a wide lens and the
/// flatness of a narrow one.
const DEFAULT_FOV_Y: f64 = std::f64::consts::FRAC_PI_4;

/// The default framing direction: a three-quarter view from above,
/// the orientation an isometric-ish CAD default takes.
const DEFAULT_YAW: f64 = -std::f64::consts::FRAC_PI_3;
/// Elevation of the default framing (30° above the horizon).
const DEFAULT_PITCH: f64 = std::f64::consts::FRAC_PI_6;

/// How much slack [`CameraOp::Frame`] leaves around the scene's
/// bounding sphere, as a multiple of its radius.
const FRAMING_MARGIN: f64 = 1.15;

/// A turntable camera: the authoritative navigation state.
///
/// Construct with [`Camera::framing`] (fit an extent) or
/// [`Camera::new`] (state given explicitly); move with [`apply`].
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    target: Point3<f64>,
    distance: f64,
    yaw: f64,
    pitch: f64,
    fov_y: f64,
    scene_radius: f64,
}

/// Equality is on the state, coordinate by coordinate.
///
/// Written out rather than derived because `Point3` carries no
/// `PartialEq` — the kernel's linalg types deliberately do not offer
/// one, since comparing geometry is a tolerance question there. Here
/// the subject is a *camera*, not geometry: two cameras are the same
/// camera when they are in the same state, and that is a plain
/// comparison of the numbers.
impl PartialEq for Camera {
    fn eq(&self, other: &Self) -> bool {
        // Every number equality is on, in declaration order — read by
        // ONE pattern that both sides go through, because a census
        // stated twice is a census that can disagree with itself.
        //
        // **Destructured rather than field-read.** A field added to
        // `Camera` is E0027 in this pattern, so it cannot land outside
        // equality without an author deciding it should be there — and
        // a second pattern carries `Point3`'s three coordinates for the
        // same reason, since expanding `target` by hand is where the
        // census would otherwise stop at this crate's boundary. The tie
        // is worth more here than in a dump: an `eq` that misses a
        // field answers *wrong*, it does not merely say less. That tie
        // is also why this reads the fields rather than the six public
        // accessors below: an accessor call is a field READ, so a
        // seventh field would leave a census built from them silently
        // short, which is the whole property being bought.
        //
        // Nested in its only caller rather than sited in `impl Camera`:
        // the helper exists for `eq` alone, and a second inherent block
        // would leave a reader of `impl Camera` unable to see the type's
        // surface in one place.
        fn coordinates(camera: &Camera) -> [f64; 8] {
            let &Camera {
                target,
                distance,
                yaw,
                pitch,
                fov_y,
                scene_radius,
            } = camera;
            let Point3 { x, y, z } = target;
            [x, y, z, distance, yaw, pitch, fov_y, scene_radius]
        }
        coordinates(self) == coordinates(other)
    }
}

/// [`Camera::framing`] of nothing drawn at a square aspect, as a value
/// with no refusal to carry: what a window holds before a pane has an
/// aspect to fit against.
impl Default for Camera {
    fn default() -> Self {
        Self {
            target: Point3::new(0.0, 0.0, 0.0),
            distance: stand_off(DEFAULT_FRAME_RADIUS, DEFAULT_FOV_Y, 1.0),
            yaw: DEFAULT_YAW,
            pitch: DEFAULT_PITCH,
            fov_y: DEFAULT_FOV_Y,
            scene_radius: DEFAULT_FRAME_RADIUS,
        }
    }
}

/// A camera state that is not a camera (closed enum, D4 ¶3).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CameraError {
    /// A coordinate, angle, distance or radius was not finite.
    NotFinite {
        /// Which of the constructor's arguments.
        what: &'static str,
        /// The offending value.
        value: f64,
    },
    /// The scene radius handed to [`Camera::new`] was not strictly
    /// positive. The framing doors never raise it: an extent too small
    /// to hold a zoom band is framed at [`DEFAULT_FRAME_RADIUS`].
    DegenerateScene {
        /// The caller's radius.
        radius: f64,
    },
    /// The scene radius was a length, but so large that the top of
    /// the zoom band it derives is not an `f64` — so this camera has
    /// no furthest distance to dolly to.
    ///
    /// **Reachable through [`Camera::new`] only.** [`Camera::framing`]
    /// and [`Camera::fitted`] take their radius from `sphere`, which
    /// sums three squared half-extents and refuses a non-finite sum,
    /// so the largest radius they can carry is about `1.3e154` and its
    /// band is an ordinary number. `Camera::new` takes a radius
    /// directly, and is public: a caller holding its own scene bound
    /// reaches this in one call.
    ///
    /// Its own arm because it is a different fact from
    /// [`CameraError::NotFinite`] and from
    /// [`CameraError::DegenerateScene`]: this radius IS finite and
    /// strictly positive, and every other guard in the constructor
    /// takes it. What refuses it is the band rather than the radius —
    /// [`Camera::max_distance`] is the radius times
    /// [`MAX_DISTANCE_FACTOR`], and past `f64::MAX` divided by that
    /// factor there is no furthest distance, so
    /// [`clamp_distance`] would bound the camera's distance
    /// above by nothing and a dolly out would have no stop.
    SceneRadiusOverflowsZoomBand {
        /// The offending value, in world units.
        scene_radius: f64,
    },
    /// The field of view was not strictly inside `(0, π)`.
    FieldOfViewOutOfRange {
        /// The offending value, in radians.
        fov_y: f64,
    },
    /// A framing input that cannot produce a camera: a bounding box
    /// that carried a bound that is not a finite number (NaN is `Aabb`'s
    /// poison state) or was inverted — a minimum above its maximum —
    /// **or** a viewport aspect at or below zero, or a viewport with a
    /// side of zero pixels or fewer.
    ///
    /// One arm for both inputs, deliberately, and the name is the older
    /// of the two: both are "the framing request names no view". What
    /// a caller can read off the door that returned it:
    ///
    /// - [`Camera::projection_matrix`] takes only an aspect, so here
    ///   the arm is an aspect at or below zero — which is how a pane of
    ///   infinite height reaches it (its width over `inf` is `0.0`).
    /// - [`Camera::ray_through`] and [`crate::screen::view_of`] take
    ///   only a viewport, and refuse a non-finite side as
    ///   [`CameraError::NotFinite`] first, so here the arm is a side of
    ///   zero pixels or fewer.
    /// - [`Camera::fitted`], and [`apply`] of a [`CameraOp::Frame`]
    ///   (which returns it wrapped as [`CameraOpError::Unframeable`]),
    ///   take both and check the aspect first: a non-finite aspect is
    ///   [`CameraError::NotFinite`], one at or below zero is this arm
    ///   with the box unchecked, and under a positive aspect this arm
    ///   is the box.
    /// - [`Camera::framing`] checks the box first, so here the arm is
    ///   the box unless the aspect is at or below zero, and then it is
    ///   the box or the aspect.
    UnusableBounds,
    /// The view-projection this camera and viewport give does not
    /// narrow to the `f32` a GPU matrix holds
    /// ([`crate::narrowing::Narrow`]).
    ///
    /// **Every entry of it is an ordinary finite `f64`**, which is why
    /// it is not [`CameraError::NotFinite`]: the algebra succeeded and
    /// the seam is what refuses. `f32::MAX` is about `3.40e38`, so a
    /// scene framed a few dozen orders of magnitude out gives a matrix
    /// this module is happy with and a GPU cannot be handed.
    UndrawableProjection,
    /// The distance needed to fit the scene at this aspect lies beyond
    /// the scene-derived zoom band, so no camera in the band contains
    /// the scene.
    ///
    /// The stand-off grows as `1/sin(half)` of the smaller half-angle,
    /// so this is reachable two ways: a viewport far narrower than it
    /// is tall, where the horizontal half-angle binds, and a camera
    /// whose field of view is itself too narrow ([`Camera::new`] takes
    /// any `fov_y` in `(0, π)`), where the vertical one binds at every
    /// aspect. **Refused rather than clamped** — a clamped "fit" is a
    /// camera that silently does not contain its scene, which is the
    /// one answer [`Camera::fitted`]'s own contract must never give.
    Unfittable {
        /// The stand-off the fit needed.
        required: f64,
        /// The furthest the zoom band allows.
        max_distance: f64,
        /// The viewport aspect (width / height) the fit was asked at.
        aspect: f64,
        /// The camera's vertical field of view, radians.
        fov_y: f64,
    },
}

/// Every number a sentence quotes is spelled by [`readout::number`]: a
/// refused field is whatever `f64` the caller handed, and plain
/// `Display` writes one at up to seventeen figures, or as three hundred
/// digits of decimal expansion near `f64::MAX` — a sentence nobody can
/// read, about a number nobody can read.
impl core::fmt::Display for CameraError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotFinite { what, value } => write!(
                f,
                "the camera's {what} is {}, which is not a finite number",
                readout::number(*value)
            ),
            Self::DegenerateScene { radius } => write!(
                f,
                "a scene radius of {} is not a positive extent to frame against",
                readout::number(*radius)
            ),
            Self::SceneRadiusOverflowsZoomBand { scene_radius } => write!(
                f,
                "a scene radius of {} is past the largest this viewer can frame: the \
                 furthest distance its zoom band allows is not a finite number",
                readout::number(*scene_radius)
            ),
            Self::FieldOfViewOutOfRange { fov_y } => write!(
                f,
                "a vertical field of view of {} rad is not strictly inside (0, pi)",
                readout::number(*fov_y)
            ),
            Self::UndrawableProjection => f.write_str(
                "the view projection is past the largest number a GPU matrix can hold \
                 (about 3.4e38), so there is nothing to draw the picture with",
            ),
            Self::UnusableBounds => f.write_str(
                "the framing request names no view — the box to fit has a minimum above its \
                 maximum or a bound that is not a finite number, or the viewport aspect is \
                 not a positive number",
            ),
            // The cause and the remedy. The stand-off and the band stay
            // in the value for a caller that wants them; they are not
            // numbers a reader can act on.
            Self::Unfittable { fov_y, .. } if lens_cannot_fit(*fov_y) => f.write_str(
                "the camera's field of view is too narrow to fit the picture; widen the field \
                 of view",
            ),
            // The lens alone fits, so the horizontal half-angle is the
            // one that binds and the pane is taller than it is wide. A
            // ratio is said only where it reads as one.
            Self::Unfittable { aspect, .. } => match aspect.recip() {
                tall if (2.0..1.0e6).contains(&tall) => write!(
                    f,
                    "the viewport is about {tall:.0} times taller than it is wide, too \
                     narrow to fit the picture; widen it"
                ),
                _ => f.write_str(
                    "the viewport is too narrow for its height to fit the picture; widen it",
                ),
            },
        }
    }
}

impl core::error::Error for CameraError {}

/// A move on a [`Camera`]: the whole navigation vocabulary.
///
/// Angles are radians, lengths are world units (the kernel's meters),
/// and every field is a *delta* except [`CameraOp::Frame`], which is
/// absolute by nature.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CameraOp {
    /// Turntable rotation about the target.
    Orbit {
        /// Azimuth delta, radians, positive counterclockwise seen
        /// from +Z.
        yaw: f64,
        /// Elevation delta, radians, positive toward +Z. Clamped at
        /// the poles rather than refused: a drag that runs past the
        /// top saturates, which is what a turntable does.
        pitch: f64,
    },
    /// Translation of the target in the view plane, in world units
    /// along the camera's own right and up axes.
    Pan {
        /// Along the camera's right axis.
        right: f64,
        /// Along the camera's up axis.
        up: f64,
    },
    /// Multiplicative change of the viewing distance; the target does
    /// not move. Clamped into the scene-derived zoom band.
    Dolly {
        /// Strictly positive; below 1 moves the eye toward the target.
        factor: f64,
    },
    /// Re-frame on an extent: recentre, re-derive the scene radius
    /// and the zoom band, and back off far enough that the bounding
    /// sphere fits both half-angles. Orientation is kept — a fit is
    /// not a reset.
    Frame {
        /// The box to fit, or `None` when nothing is drawn
        /// ([`Camera::fitted`] says what that frames).
        extent: Option<Aabb>,
        /// Viewport aspect ratio (width / height); the horizontal
        /// half-angle is the narrower one on a tall viewport.
        aspect: f64,
    },
}

/// An operation that is not a move (closed enum, D4 ¶3).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CameraOpError {
    /// An operation field was not finite.
    NotFinite {
        /// Which field.
        what: &'static str,
        /// The offending value.
        value: f64,
    },
    /// A dolly factor that was zero or negative: scaling a distance
    /// by it does not produce a viewing distance.
    NonPositiveDolly {
        /// The offending factor.
        factor: f64,
    },
    /// A [`CameraOp::Frame`] whose extent or aspect could not produce
    /// a camera.
    Unframeable(CameraError),
}

/// Numbers are spelled as [`CameraError`]'s are, by [`readout::number`].
impl core::fmt::Display for CameraOpError {
    /// The [`CameraOpError::Unframeable`] arm forwards to
    /// [`CameraError`]'s own `Display`: the framing layer named that
    /// failure and this layer does not restate it.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotFinite { what, value } => write!(
                f,
                "the operation's {what} is {}, which is not a finite number",
                readout::number(*value)
            ),
            Self::NonPositiveDolly { factor } => write!(
                f,
                "a dolly factor of {} is not a positive scale for a viewing distance",
                readout::number(*factor)
            ),
            Self::Unframeable(error) => write!(f, "{error}"),
        }
    }
}

impl core::error::Error for CameraOpError {}

/// The vocabulary in prose, for the status line that reports which
/// move was refused. Deltas are the operation's own units — radians
/// for angles, world units for lengths — spelled by [`readout::number`],
/// since a refused move's finite fields sit beside the one that is not.
impl core::fmt::Display for CameraOp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let n = |value: &f64| readout::number(*value);
        match self {
            Self::Orbit { yaw, pitch } => {
                write!(f, "orbit by yaw {} rad, pitch {} rad", n(yaw), n(pitch))
            }
            Self::Pan { right, up } => write!(f, "pan by right {}, up {}", n(right), n(up)),
            Self::Dolly { factor } => write!(f, "dolly by a factor of {}", n(factor)),
            // The button's name, and neither field: the error this
            // sentence rides beside says what was wrong with the extent
            // or the viewport's shape.
            Self::Frame { .. } => f.write_str("zoom to fit"),
        }
    }
}

impl Camera {
    /// A camera from explicit state.
    ///
    /// **Two arguments are normalised rather than refused, and both
    /// are normalised the way the operation that moves them does:**
    /// `pitch` is clamped to the pole margin, exactly as
    /// [`CameraOp::Orbit`] clamps it, and `distance` is clamped into
    /// the scene-derived zoom band, exactly as [`CameraOp::Dolly`]
    /// clamps it. A camera built here is therefore in the same state
    /// space a camera reached by navigating is, which is what lets
    /// every invariant in this module be stated over *reachable*
    /// states rather than over constructed ones.
    ///
    /// The remaining arguments are taken as given and refused if they
    /// are not usable. A caller who needs a distance *honoured* rather
    /// than clamped — [`Camera::fitted`] is the one such caller —
    /// checks the band itself and refuses; see
    /// [`CameraError::Unfittable`].
    ///
    /// # Errors
    ///
    /// [`CameraError::NotFinite`] for any non-finite argument,
    /// [`CameraError::DegenerateScene`] for a non-positive scene
    /// radius, [`CameraError::SceneRadiusOverflowsZoomBand`] for one
    /// that is a length but has no furthest distance, and
    /// [`CameraError::FieldOfViewOutOfRange`] for a field of view
    /// outside `(0, π)`.
    ///
    /// **The scene radius is asked two questions and they are
    /// different ones.** `is_finite` is about the caller's number; the
    /// band is about the product this camera derives from it, and a
    /// radius above `f64::MAX` divided by [`MAX_DISTANCE_FACTOR`]
    /// passes the first and fails the second. Asking only the first
    /// left `distance` clamped into `..=inf` — a dolly out with no
    /// stop, from a door whose own guard was finiteness.
    pub fn new(
        target: Point3<f64>,
        distance: f64,
        yaw: f64,
        pitch: f64,
        fov_y: f64,
        scene_radius: f64,
    ) -> Result<Self, CameraError> {
        finite("target.x", target.x)?;
        finite("target.y", target.y)?;
        finite("target.z", target.z)?;
        finite("distance", distance)?;
        finite("yaw", yaw)?;
        finite("pitch", pitch)?;
        finite("fov_y", fov_y)?;
        finite("scene_radius", scene_radius)?;
        if scene_radius < MIN_SCENE_RADIUS {
            return Err(CameraError::DegenerateScene {
                radius: scene_radius,
            });
        }
        if !band_ceiling(scene_radius).is_finite() {
            return Err(CameraError::SceneRadiusOverflowsZoomBand { scene_radius });
        }
        if fov_y <= 0.0 || fov_y >= std::f64::consts::PI {
            return Err(CameraError::FieldOfViewOutOfRange { fov_y });
        }
        Ok(Self {
            target,
            distance: clamp_distance(distance, scene_radius),
            yaw: wrap_angle(yaw),
            pitch: clamp_pitch(pitch),
            fov_y,
            scene_radius,
        })
    }

    /// The default three-quarter view fitted to `extent`, read as
    /// [`Camera::fitted`] reads it.
    ///
    /// # Errors
    ///
    /// [`CameraError::UnusableBounds`] for an inverted or non-finite box or
    /// a non-positive aspect, [`CameraError::NotFinite`] for a
    /// non-finite aspect or a box whose radius overflows, and
    /// [`CameraError::Unfittable`] for an aspect at which no camera in
    /// the zoom band contains the extent.
    pub fn framing(extent: Option<Aabb>, aspect: f64) -> Result<Self, CameraError> {
        let (centre, radius) = sphere(extent)?;
        let camera = Self::new(
            centre,
            radius * MAX_DISTANCE_FACTOR,
            DEFAULT_YAW,
            DEFAULT_PITCH,
            DEFAULT_FOV_Y,
            radius,
        )?;
        camera.fitted(extent, aspect)
    }

    /// This camera re-centred and backed off to fit `extent`, keeping
    /// its orientation and field of view.
    ///
    /// **An extent too small to frame is framed at a default.** `None`
    /// — nothing drawn — frames the world origin at
    /// [`DEFAULT_FRAME_RADIUS`], and a box whose radius is below
    /// [`MIN_SCENE_RADIUS`] (a lone point) frames its centre at that
    /// radius. Every other box is fitted on its own bounding sphere.
    ///
    /// **The postcondition is containment**: every point of `extent`
    /// projects inside the frustum at this `aspect`. The one input
    /// that can make containment unreachable is a viewport far
    /// narrower than it is tall — the horizontal half-angle binds and
    /// the required stand-off grows as `1/sin(half)` until it leaves
    /// the zoom band. That case is [`CameraError::Unfittable`], not a
    /// clamped near-miss: this door either fits or refuses, and the
    /// caller's recourse is a wider pane or a narrower field of view.
    ///
    /// # Errors
    ///
    /// As [`Camera::framing`], plus [`CameraError::Unfittable`].
    pub fn fitted(&self, extent: Option<Aabb>, aspect: f64) -> Result<Self, CameraError> {
        finite("aspect", aspect)?;
        if aspect <= 0.0 {
            return Err(CameraError::UnusableBounds);
        }
        let (centre, radius) = sphere(extent)?;
        let distance = stand_off(radius, self.fov_y, aspect);
        // The band's FLOOR can never bind here — `sin(half) <= 1`, so
        // `distance >= radius * FRAMING_MARGIN`, which is above
        // `radius * MIN_DISTANCE_FACTOR` for every scene. Only the
        // ceiling is reachable, and reaching it means no camera in the
        // band contains the scene.
        let max_distance = band_ceiling(radius);
        if distance > max_distance {
            return Err(CameraError::Unfittable {
                required: distance,
                max_distance,
                aspect,
                fov_y: self.fov_y,
            });
        }
        Self::new(centre, distance, self.yaw, self.pitch, self.fov_y, radius)
    }

    /// The point the view orbits.
    pub fn target(&self) -> Point3<f64> {
        self.target
    }

    /// The eye's distance from the target.
    pub fn distance(&self) -> f64 {
        self.distance
    }

    /// Azimuth, radians, in `[−π, π)`.
    pub fn yaw(&self) -> f64 {
        self.yaw
    }

    /// The elevation limit: `|pitch|` never exceeds it, at any
    /// reachable state.
    ///
    /// Public because it is a *contract*, and a test that restates it
    /// as a literal is a hand-synced copy of a private constant — the
    /// defect this accessor exists to remove. One home; read it.
    pub fn pitch_limit() -> f64 {
        std::f64::consts::FRAC_PI_2 - POLE_MARGIN
    }

    /// Elevation, radians, strictly inside `±(π/2 − margin)`.
    pub fn pitch(&self) -> f64 {
        self.pitch
    }

    /// Vertical field of view, radians.
    pub fn fov_y(&self) -> f64 {
        self.fov_y
    }

    /// The radius of the bounding sphere this camera was framed
    /// against — the scale everything else here is relative to.
    pub fn scene_radius(&self) -> f64 {
        self.scene_radius
    }

    /// The closest this camera may dolly.
    pub fn min_distance(&self) -> f64 {
        band_floor(self.scene_radius)
    }

    /// The furthest this camera may dolly, always a finite distance
    /// ([`CameraError::SceneRadiusOverflowsZoomBand`]).
    pub fn max_distance(&self) -> f64 {
        band_ceiling(self.scene_radius)
    }

    /// The eye position.
    pub fn eye(&self) -> Point3<f64> {
        let d = self.direction_to_eye();
        Point3::new(
            self.target.x + d.x * self.distance,
            self.target.y + d.y * self.distance,
            self.target.z + d.z * self.distance,
        )
    }

    /// The unit vector from the target toward the eye.
    pub fn direction_to_eye(&self) -> Vec3<f64> {
        let (sp, cp) = self.pitch.sin_cos();
        let (sy, cy) = self.yaw.sin_cos();
        Vec3::new(cp * cy, cp * sy, sp)
    }

    /// The unit view direction: from the eye toward the target.
    pub fn forward(&self) -> Vec3<f64> {
        let d = self.direction_to_eye();
        Vec3::new(-d.x, -d.y, -d.z)
    }

    /// The camera's right axis (screen +x), unit length.
    ///
    /// `forward × world_up`, which is well defined for every
    /// reachable pitch because `|pitch| < π/2` strictly.
    pub fn right(&self) -> Vec3<f64> {
        let (sy, cy) = self.yaw.sin_cos();
        // forward × (0,0,1), simplified: the pitch factors cancel
        // under normalization.
        Vec3::new(-sy, cy, 0.0)
    }

    /// The camera's up axis (screen +y), unit length: `right ×
    /// forward`.
    pub fn up(&self) -> Vec3<f64> {
        self.right().cross(self.forward())
    }

    /// The near plane distance: [`NEAR_FACTOR`] of the orbit
    /// distance. There is no far plane
    /// ([`Camera::projection_matrix`]).
    pub fn near(&self) -> f64 {
        self.distance * NEAR_FACTOR
    }

    /// The world→view matrix, column-major (the layout WGSL's
    /// `mat4x4<f32>` reads).
    pub fn view_matrix(&self) -> [[f64; 4]; 4] {
        let f = self.forward();
        let r = self.right();
        let u = self.up();
        let eye = self.eye();
        let dot = |v: Vec3<f64>| v.x * eye.x + v.y * eye.y + v.z * eye.z;
        [
            [r.x, u.x, -f.x, 0.0],
            [r.y, u.y, -f.y, 0.0],
            [r.z, u.z, -f.z, 0.0],
            [-dot(r), -dot(u), dot(f), 1.0],
        ]
    }

    /// The view→clip matrix for `aspect` (width / height),
    /// column-major: a perspective projection with **reversed depth
    /// and no far plane**.
    ///
    /// Depth is `near / distance-in-front-of-the-eye`: 1 on the near
    /// plane, falling toward 0 at infinity, so NEARER is GREATER and
    /// every depth pipeline compares `Greater` and clears to 0
    /// (`crate::gpu`). The reversal is what a `Depth32Float` buffer's
    /// resolution needs: a float's quanta are dense near 0, which is
    /// where this puts the far field, so resolution stays roughly
    /// relative to depth everywhere instead of being spent next to the
    /// near plane. And with no far plane nothing drawn is ever clipped
    /// for being far — a plane receding to its horizon included.
    ///
    /// # Errors
    ///
    /// [`CameraError::NotFinite`] for a non-finite aspect and
    /// [`CameraError::UnusableBounds`] for a non-positive one — a
    /// viewport of zero width or height has no projection.
    pub fn projection_matrix(&self, aspect: f64) -> Result<[[f64; 4]; 4], CameraError> {
        finite("aspect", aspect)?;
        if aspect <= 0.0 {
            return Err(CameraError::UnusableBounds);
        }
        let t = 1.0 / (self.fov_y * 0.5).tan();
        Ok([
            [t / aspect, 0.0, 0.0, 0.0],
            [0.0, t, 0.0, 0.0],
            [0.0, 0.0, 0.0, -1.0],
            [0.0, 0.0, self.near(), 0.0],
        ])
    }

    /// `projection · view`, column-major.
    ///
    /// # Errors
    ///
    /// As [`Camera::projection_matrix`].
    pub fn view_projection(&self, aspect: f64) -> Result<[[f64; 4]; 4], CameraError> {
        Ok(mul(&self.projection_matrix(aspect)?, &self.view_matrix()))
    }

    /// **The same matrix, at the display seam**:
    /// [`Camera::view_projection`] narrowed to the `f32` a GPU holds.
    ///
    /// Here rather than at the caller because the caller is a driver
    /// and this is the algebra's own output type meeting the
    /// renderer's — and because [`cursor_projection`] takes exactly
    /// this value, so the module that owns the transform also
    /// produces the thing it transforms. The narrowing is
    /// [`crate::narrowing::Narrow`]'s single test and is not
    /// re-decided here.
    ///
    /// # Errors
    ///
    /// As [`Camera::view_projection`], plus
    /// [`CameraError::UndrawableProjection`] when the matrix is a
    /// projection this module can form and a GPU cannot hold.
    pub fn view_projection_f32(&self, aspect: f64) -> Result<[[f32; 4]; 4], CameraError> {
        self.view_projection(aspect)?
            .narrow()
            .ok_or(CameraError::UndrawableProjection)
    }

    /// Where a world point lands in normalized device coordinates,
    /// or `None` when it is on or behind the eye plane (`w ≤ 0`).
    ///
    /// This is the projection the renderer performs, available
    /// without a renderer — which is what lets a test assert that a
    /// framed scene actually fits the frustum.
    ///
    /// # Errors
    ///
    /// As [`Camera::projection_matrix`].
    pub fn project(
        &self,
        point: Point3<f64>,
        aspect: f64,
    ) -> Result<Option<[f64; 3]>, CameraError> {
        let m = self.view_projection(aspect)?;
        let v = [point.x, point.y, point.z, 1.0];
        let mut out = [0.0f64; 4];
        for (row, slot) in out.iter_mut().enumerate() {
            *slot = m[0][row] * v[0] + m[1][row] * v[1] + m[2][row] * v[2] + m[3][row] * v[3];
        }
        if out[3].is_nan() || out[3] <= 0.0 {
            return Ok(None);
        }
        Ok(Some([out[0] / out[3], out[1] / out[3], out[2] / out[3]]))
    }

    /// The world ray through a cursor position — the **un-projection**,
    /// and the inverse of [`Camera::project`] on the frustum's
    /// direction (not on depth: a pixel names a ray, never a point).
    ///
    /// `cursor_px` is in the viewport's own physical pixels, `+x`
    /// right and `+y` DOWN — the one screen convention this crate has
    /// ([`crate::input`]'s module docs), so the flip to the camera's
    /// `+y`-up frame happens here and only here.
    ///
    /// The ray starts at the eye and its direction is a UNIT vector,
    /// which makes the `t` a hit comes back with a world distance.
    /// A cursor outside the viewport rectangle is not refused: it
    /// denotes a ray outside the frustum, which is a well-defined
    /// direction and an honest miss, and refusing it would put a
    /// bounds check on the caller for no gain.
    ///
    /// # Errors
    ///
    /// [`CameraError::NotFinite`] for a non-finite cursor coordinate
    /// or viewport dimension, and [`CameraError::UnusableBounds`] for
    /// a viewport with no area — the same two refusals
    /// [`Camera::projection_matrix`] makes about the same quantities.
    pub fn ray_through(
        &self,
        cursor_px: [f64; 2],
        viewport: crate::input::ViewportSize,
    ) -> Result<Ray, CameraError> {
        let [cx, cy] = cursor_px;
        finite("cursor x", cx)?;
        finite("cursor y", cy)?;
        finite("viewport width", viewport.width_px)?;
        finite("viewport height", viewport.height_px)?;
        let Some(aspect) = viewport.aspect() else {
            return Err(CameraError::UnusableBounds);
        };
        // Normalized device coordinates, through the conversion's one
        // home (`ViewportSize::ndc_of`) rather than a second spelling
        // of the y-flip here.
        let Some([ndc_x, ndc_y]) = viewport.ndc_of([cx, cy]) else {
            return Err(CameraError::UnusableBounds);
        };
        // The projection scales view-space x by `t / aspect` and y by
        // `t`, where `t = cot(fov_y / 2)`; inverting that on a point at
        // unit distance down the view axis gives the offsets below.
        let half_height = (self.fov_y * 0.5).tan();
        let (f, r, u) = (self.forward(), self.right(), self.up());
        let sx = ndc_x * half_height * aspect;
        let sy = ndc_y * half_height;
        let dir = Vec3::new(
            f.x + r.x * sx + u.x * sy,
            f.y + r.y * sx + u.y * sy,
            f.z + r.z * sx + u.z * sy,
        );
        // `powi(2)`, not `x * x` — the ratified interval-square rule
        // (`scripts/gates/interval-square-allowlist.sh`), which this
        // file is subject to like every other. The rule's own reason is
        // the gate's: `powi(2)` is strictly tighter than `x * x` when
        // the enclosure straddles zero and equal elsewhere, EXCEPT for a
        // square below 2^-960 where the backend pads once more — so it
        // is not "never wider", and whether a given enclosure can
        // straddle zero is a global property of upstream callers that
        // refactors change silently. Which is why the spelling is a
        // gate rather than a judgement call at each site.
        let len = (dir.x.powi(2) + dir.y.powi(2) + dir.z.powi(2)).sqrt();
        // `forward` is a unit vector and the offsets are perpendicular
        // to it, so the length is at least 1 for every finite cursor;
        // the guard is here because a non-finite one would otherwise
        // divide by NaN and hand back a poisoned ray as if it were an
        // answer, and the finiteness checks above are on the INPUT,
        // not on the arithmetic.
        if !(len.is_finite() && len > 0.0) {
            return Err(CameraError::NotFinite {
                what: "ray direction",
                value: len,
            });
        }
        Ok(Ray {
            origin: self.eye(),
            dir: Vec3::new(dir.x / len, dir.y / len, dir.z / len),
        })
    }
}

/// Perform one operation. The only way a [`Camera`] moves.
///
/// Pure: the argument is untouched and the result is a new value.
///
/// # Errors
///
/// [`CameraOpError`] for an operation that is not a move — a
/// non-finite delta, a non-positive dolly factor, or a frame whose
/// extent yields no camera. The caller keeps the camera it had.
pub fn apply(camera: &Camera, op: &CameraOp) -> Result<Camera, CameraOpError> {
    match *op {
        CameraOp::Orbit { yaw, pitch } => {
            op_finite("yaw", yaw)?;
            op_finite("pitch", pitch)?;
            Ok(Camera {
                yaw: wrap_angle(camera.yaw + yaw),
                pitch: clamp_pitch(camera.pitch + pitch),
                ..*camera
            })
        }
        CameraOp::Pan { right, up } => {
            op_finite("right", right)?;
            op_finite("up", up)?;
            let r = camera.right();
            let u = camera.up();
            Ok(Camera {
                target: Point3::new(
                    camera.target.x + r.x * right + u.x * up,
                    camera.target.y + r.y * right + u.y * up,
                    camera.target.z + r.z * right + u.z * up,
                ),
                ..*camera
            })
        }
        CameraOp::Dolly { factor } => {
            op_finite("factor", factor)?;
            if factor <= 0.0 {
                return Err(CameraOpError::NonPositiveDolly { factor });
            }
            Ok(Camera {
                distance: clamp_distance(camera.distance * factor, camera.scene_radius),
                ..*camera
            })
        }
        CameraOp::Frame { extent, aspect } => camera
            .fitted(extent, aspect)
            .map_err(CameraOpError::Unframeable),
    }
}

/// What a fold reached: the camera, and the refusal that stopped it.
///
/// **This is the one fold in the crate**, and the reason it is a
/// struct rather than a `Result` is that the two consumers need
/// different halves of the same answer. A test asks *did this refuse*;
/// an interactive viewport asks *where did I get to, and what do I
/// show the user* — and a `Result` that carries the error cannot also
/// carry the camera the fold reached before it, so the viewport used
/// to hand-roll its own loop with its own drifting semantics. Both
/// views are derived from this one ([`fold`] is the `Result` view),
/// so there is nothing left to drift.
#[derive(Clone, Debug, PartialEq)]
pub struct Folded {
    /// The camera the fold reached: the start camera when the first
    /// operation refused, and the fully folded one when none did.
    pub camera: Camera,
    /// The operations that were applied, in order — a prefix of the
    /// input, and the whole of it when `refused` is `None`.
    pub applied: Vec<CameraOp>,
    /// The refusal that stopped the fold, with the operation that
    /// provoked it. `None` when every operation applied.
    pub refused: Option<(CameraOp, CameraOpError)>,
}

/// Perform a sequence of operations in order, stopping at the first
/// refusal and **recording** it rather than discarding the progress.
///
/// Total: there is no error return, because "an operation refused" is
/// an outcome an interactive caller renders rather than a failure it
/// propagates.
pub fn fold_recorded<'a>(camera: &Camera, ops: impl IntoIterator<Item = &'a CameraOp>) -> Folded {
    let mut current = *camera;
    let mut applied = Vec::new();
    for op in ops {
        match apply(&current, op) {
            Ok(next) => {
                current = next;
                applied.push(*op);
            }
            Err(error) => {
                return Folded {
                    camera: current,
                    applied,
                    refused: Some((*op, error)),
                };
            }
        }
    }
    Folded {
        camera: current,
        applied,
        refused: None,
    }
}

/// Perform a sequence of operations in order, stopping at the first
/// refusal.
///
/// The `Result` view of [`fold_recorded`], for callers that only need
/// the verdict. A caller that also needs the camera the fold reached
/// before the refusal calls [`fold_recorded`] directly.
///
/// # Errors
///
/// The first [`CameraOpError`] the sequence produces.
pub fn fold<'a>(
    camera: &Camera,
    ops: impl IntoIterator<Item = &'a CameraOp>,
) -> Result<Camera, CameraOpError> {
    let folded = fold_recorded(camera, ops);
    match folded.refused {
        Some((_, error)) => Err(error),
        None => Ok(folded.camera),
    }
}

/// The view-projection that puts ONE source pixel over the whole 1×1
/// target the GPU id pass renders into.
///
/// A pixel centred at `cursor_ndc` spans `2 / width` by `2 / height` of
/// normalized device space, so translating that point to the origin and
/// scaling by the viewport's pixel dimensions maps exactly that pixel
/// onto the target's `[−1, 1]²`. In a column-major clip-space matrix
/// the translation is a subtraction of `cursor · w`, which is why the
/// `w` row participates.
///
/// **It is out of the render module because it is the one part of the
/// id pass a machine with no GPU can check**: composed with
/// [`Camera::project`] it says that the world point the ray path
/// un-projects to is the point the id pass rasterizes at the centre of
/// its target. That composition is the headless half of "both picking
/// paths answer the same question".
pub fn cursor_projection(
    view_projection: &[[f32; 4]; 4],
    cursor_ndc: [f32; 2],
    viewport_px: [f32; 2],
) -> [[f32; 4]; 4] {
    let [cx, cy] = cursor_ndc;
    let [sx, sy] = viewport_px;
    let mut out = *view_projection;
    for column in &mut out {
        let w = column[3];
        column[0] = (column[0] - cx * w) * sx;
        column[1] = (column[1] - cy * w) * sy;
    }
    out
}

/// The centre and radius of the sphere a fit frames: `extent`'s
/// bounding sphere, or the default [`Camera::fitted`] states where
/// there is no extent or its radius is below [`MIN_SCENE_RADIUS`].
///
/// **The endpoints being numbers does not make the radius one**, and
/// the radius is what the caller frames against. Squaring spends the
/// exponent twice, so a box a few hundred orders of magnitude across
/// — `±1e155` on one axis is enough — sums to infinity with every
/// endpoint an ordinary finite number. The check is therefore on the
/// PRODUCT as well as on the inputs, which is [`finite`]'s question
/// asked where the overflow is rather than only where the caller's
/// numbers are.
///
/// **The centre is asked the same question.** `0.5 * (lo + hi)`
/// overflows when an endpoint is past half of `f64::MAX`. A box with
/// any width there has an infinite radius too, but a point there has a
/// radius of zero, and a point is framed at its centre.
fn sphere(extent: Option<Aabb>) -> Result<(Point3<f64>, f64), CameraError> {
    let Some(bounds) = extent else {
        return Ok((Point3::new(0.0, 0.0, 0.0), DEFAULT_FRAME_RADIUS));
    };
    let lo = [
        bounds.min(Axis::X),
        bounds.min(Axis::Y),
        bounds.min(Axis::Z),
    ];
    let hi = [
        bounds.max(Axis::X),
        bounds.max(Axis::Y),
        bounds.max(Axis::Z),
    ];
    if lo.iter().chain(hi.iter()).any(|v| !v.is_finite()) {
        return Err(CameraError::UnusableBounds);
    }
    if lo.iter().zip(hi.iter()).any(|(l, h)| l > h) {
        return Err(CameraError::UnusableBounds);
    }
    let centre = Point3::new(
        0.5 * (lo[0] + hi[0]),
        0.5 * (lo[1] + hi[1]),
        0.5 * (lo[2] + hi[2]),
    );
    let half = [
        0.5 * (hi[0] - lo[0]),
        0.5 * (hi[1] - lo[1]),
        0.5 * (hi[2] - lo[2]),
    ];
    let radius: f64 = (half[0] * half[0] + half[1] * half[1] + half[2] * half[2]).sqrt();
    finite("scene radius", radius)?;
    finite("scene centre x", centre.x)?;
    finite("scene centre y", centre.y)?;
    finite("scene centre z", centre.z)?;
    if radius < MIN_SCENE_RADIUS {
        return Ok((centre, DEFAULT_FRAME_RADIUS));
    }
    Ok((centre, radius))
}

/// How far from its centre a sphere of `radius` must be viewed to fit
/// both half-angles at `aspect`, with [`FRAMING_MARGIN`] to spare. The
/// sphere subtends the smaller half-angle: on a viewport narrower than
/// it is tall, the horizontal one binds.
fn stand_off(radius: f64, fov_y: f64, aspect: f64) -> f64 {
    let half_v = fov_y * 0.5;
    let half_h = (half_v.tan() * aspect).atan();
    radius * FRAMING_MARGIN / half_v.min(half_h).sin()
}

/// Whether a field of view is too narrow for a fit at ANY aspect: the
/// vertical half-angle alone puts [`stand_off`] past [`band_ceiling`].
/// Both scale with the radius, so a unit radius asks it for every one.
fn lens_cannot_fit(fov_y: f64) -> bool {
    stand_off(1.0, fov_y, f64::INFINITY) > band_ceiling(1.0)
}

fn finite(what: &'static str, value: f64) -> Result<(), CameraError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(CameraError::NotFinite { what, value })
    }
}

fn op_finite(what: &'static str, value: f64) -> Result<(), CameraOpError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(CameraOpError::NotFinite { what, value })
    }
}

fn clamp_pitch(pitch: f64) -> f64 {
    let limit = Camera::pitch_limit();
    pitch.clamp(-limit, limit)
}

/// The closest a camera framed against `scene_radius` may dolly.
///
/// **Strictly positive for every radius the door admits, and that is a
/// property of the two constants rather than of a guard.** The factor
/// multiplies DOWN, so it cannot overflow; what it could do is flush a
/// small radius to zero, which would put the band's floor on a
/// distance that is not one and let a dolly in land on the target. It
/// does not, because the smallest admissible radius is
/// [`MIN_SCENE_RADIUS`] — `f64::MIN_POSITIVE` — and
/// [`MIN_DISTANCE_FACTOR`] leaves its product about fourteen decades
/// above the smallest subnormal. `the_bands_floor_is_a_length_at_the_smallest_radius_the_door_admits`
/// measures that margin rather than restating it, so a change to
/// either constant that spends it reds there.
fn band_floor(scene_radius: f64) -> f64 {
    scene_radius * MIN_DISTANCE_FACTOR
}

/// The furthest a camera framed against `scene_radius` may dolly.
///
/// **One home for the band's ceiling**, because four sites read it —
/// [`Camera::max_distance`], [`clamp_distance`], [`Camera::fitted`]'s
/// refusal, and [`Camera::new`]'s guard — and the guard's whole
/// subject is whether this product is a number. A second spelling at
/// any of them would be a bound on one number admitting a dolly limit
/// computed by another.
fn band_ceiling(scene_radius: f64) -> f64 {
    scene_radius * MAX_DISTANCE_FACTOR
}

fn clamp_distance(distance: f64, scene_radius: f64) -> f64 {
    distance.clamp(band_floor(scene_radius), band_ceiling(scene_radius))
}

/// Fold an angle into `[−π, π)` so orbit composition has one
/// representative per direction and repeated dragging cannot drift a
/// stored angle toward the exponent range where its resolution
/// collapses.
fn wrap_angle(angle: f64) -> f64 {
    let two_pi = std::f64::consts::TAU;
    let wrapped = angle - two_pi * ((angle + std::f64::consts::PI) / two_pi).floor();
    // `floor` on a value one ulp below a multiple of 2π can land the
    // result exactly on +π; fold that to the low end so the interval
    // is genuinely half-open.
    if wrapped >= std::f64::consts::PI {
        wrapped - two_pi
    } else {
        wrapped
    }
}

/// Column-major 4×4 product `a · b`.
fn mul(a: &[[f64; 4]; 4], b: &[[f64; 4]; 4]) -> [[f64; 4]; 4] {
    let mut out = [[0.0f64; 4]; 4];
    for (col, out_col) in out.iter_mut().enumerate() {
        for (row, slot) in out_col.iter_mut().enumerate() {
            *slot = (0..4).map(|k| a[k][row] * b[col][k]).sum();
        }
    }
    out
}

/// The zoom band's two ends, measured against the door that admits a
/// scene radius.
///
/// The rows over real scenes, operations and viewports are
/// `tests/camera_ops.rs`; what is here is the arithmetic the door
/// itself has to be right about, which needs no scene at all.
#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]

    use super::{
        Camera, CameraError, CameraOp, MAX_DISTANCE_FACTOR, MIN_DISTANCE_FACTOR, MIN_SCENE_RADIUS,
        apply, band_ceiling, band_floor,
    };
    use pncad::geom_core::Point3;

    fn camera_at(scene_radius: f64) -> Result<Camera, CameraError> {
        Camera::new(
            Point3::new(0.0, 0.0, 0.0),
            scene_radius,
            0.0,
            0.0,
            std::f64::consts::FRAC_PI_4,
            scene_radius,
        )
    }

    /// A radius past `f64::MAX / MAX_DISTANCE_FACTOR` is finite and
    /// strictly positive and every other guard in the constructor took
    /// it — and the band it derives has no top, so `max_distance`
    /// answered `inf` and `clamp_distance` bounded the camera's
    /// distance above by nothing.
    ///
    /// **The bound is measured here rather than asserted from a
    /// constant.** It is where `radius * MAX_DISTANCE_FACTOR` stops
    /// being finite, and the row walks one `f64` across it in both
    /// directions, so a change to [`MAX_DISTANCE_FACTOR`] that moved
    /// the door and not this arithmetic (or the reverse) reds here.
    #[test]
    fn the_door_refuses_a_scene_radius_whose_zoom_band_has_no_top() {
        let largest = f64::MAX / MAX_DISTANCE_FACTOR;
        assert!(
            band_ceiling(largest).is_finite(),
            "the largest framable scene still has a furthest distance"
        );
        assert!(
            camera_at(largest).is_ok(),
            "so the door takes every radius whose band names one"
        );
        let past = f64::from_bits(largest.to_bits() + 1);
        assert!(
            band_ceiling(past).is_infinite(),
            "and one `f64` further there is none — {past:e} is the bound's other side"
        );
        for refused in [past, 1.0e307, f64::MAX] {
            assert!(
                refused.is_finite() && refused >= MIN_SCENE_RADIUS,
                "{refused:e} is a radius the old predicate accepted"
            );
            assert_eq!(
                camera_at(refused),
                Err(CameraError::SceneRadiusOverflowsZoomBand {
                    scene_radius: refused
                }),
                "a scene radius with no furthest distance owes a refusal rather than a \
                 camera whose dolly has no stop"
            );
        }
    }

    /// The refusal is its own fact, and says which one it is: the
    /// radius IS a finite positive length, so a message calling it a
    /// non-number would be false, and a plain `Display` of a value
    /// three decades under `f64::MAX` is three hundred digits.
    #[test]
    fn the_band_refusal_reads_as_a_sentence_about_a_number_that_is_one() {
        let text = CameraError::SceneRadiusOverflowsZoomBand {
            scene_radius: 1.0e307,
        }
        .to_string();
        assert!(
            text.contains("1e307"),
            "the value is named in the spelling it can be read in: {text}"
        );
        let expansion = format!("{:.0}", 1.0e307_f64);
        assert!(
            expansion.len() > 300,
            "the plain spelling of a value this size is its decimal expansion"
        );
        assert!(
            text.len() < expansion.len(),
            "and the message is not that: {text}"
        );
        assert!(
            !text.contains("not a finite number to frame")
                && !text.contains("is not a positive extent"),
            "the radius is neither non-finite nor degenerate: {text}"
        );
    }

    /// Every camera the door admits has a furthest distance, and its
    /// own distance is inside the band — including a camera asked for
    /// a distance far past it, which [`Camera::new`] clamps.
    #[test]
    fn every_admitted_scene_radius_has_a_zoom_band_that_is_two_distances() {
        for radius in [
            MIN_SCENE_RADIUS,
            1.0e-12,
            1.0,
            1.0e153,
            f64::MAX / MAX_DISTANCE_FACTOR,
        ] {
            let camera = camera_at(radius).expect("a radius the door admits");
            let (floor, ceiling) = (camera.min_distance(), camera.max_distance());
            assert!(
                floor.is_finite() && ceiling.is_finite(),
                "the band at radius {radius:e} is {floor:e}..={ceiling:e}"
            );
            assert!(
                floor > 0.0 && floor < ceiling,
                "and it is a band rather than a point or an inversion: \
                 {floor:e}..={ceiling:e}"
            );
            assert!(
                camera.distance() >= floor && camera.distance() <= ceiling,
                "the camera's own distance {:e} is inside it",
                camera.distance()
            );
        }
    }

    /// [`MIN_DISTANCE_FACTOR`]'s direction, asked and answered: it
    /// multiplies DOWN, so it cannot overflow, and what it could do
    /// instead is flush a small radius to zero — a floor that is not a
    /// distance, and a dolly in that lands on the target.
    ///
    /// It cannot, and the margin is what says so: at
    /// [`MIN_SCENE_RADIUS`] the floor is about `2.2e14` times the
    /// smallest subnormal, so the factor would have to fall below
    /// about `2.2e-16` before the product reached zero. That is a
    /// property of the two constants and needs no guard at the door —
    /// which is why this row measures it, so a change to either that
    /// spends the margin reds here rather than at a viewport.
    #[test]
    fn the_bands_floor_is_a_length_at_the_smallest_radius_the_door_admits() {
        let floor = band_floor(MIN_SCENE_RADIUS);
        assert!(
            floor > 0.0,
            "the smallest framable scene still has a closest distance"
        );
        let smallest_subnormal = f64::from_bits(1);
        assert!(
            floor / smallest_subnormal > 1.0e14,
            "with decades of margin before the product flushes: {floor:e} is only {} \
             subnormals above zero",
            floor / smallest_subnormal
        );
        assert!(
            MIN_DISTANCE_FACTOR > smallest_subnormal / MIN_SCENE_RADIUS,
            "which is exactly the condition on the factor itself"
        );
        let camera = camera_at(MIN_SCENE_RADIUS).expect("the smallest framable scene");
        assert!(
            camera.distance() > 0.0,
            "so a camera framed against it stands somewhere rather than on its target"
        );
    }

    /// The band's top is what stops a dolly out, so a factor that
    /// overflows the distance lands on the furthest distance rather
    /// than on an infinity.
    #[test]
    fn a_dolly_out_past_the_band_lands_on_the_furthest_distance() {
        let camera = camera_at(1.0e300).expect("a radius the door admits");
        let dollied = apply(&camera, &CameraOp::Dolly { factor: 1.0e300 })
            .expect("a positive finite factor is a move");
        assert_eq!(
            dollied.distance(),
            camera.max_distance(),
            "a dolly whose product overflows stops at the band's top"
        );
        assert!(
            dollied.distance().is_finite(),
            "which is a distance: {:e}",
            dollied.distance()
        );
    }

    /// The framing doors cannot reach the new refusal, so it costs
    /// them nothing: `sphere` sums three squared half-extents and
    /// refuses a non-finite sum, which caps the radius they can carry
    /// at `sqrt(f64::MAX)`.
    #[test]
    fn the_largest_radius_a_framing_can_carry_still_has_a_band() {
        let largest_framable = f64::MAX.sqrt();
        assert!(
            band_ceiling(largest_framable).is_finite(),
            "the largest radius a bounding sphere can answer has a furthest distance"
        );
        assert!(
            camera_at(largest_framable).is_ok(),
            "so no framing is refused by the band guard"
        );
    }

    /// The default is the framing it claims to be, coordinate for
    /// coordinate — a second spelling of the stand-off that drifted by
    /// an ulp would red here.
    #[test]
    fn the_default_camera_is_the_framing_of_nothing_at_a_square_aspect() {
        assert_eq!(
            Camera::default(),
            Camera::framing(None, 1.0).expect("nothing frames at a square aspect"),
        );
    }
}
