//! **Combining bodies**: the modal layer-3 tools that turn body picks
//! into exactly one committed combining edit, and the lowering their
//! ops go through (GAUTH-4, Phase B).
//!
//! # Shape
//!
//! The revolve tool's shape, four times over: single-select stays
//! ruled, so each tool holds its picks in tool state and consumes the
//! ordinary selection stream — a tree click is a node pick directly, a
//! viewport face pick reaches the FEATURE the face belongs to
//! (`Selection::node`, the one viewport→tree inversion). Everything
//! before the commit is tool state; the document transition is one
//! [`SessionOp`], which commits one `DocEdit::InsertNode` through the
//! session's ordinary commit door.
//!
//! # The picks are ROLES
//!
//! Every seat here means a particular thing, and the pair is never
//! symmetric — not even the boolean's, whose two operands are the
//! difference between `A ∖ B` and `B ∖ A`. So a tool fills its seats
//! in order, a further pick replaces the LAST seat, and the panel
//! names which held pick is which. [`Seat`] is the whole vocabulary of
//! roles, so a lost pick reads as a sentence about a role rather than
//! about a slot number.
//!
//! Node KINDS are not judged here — that is the session door's job
//! ([`crate::session::Refusal::WrongNodeKind`], one arm for every
//! creation seat), so a wrong-kind pick refuses typed at commit rather
//! than being silently ignored at pick time, and the rule lives in one
//! place.
//!
//! # Survival
//!
//! Tool state survives a held node vanishing (the ratified
//! resolution-failure semantics at node scope): each tool's
//! `reconcile` re-reads its seats against the document and drops a
//! pick whose node is gone, each drop a typed [`CombineToolEvent`] the
//! chrome renders. A drop does NOT promote a surviving pick into the
//! empty seat — the seats are roles, and the next pick refills
//! whichever is empty, in seat order. That is the revolve tool's
//! divergence from the mate tool's interchangeable pair, for the same
//! reason.
//!
//! `reconcile` is the consumer's obligation: the application calls it
//! once per frame, and a consumer that forgets it is not reliably
//! caught later. These tools inherit the revolve tool's id-reuse
//! hazard verbatim — a `RecipeNodeId` is a small per-document counter,
//! so once the document is REPLACED under held picks (a
//! `NewDocument`, an open, an undo past the picks' inserts) fresh
//! inserts re-mint the same small ids and the stale picks silently
//! denote the NEW nodes. Per-frame reconcile guards the deleted-node
//! case only; the aliasing is the class hazard tracked as issue #1384.

use pncad::document::{
    BooleanOp, Dimension, DimensionError, Doc, Expr, Node, PatternKind, ProfileProgram,
    RecipeNodeId,
};

use crate::session::SessionOp;

/// Which seat of a combining tool a pick (or a drop) is about — one
/// vocabulary of roles for all four tools, so every sentence about a
/// held pick is composed the same way.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Seat {
    /// The boolean's first operand — the body `A ∖ B` KEEPS.
    OperandA,
    /// The boolean's second operand — the body `A ∖ B` REMOVES.
    OperandB,
    /// The body a split cuts.
    SplitTarget,
    /// The datum plane a split cuts with.
    SplitPlane,
    /// The body a transform places.
    TransformBody,
    /// The body a pattern replicates.
    PatternBody,
    /// The datum axis a circular pattern steps around.
    PatternAxis,
}

impl Seat {
    /// The seat's name, for sentences.
    pub fn name(self) -> &'static str {
        match self {
            Self::OperandA => "first operand",
            Self::OperandB => "second operand",
            Self::SplitTarget => "split target",
            Self::SplitPlane => "split plane",
            Self::TransformBody => "transformed body",
            Self::PatternBody => "patterned body",
            Self::PatternAxis => "pattern axis",
        }
    }
}

