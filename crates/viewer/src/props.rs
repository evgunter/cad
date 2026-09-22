//! The property panel's model: the two things it draws rows for — a
//! node's SLOTS ([`slot_rows`]) and the document's PARAMETERS
//! ([`param_rows`]) — their current values, the notation each is
//! written in, and what an edit to each one is allowed to be.
//!
//! # Canonical inside, written units outside
//!
//! Every value that CROSSES this module is canonical metres and
//! radians — [`SlotValue`], [`slot_edit`], every session operation.
//! What changed from v1 (whose ruling was "canonical units and nothing
//! else, the display layer is not a dependency of the panels") is only
//! the last inch: a row now also carries the display unit its literal
//! REMEMBERS ([`SlotRow::unit`]), and the panel divides by that unit's
//! factor to show a number and multiplies by it to author one.
//!
//! The reason it is a change worth making rather than a convenience:
//! the display unit is already stored per literal and already round-
//! trips through persistence — `Expr::literal_with_unit` exists
//! precisely so that "25 mm" comes back as `25 mm` and not as `0.025`.
//! A panel that showed `0.025` was throwing away information the
//! document was carrying for it. The conversion is the parser's own
//! one-multiply semantics in both directions
//! ([`in_written`]/[`from_written`]), so a value authored through this
//! panel and a value authored through the text door are the same bits.
//!
//! Two rules keep the unit from drifting:
//!
//! * **An edit to a number never changes how the number is written.**
//!   [`slot_edit`] takes the slot's STORED unit and re-attaches it, so
//!   dragging a slider cannot silently re-write a literal into another
//!   unit.
//! * **Changing how it is written is its own operation.** That is
//!   `SessionOp::SetSlotUnit`, which rewrites the display unit and
//!   leaves the canonical bits alone.
//!
//! **A document parameter keeps both rules, through its own pair of
//! doors.** [`ParamRow::unit`] is the notation its DECLARATION names
//! ([`DocParam::Continuous`]'s `display_unit`), the panel divides and
//! multiplies by it exactly as it does for a slot ([`shown_in`] /
//! [`authored_in`], through `crate::forms::FieldWriting`), and the
//! two facts move separately: [`param_edit`] writes a number into a
//! standing declaration and cannot mention the notation, and
//! [`param_unit_edit`] rewrites the notation and cannot mention the
//! value. Each is a carry-forward edit — `DocEdit::SetDocParamValue`
//! and `DocEdit::SetDocParamUnit` read the declaration off the
//! document and reuse it whole — so neither can drop the dimension or
//! the distribution it never names.
//!
//! The parameter pair differs from the slot pair in what it is made
//! of, and only there. A slot's unit change rebuilds a literal
//! ([`slot_unit_edit`]) because a literal's whole state is its value
//! and its unit; a parameter's rides on the declaration beside the
//! dimension and the distribution, so the kernel carries it forward
//! rather than the panel rebuilding it. Neither door validates: the
//! notation a parameter may be written in is
//! `UnitSym::measures`'s answer, asked at the edit door.
//!
//! **A parameter is authored in a notation at both of its doors.**
//! [`doc_param`] mints a declaration through
//! `DocParam::written_length`/`written_angle` — total doors, so the
//! unit measures the dimension by construction — and the standing
//! row's field reads `50 mm` through the ONE parser a
//! unit-bearing number has in this workspace, `editor_core::parse`.
//! Nothing here maps a symbol to a factor.
//!
//! # Structural is not continuous
//!
//! The recipe's structural/continuous divide is typed, not emergent:
//! a Count slot is edited by `SetStructuralParam` and everything else
//! by `SetParam`, and [`SlotRow::structural`] carries which. The panel
//! never picks the edit arm by inspecting the value.
//!
//! # The expression-driven refusal
//!
//! Setting a number into a slot that is DRIVEN — by a document
//! parameter or by arithmetic — is refused, with an affordance
//! (the ratified micro-decision). [`SlotDriver`] is how a slot says
//! which it is, using only public expression API: a bare literal is a
//! leaf with no parameter references, and anything else is driven.
//!
//! **What the affordance offers, measured against this substrate.**
//! The typed expression API has a text door in BOTH directions —
//! `parse_expr` inward and `unparse` outward (issue #1103, closed) —
//! so the panel neither parses nor renders expression text itself. It
//! shows the slot's own source, and hands edited text straight back
//! through the parser. Beside that it still shows what a user needs in
//! order to act on a refusal: the slot's CURRENT VALUE under the
//! document's parameters, and the names of the parameters driving it,
//! each of which the panel can navigate to and edit as a document
//! parameter.
//!
//! # One field for numbers and expressions
//!
//! **Both panel value fields have this shape**, and one function draws
//! them (`crate::widgets::value_field_ops`): the slot row's and the
//! document parameter's. What a user types decides which of the
//! field's two doors the edit takes, and [`field_edit`] is the one
//! reading of the text that decides it:
//!
//! * Bare digits mean a number in the field's WRITTEN unit, through
//!   [`from_written`] — `SessionOp::SetSlot` at a slot,
//!   `SessionOp::SetParam` at a parameter — leaving the stored display
//!   unit alone.
//! * **Anything else is text for the field's other door**, including a
//!   number with a unit on it. That is the unit-authoring rule, and it
//!   is one rule rather than two: `25 in` is read by the one parser a
//!   unit-bearing number has, and the literal it yields REMEMBERS `in`
//!   — so the field and the unit picker agree afterwards without
//!   either being told about the other.
//!
//! **The two doors differ in WHERE that text may land, and only
//! there.** A slot can be driven by an expression, so its text door is
//! `SessionOp::SetSlotExpression` and `w * 2` is an edit. A document
//! parameter holds an `f64` and nothing else — there is no
//! `SetDocParamExpression` — so its text door is
//! `SessionOp::SetParamText`, which takes a number and its notation
//! (`50 mm`) and refuses every other expression by name.
//!
//! What a slot field SHOWS is [`field_text`]: a bare literal shows its
//! number alone (the unit is the picker's to say, not the field's),
//! and everything else shows its source. A parameter's always shows
//! its number, because a parameter is never driven by anything.
//!
//! **Text the field itself produced is not an edit**, at either field
//! — [`echoed`], one function because it is one rule, asked of the
//! render the field actually made.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{
    Dimension, DimensionError, Doc, DocEdit, DocParam, DocParamValue, EvalError, Expr, Node,
    ParamName, ProfileProgram, RecipeNodeId, SlotId, UnitSym, VectorSlot, eval, eval_count,
    unparse,
};
use pncad::prelude::{M, RAD};
use pncad::quantity::{self, UNITS, UnitDef, UnitQuantity, WrittenAngle, WrittenLength};

