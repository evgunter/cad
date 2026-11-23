//! Viewport input, mapped to camera operations.
//!
//! The toolkit's job is to say *what the pointer did*; this module's
//! job is to say *which operation that is*. Both halves are ordinary
//! values — [`ViewportEvent`] in, [`CameraOp`] out — so the whole
//! mapping is replayable in a headless test. The egui layer above
//! decides which toolkit values reach this vocabulary — for one, which
//! axis a wheel turned with a modifier held came in on
//! (`pane::viewport::scroll_events`) — and none of what an event
//! means.
//!
//! # Screen conventions
//!
//! Pointer deltas arrive in **physical pixels, +x right and +y down**
//! — the convention egui (and every windowing toolkit here) uses.
//! Nothing else in this crate uses a y-down axis; the flip happens
//! once, here.
//!
//! # Which gesture does what
//!
//! That is **data**: [`InputMap::bindings`], a slice of [`Binding`]
//! rows, and [`InputMap::DEFAULT`] is where the starting table and the
//! reason for each of its rows are written. The mapper
//! ([`InputMap::map`], [`InputMap::pick`]) honours a gesture because a
//! row names it, and the controls guide the running viewer shows is
//! the same rows through [`Binding::gesture`] and [`Binding::action`].
//! Nothing in this page says which button or key a gesture is on.
//!
//! What each action does, whichever gesture carries it:
//!
//! - **Orbit**: dragging right spins the model right, i.e. the camera
//!   orbits left; dragging down tips the model's top toward the
//!   viewer, i.e. the camera rises.
//! - **Pan**: the model follows the cursor. A drag of `n` pixels
//!   moves the target by exactly the world distance `n` pixels
//!   subtend *at the target plane*, so the point under the cursor
//!   stays under the cursor.
//! - **Zoom**: a scroll notch is a fixed multiplicative step, so
//!   zooming is scale-invariant and reversible. A pinch is a ratio and
//!   is read as one: spreading the fingers by a factor brings the eye
//!   that factor closer, so the model's apparent size follows the
//!   fingers, with no rate between.
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
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use crate::camera::{Camera, CameraOp, Folded};

crate::vocab::vocabulary! {
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

    /// Every button a binding can name.
    pub const ALL;
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
        /// The modifier keys down.
        held: Held,
        /// The motion since the previous event.
        delta_px: [f64; 2],
    },
    /// Wheel notches on one axis; positive is "up" on
    /// [`ScrollAxis::Wheel`], which zooms in, and content moving right
    /// on [`ScrollAxis::Sideways`].
    ///
    /// A wheel turned with ctrl or ⌘ held is one too, with that key in
    /// `held` — not a [`ViewportEvent::Pinch`].
    Scroll {
        /// The modifier keys down.
        held: Held,
        /// Which way the scroll went.
        axis: ScrollAxis,
        /// Notches (fractional on a trackpad).
        units: f64,
    },
    /// A pinch: the factor the fingers' spread changed by since the
    /// previous frame — above 1 spreading them (zoom in), below 1
    /// drawing them together.
    ///
    /// A ratio rather than [`ViewportEvent::Scroll`]'s notches, because
    /// that is what a pinch measures: converting it to notches only for
    /// [`InputMap::zoom_rate_per_notch`] to convert it back would put a
    /// rate where the gesture has none.
    Pinch {
        /// The magnification; exactly 1 is no pinch.
        factor: f64,
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
/// evaluation, which is [`crate::pickindex::PickIndex`]'s job.
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
    /// **The one home for the pixel→NDC conversion**, y-flip included.
    ///
    /// A cursor arrives in physical pixels from the pane's top-left
    /// corner with `+y` DOWN (this module's screen convention); the
    /// projection maps the frustum onto `x, y ∈ [−1, 1]` with `+y` UP.
    /// The flip is one subtraction and it was written independently in
    /// the camera's un-projection, in the id pass's query and twice in
    /// tests — four spellings of one convention, where a y-flip drift
    /// between any two is exactly the disagreement the id/ray
    /// comparison would misdiagnose as a GPU fault.
    ///
    /// `None` for a viewport with no area, which has no NDC at all.
    pub fn ndc_of(&self, cursor_px: [f64; 2]) -> Option<[f64; 2]> {
        if self.width_px > 0.0 && self.height_px > 0.0 {
            Some([
                2.0 * cursor_px[0] / self.width_px - 1.0,
                1.0 - 2.0 * cursor_px[1] / self.height_px,
            ])
        } else {
            None
        }
    }

    /// The inverse of [`ViewportSize::ndc_of`]: which cursor position
    /// names this NDC point. `None` for a viewport with no area.
    ///
    /// Here because a test that wants "the cursor over this world
    /// point" projects and then converts, and deriving that inverse at
    /// each call site is how the fourth spelling of the flip appeared.
    pub fn cursor_of(&self, ndc: [f64; 2]) -> Option<[f64; 2]> {
        if self.width_px > 0.0 && self.height_px > 0.0 {
            Some([
                (ndc[0] + 1.0) * 0.5 * self.width_px,
                (1.0 - ndc[1]) * 0.5 * self.height_px,
            ])
        } else {
            None
        }
    }

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

impl PointerButton {
    /// The button as a person names it, on a right-handed mouse.
    pub fn name(self) -> &'static str {
        match self {
            Self::Primary => "left",
            Self::Secondary => "right",
            Self::Middle => "middle",
        }
    }
}

