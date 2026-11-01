//! **Role-typed pick seats**: the state every modal tool that consumes
//! node picks is built out of — its pick rule and survival step — and
//! the sentences a seated tool says about a held pick: the still-empty
//! refusal ([`SeatError`]), the drop notice ([`SeatEvent`]) and the
//! panel line ([`seat_line`]). The mate tool's panel line is composed
//! by the same function (`picks_line`); its drop notice and refusals
//! are its own (`crate::matetool`), and the blend tool's held-picks
//! line is drawn in its panel (`crate::pane`), not here.
//!
//! # Why one value and not one per tool
//!
//! The revolve tool and the combining tools differ in exactly two
//! things: how many seats they have and what those seats MEAN. Everything
//! else — fill the first empty seat, replace the last when both are
//! full, drop a pick whose node left the document, refuse typed until a
//! needed seat is filled — is one behaviour, and a second copy of it is
//! a second place for it to drift.
//!
//! A union's or an intersect's member list is not seats: it holds any
//! number of picks, each appended, so the boolean tool keeps it beside
//! its subtraction's [`Seats`] (`crate::combine::BooleanTool`) and says
//! its drops and its empty refusal in this module's words, naming
//! [`Seat::Member`].
//!
//! # The picks are ROLES
//!
//! A seat means a particular thing, and no pair here is symmetric — not
//! even a subtraction's two operands, whose order is the difference
//! between `A ∖ B` and `B ∖ A`. So seats fill in order, a further pick
//! replaces the LAST one, and a drop empties the seat it was in without
//! promoting the survivor: moving a node between seats that mean
//! different things is not a correction, it is a different edit.
//!
//! **The mate tool is the deliberate exception and stays outside this
//! module**: its two picks are an interchangeable pair (`a`/`b` of one
//! mate), so its survival step promotes the survivor into the one-pick
//! step — meaningful there, wrong here. Its picks are also faces rather
//! than nodes, so it shares neither the state nor the rule; the
//! divergence is stated in both module docs and neither is the other's
//! accident. **What it does share is the panel line**: a held pick is
//! still a role and what fills it, so the mate panel composes its line
//! through `picks_line` — the same empty-state sentence, item shape
//! and mark — and a picked node is called what [`seat_line`] calls it.
//!
//! # Kinds ROUTE a pick; they still do not judge one
//!
//! No tool decides whether a seat's contents are legal — that is the
//! session door's job ([`crate::session::Refusal::WrongNodeKind`], one
//! arm for every creation seat), so a wrong-kind pick still refuses
//! typed at the commit rather than being silently ignored at pick
//! time, and the verdict lives in one place.
//!
//! What the kinds do decide here is WHICH SEAT a pick lands in, where
//! a tool's two seats want different kinds: a pick the seat it would
//! land in cannot hold, and the other seat can, goes to the other seat
//! rather than displacing a pick that was fine. [`Seats::pick`] states
//! the rule; the kinds themselves come from [`Seat::wants`], and the
//! classification is the commit door's ([`crate::session::admits_read`]
//! of the read the pick makes), widened by one case: a split picked
//! alone may still become either half, so a body seat can hold it.
//!
//! # A seat holds a read
//!
//! A pick is a node and the output of it the seat reads (`Port`): the
//! node alone, one output of a node that defines several, or a split's
//! half by its port. A viewport pick names the output it hit; a tree
//! pick of a split names none, so a body seat holding it waits for its
//! half ([`Seats::choose_half`]) and its commit refuses
//! [`SeatError::HalfUnchosen`] until then.
//!
//! **What is shared is the classifier, not the table.** Which kind a
//! seat wants is stated here AND at each door's own `require_kind`
//! call, so the two can drift. What holds them together is a test:
//! `every_seats_wanted_kind_is_the_one_its_door_refuses_by`
//! (`tests/combine_ops.rs`) drives every seat's op with a node of the
//! wrong kind and asserts the refusal names exactly [`Seat::wants`]. That row is what makes "a
//! seat cannot steer a pick into a seat the door would reject" a
//! fact rather than an intention.
//!
//! The routing is deliberately narrow: it moves a pick only when the
//! seat it would otherwise land in cannot hold it AND another seat
//! can. A pick that fits nowhere lands where the plain rule puts it
//! and refuses at the commit, which is the case the typed refusal
//! exists for.
//!
//! # Survival, and the hazard it does not cover
//!
//! [`Seats::reconcile`] is the consumer's obligation — the application
//! calls it once per frame ([`crate::tools::Tools::reconcile`] is where
//! that happens) — and a consumer that forgets it is not reliably
//! caught later. It guards the DELETED-node case only: a `RecipeNodeId`
//! is a small per-document counter, so once the document is REPLACED
//! under held picks (a `NewDocument`, an open, an undo past the picks'
//! inserts) fresh inserts re-mint the same small ids and stale picks
//! silently denote the NEW nodes. That aliasing is a class hazard of
//! layer-3 state holding `RecipeNodeId`s across history rewinds,
//! tracked as issue #1384.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{Doc, Node, Operand, ProfileProgram, RecipeNodeId, SpokenNode};
use pncad::select::SplitHalf;

