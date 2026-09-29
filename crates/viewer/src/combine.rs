//! **Combining bodies**: the modal layer-3 tools that turn body picks
//! into exactly one committed combining edit, and the lowering their
//! ops go through (GAUTH-4, Phase B).
//!
//! # Shape
//!
//! The revolve tool's shape, once per tool here, on the shared seat
//! machinery [`crate::seats`] carries: single-select stays ruled, so
//! each tool holds its picks in tool state and consumes the ordinary
//! selection stream — a tree click is a node pick directly, a viewport
//! face or edge pick reaches the node whose DRAWN body the ray met
//! (`Selection::seat_node`). Everything before the commit is tool
//! state; the document transition is one [`SessionOp`], committed
//! through the session's ordinary commit door as one action — one
//! `DocEdit::InsertNode` for every tool but the duplicate tool, whose
//! op inserts a pattern and its two projections as one undo.
//!
//! The seat vocabulary, the pick rule, the survival step and the
//! id-reuse hazard it does not cover (issue #1384) are all
//! [`crate::seats`]'s, and are not restated here.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{
    BooleanOp, Doc, Evaluation, Expr, Node, PartSelect, PatternKind, ProfileProgram, RecipeNodeId,
};
use pncad::geom_core::{Tol, Vec3};
use pncad::select::SplitHalf;

use crate::seats::{Seat, SeatError, SeatEvent, Seats};
use crate::session::refuse::one_body;
use crate::session::{PartSelectSpec, PatternRuleSpec, SessionOp};
use crate::vocab::vocabulary;

/// **The boolean tool**: two sequential body picks and one operation
/// choice, committing one [`SessionOp::AddBoolean`].
///
/// The operand order is DATA, not a convenience: `Subtract` keeps the
/// first pick and removes the second, so the panel says which held pick
/// is which and the seats are named for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BooleanTool {
    seats: Seats,
}

impl Default for BooleanTool {
    fn default() -> Self {
        Self::new()
    }
}

impl BooleanTool {
    /// A tool holding nothing.
    pub const fn new() -> Self {
        Self {
            seats: Seats::new([Seat::OperandA, Seat::OperandB]),
        }
    }

    /// The seats, roles and picks together — what the panel's line is
    /// composed from ([`crate::seats::seat_line`]).
    pub fn seats(&self) -> &Seats {
        &self.seats
    }

    /// The held first operand — the body a subtraction KEEPS.
    pub fn a(&self) -> Option<RecipeNodeId> {
        self.seats.held(0)
    }

    /// The held second operand — the body a subtraction REMOVES.
    pub fn b(&self) -> Option<RecipeNodeId> {
        self.seats.held(1)
    }

    /// Feed one node pick; `doc` routes it, and does not judge it.
    pub fn pick(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId) {
        self.seats.pick(doc, node);
    }

    /// Empty both seats.
    pub fn clear(&mut self) {
        self.seats.clear();
    }

    /// The survival step ([`crate::seats`]).
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<SeatEvent> {
        self.seats.reconcile(doc)
    }

    /// **The one committed edit**: the session op that inserts the
    /// boolean node through the ordinary commit door.
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] until both operands are picked. Node kinds,
    /// and the two-operands-are-one-node case, refuse at the session
    /// door.
    pub fn op(&self, op: BooleanOp) -> Result<SessionOp, SeatError> {
        Ok(SessionOp::AddBoolean {
            op,
            a: self.seats.require(0)?,
            b: self.seats.require(1)?,
        })
    }
}

/// **The split tool**: a body pick and a datum-plane pick, committing
/// one [`SessionOp::AddSplit`].
///
/// The second seat is a datum PLANE and not a body — `Node::Split`'s
/// tool operand is the plane the cut is taken on, which is why this
/// tool's two seats want different kinds where the boolean's want the
/// same one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SplitTool {
    seats: Seats,
}

impl Default for SplitTool {
    fn default() -> Self {
        Self::new()
    }
}

impl SplitTool {
    /// A tool holding nothing.
    pub const fn new() -> Self {
        Self {
            seats: Seats::new([Seat::SplitTarget, Seat::SplitPlane]),
        }
    }

    /// The seats, roles and picks together — what the panel's line is
    /// composed from ([`crate::seats::seat_line`]).
    pub fn seats(&self) -> &Seats {
        &self.seats
    }

