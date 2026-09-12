//! **The free helpers over `egui::Ui` that the panes share.**
//!
//! This module is part of the `app` driver rather than a vocabulary —
//! it names `egui`, and [`delete_button`] reads a
//! [`crate::session::DocSession`] because the wording it draws is the
//! session's own answer.
//!
//! Every other function here draws one row or one field from values
//! the caller already holds, and returns what the user did with it.
//! None of the others reads the application or the session: the pane
//! modules own that, and hand these numbers, units and labels.
//!
//! [`drag_ops`] is the exception worth naming — it is the one mapping
//! from a `DragValue` to session operations, and the whole reason a
//! dragged number in this crate emits one committed edit rather than
//! one per frame.
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

use eframe::egui;
use pncad::document::{Dimension, RecipeNodeId};
use pncad::profile::{ArcSide, ArcSweep};
use pncad::quantity::{AngleUnit, LengthUnit, UnitDef};

use crate::forms::{ANGLE_DRAG_SPEED, ArcMode, FIELD_DRAG_SPEED, PathVerb, UNIT_DRAG_SPEED};
use crate::props;
use crate::readout;
use crate::session::{DocSession, SessionOp};
use crate::sketch::{ArcSpec, PathStep, PathTarget};

/// **The text a numeric field shows**, and the one rule every field in
/// this chrome obeys: *the text reads back as the value the field
/// holds*.
///
/// An `egui::DragValue` with no `max_decimals` derives its precision
/// from its DRAG SPEED and the display scaling, and from nothing about
/// the value — `auto_decimals` is `ceil(log10(aim_radius / speed))` and
/// the range handed to a formatter is `auto_decimals ..= auto_decimals
/// + 2`. A length field at [`FIELD_DRAG_SPEED`] shown in millimetres is
/// handed `1..=3` at one point per pixel, so its coarsest spelling is
/// `{:.3}` over millimetres.
///
/// Inside that range the widget already picks the shortest spelling
/// that reads back, which is this rule. What it does when NONE of them
/// does is return the widest one anyway —
/// `emath::format_with_decimals_in_range`, under a comment saying
/// *"show the full value"*. That is where a field holding 1.6 µm reads
/// `0.002` and one holding 40 nm reads `0.000`.
///
/// **And a field's text is a commit path, not only a render.** The
/// widget seeds its keyboard edit with the text it last showed and
/// writes the parse back on losing focus, so clicking into a field and
/// clicking away again commits what the field said —
/// `crate::pane::properties`'s `slot_value_ui` says exactly that where
/// it refuses to charge that click for an undo step. A text that
/// misreads the value therefore DESTROYS it, and a field holding 40 nm
/// becomes a field holding zero.
///
/// So the widget's own spelling is kept wherever it reads back and
/// [`crate::readout::number`] carries the rest. **Keeping it is not
/// deference**: it is what makes this change invisible to a DRAG.
/// A drag commits `round_to_decimals(value, auto_decimals)`
/// (`egui::DragValue::ui`), so every value a drag produces is spelled
/// exactly by the bottom of the widget's own range — the text a drag
/// steps through is the text it steps through today, and the question
/// of what a gesture means when its number stops matching its tick is
/// one this rule never asks.
pub(crate) fn field_text(value: f64, decimals: core::ops::RangeInclusive<usize>) -> String {
    let spelling = egui::emath::format_with_decimals_in_range(value, decimals);
    if readout::reads_back(&spelling, value) {
        spelling
    } else {
        readout::number(value)
    }
}

