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
//! records each group it formed — its base and how many entities it
//! holds — and the diagnosis ladder's group-size rung
//! (`resolve::group_resized`) reads that record rather than counting
//! spellings. One membership rule, the emitter's, with two readers:
//! the qualifier the emitter mints, and the count the rung reports.
//!
//! The record is diagnosis-time evidence about one run. It rides the
//! node's value like the verdict log does, and like the verdict log it
//! is never persisted and never part of a name: no name, no table and
//! no digest changes with it.

use std::collections::BTreeMap;
use std::sync::Arc;

use super::role::{NameRef, RoleSeg, StableName};
use crate::node::RecipeNodeId;

/// One group the emitter formed.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Group {
    /// How many entities of the node's output descend from the group's
    /// parent.
    size: u32,
    /// The names its members were published under, in the minting
    /// step's space. Read only to follow a union's fold
    /// ([`FragmentGroups::folded`]); a tie's members share names, so a
    /// name may repeat.
    members: Vec<NameRef>,
}

/// The groups one node's emission formed, keyed by base name
/// (module docs).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FragmentGroups(Record);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Record {
    /// One emission's groups. A base holds one group per parent: two
    /// when two tied parents share it.
    Minted(BTreeMap<StableName, Vec<Group>>),
    /// A union's fold: each step's own groups, in fold-space names,
    /// under the union's id ([`FragmentGroups::folded`]).
    Folded {
        node: RecipeNodeId,
        steps: Vec<Arc<FragmentGroups>>,
    },
}

impl Default for Record {
    fn default() -> Self {
        Self::Minted(BTreeMap::new())
    }
}

impl FragmentGroups {
    /// No groups: a node whose op forms none.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Records one group of `size` entities whose base is `base`. For a
    /// caller that assembles a record by hand (the resolve suites); an
    /// emitter records through [`FragmentGroups::record`], which also
    /// keeps the members' names.
    pub fn record_size(&mut self, base: StableName, size: u32) {
        self.push(base, size, Vec::new());
    }

    /// Records one group the emitter formed: its `base`, and the names
    /// its `members` were minted under, one per entity.
    pub(crate) fn record(&mut self, base: &StableName, members: Vec<NameRef>) {
        let size = u32::try_from(members.len()).unwrap_or(u32::MAX);
        self.push(base.clone(), size, members);
    }

    fn push(&mut self, base: StableName, size: u32, members: Vec<NameRef>) {
        match &mut self.0 {
            Record::Minted(groups) => groups
                .entry(base)
                .or_default()
                .push(Group { size, members }),
            // A fold's record is assembled whole; nothing is added to it.
            Record::Folded { .. } => {}
        }
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
    pub(crate) fn folded(node: RecipeNodeId, steps: Vec<Arc<FragmentGroups>>) -> Self {
        Self(Record::Folded { node, steps })
    }

    /// The size of every group spelled from `base`, one per group (two
    /// tied parents give two), empty when no group is. `None` when a
    /// union's record cannot be read in its published space.
    #[must_use]
    pub fn sizes(&self, base: &StableName) -> Option<Vec<u32>> {
        match &self.0 {
            Record::Minted(groups) => Some(
                groups
                    .get(base)
                    .map(|gs| gs.iter().map(|g| g.size).collect())
                    .unwrap_or_default(),
            ),
            Record::Folded { node, steps } => folded_sizes(*node, steps, base),
        }
    }

    /// One emission's groups, or none for a fold.
    fn minted(&self) -> Option<&BTreeMap<StableName, Vec<Group>>> {
        match &self.0 {
            Record::Minted(groups) => Some(groups),
            Record::Folded { .. } => None,
        }
    }
}

/// What a group-forming emitter hands back: the name table, and the
/// groups it formed beside it.
#[derive(Debug)]
pub(crate) struct Emitted {
    /// The node's names.
    pub(crate) table: Arc<super::NameTable>,
    /// The groups those names were minted from.
    pub(crate) groups: Arc<FragmentGroups>,
}

impl Emitted {
    pub(crate) fn new(table: super::NameTable, groups: FragmentGroups) -> Self {
        Self {
            table: Arc::new(table),
            groups: Arc::new(groups),
        }
    }
}

/// [`FragmentGroups::sizes`] over a union's fold.
///
/// A group a step formed LIVES if some entity of the published body
/// descends from it: one of its members is carried into the next
/// step, by a group of that step whose base is `FromA(member)` (every
/// entity of the accumulation is the next step's A operand) and which
/// lives in turn; every group of the last step lives. A step that
/// swallows a group's members whole ends it, so a group of the prior
/// fold whose entity a later member consumed counts 0, not what the
/// step that formed it counted.
///
/// A published base can be spelled by more than one step: a step
/// that leaves an entity whole spells a group of one, and a later step
/// that divides it spells the group of its pieces under the same
/// collapsed base. The LATEST living one is the group the published
/// names discriminate.
fn folded_sizes(
    node: RecipeNodeId,
    steps: &[Arc<FragmentGroups>],
    base: &StableName,
) -> Option<Vec<u32>> {
    let minted: Vec<&BTreeMap<StableName, Vec<Group>>> =
        steps.iter().map(|s| s.minted()).collect::<Option<_>>()?;
    // Living groups, per step, walking back from the last step.
    let mut alive: Vec<BTreeMap<&StableName, Vec<bool>>> = vec![BTreeMap::new(); minted.len()];
    for k in (0..minted.len()).rev() {
        for (b, groups) in minted[k] {
            let lives = groups
                .iter()
                .map(|g| {
                    k + 1 == minted.len()
                        || g.members.iter().any(|m| {
                            let carried = StableName {
                                kind: m.kind,
                                node,
                                path: vec![RoleSeg::FromA(m.clone())],
                            };
                            alive[k + 1]
                                .get(&carried)
                                .is_some_and(|next| next.iter().any(|&l| l))
                        })
                })
                .collect();
            alive[k].insert(b, lives);
        }
    }
    let mut found: Vec<u32> = Vec::new();
    for (k, groups) in minted.iter().enumerate() {
        for (b, gs) in *groups {
            let living: Vec<u32> = gs
                .iter()
                .zip(&alive[k][b])
                .filter(|&(_, &l)| l)
                .map(|(g, _)| g.size)
                .collect();
            if living.is_empty() {
                continue;
            }
            if super::emit_union::collapse_name(node, b).ok()? == *base {
                found = living;
            }
        }
    }
    Some(found)
}
