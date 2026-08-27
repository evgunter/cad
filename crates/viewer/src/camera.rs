//! The viewport camera: one state value, one typed operation
//! vocabulary, one pure `apply`.
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

use bvh::{Aabb, Axis};
use pncad::geom_core::{Point3, Vec3};

/// Elevation is held strictly inside `±(π/2 − POLE_MARGIN)`.
///
/// At exactly ±π/2 the view direction is world up and the camera
/// frame degenerates. A margin rather than an epsilon-free clamp
/// keeps `cos(pitch)` bounded away from zero, so the derived right
/// vector stays numerically well conditioned at the extremes.
const POLE_MARGIN: f64 = 1e-3;

/// The closest the camera may dolly, as a multiple of the scene
/// radius. Below this the near plane would sit inside the model.
const MIN_DISTANCE_FACTOR: f64 = 0.05;

/// The furthest the camera may dolly, as a multiple of the scene
/// radius.
const MAX_DISTANCE_FACTOR: f64 = 100.0;

/// The near plane's floor, as a multiple of the scene radius, for the
/// zoomed-in end where `distance − radius` is negative.
const NEAR_FLOOR_FACTOR: f64 = 1e-3;

/// A scene radius of zero (a point, an empty mesh) has no framing.
/// Refused rather than defaulted — a made-up scale is a lie about the
/// model.
const MIN_SCENE_RADIUS: f64 = f64::MIN_POSITIVE;

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
/// Construct with [`Camera::framing`] (fit a scene) or
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
        self.target.x == other.target.x
            && self.target.y == other.target.y
            && self.target.z == other.target.z
            && self.distance == other.distance
            && self.yaw == other.yaw
            && self.pitch == other.pitch
            && self.fov_y == other.fov_y
            && self.scene_radius == other.scene_radius
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
    /// The scene radius was not strictly positive: a bounding box of
    /// zero extent, or an inverted (empty) one.
    DegenerateScene {
        /// The radius derived from the caller's bounds.
        radius: f64,
    },
    /// The field of view was not strictly inside `(0, π)`.
    FieldOfViewOutOfRange {
        /// The offending value, in radians.
        fov_y: f64,
    },
    /// The bounding box carried a NaN bound (`Aabb`'s poison state) or
    /// was empty, so no scene can be derived from it.
    UnusableBounds,
}

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
    /// Re-frame on a bounding box: recentre, re-derive the scene
    /// radius and the zoom band, and back off far enough that the
    /// bounding sphere fits the vertical field of view. Orientation
    /// is kept — a fit is not a reset.
    Frame {
        /// The box to fit.
        bounds: Aabb,
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
    /// A [`CameraOp::Frame`] whose bounds or aspect could not produce
    /// a camera.
    Unframeable(CameraError),
}

impl Camera {
    /// A camera from explicit state.
    ///
    /// `pitch` is clamped to the pole margin; every other argument is
    /// taken as given and refused if it is not usable.
    ///
    /// # Errors
    ///
    /// [`CameraError::NotFinite`] for any non-finite argument,
    /// [`CameraError::DegenerateScene`] for a non-positive scene
    /// radius, [`CameraError::FieldOfViewOutOfRange`] for a field of
    /// view outside `(0, π)`.
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

    /// The default three-quarter view fitted to `bounds`.
    ///
    /// # Errors
    ///
    /// [`CameraError::UnusableBounds`] for an empty or poisoned box,
    /// [`CameraError::DegenerateScene`] for a box of zero extent, and
    /// [`CameraError::NotFinite`] for a non-finite aspect.
    pub fn framing(bounds: &Aabb, aspect: f64) -> Result<Self, CameraError> {
        let (centre, radius) = sphere(bounds)?;
        let camera = Self::new(
            centre,
            radius * MAX_DISTANCE_FACTOR,
            DEFAULT_YAW,
            DEFAULT_PITCH,
            DEFAULT_FOV_Y,
            radius,
        )?;
        camera.fitted(bounds, aspect)
    }