/// **Every numeric field in the chrome**, dragged at `speed` per pixel.
///
/// One constructor rather than an `egui::DragValue::new` at each site,
/// because [`field_text`] is one decision about all of them rather than
/// a patch to the length ones. The property is that a field's text
/// names the value it holds, and that property has no dimension in it:
/// a dimensionless field reading `0.00` over 1.6e-5 and an angle field
/// reading `0.000` over a microradian make the same false claim a
/// length field does, and each is one click away from committing it.
///
/// **An INTEGER field passes through unchanged, and provably rather
/// than by exclusion**: `egui::DragValue::new` gives one
/// `max_decimals(0)`, so the range is `0..=0`, the only spelling is
/// `{:.0}`, and a whole number reads back as itself. Nothing here has
/// to know which fields those are.
pub(crate) fn number_field<Num: egui::emath::Numeric>(
    value: &mut Num,
    speed: f64,
) -> egui::DragValue<'_> {
    egui::DragValue::new(value)
        .speed(speed)
        .custom_formatter(field_text)
}

/// **One gesture vocabulary**: the four operations a drag on one field
/// emits, in the words that field's own doors speak.
///
/// A struct rather than four parameters because [`Self::commit`] and
/// [`Self::cancel`] are the same type and mean opposite things —
/// positionally they sit one transposition away from a chrome that
/// lands what the user abandoned and abandons what they landed, with
/// nothing between the mistake and the user to catch it.
///
/// **The value [`Self::preview`] carries is the GESTURE's, not a
/// widget's.** A slot and a parameter each drag one number, so for
/// those two the distinction is invisible; the free-move probe drags a
/// rigid frame written as three millimetre boxes, and every preview it
/// emits carries the WHOLE frame ([`vec3_row_ops`]). The type is
/// therefore the caller's, and the bound that says so lives on
/// [`drag_ops`] and [`drag_gesture_ops`] where the value is applied.
pub(crate) struct GestureVocabulary<Preview> {
    /// Open the gesture: emitted on the press.
    pub(crate) begin: SessionOp,
    /// Move it: emitted on every frame the value changes under the
    /// pointer, carrying that value. Nothing it emits is committed.
    pub(crate) preview: Preview,
    /// Land it: emitted when a pointer release ends the drag.
    pub(crate) commit: SessionOp,
    /// Abandon it: emitted when Escape ends the drag instead
    /// ([`drag_gesture_ops`]).
    pub(crate) cancel: SessionOp,
}

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
/// Generalized over the [`GestureVocabulary`] because the free-move
/// probe runs the same gesture over
/// display ops rather than document ops — one mapping, two
/// vocabularies, and the typed-input arm (`changed() && !dragged()`)
/// covered for BOTH, which is the arm a hand-mapped copy of this
/// function silently dropped once already.
///
/// The DRAG half is [`drag_gesture_ops`], which the slot field calls
/// directly: a field that reads its own text decides for itself what
/// was typed, so `changed()` is not what tells it.
///
/// **`widget` is the gesture's hand, which is not always one widget.**
/// Everything below is read off one `Response`, and a gesture written
/// as several boxes hands over the union of theirs ([`vec3_row_ops`]):
/// `dragged()` then means *the pointer is holding this gesture*
/// rather than *this box*, which is the question the typed arm is
/// asking.
pub(crate) fn drag_ops<Value: Copy>(
    widget: &egui::Response,
    value: Value,
    gesture: GestureVocabulary<impl Fn(Value) -> SessionOp>,
    typed: impl Fn(Value) -> Vec<SessionOp>,
    ops: &mut Vec<SessionOp>,
) {
    if drag_gesture_ops(widget, value, gesture, ops) {
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
/// every frame the value moves, and on the frame the drag ends either
/// the commit or the cancel. Answers whether the drag ended, i.e.
/// whether this frame's change was a gesture's and belongs to nothing
/// else.
///
/// **A drag has two ends and they mean opposite things.** A pointer
/// release lands the previewed value; Escape abandons it, and the
/// abandoning end is the only one a user has while the button is still
/// down — the cancel doors ([`DocSession::cancel_doors`]) are toolbar
/// controls, so reaching one costs the release that would land the
/// value. `egui` collapses the two into one `drag_stopped`, which is
/// why the cancel is a member of [`GestureVocabulary`] rather than a
/// control beside the field.
///
/// **The key is read, not bound.** Escape is already `egui`'s abort:
/// it clears the drag whatever this crate does, so what this branch
/// decides is which of the two things the toolkit did the chrome
/// reports — not which key means cancel. Read directly rather than
/// inferred from the absence of a pointer release, because a long
/// touch also ends a drag with no release and means something else
/// entirely.
///
/// **Every gesture vocabulary has a cancel**, so
/// [`GestureVocabulary::cancel`] is a `SessionOp` rather than an
/// `Option`: `gesture_table.rs`'s
/// `every_gesture_cancel_has_a_chrome_door` matches exhaustively over
/// [`SessionOp`], so a gesture that joined the enum with no cancel
/// would red there first.
pub(crate) fn drag_gesture_ops<Value>(
    widget: &egui::Response,
    value: Value,
    gesture: GestureVocabulary<impl Fn(Value) -> SessionOp>,
    ops: &mut Vec<SessionOp>,
) -> bool {
    let GestureVocabulary {
        begin,
        preview,
        commit,
        cancel,
    } = gesture;
    if widget.drag_started() {
        ops.push(begin);
    }
    if widget.dragged() && widget.changed() {
        ops.push(preview(value));
    }
    if widget.drag_stopped() {
        let escaped = widget
            .ctx
            .input(|input| input.key_pressed(egui::Key::Escape));
        ops.push(if escaped { cancel } else { commit });
        return true;
    }
    false
}

/// **Three boxes, ONE gesture**: a row of draggable components over a
/// single value, mapped through [`drag_ops`] exactly once.
///
/// The free-move probe is this shape — three millimetre fields over
/// one instance's frame, all three driving the one probe that instance
/// has. Spelling the triple once per box is what makes the second box
/// a SECOND gesture, and the boxes are reachable one at a time only by
/// the pointer: a `DragValue` enters keyboard-edit mode the frame it
/// takes focus, so Tab and an arrow key change a component while the
/// pointer holds another. Mapped per box, that frame emits a whole
/// begin/preview/commit through the typed arm — a begin the open probe
/// refuses and a commit that lands and CLOSES the probe the pointer is
/// still holding, which is a refusal describing a state its own batch
/// destroyed. Mapped once, the same frame is what it is: another hand
/// on the open gesture, one preview, nothing ended.
///
/// So the union is the point. `egui::Response`'s `|` is its own
/// summary of a row (`Response::union`), and under it `drag_started`,
/// `dragged`, `drag_stopped` and `changed` answer for the ROW: the
/// pointer opens the gesture from whichever box it pressed, every
/// component that moves previews into it, and the drag ends once.
///
/// **The previewed value is composed AFTER all three are drawn**, so a
/// component changed this frame is in the value this frame previews —
/// the caller's `gesture.preview` and `typed` take the whole `[f64; 3]`
/// rather than one box's number.
///
/// It does not draw the label: the probe names its row above it, in a
/// sentence that carries the unit ([`vec3_row`] is the creation forms'
/// labelled version, which emits no operations at all).
pub(crate) fn vec3_row_ops(
    ui: &mut egui::Ui,
    speed: f64,
    components: &mut [f64; 3],
    gesture: GestureVocabulary<impl Fn([f64; 3]) -> SessionOp>,
    typed: impl Fn([f64; 3]) -> Vec<SessionOp>,
    ops: &mut Vec<SessionOp>,
) {
    let [x, y, z] = components;
    let row = ui.add(number_field(x, speed))
        | ui.add(number_field(y, speed))
        | ui.add(number_field(z, speed));
    drag_ops(&row, *components, gesture, typed, ops);
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
            ui.add(number_field(component, speed));
        }
    });
}