/// Modifier keys: on an event, the keys down; on a binding row, the
/// keys it requires — a requirement, not an exact state, so a key a
/// row does not name is not asked about.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Held {
    /// Ctrl, or ⌘ on a Mac.
    pub ctrl: bool,
    /// The shift-style modifier.
    pub shift: bool,
    /// The alt-style modifier (⌥ on a Mac).
    pub alt: bool,
}

impl Held {
    /// No modifier.
    pub const NONE: Self = Self {
        ctrl: false,
        shift: false,
        alt: false,
    };
    /// Ctrl alone.
    pub const CTRL: Self = Self {
        ctrl: true,
        ..Self::NONE
    };
    /// Shift alone.
    pub const SHIFT: Self = Self {
        shift: true,
        ..Self::NONE
    };
    /// Alt alone.
    pub const ALT: Self = Self {
        alt: true,
        ..Self::NONE
    };

    /// Every combination of the three keys, [`Held::NONE`] first.
    pub const ALL: [Self; 8] = {
        let mut all = [Self::NONE; 8];
        let mut at = 0;
        while at < 8 {
            all[at] = Self {
                ctrl: at & 1 != 0,
                shift: at & 2 != 0,
                alt: at & 4 != 0,
            };
            at += 1;
        }
        all
    };

    /// Whether a gesture with the keys `down` meets this requirement:
    /// every key named here is down, and no other key is asked about.
    pub fn met_by(self, down: Self) -> bool {
        (!self.ctrl || down.ctrl) && (!self.shift || down.shift) && (!self.alt || down.alt)
    }

    /// How many keys are named — a gesture's most demanding matching
    /// row is the one it denotes.
    pub fn count(self) -> usize {
        usize::from(self.ctrl) + usize::from(self.shift) + usize::from(self.alt)
    }

    /// The keys, as the controls guide names them before a gesture:
    /// `""`, or `"Shift + "`, `"Ctrl + Alt + "` and so on.
    fn prefix(self) -> String {
        [
            (self.ctrl, "Ctrl"),
            (self.shift, "Shift"),
            (self.alt, "Alt"),
        ]
        .into_iter()
        .filter(|(down, _)| *down)
        .map(|(_, key)| format!("{key} + "))
        .collect()
    }
}

crate::vocab::vocabulary! {
    /// Which way a scroll went.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum ScrollAxis {
        /// Up and down: a wheel's turn, a trackpad's vertical swipe.
        Wheel,
        /// Left and right: a trackpad's sideways swipe, a tilt wheel.
        Sideways,
    }

    /// Both axes.
    pub const ALL;
}

/// How a bound drag moves the camera.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Navigation {
    /// Turn the camera about its target ([`CameraOp::Orbit`]).
    Orbit,
    /// Slide the target in the view plane ([`CameraOp::Pan`]).
    Pan,
}

crate::vocab::vocabulary! {
    /// What a bound gesture does.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum Action {
        /// [`CameraOp::Orbit`].
        Orbit,
        /// [`CameraOp::Pan`].
        Pan,
        /// [`CameraOp::Dolly`].
        Zoom,
        /// [`PickAction::Select`].
        Select,
        /// [`PickAction::Hover`].
        Highlight,
    }

    /// Every action a binding can have.
    pub const ALL;
}