    /// The held target body.
    pub fn target(&self) -> Option<RecipeNodeId> {
        self.seats.held(0)
    }

    /// The held cutting plane.
    pub fn plane(&self) -> Option<RecipeNodeId> {
        self.seats.held(1)
    }

    /// Feed one node pick; `doc` routes it, and does not judge it.
    pub fn pick(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId) {
        self.seats.pick(doc, node);
    }

    /// Empty both seats.
    pub fn clear(&mut self) {
        self.seats.clear();
    }

    /// The survival step ([`crate::seats`]).
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<SeatEvent> {
        self.seats.reconcile(doc)
    }

    /// **The one committed edit**.
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] until both seats are filled.
    pub fn op(&self) -> Result<SessionOp, SeatError> {
        Ok(SessionOp::AddSplit {
            target: self.seats.require(0)?,
            tool: self.seats.require(1)?,
        })
    }
}

/// **The transform tool**: one body pick plus the placement fields,
/// committing one [`SessionOp::AddTransform`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransformTool {
    seats: Seats,
}

impl Default for TransformTool {
    fn default() -> Self {
        Self::new()
    }
}

impl TransformTool {
    /// A tool holding nothing.
    pub const fn new() -> Self {
        Self {
            seats: Seats::one(Seat::TransformBody),
        }
    }

    /// The seats, roles and picks together — what the panel's line is
    /// composed from ([`crate::seats::seat_line`]).
    pub fn seats(&self) -> &Seats {
        &self.seats
    }

    /// The held body.
    pub fn input(&self) -> Option<RecipeNodeId> {
        self.seats.held(0)
    }

    /// Feed one node pick — a second pick REPLACES the first, this tool
    /// having only the one seat. `doc` routes it, and does not judge it.
    pub fn pick(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId) {
        self.seats.pick(doc, node);
    }

    /// Empty the seat.
    pub fn clear(&mut self) {
        self.seats.clear();
    }

    /// The survival step ([`crate::seats`]).
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<SeatEvent> {
        self.seats.reconcile(doc)
    }

    /// **The one committed edit**: the rigid placement.
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] until a body is picked.
    pub fn op(
        &self,
        translation: [Expr; 3],
        rotation_axis: [Expr; 3],
        rotation_angle: Expr,
    ) -> Result<SessionOp, SeatError> {
        Ok(SessionOp::AddTransform {
            input: self.seats.require(0)?,
            translation,
            rotation_axis,
            rotation_angle,
        })
    }
}

vocabulary! {
    /// **What a pattern's placements come out as** — the pattern form's
    /// output choice, and the only difference between its two nodes.
    ///
    /// [`Node::Pattern`] and [`Node::PlacedUnion`] share one rule
    /// vocabulary and one per-instance naming, and differ in their RESULT:
    /// N bodies that stay separate, or ONE body that is their union. That
    /// is a node-kind fork rather than a flag on one node (spec D3 forbids
    /// a variant forking a node's result type), so the choice picks the
    /// door — the shape `BlendKindChoice` takes for fillet and chamfer.
    ///
    /// **Fusing is not free.** A [`Node::PlacedUnion`] certifies its
    /// placements disjoint and refuses typed on its own badge when it
    /// cannot, where a [`Node::Pattern`] over the same rule builds
    /// regardless: the choice is between two honest answers, not between
    /// a strict door and a lax one.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum PatternOutputChoice {
        /// N separate bodies ([`Node::Pattern`]).
        #[default]
        Instances = "instances",
        /// ONE body, the union of the prototype at every placement
        /// ([`Node::PlacedUnion`]).
        Fused = "fused",
    }

    /// Both choices with their button labels — the chrome's radio row
    /// and a test that sweeps them.
    pub const ALL;
}

