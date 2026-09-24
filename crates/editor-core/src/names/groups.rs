//! **The emitter's fragment groups, recorded beside the name table.**
//!
//! A fragment qualifier (N2) discriminates the members of ONE group:
//! the entities an op made from one parent. Which entities those are
//! is decided where the names are minted, from descent and geometry:
//! the boolean groups a result face by the operand face it descends
//! from, an edge by its operand edge or seam pair, a seam vertex by its
//! parent pair, and the split groups a face by its operand face and the
//! side it landed on. The group's base name is spelled from that key,
//! and each member's name is the base, alone or with one qualifier.
//!
//! The name alone cannot say which group a row belongs to. Two TIED
//! parents (N2) have one name, so their groups share a base and the
//! tie lane merges their members' rows; and a member can be spelled
//! without the base at all, as when a split no longer divides a face
//! and passes it through under its upstream name. So the emitter
//! records each group it formed ([`GroupRecord`]) — its base, its
//! members' names, and the operand row it descends from — and the
//! diagnosis ladder's group-size rung (`resolve::group_resized`) reads
//! that record ([`FragmentGroups`]) rather than counting spellings. One
//! membership rule, the emitter's, with two readers: the qualifier the
//! emitter mints, and the count the rung reports.
//!
//! The record is diagnosis-time evidence about one run. It rides the
//! node's value like the verdict log does, and like the verdict log it
//! is never persisted and never part of a name: no name, no table and
//! no digest changes with it.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use super::role::{NameRef, StableName};
use crate::node::RecipeNodeId;

/// Where a group's members descend from, as the emitter that formed it
/// knows it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Parent {
    /// The A operand's row of this name: every member descends from
    /// that one operand entity. The union's fold reads it — each step's
    /// A operand is the previous step's accumulation — to follow a
    /// group from the step that formed it to the published body
    /// ([`FragmentGroups::folded`]).
    AOperand(NameRef),
    /// Anything else: a B-operand entity, a seam the op itself formed,
    /// a split's target face. No later fold step carries it by row.
    Elsewhere,
}

/// One group the emitter formed.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Group {
    /// The names its members were published under, one per entity: a
    /// tie's members share a name, so a name may repeat.
    members: Vec<NameRef>,
    /// Where they descend from.
    parent: Parent,
    /// Whether the emitter formed this group from SEVERAL tied parents
    /// at once. Its seam lanes key a group by the two parents' NAMES,
    /// and tied parents share names, so their pieces land in one group
    /// and no one parent's count is on record.
    tie_summed: bool,
}

/// The groups one emission formed, keyed by base name. Built by the
/// emitter that formed them and read through [`FragmentGroups`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct GroupRecord(BTreeMap<StableName, Vec<Group>>);

impl GroupRecord {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Records one group of one parent: its `base`, the names its
    /// `members` were minted under (one per entity), and where they
    /// descend from.
    pub(crate) fn record(&mut self, base: &StableName, members: Vec<NameRef>, parent: Parent) {
        self.push(base, members, parent, false);
    }

    /// Records a group the emitter formed by parent NAMES, so that
    /// tied parents' pieces share it (see [`Group::tie_summed`]).
    /// `tied` says whether any parent was tied.
    pub(crate) fn record_by_name(&mut self, base: &StableName, members: Vec<NameRef>, tied: bool) {
        self.push(base, members, Parent::Elsewhere, tied);
    }

    fn push(&mut self, base: &StableName, members: Vec<NameRef>, parent: Parent, tie_summed: bool) {
        self.0.entry(base.clone()).or_default().push(Group {
            members,
            parent,
            tie_summed,
        });
    }

    /// Each group's own size, `None` if a tie summed any of them.
    fn sizes(&self, base: &StableName) -> Option<Vec<usize>> {
        let Some(groups) = self.0.get(base) else {
            return Some(Vec::new());
        };
        if groups.iter().any(|g| g.tie_summed) {
            return None;
        }
        Some(groups.iter().map(|g| g.members.len()).collect())
    }
}

/// The groups a node's emission formed, as the diagnosis ladder reads
/// them (module docs). A consumer holds one on every node value and
/// can make an empty one; what it records reaches a consumer through
/// `Diagnosis::GroupResized`'s two counts, not through this type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FragmentGroups(Read);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Read {
    /// One emission's record.
    Minted(Arc<GroupRecord>),
    /// A union's fold: each step's record, in fold-space names, under
    /// the union's id ([`FragmentGroups::folded`]).
    Folded {
        node: RecipeNodeId,
        steps: Vec<Arc<GroupRecord>>,
    },
}

impl Default for FragmentGroups {
    fn default() -> Self {
        Self(Read::Minted(Arc::default()))
    }
}

impl FragmentGroups {
    /// No groups: a node whose op forms none.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// One emission's record.
    pub(crate) fn minted(record: Arc<GroupRecord>) -> Self {
        Self(Read::Minted(record))
    }

