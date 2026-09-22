//! **The free helpers over `egui::Ui` that the panes share.**
//!
//! This module is part of the `app` driver rather than a vocabulary —
//! it names `egui`, and [`delete_button`] reads a
//! [`crate::session::DocSession`] because the wording it draws is the
//! session's own answer.
//!
//! None of the others reads the application or the session: the pane
//! modules own that, and hand these numbers, units and labels.
//!
//! **Most of what is here draws one row or one field** from values the
//! caller already holds, and returns what the user did with it. The
//! rule that produces the exceptions is *a function that takes no
//! `ui: &mut egui::Ui`*, and there are eight: [`number_text`] and
//! [`number_field`], which render and build rather than draw;
//! [`install_number_formatter`], which writes a style; [`new_row_step`],
//! which mints a value; [`value_gesture`] and [`free_move_gesture`],
//! which mint a gesture's operations from its name; and [`drag_ops`]
//! with [`drag_gesture_ops`], which read a `Response`.
//!
//! [`drag_ops`] is the one worth naming — it is the one mapping from a
//! `DragValue` to session operations, and the whole reason a dragged
//! number in this crate emits one committed edit rather than one per
//! frame.
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

use eframe::egui;
use pncad::document::{Dimension, Frame, RecipeNodeId};
use pncad::geom_core::Point2;
use pncad::profile::{
    ArcData, ArcMode, ArcSide, ArcSweep, SpecForms, Step, Target, TargetKind, TipState, Verb,
    arc_specs_at,
};
use pncad::quantity::{AngleUnit, LengthUnit, UnitDef};

use crate::forms::{
    ANGLE_DRAG_SPEED, COUNT_DRAG_SPEED, FIELD_DRAG_SPEED, MAX_CIRCLE_SPLIT, MIN_CIRCLE_SPLIT,
    ShapeEdits, UNIT_DRAG_SPEED, arc_mode_label, target_kind_label,
};
use crate::props;
use crate::readout;
use crate::session::{DocSession, FreeMoveName, GestureName, SessionOp, ValueGestureName};
use crate::sketch;

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
///
/// # And it is kept only while it FITS
///
/// Width is no part of reading back ([`crate::readout::reads_back`]
/// says so in as many words), so the truth test alone takes the
/// widget's spelling at any length. `emath::format_with_decimals_in_
/// range` compares in `f32`, and `almost_equal(inf, inf)` is `a == b`,
/// so at the top of `f64` its FIRST candidate is accepted: `{:.1}` of
/// `f64::MAX`, the exact decimal expansion, **311 characters**. It
/// reads back, so it was returned, and a `DragValue` renders its text
/// through a `TextWrapMode::Extend` button — the field does not clip,
/// it pushes the panel out.
///
/// The bound is [`crate::readout::MAX_CHARS`] because that is the
/// bound this crate already has and this door is the one place it was
/// not applied: `readout`'s own header calls this function *the
/// fields' door* onto the same rule, and `MAX_CHARS`' doc already says
/// what a box narrower than it costs. So a field is now spelled within
/// it wherever [`crate::readout::number`] is, which is everywhere but
/// the band at the top of the type, where four figures round out of
/// `f64` and the exact scientific spelling is twenty-two characters.
///
/// **It is still invisible to a drag**, and that is a measurement
/// rather than a hope. The widest spelling a millimetre field's range
/// offers is `{:.1}`, which passes ten characters only at `1e8`
/// (`1e7` once a sign is spent) — and a drag moves `FIELD_DRAG_SPEED`
/// per pixel, so reaching either would take twenty million pixels of
/// dragging. The substituted band is out of a drag's reach at the top
/// exactly as it is at the bottom. Millimetres are the worst case
/// rather than the only one measured: `UNITS` offers no length
/// smaller, and the scalar, count and angle ticks are all coarser.
///
/// # An INTEGER field is exempt, because it has no search to end
///
/// [`crate::readout::MAX_CHARS`] is what ENDS A SEARCH — `readout`'s
/// own doc says so, and [`crate::readout::number`] is a loop over
/// precisions that has to stop somewhere. A range of `0..=0` offers
/// ONE spelling, so there is no search here to end, and applying the
/// bound anyway substitutes the render's answer for the widget's: a
/// count of `-1000000000` was rendered `-1.000e9`, and a count of
/// `12345678901` renders as whatever reads back within
/// [`crate::readout::reads_back`]'s grid — **a different count**. For a
/// continuous quantity that band is the ratified render accuracy; for
/// an integer, every value inside it is a different value, so the
/// substitution is the wrong-number-on-screen defect this door exists
/// to prevent rather than an instance of the rule.
///
/// `0..=0` is the range `egui::DragValue::new` gives an INTEGRAL
/// `Numeric`, which it also `range`s to that type's own bounds — so
/// the exemption is bounded by the integer type and not open-ended:
/// twenty characters for an `i64`, where the band this section is
/// about was three hundred and eleven. An `i64` spells inside
/// [`crate::readout::MAX_CHARS`] at every value it has, so the band
/// where the bound would substitute a DIFFERENT integer is now reached
/// only through the other door: a caller that spells
/// `max_decimals(0)` over an `f64`, which has told this door the same
/// thing and has magnitudes an `i64` does not.
pub(crate) fn number_text(value: f64, decimals: core::ops::RangeInclusive<usize>) -> String {
    let integral = *decimals.start() == 0 && *decimals.end() == 0;
    let spelling = egui::emath::format_with_decimals_in_range(value, decimals);
    let fits = integral || spelling.chars().count() <= readout::MAX_CHARS;
    if fits && readout::reads_back(&spelling, value) {
        spelling
    } else {
        readout::number(value)
    }
}

/// **Every numeric field in the chrome**, dragged at `speed` per pixel.
///
/// One constructor rather than an `egui::DragValue::new` at each site,
/// because [`number_text`] is one decision about all of them rather than
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
///
/// # Text the field itself produced is not an edit
///
/// [`number_text`] is a render AND a commit path: an `egui::DragValue`
/// seeds its keyboard edit with the text it last showed and writes the
/// parse back when focus leaves, so clicking into a field and clicking
/// away again hands the chrome's own render straight back at it. The
/// render names its value only to [`crate::readout::reads_back`]'s
/// grid, so that round trip can MOVE the value — by less than the
/// grid, which is capped a decade below ε, and by more than nothing.
///
/// **The echo is identifiable as text, exactly**, which is why the
/// rule is a comparison rather than a tolerance: the formatter below
/// is the one that produced what the keyboard edit was seeded with, so
/// the text it returned is kept and the parser compares the typed text
/// against it ([`crate::props::echoed`], the rule's one home). Equal
/// is the field talking to itself and parses to nothing; different is
/// the user's and takes the number door. A numeric test cannot do it:
/// the render is lossy by construction, so any band wide enough to
/// swallow the echo discards real edits inside it.
///
/// **Here rather than at each write-back**, for the reason the
/// formatter is here: a field's write-back is spelled at every call
/// site and this is the one constructor they share.
/// [`crate::pane::properties`]' two value fields arrive through
/// [`value_field_ops`], which replaces both closures because it also
/// has to say which DOOR a typed text took; it asks
/// [`crate::props::echoed`] the same question in the same words.
///
/// **`egui`'s builders REPLACE rather than compose, so at that one
/// call site the cell and both closures below are built and
/// discarded** — `custom_formatter` and `custom_parser` each
/// overwrite an `Option`. The rule there is carried by
/// [`value_field_ops`]' own pair and by nothing here, so deleting
/// that parser as redundant would not fall back to this one: it would
/// leave the field on `egui`'s default parser with no echo veto at
/// all. Two spellings of one rule, and this sentence is the only
/// thing standing between them and a silent merge.
///
/// **What this does not reach is the twelfth site** — see
/// [`install_number_formatter`], which can carry the render as a
/// context default and cannot carry this, because `egui::Style` has a
/// `number_formatter` and no parser.
pub(crate) fn number_field<Num: egui::emath::Numeric>(
    value: &mut Num,
    speed: f64,
) -> egui::DragValue<'_> {
    // The text this frame's field rendered, taken from the formatter
    // that rendered it rather than re-derived: `egui` chooses the
    // decimal range from the drag speed and the display scaling, so a
    // second call here could disagree with the one the field used.
    let rendered = std::rc::Rc::new(core::cell::RefCell::new(String::new()));
    let shown = std::rc::Rc::clone(&rendered);
    egui::DragValue::new(value)
        .speed(speed)
        .custom_formatter(move |value, decimals| {
            let text = number_text(value, decimals);
            shown.replace(text.clone());
            text
        })
        .custom_parser(move |text| {
            if props::echoed(text, &rendered.borrow()) {
                return None;
            }
            match props::field_edit(text) {
                props::FieldEdit::Number(number) => Some(number),
                // Not a number at all: the field keeps the value it
                // holds, which is what `egui`'s own parser does with a
                // text it cannot read.
                props::FieldEdit::Expression(_) | props::FieldEdit::Empty => None,
            }
        })
}