/// **The pattern tool**: a body pick, and — for the circular rule
/// only — a datum-axis pick, committing one [`SessionOp::AddPattern`]
/// or one [`SessionOp::AddPlacedUnion`].
///
/// **One tool, two nodes**: the output choice
/// ([`PatternOutputChoice`]) picks which op each door mints, because
/// everything the tool holds is the same either way — the same
/// prototype seat, the same axis seat, the same count and the same
/// rule fields. The kernel's fused node takes exactly a prototype, a
/// count and a parametric rule, so there is nothing more to collect.
///
/// The axis seat is filled by an ordinary second pick whichever rule is
/// chosen, and READ only by [`PatternTool::circular_op`]: a user who
/// picks an axis and then chooses the linear rule is not corrected,
/// because the pick is not wrong until a commit needs it to mean
/// something.
///
/// **Two commit doors rather than one taking a rule value**: the axis
/// is a SEAT for one rule and absent from the other, so a single door
/// would either carry an axis the linear arm ignores or an `Option`
/// every caller has to fill. The chrome's rule choice picks the door,
/// which is the same decision it was already making.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PatternTool {
    seats: Seats,
}

impl Default for PatternTool {
    fn default() -> Self {
        Self::new()
    }
}

impl PatternTool {
    /// A tool holding nothing.
    pub const fn new() -> Self {
        Self {
            seats: Seats::new([Seat::PatternBody, Seat::PatternAxis]),
        }
    }

    /// The seats, roles and picks together — what the panel's line is
    /// composed from ([`crate::seats::seat_line`]).
    pub fn seats(&self) -> &Seats {
        &self.seats
    }

    /// The held body.
    pub fn input(&self) -> Option<RecipeNodeId> {
        self.seats.held(0)
    }

    /// The held axis, if one was picked.
    pub fn axis(&self) -> Option<RecipeNodeId> {
        self.seats.held(1)
    }

    /// Feed one node pick; `doc` routes it, and does not judge it.
    pub fn pick(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId) {
        self.seats.pick(doc, node);
    }

    /// Empty both seats.
    pub fn clear(&mut self) {
        self.seats.clear();
    }

    /// The survival step ([`crate::seats`]).
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<SeatEvent> {
        self.seats.reconcile(doc)
    }

    /// **The one committed edit**, stepping along a direction.
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] with no body picked. A count of zero or
    /// less is NOT judged here: the count is a document slot like any
    /// other, and a non-positive one refuses typed at evaluation on the
    /// node's own badge.
    pub fn linear_op(
        &self,
        output: PatternOutputChoice,
        count: i64,
        direction: [Expr; 3],
        spacing: Expr,
    ) -> Result<SessionOp, SeatError> {
        Ok(pattern_op(
            output,
            self.seats.require(0)?,
            count,
            PatternRuleSpec::Linear { direction, spacing },
        ))
    }

    /// **The one committed edit**, stepping around the picked axis.
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] with no body picked, or with no axis
    /// picked — which this door is the only one to need.
    pub fn circular_op(
        &self,
        output: PatternOutputChoice,
        count: i64,
        step: Expr,
    ) -> Result<SessionOp, SeatError> {
        // The BODY seat first, so an empty form names the pick a user
        // makes first rather than the one this rule adds.
        let input = self.seats.require(0)?;
        let rule = PatternRuleSpec::Circular {
            axis: self.seats.require(1)?,
            step,
        };
        Ok(pattern_op(output, input, count, rule))
    }
}

/// Which op a filled pattern form commits — the output choice's one
/// consequence, spelled once so the two rule doors cannot disagree
/// about it.
fn pattern_op(
    output: PatternOutputChoice,
    input: RecipeNodeId,
    count: i64,
    rule: PatternRuleSpec,
) -> SessionOp {
    match output {
        PatternOutputChoice::Instances => SessionOp::AddPattern { input, count, rule },
        PatternOutputChoice::Fused => SessionOp::AddPlacedUnion { input, count, rule },
    }
}

/// Lower one pattern spec to its node, placing the authored
/// expressions and minting the STRUCTURAL count.
///
/// The count is [`Expr::count`] — an exact integer — and not a
/// continuous literal, because `SlotId::Count` is Count-dimensioned and
/// the structural/continuous split is typed rather than emergent (spec
/// D3). That is the same reason it is authored as an `i64` all the way
/// from the form: a count that arrived as an `f64` would have to be
/// rounded somewhere, and every place that rounds it is a place the
/// number can differ from what the user typed.
///
/// Total: the continuous slots arrive as `Expr`s that were checked at
/// their own construction, and whether each one's DIMENSION suits the
/// slot it lands in is the edit door's question
/// (`EditError::SlotDimensionMismatch`), asked of authored and
/// hand-written documents alike.
pub fn pattern_node(
    input: RecipeNodeId,
    count: i64,
    rule: PatternRuleSpec,
) -> Node<ProfileProgram> {
    Node::Pattern {
        input,
        count: Expr::count(count),
        kind: rule_kind(rule),
    }
}