impl Action {
    /// The action, as the controls guide names it.
    pub fn label(self) -> &'static str {
        match self {
            Self::Orbit => "orbit",
            Self::Pan => "pan",
            Self::Zoom => "zoom",
            Self::Select => "select what is under the pointer",
            Self::Highlight => "highlight what is under the pointer",
        }
    }
}

/// One row of the binding table: a gesture and what it does.
///
/// Each variant is a gesture together with the only kinds of action
/// that gesture can carry, so a row pairing a gesture with an action
/// of another kind — a scroll that orbits, a click that pans — cannot
/// be written. The type does not make rows distinct: of two rows for
/// one gesture, [`InputMap::bindings`]' rule honours one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Binding {
    /// A drag of `button` with at least `held` down navigates.
    Drag {
        /// The button held.
        button: PointerButton,
        /// The modifiers required.
        held: Held,
        /// The camera motion.
        moves: Navigation,
    },
    /// A scroll on `axis` with at least `held` down zooms, at
    /// [`InputMap::zoom_rate_per_notch`].
    Scroll {
        /// The axis scrolled.
        axis: ScrollAxis,
        /// The modifiers required.
        held: Held,
    },
    /// A pinch zooms by its own ratio.
    Pinch,
    /// A click of `button` selects ([`PickAction::Select`]).
    Click {
        /// The button clicked.
        button: PointerButton,
    },
    /// The pointer over the viewport hovers ([`PickAction::Hover`]),
    /// and leaving it clears the hover ([`PickAction::ClearHover`]).
    Hover,
}

impl Binding {
    /// The gesture, as the controls guide names it.
    pub fn gesture(&self) -> String {
        match *self {
            Self::Drag { button, held, .. } => keyed(held, &format!("{} drag", button.name())),
            Self::Scroll { axis, held } => keyed(
                held,
                match axis {
                    ScrollAxis::Wheel => "scroll",
                    ScrollAxis::Sideways => "sideways scroll",
                },
            ),
            Self::Pinch => "Pinch".to_owned(),
            Self::Click { button } => format!("{} click", capitalized(button.name())),
            Self::Hover => "Hover".to_owned(),
        }
    }

    /// What the gesture does.
    pub fn does(&self) -> Action {
        match *self {
            Self::Drag {
                moves: Navigation::Orbit,
                ..
            } => Action::Orbit,
            Self::Drag {
                moves: Navigation::Pan,
                ..
            } => Action::Pan,
            Self::Scroll { .. } | Self::Pinch => Action::Zoom,
            Self::Click { .. } => Action::Select,
            Self::Hover => Action::Highlight,
        }
    }

    /// What the gesture does, as the controls guide names it.
    pub fn action(&self) -> &'static str {
        self.does().label()
    }
}

/// `gesture` after the keys `held` names, capitalized at its start.
fn keyed(held: Held, gesture: &str) -> String {
    match held.prefix() {
        keys if keys.is_empty() => capitalized(gesture),
        keys => format!("{keys}{gesture}"),
    }
}

/// `word` with its first letter in upper case.
fn capitalized(word: &str) -> String {
    let mut letters = word.chars();
    letters
        .next()
        .map(|first| first.to_uppercase().chain(letters).collect())
        .unwrap_or_default()
}

/// The bindings and their rates.
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
    /// Which gesture does what: every gesture this map honours, and
    /// the rows the controls guide shows.
    ///
    /// A drag or a scroll denotes the matching row — a
    /// [`Binding::Drag`] on its button, a [`Binding::Scroll`] on its
    /// axis, whose keys are down — that requires the most modifiers,
    /// so a row naming one takes the gesture from the row naming none;
    /// between rows requiring equally many, the earlier.
    pub bindings: &'static [Binding],
}

