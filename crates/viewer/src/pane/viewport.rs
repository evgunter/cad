//! The viewport pane: the wgpu surface, the pointer, and the overlays
//! drawn over both.
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

use std::sync::Arc;
use std::sync::atomic::Ordering;

use eframe::egui;
use pncad::document::Evaluation;
use pncad::prelude::StableName;

use crate::app::{ViewerBehavior, chrome};
use crate::camera::{self, Camera, CameraOp};
use crate::datums::{self, datum_view};
use crate::display::DisplayView;
use crate::frame;
use crate::gpu::{IdQuery, ViewportCallback};
use crate::idpass::{self, IdStep};
use crate::input::{self, PickAction, PointerButton, ViewportEvent, ViewportSize};
use crate::marks;
use crate::narrowing::Narrow;
use crate::pickcache;
use crate::pickindex::{PickError, PickIndex, PictureKey};
use crate::session::SessionOp;
use crate::sketch::{self, PreviewLoop, TIP_MARK_PX, heading};

/// **One sketch-plane segment, placed and offered to an overlay lane**
/// as the line-list pair the edge pass draws.
///
/// This function PLACES and nothing else: whether a leg can be drawn
/// is [`marks::LegLane`]'s answer, which is where the crate keeps it
/// and where a test can ask it with no pane in existence. What is
/// particular here is that this lane is the one with an authored
/// producer: a path whose corner is `7e307` — a number the
/// add-profile form takes, because it is a number — replays,
/// flattens, and arrives on its plane with coordinates whose
/// narrowing is an infinity. A path with one such corner and the rest
/// ordinary is drawn as its ordinary legs and a gap, which is
/// `LegLane::undrawn`'s subject.
fn push_segment(
    lane: &mut marks::LegLane,
    plane: &pncad::profile::SketchPlane<f64>,
    a: [f64; 2],
    b: [f64; 2],
) {
    let [from, to] = [a, b].map(|[x, y]| plane.to_world(pncad::geom_core::Point2::new(x, y)));
    lane.leg(from, to);
}

/// **One drawn loop, placed on its plane and appended to a lane.**
///
/// A CLOSED loop's segment list wraps — the last point joins the
/// first, which is the same thing `ProfileLoop` means by being closed
/// by construction. An OPEN one's must not: the leg back to the start
/// is the provisional close `sketch::preview` walked the chain under
/// and nobody authored, so the wrap is dropped and what is drawn is the
/// authored legs exactly. That is the whole of "a path draws while it
/// is still being written".
fn push_loop(
    lane: &mut marks::LegLane,
    plane: &pncad::profile::SketchPlane<f64>,
    polyline: &PreviewLoop,
) {
    let points = &polyline.points;
    let segments = if polyline.closed {
        points.len()
    } else {
        points.len().saturating_sub(1)
    };
    for index in 0..segments {
        push_segment(
            lane,
            plane,
            points[index],
            points[(index + 1) % points.len()],
        );
    }
}

/// Land a fold: take the camera it reached, and show the refusal that
/// stopped it.
///
/// **The one place a camera move becomes application state.** Both the
/// toolbar's single operations and the viewport's event stream come
/// through here, so what a fold says has one implementation.
///
/// What it says is [`frame::fold_status`]'s to decide, and a clean fold
/// says NOTHING: a camera is the fastest-moving writer the status line
/// has, and one that assigned the line on every clean fold would erase
/// the news of whichever writer shares its frame — including, on the
/// frame a document lands, the landing's own.
pub(crate) fn land(
    camera: &mut Camera,
    notices: &mut Vec<frame::Message>,
    status: &mut Option<frame::Message>,
    folded: &camera::Folded,
) {
    *camera = folded.camera;
    // Both halves of the verdict, each by its own route
    // ([`frame::deliver`]): a refused fold is NEWS and joins this
    // frame's notices, where the ranking can weigh it against whatever
    // else the frame produced; a clean fold RETIRES the camera
    // sentence and reaches the field directly, because a notice cannot
    // un-say anything.
    frame::deliver(notices, status, frame::fold_status(folded));
}

/// The index the picture on screen was drawn FROM, or `None` when the
/// index in hand describes some other picture.
///
/// # Why a read of the index can need this and not the evaluation
///
/// `ViewerBehavior::index` is the index for the document the session
/// has landed. `ViewerBehavior::scene` is the mesh of whatever picture
/// last succeeded in being built, which is the same thing on almost
/// every frame and is NOT the same thing whenever a scene rebuild
/// refused: `ViewerApp::sync_scene` marks the pair current only on
/// success, so a landed index over a refused rebuild leaves a new index
/// beside an older picture, and nothing retries while the display and
/// the focus hold still.
///
/// Three reads care. Two of them have a pick **id** for their
/// currency:
///
/// - resolving an id the id pass produced, which is a word of the id
///   map of whichever index minted the picture's corners; and
/// - minting ids or world-space segments for the picture to draw over
///   itself, where the shader compares them against those same
///   corners.
///
/// Both are false-by-construction across two pictures, and the first
/// writes its falsehood to the status line as *the two picking paths
/// disagree* — a sentence issue #1097 §4 tells an operator to read as
/// an `R32Uint` clear fault, so a wrong subsystem gets named.
///
/// The third is the **pick**, and it asks for a different reason.
///
/// **The whole key, not the generation.** An index is keyed by a
/// [`PictureKey`] and so is its id map: a δ typed while the picture
/// stands rebuilds the index at the same generation, over a different
/// tessellation, with a different alphabet. A generation-only check
/// reads as co-identity and is not it, so the question goes to
/// [`PickIndex::current_for`], the one door that answers *does this
/// index describe this picture* — and the key is one value, so that
/// door cannot be handed half of it.
///
/// **The pick path asks too, and what it does on `None` is
/// different.** A click asks what is under the cursor in the
/// DOCUMENT, and resolves it through the index and the evaluation
/// with no id and no mesh in sight — so nothing about it is false by
/// construction, and the reason it is gated is a product ruling
/// rather than a correctness one: an answer about geometry the screen
/// is not showing selects something the user cannot see, and Ev ruled
/// on 2026-09-15 that the click is refused instead. The picture-side
/// reads skip silently on `None`, because a mark nobody can draw is
/// nothing to say; the pick path refuses TYPED, because a click is an
/// act the user made and got nothing for
/// ([`crate::pickcache::NotIndexed::AnotherPicture`]).
fn drawn_index(index: Option<&PickIndex>, scene_key: Option<PictureKey>) -> Option<&PickIndex> {
    index.filter(|index| index.current_for(scene_key))
}

/// **Whether a pick action is skipped this frame**: a hover over an
/// unchanged picture at an unmoved cursor, whose answer the session is
/// taken to hold already. A click never skips: it is an ACTION, not an
/// observation.
fn skips_the_ray(action: PickAction, step: IdStep) -> bool {
    step == IdStep::Hold && matches!(action, PickAction::Hover(_))
}

/// **Whether this frame's pick actions ask the ray at `cursor`**, and
/// so say what it refuses there through [`frame::pick_refusal`].
///
/// The loop that performs the actions skips by [`skips_the_ray`] and
/// this reads the same rule, so the two cannot disagree about which
/// frames the pick path spoke on.
fn ray_asked_at(actions: &[PickAction], step: IdStep, cursor: [f64; 2]) -> bool {
    actions.iter().any(|&action| {
        !skips_the_ray(action, step)
            && matches!(action, PickAction::Hover(at) | PickAction::Select(at) if at == cursor)
    })
}

/// Everything the ray path is asked beside the index: one cursor over
/// one evaluation, through one camera, under one display view.
#[derive(Clone, Copy)]
struct RayQuestion<'a> {
    eval: &'a Evaluation<f64>,
    camera: &'a Camera,
    viewport: ViewportSize,
    cursor: [f64; 2],
    display: &'a DisplayView,
}

/// **What the cursor comparison says this frame**: the two picking
/// paths' disagreement, the ray path's refusal, or nothing.
///
/// The ray answer travels to [`idpass::disagreement`] typed, because a
/// refusal is not a miss: that function reads a refused ray path as no
/// verdict rather than as "the ray named nothing".
///
/// **A refusal no pick action said this frame is said here.** The
/// refusal is the ray path's news, and it has words already
/// ([`frame::pick_refusal`]); the pick loop says them whenever it asks
/// the ray at this cursor, because `hovered_for` seeds through the
/// same un-projection and hit test as `faces_under_cursor`. It does
/// not ask on a frame it skips ([`skips_the_ray`]), and the skip reads
/// only the cursor and the picture — so a camera that moved under a
/// still cursor gets a ray nobody else asked. `ray_asked` is the pick
/// loop's own record of that ([`ray_asked_at`]), which is what keeps
/// the refusal said exactly once a frame: by the pick path when it
/// asked, here when it did not.
fn cursor_news(
    index: &PickIndex,
    question: RayQuestion<'_>,
    answer: u64,
    outstanding: Option<u32>,
    ray_asked: bool,
) -> Option<frame::Message> {
    let RayQuestion {
        eval,
        camera,
        viewport,
        cursor,
        display,
    } = question;
    let from_ray: Result<Vec<StableName>, PickError> = index
        .faces_under_cursor(eval, camera, viewport, cursor, display)
        .map(|faces| faces.into_iter().map(|face| face.name).collect());
    match &from_ray {
        Err(refusal) if !ray_asked => Some(frame::pick_refusal(refusal)),
        _ => idpass::disagreement(index, answer, outstanding, from_ray.as_deref())
            .map(|report| report.notice()),
    }
}

/// Direction the light travels, world space; a unit vector over the
/// viewer's left shoulder.
const LIGHT_DIRECTION: [f32; 3] = [0.408_248_3, 0.408_248_3, -0.816_496_6];