/// **One dimensioned field of a creation form.**
///
/// The draft behind it is CANONICAL (metres, radians) and the field
/// is what that value looks like written in `unit` — the property
/// panel's own rule ([`crate::props::in_written`] / [`crate::props::from_written`], the
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
    let mut field = number_field(&mut written, props::in_written(speed, unit));
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
    ui.add(number_field(value, speed).prefix(format!("{name} ")));
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
            for (option, label) in ArcMode::ALL {
                ui.selectable_value(&mut mode, option, label);
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
/// ([`crate::props::unit_options`], read off the closed unit table) and the
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

#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]

    use super::{GestureVocabulary, drag_gesture_ops, number_field, vec3_row_ops};
    use crate::session::SessionOp;
    use eframe::egui;
    use pncad::document::{Axis3, Frame, RecipeNodeId, SlotId};

    const NODE: RecipeNodeId = RecipeNodeId(7);

    /// How many Tab/ArrowUp pairs the row spends looking for the
    /// focus. A budget rather than a count: which step the focus
    /// reaches a second component on is egui's Tab order, not this
    /// crate's, and pinning it would make the row a reading of egui's
    /// traversal instead of of this chrome's ops.
    const TAB_BUDGET: usize = 8;

    /// Which gesture vocabulary the field under test is wired with.
    ///
    /// The two the panel drags are mapped by the same function, so a
    /// rule about the mapping is a rule about both, and a row that
    /// drove only one would be reading half of what it claims.
    #[derive(Clone, Copy)]
    enum Vocabulary {
        /// The free-move probe's display triple, through
        /// [`vec3_row_ops`] — `crate::pane::properties`'s
        /// `instance_ui`, which wires its three components as ONE
        /// gesture.
        FreeMove,
        /// The value gesture's document triple over a vector slot's
        /// three components, through [`drag_gesture_ops`] directly —
        /// `crate::pane::properties`'s `slot_value_ui`, which reads its
        /// own text and so does not want the typed arm.
        Slot,
    }

    struct Probe {
        ctx: egui::Context,
        vocabulary: Vocabulary,
        mm: [f64; 3],
        /// The whole row's rect, which is what the two vocabularies
        /// share: the free-move arm draws its three boxes inside
        /// [`vec3_row_ops`] and hands back no per-box response, so a
        /// pointer aims at the row and [`Probe::aim`] picks the box.
        row: egui::Rect,
    }

    impl Probe {
        fn new() -> Self {
            Self::of(Vocabulary::FreeMove)
        }

        fn of(vocabulary: Vocabulary) -> Self {
            Self {
                ctx: egui::Context::default(),
                vocabulary,
                mm: [0.0; 3],
                row: egui::Rect::NOTHING,
            }
        }

        /// Where to press for the FIRST of the row's three boxes: the
        /// middle of its first third, the boxes being three of a size.
        /// Which box the pointer takes does not matter to the rules
        /// below — that the keyboard can reach a different one does.
        fn aim(&self) -> egui::Pos2 {
            self.row.left_center() + egui::vec2(self.row.width() / 6.0, 0.0)
        }

        fn frame(&mut self, events: Vec<egui::Event>) -> Vec<SessionOp> {
            let mut ops = Vec::new();
            let ctx = self.ctx.clone();
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 600.0),
                )),
                events,
                ..Default::default()
            };
            let mm = &mut self.mm;
            let row = &mut self.row;
            let ops_ref = &mut ops;
            let vocabulary = self.vocabulary;
            let mut output = ctx.run_ui(input, |ui| {
                let laid_out = ui.horizontal(|ui| match vocabulary {
                    Vocabulary::FreeMove => {
                        let frame_of = |mm: [f64; 3]| Frame::translation(mm.map(|v| v * 1.0e-3));
                        vec3_row_ops(
                            ui,
                            0.5,
                            mm,
                            GestureVocabulary {
                                begin: SessionOp::BeginFreeMove { instance: NODE },
                                preview: |mm| SessionOp::PreviewFreeMove {
                                    instance: NODE,
                                    frame: frame_of(mm),
                                },
                                commit: SessionOp::CommitFreeMove { instance: NODE },
                                cancel: SessionOp::CancelFreeMove,
                            },
                            |mm| {
                                vec![
                                    SessionOp::BeginFreeMove { instance: NODE },
                                    SessionOp::PreviewFreeMove {
                                        instance: NODE,
                                        frame: frame_of(mm),
                                    },
                                    SessionOp::CommitFreeMove { instance: NODE },
                                ]
                            },
                            ops_ref,
                        );
                    }
                    Vocabulary::Slot => {
                        // Three components, three SLOTS, three
                        // gestures: the value drag's identity is the
                        // field, so each box spells its own triple and
                        // the sibling case is refused at the door
                        // (`gesture_table.rs`'s
                        // `a_drag_on_another_field_cannot_steer_the_open_one`).
                        for (axis, component) in mm.iter_mut().enumerate() {
                            let widget = ui.add(number_field(component, 0.5));
                            let slot = SlotId::Origin(Axis3::ALL[axis]);
                            drag_gesture_ops(
                                &widget,
                                *component,
                                GestureVocabulary {
                                    begin: SessionOp::BeginGesture { node: NODE, slot },
                                    preview: |value| SessionOp::PreviewGesture {
                                        node: NODE,
                                        slot,
                                        value,
                                    },
                                    commit: SessionOp::CommitGesture { node: NODE, slot },
                                    cancel: SessionOp::CancelGesture,
                                },
                                ops_ref,
                            );
                        }
                    }
                });
                *row = laid_out.response.rect;
            });
            output.textures_delta.clear();
            ops
        }

        fn key(&mut self, key: egui::Key) -> Vec<SessionOp> {
            self.frame(vec![egui::Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }])
        }
    }

    /// What each operation DOES to the gesture, with the vocabulary
    /// that spells it dropped: the rules below are about the shape of
    /// the triple, and reading them off the variant names would make
    /// each row a row about one vocabulary.
    fn kind(op: &SessionOp) -> &'static str {
        match op {
            SessionOp::BeginFreeMove { .. } | SessionOp::BeginGesture { .. } => "begin",
            SessionOp::PreviewFreeMove { .. } | SessionOp::PreviewGesture { .. } => "preview",
            SessionOp::CommitFreeMove { .. } | SessionOp::CommitGesture { .. } => "commit",
            SessionOp::CancelFreeMove | SessionOp::CancelGesture => "cancel",
            _ => "other",
        }
    }

    /// Lay the field out, then open a drag on its x component: the
    /// press and the move that makes `egui` call it a drag rather than
    /// a click.
    fn open_a_drag(probe: &mut Probe) {
        // Two frames: egui interacts against the PREVIOUS frame's
        // widget rects, so nothing is hittable until one has been laid
        // out.
        probe.frame(Vec::new());
        probe.frame(Vec::new());
        let x = probe.aim();
        probe.frame(vec![egui::Event::PointerMoved(x)]);
        probe.frame(vec![egui::Event::PointerButton {
            pos: x,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        }]);
        let opened = probe.frame(vec![egui::Event::PointerMoved(x + egui::vec2(40.0, 0.0))]);
        assert_eq!(
            opened.iter().map(kind).collect::<Vec<_>>(),
            ["begin", "preview"],
            "the pointer drag opens a gesture and holds it open"
        );
    }

    /// **A keyboard bump on a sibling component STEERS the held drag**
    /// rather than opening a second probe under it.
    ///
    /// The probe field is three `DragValue`s over one instance, and a
    /// pointer holds at most one of them: egui carries `dragged`,
    /// `drag_started` and `drag_stopped` as a single `Option<Id>` each,
    /// so no second pointer and no touch can open a second drag. The
    /// keyboard is the other hand. A `DragValue` enters edit mode the
    /// moment it takes focus and answers the arrow keys there, so Tab
    /// and ArrowUp change a component nobody is pointing at.
    ///
    /// The instance has one probe and all three components drive it,
    /// so that keystroke belongs to the gesture already open —
    /// [`vec3_row_ops`] maps the row once and the frame it previews is
    /// composed from all three, so the bump arrives as a preview
    /// carrying the pointer's component and the keyboard's together.
    ///
    /// Every frame after the press below carries keyboard events only,
    /// so the drag opened by the pointer is in flight at each of them:
    /// the assertion that nothing but previews is emitted is what says
    /// so in the ops themselves.
    ///
    /// Where it goes red: spell the triple per box again and the
    /// sibling bump takes the typed arm instead — a begin the open
    /// probe refuses `FreeMoveInFlight`, a preview that overwrites the
    /// pointer's frame, and a commit that lands it and closes the
    /// probe the pointer is still holding, leaving the user a refusal
    /// describing a state its own batch destroyed.
    #[test]
    fn a_keyboard_bump_steers_the_held_drag_rather_than_beginning_a_second_probe() {
        let mut probe = Probe::new();
        open_a_drag(&mut probe);
        let held = probe
            .mm
            .iter()
            .position(|component| *component != 0.0)
            .expect("the pointer drag moved the component it is holding");

        let mut emitted: Vec<&'static str> = Vec::new();
        let mut steered: Option<Frame> = None;
        // Tab walks the focus over the three components and the
        // surrounding chrome; the bound is a budget, not a measurement
        // of where the focus lands on any particular step.
        for _ in 0..TAB_BUDGET {
            emitted.extend(probe.key(egui::Key::Tab).iter().map(kind));
            let bumped = probe.key(egui::Key::ArrowUp);
            emitted.extend(bumped.iter().map(kind));
            if let Some(frame) = bumped.iter().find_map(|op| match op {
                SessionOp::PreviewFreeMove { frame, .. } => Some(*frame),
                _ => None,
            }) && frame
                .translation
                .iter()
                .enumerate()
                .any(|(axis, moved)| axis != held && *moved != 0.0)
            {
                steered = Some(frame);
                break;
            }
        }
        let steered =
            steered.expect("a keyboard bump reaches a component the pointer is not holding");
        assert!(
            !emitted.contains(&"begin")
                && !emitted.contains(&"commit")
                && !emitted.contains(&"cancel"),
            "the keyboard drives the open gesture and neither opens nor ends one; the \
             frames under the held drag carried {emitted:?}"
        );
        assert!(
            steered.translation[held] != 0.0,
            "and what it previews is the WHOLE frame — the pointer's own component is \
             still in it, at {:?}",
            steered.translation
        );
    }

    /// **And with no drag open the same keystroke still spells the
    /// whole triple**, which is the half the rule above must not buy.
    ///
    /// A typed value is one committed display value and needs its own
    /// begin and its own commit: deleting the typed arm, or guarding
    /// it on anything wider than *is this gesture already open*, would
    /// satisfy the row above by making the field unusable from the
    /// keyboard alone.
    #[test]
    fn a_keyboard_bump_with_no_drag_open_still_spells_the_whole_triple() {
        let mut probe = Probe::new();
        probe.frame(Vec::new());
        probe.frame(Vec::new());
        let mut bumped: Vec<SessionOp> = Vec::new();
        for _ in 0..TAB_BUDGET {
            probe.key(egui::Key::Tab);
            bumped = probe.key(egui::Key::ArrowUp);
            if !bumped.is_empty() {
                break;
            }
        }
        assert_eq!(
            bumped.iter().map(kind).collect::<Vec<_>>(),
            ["begin", "preview", "commit"],
            "no pointer anywhere, so the keystroke is a whole gesture of its own"
        );
    }

    /// **Escape abandons a free-move probe rather than landing it.**
    ///
    /// `egui` aborts a drag on Escape and on nothing else, by clearing
    /// the dragged widget — so the abort reaches
    /// [`drag_gesture_ops`] as a `drag_stopped` frame, indistinguishable
    /// from a release unless the key is read. The two ends of a drag
    /// mean opposite things, and this is the one that means abandon.
    ///
    /// It is also the ONLY abandon a user has while the button is
    /// still down: [`crate::session::DocSession::cancel_doors`] draws
    /// *"Cancel free-move"* in the toolbar, and reaching a toolbar
    /// control costs the pointer release that lands the frame.
    ///
    /// Where it goes red: drop the Escape branch and the probed frame
    /// is committed into `DisplayState::moved` by the key every other
    /// control in this chrome spells *abandon*.
    #[test]
    fn escape_abandons_a_free_move_drag_instead_of_landing_it() {
        let mut probe = Probe::of(Vocabulary::FreeMove);
        open_a_drag(&mut probe);
        assert_eq!(
            probe
                .key(egui::Key::Escape)
                .iter()
                .map(kind)
                .collect::<Vec<_>>(),
            ["cancel"],
            "the abort ends the probe, and ends it the way the user asked"
        );
    }

    /// The same reading on the OTHER drag the panel maps, because it
    /// is the same function: a value gesture's release arm is
    /// [`drag_gesture_ops`]' release arm.
    ///
    /// The stake is larger here rather than smaller. A free-move
    /// commit lands a display frame no history holds; a value-gesture
    /// commit reaches the DOCUMENT — one applied edit and one undo
    /// step for a gesture the user asked to throw away.
    #[test]
    fn escape_abandons_a_value_drag_instead_of_committing_it() {
        let mut probe = Probe::of(Vocabulary::Slot);
        open_a_drag(&mut probe);
        assert_eq!(
            probe
                .key(egui::Key::Escape)
                .iter()
                .map(kind)
                .collect::<Vec<_>>(),
            ["cancel"],
            "the abort ends the drag without an edit behind it"
        );
    }

    /// **And the ordinary end is still a commit**, at both doors: the
    /// rule above is about which end happened, not about ending a drag
    /// quietly. A release that committed nothing would lose exactly
    /// the work the rule above exists to protect.
    #[test]
    fn releasing_the_pointer_still_commits_both_gestures() {
        for vocabulary in [Vocabulary::FreeMove, Vocabulary::Slot] {
            let mut probe = Probe::of(vocabulary);
            open_a_drag(&mut probe);
            let released = probe.frame(vec![egui::Event::PointerButton {
                pos: probe.aim() + egui::vec2(40.0, 0.0),
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            }]);
            assert_eq!(
                released.iter().map(kind).collect::<Vec<_>>(),
                ["commit"],
                "a released drag lands what it previewed"
            );
        }
    }
}