/// What is in a slot right now.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SlotValue {
    /// A continuous value, canonical metres/radians (or a bare
    /// scalar).
    Continuous(f64),
    /// An exact integer, from a Count slot.
    Count(i64),
}

impl SlotValue {
    /// **The one rule for which arm a widget's `f64` becomes.**
    ///
    /// The DIMENSION decides, and nothing else: a `Count` dimension
    /// takes the value truncated toward zero, everything else takes it
    /// as-is. Not the widget, not the arm the value currently has, and
    /// not the slot's structurality read separately — `SlotId::
    /// is_structural` is itself defined as "the dimension is Count", so
    /// deciding on the dimension is deciding on the same thing in one
    /// place instead of three.
    ///
    /// Three call sites wanted this and two of them had spelled it
    /// differently, which is why it is a function.
    ///
    /// # Errors
    ///
    /// [`DimensionError::NonFiniteLiteral`] for a non-finite value in
    /// a `Count` dimension — **the same refusal, by name, that
    /// `Expr::literal` raises for the continuous half**, which is what
    /// makes [`field_edit`]'s promise true. That door admits `inf` and
    /// `NaN` as Numbers on the stated ground that the refusal
    /// downstream names the problem; downstream of a `Count` dimension
    /// there is no literal to refuse, because `Expr::count` takes an
    /// integer. `f64 as i64` is a SATURATING cast, not a conversion —
    /// `NaN` is `0` and `inf` is `i64::MAX` — so without this the word
    /// the user typed leaves as an ordinary count that no one asked
    /// for, and every guard downstream of it sees a number.
    ///
    /// The continuous arm refuses nothing here: its value reaches
    /// `Expr::literal` intact and is refused there, which is the
    /// arrangement this arm is being brought into line with rather
    /// than a second one.
    pub fn of(dimension: Dimension, value: f64) -> Result<Self, DimensionError> {
        if dimension == Dimension::Count {
            if !value.is_finite() {
                return Err(DimensionError::NonFiniteLiteral);
            }
            Ok(Self::Count(value as i64))
        } else {
            Ok(Self::Continuous(value))
        }
    }

    /// The value as an `f64`, for a widget that has only one kind of
    /// number. Lossless for every count a recipe can carry.
    pub fn as_f64(self) -> f64 {
        match self {
            Self::Continuous(v) => v,
            Self::Count(v) => v as f64,
        }
    }
}

/// Every unit a value of `dimension` may be written in — the picker's
/// options, read off the closed table so a unit added to `quantity`
/// appears here the day it lands.
pub fn unit_options(dimension: Dimension) -> Vec<UnitDef> {
    let wanted = match dimension {
        Dimension::Length => UnitQuantity::Length,
        Dimension::Angle => UnitQuantity::Angle,
        Dimension::Scalar | Dimension::Count => return Vec::new(),
    };
    UNITS
        .into_iter()
        .filter(|row| row.quantity() == wanted)
        .collect()
}

/// **The unit a panel row is RENDERED in**: the notation the row's
/// literal remembers, or — for a row driven by an expression — the
/// canonical one.
///
/// The `None` case is not a literal's; a literal always names its unit
/// (`Expr::display_unit` answers `None` only for the kinds that are not
/// literals). It is a COMPUTED value's, and a computed value was never
/// written by anyone, so a reader has to choose. Every reader in this
/// crate chooses through here, which is the point of it being a
/// function: the choice is CANONICAL, and `unparse` renders such a
/// value the same way, so the panel and the text door agree by
/// construction rather than by two files being edited together.
///
/// `Count` has no units at all — an instance count is a number, not a
/// quantity — and answers `None`.
pub fn rendering_unit(dimension: Dimension, remembered: Option<UnitDef>) -> Option<UnitDef> {
    if let Some(unit) = remembered {
        return Some(unit);
    }
    match dimension {
        Dimension::Length => Some(M.def()),
        Dimension::Angle => Some(RAD.def()),
        Dimension::Scalar => Some(quantity::ONE.def()),
        Dimension::Count => None,
    }
}

/// A canonical value as it is WRITTEN in `unit` — one divide, the
/// inverse of the text door's one multiply.
///
pub fn in_written(canonical: f64, unit: UnitDef) -> f64 {
    canonical / unit.factor()
}

/// [`in_written`] where that value IS a number, and `None` where the
/// notation cannot name this value at all.
///
/// **A notation is a change of exponent, and an exponent can leave the
/// type.** The divide is by a factor below one for six of the closed
/// table's eight rows, so it is a multiplication UP by up to three
/// decades, and a canonical length above `f64::MAX * MILLI`
/// (`1.7976931348623156e305` m) has no millimetre value — the quotient
/// is `inf`, which [`render_number`] spells `inf` and
/// [`crate::readout::number`] spells `inf` too. A text reading
/// infinity names no value, and the value it was asked about is one
/// the document holds perfectly well.
///
/// **The other end is the same question and is not symmetric.** One
/// row's factor is above one — `pi rad`, at π — so a canonical angle
/// of exactly `5e-324` rad divides to `0.0`, and a text reading zero
/// is a hundred percent away from the value it claims to be, which is
/// the first thing [`crate::readout::REL_TOLERANCE`] refuses. It is
/// one value rather than a band because π is barely above one: two
/// subnormals up, the quotient is a subnormal again. Measured, not
/// reasoned: `5e-324 / π == 0.0` and `1e-323 / π == 5e-324`.
///
/// The sweep behind both sentences is `quantity::UNITS` read for its
/// `factor`, every row: `mm` `1e-3`, `cm` `1e-2`, `in` `0.0254` and
/// `deg` `π/180` are below one, `m`, `rad` and the dimensionless row
/// are exactly one, and `pi rad` is π. So the overflow arm is live for
/// four rows and the flush-to-zero arm for one, and a row added to the
/// table is covered the day it lands rather than the day this sentence
/// is updated.
///
/// `None` is a fact about the PAIR and not about either half: the
/// value is a number and the unit is a unit, and the value written in
/// that unit is neither.
pub fn written(canonical: f64, unit: UnitDef) -> Option<f64> {
    let written = in_written(canonical, unit);
    let names_the_value = written.is_finite() && (written != 0.0 || canonical == 0.0);
    names_the_value.then_some(written)
}