/// Which button of the viewer's vocabulary an `egui` button denotes,
/// or `None` for one the viewer binds nothing to.
///
/// **The one place the toolkit's button set meets the viewer's**, and
/// the two are deliberately not the same set. [`input::PointerButton`]
/// names the buttons this viewer BINDS; `egui::PointerButton` names
/// the buttons a mouse can report, side buttons included.
///
/// **A side button gets `None` because there is nothing for it to
/// be.** [`input::InputMap`]'s four bindings are filled by the three
/// main buttons; no preset, preferences key or API call can name a
/// fifth; and the bindings follow mainstream CAD (`input`'s module
/// docs), which has no side-button gesture. A variant for one would
/// be a word of the vocabulary that no sentence could use — and on
/// the web backend these two are the browser's back and forward.
/// Binding them is a product decision, and it arrives as a variant on
/// `input::PointerButton`, a binding field or preset that can name it,
/// and an arm here that stops saying `None`.
///
/// **The compiler holds the SET, and nothing more.** This match names
/// every `egui::PointerButton` and the enum is not `#[non_exhaustive]`,
/// so an egui that grows a sixth button makes it non-exhaustive;
/// [`egui_buttons`] is `NUM_POINTER_BUTTONS` long, so the same upgrade
/// fails its length. A version bump is the only moment the toolkit's
/// set can change, and it is the moment both of these fire. (Were egui
/// to become `#[non_exhaustive]`, the match half dies — `_ => None` is
/// then the only shape available — and the array's length is the whole
/// hold. Say so here on the day it happens.)
///
/// **Which button pairs with which is held by a ROW, not by the
/// compiler**, and it could not be otherwise: the pairing is a naming
/// decision with nothing to derive it from. Swapping two arms here
/// type-checks and changes what every mouse does.
/// `tests::the_pairing_is_the_one_this_module_intends` is the second
/// statement of the table that makes such an edit red, and the only
/// thing in the tree that can.
fn viewer_button(button: egui::PointerButton) -> Option<PointerButton> {
    match button {
        egui::PointerButton::Primary => Some(PointerButton::Primary),
        egui::PointerButton::Secondary => Some(PointerButton::Secondary),
        egui::PointerButton::Middle => Some(PointerButton::Middle),
        egui::PointerButton::Extra1 | egui::PointerButton::Extra2 => None,
    }
}

/// Every button `egui` can report.
///
/// A function rather than a `const` item, and its length is the
/// toolkit's own `NUM_POINTER_BUTTONS`: the compiler counts this list
/// against the declaration it mirrors, so it is not a hand-maintained
/// membership list and wants no row on
/// `crates/viewer/README.md`'s roster of those.
fn egui_buttons() -> [egui::PointerButton; egui::NUM_POINTER_BUTTONS] {
    [
        egui::PointerButton::Primary,
        egui::PointerButton::Secondary,
        egui::PointerButton::Middle,
        egui::PointerButton::Extra1,
        egui::PointerButton::Extra2,
    ]
}

/// Which modifier keys this viewport binds, in the order
/// [`ViewportEvent::Drag`] carries them: shift, then alt.
///
/// **The one place the toolkit's modifier state meets the viewer's**,
/// and — with [`scroll_event`], which is the other half of the same
/// decision — the place that says what this adapter reads of a frame's
/// pointer state and what it drops. The toolkit offers five modifier
/// fields and two scroll axes; this viewport binds two of the five and
/// one of the two, and the parts it does not bind are named here and
/// discarded rather than never mentioned, because a part an adapter
/// leaves out is indistinguishable downstream from a part nobody
/// touched.
///
/// **Two, because [`input::InputMap`] has two to bind**: `alt`, which
/// turns [`input::InputMap::alt_orbit_button`]'s drag into an orbit
/// (the trackpad binding), and `shift`, which turns the orbit binding
/// into a pan. `ctrl`, `command` and `mac_cmd` are not gestures in
/// this viewport — no binding field, preset or preferences key names
/// one, and [`ViewportEvent::Drag`] carries a bare `shift` and a bare
/// `alt` and nothing else, so a ctrl-drag is a plain drag at every
/// reader downstream. Binding one arrives the way a side button's
/// would ([`viewer_button`]): a field on [`ViewportEvent::Drag`] that
/// can carry it, a binding that can name it, and a pattern here that
/// stops discarding it.
///
/// **The compiler holds the SET.** `egui::Modifiers` is a plain struct
/// and not `#[non_exhaustive]`, so a pattern over it must mention
/// every field the toolkit declares: the day egui grows a sixth
/// modifier this stops compiling, and someone answers for it in
/// writing. **A stop rather than a wall** — the error names the
/// missing field and offers `..` among its repairs, so what it buys is
/// that the answer is given HERE, on the day the field appears,
/// instead of being given by omission. It is the struct's form of the
/// exhaustive match
/// [`viewer_button`] makes over the toolkit's button enum, and it
/// fires at the same moment — a version bump, the only moment either
/// set can change. **It holds the set and nothing else**: which field
/// lands in which half of the returned pair is a naming decision with
/// nothing to derive it from, and swapping the two type-checks while
/// inverting every modified drag in the viewer.
/// `tests::a_drag_carries_the_two_modifiers_this_viewport_binds` is
/// what makes that edit red.
fn viewer_modifiers(modifiers: egui::Modifiers) -> (bool, bool) {
    let egui::Modifiers {
        shift,
        alt,
        ctrl: _,
        mac_cmd: _,
        command: _,
    } = modifiers;
    (shift, alt)
}

/// The scroll event this frame's wheel denotes, or `None` when the
/// wheel turned in no direction this viewport binds.
///
/// The other half of [`viewer_modifiers`]' decision: of the toolkit's
/// two scroll axes this viewport reads `y`, which
/// [`input::InputMap::map`] reads as the one binding a scroll has —
/// zoom.
///
/// **`x` is dropped because the viewer has no horizontal gesture.** A
/// positive `x` is content moving right: a trackpad's sideways swipe,
/// a tilt wheel, and shift+wheel on an ordinary mouse, which the
/// toolkit folds onto that axis itself (`InputOptions`'
/// `horizontal_scroll_modifier`, SHIFT by default). Zoom being the
/// only binding a scroll has, an `x` passed on would have to zoom, and
/// a sideways swipe that zooms is a worse answer than one that does
/// nothing.
///
/// **A ctrl+wheel reaches this function as a zero, and that is the
/// toolkit's doing rather than this adapter's.** `InputState`'s
/// per-frame pass routes a wheel whose modifiers match `InputOptions`'
/// `zoom_modifier` — ctrl, ⌘ or `command`, by default — into
/// `zoom_factor_delta`, leaving `smooth_scroll_delta` at zero. So the
/// gesture most CAD and browser users expect to zoom produces no
/// [`ViewportEvent`] here at all: not a plain scroll wearing a
/// modifier this adapter drops — nothing. Reading `ctrl` above would
/// not recover it; binding it means reading a THIRD toolkit value,
/// which is a product decision and is
/// `work/view/ctrl-wheel-reaches-no-zoom.md`. An alt+wheel is the one
/// modified wheel that already works, by the same mechanism in the
/// other direction: `vertical_scroll_modifier` folds it onto `y`, so
/// it zooms exactly as a plain wheel does.
///
/// **The compiler holds both axes** the way [`viewer_modifiers`] holds
/// the five fields — `egui::Vec2` is a plain struct, so the pattern
/// names `x` in order to drop it. That upgrade hold is nominal, a
/// two-axis vector being unlikely to grow a third; what the pattern
/// buys here is that the drop is written at the site rather than
/// implied by a field access.
fn scroll_event(delta: egui::Vec2) -> Option<ViewportEvent> {
    let egui::Vec2 { y: points, x: _ } = delta;
    (points != 0.0).then(|| ViewportEvent::Scroll {
        // egui reports scroll in points; a wheel notch is
        // conventionally 50 of them.
        units: f64::from(points) / 50.0,
    })
}

/// The drag and click events this frame's pointer denotes.
///
/// **Every button the toolkit can report is asked**, and the ones the
/// viewer binds nothing to are dropped by [`viewer_button`] rather
/// than by omission: a button missing from this loop produces no
/// event at all, which every reader downstream cannot tell from a
/// button nobody pressed.
///
/// **Which click selects is [`input::InputMap::select_button`]'s to
/// decide**, so a click of any bound button is produced and `pick`
/// reads the binding. A click carries the cursor position, so one
/// with no position is not an event.
///
/// Drags come before clicks: [`input::fold_events`] applies the camera
/// in stream order and [`input::pick_stream`] reads the cursor in
/// stream order.
fn button_events(
    response: &egui::Response,
    shift: bool,
    alt: bool,
    pixels_per_point: f64,
    cursor_px: Option<[f64; 2]>,
) -> Vec<ViewportEvent> {
    let mut drags = Vec::new();
    let mut clicks = Vec::new();
    for egui_button in egui_buttons() {
        let Some(button) = viewer_button(egui_button) else {
            continue;
        };
        if response.dragged_by(egui_button) {
            let delta = response.drag_delta();
            drags.push(ViewportEvent::Drag {
                button,
                shift,
                alt,
                delta_px: [
                    f64::from(delta.x) * pixels_per_point,
                    f64::from(delta.y) * pixels_per_point,
                ],
            });
        }
        if let Some(pos_px) = cursor_px
            && response.clicked_by(egui_button)
        {
            clicks.push(ViewportEvent::Click { button, pos_px });
        }
    }
    drags.append(&mut clicks);
    drags
}