impl Default for InputMap {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl InputMap {
    /// The bindings a viewer starts with.
    ///
    /// A `const` so [`PRESETS`] can name it, with [`Default`] above
    /// returning it: one set of numbers, reachable both ways, rather
    /// than a registry entry that can drift from the default it
    /// claims to be.
    pub const DEFAULT: Self = Self {
        // A full turn across roughly 800 px of drag.
        orbit_radians_per_px: 0.008,
        zoom_rate_per_notch: 0.1,
        bindings: &[
            // Mainstream CAD: rotate on this button, pan on it with a
            // modifier (the fourth row), zoom on the wheel.
            Binding::Drag {
                button: PointerButton::Middle,
                held: Held::NONE,
                moves: Navigation::Orbit,
            },
            // A trackpad has no practical drag of the first row's
            // button (first light, #1097), and the select button's
            // plain drag has to stay inert; a modifier gives it an
            // orbit without taking that.
            Binding::Drag {
                button: PointerButton::Primary,
                held: Held::ALT,
                moves: Navigation::Orbit,
            },
            // A second pan, for a mouse whose wheel does not click.
            Binding::Drag {
                button: PointerButton::Secondary,
                held: Held::NONE,
                moves: Navigation::Pan,
            },
            Binding::Drag {
                button: PointerButton::Middle,
                held: Held::SHIFT,
                moves: Navigation::Pan,
            },
            // Every wheel zooms, whatever key is held: a row each, so
            // the guide says so. Shift's is the one the toolkit would
            // have folded sideways (`pane::viewport::scroll_events`).
            Binding::Scroll {
                axis: ScrollAxis::Wheel,
                held: Held::NONE,
            },
            Binding::Scroll {
                axis: ScrollAxis::Wheel,
                held: Held::SHIFT,
            },
            Binding::Scroll {
                axis: ScrollAxis::Wheel,
                held: Held::CTRL,
            },
            Binding::Scroll {
                axis: ScrollAxis::Wheel,
                held: Held::ALT,
            },
            // No row names a sideways scroll: with zoom the one thing a
            // scroll can do, a sideways swipe that zoomed would be
            // worse than one that does nothing.
            Binding::Pinch,
            // Click-to-select, the first of
            // `crates/viewer/GUI-DESIGN.md` G3's four items.
            Binding::Click {
                button: PointerButton::Primary,
            },
            Binding::Hover,
        ],
    };

