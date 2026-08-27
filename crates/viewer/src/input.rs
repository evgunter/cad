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
//!
//! **The primary (left) button selects, and moves no camera.** It is
//! the button click-to-select takes — the first of
//! `docs/GUI-DESIGN.md` G3's four items — so it is bound to
//! [`PickAction::Select`] and to nothing in the navigation vocabulary.
//! Navigation lives on middle and secondary, which is mainstream CAD
//! convention independently of that.
//!
//! # Two mappings over one event stream
//!
//! The same [`ViewportEvent`] stream feeds two independent readings,
//! and neither knows about the other: [`InputMap::map`] says which
//! **camera** operation an event is, and [`InputMap::pick`] says which
//! **cursor** action it is. An event is at most one of the two —
//! a drag moves the camera and picks nothing, a click picks and moves
//! nothing — but that is a property of the default bindings, not an
//! invariant either function enforces on the other.

use crate::camera::{Camera, CameraOp, Folded};

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
    /// The pointer is inside the viewport at this position, with no
    /// button held. Physical pixels from the viewport's top-left
    /// corner, the same axes as [`ViewportEvent::Drag`]'s delta.
    Hover {
        /// Where the cursor is.
        pos_px: [f64; 2],
    },
    /// A button was pressed and released without dragging.
    Click {
        /// Which button.
        button: PointerButton,
        /// Where the cursor was.
        pos_px: [f64; 2],
    },
    /// The pointer left the viewport.
    ///
    /// Its own event rather than a `Hover` with no position: "the
    /// cursor is nowhere" is a state the hover highlight has to reach,
    /// and an `Option` inside `Hover` would make every consumer handle
    /// the absent case at the position it reads.
    Leave,
}

/// What a cursor event asks the viewport to pick.
///
/// The **cursor** half of the input mapping, beside [`CameraOp`]'s
/// navigation half: a value naming a query, with no ray and no scene
/// in it. Turning one into a selection needs a camera and an
/// evaluation, which is [`crate::pick::PickIndex`]'s job.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PickAction {
    /// Move the transient hover to whatever is under this cursor.
    Hover([f64; 2]),
    /// Make whatever is under this cursor the selection — and, on a
    /// miss, clear it.
    Select([f64; 2]),
    /// The cursor is gone; there is nothing to hover.
    ClearHover,
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
    /// The button that selects.
    pub select_button: PointerButton,
}

impl Default for InputMap {
    fn default() -> Self {
        Self {
            // A full turn across roughly 800 px of drag.
            orbit_radians_per_px: 0.008,
            zoom_rate_per_notch: 0.1,
            orbit_button: PointerButton::Middle,
            pan_button: PointerButton::Secondary,
            select_button: PointerButton::Primary,
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
            // Cursor events move no camera. They are not "unbound":
            // they are the other mapping's subject, and `pick` below
            // is where they are read.
            ViewportEvent::Hover { .. } | ViewportEvent::Click { .. } | ViewportEvent::Leave => {
                None
            }
        }
    }

    /// The cursor action an event denotes, or `None` when this event
    /// asks for no pick — every drag and scroll, and a click on a
    /// button that is not [`InputMap::select_button`].
    ///
    /// Needs no camera and no viewport: a cursor action names a screen
    /// position, and what lies under it is a question for the scene.
    pub fn pick(&self, event: &ViewportEvent) -> Option<PickAction> {
        match *event {
            ViewportEvent::Hover { pos_px } => Some(PickAction::Hover(pos_px)),
            ViewportEvent::Click { button, pos_px } if button == self.select_button => {
                Some(PickAction::Select(pos_px))
            }
            ViewportEvent::Leave => Some(PickAction::ClearHover),
            ViewportEvent::Click { .. } | ViewportEvent::Drag { .. } | ViewportEvent::Scroll { .. } => {
                None
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

/// Fold an event stream through the camera: map each event to the
/// operation it denotes, apply it, and stop at the first refusal —
/// **recording** it rather than discarding the progress.
///
/// The camera advances between events, which is what makes the answer
/// faithful: a pan's world-per-pixel rate depends on the distance a
/// preceding scroll left behind. Events bound to no operation are
/// dropped, which is the mapping's own answer and not a refusal
/// ([`InputMap::map`] returns `None` for them).
///
/// **This is the fold the viewport runs.** It is also, through
/// [`map_stream`]'s `Result` view, the fold the tests run — one
/// implementation, so the shipped path and the tested path cannot
/// diverge in semantics the way three hand-rolled copies did.
pub fn fold_events<'a>(
    map: &InputMap,
    camera: &Camera,
    viewport: ViewportSize,
    events: impl IntoIterator<Item = &'a ViewportEvent>,
) -> Folded {
    let mut current = *camera;
    let mut applied = Vec::new();
    for event in events {
        let Some(op) = map.map(event, viewport, &current) else {
            continue;
        };
        match crate::camera::apply(&current, &op) {
            Ok(next) => {
                current = next;
                applied.push(op);
            }
            Err(error) => {
                return Folded {
                    camera: current,
                    applied,
                    refused: Some((op, error)),
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

/// Map a whole event stream to the operations it denotes, dropping
/// the events bound to none.
///
/// The `Result` view of [`fold_events`], for callers that only need
/// the verdict.
///
/// # Errors
///
/// The first [`crate::CameraOpError`] the produced operations provoke.
/// **The `Err` arm carries the refusal and nothing else** — a caller
/// that also needs the camera the fold reached before it, or the
/// operations that did apply, calls [`fold_events`], which returns
/// both. (This sentence used to promise a camera the type does not
/// carry; `fold_events` is the door that keeps the promise.)
pub fn map_stream<'a>(
    map: &InputMap,
    camera: &Camera,
    viewport: ViewportSize,
    events: impl IntoIterator<Item = &'a ViewportEvent>,
) -> Result<(Camera, Vec<CameraOp>), crate::CameraOpError> {
    let folded = fold_events(map, camera, viewport, events);
    match folded.refused {
        Some((_, error)) => Err(error),
        None => Ok((folded.camera, folded.applied)),
    }
}

/// The cursor actions an event stream denotes, in order.
///
/// The pick-side counterpart of [`fold_events`], and deliberately not
/// a fold: a cursor action carries no state that the next one depends
/// on, so there is nothing to accumulate. **This is the sequence the
/// viewport runs and the sequence the tests run**, for the same
/// reason the camera has one fold.
pub fn pick_stream<'a>(
    map: &InputMap,
    events: impl IntoIterator<Item = &'a ViewportEvent>,
) -> Vec<PickAction> {
    events.into_iter().filter_map(|event| map.pick(event)).collect()
}