/// What a render says where [`written`] answers `None` — the one
/// spelling of that refusal, so the three renders that can meet it
/// say the same thing.
///
/// It names the unit because the unit is half of what failed: the
/// reader's next move is to write the row in a coarser notation, and
/// a marker that did not say which notation could not name the value
/// would not tell them that.
pub fn no_reading(unit: UnitDef) -> String {
    format!("no {} reading", unit.symbol())
}

/// A canonical value as text a person reads, written in `unit` and
/// carrying its symbol.
///
/// **The crate's render ([`crate::readout::number`]) over
/// [`written`]**, which is the pairing every chrome sentence that
/// writes a canonical value in a display unit wants: the shortest
/// spelling that reads back, of a value that exists. Where the value
/// does not exist it is [`no_reading`], never `inf`.
///
/// `render_number` is deliberately not the render here. That one is a
/// FIELD's — `{:?}`'s exact round-tripping digits, because the text a
/// field shows is the text an edit starts from — and a sentence is not
/// a commit path.
pub fn written_text(canonical: f64, unit: UnitDef) -> String {
    match written(canonical, unit) {
        Some(value) => format!("{} {}", crate::readout::number(value), unit.symbol()),
        None => no_reading(unit),
    }
}

/// A written value back to canonical — `n * factor`, which is exactly
/// the literal semantics `parse_expr` applies to `n <symbol>`, so a
/// number typed into a panel field and the same number typed into the
/// expression field land on the same bits.
pub fn from_written(written: f64, unit: UnitDef) -> f64 {
    written * unit.factor()
}

/// [`in_written`] over a field that may name NO unit — one canonical
/// value as such a field shows it.
///
/// `None` is not a missing answer here: it is the field that is a
/// number rather than a quantity (a count, a bare scalar), whose shown
/// number is its canonical one. Spelled once because both panel fields
/// need it and a hand-written `map_or` at each is the same identity
/// written twice, free to become two.
pub fn shown_in(unit: Option<UnitDef>, canonical: f64) -> f64 {
    unit.map_or(canonical, |unit| in_written(canonical, unit))
}

/// [`written_text`] over a field that may name NO unit — one canonical
/// value as text a person reads, in the notation such a field shows.
///
/// `None` means what it means in [`shown_in`]: the field is a number
/// rather than a quantity, so there is no conversion to make and no
/// symbol to carry, and the render is [`crate::readout::number`]
/// alone. A value cannot fail to be nameable in no notation, which is
/// why this answers a `String` where [`written`] answers an `Option`.
///
/// Spelled here rather than as a `map_or` at each caller for
/// [`shown_in`]'s own reason: a hand-written `map_or` is the same
/// identity written twice, free to become two.
pub fn shown_text(unit: Option<UnitDef>, canonical: f64) -> String {
    unit.map_or_else(
        || crate::readout::number(canonical),
        |unit| written_text(canonical, unit),
    )
}

/// [`from_written`] over a field that may name no unit — one number
/// read out of such a field, canonical. [`shown_in`]'s inverse, and
/// `None` means what it means there.
pub fn authored_in(unit: Option<UnitDef>, written: f64) -> f64 {
    unit.map_or(written, |unit| from_written(written, unit))
}

/// What decides a slot's value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SlotDriver {
    /// A bare literal: editable in place.
    Literal,
    /// An expression. Direct numeric editing is refused; `params` are
    /// the document parameters it references, in first-seen order and
    /// deduplicated — the affordance's navigation targets.
    Expression {
        /// The parameters this expression reads.
        params: Vec<ParamName>,
    },
}

impl SlotDriver {
    /// Classify an expression using public expression API only.
    ///
    /// A bare literal is a LEAF (no child at index 0) that references
    /// no parameter. Everything else — a parameter reference, or any
    /// arithmetic, however constant — is driven, which is the
    /// conservative direction: refusing to overwrite a computed slot
    /// is recoverable, silently flattening one to a number is not.
    pub fn of(expr: &Expr) -> Self {
        let mut refs = Vec::new();
        expr.param_refs(&mut refs);
        if refs.is_empty() && expr.child(0).is_none() {
            return Self::Literal;
        }
        let mut params: Vec<ParamName> = Vec::new();
        for (name, _) in refs {
            if !params.contains(&name) {
                params.push(name);
            }
        }
        Self::Expression { params }
    }

    /// Whether this slot refuses a direct numeric edit.
    pub fn is_driven(&self) -> bool {
        matches!(self, Self::Expression { .. })
    }
}

/// Why a slot has no value to show.
#[derive(Clone, Debug, PartialEq)]
pub enum SlotFault {
    /// The expression did not evaluate — the kernel's own typed reason
    /// (an unbound parameter, a non-finite result).
    Eval(EvalError),
    /// The node LISTED this slot and then carried no expression for it.
    /// A broken `Node::slots`/`Node::expr` postcondition, reported
    /// rather than skipped; see [`slot_row`].
    NoExpression,
}

impl core::fmt::Display for SlotFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Eval(error) => write!(f, "{error}"),
            Self::NoExpression => write!(
                f,
                "the node lists this slot but carries no expression for it"
            ),
        }
    }
}

impl core::error::Error for SlotFault {}