/// **The floor under [`number_field`]: the same rule, as the
/// context's own default.**
///
/// [`number_field`] states the rule where a reader can see it, and
/// that is why it exists — but a constructor can only bind the sites
/// that call it. A twelfth field written as a bare
/// `egui::DragValue::new`, a helper that wraps the widget instead of
/// this door, or an `egui::Slider` (which renders its value through a
/// `DragValue` of its own) each gets `egui`'s precision rule back, and
/// nothing at the call site says so.
///
/// Setting [`egui::Style::number_formatter`] answers that as a
/// DEFAULT rather than as a detection: a site that does not
/// deliberately spell its own `custom_formatter` is already right, so
/// there is no arrival to notice. A site that DOES spell one is
/// making a statement — a hex or a clock field — and is left alone.
///
/// **`all_styles_mut` rather than `style_mut`, and that is
/// load-bearing.** `egui` keeps one `Style` per theme and
/// `crate::app`'s `apply_polarity` states a theme PREFERENCE rather
/// than freezing visuals, so the user can move between them at any
/// time; a formatter written onto only the theme in force at startup
/// would be dropped by the first switch, in the direction nobody
/// looks. `field_tests::a_bare_field_survives_a_theme_switch` is the
/// assertion that would break.
///
/// **The floor carries the RENDER and cannot carry the COMMIT.**
/// [`number_field`]'s other half — *text the field itself produced is
/// not an edit* — lives in a `custom_parser`, and `egui::Style` has a
/// `number_formatter` and no counterpart for parsing. So a bare
/// `egui::DragValue` on a context this has run on shows the right text
/// and still commits that text back when a click leaves it, which
/// moves the value wherever the render is not exact. That gap is
/// `work/vgeom/a-bare-field-still-commits-its-own-render.md`, and
/// `field_tests::a_bare_field_commits_a_render_the_door_would_refuse`
/// is the row that pins it.
pub(crate) fn install_number_formatter(ctx: &egui::Context) {
    ctx.all_styles_mut(|style| {
        style.number_formatter = egui::style::NumberFormatter::new(number_text);
    });
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
/// **The four name ONE gesture, and the fields are private so that
/// they cannot name four.** The only way to build one outside this
/// module is [`value_gesture`] or [`free_move_gesture`], each of which
/// takes the gesture's name once and mints all four operations from
/// it ([`GestureName`]); a panel that spelled the target per operation
/// could preview into one field and commit another, which is a
/// convention away from a real edit landing on the wrong slot.
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
    begin: SessionOp,
    /// Move it: emitted on every frame the value changes under the
    /// pointer, carrying that value. Nothing it emits is committed.
    preview: Preview,
    /// Land it: emitted when a pointer release ends the drag.
    commit: SessionOp,
    /// Abandon it: emitted when Escape ends the drag instead
    /// ([`drag_gesture_ops`]).
    cancel: SessionOp,
}

/// **The four operations of a VALUE drag**, minted from the one slot
/// or parameter they all name.
///
/// The caller spells the target once and writes no operation at all,
/// so the begin, the preview and the commit cannot come to name
/// different fields and the cancel cannot be the other drag's.
pub(crate) fn value_gesture(
    name: ValueGestureName,
) -> GestureVocabulary<impl Fn(f64) -> SessionOp> {
    let gesture = GestureName::Value(name.clone());
    GestureVocabulary {
        begin: gesture.begin(),
        commit: gesture.commit(),
        cancel: gesture.cancel(),
        preview: move |value| name.preview(value),
    }
}

/// **Both halves of what one probe row hands [`drag_ops`]**: the four
/// operations a drag emits and the one-shot triple a TYPED value
/// spells.
///
/// A struct rather than a tuple because the two are passed as separate
/// arguments and are the same shape at the call, so a transposition
/// hands the drag arm to the typed parameter with nothing between the
/// mistake and the user — the argument [`GestureVocabulary`] makes
/// about its own four members.
///
/// **Boxed rather than generic.** Both members are closures minted
/// here, so a caller never names their types; spelling them as two
/// type parameters makes this function's return type the widest thing
/// in the module and says nothing a reader wants. One allocation per
/// probe row per frame is not a cost this chrome can measure.
pub(crate) struct ProbeOps<'a> {
    /// What the pointer drives.
    pub(crate) gesture: GestureVocabulary<BoxedPreview<'a>>,
    /// What a typed value spells: a begin, a preview and a commit in
    /// one batch.
    pub(crate) typed: Box<dyn Fn([f64; 3]) -> Vec<SessionOp> + 'a>,
}

/// One preview operation of a probe, minted from the row's three
/// millimetre boxes.
type BoxedPreview<'a> = Box<dyn Fn([f64; 3]) -> SessionOp + 'a>;

/// **A FREE-MOVE probe's whole vocabulary**, minted from the one
/// instance every operation in it names: the four a drag emits, and
/// the one-shot triple a TYPED value spells.
///
/// The typed arm is minted here rather than beside the call because it
/// names the same probe — a begin, a preview and a commit in one
/// batch — and a hand-written copy of it is the one place the panel
/// could still name a second instance. Both arms go to [`drag_ops`],
/// which is why they are returned together.
///
/// `frame_of` is the panel's own writing — the millimetres its three
/// boxes show composed into the rigid frame a preview carries — and is
/// the only part of the probe's vocabulary that is not the name's.
pub(crate) fn free_move_gesture<'a>(
    instance: RecipeNodeId,
    frame_of: impl Fn([f64; 3]) -> Frame + Copy + 'a,
) -> ProbeOps<'a> {
    let name = FreeMoveName { instance };
    let gesture = GestureName::FreeMove(name);
    ProbeOps {
        gesture: GestureVocabulary {
            begin: gesture.begin(),
            commit: gesture.commit(),
            cancel: gesture.cancel(),
            preview: Box::new(move |mm| name.preview(frame_of(mm))),
        },
        typed: Box::new(move |mm| vec![name.begin(), name.preview(frame_of(mm)), name.commit()]),
    }
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

/// **The two doors one panel value field has**, as the operations to
/// emit through each.
///
/// [`GestureVocabulary`]'s companion for the half of a field that is
/// not a drag. A slot's number door is `SessionOp::SetSlot` and its
/// text door `SessionOp::SetSlotExpression`; a parameter's are
/// `SessionOp::SetParam` and `SessionOp::SetParamText`. The SHAPE is
/// the same at both rows, which is why it is a parameter rather than
/// a branch: what differs between the two fields is only which
/// operation each door spells.
pub(crate) struct FieldVocabulary<Number, Text> {
    /// A bare number, already read out of the field's notation and
    /// into the value it authors.
    pub(crate) number: Number,
    /// Everything the field's parser could not read as a number, as
    /// the trimmed text the user left in it.
    pub(crate) text: Text,
}

/// **What one panel value field is SHOWING**, as the row it is drawn
/// for answers it.
///
/// Four facts about one field rather than four arguments, because
/// they are one answer: what the field is written in decides both the
/// number it displays and the tick it scrubs at, and the dimension
/// decides what a number read out of it becomes.
pub(crate) struct FieldShowing {
    /// The notation the field shows and authors in, and its tick.
    pub(crate) writing: crate::forms::FieldWriting,
    /// What the value IS — [`crate::props::SlotValue::of`]'s argument,
    /// so a count field holds a count.
    pub(crate) dimension: Dimension,
    /// The number it holds, in `writing`'s notation.
    pub(crate) number: f64,
    /// A text it shows INSTEAD of that number — a slot's source, for
    /// a row that has source rather than a number to show. `None` is
    /// a field showing its number, which is every parameter row and
    /// every literal slot that evaluated.
    pub(crate) text: Option<String>,
}

