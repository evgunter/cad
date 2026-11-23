//! **Repetition** (D10 `[repetition]`): which indices a variable or an
//! operation reaches, and the binder they are of.
//!
//! A document repeats only inside a binder `{k in 0..N: …}`: its first
//! index ([`crate::IndexSeat::Binder`]), each further index after the
//! one before it, and the names it yields. Inside a binder a step
//! repeats exactly when it reads an index, directly or through another
//! definition, pose, selection, pick or operation. A yield is a family to
//! every reader, so a read of one reaches nothing through it, and an
//! indexed read `xs[i, j]` reaches what its index expressions reach. A
//! yield is itself one member per value of all of its binder's indices,
//! whatever it reads.
//!
//! The indices one reader reaches are a chain in its binder's order,
//! outermost first ([`Reach::order`]). A reader reaching the indices of
//! two binders is refused ([`Reached::TwoBinders`]): no binder holds it.

use std::collections::{BTreeMap, BTreeSet};

use crate::doc::Doc;
use crate::node::RecipeNodeId;
use crate::var::{VarDef, VarId};

/// **The indices one variable or operation reaches**, as a reader
/// reads them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reached {
    /// The chain, outermost first; empty where nothing repeats.
    Chain(Vec<VarId>),
    /// Two indices reached together that are of two binders.
    TwoBinders(VarId, VarId),
}

impl Reached {
    /// The chain, `None` where two binders' indices are reached.
    #[must_use]
    pub fn chain(&self) -> Option<&[VarId]> {
        match self {
            Self::Chain(chain) => Some(chain),
            Self::TwoBinders(..) => None,
        }
    }

    /// Whether nothing repeats.
    #[must_use]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::Chain(chain) if chain.is_empty())
    }
}

/// **One member of a chain of indices** ([`Doc::member_envs`]): its
/// index values and the environment its readers evaluate in.
#[derive(Debug, Clone)]
pub struct MemberEnv<T> {
    /// The member's index values, outermost index first.
    pub at: Vec<i64>,
    /// The document's environment with each index bound to its value
    /// and every definition reaching one bound again.
    pub env: crate::VarEnv<T>,
}

/// **Why a chain's members could not be listed**: an index's count,
/// which says how many members it has, is not a count an index holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemberCountFault {
    /// The index's count did not evaluate.
    Count {
        /// The index.
        index: VarId,
        /// Why its count did not evaluate.
        source: crate::EvalError,
    },
    /// The index's count evaluated outside [`Self::ADMITS`].
    OutOfRange {
        /// The index.
        index: VarId,
        /// The evaluated count.
        count: i64,
    },
}

impl MemberCountFault {
    /// The counts an index admits, both ends included: none below zero,
    /// none past the largest member index the evaluator reads into a
    /// `u32`.
    pub const ADMITS: core::ops::RangeInclusive<i64> = 0..=u32::MAX as i64;

    /// The index whose count it is.
    #[must_use]
    pub fn index(&self) -> VarId {
        match *self {
            Self::Count { index, .. } | Self::OutOfRange { index, .. } => index,
        }
    }

    /// **The evaluation's refusal of a family whose members it could
    /// not list**: what a reader outside the evaluator says, so the
    /// kernel words it once.
    #[must_use]
    pub fn node_error(self) -> crate::NodeErrorKind {
        match self {
            Self::Count { index, source } => crate::NodeErrorKind::IndexCount { index, source },
            Self::OutOfRange { index, count } => {
                crate::NodeErrorKind::IndexCountOutOfRange { index, count }
            }
        }
    }
}

/// **The reach of a document's variables and operations**, memoised
/// over one walk: each variable and operation is visited once.
pub struct Reach<'d, P> {
    doc: &'d Doc<P>,
    binders: Binders,
    vars: BTreeMap<VarId, BTreeSet<VarId>>,
    nodes: BTreeMap<RecipeNodeId, BTreeSet<VarId>>,
    /// Whether a yield reaches its binder's indices because it is one
    /// ([`Self::own`] reads without these edges).
    yield_edges: bool,
}