/// **The one sentence a tool panel shows for its held picks**, so the
/// seats read the same way in every panel and a reader can tell which
/// pick is in which role — the fact that decides what a subtraction
/// removes.
///
/// Composed here rather than in the widgets because it is the same
/// vocabulary a lost-pick notice is composed from, and two copies is
/// how the two drift.
pub fn seat_line(seats: &[(Seat, Option<RecipeNodeId>)]) -> String {
    if seats.iter().all(|(_, held)| held.is_none()) {
        return "no picks yet".to_owned();
    }
    seats
        .iter()
        .map(|(seat, held)| match held {
            Some(node) => format!("{}: feature {}", seat.name(), node.0),
            None => format!("{}: —", seat.name()),
        })
        .collect::<Vec<_>>()
        .join("; ")
}

/// A typed combining-tool refusal (closed enum, D4 ¶3).
#[derive(Debug)]
pub enum CombineToolError {
    /// A seat the commit needs is still empty.
    SeatEmpty {
        /// Which one.
        seat: Seat,
    },
}

impl core::fmt::Display for CombineToolError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SeatEmpty { seat } => write!(f, "no {} picked yet", seat.name()),
        }
    }
}

impl core::error::Error for CombineToolError {}

/// A typed tool event the chrome renders — every state change that was
/// not the direct echo of an op.
#[derive(Debug, PartialEq, Eq)]
pub enum CombineToolEvent {
    /// A held pick's node is no longer in the document; the tool
    /// dropped it and the seat is open again.
    PickLost {
        /// Which seat was emptied.
        seat: Seat,
        /// The node that was held.
        node: RecipeNodeId,
    },
}

impl core::fmt::Display for CombineToolEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PickLost { seat, node } => write!(
                f,
                "the {} pick (node {}) is no longer in the document; the tool dropped it",
                seat.name(),
                node.0
            ),
        }
    }
}

/// Two role-typed seats filled in order — the state every tool here
/// holds, so the pick rule, the survival step and the clear door are
/// written once.
///
/// The roles travel WITH the seats rather than being re-supplied per
/// call: a drop's event has to name the role, and a value that knew
/// its picks but not what they were for could not compose that
/// sentence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Seats {
    roles: [Seat; 2],
    held: [Option<RecipeNodeId>; 2],
}

impl Seats {
    const fn new(roles: [Seat; 2]) -> Self {
        Self {
            roles,
            held: [None, None],
        }
    }

    /// Fill the first empty seat; with both full, REPLACE the second
    /// (the module docs' pick rule).
    fn pick(&mut self, node: RecipeNodeId) {
        if self.held[0].is_none() {
            self.held[0] = Some(node);
        } else {
            self.held[1] = Some(node);
        }
    }

    fn clear(&mut self) {
        self.held = [None, None];
    }

    /// Re-read the held picks against the document, dropping any whose
    /// node is gone.
    fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<CombineToolEvent> {
        let mut events = Vec::new();
        for (i, held) in self.held.iter_mut().enumerate() {
            if let Some(node) = *held
                && doc.node(node).is_none()
            {
                *held = None;
                events.push(CombineToolEvent::PickLost {
                    seat: self.roles[i],
                    node,
                });
            }
        }
        events
    }

    /// The seat's pick, or the typed "still empty" refusal naming it.
    fn require(&self, i: usize) -> Result<RecipeNodeId, CombineToolError> {
        self.held[i].ok_or(CombineToolError::SeatEmpty {
            seat: self.roles[i],
        })
    }
}