/// **A panel value field: one number, two doors and a gesture** — the
/// whole of what `pane::properties`' slot row and parameter row draw,
/// spelled once.
///
/// `writing` is how the field is written (`crate::forms::FieldWriting`
/// — the notation it shows and authors in, and the tick it scrubs at),
/// `showing` is the number it holds IN that notation, and `fixed` is a
/// text it shows instead of a number (a slot's source, when the row
/// has source rather than a number to show). `dimension` is what turns
/// a typed number into the value the vocabulary's number door carries.
///
/// # Text the field itself produced is not an edit
///
/// An `egui::DragValue` seeds its keyboard edit with the text it last
/// rendered and writes the parse back when focus leaves, so clicking
/// into a field and clicking away again hands the chrome's own render
/// straight back at it — a value nobody typed, committed as an edit
/// and charged an undo step. The render is not exact
/// ([`crate::readout::reads_back`]'s grid bounds it), so that round
/// trip can also MOVE the value.
///
/// **The echo is identifiable as text, exactly**, which is why this
/// is where the rule lives. The formatter below is the one that
/// produced what the keyboard edit was seeded with, so the text it
/// returned is kept and the parser compares against it
/// ([`crate::props::echoed`]): text equal to the field's own render is
/// the field talking to itself and emits nothing, and text that
/// differs is the user's and takes its door. No tolerance, no second
/// opinion about what a number means, and the same rule for a field
/// showing a number and a field showing an expression.
///
/// **Whether the edit CHANGES anything is not this question.** A user
/// who re-types a number the document already holds has still typed
/// it, and what the document does with an edit that writes what
/// stands is the document's answer, at the door that applies it.
pub(crate) fn value_field_ops(
    ui: &mut egui::Ui,
    showing: FieldShowing,
    gesture: GestureVocabulary<impl Fn(f64) -> SessionOp>,
    doors: FieldVocabulary<impl Fn(props::SlotValue) -> SessionOp, impl Fn(String) -> SessionOp>,
    ops: &mut Vec<SessionOp>,
) {
    let FieldShowing {
        writing,
        dimension,
        mut number,
        text: fixed,
    } = showing;
    // The text this frame's field rendered, taken from the formatter
    // that rendered it rather than re-derived: `egui` chooses the
    // decimal range from the drag speed and the display scaling, so a
    // second call here could disagree with the one the field used.
    let rendered = core::cell::RefCell::new(String::new());
    // The parser runs inside `ui.add`, so what it read comes back out
    // through a cell rather than a return value.
    let typed: core::cell::RefCell<Option<props::FieldEdit>> = core::cell::RefCell::new(None);
    let widget = ui.add(
        number_field(&mut number, writing.tick)
            .update_while_editing(false)
            // `number_field`'s own formatter, plus the two things this
            // field needs from it: the fixed text a row with source
            // rather than a number shows, and a copy of whatever it
            // returned.
            //
            // These two REPLACE the constructor's pair rather than
            // wrapping it — `egui`'s builders overwrite an `Option` —
            // so the echo veto below is the one that runs here and
            // the constructor's is inert. Deleting either of these
            // leaves this field on `egui`'s default parser, not on
            // `number_field`'s.
            .custom_formatter(|value, decimals| {
                let text = fixed
                    .clone()
                    .unwrap_or_else(|| number_text(value, decimals));
                rendered.replace(text.clone());
                text
            })
            .custom_parser(|text| {
                let edit = props::field_edit(text);
                let number = match edit {
                    props::FieldEdit::Number(number) => Some(number),
                    // Rejected as a number, which routes it to the
                    // text door and leaves the field where it was
                    // until the document answers.
                    _ => None,
                };
                if !props::echoed(text, &rendered.borrow()) {
                    typed.replace(Some(edit));
                }
                number
            }),
    );
    // The drag half only. A typed value is dispatched below, by the
    // match that knows which door the text takes — [`drag_ops`]'s
    // typed arm would emit the number door's operation before that
    // match ran and for every text the widget marked changed,
    // including the echo this field is built to swallow.
    drag_gesture_ops(&widget, writing.authored(number), gesture, ops);
    match typed.into_inner() {
        // **A number the dimension cannot carry is not an edit.** A
        // `Count` field takes `inf` and `NaN` from its parser like any
        // other (`props::field_edit`), and `props::SlotValue::of` is
        // where that stops being a value — so there is nothing for the
        // number door to carry and the field keeps what the document
        // says it holds. The refusal reaches a word on the DRAG path,
        // where the session's gesture door maps it to
        // `crate::session::Refusal::Dimension`; on this path there is
        // no operation to carry one, so it is silent.
        Some(props::FieldEdit::Number(written)) => {
            if let Ok(value) = props::SlotValue::of(dimension, writing.authored(written)) {
                ops.push((doors.number)(value));
            }
        }
        Some(props::FieldEdit::Expression(text)) => ops.push((doors.text)(text)),
        // An emptied field is not an edit: there is no value it could
        // mean, and blanking a field is not a way to delete anything
        // in this vocabulary.
        Some(props::FieldEdit::Empty) | None => {}
    }
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
pub(crate) fn point_fields(ui: &mut egui::Ui, unit: UnitDef, point: &mut Point2<f64>) {
    named_field(ui, "x", unit, FIELD_DRAG_SPEED, &mut point.x);
    named_field(ui, "y", unit, FIELD_DRAG_SPEED, &mut point.y);
}

/// **A path verb's target**: an authored point, the entry vertex
/// (which CLOSES the loop), or the entry vertex with the seam's
/// tangent joint declared.
///
/// One control because they are one decision — where this leg ends —
/// and `Start` is not a point somebody could type: it is the bound
/// entry, and aiming at it is what closing IS in this algebra
/// (`pncad::profile::path`, which has no `close()` alias). The forms
/// are the kernel's [`TargetKind::ALL`], so a form the vocabulary
/// gains is offered here without an edit.
///
/// Switching form REPLACES the target with a fresh one, for the
/// reason [`arc_fields`] replaces a spec: only one form has fields.
pub(crate) fn target_fields(
    ui: &mut egui::Ui,
    salt: &str,
    unit: UnitDef,
    admitted: Option<&Admitted<'_, TargetKind>>,
    shape: ShapeEdits,
    target: &mut Target<f64>,
) {
    let mut kind = target.kind();
    let before = kind;
    ui.add_enabled_ui(shape.free(), |ui| {
        egui::ComboBox::from_id_salt(("path_target", salt))
            .selected_text(target_kind_label(kind))
            .width(152.0)
            .show_ui(ui, |ui| {
                for &option in TargetKind::ALL {
                    offer(ui, admitted, &mut kind, option, target_kind_label(option));
                }
            });
    });
    if kind != before {
        *target = sketch::fresh_target(kind);
    }
    // Exhaustive, so a form that grows a payload has to be given its
    // fields here before this compiles.
    match target {
        Target::Point(point) => point_fields(ui, unit, point),
        Target::Start | Target::StartArriving => {}
    }
}

/// **What the lattice takes at a tip**, for one picker: the state (the
/// words a greyed-out choice is explained with) and the test a choice
/// has to pass.
pub(crate) struct Admitted<'a, V> {
    state: TipState,
    admits: Box<dyn Fn(V) -> bool + 'a>,
}

/// One picker choice: offered when the lattice takes it here (or when
/// nothing is known about the tip), greyed with the tip's state as its
/// hover text otherwise. The choice ALREADY made stays selectable, so
/// a spec the tip refuses still shows what it is.
fn offer<V: Copy + PartialEq>(
    ui: &mut egui::Ui,
    admitted: Option<&Admitted<'_, V>>,
    chosen: &mut V,
    option: V,
    label: &str,
) {
    let refused = admitted.filter(|a| option != *chosen && !(a.admits)(option));
    let row = ui.add_enabled(
        refused.is_none(),
        egui::Button::selectable(*chosen == option, label),
    );
    match refused {
        Some(a) => {
            row.on_disabled_hover_text(format!(
                "{label} is not well-typed here — the tip is {}",
                sketch::tip_state_words(a.state),
            ));
        }
        None if row.clicked() => *chosen = option,
        None => {}
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

/// **One arc leg's spec**: which of the kernel's modes, then that
/// mode's own fields.
///
/// Switching the mode REPLACES the spec with a fresh one of the new
/// mode rather than carrying numbers across. The modes do not share a
/// meaning for their fields — a `radius` mode's `r` is a carrier and
/// a `sweep` mode's is the same carrier with a swept angle beside it,
/// but a `via` point is not a radius at all — so a carried number
/// would sometimes be the right one and sometimes be a coincidence,
/// and a form cannot tell which.
#[expect(
    clippy::too_many_arguments,
    reason = "the row's context is seven independent facts: where, which arc, two notations, the \
              tip, whether its shape may change, and the spec itself"
)]
pub(crate) fn arc_fields(
    ui: &mut egui::Ui,
    salt: &str,
    role: &str,
    length_unit: UnitDef,
    angle_unit: UnitDef,
    at: Option<(TipState, &SpecForms)>,
    shape: ShapeEdits,
    spec: &mut ArcData<f64>,
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
    // What the lattice takes here, when the tip is known: the modes
    // this spec's dispatcher admits, and the target forms each admits.
    let modes = at.map(|(state, forms)| Admitted {
        state,
        admits: Box::new(move |mode: ArcMode| forms.admits_mode(mode)),
    });
    let targets = at.map(|(state, forms)| {
        let mode = spec.mode();
        Admitted {
            state,
            admits: Box::new(move |kind: TargetKind| forms.admits(mode, Some(kind))),
        }
    });
    let mut mode = spec.mode();
    let before = mode;
    ui.add_enabled_ui(shape.free(), |ui| {
        egui::ComboBox::from_id_salt(("arc_mode", salt))
            .selected_text(arc_mode_label(mode))
            .width(88.0)
            .show_ui(ui, |ui| {
                for &option in ArcMode::ALL {
                    offer(
                        ui,
                        modes.as_ref(),
                        &mut mode,
                        option,
                        arc_mode_label(option),
                    );
                }
            });
    });
    if mode != before {
        *spec = match at {
            Some((_, forms)) => sketch::fresh_arc_in(mode, forms),
            None => sketch::fresh_arc(mode),
        };
    }
    let targets = targets.as_ref();
    match spec {
        ArcData::Radius { r, side } => {
            named_field(ui, &radius, length_unit, FIELD_DRAG_SPEED, r);
            ui.add_enabled_ui(shape.free(), |ui| side_picker(ui, salt, side));
        }
        ArcData::Bulge { target, b } => {
            target_fields(ui, salt, length_unit, targets, shape, target);
            named_scalar(ui, "bulge", UNIT_DRAG_SPEED, b);
        }
        ArcData::Via { q, target } => {
            ui.label("via");
            point_fields(ui, length_unit, q);
            target_fields(ui, salt, length_unit, targets, shape, target);
        }
        ArcData::Center { c, winding, target } => {
            ui.label("centre");
            point_fields(ui, length_unit, c);
            ui.add_enabled_ui(shape.free(), |ui| winding_picker(ui, salt, winding));
            target_fields(ui, salt, length_unit, targets, shape, target);
        }
        ArcData::Sweep { r, side, angle } => {
            named_field(ui, &radius, length_unit, FIELD_DRAG_SPEED, r);
            ui.add_enabled_ui(shape.free(), |ui| side_picker(ui, salt, side));
            named_field(ui, "sweep", angle_unit, ANGLE_DRAG_SPEED, angle);
        }
        ArcData::ArcLen { r, side, len } => {
            named_field(ui, &radius, length_unit, FIELD_DRAG_SPEED, r);
            ui.add_enabled_ui(shape.free(), |ui| side_picker(ui, salt, side));
            named_field(ui, "arc length", length_unit, FIELD_DRAG_SPEED, len);
        }
    }
}

/// **The step a new row starts as**, by where it is going.
///
/// `at` at position 0 — it is what a chain opens with — and `line_to`
/// anywhere after it, which is the verb a chain is mostly made of. It
/// is a starting point and not a judgement: the row's own combo,
/// narrowed to what the lattice admits at that tip, is where it
/// becomes something else.
pub(crate) fn new_row_step(at: usize) -> Step<f64> {
    sketch::fresh_step(if at == 0 { Verb::At } else { Verb::LineTo })
}

