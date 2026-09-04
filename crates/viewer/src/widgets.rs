//! **The free helpers over `egui::Ui` that take plain data.**
//!
//! Every function here draws one row or one field from values the
//! caller already holds, and returns what the user did with it. None
//! of them borrows the application or the session's mutable half: the
//! pane modules own that, and hand these numbers, units and labels.
//!
//! [`drag_ops`] is the exception worth naming — it is the one mapping
//! from a `DragValue` to session operations, and the whole reason a
//! dragged number in this crate emits one committed edit rather than
//! one per frame.

use eframe::egui;
use pncad::document::{Dimension, RecipeNodeId};
use pncad::profile::{ArcSide, ArcSweep};
use pncad::quantity::{AngleUnit, LengthUnit, UnitDef};

use crate::forms::{ANGLE_DRAG_SPEED, ArcMode, FIELD_DRAG_SPEED, PathVerb, UNIT_DRAG_SPEED};
use crate::props;
use crate::session::{DocSession, SessionOp};
use crate::sketch::{ArcSpec, PathStep, PathTarget};

/// **The one mapping from a `DragValue` to session operations**, and
/// the only place in this crate that turns a widget into a gesture.
///
/// G1 ratifies the shape: a continuous gesture emits previews against
/// scratch state and exactly ONE committed edit on release. egui's
/// `DragValue` does not hand that over — it conflates dragging with
/// typing and fires `changed()` every frame of a drag — so the
/// translation is the `drag_started` / `dragged` / `drag_stopped`
/// triple below.
///
/// **It is a function because the same file once had two copies of it
/// and one of them was wrong**: the slot rows mapped the triple and the
/// document-parameter row mapped a bare `changed()`, so dragging a
/// parameter committed one edit, one undo step and one re-evaluation
/// per frame. Two spellings of a ratified rule is one spelling too
/// many. Any future dragged number in this file calls this; nothing but
/// this comment enforces that, which is the honest state of it.
/// Generalized over the GESTURE VOCABULARY (`preview`/`commit` are
/// parameters) because the free-move probe runs the same triple over
/// display ops rather than document ops — one mapping, two
/// vocabularies, and the typed-input arm (`changed() && !dragged()`)
/// covered for BOTH, which is the arm a hand-mapped copy of this
/// function silently dropped once already.
///
/// The DRAG half is [`drag_gesture_ops`], which the slot field calls
/// directly: a field that reads its own text decides for itself what
/// was typed, so `changed()` is not what tells it.
pub(crate) fn drag_ops(
    widget: &egui::Response,
    value: f64,
    begin: SessionOp,
    preview: impl Fn(f64) -> SessionOp,
    commit: SessionOp,
    typed: impl Fn(f64) -> Vec<SessionOp>,
    ops: &mut Vec<SessionOp>,
) {
    if drag_gesture_ops(widget, value, begin, preview, commit, ops) {
        return;
    }
    if widget.changed() && !widget.dragged() {
        // Typed, not dragged: whatever the vocabulary spells a direct
        // value entry as — one edit for a document slot, a one-shot
        // begin/preview/commit for the display probe.
        ops.extend(typed(value));
    }
}

/// The drag half of [`drag_ops`]'s triple: begin on press, preview on
/// every frame the value moves, commit on release. Answers whether the
/// release happened, i.e. whether this frame's change was a gesture's
/// and belongs to nothing else.
pub(crate) fn drag_gesture_ops(
    widget: &egui::Response,
    value: f64,
    begin: SessionOp,
    preview: impl Fn(f64) -> SessionOp,
    commit: SessionOp,
    ops: &mut Vec<SessionOp>,
) -> bool {
    if widget.drag_started() {
        ops.push(begin);
    }
    if widget.dragged() && widget.changed() {
        ops.push(preview(value));
    }
    if widget.drag_stopped() {
        ops.push(commit);
        return true;
    }
    false
}