use crate::session::{NodeKindWanted, admits};
use crate::vocab::vocabulary;

vocabulary! {
    /// Which seat of a tool a pick (or a drop) is about — one vocabulary of
    /// roles for every seated tool, so a sentence about a held pick is
    /// composed the same way wherever it appears.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Seat {
        /// The profile a revolve sweeps.
        RevolveProfile,
        /// A union's or an intersect's member — one of any number, held
        /// in pick order by the boolean tool rather than in a [`Seats`].
        Member,
        /// The body a subtraction KEEPS (`from`).
        SubtractFrom,
        /// The body a subtraction REMOVES (`tool`).
        SubtractTool,
        /// The body a split cuts.
        SplitTarget,
        /// The body a transform places.
        TransformBody,
        /// The body a pattern replicates.
        PatternBody,
        /// The pattern a part projects one instance of.
        PartInstance,
        /// The body a duplicate copies.
        DuplicateBody,
        /// The body a mirror reflects.
        MirrorBody,
    }

    /// Every seat, so a sweep over the vocabulary cannot silently
    /// miss one — the shape `crate::tools::ToolKind::ALL` uses, for
    /// its reason, and its readers are the same: the test suites, and
    /// nothing under `src/`.
    ///
    /// `combine_ops::every_seats_wanted_kind_is_the_one_its_door_refuses_by`
    /// is the row that sweeps it. That row used to have a second job —
    /// reading this list against a hand-written `Seat::ordinal` for
    /// holes and duplicates — and no longer needs one: there is a
    /// single declaration now, so a seat this array misses is a seat
    /// the enum does not have.
    pub const ALL;
}

impl Seat {
    /// **What kind of node this seat is for.**
    ///
    /// The same vocabulary the commit door refuses by
    /// ([`crate::session::NodeKindWanted`]) rather than a second one:
    /// this table and the door's are two readings of one fact, and
    /// two spellings of it would let a tool route a pick into a seat
    /// the door rejects.
    pub fn wants(self) -> NodeKindWanted {
        match self {
            Self::RevolveProfile => NodeKindWanted::Profile,
            Self::PartInstance => NodeKindWanted::Instances,
            Self::Member
            | Self::SubtractFrom
            | Self::SubtractTool
            | Self::SplitTarget
            | Self::TransformBody
            | Self::PatternBody
            | Self::DuplicateBody
            | Self::MirrorBody => NodeKindWanted::Body,
        }
    }