/// One editable row of the property panel.
#[derive(Clone, Debug, PartialEq)]
pub struct SlotRow {
    /// The named slot (never an index).
    pub slot: SlotId,
    /// The dimension the slot's expression must have.
    pub dimension: Dimension,
    /// Whether the slot is structural — the `SetStructuralParam`
    /// half of the edit vocabulary.
    pub structural: bool,
    /// Literal, or driven by an expression.
    pub driver: SlotDriver,
    /// The value the slot has under the document's parameters, or the
    /// typed reason it has none.
    pub value: Result<SlotValue, SlotFault>,
    /// The display unit the slot's expression REMEMBERS — `None` only
    /// where there is no literal to remember one, i.e. a slot driven by
    /// an expression. This is the STORED fact, not the shown one: put
    /// it through [`rendering_unit`] for the unit to display in, and
    /// hand it back to [`slot_edit`] unchanged so that editing the
    /// number does not rewrite how the number is written.
    pub unit: Option<UnitDef>,
    /// The slot expression's own SOURCE TEXT (`unparse`), `None` only
    /// where the node lists a slot it carries no expression for.
    ///
    /// It is the text an edit to this slot revises, so it is carried
    /// even for a slot whose value did not evaluate — a slot driven by
    /// an unbound parameter is exactly the one a user has to be able
    /// to retype.
    pub source: Option<String>,
}

/// The rows for one node, in the node vocabulary's own slot order.
///
/// Empty for a node that carries no expressions (a boolean, a mate, an
/// instance) — which is a true statement about that node, not a
/// failure.
pub fn slot_rows(doc: &Doc<ProfileProgram>, id: RecipeNodeId) -> Vec<SlotRow> {
    let Some(node) = doc.node(id) else {
        return Vec::new();
    };
    node.slots()
        .into_iter()
        .map(|slot| slot_row(doc, node, slot))
        .collect()
}

/// One row for a slot the node lists.
///
/// **`Node::slots()` is authoritative**: it is documented as "the
/// domain of `Node::expr`", so a slot it lists and `expr` denies is a
/// broken postcondition in the node vocabulary, not a slot this panel
/// should quietly skip. Dropping it silently is what a `filter_map`
/// here used to do — a fail-loud codebase's panel showing a node with
/// one fewer row than it has and no way to notice.
///
/// The row is still a value, not a panic: the panel says the slot is
/// there and that its value could not be read, which is the same shape
/// every other unreadable value takes here.
fn slot_row(doc: &Doc<ProfileProgram>, node: &Node<ProfileProgram>, slot: SlotId) -> SlotRow {
    let Some(expr) = node.expr(slot) else {
        return SlotRow {
            slot,
            dimension: slot.dimension(),
            structural: slot.is_structural(),
            // No expression to classify. Reported as driven, which is
            // the refusing direction: nothing here should be
            // overwritten with a number on the strength of an
            // invariant that just failed.
            driver: SlotDriver::Expression { params: Vec::new() },
            value: Err(SlotFault::NoExpression),
            unit: None,
            source: None,
        };
    };
    let env = doc.param_env::<f64>();
    let value = if slot.dimension() == Dimension::Count {
        eval_count(expr, &env)
            .map(SlotValue::Count)
            .map_err(SlotFault::Eval)
    } else {
        eval::<f64>(expr, &env)
            .map(SlotValue::Continuous)
            .map_err(SlotFault::Eval)
    };
    SlotRow {
        slot,
        dimension: slot.dimension(),
        structural: slot.is_structural(),
        driver: SlotDriver::of(expr),
        value,
        unit: expr.display_unit(),
        source: Some(unparse(expr)),
    }
}

/// What the value field SHOWS for one row.
///
/// **The unit is the picker's to say, not the field's.** A bare
/// literal therefore shows its number ALONE, in the unit the row is
/// written in — the same number [`in_written`] gives and the combo box
/// beside it names, said once instead of twice.
///
/// Everything else shows its SOURCE: a driven slot says what drives
/// it, which is both the honest reading of a computed value and the
/// text an edit to it revises. A slot whose value did not evaluate is
/// the same case — the source is what there is to fix.
///
/// **A literal whose value the notation cannot name shows
/// [`no_reading`]** rather than a number, because there is no number
/// to show ([`written`]).
pub fn field_text(row: &SlotRow) -> String {
    match (&row.driver, &row.value) {
        (SlotDriver::Literal, Ok(value)) => match row.unit {
            // **And the notation may not be able to name it**, which
            // is [`written`]'s question and not this one's: a literal
            // above `f64::MAX * MILLI` metres has no millimetre value,
            // and `{:?}` spells that `inf` — a field claiming a value
            // the document does not hold. [`no_reading`] says so
            // instead, and a marker is safe in a field for the reason
            // [`echoed`] gives: every field built through
            // `crate::widgets::number_field` refuses text equal to its
            // own render, so the marker cannot be handed back as an
            // edit.
            Some(unit) => {
                written(value.as_f64(), unit).map_or_else(|| no_reading(unit), render_number)
            }
            // D2 addendum row 4: this arm IS the literal case, and
            // every literal names the unit it was written in
            // (`Expr::display_unit` answers `None` only for the kinds
            // that are not literals). A fallback here would be a second
            // authority on how a stored value reads — which is the hole
            // the dimensionless row closed.
            None => unreachable!(
                "slot {:?} is driven by a literal, yet its expression remembers no unit",
                row.slot
            ),
        },
        _ => row.source.clone().unwrap_or_default(),
    }
}

/// A number as the chrome writes it: `{:?}`'s shortest round-tripping
/// digits, with a bare integral form (`8.0` → `8`) — a field showing
/// `8` and a field showing `8.0` say the same thing, and the shorter
/// one is what a user typed.
///
/// One of the crate's TWO number policies, and the one for a number
/// in an editable field or in a sentence quoting one: the value fields
/// here, and the frame poses the picker and the feature tree name a
/// frame by ([`crate::tree::frame_pose`]). The other is
/// [`crate::readout::number`], which spells a number to fit a FIXED
/// WIDTH and trades digits for it — right for the View pane's δ field,
/// wrong for a coordinate a reader compares against what the panel
/// shows.
pub fn render_number(value: f64) -> String {
    let repr = format!("{value:?}");
    match repr.strip_suffix(".0") {
        Some(integral) => integral.to_string(),
        None => repr,
    }
}

