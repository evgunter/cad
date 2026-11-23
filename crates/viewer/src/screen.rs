//! **How much world a pixel spans**: the value a mark drawn at a
//! fixed size on screen is sized through — a profile preview's tip
//! marks (`pane::viewport`) — so it reads at every zoom rather than
//! growing and shrinking with the model.
//!
//! A pure function of the camera and the pane's pixel size, with no
//! toolkit and no GPU in sight.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::geom_core::Point3;

use crate::camera::{Camera, CameraError};
use crate::input::ViewportSize;

/// **Is this a positive finite length?** — a scale or a span, handed
/// back when it is one and refused when it is not: it is asked of
/// quantities every consumer multiplies a pixel count by, where zero
/// and negative are as undrawable as `NaN`.
fn positive_length(value: f64) -> Option<f64> {
    (value.is_finite() && value > 0.0).then_some(value)
}

/// **Where the eye is and how much world a pixel spans** — everything
/// a screen-sized mark needs to be sized against the window rather
/// than against the model.
///
/// A value rather than a borrow of the camera, so a test drives these
/// numbers directly and asserts what comes out, with no camera, no
/// viewport and no renderer in the room.
#[derive(Debug, Clone, Copy)]
pub struct View {
    /// The eye's position, world metres.
    pub eye: Point3<f64>,
    /// **World metres one pixel spans at one metre from the eye.** The
    /// scale at any other depth is this times that depth, which is the
    /// whole of the perspective arithmetic here.
    ///
    /// From the camera as `2 * tan(fov_y / 2) / viewport_height_px`.
    /// **Not promised to be a length**, because this struct's fields
    /// are the caller's: [`view_of`] refuses the window that would
    /// produce a non-length, and [`View::screen_metres_at`] refuses a
    /// value that is not a scale anyway, once, for every mark.
    pub metres_per_pixel_at_one_metre: f64,
}

impl View {
    /// **What a span of `px` PIXELS measures at `point`**, in world
    /// metres, or `None` when this view lends `point` no such length:
    /// a depth that is not a number, a depth of exactly zero (the eye
    /// on the point), or a product that is not a positive finite
    /// length. Every consumer takes this `Option` and draws nothing for
    /// `None`.
    pub fn screen_metres_at(&self, point: Point3<f64>, px: f64) -> Option<f64> {
        let depth = ((point.x - self.eye.x).powi(2)
            + (point.y - self.eye.y).powi(2)
            + (point.z - self.eye.z).powi(2))
        .sqrt();
        let per_pixel = positive_length(depth * self.metres_per_pixel_at_one_metre)?;
        positive_length(per_pixel * px)
    }
}

/// **The [`View`] a camera lends a pane** of `viewport` pixels: the
/// vertical field of view over the vertical pixel count — one pixel's
/// angular share — which at one metre from the eye is that many
/// metres.
///
/// **A window that is not a number of pixels is REFUSED, by name**,
/// never floored at a pixel nor carried: the height divides the field
/// of view, so a zero height would lend an infinite metres-per-pixel
/// and a `NaN` height a `NaN` one. The refusal is the camera's own, on
/// the same two quantities and in the same words as
/// [`Camera::ray_through`]'s.
///
/// # Errors
///
/// [`CameraError::NotFinite`] for a width or a height that is not
/// finite, naming the side, and [`CameraError::UnusableBounds`] for a
/// viewport with no area.
pub fn view_of(camera: &Camera, viewport: ViewportSize) -> Result<View, CameraError> {
    let (width, height) = (viewport.width_px, viewport.height_px);
    for (what, value) in [("viewport width", width), ("viewport height", height)] {
        if !value.is_finite() {
            return Err(CameraError::NotFinite { what, value });
        }
    }
    if viewport.aspect().is_none() {
        return Err(CameraError::UnusableBounds);
    }
    Ok(View {
        eye: camera.eye(),
        metres_per_pixel_at_one_metre: 2.0 * (camera.fov_y() * 0.5).tan() / height,
    })
}