    /// The seat's name, for sentences.
    pub fn name(self) -> &'static str {
        match self {
            Self::RevolveProfile => "profile",
            Self::Member => "member",
            Self::SubtractFrom => "body kept",
            Self::SubtractTool => "body removed",
            Self::SplitTarget => "split target",
            Self::TransformBody => "transformed body",
            Self::PatternBody => "patterned body",
            Self::PartInstance => "pattern to project",
            Self::DuplicateBody => "duplicated body",
            Self::MirrorBody => "mirrored body",
        }
    }
}

/// **What a face a tool holds beside its seats is for** — the role a
/// held FACE plays, as [`Seat`] names a held node's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaceRole {
    /// The face whose plane a mirror reflects across.
    MirrorPlane,
}

impl FaceRole {
    /// The role's name, for sentences.
    pub fn name(self) -> &'static str {
        match self {
            Self::MirrorPlane => "mirror plane",
        }
    }
}

/// A typed seated-tool refusal (closed enum, D4 ¶3).
#[derive(Debug)]
pub enum SeatError {
    /// A seat the commit needs is still empty.
    Empty {
        /// Which one.
        seat: Seat,
    },
    /// A face pick the commit needs is still empty — a tool's held
    /// face beside its seats, named by the role the panel line gives
    /// it.
    NoFace {
        /// The face's role.
        role: FaceRole,
    },
    /// A seat holds a split picked in the tree, and which of its two
    /// halves it reads is not chosen yet.
    HalfUnchosen {
        /// Which one.
        seat: Seat,
    },
}

impl core::fmt::Display for SeatError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Empty { seat } => write!(f, "no {} picked yet", seat.name()),
            Self::NoFace { role } => write!(f, "no {} picked yet", role.name()),
            Self::HalfUnchosen { seat } => {
                write!(
                    f,
                    "the {} is a split: choose its above or below half",
                    seat.name()
                )
            }
        }
    }
}

impl core::error::Error for SeatError {}

/// A typed tool event the chrome renders — every state change that was
/// not the direct echo of an op.
#[derive(Debug, PartialEq, Eq)]
pub enum SeatEvent {
    /// A held pick's node is no longer in the document; the tool
    /// dropped it and the seat is open again.
    PickLost {
        /// Which seat was emptied.
        seat: Seat,
        /// The node that was held, by the last label the document gave
        /// it ([`Seats::respeak`]).
        node: SpokenNode,
    },
}

impl core::fmt::Display for SeatEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PickLost { seat, node } => write!(
                f,
                "the {} pick ({node}) is no longer in the document; the tool dropped it",
                seat.name(),
            ),
        }
    }
}

/// Two role-typed seats filled in order.
///
/// The roles travel WITH the seats rather than being re-supplied per
/// call: a drop's event has to name the role, and a value that knew its
/// picks but not what they were for could not compose that sentence.
///
/// A one-seat tool is this value with its second role unused — see
/// [`Seats::one`], which names the same seat twice. The arity is read
/// off the roles in one place (`arity`), and what reads it is
/// [`Seats::pick`] (a one-seat tool REPLACES its pick rather than
/// filling a second slot) and every walk over the seats — the panel
/// line, [`Seats::is_empty`], [`Seats::reconcile`] — which goes one
/// entry per SEAT rather than per slot.
///
/// So the second slot of a one-seat value is never set (only `pick`
/// writes a slot, and on that value it writes the first), and nothing
/// reads it either: an invariant held twice rather than relied on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Seats {
    roles: [Seat; 2],
    /// Each seat's pick, its node as the document last spoke it
    /// ([`Seats::respeak`]).
    held: [Option<SpokenNode>; 2],
    /// Which output of its node each seat reads.
    ports: [Port; 2],
    /// **The drawn part each viewport pick hit**: the node whose drawn
    /// part was under the pointer — a placement's copy, which the seat
    /// reads past to the body it places ([`crate::world::seat_read`]),
    /// or a body drawn as itself. `None` for a tree pick, which hit no
    /// part. Where the person aimed, for a preview drawn where they
    /// aimed.
    through: [Option<RecipeNodeId>; 2],
}