/// Lower one pattern spec to its FUSED node — the same prototype, the
/// same count and the same rule as [`pattern_node`], and one body out
/// instead of N.
///
/// The count slot is PRESENT, which is `Node::PlacedUnion`'s correct
/// spelling for a parametric rule: only `PatternKind::Explicit` brings
/// its own placements, and [`PatternRuleSpec`] cannot spell that rule.
/// The edit door re-checks the pairing on every insert
/// (`PlacementRuleFault::CountSpelling`), so a future rule that broke
/// this refuses typed at the commit rather than landing.
///
/// Total, for the reason [`pattern_node`] is: slot dimensions are the
/// edit door's question. Whether the placements are DISJOINT is not
/// asked here either — that certificate is evaluation's, reported on
/// the node's own badge.
pub fn placed_union_node(
    input: RecipeNodeId,
    count: i64,
    rule: PatternRuleSpec,
) -> Node<ProfileProgram> {
    Node::PlacedUnion {
        input,
        count: Some(Expr::count(count)),
        kind: rule_kind(rule),
    }
}

/// The rule vocabulary the two pattern nodes SHARE, lowered once: a
/// spec is a `PatternKind`, whichever node is about to carry it.
fn rule_kind(rule: PatternRuleSpec) -> PatternKind {
    match rule {
        PatternRuleSpec::Linear { direction, spacing } => {
            PatternKind::Linear { direction, spacing }
        }
        PatternRuleSpec::Circular { axis, step } => PatternKind::Circular { axis, step },
    }
}

/// Lower one rigid placement to its node, placing the authored
/// expressions in the [`Node::Transform`] slots (translation Length,
/// rotation axis Scalar, rotation angle Angle).
///
/// Total, for the reason [`pattern_node`] is: slot dimensions are the
/// edit door's question.
pub fn transform_node(
    input: RecipeNodeId,
    translation: [Expr; 3],
    rotation_axis: [Expr; 3],
    rotation_angle: Expr,
) -> Node<ProfileProgram> {
    Node::Transform {
        input,
        translation,
        rotation_axis,
        rotation_angle,
    }
}

/// **The part tool**: one pick of a multi-body value, committing one
/// [`SessionOp::AddPart`].
///
/// **Two seats for one pick**, which is the seat vocabulary's routing
/// rule doing the work rather than a second gate: a half is read out
/// of a `Node::Split` and an index out of a `Node::Pattern`, so the
/// two selections want different KINDS. A user clicks the thing they
/// mean and [`Seats::pick`] puts it in the seat only it can fill; the
/// form's selector then picks the commit door, exactly as the pattern
/// form's rule choice does. The alternative — one seat admitting
/// either, and the half-against-a-pattern pairing checked somewhere
/// below — would be a second authority on a question the seat already
/// answers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartTool {
    seats: Seats,
}

impl Default for PartTool {
    fn default() -> Self {
        Self::new()
    }
}

impl PartTool {
    /// A tool holding nothing.
    pub const fn new() -> Self {
        Self {
            seats: Seats::new([Seat::PartSplit, Seat::PartInstance]),
        }
    }

    /// The seats, roles and picks together — what the panel's line is
    /// composed from ([`crate::seats::seat_line`]).
    pub fn seats(&self) -> &Seats {
        &self.seats
    }

    /// The held split, if one was picked.
    pub fn split(&self) -> Option<RecipeNodeId> {
        self.seats.held(0)
    }

    /// The held pattern, if one was picked.
    pub fn pattern(&self) -> Option<RecipeNodeId> {
        self.seats.held(1)
    }

    /// Feed one node pick; `doc` routes it, and does not judge it.
    pub fn pick(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId) {
        self.seats.pick(doc, node);
    }

    /// Empty both seats.
    pub fn clear(&mut self) {
        self.seats.clear();
    }

    /// The survival step ([`crate::seats`]).
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<SeatEvent> {
        self.seats.reconcile(doc)
    }

