//! The viewport pane: the wgpu surface, the pointer, and the overlays
//! drawn over both.

use std::sync::Arc;
use std::sync::atomic::Ordering;

use eframe::egui;

use crate::app::{ViewerBehavior, chrome, to_f32};
use crate::camera::{self, Camera, CameraOp};
use crate::datums::{self, datum_view};
use crate::frame::{self, IdStep};
use crate::gpu::{IdQuery, ViewportCallback};
use crate::input::{self, PointerButton, ViewportEvent, ViewportSize};
use crate::pick::{self, PickIndex};
use crate::session::SessionOp;
use crate::sketch::{heading, tip_mark};

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
pub(crate) fn land(camera: &mut Camera, status: &mut Option<String>, folded: &camera::Folded) {
    *camera = folded.camera;
    frame::apply(status, frame::fold_status(folded));
}

/// Direction the light travels, world space; a unit vector over the
/// viewer's left shoulder.
const LIGHT_DIRECTION: [f32; 3] = [0.408_248_3, 0.408_248_3, -0.816_496_6];

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
            return;
        };

        let (shift, alt) = ui.input(|i| (i.modifiers.shift, i.modifiers.alt));
        let mut events: Vec<ViewportEvent> = Vec::new();
        for (egui_button, button) in [
            (egui::PointerButton::Primary, PointerButton::Primary),
            (egui::PointerButton::Secondary, PointerButton::Secondary),
            (egui::PointerButton::Middle, PointerButton::Middle),
        ] {
            if response.dragged_by(egui_button) {
                let delta = response.drag_delta();
                events.push(ViewportEvent::Drag {
                    button,
                    shift,
                    alt,
                    delta_px: [
                        f64::from(delta.x) * pixels_per_point,
                        f64::from(delta.y) * pixels_per_point,
                    ],
                });
            }
        }
        if response.hovered() {
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll != 0.0 {
                // egui reports scroll in points; a wheel notch is
                // conventionally 50 of them.
                events.push(ViewportEvent::Scroll {
                    units: f64::from(scroll) / 50.0,
                });
            }
        }
        // Cursor events, in the same stream. `hover_pos` is in screen
        // POINTS; the viewport speaks physical pixels from the pane's
        // own top-left corner, so the two conversions happen here and
        // the mapping below sees one convention.
        let cursor_px = response.hover_pos().map(|pos| {
            [
                f64::from(pos.x - rect.min.x) * pixels_per_point,
                f64::from(pos.y - rect.min.y) * pixels_per_point,
            ]
        });
        match cursor_px {
            Some(pos_px) => {
                if response.clicked_by(egui::PointerButton::Primary) {
                    events.push(ViewportEvent::Click {
                        button: PointerButton::Primary,
                        pos_px,
                    });
                }
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
            land(self.camera, self.status, &folded);
        }

        // ONE fold, the same one `map_stream` gives the tests.
        //
        // Landed only when the fold actually MOVED something: the
        // stream now carries cursor events too, and a stream that
        // denotes no camera operation is not a camera event — landing
        // it would clear the status line on every frame the pointer is
        // inside the viewport.
        let folded = input::fold_events(&self.input, self.camera, viewport, &events);
        if frame::folded_moved(&folded) {
            land(self.camera, self.status, &folded);
        }

        // **One movement verdict for both picking paths.** The id
        // query's bookkeeping answers "has anything changed under this
        // cursor since the last question", and the CPU ray obeys the
        // same answer: a still cursor over an unchanged picture is a
        // ray cast whose result is already known. Without it an orbit
        // drag ran a full ray cast AND a blocking GPU readback on every
        // frame, because the app pushes a `Hover` whenever the pointer
        // is inside the pane — true of every frame of a drag.
        let generation = self.index.map(PickIndex::generation);
        let step = self.id_log.step(cursor_px, generation);

        // The cursor path: actions in, session operations out. Every
        // step of it — the un-projection, the ray service, the miss
        // rule — lives in `pick::PickIndex::op_for`, so this is the
        // same path a headless test drives.
        let actions = input::pick_stream(&self.input, &events);
        // **An open tool narrows the priority rule, it does not
        // re-decide it** — which tool narrows what is
        // `ToolKind::pick_kinds`, an exhaustive match beside the tool
        // vocabulary, and the narrowing travels through `hovered_for`,
        // the one door that answers what a cursor means, so a tool
        // cannot end up on a different rule.
        let kinds = self.tools.pick_kinds();
        if let (Some(index), Some(eval)) = (self.index, self.session.evaluation()) {
            for action in actions {
                // A hover over an unchanged picture at an unmoved
                // cursor asks a question whose answer the session
                // already holds. A click never skips: it is an
                // ACTION, not an observation.
                if step == IdStep::Hold && matches!(action, input::PickAction::Hover(_)) {
                    continue;
                }
                match index.op_under(eval, self.camera, viewport, action, self.display, kinds) {
                    // A hover that changes nothing is not queued: an
                    // operation per frame that performs no transition
                    // is churn in the one log a test reads.
                    Ok(SessionOp::Hover(face)) if face.as_ref() == self.session.hover() => {}
                    Ok(op) => self.ops.push(op),
                    Err(error) => *self.status = Some(error.to_string()),
                }
            }
        }

        // What to mark, as a pure function of what is drawn and what is
        // selected. Recomputed every frame; nothing retains it.
        let highlight = self
            .index
            .map(|index| pick::highlight(index, self.session.selection(), self.session.hover()));
        // The edge half of the same question, and the same discipline:
        // recomputed every frame from state that lives in one place.
        let mut edges = self
            .index
            .map(|index| {
                pick::edge_overlay(
                    index,
                    self.display,
                    self.session.selection(),
                    self.session.hover(),
                )
            })
            .unwrap_or_default();
        // **The open blend tool's held set is marked too** — all of
        // it, because the set IS what the user is composing and a
        // count alone cannot tell them WHICH twelve edges they hold.
        //
        // Marked as SELECTED, the mark meaning "a choice you have
        // made". `BlendTool::mark_segments` applies the same (node,
        // body) narrowing a single selection gets — one pass over the
        // target's drawn edges, so the cost is the body's edge count
        // and not its square.
        if let (Some(index), Some(tool)) = (self.index, self.tools.blend()) {
            edges
                .selected
                .extend(tool.mark_segments(index, self.display));
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
        // **The document's construction geometry**, drawn before the
        // preview so a form composing something over a datum reads on
        // top of it. Sized against the VIEW (`datums::draws`): a datum
        // has no size of its own, and one sized against the model
        // opens into a hole the moment the camera is closer than a
        // grid cell is wide.
        if let Some((doc, evaluation)) = self.session.landed_pair().filter(|_| *self.show_datums) {
            for drawn in datums::draws(doc, evaluation, datum_view(self.camera, viewport)) {
                for point in drawn.segments {
                    edges
                        .datums
                        .push([point[0] as f32, point[1] as f32, point[2] as f32]);
                }
            }
        }
        if let Some(Ok(drawn)) = self.profile_preview {
            let plane = drawn.plane;
            // ONE size for every loop in the preview, from the whole
            // picture's extent: marks that each scaled to their own
            // loop would draw a bore's crosses smaller than its
            // outer's for no reason a reader could name.
            let tick = tip_mark(&drawn.loops);
            for polyline in &drawn.loops {
                let points = &polyline.points;
                // A CLOSED loop's segment list wraps — the last point
                // joins the first, which is the same thing
                // `ProfileLoop` means by being closed by construction.
                // An OPEN one's must not: the leg back to the start is
                // the provisional close `sketch::preview` walked the
                // chain under and nobody authored, so the wrap is
                // dropped and what is drawn is the authored legs
                // exactly. That is the whole of "a path draws while it
                // is still being written".
                let segments = if polyline.closed {
                    points.len()
                } else {
                    points.len().saturating_sub(1)
                };
                let mut segment = |a: [f64; 2], b: [f64; 2]| {
                    for [x, y] in [a, b] {
                        let world = plane.to_world(pncad::geom_core::Point2::new(x, y));
                        edges
                            .preview
                            .push([world.x as f32, world.y as f32, world.z as f32]);
                    }
                };
                for index in 0..segments {
                    segment(points[index], points[(index + 1) % points.len()]);
                }
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

        let matrix = match self.camera.view_projection(aspect) {
            Ok(matrix) => matrix,
            Err(error) => {
                *self.status = Some(format!("projection: {error}"));
                return;
            }
        };

        // The two paths' agreement, compared BY NAME (`frame::
        // disagreement` says why ids are the wrong currency, and
        // records the ray-authoritative role inversion against
        // GQ6-RESURVEY §3). Reported, never resolved.
        //
        // **The ray side of this comparison is the FACE under the
        // cursor, not the hover.** An id buffer can answer with a
        // patch and nothing else, so the question both sides must
        // answer is "which patch is here"; the hover answers a
        // different one as soon as the priority rule picks an edge,
        // and feeding it would report a disagreement between two
        // questions on every frame the cursor came within
        // `EDGE_PICK_RADIUS_PX` of an edge. So the face is re-derived
        // through `face_under_cursor`, and only where there is a fresh
        // answer waiting for it — `disagreement` still owns the
        // freshness rule, this only declines to do the work when no
        // question is outstanding at all.
        let outstanding = self.id_log.outstanding();
        let from_ray = outstanding.and_then(|_| {
            let index = self.index?;
            let eval = self.session.evaluation()?;
            index
                .face_under_cursor(eval, self.camera, viewport, cursor_px?, self.display)
                .ok()
                .flatten()
        });
        if let Some(report) = self.index.and_then(|index| {
            frame::disagreement(
                index,
                self.id_answer.load(Ordering::Relaxed),
                outstanding,
                from_ray.as_ref().map(|face| &face.name),
            )
        }) {
            *self.status = Some(report.to_string());
        }

        let id_query = match (step, cursor_px) {
            (IdStep::Ask { serial }, Some(cursor)) => {
                viewport.ndc_of(cursor).map(|[nx, ny]| IdQuery {
                    cursor_ndc: [nx as f32, ny as f32],
                    viewport_px: [viewport.width_px as f32, viewport.height_px as f32],
                    serial,
                    answer: Arc::clone(self.id_answer),
                })
            }
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
                view_projection: to_f32(&matrix),
                light_direction: LIGHT_DIRECTION,
                theme: self.theme,
                viewport_px: [viewport.width_px as f32, viewport.height_px as f32],
                pixels_per_point: pixels_per_point as f32,
                highlight: highlight.unwrap_or_default(),
                edges,
                id_query,
            },
        ));
    }
}