/// **Which output of its node a pick reads** — a seat's, or a boolean
/// member's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Port {
    /// The node read alone: its one output.
    Alone,
    /// One output of a node that defines several and is not a split (a
    /// revolve's body).
    Output(u8),
    /// A split's half, read by its port.
    Half(SplitHalf),
    /// A split picked in the tree, its half not chosen yet.
    HalfPending,
}

impl Port {
    /// **The port a pick of `node` reads at a seat wanting `wanted`**:
    /// the output `port` names (a split's half, by its port), else a
    /// split's half still to choose when the seat takes a body, else the
    /// node alone.
    pub(crate) fn of_pick(
        doc: &Doc<ProfileProgram>,
        node: RecipeNodeId,
        port: Option<u8>,
        wanted: NodeKindWanted,
    ) -> Self {
        let split = matches!(doc.node(node), Some(Node::Split { .. }));
        match port {
            Some(port) if split => {
                SplitHalf::of_output_body(u32::from(port)).map_or(Self::Output(port), Self::Half)
            }
            Some(port) => Self::Output(port),
            None if split && wanted == NodeKindWanted::Body => Self::HalfPending,
            None => Self::Alone,
        }
    }

    /// The read of `node` at this port.
    pub(crate) fn read(self, node: RecipeNodeId) -> Operand {
        match self {
            Self::Output(port) => Operand::output(node, port),
            Self::Half(half) => Operand::output(node, half.port()),
            Self::Alone | Self::HalfPending => Operand::Node(node),
        }
    }

    /// This port with its half chosen, where it waits for one.
    pub(crate) fn chosen(self, half: SplitHalf) -> Self {
        match self {
            Self::HalfPending => Self::Half(half),
            other => other,
        }
    }
}

/// **A pick as the panel says it**: the node as `doc` speaks it, and
/// the half it reads or still waits for.
pub(crate) fn pick_said(doc: &Doc<ProfileProgram>, node: RecipeNodeId, port: Port) -> String {
    let node_said = doc.spoken(node).to_string();
    match port {
        Port::Half(half) => format!("{node_said}, {} half", half.name()),
        Port::HalfPending => format!("{node_said}, half not chosen"),
        Port::Alone | Port::Output(_) => node_said,
    }
}

impl Seats {
    /// Two empty seats in their roles.
    pub const fn new(roles: [Seat; 2]) -> Self {
        Self {
            roles,
            held: [None, None],
            ports: [Port::Alone, Port::Alone],
            through: [None, None],
        }
    }

    /// One empty seat: the role twice, so a pick always lands in it and
    /// a reconcile can only ever drop it once (both slots hold the same
    /// `Option`'s worth of state because only the first is ever set).
    pub const fn one(role: Seat) -> Self {
        Self::new([role, role])
    }

    /// Two seats in their roles, holding `held` as already spoken — the
    /// door a tool re-seating picks it already holds goes through (the
    /// boolean tool's change of operation), so nothing is re-read.
    pub(crate) fn holding(roles: [Seat; 2], held: [Option<(SpokenNode, Port)>; 2]) -> Self {
        let [a, b] = held;
        let port = |pick: &Option<(SpokenNode, Port)>| pick.as_ref().map_or(Port::Alone, |p| p.1);
        let ports = [port(&a), port(&b)];
        Self {
            roles,
            held: [a.map(|p| p.0), b.map(|p| p.0)],
            ports,
            // Re-seated, not picked: no copy was aimed at.
            through: [None, None],
        }
    }

    /// Each filled seat's pick as last spoken, with the port it reads,
    /// in seat order (what [`Seats::holding`] takes back).
    pub(crate) fn spoken(&self) -> Vec<(SpokenNode, Port)> {
        (0..2)
            .filter_map(|i| Some((self.held[i].clone()?, self.ports[i])))
            .collect()
    }