    /// **The one committed edit**, selecting a split's named half.
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] until a split is picked.
    pub fn half_op(&self, half: SplitHalf) -> Result<SessionOp, SeatError> {
        Ok(SessionOp::AddPart {
            of: self.seats.require(0)?,
            select: PartSelectSpec::SplitHalf(half),
        })
    }

    /// **The one committed edit**, selecting one instance of a
    /// pattern.
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] until a pattern is picked. An index
    /// outside the pattern's instances is NOT judged here: it is a
    /// fact about the pattern's VALUE, and one past the end refuses
    /// typed at evaluation on the node's own badge
    /// (`NodeErrorKind::InstanceOutOfRange`) — the division of labour
    /// a non-positive pattern count already takes.
    pub fn instance_op(&self, index: i64) -> Result<SessionOp, SeatError> {
        Ok(SessionOp::AddPart {
            of: self.seats.require(1)?,
            select: PartSelectSpec::Instance(index),
        })
    }
}

/// **The duplicate tool**: one body pick, committing one
/// [`SessionOp::Duplicate`].
///
/// One seat and no fields: the step after duplicating is to MOVE the
/// copy away, so a form asking where the copy should go first would be
/// the pattern form again under another name. Where the copy lands is
/// [`duplicate_step`]'s rule — along [`STEP_DIRECTION`], clear of the
/// original by at least [`DUPLICATE_GAP`] of its own width — and both
/// numbers
/// land in ordinary slots of the pattern node the gesture authors,
/// editable in the property panel the moment the edit lands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DuplicateTool {
    seats: Seats,
}

impl Default for DuplicateTool {
    fn default() -> Self {
        Self::new()
    }
}

impl DuplicateTool {
    /// A tool holding nothing.
    pub const fn new() -> Self {
        Self {
            seats: Seats::one(Seat::DuplicateBody),
        }
    }

    /// The seats, roles and picks together — what the panel's line is
    /// composed from ([`crate::seats::seat_line`]).
    pub fn seats(&self) -> &Seats {
        &self.seats
    }

    /// The held body.
    pub fn input(&self) -> Option<RecipeNodeId> {
        self.seats.held(0)
    }

    /// Feed one node pick — a second pick REPLACES the first, this
    /// tool having only the one seat. `doc` routes it, and does not
    /// judge it.
    pub fn pick(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId) {
        self.seats.pick(doc, node);
    }

    /// Empty the seat.
    pub fn clear(&mut self) {
        self.seats.clear();
    }

    /// The survival step ([`crate::seats`]).
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<SeatEvent> {
        self.seats.reconcile(doc)
    }

    /// **The one committed edit**.
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] until a body is picked.
    pub fn op(&self) -> Result<SessionOp, SeatError> {
        Ok(SessionOp::Duplicate {
            input: self.seats.require(0)?,
        })
    }
}

/// **Which way a stepped copy goes by default**: world +x,
/// unnormalized as every direction slot in this vocabulary is.
///
/// ONE home for the two gestures that step a copy along a line: the
/// duplicate tool commits it, and the pattern form's direction field
/// opens on it (`Drafts::default`). The pattern form's SPACING is not
/// shared, because the two answer different questions — a pattern's
/// spacing is a number the person types, a duplicate's is measured off
/// the body ([`duplicate_step`]).
pub const STEP_DIRECTION: [f64; 3] = [1.0, 0.0, 0.0];

/// **How much clear space a duplicate leaves AT LEAST**, as a fraction
/// of the body's own width along [`STEP_DIRECTION`]: the copy's near
/// side lands at least this far past the original's far side, and
/// further for a body thin along the step ([`duplicate_step`] says
/// why).
///
/// A fraction rather than a length, so the gap scales with the part —
/// a millimetre gap beside a metre-long beam is invisible, and beside
/// a millimetre pin it is the whole view.
pub const DUPLICATE_GAP: f64 = 0.25;

