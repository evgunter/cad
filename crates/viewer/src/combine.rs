//! **Combining bodies**: the modal layer-3 tools that turn body picks
//! into exactly one committed combining edit, and the lowering their
//! ops go through (GAUTH-4, Phase B).
//!
//! # Shape
//!
//! The revolve tool's shape, four times over, on the shared seat
//! machinery [`crate::seats`] carries: single-select stays ruled, so
//! each tool holds its picks in tool state and consumes the ordinary
//! selection stream — a tree click is a node pick directly, a viewport
//! face or edge pick reaches the FEATURE it belongs to
//! (`Selection::node`, the one viewport→tree inversion). Everything
//! before the commit is tool state; the document transition is one
//! [`SessionOp`], which commits one `DocEdit::InsertNode` through the
//! session's ordinary commit door.
//!
//! The seat vocabulary, the pick rule, the survival step and the
//! id-reuse hazard it does not cover (issue #1384) are all
//! [`crate::seats`]'s, and are not restated here.

use pncad::document::{
    BooleanOp, Dimension, DimensionError, Doc, Expr, Node, PatternKind, ProfileProgram,
    RecipeNodeId,
};

use crate::seats::{Seat, SeatError, SeatEvent, Seats};
use crate::session::{PatternRuleSpec, SessionOp};

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

    /// The held first operand — the body a subtraction KEEPS.
    pub fn a(&self) -> Option<RecipeNodeId> {
        self.seats.held(0)
    }

    /// The held second operand — the body a subtraction REMOVES.
    pub fn b(&self) -> Option<RecipeNodeId> {
        self.seats.held(1)
    }

    /// Feed one node pick.
    pub fn pick(&mut self, node: RecipeNodeId) {
        self.seats.pick(node);
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

    /// The held target body.
    pub fn target(&self) -> Option<RecipeNodeId> {
        self.seats.held(0)
    }

    /// The held cutting plane.
    pub fn plane(&self) -> Option<RecipeNodeId> {
        self.seats.held(1)
    }

    /// Feed one node pick.
    pub fn pick(&mut self, node: RecipeNodeId) {
        self.seats.pick(node);
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

    /// The held body.
    pub fn input(&self) -> Option<RecipeNodeId> {
        self.seats.held(0)
    }

    /// Feed one node pick — a second pick REPLACES the first, this tool
    /// having only the one seat.
    pub fn pick(&mut self, node: RecipeNodeId) {
        self.seats.pick(node);
    }

    /// Empty the seat.
    pub fn clear(&mut self) {
        self.seats.clear();
    }

    /// The survival step ([`crate::seats`]).
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<SeatEvent> {
        self.seats.reconcile(doc)
    }

    /// **The one committed edit**: the rigid placement, in metres and
    /// radians.
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] until a body is picked.
    pub fn op(
        &self,
        translation: [f64; 3],
        rotation_axis: [f64; 3],
        rotation_angle: f64,
    ) -> Result<SessionOp, SeatError> {
        Ok(SessionOp::AddTransform {
            input: self.seats.require(0)?,
            translation,
            rotation_axis,
            rotation_angle,
        })
    }
}

/// **The pattern tool**: a body pick, and — for the circular rule
/// only — a datum-axis pick, committing one [`SessionOp::AddPattern`].
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

    /// The held body.
    pub fn input(&self) -> Option<RecipeNodeId> {
        self.seats.held(0)
    }

    /// The held axis, if one was picked.
    pub fn axis(&self) -> Option<RecipeNodeId> {
        self.seats.held(1)
    }

    /// Feed one node pick.
    pub fn pick(&mut self, node: RecipeNodeId) {
        self.seats.pick(node);
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
        count: i64,
        direction: [f64; 3],
        spacing: f64,
    ) -> Result<SessionOp, SeatError> {
        Ok(SessionOp::AddPattern {
            input: self.seats.require(0)?,
            count,
            rule: PatternRuleSpec::Linear { direction, spacing },
        })
    }

    /// **The one committed edit**, stepping around the picked axis.
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] with no body picked, or with no axis
    /// picked — which this door is the only one to need.
    pub fn circular_op(&self, count: i64, step: f64) -> Result<SessionOp, SeatError> {
        Ok(SessionOp::AddPattern {
            input: self.seats.require(0)?,
            count,
            rule: PatternRuleSpec::Circular {
                axis: self.seats.require(1)?,
                step,
            },
        })
    }
}