    /// What a drag or scroll with the keys `down` denotes, by the rule
    /// [`InputMap::bindings`] states: `fits` reads a row on the
    /// gesture's button or axis as the keys it requires and what it
    /// does, and is `None` on every other row.
    fn denoted<T>(&self, down: Held, fits: impl Fn(&Binding) -> Option<(Held, T)>) -> Option<T> {
        self.bindings
            .iter()
            .filter_map(|binding| fits(binding).filter(|(held, _)| held.met_by(down)))
            .map(|(held, does)| (held.count(), does))
            .reduce(|best, next| if next.0 > best.0 { next } else { best })
            .map(|(_, does)| does)
    }
}

/// The registered input presets, by the name a preferences file uses.
///
/// **A registry of one, and deliberately shaped like
/// [`crate::theme::Theme::ALL`]** — same lookup, same refusal on a
/// name nobody registered, same place a second entry goes.
///
/// It is worth being exact about what this does and does not reserve,
/// because the name "preset" invites the wrong reading. [`InputMap`]
/// binds POINTERS: two rate scalars and a table of pointer gestures
/// ([`Binding`]). **There is no
/// keyboard-binding vocabulary in this crate at all** — so a modal or
/// vim-shaped preset is not a row that can be added here. It needs
/// that vocabulary built first: which key means which
/// `CameraOp`/`SessionOp`, how modal state is held, how a chord is
/// represented.
///
/// **One key is read, and reading is not binding.**
/// [`crate::widgets::drag_gesture_ops`] asks whether Escape was
/// pressed on the frame a drag ended, because `egui` ends a drag on
/// Escape and on nothing else and the chrome would otherwise report
/// that abandonment as a release. The key is the toolkit's already;
/// what the branch decides is which of two things the toolkit did, not
/// which operation a key denotes.
///
/// What this reserves is the door and the naming: a preferences file
/// may say which preset it wants, an unknown name is refused rather
/// than silently ignored, and the day a real second preset exists it
/// plugs in here without the file format changing shape.
pub const PRESETS: &[(&str, InputMap)] = &[("default", InputMap::DEFAULT)];

/// The registered preset called `name`, if there is one.
///
/// `None` rather than a fallback, for [`crate::theme::Theme::by_name`]'s
/// reason: a name nobody registered is a typo, and silently binding
/// something else would hide it.
pub fn preset_by_name(name: &str) -> Option<InputMap> {
    PRESETS
        .iter()
        .find(|(registered, _)| *registered == name)
        .map(|(_, map)| *map)
}

impl InputMap {
    /// The operation an event denotes, or `None` when this event is
    /// bound to no operation (a gesture no [`InputMap::bindings`] row
    /// names, a drag or scroll of exactly zero, a pinch of exactly 1,
    /// or a viewport with no area).
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
                held: down,
                delta_px: [dx, dy],
            } => {
                if dx == 0.0 && dy == 0.0 {
                    return None;
                }
                let moves = self.denoted(down, |binding| match *binding {
                    Binding::Drag {
                        button: bound,
                        held,
                        moves,
                    } if bound == button => Some((held, moves)),
                    _ => None,
                })?;
                match moves {
                    Navigation::Pan => {
                        let world_per_px = self.world_per_px(viewport, camera)?;
                        // The model follows the cursor, so the target
                        // moves the other way; +y is down on screen and
                        // up is +y in the camera frame, hence the signs.
                        Some(CameraOp::Pan {
                            right: -dx * world_per_px,
                            up: dy * world_per_px,
                        })
                    }
                    Navigation::Orbit => Some(CameraOp::Orbit {
                        yaw: -dx * self.orbit_radians_per_px,
                        pitch: dy * self.orbit_radians_per_px,
                    }),
                }
            }
            ViewportEvent::Scroll {
                held: down,
                axis,
                units,
            } => {
                if units == 0.0 {
                    return None;
                }
                self.denoted(down, |binding| match *binding {
                    Binding::Scroll { axis: bound, held } if bound == axis => Some((held, ())),
                    _ => None,
                })?;
                Some(CameraOp::Dolly {
                    factor: (-units * self.zoom_rate_per_notch).exp(),
                })
            }
            ViewportEvent::Pinch { factor } => {
                if factor == 1.0 || !self.bindings.contains(&Binding::Pinch) {
                    return None;
                }
                // Apparent size goes as the reciprocal of distance, so
                // the distance scales by the reciprocal of the spread.
                // A factor that is not a positive number is passed on:
                // `camera::apply` refuses it.
                Some(CameraOp::Dolly {
                    factor: factor.recip(),
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
    /// asks for no pick — every drag, scroll and pinch, and a click or
    /// hover no [`InputMap::bindings`] row names.
    ///
    /// Needs no camera and no viewport: a cursor action names a screen
    /// position, and what lies under it is a question for the scene.
    pub fn pick(&self, event: &ViewportEvent) -> Option<PickAction> {
        match *event {
            ViewportEvent::Hover { pos_px } => self
                .bindings
                .contains(&Binding::Hover)
                .then_some(PickAction::Hover(pos_px)),
            ViewportEvent::Click { button, pos_px } => self
                .bindings
                .contains(&Binding::Click { button })
                .then_some(PickAction::Select(pos_px)),
            ViewportEvent::Leave => self
                .bindings
                .contains(&Binding::Hover)
                .then_some(PickAction::ClearHover),
            ViewportEvent::Drag { .. }
            | ViewportEvent::Scroll { .. }
            | ViewportEvent::Pinch { .. } => None,
        }
    }

    /// World units per physical pixel, measured in the plane through
    /// the target perpendicular to the view.
    ///
    /// **`None` is this door's refusal — there is no viewport to scale
    /// against** — and a height that is not a number is one of those.
    /// `height_px <= 0.0` alone does not say so: it is FALSE for a
    /// `NaN`, and an ordering is not a domain test on a value that has
    /// no order, so the height is asked whether it is a number before
    /// the bound on it is read as one. [`Camera`]'s own `finite` and
    /// `crate::screen::View::screen_metres_at` ask it the same way.
    ///
    /// **And the quotient is asked as well as the height**, because a
    /// height that is a positive number does not make every division
    /// by it one: the visible height is `2 · distance · tan(fov/2)`,
    /// which overflows for a camera standing off far enough, and a
    /// subnormal height divides a perfectly ordinary one to infinity.
    /// A scale of infinity is not a scale, and the pan below is
    /// `delta_px` times this — so without the second check the door
    /// hands a rate to an arithmetic whose product only
    /// [`Camera`]'s op guard would catch, one refusal away from the
    /// question the user actually asked.
    fn world_per_px(&self, viewport: ViewportSize, camera: &Camera) -> Option<f64> {
        if !(viewport.height_px.is_finite() && viewport.height_px > 0.0) {
            return None;
        }
        let visible_height = 2.0 * camera.distance() * (camera.fov_y() * 0.5).tan();
        let scale = visible_height / viewport.height_px;
        scale.is_finite().then_some(scale)
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
    events
        .into_iter()
        .filter_map(|event| map.pick(event))
        .collect()
}
