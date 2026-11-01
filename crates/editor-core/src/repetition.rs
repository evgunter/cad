//! **Repetition** (D10 `[repetition]`): which indices a variable or an
//! operation reaches, and the order they nest in.
//!
//! A variable reaches the indices its definition reads, through every
//! definition, pose, selection, pick and operation between; an
//! operation reaches what its slots and its reads reach. A read that
//! takes a family whole — a `union`'s, an `intersect`'s or an
//! `assemble`'s member, or an indexed read `xs[i, j]`, whose own index
//! expressions are slots — reaches nothing through the family it reads:
//! the family is its operand, not its repetition. So does a selection
//! whose names say which member they name (`Member { at, of }`, the
//! family's own table): it reads the family whole.
//!
//! The indices one reader reaches are a chain: each within the one
//! before it, outermost first ([`Reach::order`]). An index is within
//! the index it is declared within, and within the innermost index its
//! count reaches. Two indices neither of which is within the other
//! meet in no reader ([`Reached::Unrelated`]).

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
    /// Two indices reached together, neither within the other.
    Unrelated(VarId, VarId),
}

impl Reached {
    /// The chain, `None` where two indices are unrelated.
    #[must_use]
    pub fn chain(&self) -> Option<&[VarId]> {
        match self {
            Self::Chain(chain) => Some(chain),
            Self::Unrelated(..) => None,
        }
    }

    /// Whether nothing repeats.
    #[must_use]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::Chain(chain) if chain.is_empty())
    }
}

/// **One member's index values**, outermost index first, **and the
/// environment its readers evaluate in** ([`Doc::member_envs`]).
pub type MemberEnv<T> = (Vec<i64>, crate::VarEnv<T>);

/// **The reach of a document's variables and operations**, memoised
/// over one walk: each variable and operation is visited once.
pub struct Reach<'d, P> {
    doc: &'d Doc<P>,
    vars: BTreeMap<VarId, BTreeSet<VarId>>,
    nodes: BTreeMap<RecipeNodeId, BTreeSet<VarId>>,
}

impl<'d, P: crate::ProfilePayload> Reach<'d, P> {
    /// An empty memo over `doc`.
    #[must_use]
    pub fn new(doc: &'d Doc<P>) -> Self {
        Self {
            doc,
            vars: BTreeMap::new(),
            nodes: BTreeMap::new(),
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
                    out.extend(self.of_var(read));
                }
            }
            Some(VarDef::Output { node, .. }) => out = self.of_node(*node),
            Some(VarDef::Select(select)) if names_members(doc, select) => {}
            Some(VarDef::Defined(_) | VarDef::Pose(_) | VarDef::Select(_) | VarDef::Pick(_)) => {
                for read in doc.definition_reads(var) {
                    out.extend(self.of_var(read));
                }
            }
            Some(VarDef::Free(_)) | None => {}
        }
        self.vars.insert(var, out.clone());
        out
    }

    /// The indices the operation `id` reaches, unordered: its slots'
    /// and its reads', a read that takes a family whole excepted.
    pub fn of_node(&mut self, id: RecipeNodeId) -> BTreeSet<VarId> {
        if let Some(held) = self.nodes.get(&id) {
            return held.clone();
        }
        self.nodes.insert(id, BTreeSet::new());
        let doc = self.doc;
        let mut out = BTreeSet::new();
        if let Some(node) = doc.node(id) {
            for &var in node.exprs() {
                out.extend(self.of_var(var));
            }
            for (seat, var) in node.operand_rows() {
                if takes_whole(node, seat) || crate::mate::solve::own_copy_side(doc, id, seat, var)
                {
                    continue;
                }
                out.extend(self.of_var(var));
            }
        }
        self.nodes.insert(id, out.clone());
        out
    }

    /// `set` as a chain, outermost first, or the first two indices of
    /// it that are unrelated.
    #[must_use]
    pub fn order(&self, set: &BTreeSet<VarId>) -> Reached {
        let mut chain: Vec<VarId> = set.iter().copied().collect();
        chain.sort_by_key(|&index| (self.depth(index), index));
        for pair in chain.windows(2) {
            let [outer, inner] = [pair[0], pair[1]];
            if !self.within(inner, outer) {
                return Reached::Unrelated(outer, inner);
            }
        }
        Reached::Chain(chain)
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

    /// How many indices `index` is within, through its declared and
    /// implied outer indices.
    fn depth(&self, index: VarId) -> usize {
        let mut depth = 0;
        let mut at = index;
        let mut seen = BTreeSet::from([index]);
        while let Some(outer) = self.doc.index_within(at) {
            if !seen.insert(outer) {
                break;
            }
            depth += 1;
            at = outer;
        }
        depth
    }

    /// Whether `inner` is within `outer`, through any number of indices.
    fn within(&self, inner: VarId, outer: VarId) -> bool {
        let mut at = inner;
        let mut seen = BTreeSet::from([inner]);
        while let Some(next) = self.doc.index_within(at) {
            if next == outer {
                return true;
            }
            if !seen.insert(next) {
                return false;
            }
            at = next;
        }
        false
    }
}