/// Lower one pattern spec to its node, minting the literal slots and
/// the STRUCTURAL count.
///
/// The count is [`Expr::count`] — an exact integer — and not a
/// continuous literal, because `SlotId::Count` is Count-dimensioned and
/// the structural/continuous split is typed rather than emergent (spec
/// D3). That is the same reason it is authored as an `i64` all the way
/// from the form: a count that arrived as an `f64` would have to be
/// rounded somewhere, and every place that rounds it is a place the
/// number can differ from what the user typed.
///
/// # Errors
///
/// A non-finite continuous component (the literal door's refusal).
pub fn pattern_node(
    input: RecipeNodeId,
    count: i64,
    rule: PatternRuleSpec,
) -> Result<Node<ProfileProgram>, DimensionError> {
    let kind = match rule {
        PatternRuleSpec::Linear { direction, spacing } => PatternKind::Linear {
            direction: scalars(direction)?,
            spacing: Expr::literal(spacing, Dimension::Length)?,
        },
        PatternRuleSpec::Circular { axis, step } => PatternKind::Circular {
            axis,
            step: Expr::literal(step, Dimension::Angle)?,
        },
    };
    Ok(Node::Pattern {
        input,
        count: Expr::count(count),
        kind,
    })
}

/// Lower one rigid placement to its node, minting the literal slots —
/// translation Length, rotation axis Scalar, rotation angle Angle (the
/// [`Node::Transform`] slot dimensions).
///
/// # Errors
///
/// A non-finite component (the literal door's refusal).
pub fn transform_node(
    input: RecipeNodeId,
    translation: [f64; 3],
    rotation_axis: [f64; 3],
    rotation_angle: f64,
) -> Result<Node<ProfileProgram>, DimensionError> {
    Ok(Node::Transform {
        input,
        translation: [
            Expr::literal(translation[0], Dimension::Length)?,
            Expr::literal(translation[1], Dimension::Length)?,
            Expr::literal(translation[2], Dimension::Length)?,
        ],
        rotation_axis: scalars(rotation_axis)?,
        rotation_angle: Expr::literal(rotation_angle, Dimension::Angle)?,
    })
}

/// Three dimensionless literals — a direction or a rotation axis.
fn scalars(v: [f64; 3]) -> Result<[Expr; 3], DimensionError> {
    Ok([
        Expr::literal(v[0], Dimension::Scalar)?,
        Expr::literal(v[1], Dimension::Scalar)?,
        Expr::literal(v[2], Dimension::Scalar)?,
    ])
}

/// **Whether a node's value is a single body** — the question every
/// body seat asks, answered off the node vocabulary alone.
///
/// The rule this tracks is the evaluator's single-body OPERAND door
/// (`eval::wire::body_operand`): a `Body` payload, or a boolean's
/// non-empty result. A split's two sides and a pattern's instances are
/// the cases that matter — each is SEVERAL bodies, and selecting one of
/// them needs a vocabulary the recipe does not yet have, so a seat
/// filled with one refuses at the door rather than after the edit
/// lands. That the sentences those seats would spell ("union the upper
/// half of that split") are ordinary ones is issue #1394, which widens
/// at this function when the operand vocabulary answers it.
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
        | Node::Loft { .. }
        | Node::Sweep { .. }
        | Node::Fillet { .. }
        | Node::Chamfer { .. }
        | Node::Boolean { .. }
        | Node::Transform { .. }
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