/// What text typed into a value field MEANS.
#[derive(Clone, Debug, PartialEq)]
pub enum FieldEdit {
    /// A bare number, in the unit the field is written in — the
    /// numeric door, `SessionOp::SetSlot` at a slot and
    /// `SessionOp::SetParam` at a document parameter. Both leave the
    /// notation alone: one re-attaches the slot's stored display unit
    /// and the other carries the declaration forward.
    Number(f64),
    /// Anything else: the text door's, which is `SessionOp::
    /// SetSlotExpression` at a slot and `SessionOp::SetParamText` at a
    /// document parameter. A number carrying a UNIT is this variant at
    /// both — see the module docs' authoring rule — and so is every
    /// expression, which is an edit at one field and a refusal by name
    /// at the other.
    ///
    /// Named for the slot's reading of it because that is the wider
    /// one: a parameter's door accepts a strict subset.
    Expression(String),
    /// Nothing was typed. Not an edit, and not a refusal either.
    Empty,
}

/// Read a value field's text (module docs: one field, two doors).
///
/// The test for "a bare number" is `f64`'s own: what Rust reads as a
/// float is a number and everything else is source. That draws the
/// line exactly where the user sees it — `25` is a number, `25 in` is
/// not, `w * 2` is not — and it inherits `1e-3` and `-4` for free.
/// A non-finite spelling (`inf`, `NaN`) reads as a Number here on
/// purpose: `Expr::literal`'s refusal names the problem ("a literal
/// value must be finite"), where the parser would only say the word
/// is not a parameter.
pub fn field_edit(text: &str) -> FieldEdit {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return FieldEdit::Empty;
    }
    match trimmed.parse::<f64>() {
        Ok(number) => FieldEdit::Number(number),
        Err(_) => FieldEdit::Expression(trimmed.to_owned()),
    }
}

/// **Text the field itself produced is not an edit** — the one home
/// of that rule, for both of the panel's value fields.
///
/// `typed` is what the field's parser was handed; `rendered` is what
/// the field's own formatter returned for the value it holds, on the
/// frame the parse ran. The answer is whether the one is the other.
///
/// # Why a field commits anything it was not typed into
///
/// An `egui::DragValue` seeds its keyboard edit with the text it last
/// rendered and writes the parse back when focus leaves, so clicking
/// into a field and clicking away again hands the chrome's own render
/// straight back at it. That text is accepted within the render's own
/// accuracy ([`crate::readout::REL_TOLERANCE`], through
/// `crate::widgets::number_text`), so writing it back can move the
/// value by up to that much AND cost an undo step for a click nobody
/// meant as one. `readout`'s own words: the number a value moves to on
/// purpose is one a user types, never one the chrome echoed at them.
///
/// # Judged as TEXT, which is what the question is about
///
/// An echo is a text the field produced and a re-type is a text the
/// user produced, and the render is the thing that tells them apart —
/// exactly, with no tolerance to choose and no band for a real edit to
/// fall into. A numeric comparison cannot do it: the render is lossy
/// by construction, so any number-shaped test has to accept a band
/// around the value, and every edit inside that band is then discarded
/// — ±0.5 mm on a field showing `1000` in millimetres, which is an
/// edit a person can plainly mean and plainly type.
///
/// It also asks one question of both of a field's doors. A row showing
/// SOURCE rather than a number (a slot driven by an expression, a slot
/// whose value did not evaluate) echoes that source, and the same
/// comparison answers for it; a number typed over it is no echo of
/// anything and takes its door, which is what makes a driven slot's
/// refusal reachable.
///
/// Whitespace is not part of what a field says: the parser trims
/// before reading ([`field_edit`]), so this does too.
pub fn echoed(typed: &str, rendered: &str) -> bool {
    typed.trim() == rendered.trim()
}

/// The display unit a slot's expression currently REMEMBERS — the
/// value [`slot_edit`] must be handed so that writing a number leaves
/// the notation alone.
///
/// One function rather than a field read at each call site because
/// three of them wanted it (the direct write, and both ends of a
/// gesture) and each would otherwise have spelled "the node's
/// expression's display unit, or None if the node or the expression is
/// gone" for itself.
pub fn slot_unit(doc: &Doc<ProfileProgram>, node: RecipeNodeId, slot: SlotId) -> Option<UnitDef> {
    doc.node(node)?.expr(slot)?.display_unit()
}

/// One document-level parameter, as the panel shows it.
#[derive(Clone, Debug, PartialEq)]
pub struct ParamRow {
    /// The parameter's name.
    pub name: ParamName,
    /// Its declared dimension.
    pub dimension: Dimension,
    /// Its exact stored value, canonical.
    pub value: SlotValue,
    /// The display unit the parameter was AUTHORED in — `None` only for
    /// a `Count`, which is a number rather than a quantity and has no
    /// notation to name.
    ///
    /// Unlike [`SlotRow::unit`] there is no computed case: a
    /// parameter's notation rides with its DECLARATION, beside the
    /// dimension, so a continuous parameter always names one. It is
    /// also why no value edit has to carry it — `SetDocParamValue`
    /// leaves the declaration alone ([`param_edit`]) where a slot's
    /// literal has to be rebuilt around its unit.
    pub unit: Option<UnitDef>,
}

/// Every document parameter, name order.
pub fn param_rows(doc: &Doc<ProfileProgram>) -> Vec<ParamRow> {
    doc.params()
        .iter()
        .map(|(name, param)| ParamRow {
            name: name.clone(),
            dimension: param.dim(),
            value: match param {
                DocParam::Continuous { value, .. } => SlotValue::Continuous(*value),
                DocParam::Count { value } => SlotValue::Count(*value),
            },
            unit: match param {
                DocParam::Continuous { display_unit, .. } => Some(display_unit.def()),
                DocParam::Count { .. } => None,
            },
        })
        .collect()
}