/// **A yield rule a binder's yields break** ([`Doc::yield_rule`]), the
/// fault the door and load each say in their own words.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum YieldRule {
    /// The binder yields nothing.
    Nothing,
    /// A yield is no live variable.
    NotLive(VarId),
    /// A yield is an index.
    Index(VarId),
    /// A yield is written twice.
    Twice(VarId),
    /// A yield is another binder's.
    Elsewhere {
        /// The yield.
        var: VarId,
        /// The binder yielding it.
        binder: VarId,
    },
    /// A yield reads none of the binder's indices.
    Outside(VarId),
}

/// **A document's binders**, read once: each index's binder and place
/// in it, each binder's indices in order, and each yield's binder. A
/// pass that asks of many reads builds it once ([`Doc::binders`]) and
/// asks it, where each of [`Doc::yielding`] and [`Doc::spell`] reads the
/// document's binders afresh.
#[derive(Debug, Clone, Default)]
pub struct Binders {
    /// Each index: its binder's first index, and its place in it.
    seat: BTreeMap<VarId, (VarId, usize)>,
    /// Each binder, by its first index: its indices, outermost first.
    indices: BTreeMap<VarId, Vec<VarId>>,
    /// Each yield: the binder yielding it.
    yields: BTreeMap<VarId, VarId>,
}

impl Binders {
    /// `doc`'s binders. An index whose chain of `after` reaches no
    /// binder's first index (a cycle no door admits) is of no binder.
    pub(crate) fn of<P: crate::ProfilePayload>(doc: &Doc<P>) -> Self {
        let mut out = Self::default();
        let mut next: BTreeMap<VarId, VarId> = BTreeMap::new();
        let mut firsts = Vec::new();
        for (&id, var) in doc.vars() {
            let Some(index) = var.def().index() else {
                continue;
            };
            match &index.seat {
                crate::IndexSeat::Binder { yields } => {
                    firsts.push(id);
                    for &y in yields {
                        out.yields.insert(y, id);
                    }
                }
                crate::IndexSeat::After(before) => {
                    next.insert(*before, id);
                }
            }
        }
        for first in firsts {
            let mut chain = vec![first];
            let mut at = first;
            while let Some(&after) = next.get(&at) {
                if chain.contains(&after) {
                    break;
                }
                chain.push(after);
                at = after;
            }
            for (place, &index) in chain.iter().enumerate() {
                out.seat.insert(index, (first, place));
            }
            out.indices.insert(first, chain);
        }
        out
    }

    /// The binder yielding `var`, if it is a yield.
    #[must_use]
    pub fn yielding(&self, var: VarId) -> Option<VarId> {
        self.yields.get(&var).copied()
    }

    /// [`Doc::family_through_sources`] against these binders, `doc`'s.
    #[must_use]
    pub fn family_through_sources<P: crate::ProfilePayload>(
        &self,
        doc: &Doc<P>,
        var: VarId,
    ) -> bool {
        let read = doc.through_sources(var);
        !read.is_indexed() && self.yielding(read.read).is_some()
    }

    /// [`Doc::spell`] against these binders, `doc`'s.
    #[must_use]
    pub fn spell<P: crate::ProfilePayload, R>(
        &self,
        doc: &Doc<P>,
        var: VarId,
        read: R,
    ) -> crate::Spell<R> {
        crate::Spell::of(read, self.family_through_sources(doc, var))
    }

    /// The binder of the index `index`, and its place there.
    pub(crate) fn seat(&self, index: VarId) -> Option<(VarId, usize)> {
        self.seat.get(&index).copied()
    }

    /// The indices of the binder `binder`, outermost first.
    pub(crate) fn indices(&self, binder: VarId) -> &[VarId] {
        self.indices.get(&binder).map_or(&[], Vec::as_slice)
    }
}

impl<'d, P: crate::ProfilePayload> Reach<'d, P> {
    /// An empty memo over `doc`.
    #[must_use]
    pub fn new(doc: &'d Doc<P>) -> Self {
        Self {
            doc,
            binders: Binders::of(doc),
            vars: BTreeMap::new(),
            nodes: BTreeMap::new(),
            yield_edges: true,
        }
    }