    /// This camera re-centred and backed off to fit `bounds`, keeping
    /// its orientation and field of view.
    ///
    /// # Errors
    ///
    /// As [`Camera::framing`].
    pub fn fitted(&self, bounds: &Aabb, aspect: f64) -> Result<Self, CameraError> {
        finite("aspect", aspect)?;
        if aspect <= 0.0 {
            return Err(CameraError::UnusableBounds);
        }
        let (centre, radius) = sphere(bounds)?;
        // The bounding sphere subtends the smaller of the two
        // half-angles: on a viewport narrower than it is tall, the
        // horizontal one binds.
        let half_v = self.fov_y * 0.5;
        let half_h = (half_v.tan() * aspect).atan();
        let half = half_v.min(half_h);
        let distance = radius * FRAMING_MARGIN / half.sin();
        Self::new(
            centre,
            distance,
            self.yaw,
            self.pitch,
            self.fov_y,
            radius,
        )
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
        self.scene_radius * MIN_DISTANCE_FACTOR
    }

    /// The furthest this camera may dolly.
    pub fn max_distance(&self) -> f64 {
        self.scene_radius * MAX_DISTANCE_FACTOR
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
        let f = self.forward();
        let r = self.right();
        Vec3::new(
            r.y * f.z - r.z * f.y,
            r.z * f.x - r.x * f.z,
            r.x * f.y - r.y * f.x,
        )
    }

    /// The near plane distance.
    ///
    /// Tracks the eye so the depth range stays tight at every zoom:
    /// it is the distance to the front of the bounding sphere, with a
    /// floor for the zoomed-in end where the eye is inside it.
    pub fn near(&self) -> f64 {
        let floor = self.scene_radius * NEAR_FLOOR_FACTOR;
        (self.distance - self.scene_radius).max(floor)
    }

    /// The far plane distance: the back of the bounding sphere.
    pub fn far(&self) -> f64 {
        self.distance + self.scene_radius * 2.0
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
    /// column-major, mapping the view frustum onto wgpu's `z ∈ [0, 1]`
    /// clip range.
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
        let (near, far) = (self.near(), self.far());
        let range = near - far;
        Ok([
            [t / aspect, 0.0, 0.0, 0.0],
            [0.0, t, 0.0, 0.0],
            [0.0, 0.0, far / range, -1.0],
            [0.0, 0.0, near * far / range, 0.0],
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
}

/// Perform one operation. The only way a [`Camera`] moves.
///
/// Pure: the argument is untouched and the result is a new value.
///
/// # Errors
///
/// [`CameraOpError`] for an operation that is not a move — a
/// non-finite delta, a non-positive dolly factor, or a frame whose
/// bounds yield no camera. The caller keeps the camera it had.
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
        CameraOp::Frame { bounds, aspect } => camera
            .fitted(&bounds, aspect)
            .map_err(CameraOpError::Unframeable),
    }
}

/// Perform a sequence of operations in order, stopping at the first
/// refusal.
///
/// # Errors
///
/// The first [`CameraOpError`] the sequence produces.
pub fn fold<'a>(
    camera: &Camera,
    ops: impl IntoIterator<Item = &'a CameraOp>,
) -> Result<Camera, CameraOpError> {
    let mut current = *camera;
    for op in ops {
        current = apply(&current, op)?;
    }
    Ok(current)
}

/// The centre and radius of a box's bounding sphere.
fn sphere(bounds: &Aabb) -> Result<(Point3<f64>, f64), CameraError> {
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
    if radius < MIN_SCENE_RADIUS {
        return Err(CameraError::DegenerateScene { radius });
    }
    Ok((centre, radius))
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
    let limit = std::f64::consts::FRAC_PI_2 - POLE_MARGIN;
    pitch.clamp(-limit, limit)
}

fn clamp_distance(distance: f64, scene_radius: f64) -> f64 {
    distance.clamp(
        scene_radius * MIN_DISTANCE_FACTOR,
        scene_radius * MAX_DISTANCE_FACTOR,
    )
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