/// The edit that writes `value` into `slot` on `node`.
///
/// The structural/continuous divide is decided by the SLOT, which is
/// the only thing that knows: a Count slot takes
/// `SetStructuralParam`, everything else `SetParam`. Returning the
/// edit rather than applying it keeps this a pure function of the
/// request, which is what lets a test assert on the emitted edit.
///
/// `unit` is the display unit the new literal REMEMBERS. Callers pass
/// the slot's existing one (`SlotRow::unit`), which is what makes an
/// edit to the number leave the way it is written alone; the door that
/// changes the unit is `SessionOp::SetSlotUnit`. A `Count` slot has no
/// unit to carry and the argument is ignored for it, which is not a
/// silent drop: `Dimension::Count` has no row in the unit table at all,
/// so there is nothing a caller could legitimately have passed.
///
/// # Errors
///
/// The dimension refusal `Expr::literal` raises for a value that is
/// not finite, and `Expr::literal_with_unit`'s
/// `DisplayUnitMismatch` for a unit that does not measure the slot's
/// dimension — reported rather than silently dropped, because a
/// mismatched unit means the caller's idea of the slot disagrees with
/// the slot's own.
pub fn slot_edit(
    node: RecipeNodeId,
    slot: SlotId,
    value: SlotValue,
    unit: Option<UnitDef>,
) -> Result<DocEdit<ProfileProgram>, pncad::document::DimensionError> {
    let expr = match (value, unit) {
        (SlotValue::Count(count), _) => Expr::count(count),
        (SlotValue::Continuous(v), None) => Expr::literal(v, slot.dimension())?,
        (SlotValue::Continuous(v), Some(unit)) => {
            Expr::literal_with_unit(v, slot.dimension(), unit)?
        }
    };
    Ok(if slot.is_structural() {
        DocEdit::SetStructuralParam { node, slot, expr }
    } else {
        DocEdit::SetParam { node, slot, expr }
    })
}

/// The `DocParam` a dimension, a value and a NOTATION mint — the
/// panel's CREATE-parameter affordance, where a declaration really is
/// being authored from parts. Moving an existing parameter's value is
/// [`param_edit`]'s door and re-noting it is [`param_unit_edit`]'s;
/// neither mints a declaration at all.
///
/// `value` is canonical, as everything crossing this module is, and
/// `unit` is the notation the declaration will REMEMBER — the shape
/// [`slot_edit`] already has, for its reason: the two are independent
/// facts about the thing being authored, and deriving one from the
/// other is how a form comes to author a value it did not mean.
/// `None` is the field that names no notation (a `Count`, a bare
/// `Scalar`), and the canonical declaration is right for it.
///
/// **Minted through `DocParam::written_length` /
/// `written_angle`, which are TOTAL**: each takes a typed view that is
/// an index into a row of its own quantity, so the unit measures the
/// dimension by construction and there is no pairing left for the
/// declaration to get wrong.
///
/// **A unit that does not measure `dimension` is a caller's mistake
/// and says so.** `UnitDef::as_length`/`as_angle` answer `None` for
/// it, and the two readings of that `None` — "there is no notation to
/// name here" and "a notation was offered that this dimension cannot
/// be written in" — are not the same fact. Quietly minting the
/// canonical declaration for the second would store a notation nobody
/// asked for and report success, which is the confident wrong answer
/// this codebase refuses; the pairing has no run-time recourse at this
/// seat, so it is `unreachable!` rather than a `Result` nobody could
/// act on. The one caller reaches it through
/// `ViewerBehavior::new_param_unit`, which answers off the same
/// dimension.
///
/// **No multiply.** `WrittenLength::canonical_in` attaches the
/// notation to an already-canonical value, which is the form's shape:
/// the draft behind a form field is canonical whatever the picker
/// says (`crate::widgets::unit_field`), so applying the factor here
/// would apply it twice.
pub fn doc_param(dimension: Dimension, value: SlotValue, unit: Option<UnitDef>) -> DocParam {
    let value = match value {
        SlotValue::Count(value) => return DocParam::Count { value },
        SlotValue::Continuous(value) => value,
    };
    // No notation offered at all: the canonical declaration is the
    // whole of what there is to mint.
    let Some(unit) = unit else {
        return DocParam::continuous(dimension, value);
    };
    let written = match dimension {
        Dimension::Length => unit
            .as_length()
            .map(|unit| DocParam::written_length(WrittenLength::canonical_in(value, unit))),
        Dimension::Angle => unit
            .as_angle()
            .map(|unit| DocParam::written_angle(WrittenAngle::canonical_in(value, unit))),
        // A dimension with no written door — a bare `Scalar` — has one
        // unit and the canonical declaration already names it, so
        // being handed it is no mistake and nothing to refuse.
        Dimension::Scalar | Dimension::Count => {
            return DocParam::continuous(dimension, value);
        }
    };
    written.unwrap_or_else(|| {
        unreachable!(
            "a {dimension} parameter was offered {}, which does not measure it",
            unit.symbol()
        )
    })
}

/// The edit that changes how a standing parameter's value is WRITTEN,
/// leaving its exact value alone — [`param_edit`]'s mirror over the
/// other field of the declaration, and [`slot_unit_edit`]'s
/// counterpart for a parameter.
///
/// Unlike a slot's, this rebuilds nothing: `DocEdit::SetDocParamUnit`
/// carries the declaration forward, so the dimension, the value and
/// any distribution ride through without this function naming them.
/// The refusals (an undeclared name, a `Count`, a unit that does not
/// measure the declared dimension) belong to the edit door; this is
/// the spelling, not a second validator.
pub fn param_unit_edit(name: ParamName, unit: UnitDef) -> DocEdit<ProfileProgram> {
    DocEdit::SetDocParamUnit {
        name,
        unit: UnitSym::from_def(&unit),
    }
}

/// The edit that writes a new VALUE into an already-declared document
/// parameter.
///
/// The panel authors a number and nothing else, so it spells the edit
/// that carries a number and nothing else: `SetDocParamValue` reads
/// the declaration off the document and keeps it — the dimension and
/// any distribution alike. The panel is therefore structurally unable
/// to delete an annotation it never mentions, rather than remembering
/// to copy one across.
///
/// The refusals (an undeclared name, a kind mismatch) belong to the
/// edit door; this is the spelling, not a second validator.
pub fn param_edit(name: ParamName, value: SlotValue) -> DocEdit<ProfileProgram> {
    DocEdit::SetDocParamValue {
        name,
        value: match value {
            SlotValue::Count(value) => DocParamValue::Count(value),
            SlotValue::Continuous(value) => DocParamValue::Continuous(value),
        },
    }
}