/// **The one wiring `frame`'s rows cannot reach.**
///
/// Every rule [`land`] obeys is a value in [`crate::frame`] and is
/// asserted there. What is asserted HERE is that `land` still asks:
/// the whole defect was a status-line assignment written at this call
/// site, so a row that only exercises the policy would stay green
/// through the exact regression it is meant to catch.
#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]

    use super::land;
    use crate::camera::{Camera, CameraOp, fold_recorded};
    use crate::scene::PLATE_EXTENT;
    use bvh::Aabb;

    fn plate_bounds() -> Aabb {
        let [width, depth, thickness] = PLATE_EXTENT;
        Aabb {
            min_x: 0.0,
            min_y: 0.0,
            min_z: 0.0,
            max_x: width,
            max_y: depth,
            max_z: thickness,
        }
    }

    fn framed() -> Camera {
        Camera::framing(&plate_bounds(), 16.0 / 9.0).expect("the plate frames")
    }

    #[test]
    fn landing_a_clean_fold_does_not_clear_a_message_it_did_not_write() {
        // The re-frame an opened document books, landed on the same
        // frame the landing raised its own news.
        let fit = CameraOp::Frame {
            bounds: plate_bounds(),
            aspect: 16.0 / 9.0,
        };
        let mut camera = framed();
        let folded = fold_recorded(&camera, std::slice::from_ref(&fit));
        assert!(folded.refused.is_none(), "the re-frame applies");

        let mut status = Some("product: the landing's own news".to_owned());
        land(&mut camera, &mut status, &folded);
        assert_eq!(camera, folded.camera, "the camera still lands");
        assert_eq!(
            status.as_deref(),
            Some("product: the landing's own news"),
            "and the line is not the fold's to clear"
        );
    }

    #[test]
    fn landing_a_refused_fold_shows_the_refusal() {
        let mut camera = framed();
        let refuses = CameraOp::Dolly { factor: 0.0 };
        let folded = fold_recorded(&camera, std::slice::from_ref(&refuses));
        let mut status = Some("older news".to_owned());
        land(&mut camera, &mut status, &folded);
        let shown = status.expect("a refused fold is news");
        assert!(
            shown.contains("camera:") && shown.contains("dolly by a factor"),
            "{shown}"
        );
    }
}