/// **One authoring verb's own fields.**
///
/// Exhaustive on the kernel's [`Step`]: a verb the transition table
/// gains has to be given a row here before this compiles, as it has
/// to be given a starting step in [`sketch::fresh_step`] — a verb in
/// the menu with no fields would be a verb nobody can use.
pub(crate) fn path_step_fields(
    ui: &mut egui::Ui,
    salt: &str,
    length_unit: UnitDef,
    angle_unit: UnitDef,
    state: Option<TipState>,
    shape: ShapeEdits,
    step: &mut Step<f64>,
) {
    // The forms each of this step's arc specs takes at the tip, in the
    // step's field order; empty when the tip is not known.
    let forms = state.map_or(&[][..], |state| arc_specs_at(step.verb(), state));
    let at = |i: usize| Some((state?, *forms.get(i)?));
    // **Every field says which quantity it is.** The arms below are
    // split further than the step's shapes would need — `line` and
    // `fillet` both carry one Length — because what a number MEANS is
    // the thing a row has to say, and a shared arm can only give two
    // different quantities one name.
    match step {
        Step::At(point) => point_fields(ui, length_unit, point),
        Step::FarEndTo(point) => {
            ui.label("far end");
            point_fields(ui, length_unit, point);
        }
        Step::Angle(angle) => {
            named_field(ui, "angle", angle_unit, ANGLE_DRAG_SPEED, angle);
        }
        Step::Turn(angle) => {
            named_field(ui, "turn", angle_unit, ANGLE_DRAG_SPEED, angle);
        }
        Step::Toward { dx, dy } => {
            named_scalar(ui, "dx", UNIT_DRAG_SPEED, dx);
            named_scalar(ui, "dy", UNIT_DRAG_SPEED, dy);
        }
        Step::Line(length) => {
            named_field(ui, "length", length_unit, FIELD_DRAG_SPEED, length);
        }
        Step::Fillet { radius } => {
            named_field(ui, "fillet r", length_unit, FIELD_DRAG_SPEED, radius);
        }
        Step::LineTo(target) | Step::ContinueTo(target) | Step::TangentArcTo(target) => {
            target_fields(ui, salt, length_unit, None, shape, target);
        }
        Step::ArcTo(spec) => arc_fields(ui, salt, "", length_unit, angle_unit, at(0), shape, spec),
        // The two mixed verbs read in the order their names do, so the
        // row is the step spelled left to right.
        Step::FilletArc { radius, spec } => {
            named_field(ui, "fillet r", length_unit, FIELD_DRAG_SPEED, radius);
            arc_fields(ui, salt, "arc", length_unit, angle_unit, at(0), shape, spec);
        }
        Step::ArcFillet { spec, radius } => {
            arc_fields(ui, salt, "arc", length_unit, angle_unit, at(0), shape, spec);
            named_field(ui, "fillet r", length_unit, FIELD_DRAG_SPEED, radius);
        }
        Step::ArcFilletArc {
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
                at(0),
                shape,
                spec,
            );
            named_field(ui, "fillet r", length_unit, FIELD_DRAG_SPEED, radius);
            arc_fields(
                ui,
                &format!("{salt}_out"),
                "out arc",
                length_unit,
                angle_unit,
                at(1),
                shape,
                spec2,
            );
        }
        // The complete-loop verbs: the whole loop in one step.
        Step::Circle { centre, radius } => {
            ui.label("centre");
            point_fields(ui, length_unit, centre);
            named_field(ui, "radius", length_unit, FIELD_DRAG_SPEED, radius);
        }
        Step::CircleSplit {
            centre,
            radius,
            n,
            phase,
        } => {
            ui.label("centre");
            point_fields(ui, length_unit, centre);
            named_field(ui, "radius", length_unit, FIELD_DRAG_SPEED, radius);
            // Bounded both ways. Below two the lattice refuses, and
            // the form does not offer that; above the cap the preview
            // would build the whole subdivision every frame, and a
            // typed count is enough to exhaust memory doing it.
            // The count is the loop's vertex count — its SHAPE, not one
            // of its arguments — so a locked editor shows it and
            // does not take it.
            //
            // The range bounds what a person AUTHORS here, never what
            // is shown: the field can be handed a committed profile's
            // count, which the document admits above the cap, and a
            // drawn widget must not rewrite a document value. egui
            // clamps an existing value into the range by default.
            ui.add_enabled(
                shape.free(),
                number_field(n, COUNT_DRAG_SPEED)
                    .range(MIN_CIRCLE_SPLIT..=MAX_CIRCLE_SPLIT)
                    .clamp_existing_to_range(false)
                    .prefix("n "),
            );
            named_field(ui, "phase", angle_unit, ANGLE_DRAG_SPEED, phase);
        }
        // Structural verbs: the verb IS the whole step.
        Step::Tangent | Step::Cusp | Step::CloseTo => {}
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
    if let Some(row) = pick_unit(ui, "creation_unit", salt, Dimension::Length, chosen.def())
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
    if let Some(row) = pick_unit(ui, "creation_unit", salt, Dimension::Angle, chosen.def())
        && let Some(unit) = row.as_angle()
    {
        *chosen = unit;
    }
}

/// **How wide a written-unit combo is drawn**, in points.
///
/// Wide enough for the longest symbol the table carries (`pi rad`)
/// plus the combo's arrow. One constant rather than a number at each
/// combo: every picker in the chrome offers rows off the SAME table
/// ([`crate::props::unit_options`]), so the width they need is one
/// property of that table and a second spelling of it is free to be
/// narrower than the symbol it has to show.
pub(crate) const UNIT_PICKER_WIDTH: f32 = 72.0;

/// The combo itself: the rows `dimension` admits, with `shown`
/// selected; `Some(row)` when this frame's click chose one.
///
/// `prefix` and `salt` are the two halves of the combo's id — the
/// FAMILY of picker and which one of it. Two families that shared a
/// prefix would collide the day two of their salts agreed, and egui
/// identifies a popup by its id, so a collision opens one picker over
/// two fields rather than failing.
pub(crate) fn pick_unit(
    ui: &mut egui::Ui,
    prefix: &str,
    salt: &str,
    dimension: Dimension,
    shown: UnitDef,
) -> Option<UnitDef> {
    let options = props::unit_options(dimension);
    if options.is_empty() {
        return None;
    }
    let mut picked = None;
    egui::ComboBox::from_id_salt((prefix, salt))
        .selected_text(shown.symbol())
        .width(UNIT_PICKER_WIDTH)
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

    use super::{
        ProbeOps, drag_gesture_ops, free_move_gesture, number_field, value_gesture, vec3_row_ops,
    };
    use crate::session::{SessionOp, ValueGestureName};
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
                        let ProbeOps { gesture, typed } = free_move_gesture(NODE, frame_of);
                        vec3_row_ops(ui, 0.5, mm, gesture, typed, ops_ref);
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
                                value_gesture(ValueGestureName::Slot { node: NODE, slot }),
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

    /// **Every operation one drag emits names the SAME gesture.**
    ///
    /// The concept both vocabularies are spellings of is
    /// [`crate::session::GestureName`], and this is the row that says a
    /// control cannot drive two: the four operations are minted from
    /// one name ([`value_gesture`], [`free_move_gesture`]) rather than
    /// written per operation, so a preview cannot land in one field and
    /// a commit in another.
    ///
    /// Driven through the real widget rather than off the constructor,
    /// because what is being asserted is what a pointer causes: the
    /// press, the move and the release each emit through
    /// [`drag_gesture_ops`], and it is their ops that are read back.
    /// Both vocabularies, because one mapping serves both.
    #[test]
    fn every_operation_one_drag_emits_names_the_same_gesture() {
        for vocabulary in [Vocabulary::FreeMove, Vocabulary::Slot] {
            let mut probe = Probe::of(vocabulary);
            probe.frame(Vec::new());
            probe.frame(Vec::new());
            let x = probe.aim();
            probe.frame(vec![egui::Event::PointerMoved(x)]);
            let mut emitted = probe.frame(vec![egui::Event::PointerButton {
                pos: x,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            }]);
            emitted.extend(probe.frame(vec![egui::Event::PointerMoved(x + egui::vec2(40.0, 0.0))]));
            emitted.extend(probe.frame(vec![egui::Event::PointerButton {
                pos: x + egui::vec2(40.0, 0.0),
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            }]));
            assert_eq!(
                emitted.iter().map(kind).collect::<Vec<_>>(),
                ["begin", "preview", "commit"],
                "the drag this row reads back"
            );
            let named: Vec<_> = emitted
                .iter()
                .filter_map(SessionOp::names_gesture)
                .collect();
            assert_eq!(named.len(), emitted.len(), "an operation named no gesture");
            assert!(
                named.windows(2).all(|pair| pair[0] == pair[1]),
                "one drag drove more than one gesture: {named:?}"
            );
        }
    }

    /// **The TYPED arm names the same gesture the drag does**, and it
    /// is a separate row because it is a separate spelling.
    ///
    /// A keystroke with no pointer anywhere emits a whole
    /// begin/preview/commit of its own
    /// ([`a_keyboard_bump_with_no_drag_open_still_spells_the_whole_triple`]),
    /// and that triple is handed to [`drag_ops`] beside the drag's
    /// four rather than inside them. So the drag row above cannot see
    /// it, and a probe whose typed arm named a neighbouring instance
    /// would commit a frame onto a part the user never touched, in
    /// silence.
    ///
    /// Only the free-move vocabulary: the value drag's typed arm is
    /// `SetSlot` or `SetParam`, a direct edit that drives no gesture
    /// and names none.
    #[test]
    fn the_typed_arm_names_the_gesture_the_drag_does() {
        let mut probe = Probe::of(Vocabulary::FreeMove);
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
            "the typed triple this row reads back"
        );
        let named: Vec<_> = bumped.iter().filter_map(SessionOp::names_gesture).collect();
        assert_eq!(named.len(), bumped.len(), "an operation named no gesture");
        assert!(
            named.windows(2).all(|pair| pair[0] == pair[1]),
            "the typed triple drove more than one gesture: {named:?}"
        );
        let drawn = free_move_gesture(NODE, |mm: [f64; 3]| Frame::translation(mm))
            .gesture
            .commit
            .names_gesture();
        assert_eq!(
            named.first(),
            drawn.as_ref(),
            "the typed triple names an instance the row is not drawing"
        );
    }
}