    /// **A walk of what each variable reads itself**: [`Self::new`]'s,
    /// without the edge by which a yield reaches its binder's indices
    /// for being one. What judges whether a yield repeats in its binder
    /// (`YieldsFault::Outside`).
    #[must_use]
    pub fn own(doc: &'d Doc<P>) -> Self {
        Self {
            yield_edges: false,
            ..Self::new(doc)
        }
    }

    /// The document's binders, as this walk reads them.
    pub(crate) fn binders(&self) -> &Binders {
        &self.binders
    }

    /// **What a read of `var` reaches**: nothing through a yield, which
    /// is a family to every reader; else what `var` reaches.
    pub fn of_read(&mut self, var: VarId) -> BTreeSet<VarId> {
        if self.binders.yielding(var).is_some() {
            BTreeSet::new()
        } else {
            self.of_var(var)
        }
    }

    /// The indices `var` reaches, unordered.
    pub fn of_var(&mut self, var: VarId) -> BTreeSet<VarId> {
        if let Some(held) = self.vars.get(&var) {
            return held.clone();
        }
        // A read cycle, which no door admits, reaches nothing rather
        // than recursing.
        self.vars.insert(var, BTreeSet::new());
        let doc = self.doc;
        let mut out = BTreeSet::new();
        match doc.var(var).map(crate::Var::def) {
            Some(VarDef::Index(_)) => {
                out.insert(var);
                for read in doc.definition_reads(var) {
                    out.extend(self.of_read(read));
                }
            }
            Some(VarDef::Output { node, .. }) => out = self.of_node(*node),
            // A member read reads the family whole: the selection reaches
            // what its indices reach.
            Some(VarDef::Select(select)) if select.body.is_indexed() => {
                for &index in &select.body.at {
                    out.extend(self.of_read(index));
                }
            }
            Some(VarDef::Source(body)) if body.is_indexed() => {
                for &index in &body.at {
                    out.extend(self.of_read(index));
                }
            }
            Some(
                VarDef::Defined(_)
                | VarDef::Pose(_)
                | VarDef::Select(_)
                | VarDef::Pick(_)
                | VarDef::Source(_),
            ) => {
                for read in doc.definition_reads(var) {
                    out.extend(self.of_read(read));
                }
            }
            Some(VarDef::Free(_)) | None => {}
        }
        // A yield is one member per value of its binder's every index.
        if self.yield_edges
            && let Some(binder) = self.binders.yielding(var)
        {
            out.extend(self.binders.indices(binder).iter().copied());
        }
        self.vars.insert(var, out.clone());
        out
    }

    /// The indices `expr` reaches, unordered: those the variables it
    /// reads reach.
    pub fn of_expr(&mut self, expr: &crate::ExprTree<impl crate::LeafSet>) -> BTreeSet<VarId> {
        let mut reads = Vec::new();
        expr.var_reads(&mut reads);
        reads
            .into_iter()
            .flat_map(|(read, _)| self.of_read(read))
            .collect()
    }

    /// The indices the operation `id` reaches, unordered: its slots'
    /// and its reads', a yield's excepted, and every index of the binder
    /// of each of its outputs that is a yield.
    pub fn of_node(&mut self, id: RecipeNodeId) -> BTreeSet<VarId> {
        if let Some(held) = self.nodes.get(&id) {
            return held.clone();
        }
        self.nodes.insert(id, BTreeSet::new());
        let doc = self.doc;
        let mut out = BTreeSet::new();
        if let Some(node) = doc.node(id) {
            for &var in node.exprs() {
                out.extend(self.of_read(var));
            }
            for (seat, var) in node.operand_rows() {
                if crate::mate::solve::own_copy_side(doc, id, seat, var) {
                    continue;
                }
                out.extend(self.of_read(var));
            }
            for output in doc.outputs(id).into_iter().filter(|_| self.yield_edges) {
                if let Some(binder) = self.binders.yielding(output) {
                    out.extend(self.binders.indices(binder).iter().copied());
                }
            }
        }
        self.nodes.insert(id, out.clone());
        out
    }