    /// The pick in seat `i` (0 or 1).
    pub fn held(&self, i: usize) -> Option<RecipeNodeId> {
        self.held.get(i)?.as_ref().map(SpokenNode::id)
    }

    /// **The read seat `i` holds**: its node's port where the pick
    /// named one (a split's half), else the node read alone.
    pub fn read(&self, i: usize) -> Option<Operand> {
        let node = self.held(i)?;
        Some(self.ports[i].read(node))
    }

    /// **The half a seat holding a split reads**, chosen after a tree
    /// pick named the split alone; a no-op on a seat holding anything
    /// else.
    pub fn choose_half(&mut self, i: usize, half: SplitHalf) {
        if let Some(port) = self.ports.get_mut(i) {
            *port = port.chosen(half);
        }
    }

    /// Whether seat `i` holds a split whose half is still to choose.
    pub fn awaits_half(&self, i: usize) -> bool {
        self.ports.get(i) == Some(&Port::HalfPending)
    }

    /// **Seat `i`'s pick as the panel says it** ([`pick_said`]), the
    /// item [`seat_line`] composes; `None` for an open seat.
    pub(crate) fn said(&self, doc: &Doc<ProfileProgram>, i: usize) -> Option<String> {
        self.held(i).map(|node| pick_said(doc, node, self.ports[i]))
    }

    /// Seat `i`'s role.
    pub fn role(&self, i: usize) -> Seat {
        self.roles[i]
    }

    /// Whether any seat holds a pick.
    pub fn is_empty(&self) -> bool {
        self.each().all(|(_, held)| held.is_none())
    }

    /// **How many seats this value has**: one when its role names
    /// itself twice ([`Seats::one`]), else two — the one place the
    /// arity is read off the roles.
    fn arity(&self) -> usize {
        if self.roles[0] == self.roles[1] { 1 } else { 2 }
    }