    /// A record of one group per `(base, size)`, each of one parent, as
    /// an emitter that formed exactly those groups would leave it: for
    /// the resolve suites, which build a run by hand.
    #[cfg(any(test, feature = "test-support"))]
    #[must_use]
    pub fn from_sizes(groups: impl IntoIterator<Item = (StableName, usize)>) -> Self {
        let mut r = GroupRecord::new();
        for (base, size) in groups {
            let members = (0..size).map(|_| NameRef::new(base.clone())).collect();
            r.record(&base, members, Parent::Elsewhere);
        }
        Self::minted(Arc::new(r))
    }

    /// A union's record: the groups each fold step formed, in order.
    ///
    /// Every step's rows are minted under the union's id in the fold's
    /// space, and the published table is their collapse
    /// (`emit_union`), so a published base is the collapse of the step
    /// base that spelled it. [`FragmentGroups::sizes`] makes that
    /// collapse when it is asked, not here: the record is evidence for
    /// the diagnosis ladder, and a fold-space name the collapse refuses
    /// is the ladder's to decline over, never the evaluation's to fail
    /// on.
    pub(crate) fn folded(node: RecipeNodeId, steps: Vec<Arc<GroupRecord>>) -> Self {
        Self(Read::Folded { node, steps })
    }

    /// How many entities of the node's output descend from each group
    /// spelled from `base`, one count per group (two tied parents give
    /// two), empty when no group is. `None` when no one parent's count
    /// is on record: a group tied parents share, or a union's record
    /// that cannot be read in its published space.
    pub(crate) fn sizes(&self, base: &StableName) -> Option<Vec<usize>> {
        match &self.0 {
            Read::Minted(r) => r.sizes(base),
            Read::Folded { node, steps } => folded_sizes(*node, steps, base),
        }
    }
}

/// [`FragmentGroups::sizes`] over a union's fold.
///
/// A group a step formed counts the entities of the PUBLISHED body that
/// descend from it. The last step's groups are the published body's, so
/// each counts its members. An earlier step's group counts what the
/// following steps made of its members: a member row is carried into
/// the next step by that step's groups whose [`Parent::AOperand`] is
/// that row (every entity of the accumulation is the next step's A
/// operand), and it counts the entities those groups count in turn. A
/// member the next step swallows is carried by no group and counts 0;
/// a member the next step divides counts each piece. The walk matches
/// member rows to the recorded parent rows by name, which is the key
/// the emitter itself descended by: nothing re-spells a base.
///
/// A published base can be spelled by more than one step: a step that
/// leaves an entity whole spells a group of one, and a later step that
/// divides it spells the group of its pieces under the same collapsed
/// base. Both count the same published entities, so the latest step
/// that spells the base answers. Two groups of ONE step whose different
/// bases collapse to the same published base are two answers with no
/// rule between them, and the fold declines.
fn folded_sizes(
    node: RecipeNodeId,
    steps: &[Arc<GroupRecord>],
    base: &StableName,
) -> Option<Vec<usize>> {
    let counts = descendant_counts(steps);
    for (k, step) in steps.iter().enumerate().rev() {
        let mut hit: Option<&StableName> = None;
        for b in step.0.keys() {
            if super::emit_union::collapse_name(node, b).ok()? != *base {
                continue;
            }
            if hit.is_some() {
                return None;
            }
            hit = Some(b);
        }
        if let Some(b) = hit {
            let groups = &step.0[b];
            if groups.iter().any(|g| g.tie_summed) {
                return None;
            }
            return Some(counts[k][b].clone());
        }
    }
    Some(Vec::new())
}

/// For every step, every group's count of published descendants
/// ([`folded_sizes`]), in the record's own order.
fn descendant_counts(steps: &[Arc<GroupRecord>]) -> Vec<BTreeMap<&StableName, Vec<usize>>> {
    let mut out: Vec<BTreeMap<&StableName, Vec<usize>>> = vec![BTreeMap::new(); steps.len()];
    // What the step after `k` makes of each of its A-operand rows.
    let mut carried: BTreeMap<&NameRef, usize> = BTreeMap::new();
    for (k, step) in steps.iter().enumerate().rev() {
        let last = k + 1 == steps.len();
        for (b, groups) in &step.0 {
            let counts = groups
                .iter()
                .map(|g| {
                    if last {
                        g.members.len()
                    } else {
                        let rows: BTreeSet<&NameRef> = g.members.iter().collect();
                        rows.iter()
                            .map(|r| carried.get(r).copied().unwrap_or(0))
                            .sum()
                    }
                })
                .collect();
            out[k].insert(b, counts);
        }
        carried = BTreeMap::new();
        for (b, groups) in &step.0 {
            for (g, n) in groups.iter().zip(&out[k][b]) {
                if let Parent::AOperand(row) = &g.parent {
                    *carried.entry(row).or_default() += n;
                }
            }
        }
    }
    out
}

/// What a group-forming emitter hands back: the name table, and the
/// groups it formed beside it.
#[derive(Debug)]
pub(crate) struct Emitted {
    /// The node's names.
    pub(crate) table: Arc<super::NameTable>,
    /// The groups those names were minted from.
    pub(crate) groups: Arc<GroupRecord>,
}

impl Emitted {
    pub(crate) fn new(table: super::NameTable, groups: GroupRecord) -> Self {
        Self {
            table: Arc::new(table),
            groups: Arc::new(groups),
        }
    }
}