/// One labeled row of three draggable components — every creation
/// form's vector fields (a datum's origin and normal, a placement's
/// translation and rotation axis, a pattern's direction).
///
/// **The speed is the caller's** because the unit is: a metre field
/// and a dimensionless direction component want drag rates two orders
/// of magnitude apart, and one shared rate makes one of them
/// undraggable. [`FIELD_DRAG_SPEED`] and [`UNIT_DRAG_SPEED`] are the
/// two values in use.
pub(crate) fn vec3_row(ui: &mut egui::Ui, label: &str, speed: f64, value: &mut [f64; 3]) {
    ui.horizontal(|ui| {
        ui.label(label);
        for component in value {
            ui.add(egui::DragValue::new(component).speed(speed));
        }
    });
}

/// **One dimensioned field of a creation form.**
///
/// The draft behind it is CANONICAL (metres, radians) and the field
/// is what that value looks like written in `unit` — the property
/// panel's own rule (`props::in_written` / `props::from_written`, the
/// text door's one multiply), applied to the forms so a number typed
/// into a form and the same number typed into a panel field mean the
/// same thing.
///
/// Held canonical rather than as-typed for the reason the panel holds
/// it that way: switching the unit is a change of NOTATION, and a
/// draft that stored what was typed would silently become a different
/// length when the picker moved.
///
/// **The drag speed travels through the same conversion**, which is
/// the half of this that is easy to leave out: a tick in metres
/// applied to a field showing millimetres is the same gesture made a
/// thousand times finer by a change of notation.
pub(crate) fn unit_field(ui: &mut egui::Ui, unit: UnitDef, speed: f64, canonical: &mut f64) {
    named_field(ui, "", unit, speed, canonical);
}

/// [`unit_field`] with the quantity's NAME written into the field
/// itself.
///
/// A prefix rather than a `Label` beside it, and that is the point: a
/// path step's row is a horizontal strip of controls, and the labels
/// that used to sit between them belonged to whichever field a reader
/// guessed. `arc_fillet` is the case that made it matter — an arc
/// radius and a fillet radius, both written `r`, both in the same row,
/// with nothing saying which was which. A prefix cannot drift away
/// from its field.
///
/// The name is the QUANTITY, never the unit: the picker beside the
/// form says the unit, and a second statement of it here would be
/// free to disagree ([`length_picker`]'s own rule).
pub(crate) fn named_field(
    ui: &mut egui::Ui,
    name: &str,
    unit: UnitDef,
    speed: f64,
    canonical: &mut f64,
) {
    let mut written = props::in_written(*canonical, unit);
    let mut field = egui::DragValue::new(&mut written).speed(props::in_written(speed, unit));
    if !name.is_empty() {
        field = field.prefix(format!("{name} "));
    }
    let response = ui.add(field);
    // Written back only on a real edit: an untouched field would
    // otherwise round-trip its value through a divide and a multiply
    // every frame, which is a drift nobody asked for.
    if response.changed() {
        *canonical = props::from_written(written, unit);
    }
}

/// A dimensionless field with its own name written in — the scalar
/// twin of [`named_field`], for the components and bulges that carry
/// no unit at all.
pub(crate) fn named_scalar(ui: &mut egui::Ui, name: &str, speed: f64, value: &mut f64) {
    ui.add(
        egui::DragValue::new(value)
            .speed(speed)
            .prefix(format!("{name} ")),
    );
}

/// The vector twin of [`unit_field`] — one label, three components,
/// one unit.
pub(crate) fn unit_vec3_row(
    ui: &mut egui::Ui,
    label: &str,
    unit: UnitDef,
    speed: f64,
    value: &mut [f64; 3],
) {
    ui.horizontal(|ui| {
        ui.label(label);
        for component in value {
            unit_field(ui, unit, speed, component);
        }
    });
}

/// Two Length fields, one point of the sketch frame — each carrying
/// the axis it is, because a row of a path form holds several points
/// and a bare pair of numbers says which of them it belongs to only
/// by position.
pub(crate) fn point_fields(ui: &mut egui::Ui, unit: UnitDef, point: &mut [f64; 2]) {
    for (axis, component) in ["x", "y"].into_iter().zip(point) {
        named_field(ui, axis, unit, FIELD_DRAG_SPEED, component);
    }
}