/// The measuring tessellation's chord, as a fraction of the body's
/// FLOOR-mesh extent (`crate::scene::SCALE_PROBE_DELTA`'s mesh).
///
/// Fine enough that the conservative inflation it costs — twice the
/// chord, on the width — stays well under [`DUPLICATE_GAP`] for a body
/// about as wide along the step as it is across; coarse enough that
/// measuring a body is one cheap tessellation rather than a picture's
/// worth. For a body thin along the step the inflation is NOT small
/// against the width, and the copy lands further off than the gap
/// alone would put it ([`duplicate_step`]).
///
/// Public for [`crate::scene::SCALE_PROBE_DELTA`]'s reason: a row that
/// asserts a landed step against a body's closed form has to run the
/// rule at this number, and a literal copy in a suite goes stale
/// without the build noticing.
pub const MEASURE_CHORD: f64 = 1.0 / 64.0;

/// **Why a duplicate could not be placed** — every way the landed body
/// can fail to be one thing with a width.
#[derive(Debug)]
pub enum DuplicateFault {
    /// Nothing has landed, so there is no body to measure yet.
    NotLanded,
    /// The picture on screen answers an OLDER document than the one
    /// the duplicate would be committed to — an edit has not landed
    /// yet ([`crate::session::DocSession::busy`]). The landed value of
    /// an edited node is its value BEFORE the edit, so a step measured
    /// off it could be the old width, and after a document replacement
    /// the same id may name a different node altogether (issue #1384).
    Stale,
    /// The picture on screen — which answers the current document —
    /// holds no value for the input: its evaluation failed, or was
    /// poisoned by a failure upstream.
    NoValue {
        /// The node picked.
        input: RecipeNodeId,
    },
    /// The input's VALUE is several bodies. A pattern of two over it
    /// would index the flat list of those bodies, so its two
    /// projections would select two of the ORIGINAL bodies in place and
    /// the gesture would add nothing to the picture.
    ///
    /// Reachable only past the body seat's own gate, which classifies
    /// by node kind and admits a transform of a pattern
    /// (`work/forms/body-seat-reads-through-the-placer-chain`); this
    /// door asks the value, which is the evaluator's own question.
    NotOneBody {
        /// The node picked.
        input: RecipeNodeId,
    },
    /// The measuring tessellation refused.
    Unmeasured {
        /// The node picked.
        input: RecipeNodeId,
        /// The tessellator's own refusal.
        error: pncad::mesh::TessellateError,
    },
    /// The body's mesh has no extent to step by.
    NoExtent {
        /// The node picked.
        input: RecipeNodeId,
    },
}

impl core::fmt::Display for DuplicateFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotLanded => f.write_str(
                "the document has not evaluated yet, so there is no body to measure a copy's \
                 step off",
            ),
            Self::Stale => f.write_str(
                "the picture is older than the document — wait for the latest edit to evaluate, \
                 so the copy's step is measured off the body as it now is",
            ),
            Self::NoValue { input } => write!(
                f,
                "feature {} did not evaluate, so there is no body to copy",
                input.0
            ),
            Self::NotOneBody { input } => write!(
                f,
                "feature {}'s value is several bodies; a duplicate copies ONE — project the one \
                 you mean first",
                input.0
            ),
            Self::Unmeasured { input, error } => write!(
                f,
                "feature {}'s body could not be measured for the copy's step: {error}",
                input.0
            ),
            Self::NoExtent { input } => write!(
                f,
                "feature {}'s body has no width to step a copy by",
                input.0
            ),
        }
    }
}

impl core::error::Error for DuplicateFault {}