    /// Each seat's role and what it holds, in seat order — one entry
    /// per SEAT, so a one-seat tool's unused second slot is not one.
    fn each(&self) -> impl Iterator<Item = (Seat, Option<RecipeNodeId>)> + '_ {
        (0..self.arity()).map(|i| (self.roles[i], self.held(i)))
    }

    /// Fill the first empty seat; with both full, REPLACE the second
    /// (the module docs' pick rule) — **unless the seat that would
    /// take the pick cannot hold this KIND of node and the other seat
    /// can**, in which case it lands in the seat it can only have
    /// been meant for.
    ///
    /// The narrow condition is the point (module docs): a pick that
    /// fits both seats, or neither, follows the plain rule, so the
    /// roles still fill in order and a re-pick still replaces the
    /// last. What the routing removes is the one move that could only
    /// ever refuse — a pick clicked into a seat of another kind — where
    /// the other seat is the only thing the click can have meant.
    ///
    /// **A pick on a copy seats its body** ([`crate::world::seat_read`]):
    /// a world placement's row, or the copy it draws, is a pick of the
    /// body it places, which is what a tool authors against — a split's
    /// half by its port. `body` is the output a viewport pick hit; a
    /// tree pick names none, and a split picked so waits for its half
    /// ([`Seats::choose_half`]).
    pub fn pick(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId) {
        self.pick_at(doc, node, None);
    }

    /// [`Seats::pick`] of the output `body` a viewport pick hit.
    pub fn pick_at(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId, body: Option<u32>) {
        // A viewport pick names the body it hit; a tree pick names none.
        let through = body.map(|_| node);
        let (node, port) = crate::world::seat_read(doc, node, body);
        let state = |seat: Seat| Port::of_pick(doc, node, port, seat.wants());
        // **A one-seat tool has one seat, and its role names itself
        // twice to say so** (`Seats::one`) — so a pick can only
        // land in the first slot, and the second is not a seat to
        // route to or to reconcile.
        //
        // Stated here rather than left to the rule below, which is
        // written over the first EMPTY slot: on a one-seat tool that
        // sends a second pick to a slot nothing reads, and the pick
        // is then a click that silently did nothing.
        let said = Some(doc.spoken(node));
        if self.arity() == 1 {
            self.held[0] = said;
            self.ports[0] = state(self.roles[0]);
            self.through[0] = through;
            return;
        }
        let plain = usize::from(self.held[0].is_some());
        let other = 1 - plain;
        let fits = |seat: usize| admits_pick(doc, node, port, self.roles[seat].wants());
        let seat = if fits(other) && !fits(plain) {
            other
        } else {
            plain
        };
        self.held[seat] = said;
        self.ports[seat] = state(self.roles[seat]);
        self.through[seat] = through;
    }

    /// Empty every seat — the chrome's "start the picks over" door.
    ///
    /// Recorded ergonomic limit of the pick rule: once both seats are
    /// full, a further pick that the SECOND seat's kind admits replaces
    /// the second, so a wrong first pick of a kind both seats admit is
    /// corrected by clearing and re-picking, not by a third click.
    /// (A pick only the first seat admits is routed there — see
    /// [`Seats::pick`] — which is this limit's other half, lifted.) A
    /// per-seat re-pick affordance is chrome a later unit can add
    /// without touching this value.
    pub fn clear(&mut self) {
        self.held = [None, None];
        self.ports = [Port::Alone, Port::Alone];
        self.through = [None, None];
    }

    /// **The held picks' nodes, spoken again from `doc`**, the shown
    /// document after an operation ([`SpokenNode::respoken`]'s rule): a
    /// node `doc` holds takes its label now, and one it no longer holds
    /// keeps the last it had, which is what a drop names it by.
    pub fn respeak(&mut self, doc: &Doc<ProfileProgram>) {
        respeak_each(&mut self.held, doc);
        self.drop_stale_through(doc);
    }

    /// **A drawn part that no longer draws its seat's pick is forgotten**:
    /// gone from the document, or now drawing another body (a takeover
    /// re-pointed the copy). The pick stands; where it was aimed is no
    /// longer drawn.
    fn drop_stale_through(&mut self, doc: &Doc<ProfileProgram>) {
        for i in 0..2 {
            let held = self.held(i);
            if let Some(part) = self.through[i]
                && (doc.node(part).is_none()
                    || Some(crate::world::seat_read(doc, part, None).0) != held)
            {
                self.through[i] = None;
            }
        }
    }

    /// Re-read the held picks against the document, dropping any whose
    /// node is gone (module docs: the survival semantics). Returns the
    /// typed drops.
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<SeatEvent> {
        let mut events = Vec::new();
        for i in 0..self.arity() {
            if self.held[i]
                .as_ref()
                .is_some_and(|node| doc.node(node.id()).is_none())
                && let Some(node) = self.held[i].take()
            {
                self.ports[i] = Port::Alone;
                self.through[i] = None;
                events.push(SeatEvent::PickLost {
                    seat: self.roles[i],
                    node,
                });
            }
        }
        self.drop_stale_through(doc);
        events
    }

    /// **The drawn part seat `i`'s viewport pick hit** (the field's
    /// rule), `None` for a tree pick or an empty seat.
    pub fn through(&self, i: usize) -> Option<RecipeNodeId> {
        self.through.get(i).copied().flatten()
    }

    /// Seat `i`'s pick, or the typed "still empty" refusal naming it.
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] when that seat holds nothing.
    pub fn require(&self, i: usize) -> Result<RecipeNodeId, SeatError> {
        self.held(i).ok_or(SeatError::Empty {
            seat: self.roles[i],
        })
    }

    /// Seat `i`'s read, or the typed refusal naming it: still empty, or
    /// a split whose half is not chosen.
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] when that seat holds nothing,
    /// [`SeatError::HalfUnchosen`] when it holds a split read alone.
    pub fn require_read(&self, i: usize) -> Result<Operand, SeatError> {
        self.require(i)?;
        if self.awaits_half(i) {
            return Err(SeatError::HalfUnchosen {
                seat: self.roles[i],
            });
        }
        self.read(i).ok_or(SeatError::Empty {
            seat: self.roles[i],
        })
    }
}

