//! Viewport input, mapped to camera operations.
//!
//! The toolkit's job is to say *what the pointer did*; this module's
//! job is to say *which operation that is*. Both halves are ordinary
//! values — [`ViewportEvent`] in, [`CameraOp`] out — so the whole
//! mapping is replayable in a headless test, and the egui layer above
//! contains no navigation policy at all.
//!
//! # Screen conventions
//!
//! Pointer deltas arrive in **physical pixels, +x right and +y down**
//! — the convention egui (and every windowing toolkit here) uses.
//! Nothing else in this crate uses a y-down axis; the flip happens
//! once, here.
//!
//! # Which drag does what
//!
//! The bindings follow mainstream CAD (rotate on the middle button,
//! pan on the middle button with shift, zoom on the wheel), with the
//! secondary button as a second pan binding for mice without a usable
//! middle click:
//!
//! - **Orbit**: dragging right spins the model right, i.e. the camera
//!   orbits left; dragging down tips the model's top toward the
//!   viewer, i.e. the camera rises.
//! - **Pan**: the model follows the cursor. A drag of `n` pixels
//!   moves the target by exactly the world distance `n` pixels
//!   subtend *at the target plane*, so the point under the cursor
//!   stays under the cursor.
//! - **Zoom**: a scroll notch is a fixed multiplicative step, so
//!   zooming is scale-invariant and reversible.

use crate::camera::{Camera, CameraOp};

/// A pointer button, named rather than numbered.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PointerButton {
    /// Left, on a right-handed mouse.
    Primary,
    /// Right, on a right-handed mouse.
    Secondary,
    /// The wheel button.
    Middle,
}

/// What the pointer did inside the viewport.
///
/// This is the whole vocabulary the viewport consumes; a toolkit
/// binding produces these and nothing else.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewportEvent {
    /// A drag, in physical pixels (+x right, +y down).
    Drag {
        /// Which button is held.
        button: PointerButton,
        /// Whether a shift-style modifier is held — the modifier that
        /// turns the orbit binding into a pan binding.
        shift: bool,
        /// The motion since the previous event.
        delta_px: [f64; 2],
    },
    /// A wheel notch; positive scrolls "up", which zooms in.
    Scroll {
        /// Notches (fractional on a trackpad).
        units: f64,
    },
}

/// The viewport's size in physical pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewportSize {
    /// Width in physical pixels.
    pub width_px: f64,
    /// Height in physical pixels.
    pub height_px: f64,
}

impl ViewportSize {
    /// The aspect ratio, or `None` when the viewport has no area —
    /// which happens for real, on the frame a pane is first laid out
    /// and whenever one is dragged shut.
    pub fn aspect(&self) -> Option<f64> {
        if self.width_px > 0.0 && self.height_px > 0.0 {
            Some(self.width_px / self.height_px)
        } else {
            None
        }
    }
}

/// The navigation bindings and their rates.
///
/// A value rather than a set of constants because rates are the one
/// thing a user is entitled to disagree with, and because a test that
/// wants a round number can say so.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputMap {
    /// Radians of turntable rotation per pixel dragged.
    pub orbit_radians_per_px: f64,
    /// Multiplicative distance step per scroll notch. `0.1` means one
    /// notch toward the viewer multiplies the distance by `e^-0.1`.
    pub zoom_rate_per_notch: f64,
    /// The button that orbits.
    pub orbit_button: PointerButton,
    /// The button that pans without a modifier.
    pub pan_button: PointerButton,
}

impl Default for InputMap {
    fn default() -> Self {
        Self {
            // A full turn across roughly 800 px of drag.
            orbit_radians_per_px: 0.008,
            zoom_rate_per_notch: 0.1,
            orbit_button: PointerButton::Middle,
            pan_button: PointerButton::Secondary,
        }
    }
}

impl InputMap {
    /// The operation an event denotes, or `None` when this event is
    /// bound to no operation (a drag on an unbound button, a drag or
    /// scroll of exactly zero, or a viewport with no area).
    ///
    /// `camera` and `viewport` enter because pan is defined in world
    /// units: the conversion from pixels needs the distance to the
    /// target and the height the field of view subtends there.
    pub fn map(
        &self,
        event: &ViewportEvent,
        viewport: ViewportSize,
        camera: &Camera,
    ) -> Option<CameraOp> {
        match *event {
            ViewportEvent::Drag {
                button,
                shift,
                delta_px: [dx, dy],
            } => {
                if dx == 0.0 && dy == 0.0 {
                    return None;
                }
                let pans = button == self.pan_button || (button == self.orbit_button && shift);
                if pans {
                    let world_per_px = self.world_per_px(viewport, camera)?;
                    // The model follows the cursor, so the target
                    // moves the other way; +y is down on screen and
                    // up is +y in the camera frame, hence the signs.
                    Some(CameraOp::Pan {
                        right: -dx * world_per_px,
                        up: dy * world_per_px,
                    })
                } else if button == self.orbit_button {
                    Some(CameraOp::Orbit {
                        yaw: -dx * self.orbit_radians_per_px,
                        pitch: dy * self.orbit_radians_per_px,
                    })
                } else {
                    None
                }
            }
            ViewportEvent::Scroll { units } => {
                if units == 0.0 {
                    return None;
                }
                Some(CameraOp::Dolly {
                    factor: (-units * self.zoom_rate_per_notch).exp(),
                })
            }
        }
    }

    /// World units per physical pixel, measured in the plane through
    /// the target perpendicular to the view.
    fn world_per_px(&self, viewport: ViewportSize, camera: &Camera) -> Option<f64> {
        if viewport.height_px <= 0.0 {
            return None;
        }
        let visible_height = 2.0 * camera.distance() * (camera.fov_y() * 0.5).tan();
        Some(visible_height / viewport.height_px)
    }
}

/// Map a whole event stream to the operations it denotes, dropping
/// the events bound to none.
///
/// The camera advances between events, which is what makes the answer
/// faithful: a pan's world-per-pixel rate depends on the distance a
/// preceding scroll left behind.
///
/// # Errors
///
/// The first [`crate::CameraOpError`] the produced operations
/// provoke — the camera at that point is the last good one.
pub fn map_stream<'a>(
    map: &InputMap,
    camera: &Camera,
    viewport: ViewportSize,
    events: impl IntoIterator<Item = &'a ViewportEvent>,
) -> Result<(Camera, Vec<CameraOp>), crate::CameraOpError> {
    let mut current = *camera;
    let mut ops = Vec::new();
    for event in events {
        let Some(op) = map.map(event, viewport, &current) else {
            continue;
        };
        current = crate::camera::apply(&current, &op)?;
        ops.push(op);
    }
    Ok((current, ops))
}