impl ViewerBehavior<'_> {
    /// The viewport pane: read the pointer, fold it into camera
    /// operations, then queue the paint callback.
    pub(crate) fn viewport_ui(&mut self, ui: &mut egui::Ui) {
        let (rect, response) =
            ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
        let pixels_per_point = f64::from(ui.ctx().pixels_per_point());
        let viewport = ViewportSize {
            width_px: f64::from(rect.width()) * pixels_per_point,
            height_px: f64::from(rect.height()) * pixels_per_point,
        };
        let Some(aspect) = viewport.aspect() else {
            // **A pane with no extent projects nothing, so it holds no
            // projection refusal.** `view_projection` is not reached
            // below, so the fault would otherwise be a claim about a
            // camera nobody is asking to project — and unlike the
            // sentence this replaced, a badge has no `Clear` to sweep
            // it. Dragging a splitter to zero is an ordinary gesture.
            //
            // This closes that arm and NOT the one where the pane is
            // not drawn at all, which needs a "the viewport did not
            // draw this frame" latch and is
            // `work/view/projection-fault-has-no-sweeper.md`.
            *self.projection_fault = None;
            return;
        };

        let (shift, alt) = ui.input(|i| viewer_modifiers(i.modifiers));
        // The cursor, first: `hover_pos` is in screen POINTS, and the
        // viewport speaks physical pixels from the pane's own top-left
        // corner, so the two conversions happen here and everything
        // below sees one convention. A click carries a position, so
        // the button reading needs this before it can run.
        let cursor_px = response.hover_pos().map(|pos| {
            [
                f64::from(pos.x - rect.min.x) * pixels_per_point,
                f64::from(pos.y - rect.min.y) * pixels_per_point,
            ]
        });
        let mut events = button_events(&response, shift, alt, pixels_per_point, cursor_px);
        if response.hovered() {
            events.extend(ui.input(|i| scroll_event(i.smooth_scroll_delta)));
        }
        match cursor_px {
            Some(pos_px) => {
                events.push(ViewportEvent::Hover { pos_px });
            }
            // Only when there is a hover to clear — the session's own
            // state answers that, so nothing here shadows it.
            None if self.session.hover().is_some() => events.push(ViewportEvent::Leave),
            None => {}
        }
        // An owed fit is taken here and nowhere else: this is the only
        // place with a real aspect to fit against.
        if *self.pending_fit {
            *self.pending_fit = false;
            let fit = CameraOp::Frame {
                bounds: self.scene.bounds(),
                aspect,
            };
            let folded = camera::fold_recorded(self.camera, std::slice::from_ref(&fit));
            land(self.camera, self.notices, self.status, &folded);
        }

        // ONE fold, the same one `map_stream` gives the tests.
        //
        // Landed only when the fold actually MOVED something: the
        // stream carries cursor events too, and a stream that denotes
        // no camera operation is not a camera event. `land` is where a
        // camera MOVE becomes application state, so running it on a
        // frame with no move would make that sentence false — see
        // `frame::folded_moved`, which owns the rule and states what it
        // does and does not buy now that a clean fold clears nothing.
        let folded = input::fold_events(&self.input, self.camera, viewport, &events);
        if frame::folded_moved(&folded) {
            land(self.camera, self.notices, self.status, &folded);
        }

        // **One movement verdict for both picking paths.** The id
        // query's bookkeeping answers "has anything changed under this
        // cursor since the last question", and the CPU ray obeys the
        // same answer: a still cursor over an unchanged picture is a
        // ray cast whose result is already known. Without it an orbit
        // drag ran a full ray cast AND a blocking GPU readback on every
        // frame, because the app pushes a `Hover` whenever the pointer
        // is inside the pane — true of every frame of a drag.
        //
        // **The subject is the pair.** What is under this cursor
        // depends on the picture the id pass reads — which a hidden
        // part changes without moving the generation — and on the
        // index that resolves its ids, which a landing over a refused
        // rebuild changes without moving the picture. `idpass::IdSubject`
        // carries the argument for both.
        let subject = idpass::IdSubject {
            revision: self.revision,
            generation: self.index.map(PickIndex::generation),
        };
        let step = self.id_log.step(cursor_px, subject);
        // **A cursor event retires what the cursor last said.** The id
        // log has just judged whether the outstanding pick question
        // still describes this cursor and this picture; a message
        // about what was under the cursor is stale on exactly that
        // judgement, so `frame::cursor_status` reads it. It only ever
        // expires — what the cursor has to SAY is raised below, where
        // the two picking paths are compared.
        frame::apply(self.status, frame::cursor_status(step));

        // The cursor path: actions in, session operations out. Every
        // step of it — the un-projection, the ray service, the miss
        // rule — lives in `pickindex::PickIndex::op_for`, so this is the
        // same path a headless test drives.
        let actions = input::pick_stream(&self.input, &events);
        // **An open tool narrows the priority rule, it does not
        // re-decide it** — which tool narrows what is
        // `ToolKind::pick_kinds`, an exhaustive match beside the tool
        // vocabulary, and the narrowing travels through `hovered_for`,
        // the one door that answers what a cursor means, so a tool
        // cannot end up on a different rule.
        let kinds = self.tools.pick_kinds();
        // **The index the PICTURE was drawn from**, which is the index
        // in hand on every frame but the ones `drawn_index` exists for.
        // Every read of an index below this line asks this one:
        // the marks composited against the drawn corners' ids, the id
        // pass's answer read back through an id map, and — since Ev's
        // 2026-09-15 ruling — the pick itself, which would otherwise
        // answer about geometry the screen is not showing.
        let on_screen = drawn_index(self.index, self.scene_key);
        // Read before the loop spends `actions`: whether a pick action
        // below asks the ray at this frame's cursor, and so words its
        // refusal itself ([`cursor_news`] reads it).
        let ray_asked = cursor_px.is_some_and(|cursor| ray_asked_at(&actions, step, cursor));
        if let (Some(index), Some(eval)) = (on_screen, self.session.evaluation()) {
            for action in actions {
                if skips_the_ray(action, step) {
                    continue;
                }
                match index.op_under(eval, self.camera, viewport, action, self.display, kinds) {
                    // A hover that changes nothing is not queued: an
                    // operation per frame that performs no transition
                    // is churn in the one log a test reads.
                    Ok(SessionOp::Hover(face)) if face.as_ref() == self.session.hover() => {}
                    Ok(op) => self.ops.push(op),
                    Err(error) => self.notices.push(frame::pick_refusal(&error)),
                }
            }
        } else if let Some(refusal) = pickcache::unindexed(&actions, self.index, self.indexing) {
            // **Not indexed yet is not a miss.** There is nobody to
            // ask about the picture on screen — no index at all while
            // one is built on its own seam, or an index in hand that
            // describes a rebuild nobody has seen — and a click that
            // quietly did nothing here is the fail-quiet this window's
            // indexing indicator would otherwise be painted over.
            //
            // `self.index` rather than `on_screen`: this arm is the
            // `else` of the currency read above, so an index reaching
            // the door is by construction one for another picture,
            // which is the fact `pickcache::unindexed` reads the
            // sentence from.
            self.notices.push(frame::unindexed_refusal(&refusal));
        }

        // What to mark, as a pure function of what is drawn and what is
        // selected. Recomputed every frame; nothing retains it.
        let highlight = on_screen
            .map(|index| marks::highlight(index, self.session.selection(), self.session.hover()));
        // The edge half of the same question, and the same discipline:
        // recomputed every frame from state that lives in one place.
        let mut edges = on_screen
            .map(|index| {
                marks::edge_overlay(
                    index,
                    self.display,
                    self.session.selection(),
                    self.session.hover(),
                )
            })
            .unwrap_or_default();
        // **The three lanes this pane composes itself**, each as the
        // value that owns the display seam's rule
        // ([`marks::LegLane`]) rather than as a bare `Vec` each block
        // narrows into on its own. What they carry beyond the
        // segments is `undrawn()` — how many legs the seam refused —
        // which is the state a reader would have to be told to know
        // that an outline is missing a leg rather than ending where
        // it appears to. Nothing here holds it: the badge that would
        // say so and the field that would carry it between frames are
        // `crate::frame`'s and `crate::app`'s. That half is the VNEWS
        // row `an-overlay-leg-past-the-display-seam-is-not-badged`.
        let mut datums = marks::LegLane::default();
        let mut profiles = marks::LegLane::default();
        let mut preview = marks::LegLane::default();
        // **The open blend tool's held set is marked too** — all of
        // it, because the set IS what the user is composing and a
        // count alone cannot tell them WHICH twelve edges they hold.
        //
        // Marked as SELECTED, the mark meaning "a choice you have
        // made". `BlendTool::mark_segments` applies the same (node,
        // body) narrowing a single selection gets — one pass over the
        // target's drawn edges, so the cost is the body's edge count
        // and not its square.
        if let (Some(index), Some(tool)) = (on_screen, self.tools.blend()) {
            edges
                .selected
                .extend(tool.mark_segments(index, self.display));
        }
        // **The document's construction geometry.** Which lane is drawn
        // over which is `marks::EdgeLane::DRAW_ORDER`'s, not the order
        // these blocks fill them in. Sized against the VIEW
        // (`datums::draws`): a datum has no size of its own, and one
        // sized against the model opens into a hole the moment the
        // camera is closer than a grid cell is wide.
        if let Some((doc, evaluation)) = self.session.landed_pair().filter(|_| *self.show_datums) {
            // **A window this camera has no view of is the projection
            // refusal, said one step earlier and by name.** The door
            // refuses exactly the two quantities `view_projection`
            // refuses below — a viewport dimension that is not finite,
            // or a viewport with no area — so every input that gets
            // here is one the matrix would decline a hundred lines
            // down. `aspect()` has already answered `Some` above, so
            // the arm that actually reaches this door is a dimension
            // that is INFINITE, and the matrix declines those by two
            // different names: an infinite width gives an aspect of
            // `inf` and `NotFinite { what: "aspect" }`, an infinite
            // height an aspect of `0.0` and `UnusableBounds`. Either
            // way the badge names an argument nobody passed; what
            // this writes names the side of the pane that was not a
            // number of pixels. Held in the same field for the same
            // reason: it is true of this camera and this pane on
            // every frame until one of them changes, which is what a
            // badge reads.
            let view = match datum_view(self.camera, viewport) {
                Ok(view) => view,
                Err(error) => {
                    *self.projection_fault = Some(error);
                    return;
                }
            };
            let drawn = datums::draws(doc, evaluation, view);
            // **Counted every frame, never latched.** The count is
            // recomputed here from this frame's drawings and written
            // back by the frame entry point whether or not this pane
            // drew — so a viewport tabbed away reports none rather
            // than leaving yesterday's count standing, which is the
            // hole `work/view/projection-fault-has-no-sweeper.md`
            // records in the field above.
            *self.datums_vanished = drawn.vanished();
            for drawn in drawn.drawn {
                // The same seam and the same rule as [`push_segment`],
                // through the same value: a datum mark's endpoint the
                // GPU cannot hold is not drawn. `datums::draws` sizes
                // every mark against the view and refuses on its own
                // scale, so nothing in tree produces one — the lane is
                // here so the crate has ONE disposition for the
                // narrowing rather than a cast that happens not to
                // fail.
                for leg in drawn.segments.chunks_exact(2) {
                    datums.leg(leg[0], leg[1]);
                }
            }
        }
        // **The profiles the document holds**, from the landed
        // evaluation, every frame — the treatment datums get, because a
        // profile node has no body until something extrudes it and so
        // nothing else would draw it. Not behind the datum toggle: a
        // profile is authored content, not construction geometry.
        //
        // `except`: the profile the edit door is previewing, if any
        // ([`ViewerBehavior::profile_edited`]) — drawn by its live
        // preview below and not also as it was committed, which would
        // show two shapes where there is one. The create door's
        // profile is not a node while it is composed, and comes to
        // rest when its add is accepted (`Drafts::accepted`), so it
        // has nothing to leave out.
        if let Some((doc, evaluation)) = self.session.landed_pair() {
            let committed =
                sketch::committed(doc, evaluation, self.delta.get(), self.profile_edited);
            *self.profiles_undrawn = committed.undrawn.len();
            for profile in &committed.drawn {
                for polyline in &profile.loops {
                    push_loop(&mut profiles, &profile.plane, polyline);
                }
            }
        }
        // **The profile being authored, drawn where it would land.**
        //
        // The form's loops are on a sketch plane, so they HAVE a
        // place: the wireframe goes in the viewport, at that place,
        // rather than into a pane of its own — a preview beside the
        // model cannot show what a preview is mostly for, which is
        // whether the shape is the right size and in the right spot
        // relative to what is already there.
        //
        // Drawn in the probe mark, never the selection mark, because
        // it is not in the document (`EdgeOverlay::preview`). A
        // preview that failed to replay draws nothing and says why in
        // the form; one that replayed but does not VALIDATE draws
        // anyway, which is the case where looking at it is the whole
        // point.
        // Both doors of the one profile editor draw the same way: the
        // add-profile form's loops and an edit's, each where it lands.
        let previews = self
            .profile_previews
            .as_ref()
            .into_array()
            .into_iter()
            .filter_map(|preview| preview.as_ref()?.as_ref().ok());
        for drawn in previews {
            let plane = drawn.plane;
            // The marks are sized in pixels, read at each vertex's own
            // depth — the same door the datum glyphs go through. A
            // window this camera has no view of draws the chain and no
            // marks; the projection refusal below is what says why.
            let view = datum_view(self.camera, viewport).ok();
            for polyline in &drawn.loops {
                let points = &polyline.points;
                push_loop(&mut preview, &plane, polyline);
                let mut segment = |a: [f64; 2], b: [f64; 2]| {
                    push_segment(&mut preview, &plane, a, b);
                };
                // **The directed point at each step.** A tip is a
                // position and, once a verb has bound one, a
                // direction — the pair the lattice calls a directed
                // point, and the thing a person composing a chain is
                // actually reasoning about. The polyline alone shows
                // where the chain went and not where its steps ARE:
                // an arc's flattening puts a dozen indistinguishable
                // points along one leg, which is why
                // `PreviewLoop::vertices` says which of them the loop
                // owns.
                //
                // Each is drawn as a small cross with a tick along the
                // heading. The heading is taken from the polyline
                // itself rather than from bulge arithmetic: the next
                // flattened point IS the tangent to within the chord
                // tolerance, and a second derivation of a direction is
                // a second thing to get wrong.
                for &at in &polyline.vertices {
                    let here = points[at];
                    let Some([dx, dy]) = heading(points, at, polyline.closed) else {
                        continue;
                    };
                    let world = plane.to_world(pncad::geom_core::Point2::new(here[0], here[1]));
                    let Some(tick) =
                        view.and_then(|view| view.screen_metres_at(world, TIP_MARK_PX))
                    else {
                        continue;
                    };
                    // Both marks are drawn ACROSS the heading, never
                    // along it. A tick that ran along the chain would
                    // lie on the leg already drawn there and be
                    // invisible on every vertex but an open chain's
                    // last — which is the one place a reader needs it
                    // least.
                    let (nx, ny) = (-dy, dx);
                    let at_offset = |along: f64, across: f64| {
                        [
                            here[0] + dx * along * tick + nx * across * tick,
                            here[1] + dy * along * tick + ny * across * tick,
                        ]
                    };
                    // The position: a tick through the point, square
                    // to the path.
                    segment(at_offset(0.0, -0.5), at_offset(0.0, 0.5));
                    // The direction: an arrowhead just ahead of it,
                    // opening backward, so the pair reads as "here,
                    // going that way".
                    let tip = at_offset(1.0, 0.0);
                    segment(tip, at_offset(0.2, 0.45));
                    segment(tip, at_offset(0.2, -0.45));
                }
            }
        }

        edges.datums = datums.into_segments();
        edges.profiles = profiles.into_segments();
        edges.preview = preview.into_segments();

        // **Held, not said.** A view matrix that cannot be formed is
        // true of this camera on every frame until it moves somewhere
        // one can be, so it is a read the toolbar badges
        // (`frame::projection_badge`) rather than a sentence. As a
        // sentence it was written here, AFTER the toolbar had already
        // painted the line, and `perform_batch` then ran after this
        // pane — so on every frame whose batch acted cleanly the
        // `Clear` took it before any frame drew it.
        // **The matrix the GPU will actually hold**, not the algebra's
        // own: `Camera::view_projection_f32` is the camera's door at
        // the display seam, and a projection this module can form and
        // a GPU cannot hold refuses here by the same route and into
        // the same badge as one the camera could not form at all.
        let matrix = match self.camera.view_projection_f32(aspect) {
            Ok(matrix) => {
                *self.projection_fault = None;
                matrix
            }
            Err(error) => {
                *self.projection_fault = Some(error);
                return;
            }
        };

        // The two paths' agreement, compared BY NAME
        // (`idpass::disagreement` says why ids are the wrong currency,
        // and records the ray-authoritative role inversion against
        // GQ6-RESURVEY §3). Reported, never resolved.
        //
        // **The ray side of this comparison is the FACE under the
        // cursor, not the hover.** An id buffer can answer with a
        // patch and nothing else, so the question both sides must
        // answer is "which patch is here"; the hover answers a
        // different one as soon as the priority rule picks an edge,
        // and feeding it would report a disagreement between two
        // questions on every frame the cursor came within
        // `EDGE_PICK_RADIUS_PX` of an edge. So the faces are re-derived
        // through `faces_under_cursor`, and only where there is a fresh
        // answer waiting for them — `disagreement` still owns the
        // freshness rule, this only declines to do the work when no
        // question is outstanding at all. A ray path that could not be
        // asked — no evaluation to ask it of — is no comparison either.
        let outstanding = self.id_log.outstanding();
        let said = outstanding.and_then(|_| {
            let question = RayQuestion {
                eval: self.session.evaluation()?,
                camera: self.camera,
                viewport,
                cursor: cursor_px?,
                display: self.display,
            };
            cursor_news(
                on_screen?,
                question,
                self.id_answer.load(Ordering::Relaxed),
                outstanding,
                ray_asked,
            )
        });
        if let Some(news) = said {
            self.notices.push(news);
        }

        // **The pane's own numbers at the same seam the matrix just
        // crossed.** The size the renderer is told and the point
        // scale the edge pass sizes marks with are the last two `f64`
        // the GPU sees, and they cross the one narrowing rather than
        // a cast written twice here. What this arm buys is that the
        // seam has ONE disposition, not a door that refuses over
        // there and a cast that cannot fail over here.
        //
        // **It drops the frame and says nothing, and that is the
        // whole of what it should do.** The other two dispositions of
        // this seam reach a reader — a scene refuses as a whole and a
        // projection that will not narrow badges — and this one does
        // not, deliberately: no input can reach it. `aspect()` above
        // has already declined an extent that is not positive and
        // finite, and what is left is an extent in physical pixels
        // above `f32::MAX`, about `3.4e38`, which is not a window and
        // not a scale factor. A badge here would name a state no
        // person can put the pane into, on a frame no person can see,
        // so the refusal is spent on not drawing an infinity and on
        // nothing else. An overlay LEG past the seam is the opposite
        // case and is badged (`marks::LegLane::undrawn`, and the
        // VNEWS row above): that one is authorable, and it leaves a
        // picture a person is looking at.
        let (Some(viewport_px), Some(point_scale)) = (
            [viewport.width_px, viewport.height_px].narrow(),
            pixels_per_point.narrow(),
        ) else {
            return;
        };
        let id_query = match (step, cursor_px) {
            (IdStep::Ask { serial }, Some(cursor)) => viewport
                .ndc_of(cursor)
                .and_then(|ndc| ndc.narrow())
                .map(|cursor_ndc| IdQuery {
                    cursor_ndc,
                    viewport_px,
                    serial,
                    answer: Arc::clone(self.id_answer),
                }),
            _ => None,
        };

        // **The ground first, then the picture on it.** The pane
        // allocates its rectangle and the paint callback fills only
        // what the model covers, so without this the pixels around a
        // part are whatever the window happened to be cleared to —
        // the toolkit's colour, not the palette's. The palette states
        // it (`Theme::ground`) and this is the one place it is drawn.
        ui.painter()
            .rect_filled(rect, 0.0, chrome(self.theme.ground));
        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
            rect,
            ViewportCallback {
                scene: Arc::clone(self.scene),
                revision: self.revision,
                view_projection: matrix,
                light_direction: LIGHT_DIRECTION,
                theme: self.theme,
                viewport_px,
                pixels_per_point: point_scale,
                highlight: highlight.unwrap_or_default(),
                edges,
                id_query,
            },
        ));
    }
}