/// **Whether a seat wanting `wanted` can hold this pick**, for routing
/// only ([`Seats::pick`]): a port read is the variable it names, and a
/// split read alone may still become either half, so a body seat can
/// hold it until its half is chosen.
fn admits_pick(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
    port: Option<u8>,
    wanted: NodeKindWanted,
) -> bool {
    match port {
        Some(port) => crate::session::admits_read(doc, &Operand::output(node, port), wanted),
        None => {
            admits(doc, node, wanted)
                || (wanted == NodeKindWanted::Body
                    && matches!(doc.node(node), Some(Node::Split { .. })))
        }
    }
}

/// **Each held node spoken again from `doc`** ([`SpokenNode::respoken`]):
/// the seats' picks and the mate tool's.
pub(crate) fn respeak_each(held: &mut [Option<SpokenNode>], doc: &Doc<ProfileProgram>) {
    for node in held.iter_mut().flatten() {
        *node = node.respoken(doc);
    }
}

/// **The line a seated tool's panel shows for its held picks**: each
/// seat's role and the feature it holds, in the order the tool's
/// [`Seats`] declares them — so a reader can tell which pick is in
/// which role, the fact that decides what a subtraction removes.
///
/// Takes the value that owns the roles rather than a list of them: a
/// panel that re-listed its tool's roles could name them in another
/// order, or name ones the tool no longer has, and still compile.
///
/// Each item is said in the words a seat's drop notice uses
/// ([`SeatEvent`]): the role by [`Seat::name`] and the pick as `doc`
/// speaks it, so the panel and the notice about the same pick call it
/// one thing.
pub fn seat_line(seats: &Seats, doc: &Doc<ProfileProgram>) -> String {
    picks_line((0..seats.arity()).map(|i| (seats.roles[i].name(), seats.said(doc, i))))
}

/// **The composition of the seated tools' and the mate tool's
/// held-picks line**: `no picks yet` while nothing is held, else one
/// `role: pick` item per seat, `—` for an open one, joined on `"; "`.
/// [`seat_line`] reaches it for the seated tools and
/// [`crate::matetool::MateToolState::line`] for the mate tool, whose
/// picks are faces and whose state is not [`Seats`] (module docs) but
/// whose line has this shape. Each caller spells its own items; the
/// shape, the empty sentence and the mark are this function's.
///
/// **The mark is a literal, and this function is its only spelling.**
/// It is deliberately not [`crate::frame::LIST_SEPARATOR`], which it
/// shares two characters with: that constant is what ONE notice puts
/// between the items of a list of its own — items a counted preamble
/// introduces, inside an enclosing sentence. This is a panel label,
/// not a notice: it reaches no [`crate::frame::Message`], nothing
/// counts the picks and no preamble introduces them. Reading the
/// constant here would put a line outside that population under its
/// edits (`crates/viewer/README.md`, "The third consumer was the
/// second level misread"). Nor does the mark earn a constant of its
/// own: nothing splits a panel line back into its items, so there is
/// no hold for a name to carry.
pub(crate) fn picks_line<R: core::fmt::Display>(
    picks: impl IntoIterator<Item = (R, Option<String>)>,
) -> String {
    let picks: Vec<_> = picks.into_iter().collect();
    if picks.iter().all(|(_, held)| held.is_none()) {
        return "no picks yet".to_owned();
    }
    picks
        .iter()
        .map(|(role, held)| format!("{role}: {}", held.as_deref().unwrap_or("—")))
        .collect::<Vec<_>>()
        .join("; ")
}