/// **Where a duplicate's copy lands**: the step, in metres along
/// [`STEP_DIRECTION`], that puts the copy's near side AT LEAST
/// [`DUPLICATE_GAP`] of the body's width past the original's far side —
/// so the two never touch, whatever the body's size.
///
/// **Read off `eval`, which the caller must hand in CURRENT** — an
/// evaluation of the document the duplicate will be committed to. The
/// session door refuses [`DuplicateFault::NotLanded`] and
/// [`DuplicateFault::Stale`] before calling this; a stale evaluation
/// would hand back an edited node's OLD value, and nothing here could
/// tell.
///
/// **Conservative, and why.** The width is measured on a tessellation
/// at chord δ. The step needs every point of the exact surfaces to lie
/// within a known distance of the mesh, and `pncad::mesh`'s crate docs
/// state the promise the other way round — each triangle within δ of
/// the surface — with an honest bound of δ + ε (+ rounding), ε being
/// the kernel tolerance its boundary vertices sit within. The direction
/// this needs holds too, and rests on the same facts: the mesh vertices
/// lie on the surfaces (within ε), and each face kind's certificate
/// bounds the gap between the surface and the linear interpolant
/// through them over the triangle's own parameter cell, which runs both
/// ways, while the cells cover the face. So the true width is at most
/// the mesh's plus 2(δ + ε), and the step adds that before the gap;
/// rounding is left to the gap, which exceeds it by many orders.
///
/// **So the gap is a floor, not the step's exact share.** δ is
/// [`MEASURE_CHORD`] of the body's floor-mesh DIAGONAL, which sets the
/// scale before any width is known; for a body thin along
/// [`STEP_DIRECTION`] the 2(δ + ε) margin can be several times the
/// width, and the copy lands further away than a quarter-width. Never
/// nearer.
///
/// # Errors
///
/// [`DuplicateFault::NoValue`], [`DuplicateFault::NotOneBody`],
/// [`DuplicateFault::Unmeasured`], [`DuplicateFault::NoExtent`].
pub fn duplicate_step(
    eval: &Evaluation<f64>,
    input: RecipeNodeId,
    tol: Tol,
) -> Result<f64, DuplicateFault> {
    let value = eval.value(input).ok_or(DuplicateFault::NoValue { input })?;
    let body = one_body(&value.payload).ok_or(DuplicateFault::NotOneBody { input })?;
    let measured = |chord: f64| {
        pncad::mesh::tessellate(body, chord, tol)
            .map_err(|error| DuplicateFault::Unmeasured { input, error })
    };
    let floor = measured(crate::scene::SCALE_PROBE_DELTA)?;
    // The floor mesh's box diagonal: the body's size, read before a
    // chord can be chosen for it.
    let [wx, wy, wz] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
        .map(|axis| width_along(&floor.positions, axis).unwrap_or(0.0));
    let scale = Vec3::new(wx, wy, wz).norm();
    if !(scale.is_finite() && scale > 0.0) {
        return Err(DuplicateFault::NoExtent { input });
    }
    let chord = scale * MEASURE_CHORD;
    let mesh = measured(chord)?;
    let width = width_along(&mesh.positions, STEP_DIRECTION)
        .filter(|w| w.is_finite() && *w > 0.0)
        .ok_or(DuplicateFault::NoExtent { input })?;
    Ok((width + 2.0 * (chord + tol.eps())) * (1.0 + DUPLICATE_GAP))
}

/// How far `points` spread along `direction` (normalized here): the
/// largest projection less the smallest. `None` for no points or a
/// zero direction.
fn width_along(points: &[pncad::geom_core::Point3<f64>], direction: [f64; 3]) -> Option<f64> {
    let [dx, dy, dz] = direction;
    let direction = Vec3::new(dx, dy, dz);
    let norm = direction.norm();
    if !(norm.is_finite() && norm > 0.0) {
        return None;
    }
    let mut along = points
        .iter()
        .map(|p| Vec3::new(p.x, p.y, p.z).dot(direction) / norm);
    let first = along.next()?;
    let (lo, hi) = along.fold((first, first), |(lo, hi), r| (lo.min(r), hi.max(r)));
    Some(hi - lo)
}

/// The pattern rule a duplicate commits: [`STEP_DIRECTION`] stepped by
/// `step` metres ([`duplicate_step`]'s answer).
///
/// # Errors
///
/// [`pncad::document::DimensionError`] for a non-finite `step` — which
/// [`duplicate_step`] never answers — and it is a `Result` for
/// [`crate::session::ProfilePlane::world_xy`]'s reason: whether a
/// number is authorable keeps ONE home, the expression door.
pub fn duplicate_rule(step: f64) -> Result<PatternRuleSpec, pncad::document::DimensionError> {
    use pncad::document::Dimension;
    let scalar = |v: f64| Expr::literal(v, Dimension::Scalar);
    let [x, y, z] = STEP_DIRECTION;
    Ok(PatternRuleSpec::Linear {
        direction: [scalar(x)?, scalar(y)?, scalar(z)?],
        spacing: Expr::literal(step, Dimension::Length)?,
    })
}