/// **What a numeric field says, and what saying it commits.**
///
/// [`super::field_text`] is a render, so the rows over it are a table;
/// the row that matters is not, because the defect is that the render
/// is ALSO the text a click-in and a click-away hands back to the
/// document, and only driving the real widget says whether it is.
#[cfg(test)]
mod field_tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]

    use super::{field_text, number_field};
    use eframe::egui;

    /// The decimal range a length field shown in millimetres is handed:
    /// `FIELD_DRAG_SPEED` written in millimetres is 0.5, one point per
    /// pixel makes `auto_decimals` `ceil(log10(1.0 / 0.5))`, and egui
    /// adds two. Derived here rather than asserted off the widget
    /// because these rows are about the RULE over a range, and the
    /// widget's own arithmetic is pinned by
    /// [`a_field_shows_what_the_widget_shows_wherever_that_reads_back`].
    const MM: core::ops::RangeInclusive<usize> = 1..=3;

    /// **A value the widget's own spelling names is spelled its way.**
    ///
    /// This half is what keeps the change invisible to a drag, so it is
    /// asserted as sameness rather than as a table of strings: for
    /// every value a drag can commit, the two renders agree. The
    /// population is the drag's, not a grid of pretty numbers — egui
    /// rounds what a drag commits to `auto_decimals`, so a value a drag
    /// produces is `{:.1}`-exact in millimetres by construction.
    #[test]
    fn a_field_shows_what_the_widget_shows_wherever_that_reads_back() {
        let mut tenths = -200_000_i64;
        while tenths <= 200_000 {
            #[expect(
                clippy::cast_precision_loss,
                reason = "the grid is the drag's own landing set, \
                          and every member is exact in f64"
            )]
            let value = tenths as f64 / 10.0;
            assert_eq!(
                field_text(value, MM),
                egui::emath::format_with_decimals_in_range(value, MM),
                "a drag lands on {value}, where this rule must say nothing new"
            );
            tenths += 1;
        }
    }

    /// **A value no spelling in the range names is spelled truthfully
    /// instead**, which is the defect: the widget returns its widest
    /// spelling anyway, and the widest spelling of 40 nm in millimetres
    /// is `0.000`.
    #[test]
    fn a_value_the_range_cannot_name_gets_a_text_that_names_it() {
        for (value, text) in [
            (1.6e-3_f64, "0.0016"),
            (4.0e-5, "0.00004"),
            (-4.0e-5, "-0.00004"),
            (0.0625, "0.0625"),
            (1.0e-9, "1.000e-9"),
        ] {
            assert_eq!(field_text(value, MM), text, "the field's text for {value}");
            assert_ne!(
                text,
                egui::emath::format_with_decimals_in_range(value, MM),
                "{value} is only a row here because the widget misreads it"
            );
        }
    }

    /// **Zero is a number a field really holds**, and the rule that
    /// refuses a rendered zero must not refuse a real one.
    #[test]
    fn a_field_holding_zero_says_zero() {
        assert_eq!(field_text(0.0, MM), "0.0");
    }

    /// **An integer field is untouched, and by construction.**
    /// `DragValue::new` gives an integral value one `max_decimals(0)`,
    /// so the range is `0..=0` and the only spelling is the exact one.
    #[test]
    fn an_integer_field_is_spelled_the_way_it_always_was() {
        for value in [3.0_f64, -12.0, 0.0, 1.0e9] {
            assert_eq!(field_text(value, 0..=0), format!("{value:.0}"));
        }
    }

    /// One field, laid out and driven by events, so the rule below is
    /// read off the widget rather than off the function under it.
    struct Field {
        ctx: egui::Context,
        value: f64,
        rect: egui::Rect,
    }

    impl Field {
        fn new(value: f64) -> Self {
            Self {
                ctx: egui::Context::default(),
                value,
                rect: egui::Rect::NOTHING,
            }
        }

        fn frame(&mut self, events: Vec<egui::Event>) {
            let ctx = self.ctx.clone();
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 600.0),
                )),
                events,
                ..Default::default()
            };
            let value = &mut self.value;
            let rect = &mut self.rect;
            let mut output = ctx.run_ui(input, |ui| {
                *rect = ui.add(number_field(value, 0.5)).rect;
            });
            output.textures_delta.clear();
        }

        fn click(&mut self, at: egui::Pos2) {
            self.frame(vec![
                egui::Event::PointerMoved(at),
                egui::Event::PointerButton {
                    pos: at,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
                egui::Event::PointerButton {
                    pos: at,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::NONE,
                },
            ]);
        }
    }

    /// **The rule this whole door exists for.** A `DragValue` seeds its
    /// keyboard edit with the text it last showed and writes the parse
    /// back when it loses focus, so a field's render is what clicking
    /// into it and clicking away again COMMITS. Held over the values
    /// the widget's own spelling cannot name, because those are the
    /// ones it used to commit as something else — 40 nm as zero.
    #[test]
    fn clicking_into_a_field_and_away_again_leaves_the_value_alone() {
        for start in [4.0e-5_f64, 1.6e-3, 12.0, -4.0e-5, 0.0, 1024.5] {
            let mut field = Field::new(start);
            // Two frames: egui interacts against the PREVIOUS frame's
            // widget rects, so nothing is hittable until one has been
            // laid out.
            field.frame(Vec::new());
            field.frame(Vec::new());
            let target = field.rect.center();
            field.click(target);
            field.frame(Vec::new());
            field.click(egui::pos2(700.0, 500.0));
            field.frame(Vec::new());
            field.frame(Vec::new());
            assert_eq!(
                field.value, start,
                "clicking into a field holding {start} and away again committed \
                 {} — the text it showed was not the value it held",
                field.value
            );
        }
    }
}