/// **Whether `select` names members of the family it reads**: every
/// name headed at the body's operation under a `Member` qualifier, the
/// family's own spelling of one member's entity. Such a selection reads
/// the family whole; one whose names are a member's own is read at each
/// member by a reader evaluated per member.
pub(crate) fn names_members<P>(doc: &Doc<P>, select: &crate::var::Select) -> bool {
    let Some(at) = doc.operation_of(select.body) else {
        return false;
    };
    !select.names.is_empty() && select.names.iter().all(|name| names_a_member(name, at))
}

/// Whether `name` names an entity of one of the members of the family
/// the operation `at` defines: headed at `at` under a `Member`
/// qualifier.
pub(crate) fn names_a_member(name: &crate::names::StableName, at: RecipeNodeId) -> bool {
    name.node == at
        && matches!(
            name.path.first(),
            Some(crate::names::RoleSeg::Member { .. })
        )
}

/// Whether `node` reads the variable at `seat` as a family whole: a
/// member of a `Bodies` argument, or an indexed read.
pub(crate) fn takes_whole<P: crate::ProfilePayload>(
    node: &crate::Node<P>,
    seat: crate::OperandSlot,
) -> bool {
    matches!(
        seat,
        crate::OperandSlot::Members | crate::OperandSlot::Member(_)
    ) || node
        .body_read(seat)
        .is_some_and(crate::BodyRead::is_indexed)
}

impl<P: crate::ProfilePayload> Doc<P> {
    /// **The index `index` is within** (D10 `[repetition]`): the one it
    /// is declared within, else the innermost index its count reaches.
    /// `None` for an outermost index, or a variable that is no index.
    #[must_use]
    pub fn index_within(&self, index: VarId) -> Option<VarId> {
        let def = self.var(index)?.def().index()?;
        if let Some(within) = def.within {
            return Some(within);
        }
        let mut reach = Reach::new(self);
        let mut reached = BTreeSet::new();
        let mut reads = Vec::new();
        def.count.var_reads(&mut reads);
        for (read, _) in reads {
            reached.extend(reach.of_var(read));
        }
        reached.remove(&index);
        // The innermost: the one no other reached index is within.
        reached.iter().copied().find(|&candidate| {
            reached
                .iter()
                .all(|&other| other == candidate || !reach.within(other, candidate))
        })
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

    /// **The indices `expr` reaches**, ordered outermost first: those
    /// the variables it reads reach ([`Reach`]).
    #[must_use]
    pub fn expr_reach(&self, expr: &crate::ExprTree<impl crate::LeafSet>) -> Reached {
        let mut reads = Vec::new();
        expr.var_reads(&mut reads);
        let mut reach = Reach::new(self);
        let set = reads
            .into_iter()
            .flat_map(|(read, _)| reach.of_var(read))
            .collect();
        reach.order(&set)
    }

    /// **Each member of the indices `chain`**, ordered outermost first
    /// as a [`Reached::Chain`] orders them: its index values, and the
    /// environment its readers evaluate in, each index bound to its
    /// value and every definition reaching one bound again — what an
    /// operation reaching `chain` is evaluated over, once per member.
    ///
    /// # Errors
    ///
    /// [`crate::NodeErrorKind::IndexCount`] where an index's count does
    /// not evaluate, and [`crate::NodeErrorKind::IndexCountOutOfRange`]
    /// where it evaluates below zero or past `u32::MAX`.
    pub fn member_envs<T: geom_core::Decide>(
        &self,
        chain: &[VarId],
    ) -> Result<Vec<MemberEnv<T>>, crate::NodeErrorKind> {
        let outside = self.var_env::<T>();
        let tuples = crate::eval::members::tuples(self, &outside, chain)?;
        Ok(tuples
            .into_iter()
            .map(|values| {
                let at: Vec<(VarId, i64)> =
                    chain.iter().copied().zip(values.iter().copied()).collect();
                let mut env = outside.clone();
                self.bind_member(&mut env, &at);
                (values, env)
            })
            .collect())
    }

    /// **Every operation reading two indices neither of which is within
    /// the other**, in id order, with the two (D10 `[repetition]`): what
    /// the edit door refuses and the evaluation refuses typed.
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
                Reached::Unrelated(a, b) => Some((id, a, b)),
                Reached::Chain(_) => None,
            })
            .collect()
    }
}
