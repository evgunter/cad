//! The property panel's model: a node's slots, their current values,
//! and what an edit to each one is allowed to be.
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
//!   dragging a slider cannot silently canonicalize a literal, and a
//!   literal that remembers nothing keeps remembering nothing.
//! * **Changing how it is written is its own operation.** That is
//!   `SessionOp::SetSlotUnit`, which rewrites the display unit and
//!   leaves the canonical bits alone.
//!
//! Document parameters are the one asymmetry, and it is the storage's:
//! `DocParam::Continuous` has no unit field, so a parameter has no
//! authored unit to remember and its row shows the canonical one. The
//! panel does not paper over that.
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
//! A slot has ONE value field, and what a user types into it decides
//! which door the edit takes ([`field_edit`]):
//!
//! * Bare digits mean exactly what they have always meant — a number
//!   in the slot's WRITTEN unit, through [`from_written`] and
//!   `SessionOp::SetSlot`, leaving the stored display unit alone.
//! * **Anything else is expression source**, including a number with
//!   a unit on it. That is the unit-authoring rule, and it is one
//!   rule rather than two: `25 in` is the expression `25 in`, whose
//!   literal REMEMBERS `in` because that is what the text door does
//!   with a suffix — so the field and the unit picker agree
//!   afterwards without either being told about the other.
//!
//! What the field SHOWS is [`field_text`]: a bare literal shows its
//! number alone (the unit is the picker's to say, not the field's),
//! and everything else shows its source.

use pncad::document::{
    Dimension, Doc, DocEdit, DocParam, DocParamValue, EvalError, Expr, Node, ParamName,
    ProfileProgram, RecipeNodeId, SlotId, VectorSlot, eval, eval_count, unparse,
};
use pncad::prelude::{M, RAD};
use pncad::quantity::{UNITS, UnitDef, UnitQuantity};

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
    pub fn of(dimension: Dimension, value: f64) -> Self {
        if dimension == Dimension::Count {
            Self::Count(value as i64)
        } else {
            Self::Continuous(value)
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

/// The unit a value of `dimension` is WRITTEN in, given the unit its
/// literal remembers (`None` when it remembers none, and for every
/// value that is not a stored literal).
///
/// The fallback is the CANONICAL row (`m`, `rad`), never "no unit":
/// factor exactly 1.0, so showing a canonical value in it is the f64
/// identity, and the reader gets a suffix saying which canonical unit
/// they are looking at instead of a bare number they have to know is
/// metres. `Scalar` and `Count` have no units at all — a direction
/// component and an instance count are numbers, not quantities — and
/// answer `None`.
pub fn written_unit(dimension: Dimension, remembered: Option<UnitDef>) -> Option<UnitDef> {
    if let Some(unit) = remembered {
        return Some(unit);
    }
    match dimension {
        Dimension::Length => Some(M.def()),
        Dimension::Angle => Some(RAD.def()),
        Dimension::Scalar | Dimension::Count => None,
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

/// A canonical value as it is WRITTEN in `unit` — one divide, the
/// inverse of the text door's one multiply.
pub fn in_written(canonical: f64, unit: Option<UnitDef>) -> f64 {
    unit.map_or(canonical, |u| canonical / u.factor())
}

/// A written value back to canonical — `n * factor`, which is exactly
/// the literal semantics `parse_expr` applies to `n <symbol>`, so a
/// number typed into a panel field and the same number typed into the
/// expression field land on the same bits.
pub fn from_written(written: f64, unit: Option<UnitDef>) -> f64 {
    unit.map_or(written, |u| written * u.factor())
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
    /// The display unit the slot's expression REMEMBERS, `None` when it
    /// remembers none. This is the STORED fact, not the shown one — put
    /// it through [`written_unit`] for the unit to display in, and hand
    /// it back to [`slot_edit`] unchanged so that editing the number
    /// does not rewrite how the number is written.
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
pub fn field_text(row: &SlotRow) -> String {
    match (&row.driver, &row.value) {
        (SlotDriver::Literal, Ok(value)) => {
            let written = in_written(value.as_f64(), written_unit(row.dimension, row.unit));
            render_number(written)
        }
        _ => row.source.clone().unwrap_or_default(),
    }
}

/// A number as the field writes it: `{:?}`'s shortest round-tripping
/// digits, with a bare integral form (`8.0` → `8`) — a field showing
/// `8` and a field showing `8.0` say the same thing, and the shorter
/// one is what a user typed.
fn render_number(value: f64) -> String {
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
    /// numeric door (`SessionOp::SetSlot`), which re-attaches the
    /// slot's stored display unit and so leaves the notation alone.
    Number(f64),
    /// Anything else: source for the expression door
    /// (`SessionOp::SetSlotExpression`), which is also where a number
    /// carrying a UNIT goes — see the module docs' authoring rule.
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
    /// Its exact stored value.
    pub value: SlotValue,
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

/// The `DocParam` a dimension and a value mint — the panel's
/// CREATE-parameter affordance, where a declaration really is being
/// authored from parts. Moving an existing parameter's value is
/// [`param_edit`]'s door, which mints no declaration at all.
pub fn doc_param(dimension: Dimension, value: SlotValue) -> DocParam {
    match value {
        SlotValue::Count(value) => DocParam::Count { value },
        SlotValue::Continuous(value) => DocParam::continuous(dimension, value),
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
    unit: Option<UnitDef>,
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
    let expr = match unit {
        None => Expr::literal(value, slot.dimension()),
        Some(unit) => Expr::literal_with_unit(value, slot.dimension(), unit),
    }
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