    /// `set` as a chain in its binder's order, outermost first, or the
    /// first two indices of it that are of two binders.
    #[must_use]
    pub fn order(&self, set: &BTreeSet<VarId>) -> Reached {
        let mut chain: Vec<(Option<(VarId, usize)>, VarId)> = set
            .iter()
            .map(|&index| (self.binders.seat(index), index))
            .collect();
        chain.sort();
        for pair in chain.windows(2) {
            let [(outer_seat, outer), (inner_seat, inner)] = [pair[0], pair[1]];
            if outer_seat.map(|(binder, _)| binder) != inner_seat.map(|(binder, _)| binder)
                || outer_seat.is_none()
            {
                return Reached::TwoBinders(outer, inner);
            }
        }
        Reached::Chain(chain.into_iter().map(|(_, index)| index).collect())
    }

    /// The indices `var` reaches, ordered.
    pub fn var(&mut self, var: VarId) -> Reached {
        let set = self.of_var(var);
        self.order(&set)
    }

    /// The indices `id` reaches, ordered.
    pub fn node(&mut self, id: RecipeNodeId) -> Reached {
        let set = self.of_node(id);
        self.order(&set)
    }
}

impl<P: crate::ProfilePayload> Doc<P> {
    /// **The index after `index` in its binder**, if one is declared
    /// after it.
    #[must_use]
    pub fn index_after(&self, index: VarId) -> Option<VarId> {
        self.vars
            .iter()
            .find(|(_, v)| v.def().index().and_then(crate::Index::after) == Some(index))
            .map(|(&id, _)| id)
    }

    /// **The binder yielding `var`**, by its first index, if `var` is a
    /// yield: a family to every reader (D10 `[repetition]`).
    #[must_use]
    pub fn yielding(&self, var: VarId) -> Option<VarId> {
        Binders::of(self).yielding(var)
    }

    /// **Why the binder `binder` yielding `yields` breaks a yield rule**
    /// (D10 `[repetition]`), the first in the order written, or `None`:
    /// the one home of the rules, which `DocEdit::SetYields`'s door asks
    /// with what it would write and [`Self::yield_faults`] asks of every
    /// binder a document holds. `own` is a walk of what each variable
    /// reads itself ([`Reach::own`]), so a yield is judged by its own
    /// reads, not by being yielded.
    pub(crate) fn yield_rule(
        &self,
        binder: VarId,
        yields: &[VarId],
        own: &mut Reach<'_, P>,
    ) -> Option<YieldRule> {
        if yields.is_empty() {
            return Some(YieldRule::Nothing);
        }
        let indices = own.binders().indices(binder).to_vec();
        let mut seen = BTreeSet::new();
        for &var in yields {
            let Some(held) = self.var(var) else {
                return Some(YieldRule::NotLive(var));
            };
            if held.def().index().is_some() {
                return Some(YieldRule::Index(var));
            }
            if !seen.insert(var) {
                return Some(YieldRule::Twice(var));
            }
            if let Some(other) = own.binders().yielding(var).filter(|&other| other != binder) {
                return Some(YieldRule::Elsewhere { var, binder: other });
            }
            if !own.of_var(var).iter().any(|k| indices.contains(k)) {
                return Some(YieldRule::Outside(var));
            }
        }
        None
    }