/// One ROW OF THE PANEL: either a slot on its own, or the three
/// components of a 3-vector shown together.
///
/// # Why the panel groups at all
///
/// A datum plane's origin is three `SlotId`s and one idea. Shown as
/// three stacked rows it reads as three unrelated numbers that happen
/// to sort adjacently, and a plane with an origin and a normal is six
/// of them; shown as `origin  [x] [y] [z]` it reads as the point it is.
/// The grouping is presentation and nothing else — the underlying
/// edits are still per-slot `SetParam`s, one per component, and a
/// gesture on the y field is a gesture on `Origin(Y)`.
///
/// # The grouping is the vocabulary's, not this module's
///
/// Which slots form a vector is `SlotId::component` (see
/// `VectorSlot`), so a vector-valued slot added to the node vocabulary
/// is grouped here without an edit, and one added WITHOUT answering
/// there fails to compile there. This module decides only the layout.
///
/// # A partial family degrades rather than lying
///
/// A group is emitted only when all three components are present. A
/// node listing two of them is a `Node::slots` postcondition break, and
/// the honest rendering of it is the components it actually has, as
/// scalars — not a vector with a hole in it, and not a silently dropped
/// row. This is the same posture `slot_row` takes for a listed slot
/// with no expression.
#[derive(Clone, Debug, PartialEq)]
pub enum SlotGroup {
    /// A slot that is not a vector component, or one whose family is
    /// incomplete.
    Scalar(SlotRow),
    /// One 3-vector, x/y/z in [`pncad::document::Axis3::ALL`] order.
    ///
    /// The rows are BOXED: three of them are three times the size of a
    /// scalar arm, and a `Vec<SlotGroup>` sized for the widest variant
    /// would pay that for every scalar slot in the panel. The indirection
    /// costs one allocation per vector row drawn, which is a per-selection
    /// cost of at most a handful.
    Vector {
        /// Which vector this is.
        family: VectorSlot,
        /// The three component rows, x/y/z.
        rows: Box<[SlotRow; 3]>,
    },
}

impl SlotGroup {
    /// Every row this group shows, in display order — the flattening
    /// that makes "the groups cover exactly the rows" checkable.
    pub fn rows(&self) -> Vec<&SlotRow> {
        match self {
            Self::Scalar(row) => vec![row],
            Self::Vector { rows, .. } => rows.iter().collect(),
        }
    }
}

/// Fold a node's slot rows into panel rows.
///
/// **Order is the vocabulary's.** A group lands where its FIRST
/// component appeared in `rows`, and everything else keeps its
/// position, so the panel's reading order is still `Node::slots`'
/// deterministic order and grouping never reshuffles a node's
/// properties. A family whose components are not adjacent in that
/// order is still grouped — the fact that they are one vector does not
/// depend on the vocabulary having listed them together.
///
/// **Every input row appears in exactly one output group** (the
/// property the suite pins): the walk consumes each row once, by
/// position.
pub fn group_rows(rows: Vec<SlotRow>) -> Vec<SlotGroup> {
    // Which positions hold each family's components, indexed by
    // `Axis3::index` — the recipe's own component order, so the row a
    // panel draws first is the row the document stores first.
    let mut families: Vec<(VectorSlot, [Option<usize>; 3])> = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        let Some((family, axis)) = row.slot.component() else {
            continue;
        };
        let at = match families.iter().position(|(seen, _)| *seen == family) {
            Some(at) => at,
            None => {
                families.push((family, [None; 3]));
                families.len() - 1
            }
        };
        // `at` is a position just read from (or just pushed onto) this
        // very vector, so the miss arm is unreachable; written as a
        // `if let` rather than an index so that no panicking door
        // exists here at all.
        if let Some((_, slots)) = families.get_mut(at) {
            let slot = &mut slots[axis.index()];
            // First component wins a repeated axis; a repeat is a
            // `Node::slots` break, and the extra row falls through to
            // the scalar arm below rather than displacing the one
            // already held.
            if slot.is_none() {
                *slot = Some(index);
            }
        }
    }
    // Only COMPLETE families group; the rest of their rows stay
    // scalars (the degradation rule on `SlotGroup`).
    let complete: Vec<(VectorSlot, [usize; 3])> = families
        .into_iter()
        .filter_map(|(family, slots)| {
            let [x, y, z] = slots;
            Some((family, [x?, y?, z?]))
        })
        .collect();
    let mut taken = vec![false; rows.len()];
    let mut out: Vec<SlotGroup> = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        if taken[index] {
            continue;
        }
        match complete
            .iter()
            .find(|(_, positions)| positions.contains(&index))
        {
            Some(&(family, positions)) => {
                for position in positions {
                    taken[position] = true;
                }
                out.push(SlotGroup::Vector {
                    family,
                    rows: Box::new(positions.map(|position| rows[position].clone())),
                });
            }
            None => {
                taken[index] = true;
                out.push(SlotGroup::Scalar(row.clone()));
            }
        }
    }
    out
}

/// The panel rows for one node: [`slot_rows`] folded by [`group_rows`].
pub fn slot_groups(doc: &Doc<ProfileProgram>, id: RecipeNodeId) -> Vec<SlotGroup> {
    group_rows(slot_rows(doc, id))
}

/// The edit that changes how a slot's literal is WRITTEN, leaving its
/// canonical value bit-identical.
///
/// This is the "here's how I want this number written" door, and it is
/// separate from [`slot_edit`] on purpose: the value and its notation
/// are independent facts about a literal (D7 excludes the unit from
/// expression identity entirely), so an operation that changed both
/// would make it impossible to change either alone.
///
/// # Errors
///
/// [`SlotUnitFault`], per arm — a slot with no expression, one whose
/// expression is not a bare literal, or a unit that does not measure
/// the slot's dimension.
pub fn slot_unit_edit(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
    slot: SlotId,
    unit: UnitDef,
) -> Result<DocEdit<ProfileProgram>, SlotUnitFault> {
    let expr = doc
        .node(node)
        .and_then(|n| n.expr(slot))
        .ok_or(SlotUnitFault::NoExpression { node, slot })?;
    // A display unit belongs to a LITERAL. An expression's value is
    // computed, so there is no authored notation to change — refused
    // rather than silently flattened to the computed number, which is
    // the same direction `SlotDriver` refuses a numeric edit in.
    let value = expr
        .literal_value()
        .ok_or(SlotUnitFault::NotALiteral { node, slot })?;
    let expr = Expr::literal_with_unit(value, slot.dimension(), unit)
        .map_err(|source| SlotUnitFault::Dimension { slot, source })?;
    Ok(DocEdit::SetParam { node, slot, expr })
}