/// **The boolean tool**: two sequential body picks and one operation
/// choice, committing one [`SessionOp::AddBoolean`].
///
/// The operand order is DATA, not a convenience: `Subtract` keeps the
/// first pick and removes the second, so the panel says which held
/// pick is which and the seats are named for it.
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
        self.seats.held[0]
    }

    /// The held second operand — the body a subtraction REMOVES.
    pub fn b(&self) -> Option<RecipeNodeId> {
        self.seats.held[1]
    }

    /// Feed one node pick.
    pub fn pick(&mut self, node: RecipeNodeId) {
        self.seats.pick(node);
    }

    /// Empty both seats.
    pub fn clear(&mut self) {
        self.seats.clear();
    }

    /// The survival step (module docs).
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<CombineToolEvent> {
        self.seats.reconcile(doc)
    }

    /// **The one committed edit**: the session op that inserts the
    /// boolean node through the ordinary commit door.
    ///
    /// # Errors
    ///
    /// [`CombineToolError::SeatEmpty`] until both operands are picked.
    /// Node kinds, and the two-operands-are-one-node case, refuse at
    /// the session door (module docs).
    pub fn op(&self, op: BooleanOp) -> Result<SessionOp, CombineToolError> {
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
        self.seats.held[0]
    }

    /// The held cutting plane.
    pub fn plane(&self) -> Option<RecipeNodeId> {
        self.seats.held[1]
    }

    /// Feed one node pick.
    pub fn pick(&mut self, node: RecipeNodeId) {
        self.seats.pick(node);
    }

    /// Empty both seats.
    pub fn clear(&mut self) {
        self.seats.clear();
    }

    /// The survival step (module docs).
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<CombineToolEvent> {
        self.seats.reconcile(doc)
    }

    /// **The one committed edit**.
    ///
    /// # Errors
    ///
    /// [`CombineToolError::SeatEmpty`] until both seats are filled.
    pub fn op(&self) -> Result<SessionOp, CombineToolError> {
        Ok(SessionOp::AddSplit {
            target: self.seats.require(0)?,
            tool: self.seats.require(1)?,
        })
    }
}

/// **The transform tool**: one body pick plus the placement fields,
/// committing one [`SessionOp::AddTransform`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TransformTool {
    input: Option<RecipeNodeId>,
}

impl TransformTool {
    /// A tool holding nothing.
    pub const fn new() -> Self {
        Self { input: None }
    }

    /// The held body.
    pub fn input(&self) -> Option<RecipeNodeId> {
        self.input
    }

    /// Feed one node pick — a second pick REPLACES the first, this
    /// tool having only the one seat.
    pub fn pick(&mut self, node: RecipeNodeId) {
        self.input = Some(node);
    }

    /// Empty the seat.
    pub fn clear(&mut self) {
        self.input = None;
    }

    /// The survival step (module docs).
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<CombineToolEvent> {
        match self.input {
            Some(node) if doc.node(node).is_none() => {
                self.input = None;
                vec![CombineToolEvent::PickLost {
                    seat: Seat::TransformBody,
                    node,
                }]
            }
            _ => Vec::new(),
        }
    }

    /// **The one committed edit**: the rigid placement, in metres and
    /// radians.
    ///
    /// # Errors
    ///
    /// [`CombineToolError::SeatEmpty`] until a body is picked.
    pub fn op(
        &self,
        translation: [f64; 3],
        rotation_axis: [f64; 3],
        rotation_angle: f64,
    ) -> Result<SessionOp, CombineToolError> {
        Ok(SessionOp::AddTransform {
            input: self.input.ok_or(CombineToolError::SeatEmpty {
                seat: Seat::TransformBody,
            })?,
            translation,
            rotation_axis,
            rotation_angle,
        })
    }
}

/// The pattern form's rule choice, without the axis: the axis is a
/// PICK and lives in the tool's second seat, so the form carries only
/// what a form can carry.
///
/// `Explicit` is deliberately absent (the plan's ruling): a list of
/// absolute frames is not a form's job.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PatternForm {
    /// Stepped along a direction.
    Linear {
        /// Step direction components (Scalar).
        direction: [f64; 3],
        /// Distance between instances, metres.
        spacing: f64,
    },
    /// Stepped around the picked datum axis.
    Circular {
        /// Angular step between instances, radians.
        step: f64,
    },
}