/// A path verb's target: the entry vertex (which CLOSES the loop), or
/// an authored point.
///
/// The two are one control because they are one decision — where this
/// leg ends — and `Start` is not a point somebody could type: it is
/// the bound entry, and aiming at it is what closing IS in this
/// algebra (`pncad::profile::path`, which has no `close()` alias).
pub(crate) fn target_fields(ui: &mut egui::Ui, unit: UnitDef, target: &mut PathTarget) {
    let closing = matches!(target, PathTarget::Start);
    let mut to_start = closing;
    ui.checkbox(&mut to_start, "to start");
    if to_start != closing {
        *target = if to_start {
            PathTarget::Start
        } else {
            PathTarget::Point([0.01, 0.0])
        };
    }
    if let PathTarget::Point(point) = target {
        point_fields(ui, unit, point);
    }
}

/// Which side of travel an arc's centre sits on.
pub(crate) fn side_picker(ui: &mut egui::Ui, salt: &str, side: &mut ArcSide) {
    egui::ComboBox::from_id_salt(("arc_side", salt))
        .selected_text(match side {
            ArcSide::Left => "left",
            ArcSide::Right => "right",
        })
        .width(64.0)
        .show_ui(ui, |ui| {
            ui.selectable_value(side, ArcSide::Left, "left");
            ui.selectable_value(side, ArcSide::Right, "right");
        });
}

/// Which way round an arc about a named centre travels.
pub(crate) fn winding_picker(ui: &mut egui::Ui, salt: &str, winding: &mut ArcSweep) {
    egui::ComboBox::from_id_salt(("arc_winding", salt))
        .selected_text(match winding {
            ArcSweep::Ccw => "ccw",
            ArcSweep::Cw => "cw",
        })
        .width(64.0)
        .show_ui(ui, |ui| {
            ui.selectable_value(winding, ArcSweep::Ccw, "ccw");
            ui.selectable_value(winding, ArcSweep::Cw, "cw");
        });
}

/// **One arc leg's spec**: which of the six modes, then that mode's
/// own fields.
///
/// Switching the mode REPLACES the spec with a fresh one of the new
/// mode rather than carrying numbers across. The modes do not share a
/// meaning for their fields — a `radius` mode's `r` is a carrier and
/// a `sweep` mode's is the same carrier with a swept angle beside it,
/// but a `via` point is not a radius at all — so a carried number
/// would sometimes be the right one and sometimes be a coincidence,
/// and a form cannot tell which.
pub(crate) fn arc_fields(
    ui: &mut egui::Ui,
    salt: &str,
    role: &str,
    length_unit: UnitDef,
    angle_unit: UnitDef,
    spec: &mut ArcSpec,
) {
    // What to call this arc's own radius. A step can hold TWO arcs and
    // a fillet between them (`arc_fillet_arc`), and every one of the
    // three has a radius: unqualified, all three fields read `r` and
    // the row is unreadable. The caller names the role because only
    // the caller knows which arc of the step this is.
    let radius = if role.is_empty() {
        "r".to_owned()
    } else {
        format!("{role} r")
    };
    let mut mode = ArcMode::of(spec);
    let before = mode;
    egui::ComboBox::from_id_salt(("arc_mode", salt))
        .selected_text(mode.label())
        .width(88.0)
        .show_ui(ui, |ui| {
            for option in ArcMode::ALL {
                ui.selectable_value(&mut mode, option, option.label());
            }
        });
    if mode != before {
        *spec = mode.fresh();
    }
    match spec {
        ArcSpec::Radius { r, side } => {
            named_field(ui, &radius, length_unit, FIELD_DRAG_SPEED, r);
            side_picker(ui, salt, side);
        }
        ArcSpec::Bulge { target, b } => {
            target_fields(ui, length_unit, target);
            named_scalar(ui, "bulge", UNIT_DRAG_SPEED, b);
        }
        ArcSpec::Via { q, target } => {
            ui.label("via");
            point_fields(ui, length_unit, q);
            target_fields(ui, length_unit, target);
        }
        ArcSpec::Center { c, winding, target } => {
            ui.label("centre");
            point_fields(ui, length_unit, c);
            winding_picker(ui, salt, winding);
            target_fields(ui, length_unit, target);
        }
        ArcSpec::Sweep { r, side, angle } => {
            named_field(ui, &radius, length_unit, FIELD_DRAG_SPEED, r);
            side_picker(ui, salt, side);
            named_field(ui, "sweep", angle_unit, ANGLE_DRAG_SPEED, angle);
        }
        ArcSpec::ArcLen { r, side, len } => {
            named_field(ui, &radius, length_unit, FIELD_DRAG_SPEED, r);
            side_picker(ui, salt, side);
            named_field(ui, "arc length", length_unit, FIELD_DRAG_SPEED, len);
        }
    }
}