/// **What a numeric field says, and what saying it commits.**
///
/// [`super::number_text`] is a render, so the rows over it are a table;
/// the row that matters is not, because the defect is that the render
/// is ALSO the text a click-in and a click-away hands back to the
/// document, and only driving the real widget says whether it is.
#[cfg(test)]
mod field_tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]

    use super::{install_number_formatter, number_field, number_text};
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
                number_text(value, MM),
                egui::emath::format_with_decimals_in_range(value, MM),
                "a drag lands on {value}, where this rule must say nothing new"
            );
            tenths += 1;
        }
    }

    /// **And nothing at or above one display unit renders differently
    /// either**, up to the width bound, which is the bound on how much
    /// of the chrome this rule can reach at all.
    ///
    /// **A DRAG's text names the value the drag commits**, which is the
    /// property the rule is for and the one this row measures. A drag
    /// commits `round_to_decimals(value, auto_decimals)`
    /// (`egui::DragValue::ui`), so every value a drag produces sits on
    /// the range's own decimal grid and something in the range spells
    /// it exactly.
    ///
    /// **What it is no longer is byte-identical to `egui`'s choice, and
    /// that is a disclosure rather than a caveat.**
    /// `format_with_decimals_in_range` accepts the SHORTEST spelling
    /// its own `almost_equal` passes, and that test is `f32` at
    /// 16·`f32::EPSILON` — about 1.9·10⁻⁶ relative, far coarser than
    /// the render's grid. So a dragged value whose two-decimal
    /// rounding `egui` accepts is now spelled to three: the field shows
    /// what it holds, mid-drag as everywhere else.
    ///
    /// **Off the range's grid the widget's rounding no longer passes at
    /// all**, and the second half of the row is that: a millimetre
    /// value used to keep `egui`'s rounded spelling wherever it landed
    /// within 5·10⁻⁴ of the value, and now falls to
    /// `crate::readout::number`. That is the item this unit closes,
    /// measured from the field's own door.
    #[test]
    fn a_drag_steps_through_a_text_that_names_what_it_commits() {
        let mut steps = 0_u32;
        let mut value = 1.0_f64;
        while value < 1.0e7 {
            // What a drag commits at this magnitude: the range's own
            // decimal grid, which is what makes the spelling exact.
            let dragged = (value * 1.0e3).round() / 1.0e3;
            for signed in [dragged, -dragged] {
                let text = number_text(signed, MM);
                assert_eq!(
                    text.parse::<f64>(),
                    Ok(signed),
                    "{signed} is a value a drag commits and the field spells \
                     it {text}, which is a different number"
                );
                assert!(
                    text.chars().count() <= crate::readout::MAX_CHARS,
                    "{signed} renders as {text}, past the bound"
                );
            }
            steps += 1;
            value *= 1.000_7;
        }
        assert!(steps > 9_000, "the sweep covered only {steps} magnitudes");

        // Off that grid the widget's rounding does not read back, so
        // the field shows the render instead.
        for off_grid in [1_000.000_1_f64, -1_000.000_1, 1.000_7] {
            assert_eq!(
                number_text(off_grid, MM),
                crate::readout::number(off_grid),
                "{off_grid} is not on the range's grid, so the widget's \
                 rounded spelling does not read back"
            );
            assert_ne!(
                number_text(off_grid, MM),
                egui::emath::format_with_decimals_in_range(off_grid, MM),
                "{off_grid} is only a row here because egui's own spelling \
                 of it is not the value"
            );
        }
    }

    /// **A field's text is bounded, and by the bound this crate already
    /// had.** `crate::readout::MAX_CHARS` ends
    /// `crate::readout::number`'s search; [`number_text`] is the same
    /// rule's fields' door and was applying only its truth half, so the
    /// widget's own spelling was taken at any length — three hundred
    /// and eleven characters for `f64::MAX`, which a `DragValue`
    /// renders by EXTENDING rather than clipping.
    ///
    /// Each value is a row only because the widget's spelling does not
    /// fit, which is asserted rather than assumed; the answer is the
    /// render's own, which is what makes this one rule rather than a
    /// second width policy.
    #[test]
    fn a_field_spells_no_more_than_the_render_bound_covers() {
        for value in [1.0e22_f64, -1.0e22, 1.0e50, -1.0e50, 1.0e300] {
            let widget = egui::emath::format_with_decimals_in_range(value, MM);
            assert!(
                widget.chars().count() > crate::readout::MAX_CHARS,
                "{value} is only a row here because the widget spells it in \
                 {} characters",
                widget.chars().count()
            );
            let text = number_text(value, MM);
            assert_eq!(
                text,
                crate::readout::number(value),
                "{value} has no spelling in the field's own range that fits \
                 the bound, so the field shows the render's"
            );
            assert!(
                text.chars().count() <= crate::readout::MAX_CHARS,
                "{value} renders as {text}, past the bound"
            );
        }
    }

    /// **The top of the type is what the bound is the width of**, and
    /// it is no longer an exception to it: four figures round out of
    /// `f64` there, so the value is spelled exactly, and that exact
    /// spelling is `crate::readout::MAX_CHARS` characters. Twenty-two,
    /// not three hundred and eleven.
    #[test]
    fn the_top_of_the_type_is_the_render_bounds_own_width() {
        let text = number_text(f64::MAX, MM);
        assert_eq!(text, crate::readout::number(f64::MAX));
        assert_eq!(
            text.chars().count(),
            crate::readout::MAX_CHARS,
            "the top of the type is spelled exactly, as {text}"
        );
        assert_eq!(
            egui::emath::format_with_decimals_in_range(f64::MAX, MM)
                .chars()
                .count(),
            311,
            "and the widget's own spelling of it is what this bound is for"
        );
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
            (1.0e-9, "0.000000001"),
            (1.0e-11, "1e-11"),
        ] {
            assert_eq!(number_text(value, MM), text, "the field's text for {value}");
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
        assert_eq!(number_text(0.0, MM), "0.0");
    }

    /// **An integer field is untouched, and by construction.**
    /// `DragValue::new` gives an integral value one `max_decimals(0)`,
    /// so the range is `0..=0` and the only spelling is the exact one.
    ///
    /// **The construction is now the exemption and not an accident of
    /// width.** The first half is inside `crate::readout::MAX_CHARS`
    /// and so would pass whatever the door did; the second is the band
    /// where the width bound would substitute
    /// `crate::readout::number`'s answer, which for an integer is a
    /// DIFFERENT integer. Each of those is asserted to be past the
    /// bound, so the row is a row for a reason it states — and they
    /// are `f64` magnitudes because an `i64` no longer reaches past
    /// the bound at all.
    #[test]
    fn an_integer_field_is_spelled_the_way_it_always_was() {
        for value in [3.0_f64, -12.0, 0.0, 1.0e9, i64::MAX as f64] {
            assert_eq!(number_text(value, 0..=0), format!("{value:.0}"));
        }
        for value in [1.0e23_f64, -1.0e23, 1.0e30, -1.0e30, 1.0e40] {
            let exact = format!("{value:.0}");
            assert!(
                exact.chars().count() > crate::readout::MAX_CHARS,
                "{value} is only a row here because its exact spelling is \
                 {} characters",
                exact.chars().count()
            );
            assert_eq!(
                number_text(value, 0..=0),
                exact,
                "an integer field spells its integer, not \
                 crate::readout::number's nearest reading of it"
            );
        }
    }

    /// Which constructor the harness builds its field with.
    ///
    /// [`Built::Bare`] is the twelfth site: an `egui::DragValue` that
    /// has never seen [`number_field`], written the way a lane that
    /// does not know the door exists would write it — and therefore
    /// the only widget in this file that reads the rule off the
    /// CONTEXT rather than off its own builder.
    #[derive(Clone, Copy)]
    enum Built {
        Door,
        Bare,
        /// The creation forms' constructor, which holds its value
        /// CANONICAL and renders it in a display unit
        /// ([`super::named_field`]). It reaches [`number_field`]
        /// through one more conversion than the panel does, and is the
        /// half of the chrome whose write-back is `response.changed()`.
        Named,
    }

    /// One field, laid out and driven by events, so the rule below is
    /// read off the widget rather than off the function under it.
    struct Field {
        ctx: egui::Context,
        value: f64,
        rect: egui::Rect,
        built: Built,
    }

    impl Field {
        fn new(value: f64) -> Self {
            Self::with(value, Built::Door)
        }

        /// A bare field on a context the rule has been installed on —
        /// production's arrangement, minus the door.
        fn bare(value: f64) -> Self {
            let field = Self::with(value, Built::Bare);
            install_number_formatter(&field.ctx);
            field
        }

        /// The same bare field on a context nothing has been installed
        /// on, which is what the twelfth site gets today.
        fn bare_without_the_rule(value: f64) -> Self {
            Self::with(value, Built::Bare)
        }

        /// A creation form's field over a CANONICAL value, shown in
        /// millimetres — [`super::named_field`], the constructor the
        /// panel's two value fields do not go through.
        fn named(canonical: f64) -> Self {
            Self::with(canonical, Built::Named)
        }

        fn with(value: f64, built: Built) -> Self {
            Self {
                ctx: egui::Context::default(),
                value,
                rect: egui::Rect::NOTHING,
                built,
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
            let built = self.built;
            let mut output = ctx.run_ui(input, |ui| {
                *rect = match built {
                    Built::Door => ui.add(number_field(value, 0.5)).rect,
                    Built::Bare => ui.add(egui::DragValue::new(value).speed(0.5)).rect,
                    Built::Named => {
                        ui.scope(|ui| {
                            super::named_field(
                                ui,
                                "",
                                pncad::quantity::MM.def(),
                                crate::forms::FIELD_DRAG_SPEED,
                                value,
                            );
                        })
                        .response
                        .rect
                    }
                };
            });
            output.textures_delta.clear();
        }

        /// Lay the field out, click into it, click somewhere else, and
        /// settle — the gesture that makes a field's render its commit
        /// path. Two frames before the first click because egui
        /// interacts against the PREVIOUS frame's widget rects.
        fn click_in_and_away(&mut self) {
            self.frame(Vec::new());
            self.frame(Vec::new());
            let target = self.rect.center();
            self.click(target);
            self.frame(Vec::new());
            self.click(egui::pos2(700.0, 500.0));
            self.frame(Vec::new());
            self.frame(Vec::new());
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
        for start in ROUND_TRIP {
            let mut field = Field::new(start);
            field.click_in_and_away();
            assert_eq!(
                field.value, start,
                "clicking into a field holding {start} and away again committed \
                 {} — the text it showed was not the value it held",
                field.value
            );
        }
    }

    /// The values the gesture rows are held over: three the widget's
    /// own spelling cannot name, and three it can.
    const ROUND_TRIP: [f64; 6] = [4.0e-5, 1.6e-3, 12.0, -4.0e-5, 0.0, 1024.5];

    /// Values whose RENDER is not exact — the band the round-trip rows
    /// above cannot reach, because every value in them is spelled
    /// exactly by something.
    ///
    /// Each is a value no spelling inside `crate::readout::MAX_CHARS`
    /// names exactly: the render's grid is capped a decade below ε, so
    /// an inexact render now needs more significant figures than the
    /// bound can spend rather than merely more than four.
    const RENDERED_INEXACTLY: [f64; 3] = [
        1_234.567_890_123_456_7,
        -1_234.567_890_123_456_7,
        1.234_567_890_123_456_7e12,
    ];

    /// **A click through a field commits nothing, including where the
    /// render is not the value.**
    ///
    /// [`clicking_into_a_field_and_away_again_leaves_the_value_alone`]
    /// holds the same gesture over values something spells exactly, so
    /// it passes on a door with no echo guard at all. These three do
    /// not: the text the field seeds its keyboard edit with reads back
    /// as a DIFFERENT `f64`, so writing the parse back on lost focus
    /// moves the value — by 10⁻⁶ mm for the first two, and by half a
    /// millimetre for the third.
    #[test]
    fn a_field_whose_render_is_not_exact_still_commits_nothing() {
        for start in RENDERED_INEXACTLY {
            let mut field = Field::new(start);
            field.click_in_and_away();
            assert_eq!(
                field.value, start,
                "clicking into a field holding {start} and away again \
                 committed {}, which is the text it showed rather than the \
                 value it held",
                field.value
            );
        }
    }

    /// **And a creation form's field commits nothing either**, which is
    /// the half of the chrome the panel's own guard does not reach:
    /// `super::named_field` writes back on `response.changed()`, and
    /// `egui` marks a `DragValue` changed exactly when the value MOVED
    /// — so before the guard, the one gesture that fired that write was
    /// the echo that was not exact.
    ///
    /// Driven through `named_field` rather than through
    /// [`number_field`] because the form holds its value canonical and
    /// renders it in a display unit: the round trip under test is
    /// `crate::props::in_written` → render → parse →
    /// `crate::props::from_written`, and only the real constructor has
    /// all four.
    #[test]
    fn a_creation_forms_field_commits_nothing_on_a_click_through() {
        // 1.000001 m is 1000.001 mm, which the field spells `1000.0`.
        for canonical in [1.000_001_f64, -1.000_001, 0.012_5] {
            let mut field = Field::named(canonical);
            field.click_in_and_away();
            assert_eq!(
                field.value, canonical,
                "a form field holding {canonical} m committed {} after a \
                 click that typed nothing",
                field.value
            );
        }
    }

    /// **The floor under the door carries the render and not the
    /// commit**, and that is a gap rather than a property: a bare
    /// `egui::DragValue` takes [`number_text`] from the context's
    /// `number_formatter` and has no parser to take the echo rule from,
    /// because `egui::Style` has no counterpart for parsing.
    ///
    /// Pinned rather than argued, and filed as
    /// `work/vgeom/a-bare-field-still-commits-its-own-render.md`. The
    /// door's own answer is asserted beside it so the row says what the
    /// difference IS.
    #[test]
    fn a_bare_field_commits_a_render_the_door_would_refuse() {
        let start = 1_234.567_890_123_456_7_f64;
        let mut bare = Field::bare(start);
        bare.click_in_and_away();
        assert_eq!(
            bare.value, 1_234.567_890_123_5,
            "the context carries the render, so the bare field showed \
             `1234.5678901235` — and committed it"
        );
        let mut door = Field::new(start);
        door.click_in_and_away();
        assert_eq!(
            door.value, start,
            "and the door refuses the same echo, which is the difference"
        );
    }

    /// **The twelfth site is held by the CONTEXT, not by the door.**
    ///
    /// A bare `egui::DragValue` — what a lane that has not met
    /// [`number_field`] writes, and what a new helper wrapping the
    /// widget produces — round-trips once
    /// [`install_number_formatter`] has run, because the rule is that
    /// context's default rather than a property of one constructor.
    #[test]
    fn a_bare_field_keeps_its_value_once_the_rule_is_installed() {
        for start in ROUND_TRIP {
            let mut field = Field::bare(start);
            field.click_in_and_away();
            assert_eq!(
                field.value, start,
                "a field built without the door, on a context the rule is \
                 installed on, committed {} over {start}",
                field.value
            );
        }
    }

    /// **And installing it is what does that**, held over the same
    /// widget on a context nothing has been installed on. Without this
    /// row the one above passes on every value `egui` already spells
    /// correctly and says nothing about the rule: these three are
    /// exactly the ones it does not.
    #[test]
    fn a_bare_field_without_the_rule_destroys_the_value() {
        for start in [4.0e-5_f64, -4.0e-5, 1.6e-3] {
            let mut field = Field::bare_without_the_rule(start);
            field.click_in_and_away();
            assert_ne!(
                field.value, start,
                "a bare field over {start} kept it with no rule installed — \
                 then the row above is proving nothing"
            );
        }
    }

    /// **`all_styles_mut` rather than `style_mut`.**
    ///
    /// `egui` keeps one `Style` per theme and `crate::app`'s
    /// `apply_polarity` states a PREFERENCE, so the user moves between
    /// them while the chrome runs. A formatter written onto only the
    /// theme in force at install time is dropped by the first switch —
    /// silently, and in the direction nobody watches. Both directions,
    /// because a bare context's theme is whichever one it defaults to
    /// and this row must not depend on which.
    #[test]
    fn a_bare_field_survives_a_theme_switch() {
        for theme in [egui::ThemePreference::Light, egui::ThemePreference::Dark] {
            for start in [4.0e-5_f64, 1.6e-3] {
                let mut field = Field::bare(start);
                field.ctx.set_theme(theme);
                field.click_in_and_away();
                assert_eq!(
                    field.value, start,
                    "after switching to {theme:?} a bare field committed {} \
                     over {start} — the rule reached only one style",
                    field.value
                );
            }
        }
    }
}

/// **What the panel's value field EMITS**, read off the real widget
/// over a real session.
///
/// The rows above are about one field's text. These are about the row
/// a person edits: [`super::value_field_ops`] laid out on an
/// `egui::Context`, driven by pointer and key events, with everything
/// it emits performed against a `DocSession` the way the frame entry
/// point performs a frame's operations. Nothing here stands in for
/// anything — the field is the panel's own call, the session is the
/// document's own door, and what the assertions read is the history
/// the user would undo.
///
/// It exists because both of this field's defects lived exactly here:
/// an operation emitted by a path no value test could see, and a guard
/// that discarded edits no value test was asked about.
#[cfg(test)]
mod value_field_tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]
    #![allow(clippy::panic)]

    use super::{FieldVocabulary, number_text, value_field_ops, value_gesture};
    use crate::forms::FieldWriting;
    use crate::props;
    use crate::session::ValueGestureName;
    use crate::session::{DocSession, SessionOp};
    use eframe::egui;
    use pncad::document::{
        Datum, Dimension, Doc, DocEdit, DocParam, Expr, LoopProgram, Node, ParamName,
        ProfileProgram, RecipeNodeId, RefusingReach, SlotId, apply,
    };
    use pncad::geom_core::Tol;
    use pncad::prelude::MM;
    use pncad::quantity::WrittenLength;

    /// **Which of the panel's two rows the field under test is drawn
    /// for.**
    ///
    /// `pane::properties` draws both through the one
    /// [`super::value_field_ops`] call, with the one formatter and the
    /// one parser — so a rule about the field is a rule about both,
    /// and a harness that drove only the parameter row would be
    /// reading half of what it claims. The two differ in exactly what
    /// this enum carries: which pair of doors a typed text takes, and
    /// (at a slot) whether the field shows a number or a SOURCE.
    #[derive(Clone)]
    enum Subject {
        /// A document parameter's row — `Selection::Param`'s arm.
        Param(ParamName),
        /// A feature's slot row — `slot_value_ui`.
        Slot { node: RecipeNodeId, slot: SlotId },
    }

    /// A panel value row: the field, the session behind it, and what
    /// the gesture emitted.
    struct Row {
        ctx: egui::Context,
        session: DocSession,
        subject: Subject,
        rect: egui::Rect,
        /// Every operation the frames of this gesture emitted, in
        /// order.
        emitted: Vec<SessionOp>,
        /// The subset of those that CHANGED the document — what the
        /// user would undo.
        landed: Vec<SessionOp>,
    }

    /// Apply one edit to a fixture document, answering the document
    /// and any minted id — `tests/common`'s `edited`, spelled here
    /// because this suite lives inside the crate.
    fn edited(
        doc: &Doc<ProfileProgram>,
        edit: DocEdit<ProfileProgram>,
        tol: Tol,
    ) -> (Doc<ProfileProgram>, Option<RecipeNodeId>) {
        let applied = apply(doc, &edit, tol, &RefusingReach).expect("the fixture's edit applies");
        (applied.doc, applied.record.minted)
    }

    fn inserted(
        doc: &Doc<ProfileProgram>,
        node: Node<ProfileProgram>,
        tol: Tol,
    ) -> (Doc<ProfileProgram>, RecipeNodeId) {
        let (doc, minted) = edited(doc, node_insert(node), tol);
        (doc, minted.expect("an insert mints an id"))
    }

    fn node_insert(node: Node<ProfileProgram>) -> DocEdit<ProfileProgram> {
        DocEdit::InsertNode { node }
    }

    fn len(metres: f64) -> Expr {
        Expr::literal(metres, Dimension::Length).expect("a finite length")
    }

    fn scl(value: f64) -> Expr {
        Expr::literal(value, Dimension::Scalar).expect("a finite scalar")
    }

    impl Row {
        /// One length parameter, declared in millimetres and holding
        /// `canonical` metres.
        fn millimetres(label: &str, canonical: f64) -> Self {
            let tol = Tol::witness();
            let name = ParamName::new("base_r");
            let doc: Doc<ProfileProgram> = Doc::empty_derived(label, tol);
            let mut session = DocSession::inline(doc, tol);
            let outcome = session.perform(SessionOp::CreateParam {
                name: name.clone(),
                value: DocParam::written_length(WrittenLength::canonical_in(canonical, MM)),
            });
            assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
            Self {
                ctx: egui::Context::default(),
                session,
                subject: Subject::Param(name),
                rect: egui::Rect::NOTHING,
                emitted: Vec::new(),
                landed: Vec::new(),
            }
        }

        /// **An extrude's distance slot**, written in millimetres and
        /// standing at `canonical` metres, over a square on the world
        /// xy frame — the row `slot_value_ui` draws, and the row the
        /// PR's expression door is reached from.
        ///
        /// One declared parameter comes with it (`base_r`, 4 mm), so a
        /// row about a DRIVEN slot has something to drive it with.
        fn extrude_distance(label: &str, canonical: f64) -> Self {
            let tol = Tol::witness();
            let doc: Doc<ProfileProgram> = Doc::empty_derived(label, tol);
            let (doc, _) = edited(
                &doc,
                DocEdit::SetDocParam {
                    name: ParamName::new("base_r"),
                    value: DocParam::written_length(WrittenLength::canonical_in(0.004, MM)),
                },
                tol,
            );
            let (doc, plane) = inserted(
                &doc,
                Node::Datum(Datum::Frame {
                    origin: [len(0.0), len(0.0), len(0.0)],
                    u: [scl(1.0), scl(0.0), scl(0.0)],
                    v: [scl(0.0), scl(1.0), scl(0.0)],
                }),
                tol,
            );
            let (doc, profile) = inserted(
                &doc,
                Node::Profile(ProfileProgram {
                    plane,
                    loops: vec![
                        LoopProgram::polygon([(0.0, 0.0), (0.04, 0.0), (0.04, 0.04), (0.0, 0.04)])
                            .expect("finite corners"),
                    ],
                }),
                tol,
            );
            let (doc, extrude) = inserted(
                &doc,
                Node::Extrude {
                    profile,
                    distance: Expr::written_length(WrittenLength::canonical_in(canonical, MM))
                        .expect("a finite written length"),
                },
                tol,
            );
            Self {
                ctx: egui::Context::default(),
                session: DocSession::inline(doc, tol),
                subject: Subject::Slot {
                    node: extrude,
                    slot: SlotId::Distance,
                },
                rect: egui::Rect::NOTHING,
                emitted: Vec::new(),
                landed: Vec::new(),
            }
        }

        /// **The four facts `FieldShowing` carries**, read off the
        /// document the way the panel's own row reads them — including
        /// the fixed text, which is where the two rows differ: a
        /// parameter row never has one, and a slot row has one exactly
        /// when it shows SOURCE rather than a number.
        fn field(&self) -> (FieldWriting, Dimension, f64, Option<String>) {
            match &self.subject {
                Subject::Param(_) => {
                    let row = self.row();
                    let writing = FieldWriting::of(row.dimension, row.unit);
                    (
                        writing,
                        row.dimension,
                        writing.shown(row.value.as_f64()),
                        None,
                    )
                }
                Subject::Slot { .. } => {
                    let row = self.slot();
                    let writing = FieldWriting::of(row.dimension, row.unit);
                    // `slot_value_ui`'s own rule for the fixed text,
                    // minus the in-flight draft this harness has no
                    // draft store for: a driven slot and a slot that
                    // did not evaluate show their SOURCE, and a
                    // literal that evaluated shows the number egui
                    // formats.
                    let fixed = (row.driver.is_driven() || row.value.is_err())
                        .then(|| props::field_text(&row));
                    let number = writing.shown(match row.value {
                        Ok(value) => value.as_f64(),
                        Err(_) => 0.0,
                    });
                    (writing, row.dimension, number, fixed)
                }
            }
        }

        /// What the row's field is showing, in the notation it is
        /// written in — the number, and the text the field renders for
        /// it (its fixed text where it has one, else the formatter's).
        fn showing(&self) -> (f64, String) {
            let (_, _, number, fixed) = self.field();
            (number, fixed.unwrap_or_else(|| number_text(number, 1..=3)))
        }

        fn row(&self) -> props::ParamRow {
            let Subject::Param(name) = &self.subject else {
                panic!("this row is not a parameter row");
            };
            props::param_rows(self.session.doc())
                .into_iter()
                .find(|row| &row.name == name)
                .expect("the parameter is declared")
        }

        fn slot(&self) -> props::SlotRow {
            let Subject::Slot { node, slot } = &self.subject else {
                panic!("this row is not a slot row");
            };
            props::slot_rows(self.session.doc(), *node)
                .into_iter()
                .find(|row| row.slot == *slot)
                .expect("the slot is listed")
        }

        /// One frame: lay the field out, collect what it emitted, and
        /// perform it — the frame entry point's own order.
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
            let (writing, dimension, number, fixed) = self.field();
            let subject = self.subject.clone();
            let mut ops = Vec::new();
            let rect = &mut self.rect;
            let mut output = ctx.run_ui(input, |ui| {
                let showing = super::FieldShowing {
                    writing,
                    dimension,
                    number,
                    text: fixed.clone(),
                };
                match &subject {
                    Subject::Param(name) => value_field_ops(
                        ui,
                        showing,
                        value_gesture(ValueGestureName::Param(name.clone())),
                        FieldVocabulary {
                            number: |value| SessionOp::SetParam {
                                name: name.clone(),
                                value,
                            },
                            text: |text| SessionOp::SetParamText {
                                name: name.clone(),
                                text,
                            },
                        },
                        &mut ops,
                    ),
                    Subject::Slot { node, slot } => value_field_ops(
                        ui,
                        showing,
                        value_gesture(ValueGestureName::Slot {
                            node: *node,
                            slot: *slot,
                        }),
                        FieldVocabulary {
                            number: |value| SessionOp::SetSlot {
                                node: *node,
                                slot: *slot,
                                value,
                            },
                            text: |text| SessionOp::SetSlotExpression {
                                node: *node,
                                slot: *slot,
                                text,
                            },
                        },
                        &mut ops,
                    ),
                }
                *rect = ui.min_rect();
            });
            output.textures_delta.clear();
            for op in ops {
                self.emitted.push(op.clone());
                let outcome = self.session.perform(op.clone());
                // The harness drives the paths a row can DRIVE: every
                // refusal this field can reach (a driven slot's number
                // door, an unparseable text) is a path `Row` cannot
                // hold, because a refused frame stops here. Those stay
                // with `tests/panel_edits.rs`, which reads them at the
                // session door.
                assert!(outcome.refusal.is_none(), "{op:?}: {:?}", outcome.refusal);
                if !outcome.committed.is_empty() {
                    self.landed.push(op);
                }
            }
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

        /// Settle the layout, then click into the field. Two frames
        /// first because egui interacts against the PREVIOUS frame's
        /// widget rects.
        fn click_in(&mut self) {
            self.frame(Vec::new());
            self.frame(Vec::new());
            let target = self.rect.center();
            self.click(target);
            // The frame the keyboard edit opens on: the text box takes
            // focus and selects what it was seeded with.
            self.frame(Vec::new());
        }

        /// Click somewhere else and settle — the gesture that makes a
        /// field's render its commit path.
        fn click_away(&mut self) {
            self.click(egui::pos2(700.0, 500.0));
            self.frame(Vec::new());
            self.frame(Vec::new());
        }

        /// Everything the gesture emitted since the last reading.
        fn taken(&mut self) -> Vec<SessionOp> {
            core::mem::take(&mut self.emitted)
        }

        /// The ones that moved the document.
        fn landed(&mut self) -> Vec<SessionOp> {
            core::mem::take(&mut self.landed)
        }
    }

    /// **Clicking into a parameter field and away again emits
    /// nothing** — C5 at the panel, over the case the claim is
    /// actually about.
    ///
    /// The value is chosen so the field's render is NOT exact:
    /// 1234.5678901234567 millimetres renders `1234.5678901235`, which
    /// read back is a different number. A row over a value that renders
    /// exactly would pass on any guard at all, including none.
    #[test]
    fn clicking_into_a_parameter_field_and_away_emits_no_operation() {
        let mut row = Row::millimetres("auth2-echo", 1.234_567_890_123_456_7);
        let (shown, text) = row.showing();
        assert_eq!(
            text, "1234.5678901235",
            "the fixture is a value whose render is not exact"
        );
        assert_ne!(
            text.parse::<f64>().expect("a number"),
            shown,
            "the render must MISREAD the value, or this row holds nothing"
        );
        let before = row.session.history().len();
        row.click_in();
        row.click_away();
        let emitted = row.taken();
        assert!(
            emitted.is_empty(),
            "a click in and away is not an edit: {emitted:?}"
        );
        assert_eq!(
            row.session.history().len(),
            before,
            "and costs no undo step"
        );
        assert_eq!(row.showing().0, shown, "and moves the value nowhere");
    }

    /// **One typed number is one operation and one undo step.**
    ///
    /// Held over the whole gesture rather than one frame: the defect
    /// this replaces emitted the value door's operation from two
    /// places, so a row reading only the last frame saw one of them.
    #[test]
    fn one_typed_number_is_one_undo_step() {
        let mut row = Row::millimetres("auth2-typed", 1.0);
        assert_eq!(row.showing().1, "1000.0");
        let before = row.session.history().len();
        row.click_in();
        row.frame(vec![egui::Event::Text("1002".to_owned())]);
        row.click_away();
        let landed = row.landed();
        assert!(
            matches!(landed.as_slice(), [SessionOp::SetParam { .. }]),
            "one number typed, one edit: {landed:?}"
        );
        assert_eq!(
            row.session.history().len(),
            before + 1,
            "one number typed, one undo step"
        );
        assert_eq!(row.showing().0, 1002.0);
        row.session.perform(SessionOp::Undo);
        assert_eq!(
            row.showing().0,
            1000.0,
            "and one undo takes the whole of it back"
        );
    }

    /// **`egui` hands one typed text over on TWO frames**, and this
    /// row is what says the second costs nothing.
    ///
    /// `DragValue` parses the text it buffered both on the frame the
    /// keyboard edit ends and again on the next, so a field that
    /// turned every parse into an undo step would charge two for one
    /// number.
    ///
    /// **Why the field's own guard does not stop THIS one**, and the
    /// reason is narrower than "the document has moved": the
    /// formatter runs before both parse sites, so the render the echo
    /// compares against is populated both times. What lets the second
    /// through is that the render is not the text the user typed —
    /// [`number_text`] spells at least one decimal, so `1002` is
    /// judged against `1002.0`. Where the round trip IS exact the
    /// field's guard swallows the second hand-over by itself
    /// ([`the_field_swallows_the_second_hand_over_when_its_render_round_trips`]),
    /// and what is left over for `DocSession::writes_nothing`, the
    /// document's own rule, is this case. This row is where the two
    /// are held together.
    #[test]
    fn the_second_hand_over_of_one_typed_text_changes_nothing() {
        let mut row = Row::millimetres("auth2-twice", 1.0);
        row.click_in();
        row.frame(vec![egui::Event::Text("1002".to_owned())]);
        row.click_away();
        let emitted = row.taken();
        assert_eq!(
            emitted.len(),
            2,
            "the widget hands the text over twice; if it stops, this row              and the guard behind it are answering a question nobody asks:              {emitted:?}"
        );
        assert_eq!(
            format!("{:?}", emitted[0]),
            format!("{:?}", emitted[1]),
            "and the same operation both times"
        );
        assert_eq!(row.landed().len(), 1, "one of them moves the document");
    }

    /// **The other half of the two-frame reading: where the render
    /// round-trips, the FIELD stops the second hand-over.**
    ///
    /// `1000.4` is spelled `1000.4` by the formatter, so on the
    /// second frame the buffered text and the field's render are the
    /// same string and [`props::echoed`] answers the question without
    /// the document being consulted at all. It is the row that keeps
    /// the sibling above honest about which rule does what: the two
    /// guards are two because they answer two questions, not because
    /// one of them is blind on the second frame.
    #[test]
    fn the_field_swallows_the_second_hand_over_when_its_render_round_trips() {
        let mut row = Row::millimetres("auth2-twice-exact", 1.0);
        row.click_in();
        row.frame(vec![egui::Event::Text("1000.4".to_owned())]);
        row.click_away();
        let emitted = row.taken();
        assert_eq!(
            row.showing().1,
            "1000.4",
            "the formatter spells the typed value back exactly"
        );
        assert_eq!(
            emitted.len(),
            1,
            "so the second parse IS an echo, and the field's own guard \
             is what stops it: {emitted:?}"
        );
    }

    /// **A number the render cannot distinguish from what the field
    /// shows is still an edit.**
    ///
    /// A field showing `1000.0` millimetres and a user typing
    /// `1000.00000000001`: the typed number is inside the render's own
    /// grid at this magnitude, so a guard that judged the two numbers
    /// rather than the two TEXTS would discard it — silently, with the
    /// field reverting and no refusal to read. The band is narrower
    /// than it was (the grid is capped a decade below ε) and the
    /// argument is unchanged: a band of any width has edits inside it.
    #[test]
    fn a_number_inside_the_renders_accuracy_is_still_an_edit() {
        const TYPED: f64 = 1_000.000_000_000_01;
        let mut row = Row::millimetres("auth2-near", 1.0);
        let (_, text) = row.showing();
        assert_eq!(text, "1000.0");
        assert!(
            crate::readout::reads_back(&text, TYPED),
            "the fixture must sit INSIDE the render's grid, or this row \
             is not about the guard"
        );
        let before = row.session.history().len();
        row.click_in();
        row.frame(vec![egui::Event::Text(format!("{TYPED}"))]);
        row.click_away();
        let landed = row.landed();
        assert!(
            matches!(landed.as_slice(), [SessionOp::SetParam { .. }]),
            "the edit the user typed: {landed:?}"
        );
        assert_eq!(row.session.history().len(), before + 1);
        assert_eq!(row.showing().0, TYPED, "and the field says what they typed");
    }

    /// **A unit-bearing text takes the OTHER door**, from the same
    /// field and the same gesture.
    #[test]
    fn a_unit_bearing_text_takes_the_text_door() {
        let mut row = Row::millimetres("auth2-unit-text", 1.0);
        row.click_in();
        row.frame(vec![egui::Event::Text("2 m".to_owned())]);
        row.click_away();
        let landed = row.landed();
        assert!(
            matches!(landed.as_slice(), [SessionOp::SetParamText { .. }]),
            "{landed:?}"
        );
        let after = row.row();
        assert_eq!(after.value, props::SlotValue::Continuous(2.0));
        assert_eq!(after.unit.map(|unit| unit.symbol()), Some("m"));
    }

    /// The doc-parameter fixture, read back: a declaration minted at
    /// `canonical` metres really is written in millimetres, so the
    /// rows above are about a field whose render and whose value are
    /// in different notations.
    #[test]
    fn the_fixture_is_written_in_millimetres() {
        let row = Row::millimetres("auth2-fixture", 0.05);
        assert_eq!(row.row().unit.map(|unit| unit.symbol()), Some("mm"));
        assert_eq!(row.showing().0, 50.0);
    }

    /// **Re-typing a literal slot's own SOURCE writes nothing.**
    ///
    /// The slot row's text door is the one the field's guard cannot
    /// answer for, and this is why: a literal slot that evaluated
    /// shows its NUMBER (`slot_value_ui` pins no text for it), so the
    /// field's render is `8.0` while the slot's source is `8 mm` — and
    /// `props::echoed` is right to let the second through, because it
    /// is not the field's render. What answers it is the DOCUMENT's
    /// rule at the expression door: the parse of `8 mm` is the
    /// expression already standing, so nothing is written and nothing
    /// is undone.
    #[test]
    fn re_typing_a_literal_slots_own_source_is_not_an_edit() {
        let mut row = Row::extrude_distance("auth2-slot-source", 0.008);
        let (shown, render) = row.showing();
        assert_eq!(shown, 8.0, "the slot is shown in millimetres");
        assert_eq!(render, "8.0", "and a literal that evaluated shows a NUMBER");
        let source = row.slot().source.expect("a literal has source");
        assert_eq!(source, "8 mm", "written in the unit it was authored in");
        assert!(
            !props::echoed(&source, &render),
            "the source is not the field's render, so the field's guard \
             cannot be what answers this"
        );

        let before = row.session.history().len();
        row.click_in();
        row.frame(vec![egui::Event::Text(source.clone())]);
        row.click_away();
        let landed = row.landed();
        assert!(
            landed.is_empty(),
            "re-typing the slot's own source writes nothing: {landed:?}"
        );
        assert_eq!(
            row.session.history().len(),
            before,
            "and costs no undo step"
        );
        assert_eq!(row.showing().0, shown, "and moves the value nowhere");
    }

    /// **The same rule for a DRIVEN slot re-typed in different
    /// characters** — and the one row that drives the field over a
    /// fixed text rather than a number.
    ///
    /// A driven slot shows its source, so the echo guard swallows the
    /// source spelled exactly; re-spaced, it is a different text and
    /// reaches the door, where the same expression parses back out of
    /// it.
    #[test]
    fn re_typing_a_driven_slots_source_respaced_is_not_an_edit() {
        let mut row = Row::extrude_distance("auth2-slot-driven", 0.008);
        let Subject::Slot { node, slot } = row.subject.clone() else {
            panic!("the fixture is a slot row");
        };
        let outcome = row.session.perform(SessionOp::SetSlotExpression {
            node,
            slot,
            text: "base_r * 2.0".to_owned(),
        });
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        let (_, render) = row.showing();
        assert_eq!(render, "base_r * 2.0", "a driven slot shows its source");

        let respaced = render.replace(' ', "");
        assert_ne!(respaced, render);
        assert!(
            !props::echoed(&respaced, &render),
            "re-spaced, it is not the field's render"
        );
        let before = row.session.history().len();
        row.click_in();
        row.frame(vec![egui::Event::Text(respaced)]);
        row.click_away();
        let landed = row.landed();
        assert!(
            landed.is_empty(),
            "the same expression, differently spelled, writes nothing: {landed:?}"
        );
        assert_eq!(
            row.session.history().len(),
            before,
            "and costs no undo step"
        );
    }

    /// **A slot row's number door still lands**, so the two rows above
    /// are reading a guard rather than a field that emits nothing.
    #[test]
    fn a_number_typed_over_a_slot_is_one_undo_step() {
        let mut row = Row::extrude_distance("auth2-slot-number", 0.008);
        let before = row.session.history().len();
        row.click_in();
        row.frame(vec![egui::Event::Text("12".to_owned())]);
        row.click_away();
        let landed = row.landed();
        assert!(
            matches!(landed.as_slice(), [SessionOp::SetSlot { .. }]),
            "one number typed, one edit: {landed:?}"
        );
        assert_eq!(row.session.history().len(), before + 1);
        assert_eq!(row.showing().0, 12.0);
    }
}