    /// **Every binder whose yields break a yield rule**, by its first
    /// index ([`Self::yield_rule`]), in declaration order: what every
    /// edit's door diffs before and after, and what load refuses. A
    /// binder that yields nothing is no fault here: an edit that deletes
    /// its last yield reports it (`Maintenance::YieldRemoved`) and the
    /// evaluation refuses what stands in it (`YieldsNothing`); only
    /// `SetYields` refuses writing nothing.
    pub(crate) fn yield_faults(&self) -> Vec<(VarId, YieldRule)> {
        if !self.declares_index() {
            return Vec::new();
        }
        let mut own = Reach::own(self);
        let mut written: BTreeMap<VarId, VarId> = BTreeMap::new();
        let mut out = Vec::new();
        for (&binder, var) in self.vars() {
            let Some(crate::IndexSeat::Binder { yields }) = var.def().index().map(|i| &i.seat)
            else {
                continue;
            };
            // A variable two binders write is the later one's fault.
            if let Some(&var) = yields
                .iter()
                .find(|y| written.get(y).is_some_and(|&b| b != binder))
            {
                out.push((
                    binder,
                    YieldRule::Elsewhere {
                        var,
                        binder: written[&var],
                    },
                ));
                continue;
            }
            for &y in yields {
                written.insert(y, binder);
            }
            match self.yield_rule(binder, yields, &mut own) {
                None | Some(YieldRule::Nothing | YieldRule::Elsewhere { .. }) => {}
                Some(rule) => out.push((binder, rule)),
            }
        }
        out
    }

    /// **Whether `read` is a member read at its binder's own key**,
    /// `xs[k]` inside `{k in xs: …}`: `xs` a yield and its indices
    /// exactly its binder's, outermost first (D10 `[repetition]`). Such a
    /// read is the member the reader stands at, as a plain read of a
    /// k-reader is.
    #[must_use]
    pub fn reads_own_member(&self, read: &crate::BodyRead<VarId>) -> bool {
        read.is_indexed()
            && self
                .yielding(read.read)
                .is_some_and(|binder| self.binder_indices(binder) == read.at)
    }

    /// **The list entry reading `var` as `read`**: spliced in whole,
    /// `…var`, where `var` is through its sources a family a binder
    /// yields, and read as one body otherwise (D10 `[repetition]`).
    #[must_use]
    pub fn spell<R>(&self, var: VarId, read: R) -> crate::Spell<R> {
        self.binders().spell(self, var, read)
    }

    /// **Whether `var` is, through its sources, a family a binder
    /// yields**, read whole.
    #[must_use]
    pub fn family_through_sources(&self, var: VarId) -> bool {
        self.binders().family_through_sources(self, var)
    }

    /// **The document's binders**, read once, for a pass that asks of
    /// many reads ([`Binders`]).
    #[must_use]
    pub fn binders(&self) -> Binders {
        Binders::of(self)
    }

    /// **The binder of the index `index`**, by its first index; `None`
    /// for a variable that is no index.
    #[must_use]
    pub fn binder_of(&self, index: VarId) -> Option<VarId> {
        Binders::of(self).seat(index).map(|(binder, _)| binder)
    }

    /// **The binder's indices**, outermost first, from its first index
    /// `binder`; empty where `binder` is no binder's first index.
    #[must_use]
    pub fn binder_indices(&self, binder: VarId) -> Vec<VarId> {
        Binders::of(self).indices(binder).to_vec()
    }

    /// **What the binder `binder` yields**, in the order written; empty
    /// where `binder` is no binder's first index.
    #[must_use]
    pub fn yields(&self, binder: VarId) -> &[VarId] {
        match self
            .var(binder)
            .and_then(|v| v.def().index())
            .map(|i| &i.seat)
        {
            Some(crate::IndexSeat::Binder { yields }) => yields,
            _ => &[],
        }
    }

    /// Whether the document declares an index: where it declares none,
    /// nothing repeats.
    #[must_use]
    pub fn declares_index(&self) -> bool {
        self.vars.values().any(|v| v.def().index().is_some())
    }

    /// **The indices `var` reaches**, ordered outermost first
    /// ([`Reach`]).
    #[must_use]
    pub fn var_reach(&self, var: VarId) -> Reached {
        Reach::new(self).var(var)
    }

    /// **The indices the operation `id` reaches**, ordered outermost
    /// first ([`Reach`]).
    #[must_use]
    pub fn node_reach(&self, id: RecipeNodeId) -> Reached {
        Reach::new(self).node(id)
    }