/// **The step a fresh row starts as**, by where it is going.
///
/// `at` at position 0 — nothing else is well-typed at the entry, so
/// offering anything there would be offering a refusal — and `line_to`
/// anywhere after it, which is the verb a chain is mostly made of. It
/// is a starting point and not a judgement: the row's own combo,
/// narrowed to what the lattice admits at that tip, is where it
/// becomes something else.
pub(crate) fn fresh_step(at: usize) -> PathStep {
    if at == 0 {
        PathVerb::At.fresh()
    } else {
        PathVerb::LineTo.fresh()
    }
}

/// **One authoring verb's own fields.**
///
/// Exhaustive on [`PathStep`], like the lowering it feeds: a verb the
/// vocabulary gains has to be given a row here before it compiles,
/// which is the same protection `crate::sketch`'s lowering has and
/// for the same reason — a verb reachable in one and not the other is
/// a verb nobody can use.
pub(crate) fn path_step_fields(
    ui: &mut egui::Ui,
    salt: &str,
    length_unit: UnitDef,
    angle_unit: UnitDef,
    step: &mut PathStep,
) {
    // **Every field says which quantity it is.** The arms below are
    // split further than the lowering's are — `line` and `fillet` both
    // carry one Length and shared an arm — because what a number MEANS
    // is the thing a row has to say, and a shared arm can only give
    // two different quantities one name.
    match step {
        PathStep::At(point) => point_fields(ui, length_unit, point),
        PathStep::ArcContinue(point) => {
            ui.label("through");
            point_fields(ui, length_unit, point);
        }
        PathStep::FarEndTo(point) => {
            ui.label("far end");
            point_fields(ui, length_unit, point);
        }
        PathStep::Angle(angle) => {
            named_field(ui, "angle", angle_unit, ANGLE_DRAG_SPEED, angle);
        }
        PathStep::Turn(angle) => {
            named_field(ui, "turn", angle_unit, ANGLE_DRAG_SPEED, angle);
        }
        PathStep::Toward { dx, dy } => {
            named_scalar(ui, "dx", UNIT_DRAG_SPEED, dx);
            named_scalar(ui, "dy", UNIT_DRAG_SPEED, dy);
        }
        PathStep::Line(length) => {
            named_field(ui, "length", length_unit, FIELD_DRAG_SPEED, length);
        }
        PathStep::Fillet(radius) => {
            named_field(ui, "fillet r", length_unit, FIELD_DRAG_SPEED, radius);
        }
        PathStep::LineTo(target) | PathStep::TangentArcTo(target) => {
            target_fields(ui, length_unit, target);
        }
        PathStep::ArcTo(spec) => arc_fields(ui, salt, "", length_unit, angle_unit, spec),
        // The two mixed verbs read in the order their names do, so the
        // row is the step spelled left to right.
        PathStep::FilletArc { radius, spec } => {
            named_field(ui, "fillet r", length_unit, FIELD_DRAG_SPEED, radius);
            arc_fields(ui, salt, "arc", length_unit, angle_unit, spec);
        }
        PathStep::ArcFillet { spec, radius } => {
            arc_fields(ui, salt, "arc", length_unit, angle_unit, spec);
            named_field(ui, "fillet r", length_unit, FIELD_DRAG_SPEED, radius);
        }
        PathStep::ArcFilletArc {
            spec,
            radius,
            spec2,
        } => {
            arc_fields(
                ui,
                &format!("{salt}_in"),
                "in arc",
                length_unit,
                angle_unit,
                spec,
            );
            named_field(ui, "fillet r", length_unit, FIELD_DRAG_SPEED, radius);
            arc_fields(
                ui,
                &format!("{salt}_out"),
                "out arc",
                length_unit,
                angle_unit,
                spec2,
            );
        }
        // Structural verbs: the verb IS the whole step.
        PathStep::Tangent | PathStep::Cusp | PathStep::CloseTo => {}
    }
}