/// **Where a real fold meets a real landing.**
///
/// The rules [`land`] obeys are values in [`crate::frame`] and are
/// asserted there over hand-built values, because `frame` is a
/// vocabulary and has to be testable with no session in existence.
/// This module is the driver, so the rows that need one are here: the
/// wiring — does `land` still ASK — and the composition the issue
/// reproduces, a document landing with a fault on the same frame it
/// books its own re-frame.
#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]

    use eframe::egui;

    use super::{
        RayQuestion, button_events, cursor_news, drawn_index, egui_buttons, land, push_loop,
        push_segment, ray_asked_at, scroll_event, viewer_button, viewer_modifiers,
    };
    use crate::camera::{self, Camera, CameraOp, fold_recorded};
    use crate::display::DisplayView;
    use crate::frame::{self, product_badge};
    use crate::idpass::{self, IdStep};
    use crate::input::{self, InputMap, PointerButton, ViewportEvent, ViewportSize};
    use crate::marks;
    use crate::pickcache::{self, NotIndexed};
    use crate::pickindex::{IdMap, PickError, PickIndex, PictureKey};
    use crate::props::SlotValue;
    use crate::scene::{self, DisplayTolerance};
    use crate::session::{DocSession, SessionOp};
    use crate::sketch::PreviewLoop;
    use pncad::document::{Doc, ProfileProgram, SlotId};
    use pncad::geom_core::Tol;
    use pncad::prelude::StableName;
    use pncad::select::HitTestError;

    fn framed() -> Camera {
        Camera::framing(&scene::plate_bounds(), 16.0 / 9.0).expect("the plate frames")
    }

    /// The re-frame `fit_on_scene` books when a document lands — the
    /// operation the viewport folds and lands on that very frame.
    fn the_re_frame_an_open_books() -> CameraOp {
        CameraOp::Frame {
            bounds: scene::plate_bounds(),
            aspect: 16.0 / 9.0,
        }
    }

    #[test]
    fn landing_a_clean_fold_does_not_clear_a_message_it_did_not_write() {
        let fit = the_re_frame_an_open_books();
        let mut camera = framed();
        let folded = fold_recorded(&camera, std::slice::from_ref(&fit));
        assert!(folded.refused.is_none(), "the re-frame applies");

        // A message about the DOCUMENT: a clean fold retires what the
        // camera said and nothing else, so this row goes red if the
        // expiry reaches past its own subject.
        let landing =
            frame::Message::new(frame::Subject::Document, "product: the landing's own news");
        let mut status = Some(landing.clone());
        let mut notices = Vec::new();
        land(&mut camera, &mut notices, &mut status, &folded);
        assert_eq!(camera, folded.camera, "the camera still lands");
        assert_eq!(
            status,
            Some(landing),
            "and the line is not the fold's to clear"
        );
    }

    /// **A refused fold is NEWS, so it joins the frame rather than
    /// writing the line.**
    ///
    /// It used to assign the field here, which is what this sweep
    /// removed: `perform_batch` runs after the panes have drawn, so a
    /// sentence written straight to the field was erased by the same
    /// frame's accepted batch before the toolbar painted it. Going
    /// through `notices` puts it in `frame::frame_status`'s rank 2,
    /// where the batch's verdict can no longer outrank it.
    ///
    /// The older sentence on the line is left ALONE — a notice adds to
    /// what the frame has to say and takes nothing away — and the
    /// ranking is what decides between them.
    #[test]
    fn landing_a_refused_fold_is_news_and_joins_the_frames_notices() {
        let mut camera = framed();
        let refuses = CameraOp::Dolly { factor: 0.0 };
        let folded = fold_recorded(&camera, std::slice::from_ref(&refuses));
        let older = frame::Message::new(frame::Subject::Document, "older news");
        let mut status = Some(older.clone());
        let mut notices = Vec::new();
        land(&mut camera, &mut notices, &mut status, &folded);

        assert_eq!(
            notices.len(),
            1,
            "a refused fold is one notice: {notices:?}"
        );
        let raised = notices.first().expect("the notice just asserted");
        assert!(
            raised.text().contains("camera:") && raised.text().contains("dolly by a factor"),
            "{raised}"
        );
        assert_eq!(raised.subject(), frame::Subject::Camera);
        assert_eq!(
            status,
            Some(older),
            "and it writes nothing: the ranking decides, not the writer"
        );
    }

    /// **The refusal is put on the line by the RANKING, not by hand.**
    ///
    /// The two frames are composed the way the frame loop composes
    /// them: `land` on the first, then `frame::frame_status` over the
    /// notices it produced and `frame::apply` for the verdict — which
    /// is `perform_batch`'s own pair, with an empty batch because
    /// navigating acts on nothing. Reaching into `notices` for the
    /// message would assert the retirement against a sentence this row
    /// placed rather than one the frame landed, and the subject is
    /// exactly what the ranking decides: `frame::joined_subject`
    /// answers `Document` for two notices that disagree, and the
    /// `Expire(Camera)` below would then retire nothing. One notice is
    /// the case where the two answers coincide, and that coincidence
    /// is the row's premise rather than a step it skips.
    #[test]
    fn landing_a_clean_fold_retires_the_camera_refusal_it_landed_before() {
        // The item's own reproduction, through the driver: refuse a
        // camera operation, then navigate. Nothing acts, so nothing
        // clears the line, and before the subject rule the refusal
        // stayed for as long as the user orbited.
        let mut camera = framed();
        let refuses = CameraOp::Dolly { factor: 0.0 };
        let mut status = None;
        let folded = fold_recorded(&camera, std::slice::from_ref(&refuses));
        let mut notices = Vec::new();
        land(&mut camera, &mut notices, &mut status, &folded);
        assert_eq!(notices.len(), 1, "the refusal is news the frame carries");

        // The end of that frame: the ranking weighs what the frame
        // said against a batch that did nothing, and the winner
        // becomes the line.
        frame::apply(&mut status, frame::frame_status(&notices, &[], None));
        let landed = status.clone().expect("the ranking put the refusal up");
        assert_eq!(
            landed.subject(),
            frame::Subject::Camera,
            "and it is the RANKING that says what the line is about: {landed}"
        );

        let orbit = CameraOp::Orbit {
            yaw: 0.2,
            pitch: 0.1,
        };
        let folded = fold_recorded(&camera, std::slice::from_ref(&orbit));
        assert!(folded.refused.is_none(), "the orbit applies");
        let mut notices = Vec::new();
        land(&mut camera, &mut notices, &mut status, &folded);
        assert!(
            notices.is_empty(),
            "a clean fold has nothing to say: {notices:?}"
        );
        assert_eq!(
            status, None,
            "and the next camera event retires it, whatever that event says"
        );
    }

    #[test]
    fn a_gather_fault_the_tree_cannot_badge_outlives_the_open_that_raised_it() {
        // The composition the issue reproduces, end to end and through
        // the real doors: a landed pair whose product does not gather,
        // and the re-frame that landing books, on one frame.
        //
        // The fault is built by hand rather than provoked, and that is
        // the honest way round. A fault a document can REACH by an
        // ordinary edit — a root driven to a zero distance — is a
        // failed root, which the feature tree badges at the node and
        // `product_badge` therefore declines. The faults this channel
        // is for are gather-level and emission-level: they are not
        // authorable from the panels, which is exactly why nothing else
        // reports them.
        let tol = Tol::witness();
        let (doc, extrude) = scene::plate_with_hole(tol).expect("the plate authors");
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        assert!(
            session.product_fault().is_none(),
            "the plate's own product gathers: {:?}",
            session.product_fault()
        );

        // A landing that DOES fault, reached the way a user reaches it.
        let outcome = session.perform(SessionOp::SetSlot {
            node: extrude,
            slot: SlotId::Distance,
            value: SlotValue::Continuous(0.0),
        });
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        session.pump();
        let fault = session.product_fault().expect("the gather refuses");
        // …and it is one the tree carries, so the badge stays silent
        // and the tree row is the channel. Both halves asserted, since
        // silence is only correct while the other channel speaks.
        assert!(
            product_badge(Some(fault)).is_none(),
            "a failed root is the tree's to badge: {fault}"
        );
        assert!(
            session
                .tree_rows()
                .iter()
                .any(|row| matches!(row.status, crate::tree::RowStatus::Failed { .. })),
            "and the tree does badge it"
        );

        // Now the frame the issue is about: the message the landing
        // raised, and the re-frame the same landing booked. Whatever
        // the line holds when the fit is landed, the fit is not what
        // takes it away.
        let mut camera = framed();
        let fit = the_re_frame_an_open_books();
        let folded = fold_recorded(&camera, std::slice::from_ref(&fit));
        let raised = frame::Message::new(
            frame::Subject::Document,
            "product: two roots collide in the name table",
        );
        let mut status = Some(raised.clone());
        let mut notices = Vec::new();
        land(&mut camera, &mut notices, &mut status, &folded);
        assert_eq!(
            status,
            Some(raised),
            "the re-frame an Open books is not news and erases none"
        );
    }

    /// A pane that senses what the viewport's does, driven by raw
    /// `egui` events.
    ///
    /// The viewport pane itself needs a GPU, a session and a scene;
    /// what the rows below are about is one part of it — the
    /// translation from what the toolkit says the pointer did to the
    /// vocabulary `input` consumes — so the probe allocates the same
    /// [`egui::Sense`] over a bare `Ui` and reads the same three
    /// functions [`ViewerBehavior::viewport_ui`] does:
    /// [`viewer_modifiers`], [`button_events`] and [`scroll_event`].
    struct Pane {
        ctx: egui::Context,
    }

    /// Where the probe's pointer aims: the middle of its 800x600
    /// screen, which is inside the pane the probe allocates.
    const AIM: egui::Pos2 = egui::pos2(400.0, 300.0);

    /// [`AIM`] in the pane's own physical pixels — the pane fills the
    /// screen from its origin and the probe runs at one pixel per
    /// point, so the two agree.
    const AIM_PX: [f64; 2] = [400.0, 300.0];

    impl Pane {
        fn new() -> Self {
            Self {
                ctx: egui::Context::default(),
            }
        }

        /// Run one frame and hand back the pointer events the viewport
        /// would read from it.
        fn frame(&self, events: Vec<egui::Event>) -> Vec<ViewportEvent> {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 600.0),
                )),
                events,
                ..Default::default()
            };
            let mut read = Vec::new();
            let out = &mut read;
            let mut output = self.ctx.clone().run_ui(input, |ui| {
                let (rect, response) =
                    ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
                let cursor_px = response
                    .hover_pos()
                    .map(|pos| [f64::from(pos.x - rect.min.x), f64::from(pos.y - rect.min.y)]);
                let (shift, alt) = ui.input(|i| viewer_modifiers(i.modifiers));
                let mut events = button_events(&response, shift, alt, 1.0, cursor_px);
                if response.hovered() {
                    events.extend(ui.input(|i| scroll_event(i.smooth_scroll_delta)));
                }
                *out = events;
            });
            // A frame's texture upload is the caller's to apply; this
            // probe paints nothing, and dropping it unapplied panics.
            output.textures_delta.clear();
            read
        }

        /// Lay the pane out and put the pointer on it. Two empty
        /// frames first: egui interacts against the PREVIOUS frame's
        /// widget rects, so nothing is hittable until one has been
        /// laid out.
        fn reach(&self) {
            self.frame(Vec::new());
            self.frame(Vec::new());
            self.frame(vec![egui::Event::PointerMoved(AIM)]);
        }
    }

    fn button(button: egui::PointerButton, pressed: bool, pos: egui::Pos2) -> egui::Event {
        egui::Event::PointerButton {
            pos,
            button,
            pressed,
            modifiers: egui::Modifiers::NONE,
        }
    }

    /// Press and release without moving: what the toolkit calls a
    /// click. The release frame is the one that reports it.
    fn click(pane: &Pane, egui_button: egui::PointerButton) -> Vec<ViewportEvent> {
        pane.reach();
        pane.frame(vec![button(egui_button, true, AIM)]);
        pane.frame(vec![button(egui_button, false, AIM)])
    }

    /// Press and move: what the toolkit calls a drag. The moving frame
    /// is the one that reports it.
    fn drag(pane: &Pane, egui_button: egui::PointerButton) -> Vec<ViewportEvent> {
        drag_modified(pane, egui_button, egui::Modifiers::NONE)
    }

    /// The same drag with `modifiers` held.
    ///
    /// The modifier change is its own event because that is how the
    /// toolkit carries one: `InputState` keeps the modifier state
    /// across frames and only `ModifiersChanged` moves it, so a
    /// modifier named on a pointer event alone never reaches
    /// `i.modifiers`.
    fn drag_modified(
        pane: &Pane,
        egui_button: egui::PointerButton,
        modifiers: egui::Modifiers,
    ) -> Vec<ViewportEvent> {
        pane.reach();
        pane.frame(vec![
            egui::Event::ModifiersChanged(modifiers),
            egui::Event::PointerButton {
                pos: AIM,
                button: egui_button,
                pressed: true,
                modifiers,
            },
        ]);
        pane.frame(vec![egui::Event::PointerMoved(AIM + egui::vec2(40.0, 0.0))])
    }

    /// A wheel turn small enough that the toolkit hands it straight
    /// over: `InputState` smooths a wheel of eight points or more
    /// across several frames, and this probe reads one frame.
    const WHEEL_POINTS: f32 = 4.0;

    /// Turn the wheel over the pane with `modifiers` held, and hand
    /// back what the viewport read from that frame.
    fn turn_wheel(pane: &Pane, modifiers: egui::Modifiers) -> Vec<ViewportEvent> {
        pane.reach();
        pane.frame(vec![
            egui::Event::ModifiersChanged(modifiers),
            egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(0.0, WHEEL_POINTS),
                phase: egui::TouchPhase::Move,
                modifiers,
            },
        ])
    }

    /// The scroll a plain [`turn_wheel`] denotes.
    fn a_wheels_scroll() -> ViewportEvent {
        ViewportEvent::Scroll {
            units: f64::from(WHEEL_POINTS) / 50.0,
        }
    }

    /// A δ, coarse enough to index the plate quickly.
    fn a_delta(mm: f64) -> DisplayTolerance {
        DisplayTolerance::new(mm * 1.0e-3).expect("a positive δ")
    }

    /// The plate, landed, plus the index of its landed evaluation at
    /// `delta`.
    fn plate_index(session: &DocSession, delta: DisplayTolerance) -> PickIndex {
        let (doc, eval) = session.landed_pair().expect("the inline seam lands");
        let generation = session
            .landed_generation()
            .expect("a landed evaluation has a generation");
        PickIndex::build(doc, eval, PictureKey::of(generation, delta), session.tol())
            .expect("the plate indexes")
    }

    /// **The picture's alphabet is `(generation, δ)`, and the guard
    /// holds both halves.**
    ///
    /// The half a generation-only check would drop is δ: a δ typed
    /// while the document stands rebuilds the index at the SAME
    /// generation over a different tessellation, so the id map is a
    /// different alphabet under an identical generation. A guard that
    /// compared generations would pass the cross pairing below and read
    /// as co-identity while checking something else — which is the
    /// shape this unit was sent to remove, not to re-mint.
    #[test]
    fn the_drawn_index_is_the_one_whose_generation_and_delta_the_picture_carries() {
        let tol = Tol::witness();
        let (doc, extrude) = scene::plate_with_hole(tol).expect("the plate authors");
        let mut session = DocSession::inline(doc, tol);
        session.pump();

        let coarse = plate_index(&session, a_delta(0.5));
        let fine = plate_index(&session, a_delta(0.05));
        let key = PickIndex::key;
        assert_eq!(
            coarse.generation(),
            fine.generation(),
            "both index the same landed evaluation, so only δ separates them"
        );
        assert_ne!(
            coarse.key().delta(),
            fine.key().delta(),
            "and δ does separate them"
        );

        assert!(
            drawn_index(Some(&coarse), Some(key(&coarse))).is_some(),
            "the index the picture was built from IS the drawn index"
        );
        assert!(
            drawn_index(Some(&fine), Some(key(&coarse))).is_none(),
            "an index at another δ did not mint this picture's ids"
        );

        // The other half, over the same predicate: an edit lands a new
        // generation, and the index of it is not the index of the
        // picture still on screen.
        let outcome = session.perform(SessionOp::SetSlot {
            node: extrude,
            slot: SlotId::Distance,
            value: SlotValue::Continuous(0.004),
        });
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        session.pump();
        let edited = plate_index(&session, a_delta(0.5));
        assert_ne!(
            edited.generation(),
            coarse.generation(),
            "the edit landed a new generation"
        );
        assert!(
            drawn_index(Some(&edited), Some(key(&coarse))).is_none(),
            "an index of the edited document did not mint the old picture's ids"
        );
    }

    /// **The false diagnosis this guard exists to stop, shown to be a
    /// real sentence, and shown to be unreachable through the guard.**
    ///
    /// A picture no index minted ids for — the startup mesh, whose
    /// corners all carry [`IdMap::NOTHING`] — makes the id pass answer
    /// *nothing* everywhere. Compared against a ray that names a face,
    /// that is a disagreement, and [`idpass::Disagreement`] writes it to
    /// the status line as *the two picking paths disagree*, which issue
    /// #1097 §4 tells an operator to read as an `R32Uint` clear fault.
    ///
    /// So the first assertion is that the sentence really is produced
    /// by the pairing, and the second is that `drawn_index` never hands
    /// the comparison that pairing.
    #[test]
    fn a_picture_no_index_minted_ids_for_is_not_compared_against_one() {
        let tol = Tol::witness();
        let (doc, _extrude) = scene::plate_with_hole(tol).expect("the plate authors");
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        let index = plate_index(&session, a_delta(0.5));

        let id = index.ids().ids().next().expect("the plate draws patches");
        let named = index
            .name_of(id)
            .expect("an id of this index has an entry")
            .as_ref()
            .expect("and the plate's patches name cleanly")
            .clone();

        let serial = 7u32;
        let nothing = (u64::from(serial) << 32) | u64::from(IdMap::NOTHING);
        let report = idpass::disagreement(
            &index,
            nothing,
            Some(serial),
            Ok(std::slice::from_ref(&named)),
        )
        .expect("nothing-under-the-cursor against a named face is a disagreement");
        assert_eq!(
            report.from_gpu,
            idpass::IdAnswer::Nothing,
            "the id pass answered nothing"
        );
        assert_eq!(report.from_ray, vec![named], "the ray answered a face");

        assert!(
            drawn_index(Some(&index), None).is_none(),
            "a picture with no index behind it is compared against no index"
        );
    }

    /// **A refused ray beside an id-pass face**: the plate's index, an
    /// evaluation of ANOTHER document its hit test refuses before
    /// reading a table (a distinct identity; a twin recipe derives the
    /// same one), a framed cursor, and the channel word of an id-pass
    /// answer naming a face the plate really draws.
    struct RefusedRay {
        index: PickIndex,
        foreign: DocSession,
        camera: Camera,
        pane: ViewportSize,
        cursor: [f64; 2],
        named: StableName,
        /// The id the named face is drawn under.
        id: u32,
        serial: u32,
        answer: u64,
    }

    fn refused_ray() -> RefusedRay {
        let tol = Tol::witness();
        let (doc, _extrude) = scene::plate_with_hole(tol).expect("the plate authors");
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        let index = plate_index(&session, a_delta(0.5));
        let mut foreign =
            DocSession::inline(Doc::<ProfileProgram>::empty_derived("another", tol), tol);
        foreign.pump();
        let id = index.ids().ids().next().expect("the plate draws patches");
        let named = index
            .name_of(id)
            .expect("an id of this index has an entry")
            .as_ref()
            .expect("and the plate's patches name cleanly")
            .clone();
        let serial = 7u32;
        RefusedRay {
            index,
            foreign,
            camera: framed(),
            pane: ViewportSize {
                width_px: 1600.0,
                height_px: 900.0,
            },
            cursor: [800.0, 450.0],
            named,
            id,
            serial,
            answer: (u64::from(serial) << 32) | u64::from(id),
        }
    }

    /// **A ray path that REFUSED is not a ray path that named
    /// nothing**, at the comparison itself: the same id-pass face is no
    /// verdict against the refusal and a disagreement against an empty
    /// answer, so the type tells the two apart.
    #[test]
    fn a_refused_ray_path_is_no_verdict_against_a_named_face() {
        let fixture = refused_ray();
        let foreign = fixture
            .foreign
            .evaluation()
            .expect("the other document lands");
        let refusal = fixture
            .index
            .faces_under_cursor(
                foreign,
                &fixture.camera,
                fixture.pane,
                fixture.cursor,
                &DisplayView::none(),
            )
            .expect_err("an evaluation of another document is refused");
        assert!(
            matches!(
                refusal,
                PickError::HitTest(HitTestError::EvaluationOfAnotherDocument { .. })
            ),
            "the planted refusal is the kernel declining: {refusal:?}"
        );
        let (index, answer, serial) = (&fixture.index, fixture.answer, Some(fixture.serial));
        assert_eq!(
            idpass::disagreement(index, answer, serial, Err(&refusal)),
            None,
            "a refused ray path is compared against nothing"
        );
        assert_eq!(
            idpass::disagreement(index, answer, serial, Ok(&[])),
            Some(idpass::Disagreement {
                from_gpu: idpass::IdAnswer::Named(fixture.named),
                from_ray: Vec::new(),
            }),
            "while a ray path that answered nothing is contradicted by the face"
        );
    }

    /// **A ray refusal is said exactly once a frame — by the pick path
    /// when it asked, by the comparison when it did not — and never as
    /// a disagreement.** This drives the pane's own wiring
    /// (`ray_asked_at`, `cursor_news`) across the two frames the id
    /// log distinguishes.
    ///
    /// The second frame is the one the pick path skips: the camera
    /// orbits under a still cursor, which the id log reads as `Hold`,
    /// so no hover asks the ray, while the comparison asks it through
    /// the moved camera. The refusal a camera move ALONE can bring on
    /// is the hit test's unnamed-entity arm, which nothing in this
    /// crate can plant; the evaluation of another document stands in
    /// for it, and what this row pins is who says a refusal on a frame
    /// nobody else asked the ray, not which refusal it is.
    #[test]
    fn a_ray_refusal_the_pick_path_did_not_ask_for_is_said_by_the_comparison() {
        let fixture = refused_ray();
        let foreign = fixture
            .foreign
            .evaluation()
            .expect("the other document lands");
        let display = DisplayView::none();
        let at = fixture.cursor;
        let actions = [input::PickAction::Hover(at)];
        let subject = idpass::IdSubject {
            revision: 1,
            generation: Some(fixture.index.generation()),
        };
        let mut log = idpass::IdQueryLog::new();

        // The cursor arrives: a new question, and the pick path asks
        // the ray there — so it says the refusal, and the comparison
        // says nothing.
        let arrived = log.step(Some(at), subject);
        let serial = match arrived {
            IdStep::Ask { serial } => Some(serial),
            IdStep::Hold | IdStep::Void => None,
        }
        .expect("a cursor arriving is a new question");
        // The id pass's answer to THIS question, naming a face: fresh,
        // so a ray answer read as empty would be a disagreement.
        let answer = (u64::from(serial) << 32) | u64::from(fixture.id);
        assert!(
            ray_asked_at(&actions, arrived, at),
            "the hover asks the ray"
        );
        let asked = RayQuestion {
            eval: foreign,
            camera: &fixture.camera,
            viewport: fixture.pane,
            cursor: at,
            display: &display,
        };
        assert_eq!(
            cursor_news(&fixture.index, asked, answer, log.outstanding(), true),
            None,
            "the pick path said this refusal; the comparison adds nothing"
        );

        // The camera orbits; the cursor does not move.
        let moved = camera::apply(
            &fixture.camera,
            &CameraOp::Orbit {
                yaw: 0.3,
                pitch: 0.1,
            },
        )
        .expect("the orbit applies");
        let held = log.step(Some(at), subject);
        assert_eq!(held, IdStep::Hold, "the id log does not read the camera");
        assert!(
            !ray_asked_at(&actions, held, at),
            "so the hover skips the frame and nobody on the pick path asks the ray"
        );
        let unasked = RayQuestion {
            camera: &moved,
            ..asked
        };
        let refusal = fixture
            .index
            .faces_under_cursor(foreign, &moved, fixture.pane, at, &display)
            .expect_err("the moved ray is refused");
        assert_eq!(
            cursor_news(&fixture.index, unasked, answer, log.outstanding(), false),
            Some(frame::pick_refusal(&refusal)),
            "the comparison says the refusal in the pick path's own words"
        );
    }

    /// **What the pane says about an id the drawn index never
    /// assigned**, through its own wiring (`cursor_news`) over a real
    /// plate: the id, the ray's answer at the first cursor whose ray
    /// answer `wanted` accepts, and the news.
    ///
    /// An unassigned id is what a corrupt readback looks like. Read as
    /// the clear value, it was misreported both ways: as *id buffer
    /// nothing* against a ray that named a face, and as silent
    /// agreement against a ray that named nothing. The two rows below
    /// each ask one of those cursors.
    fn unassigned_id_news(
        wanted: impl Fn(&[StableName]) -> bool,
    ) -> (u32, Vec<StableName>, Option<frame::Message>) {
        let tol = Tol::witness();
        let (doc, _extrude) = scene::plate_with_hole(tol).expect("the plate authors");
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        let index = plate_index(&session, a_delta(0.5));
        let eval = session.evaluation().expect("the plate lands");
        let unassigned = index.ids().ids().max().expect("the plate draws patches") + 1;
        assert!(
            index.name_of(unassigned).is_none(),
            "an id past the last assigned one is no entry of this index"
        );
        let camera = framed();
        let pane = ViewportSize {
            width_px: 1600.0,
            height_px: 900.0,
        };
        let display = DisplayView::none();
        // The pane's corner, then out from the plate's centre (its
        // hole) along the horizontal.
        let (cursor, from_ray) = std::iter::once([1.0, 1.0])
            .chain((0..16).map(|step| [800.0 - 40.0 * f64::from(step), 450.0]))
            .map(|cursor| {
                let faces = index
                    .faces_under_cursor(eval, &camera, pane, cursor, &display)
                    .expect("the plate's ray path answers");
                (
                    cursor,
                    faces.into_iter().map(|face| face.name).collect::<Vec<_>>(),
                )
            })
            .find(|(_, names)| wanted(names))
            .expect("some cursor's ray answer is the one asked for");

        let mut log = idpass::IdQueryLog::new();
        let subject = idpass::IdSubject {
            revision: 1,
            generation: Some(index.generation()),
        };
        let serial = match log.step(Some(cursor), subject) {
            IdStep::Ask { serial } => Some(serial),
            IdStep::Hold | IdStep::Void => None,
        }
        .expect("a cursor arriving is a new question");
        let question = RayQuestion {
            eval,
            camera: &camera,
            viewport: pane,
            cursor,
            display: &display,
        };
        let answer = (u64::from(serial) << 32) | u64::from(unassigned);
        let news = cursor_news(&index, question, answer, log.outstanding(), true);
        (unassigned, from_ray, news)
    }

    /// Over what the ray calls empty space, an unassigned id is said,
    /// not agreed with.
    #[test]
    fn an_unassigned_id_over_empty_space_is_said_as_that_id() {
        let (id, _, news) = unassigned_id_news(<[StableName]>::is_empty);
        assert_eq!(
            news,
            Some(frame::Message::new(
                frame::Subject::Cursor,
                format!(
                    "picking paths disagree at the cursor: id buffer id {id}, \
                     which no patch of this picture draws, ray nothing"
                ),
            )),
        );
    }

    /// Against a ray that named a face, an unassigned id is said as the
    /// id, not as the id buffer naming nothing.
    #[test]
    fn an_unassigned_id_against_a_named_face_is_said_as_that_id() {
        let (id, from_ray, news) = unassigned_id_news(|names| names.len() == 1);
        let named = from_ray
            .first()
            .expect("the helper returns the one-face answer it was asked for");
        assert_eq!(
            news.expect("an unassigned id against a named face is a disagreement")
                .text(),
            format!(
                "picking paths disagree at the cursor: id buffer id {id}, \
                 which no patch of this picture draws, ray {named} ({:?})",
                named.path
            ),
        );
    }

    /// **A click over a picture the index in hand did not draw is
    /// refused, typed** — Ev's ruling, 2026-09-15.
    ///
    /// This row is the PAIR of predicates the pick path composes, over
    /// a real session: `drawn_index` answers `None` for an index that
    /// did not mint the picture's corners, and the index it declined
    /// is what `pickcache::unindexed` reads
    /// [`NotIndexed::AnotherPicture`] from. Each assertion can fail for
    /// the reason the rule exists: a generation-only guard passes the
    /// cross pairing, and a door that ignored the held index says *the
    /// picture has no pick index* over one that plainly has.
    ///
    /// **What it does not assert is the wiring**, and no row in this
    /// crate can. `ViewerBehavior::viewport_ui` is a private method
    /// over an `egui::Ui` that paints through a wgpu callback and
    /// borrows twenty-odd fields of the application, so it is driven by
    /// nothing headless — the probe below reaches only the event
    /// translation, for the same reason. That the pick path asks
    /// `drawn_index` rather than the index in hand is held by its one
    /// call site being one line, the way
    /// `crates/viewer/tests/panel_display.rs` records for the
    /// parameter field's widget.
    #[test]
    fn a_click_over_a_picture_the_index_did_not_draw_is_refused_typed() {
        let tol = Tol::witness();
        let (doc, _extrude) = scene::plate_with_hole(tol).expect("the plate authors");
        let mut session = DocSession::inline(doc, tol);
        session.pump();

        let drawn = plate_index(&session, a_delta(0.5));
        let landed = plate_index(&session, a_delta(0.05));
        let picture = drawn.key();
        assert!(
            drawn_index(Some(&landed), Some(picture)).is_none(),
            "the index in hand describes a rebuild the picture is not"
        );

        let click = [input::PickAction::Select([10.0, 10.0])];
        assert_eq!(
            pickcache::unindexed(&click, Some(&landed), false),
            Some(NotIndexed::AnotherPicture),
            "so the click gets the picture's answer, which is a refusal",
        );
        assert_eq!(
            pickcache::unindexed(
                &[
                    input::PickAction::Hover([10.0, 10.0]),
                    input::PickAction::ClearHover,
                ],
                Some(&landed),
                false,
            ),
            None,
            "and a hover, pushed every frame the pointer is inside the \
             pane, is not news",
        );

        // The other side of the same door: the index that DID draw the
        // picture answers, so nothing is refused over it.
        assert!(
            drawn_index(Some(&drawn), Some(picture)).is_some(),
            "the index that minted the picture's corners still answers",
        );
    }

    /// **[`egui_buttons`] is every toolkit button exactly once, and
    /// that is a THEOREM rather than a reading of the list.**
    ///
    /// Three facts compose to it. The array's length is
    /// `egui::NUM_POINTER_BUTTONS`, which the compiler checks against
    /// the declaration. [`viewer_button`]'s exhaustive match means the
    /// enum has exactly that many variants — an egui that adds one
    /// without raising the constant reds there. And this row says the
    /// entries are pairwise distinct. `n` distinct members of an
    /// `n`-member set are all of them, so the array is a permutation
    /// of the enum: nothing missing, nothing doubled.
    ///
    /// **Without this row the length is the only hold, and length
    /// alone is not membership.** `[Primary; NUM_POINTER_BUTTONS]`
    /// compiles, and the rows below derive their expectations from
    /// [`egui_buttons`] itself, so a doubled entry asks one button
    /// twice and another never — silently, for any pair the viewer
    /// binds nothing to. A complete list held only by its length is
    /// the class this fix was sent to close, and it would have been
    /// re-minted here.
    #[test]
    fn the_toolkits_buttons_are_each_asked_exactly_once() {
        let buttons = egui_buttons();
        for (index, button) in buttons.iter().enumerate() {
            for other in &buttons[index + 1..] {
                assert_ne!(button, other, "{buttons:?} asks a button twice");
            }
        }
    }

    /// **Every button the toolkit can report reaches the pane, and
    /// what [`viewer_button`] says of it is what comes out.**
    ///
    /// This row is over the PLUMBING, and its reach is exactly that.
    /// Buttons come from [`egui_buttons`] and the expectation from
    /// [`viewer_button`], so a button the loop stopped polling fails
    /// here — the defect this row was written for, where a button
    /// produced no event and no reader could tell that from a button
    /// nobody pressed.
    ///
    /// **What it cannot catch is a change to `viewer_button` itself**,
    /// because both sides of the assertion move with it: give `Extra1`
    /// an arm and this row stays green, having asked for the new
    /// answer and got it. The decision that function encodes is held
    /// by [`the_pairing_is_the_one_this_module_intends`] instead, and
    /// the two rows are complementary rather than overlapping.
    #[test]
    fn every_toolkit_button_the_adapter_binds_produces_its_click() {
        for egui_button in egui_buttons() {
            let events = click(&Pane::new(), egui_button);
            let expected: Vec<ViewportEvent> = viewer_button(egui_button)
                .map(|button| ViewportEvent::Click {
                    button,
                    pos_px: AIM_PX,
                })
                .into_iter()
                .collect();
            assert_eq!(events, expected, "clicking {egui_button:?}");
        }
    }

    /// The same plumbing over drags, with the same reach and the same
    /// blind spot: the three main buttons already dragged before this
    /// unit, so what it adds is the side buttons, whose events stop at
    /// [`viewer_button`]'s `None` rather than at a loop that never
    /// asked. Whether `None` is the right answer for them is
    /// [`the_pairing_is_the_one_this_module_intends`]'s to say.
    #[test]
    fn every_toolkit_button_the_adapter_binds_produces_its_drag() {
        for egui_button in egui_buttons() {
            let events = drag(&Pane::new(), egui_button);
            let expected: Vec<ViewportEvent> = viewer_button(egui_button)
                .map(|button| ViewportEvent::Drag {
                    button,
                    shift: false,
                    alt: false,
                    delta_px: [40.0, 0.0],
                })
                .into_iter()
                .collect();
            assert_eq!(events, expected, "dragging {egui_button:?}");
        }
    }

    /// **`select_button` decides which click selects, and the pane
    /// produces the click it names.**
    ///
    /// [`InputMap::select_button`] is a binding: any button of the
    /// vocabulary may hold it, and `InputMap::pick` reads the field
    /// rather than a fixed button. `InputMap` is `pub` with `pub`
    /// fields and re-exported from the crate root, so an embedder can
    /// already write `select_button: Middle` — and before this unit
    /// that setting selected nothing, silently, because the pane
    /// produced a click for `Primary` only.
    ///
    /// **The bound buttons are DERIVED, not listed.** Filtering
    /// [`egui_buttons`] through [`viewer_button`] is every button the
    /// viewer binds, by construction: a hand-written
    /// `[Primary, Secondary, Middle]` here would be a complete list of
    /// [`input::PointerButton`] that nothing forces — the shape
    /// `work/view/viewer-suites-hold-hand-written-complete-variant-lists.md`
    /// catalogues, and one `viewer-vocab-declared-once.sh` names as a
    /// blind spot it cannot see. Derived, the row also widens itself
    /// on the day a side button gains a binding.
    #[test]
    fn a_click_selects_through_whichever_button_the_map_binds() {
        for egui_button in egui_buttons() {
            let Some(select_button) = viewer_button(egui_button) else {
                continue;
            };
            let map = InputMap {
                select_button,
                ..InputMap::DEFAULT
            };
            assert_eq!(
                input::pick_stream(&map, &click(&Pane::new(), egui_button)),
                vec![input::PickAction::Select(AIM_PX)],
                "a click of {select_button:?}, the button this map binds, selects"
            );
        }
    }

    /// **The pairing itself, written a second time so that changing it
    /// by accident is red.**
    ///
    /// Everything else about the adapter is derivable and so is
    /// derived. This is not: which toolkit button denotes which of the
    /// viewer's is a naming decision, with nothing in the tree to
    /// check it against. Swapping two arms of [`viewer_button`]
    /// type-checks, keeps the set complete and the list a permutation,
    /// and leaves every other row here green — because they all ask
    /// that function what to expect. The mouse would simply behave
    /// wrongly.
    ///
    /// So the table is stated twice on purpose, and the second copy
    /// costs an edit that has to be made deliberately in two places.
    /// **That cost is the guard, not a defect in it.** Both copies are
    /// exhaustive matches over a closed enum, so neither can fall
    /// behind the toolkit while the other moves: a sixth
    /// `egui::PointerButton` reds them together.
    /// **A drag carries the two modifiers this viewport binds, and the
    /// other three leave no trace on it.**
    ///
    /// Two claims, and the second is the one nothing else can make.
    /// [`viewer_modifiers`]'s pattern holds the SET of fields it reads;
    /// the PAIRING — shift into the first half of the returned pair,
    /// alt into the second — is a naming decision with nothing to
    /// derive it from, and swapping the two type-checks while turning
    /// every constrain into an orbit and back. That is the same gap
    /// [`viewer_button`]'s doc names for the button table, answered the
    /// same way: a second statement of the table, here at the far end
    /// of a real toolkit frame.
    #[test]
    fn a_drag_carries_the_two_modifiers_this_viewport_binds() {
        for (modifiers, expected) in [
            (egui::Modifiers::NONE, (false, false)),
            (egui::Modifiers::SHIFT, (true, false)),
            (egui::Modifiers::ALT, (false, true)),
            (egui::Modifiers::CTRL, (false, false)),
            (egui::Modifiers::COMMAND, (false, false)),
            (egui::Modifiers::MAC_CMD, (false, false)),
        ] {
            let pane = Pane::new();
            let events = drag_modified(&pane, egui::PointerButton::Middle, modifiers);
            let (shift, alt) = expected;
            assert_eq!(
                events,
                vec![ViewportEvent::Drag {
                    button: PointerButton::Middle,
                    shift,
                    alt,
                    delta_px: [40.0, 0.0],
                }],
                "what a middle drag carries with {modifiers:?} held"
            );
        }
    }

    /// **A plain wheel is the one scroll this viewport binds.**
    ///
    /// The control the three rows below need: each of them asserts that
    /// some modified wheel produces NO scroll, and a probe that never
    /// delivered a wheel event at all would satisfy every one of them.
    #[test]
    fn a_plain_wheel_is_the_one_scroll_this_viewport_binds() {
        let pane = Pane::new();
        assert_eq!(
            turn_wheel(&pane, egui::Modifiers::NONE),
            vec![a_wheels_scroll()]
        );
    }

    /// **A ctrl+wheel produces no viewport event at all, and this
    /// adapter is not where that is decided.**
    ///
    /// The toolkit spends the modifier first: a wheel whose modifiers
    /// match `InputOptions`' `zoom_modifier` goes into
    /// `zoom_factor_delta` and `smooth_scroll_delta` stays at zero, so
    /// there is no `y` left for [`scroll_event`] to read and reading
    /// `ctrl` in [`viewer_modifiers`] would recover nothing. The second
    /// assertion is what makes the first mean anything — it shows the
    /// wheel arrived and where the toolkit put it, which is also the
    /// value a binding would have to read
    /// (`work/view/ctrl-wheel-reaches-no-zoom.md`).
    #[test]
    fn a_ctrl_wheel_is_spent_by_the_toolkit_before_the_adapter_sees_it() {
        let pane = Pane::new();
        assert_eq!(
            turn_wheel(&pane, egui::Modifiers::CTRL),
            Vec::<ViewportEvent>::new(),
            "the gesture CAD and browsers zoom with reaches the viewport as nothing"
        );
        assert_ne!(
            pane.ctx.input(|i| i.zoom_delta()),
            1.0,
            "the wheel did arrive, and the toolkit put it in the zoom accumulator"
        );
    }

    /// **A shift+wheel is folded onto the axis this viewport drops.**
    ///
    /// `horizontal_scroll_modifier` is SHIFT, so the toolkit moves the
    /// whole delta onto `x` before the adapter sees it — the axis
    /// [`scroll_event`] names in order to drop. Shift is a modifier
    /// this viewport DOES bind on a drag, which is why the row is worth
    /// having: the two halves of the decision are independent.
    #[test]
    fn a_shift_wheel_is_folded_onto_the_axis_this_viewport_drops() {
        let pane = Pane::new();
        assert_eq!(
            turn_wheel(&pane, egui::Modifiers::SHIFT),
            Vec::<ViewportEvent>::new()
        );
        assert_ne!(
            pane.ctx.input(|i| i.smooth_scroll_delta.x),
            0.0,
            "the wheel did arrive, on the axis the viewer has no gesture for"
        );
    }

    /// **An alt+wheel is folded onto the axis this viewport binds**, so
    /// it zooms exactly as a plain wheel does. The same toolkit
    /// mechanism as the two rows above, pointing the other way:
    /// `vertical_scroll_modifier` is ALT.
    #[test]
    fn an_alt_wheel_is_folded_onto_the_axis_this_viewport_binds() {
        let pane = Pane::new();
        assert_eq!(
            turn_wheel(&pane, egui::Modifiers::ALT),
            vec![a_wheels_scroll()]
        );
    }

    #[test]
    fn the_pairing_is_the_one_this_module_intends() {
        for egui_button in egui_buttons() {
            let intended = match egui_button {
                egui::PointerButton::Primary => Some(PointerButton::Primary),
                egui::PointerButton::Secondary => Some(PointerButton::Secondary),
                egui::PointerButton::Middle => Some(PointerButton::Middle),
                egui::PointerButton::Extra1 | egui::PointerButton::Extra2 => None,
            };
            assert_eq!(
                viewer_button(egui_button),
                intended,
                "{egui_button:?} denotes the wrong button of the viewer's vocabulary"
            );
        }
    }
    /// **The pane's own producer reaches the display seam's rule**,
    /// and a leg it refuses does not reach the lane.
    ///
    /// `push_segment` PLACES a sketch-plane pair and offers it; what
    /// happens to a leg the seam cannot carry is
    /// [`marks::LegLane`]'s, asserted there over the arithmetic. What
    /// is asserted here is the wiring: that this producer goes
    /// through that value at all, so a corner past `f32::MAX` reaches
    /// the vertex buffer as an absence rather than as an infinity.
    #[test]
    fn a_placed_leg_past_the_display_seam_is_offered_and_refused() {
        let plane = pncad::profile::SketchPlane::xy();
        let mut lane = marks::LegLane::default();
        push_segment(&mut lane, &plane, [0.0, 0.0], [1.0, 0.0]);
        assert_eq!(lane.undrawn(), 0);
        assert_eq!(lane.segments().len(), 2, "one leg is two positions");
        push_segment(&mut lane, &plane, [1.0, 0.0], [7.0e307, 0.0]);
        assert_eq!(
            lane.segments().len(),
            2,
            "the refused leg added no position to the lane"
        );
        assert_eq!(lane.undrawn(), 1);
        assert!(
            lane.segments().iter().flatten().all(|c| c.is_finite()),
            "no infinity reaches the vertex buffer"
        );
    }

    /// **An authored outline with one far corner draws as a gap**, not
    /// as nothing and not as a smear.
    ///
    /// The loop a person could compose in the add-profile form:
    /// ordinary corners and one at `7e307`, a number the form takes
    /// because it is a number. Four legs are authored, two of them
    /// reach the far corner, and what the lane holds afterwards is the
    /// other two plus a count of what it lost — which is the evidence
    /// that the drop is visible in a picture a person is looking at
    /// rather than only in one that is nowhere.
    #[test]
    fn an_authored_loop_with_one_far_corner_draws_its_other_legs() {
        let plane = pncad::profile::SketchPlane::xy();
        let polyline = PreviewLoop {
            points: vec![[0.0, 0.0], [1.0, 0.0], [7.0e307, 0.0], [0.0, 1.0]],
            vertices: vec![0, 1, 2, 3],
            closed: true,
        };
        let mut lane = marks::LegLane::default();
        push_loop(&mut lane, &plane, &polyline);
        assert_eq!(lane.undrawn(), 2, "the two legs that reach the far corner");
        assert_eq!(
            lane.segments().len(),
            4,
            "the two legs between ordinary corners are drawn"
        );
    }
}