/// **How many bodies a duplicate leaves**: the original and one copy.
///
/// The pattern's count, and the range the same action's projections
/// are generated over (`0..DUPLICATE_COUNT` at the session door) — one
/// number, so a pattern and its projections cannot disagree about how
/// many bodies there are.
pub const DUPLICATE_COUNT: i64 = 2;

/// Lower one part spec to its node, minting the STRUCTURAL index.
///
/// The index is [`Expr::count`] — an exact integer — for the reason
/// [`pattern_node`]'s count is: `SlotId::Instance` is Count-dimensioned
/// and the structural/continuous split is typed rather than emergent
/// (spec D3).
///
/// Total, for [`pattern_node`]'s reason: whether the selection suits
/// the value it reads is evaluation's question, asked of authored and
/// hand-written documents alike.
pub fn part_node(of: RecipeNodeId, select: PartSelectSpec) -> Node<ProfileProgram> {
    Node::Part {
        of,
        select: match select {
            PartSelectSpec::SplitHalf(half) => PartSelect::SplitHalf(half),
            PartSelectSpec::Instance(index) => PartSelect::Instance(Expr::count(index)),
        },
    }
}

/// **Whether a node's value is a single body** — the question every
/// body seat asks, answered off the node vocabulary alone.
///
/// The rule this tracks is the evaluator's single-body OPERAND door
/// (`eval::wire::body_operand`): a `Body` payload, or a boolean's
/// non-empty result. A split's two sides and a pattern's instances are
/// the cases that matter — each is SEVERAL bodies, so a seat filled
/// with one refuses at the door rather than after the edit lands. The
/// recipe's way of saying which of them is meant is [`Node::Part`]
/// (`crates/editor-core/REFERENCES.md` DM3): a projection of one half
/// or one
/// instance, which evaluates to ONE `Body` value and is admitted here
/// for exactly that reason. "Union the upper half of that split" is
/// therefore a Part of the split at a boolean seat; the door that
/// authors one is CHROME's.
///
/// **Tracks, and is not equal to, in two named directions.** A node
/// this admits may still refuse downstream — an empty boolean result is
/// a typed success that is not a body — and `Sweep` is admitted here
/// while `wire_sweep` refuses every recipe-expressible sweep today: it
/// is the curved-solid frontier, and a seat that refused it would
/// answer "that is not a body" to a node that is one in every sense but
/// the one the kernel has not reached. Its own frontier refusal is the
/// honest diagnosis, and it arrives by poison propagation. Neither
/// direction is left to prose: `combine_ops::
/// the_body_seat_tracks_the_evaluators_operand_door` drives each
/// admitted kind into the real door and asserts the exception by name.
///
/// **Not `product`'s "body-denoting"**, which is a WIDER set: the
/// product gather counts a pattern's instances and a split's sides
/// among the bodies it collects, because collecting several is what it
/// does. This answers the narrower question a single-body operand seat
/// asks.
///
/// The match is exhaustive on purpose: a new node variant does not
/// compile until someone decides which side of this line it is on.
pub fn denotes_body(node: &Node<ProfileProgram>) -> bool {
    match node {
        Node::Extrude { .. }
        | Node::Revolve { .. }
        // Both tube kinds denote ONE body, hollow or not: the hollow
        // one's cavity is a void inside a single solid, not a second
        // body, so the seat admits them for the same reason it admits
        // a revolve.
        | Node::Tube { .. }
        | Node::HollowTube { .. }
        | Node::Loft { .. }
        | Node::Sweep { .. }
        | Node::Fillet { .. }
        | Node::Chamfer { .. }
        // A thin solid is ONE body: the cavity is a void inside it,
        // exactly as the hollow tube's is.
        | Node::Shell { .. }
        | Node::Boolean { .. }
        // ONE body out, exactly as the pair union it generalizes: the
        // members are folded, not collected.
        | Node::Union { .. }
        | Node::Transform { .. }
        // ONE body out of a split's or a pattern's value — the
        // projection is what makes one of several bodies a body.
        | Node::Part { .. }
        | Node::PlacedUnion { .. }
        | Node::InstantiatePart { .. } => true,
        Node::Datum(_)
        | Node::Profile(_)
        | Node::Split { .. }
        | Node::Pattern { .. }
        | Node::Declare { .. }
        | Node::Mate { .. }
        | Node::Measure { .. }
        | Node::Assertion { .. } => false,
    }
}