/// **The creation forms' written-unit picker.**
///
/// The panel's picker as a form control: the same options
/// (`props::unit_options`, read off the closed unit table) and the
/// same rule about what the label beside a field may say. There is no
/// "nothing chosen" state to fall back from — a form is always
/// authoring in some notation, and says which. **The unit is the picker's
/// to say, not the field's**, which is why the form labels next to
/// these are bare ("radius", not "radius (m)"): a label with the unit
/// baked in is a second place for it to be stated, free to say metres
/// beside a field written in millimetres.
///
/// It differs from the panel's in one way, and deliberately: the
/// panel's picker is an EDIT (`SessionOp::SetSlotUnit` — how a
/// literal that exists is written), while this one only decides how
/// the field beside it reads. Nothing here reaches a document until
/// the form's own commit button.
///
/// **It is drawn after the fields it governs**, so a unit picked now
/// re-writes them on the NEXT frame: the fields were built before
/// this widget ran, and the pick is an input event, so that next
/// frame is the one egui draws in response to it. Drawing it first
/// would close the gap and put the unit above the number it is the
/// unit of, which is the worse trade for a lag nobody can see.
pub(crate) fn length_picker(ui: &mut egui::Ui, salt: &str, chosen: &mut LengthUnit) {
    if let Some(row) = pick_unit(ui, salt, Dimension::Length, chosen.def())
        && let Some(unit) = row.as_length()
    {
        *chosen = unit;
    }
}

/// [`length_picker`]'s angle twin. Two functions rather than one over
/// a dimension, because a length picker that could write a `deg` into
/// its draft is the mismatch the typed views exist to make
/// unrepresentable — the pairing is checked by the compiler here, not
/// by a branch.
pub(crate) fn angle_picker(ui: &mut egui::Ui, salt: &str, chosen: &mut AngleUnit) {
    if let Some(row) = pick_unit(ui, salt, Dimension::Angle, chosen.def())
        && let Some(unit) = row.as_angle()
    {
        *chosen = unit;
    }
}

/// The combo itself: the rows `dimension` admits, with `shown`
/// selected; `Some(row)` when this frame's click chose one.
pub(crate) fn pick_unit(
    ui: &mut egui::Ui,
    salt: &str,
    dimension: Dimension,
    shown: UnitDef,
) -> Option<UnitDef> {
    let options = props::unit_options(dimension);
    if options.is_empty() {
        return None;
    }
    let mut picked = None;
    egui::ComboBox::from_id_salt(("creation_unit", salt))
        .selected_text(shown.symbol())
        // Wide enough for the longest symbol the table carries
        // (`pi rad`) plus the combo's arrow — the panel's width, for
        // the panel's reason.
        .width(72.0)
        .show_ui(ui, |ui| {
            for option in options {
                if ui
                    .selectable_label(shown == option, option.symbol())
                    .clicked()
                {
                    picked = Some(option);
                }
            }
        });
    picked
}

/// The delete button: a renderer for [`DocSession::delete_affordance`]
/// and nothing else, so the two places a delete is reachable from (a
/// node selection and a face selection) cannot state different costs
/// for the same operation, and the sentence itself is testable without
/// a window.
pub(crate) fn delete_button(ui: &mut egui::Ui, session: &DocSession, node: RecipeNodeId) -> bool {
    let affordance = session.delete_affordance(node);
    let button = ui.button(affordance.label);
    match affordance.hover {
        Some(text) => button.on_hover_text(text).clicked(),
        None => button.clicked(),
    }
}