/// **The pattern tool**: a body pick, a rule choice, and — for the
/// circular rule only — a datum-axis pick, committing one
/// [`SessionOp::AddPattern`].
///
/// The axis seat is filled by an ordinary second pick whichever rule
/// is chosen, and READ only by the circular commit: a user who picks
/// an axis and then chooses Linear is not corrected, because the pick
/// is not wrong until a commit needs it to mean something.
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
        self.seats.held[0]
    }

    /// The held axis, if one was picked.
    pub fn axis(&self) -> Option<RecipeNodeId> {
        self.seats.held[1]
    }

    /// Feed one node pick.
    pub fn pick(&mut self, node: RecipeNodeId) {
        self.seats.pick(node);
    }

    /// Empty both seats.
    pub fn clear(&mut self) {
        self.seats.clear();
    }

    /// The survival step (module docs).
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<CombineToolEvent> {
        self.seats.reconcile(doc)
    }

    /// **The one committed edit**, at `count` instances.
    ///
    /// # Errors
    ///
    /// [`CombineToolError::SeatEmpty`] with no body picked, or with no
    /// axis picked under the circular rule. A count of zero or less is
    /// NOT judged here: the count is a document slot like any other,
    /// and a non-positive one refuses typed at evaluation on the
    /// node's own badge.
    pub fn op(&self, count: i64, form: PatternForm) -> Result<SessionOp, CombineToolError> {
        let input = self.seats.require(0)?;
        let rule = match form {
            PatternForm::Linear { direction, spacing } => {
                PatternRuleSpec::Linear { direction, spacing }
            }
            PatternForm::Circular { step } => PatternRuleSpec::Circular {
                axis: self.seats.require(1)?,
                step,
            },
        };
        Ok(SessionOp::AddPattern { input, count, rule })
    }
}

/// The literal payload of one pattern form (the add-datum forms'
/// [`crate::session::DatumSpec`] one vocabulary over): plain numbers in
/// canonical units plus the axis pick, out of which the SESSION mints
/// the `Expr` slots.
///
/// `Explicit` has no arm here by the plan's ruling — the rule
/// vocabulary's non-parametric member is not something a form
/// authors.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PatternRuleSpec {
    /// Stepped along a direction (`PatternKind::Linear`).
    Linear {
        /// Step direction components, dimensionless.
        direction: [f64; 3],
        /// Distance between instances, metres.
        spacing: f64,
    },
    /// Stepped around a datum axis (`PatternKind::Circular`).
    Circular {
        /// The datum-axis node stepped around.
        axis: RecipeNodeId,
        /// Angular step between instances, radians.
        step: f64,
    },
}

/// Lower one pattern spec to its node, minting the literal slots and
/// the STRUCTURAL count.
///
/// The count is [`Expr::count`] — an exact integer — and not a
/// continuous literal, because `SlotId::Count` is Count-dimensioned
/// and the structural/continuous split is typed rather than emergent
/// (spec D3). That is the same reason it is authored as an `i64` all
/// the way from the form: a count that arrived as an `f64` would have
/// to be rounded somewhere, and every place that rounds it is a place
/// the number can differ from what the user typed.
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
/// The admissible set is exactly the one the evaluator's single-body
/// operand door takes, and that is the invariant to hold: a node this
/// answers `true` for may still refuse downstream (an empty boolean
/// result is a typed success that is not a body), but a node it
/// answers `false` for could never have been an operand at all. A
/// split's two sides and a pattern's instances are the cases that
/// matter — each is SEVERAL bodies, and selecting one of them needs a
/// vocabulary the recipe does not yet have, so a seat filled with one
/// refuses at the door rather than after the edit lands.
///
/// `Sweep` is on the true side though it evaluates to nothing today —
/// it is the curved-solid frontier, and a seat that refused it here
/// would answer "that is not a body" to a node that is one in every
/// sense but the one the kernel has not reached. Its own frontier
/// refusal is the honest diagnosis and it arrives by poison
/// propagation.
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