/// Why a display-unit change was refused.
#[derive(Clone, Debug, PartialEq)]
pub enum SlotUnitFault {
    /// The node carries no expression in that slot.
    NoExpression {
        /// The node named.
        node: RecipeNodeId,
        /// The slot named.
        slot: SlotId,
    },
    /// The slot's expression is computed, so it has no authored
    /// notation to rewrite.
    NotALiteral {
        /// The node.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
    },
    /// The unit does not measure the slot's dimension.
    Dimension {
        /// The slot.
        slot: SlotId,
        /// The expression layer's own refusal, unaltered.
        source: pncad::document::DimensionError,
    },
}

impl core::fmt::Display for SlotUnitFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoExpression { node, slot } => {
                write!(
                    f,
                    "node {} carries no expression in its {} slot",
                    node.0,
                    slot.label()
                )
            }
            Self::NotALiteral { node, slot } => write!(
                f,
                "the {} slot on node {} is computed, so it has no written unit to change — \
                 set an expression to change what it says",
                slot.label(),
                node.0
            ),
            Self::Dimension { slot, source } => {
                write!(
                    f,
                    "the {} slot cannot be written in that unit: {source}",
                    slot.label()
                )
            }
        }
    }
}

impl core::error::Error for SlotUnitFault {}

#[cfg(test)]
mod written_tests {
    use super::{in_written, no_reading, written, written_text};
    use pncad::document::{Dimension, SlotId};
    use pncad::prelude::{M, MM, PI, RAD};

    /// **A notation is a change of exponent, and an exponent can leave
    /// the type** — in both directions, and [`written`] is the one
    /// place that is asked.
    ///
    /// **The pair, because neither half says anything alone.** A door
    /// answering `None` for everything would satisfy the first two
    /// assertions; the values below them are the ones the chrome shows
    /// and they have to come back. Both edges are named as the
    /// PRODUCTS they are rather than as magnitudes somebody typed —
    /// `f64::MAX * MILLI` is the coarsest length with a millimetre
    /// value, two subnormals is the smallest angle `pi rad` does not
    /// divide to zero — so a bound drawn at a round number fails the
    /// second half of each pair.
    #[test]
    fn a_value_the_notation_cannot_name_has_no_written_value() {
        let (mm, pi_rad) = (MM.def(), PI.def());

        assert_eq!(
            written(1.0e306, mm),
            None,
            "1e306 m has no millimetre value: the quotient is inf"
        );
        let coarsest = f64::MAX * 1.0e-3;
        assert_eq!(
            written(coarsest, mm),
            Some(in_written(coarsest, mm)),
            "the coarsest length whose millimetre value is a number is written in millimetres"
        );

        assert_eq!(
            written(5.0e-324, pi_rad),
            None,
            "the smallest subnormal over π is 0.0, which is not this angle"
        );
        let two_subnormals = 1.0e-323;
        assert_eq!(
            written(two_subnormals, pi_rad),
            Some(in_written(two_subnormals, pi_rad)),
            "one step up the quotient is a subnormal again, and a subnormal is a value"
        );

        // The values the chrome actually shows, on both dimensions and
        // on the canonical rows, which divide by one.
        assert_eq!(written(0.025, mm), Some(25.0));
        assert_eq!(written(1.5, M.def()), Some(1.5));
        assert_eq!(written(1.0, RAD.def()), Some(1.0));
        assert_eq!(
            written(0.0, mm),
            Some(0.0),
            "a value that IS zero is a zero in every notation"
        );
    }

    /// **The render says the notation cannot name it, and never spells
    /// `inf`** — which is the whole of this class: `inf` is a text
    /// that reads back as no value at all, offered for a value the
    /// document holds perfectly well.
    #[test]
    fn a_render_of_an_unnameable_value_is_not_an_infinity() {
        let mm = MM.def();
        assert_eq!(no_reading(mm), "no mm reading");
        assert_eq!(written_text(1.0e306, mm), no_reading(mm));
        assert!(
            !written_text(1.0e306, mm).contains("inf"),
            "the render spelled the product instead of refusing it"
        );
        assert_eq!(
            written_text(5.0e-324, PI.def()),
            no_reading(PI.def()),
            "and the other end is the same refusal, not a zero"
        );
        // The ordinary case is a number and its symbol, unchanged.
        assert_eq!(written_text(0.025, mm), "25 mm");
        assert_eq!(written_text(1.5, M.def()), "1.5 m");
    }

    /// **A field's text is what an edit starts from**, so the one
    /// thing it must not be is a value the document does not hold.
    /// `render_number` is `{:?}`, which spells an overflowed quotient
    /// `inf`; the marker is what stands there instead.
    ///
    /// **The pair**: the same row a decade below the overflow shows an
    /// ordinary number, so a `field_text` that gave up on millimetres
    /// altogether fails the second half.
    #[test]
    fn a_literal_with_no_millimetre_value_does_not_show_one() {
        let row = |canonical: f64| super::SlotRow {
            slot: SlotId::ShellThickness,
            dimension: Dimension::Length,
            structural: false,
            driver: super::SlotDriver::Literal,
            value: Ok(super::SlotValue::Continuous(canonical)),
            unit: Some(MM.def()),
            source: Some("unused".to_owned()),
        };
        assert_eq!(
            super::field_text(&row(1.0e306)),
            no_reading(MM.def()),
            "a literal whose millimetre value is not a number showed one"
        );
        assert_eq!(
            super::field_text(&row(1.0e304)),
            "9.999999999999999e306",
            "and one decade below the overflow is an ordinary field, spelled \
             by `{{:?}}`'s exact round-tripping digits — the quotient's, \
             which is not `1e307`"
        );
    }
}