    /// **The indices `expr` reaches**, ordered outermost first
    /// ([`Reach::of_expr`]).
    #[must_use]
    pub fn expr_reach(&self, expr: &crate::ExprTree<impl crate::LeafSet>) -> Reached {
        let mut reach = Reach::new(self);
        let set = reach.of_expr(expr);
        reach.order(&set)
    }

    /// **Each member of the indices `chain`**, ordered outermost first
    /// as a [`Reached::Chain`] orders them, over the document's own
    /// environment: what an operation reaching `chain` is evaluated
    /// over, once per member. Lazy, as the evaluator's members are
    /// (`eval::members::members`): one member's environment at a time.
    ///
    /// # Errors
    ///
    /// An item is a [`MemberCountFault`] where an index's count did not
    /// evaluate, or is not a count an index admits; nothing follows it.
    pub fn member_envs<'a, T: geom_core::Decide + 'a>(
        &'a self,
        chain: &'a [VarId],
    ) -> impl Iterator<Item = Result<MemberEnv<T>, MemberCountFault>> + 'a {
        crate::eval::members::members(
            self,
            self.var_env::<T>(),
            chain,
            |env: &crate::VarEnv<T>| env,
            |env: &mut crate::VarEnv<T>, at| self.bind_member(env, at),
        )
        .map(|member| member.map(|(at, env)| MemberEnv { at, env }))
    }

    /// **Every operation reaching the indices of two binders**, in id
    /// order, with two of them (D10 `[repetition]`): what the edit door
    /// refuses and the evaluation refuses typed.
    #[must_use]
    pub fn indices_faults(&self) -> Vec<(RecipeNodeId, VarId, VarId)> {
        // A document that declares no two indices has no such reader.
        let indices = self
            .vars
            .values()
            .filter(|v| v.def().index().is_some())
            .take(2)
            .count();
        if indices < 2 {
            return Vec::new();
        }
        let mut reach = Reach::new(self);
        self.ids()
            .into_iter()
            .filter_map(|id| match reach.node(id) {
                Reached::TwoBinders(a, b) => Some((id, a, b)),
                Reached::Chain(_) => None,
            })
            .collect()
    }
}

/// **A seat handed a yield where it takes one body, refused** (D10
/// `[repetition]`): what every edit's door diffs before and after
/// ([`Doc::family_read_faults`]), so an edit that makes a read a family
/// at a one-body seat (a redefinition, a new yield) refuses by kind. A
/// source is no such reader: it is what it reads, and its readers are
/// asked instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum FamilyReadFault {
    /// An operation's seat.
    Seat {
        /// The operation.
        node: RecipeNodeId,
        /// Its seat.
        slot: crate::OperandSlot,
        /// The variable read.
        var: VarId,
        /// The kind it is read as.
        found: crate::VarKind,
    },
}

impl<P: crate::ProfilePayload> Doc<P> {
    /// **Every read a binder's yields make ill-typed**, in id order: the
    /// seat reads [`Doc::read_fault`] refuses. What the edit
    /// door holds against the document before an edit, so writing a
    /// binder's yields refuses where an existing reader would read a
    /// family at a seat for one body. Empty in a document with no index.
    #[must_use]
    pub(crate) fn family_read_faults(&self) -> Vec<FamilyReadFault> {
        if !self.declares_index() {
            return Vec::new();
        }
        // The binders, read once: a seat is asked only where it reads a
        // yield, which is the one family a reader can meet.
        let binders = Binders::of(self);
        let mut out = Vec::new();
        for (&id, node) in &self.nodes {
            let own = self.outputs(id);
            for (slot, var) in node.operand_rows() {
                // A placement's mate reads its own copy, one member.
                if own.contains(&var) || binders.yielding(self.through_sources(var).read).is_none()
                {
                    continue;
                }
                let Some(held) = self.var(var) else { continue };
                let expected = node.seat_kind(slot);
                if let Some(crate::doc::ReadFault::Kind { found }) =
                    self.read_fault(var, held, expected, crate::doc::Reader::of(expected))
                {
                    out.push(FamilyReadFault::Seat {
                        node: id,
                        slot,
                        var,
                        found,
                    });
                }
            }
        }
        out
    }
}
