//! **The emitter's fragment groups, recorded beside the name table.**
//!
//! A fragment qualifier (N2) discriminates the members of ONE group:
//! the entities an op made from one parent. Which entities those are
//! is decided where the names are minted, from descent and geometry:
//! the boolean groups a result face by the operand face it descends
//! from, an edge by its operand edge, seam pair or joined set, a corner
//! held at several vertices by that corner, and the split groups a face
//! by its operand face and the side it landed on. The group's base
//! name is spelled from that key, and each member's name is the base,
//! alone or with one qualifier.
//!
//! The name alone cannot say which group a row belongs to. Two TIED
//! parents (N2) have one name, so their groups share a base and the
//! tie lane merges their members' rows; and a member can be spelled
//! without the base at all, as when a split no longer divides a face
//! and passes it through under its upstream name. So the emitter
//! records each group it formed ([`GroupRecord`]) — its base, its
//! members as ENTITIES of its output, and the operand entity they
//! descend from — and the diagnosis ladder's group-size rung
//! (`resolve::group_resized`) reads that record ([`FragmentGroups`])
//! rather than counting spellings. One membership rule, the emitter's,
//! with two readers: the qualifier the emitter mints, and the count the
//! rung reports. Membership is never matched by name; the one
//! name match is the rung's query, a union's fold-space bases collapsed
//! to the queried base, and two bases that collapse to one decline.
//!
//! What a group counts is the number of DISTINCT entities of the node's
//! output that descend from the group's parent within that group. For
//! a pair boolean or a split that is the group's own members, and so
//! it is for a union's faces, seam edges and vertices, which the union
//! groups over its finished body. For a union's other edges, and the
//! vertices whose fold-step names it keeps, it is the published body's
//! entities, followed from the fold step that formed the group through
//! every later step ([`FragmentGroups::folded`]).
//!
//! The record is diagnosis-time evidence about one run. It rides the
//! node's value like the verdict log does, and like the verdict log it
//! is never persisted and never part of a name: no name, no table and
//! no digest changes with it.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use super::role::StableName;
use super::table::EntityRef;
use crate::node::RecipeNodeId;

/// Where a group's members descend from, as the emitter that formed it
/// knows it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Parent {
    /// The A operand's entity: every member descends from that one
    /// operand entity. A union's fold reads it — each step's A operand
    /// is the previous step's output, so this is a key of that output —
    /// to carry a group's members forward ([`FragmentGroups::folded`]).
    AOperand(EntityRef),
    /// Anything else: a B-operand entity, a seam the op itself formed,
    /// a split's target face. No later fold step carries it.
    Elsewhere,
}

/// One group the emitter formed.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Group {
    /// Its members: entities of the emitting op's output.
    members: Vec<EntityRef>,
    /// Where they descend from.
    parent: Parent,
    /// Whether the group is one parent's share of a LINE's group: the
    /// edges a node holds on one line (N2), whichever parent edge on it
    /// each descends from, are one group, counted together
    /// ([`GroupRecord::record_on_line`]).
    on_line: bool,
}

/// The groups one emission formed, keyed by base name. Built by the
/// emitter that formed them and read through [`FragmentGroups`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct GroupRecord(BTreeMap<StableName, Vec<Group>>);

impl GroupRecord {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Records one group of one parent: its `base`, its `members` (the
    /// output's entities, each once), and where they descend from.
    pub(crate) fn record(&mut self, base: &StableName, members: Vec<EntityRef>, parent: Parent) {
        self.push(base, members, parent, false);
    }

    /// Records one parent edge's share of the group of the line `line`:
    /// its `members`, and where they descend from. Every share recorded
    /// under one line is counted as one group, so two untied parents on
    /// one line are one group and not two.
    pub(crate) fn record_on_line(
        &mut self,
        line: &StableName,
        members: Vec<EntityRef>,
        parent: Parent,
    ) {
        self.push(line, members, parent, true);
    }

    fn push(&mut self, base: &StableName, members: Vec<EntityRef>, parent: Parent, on_line: bool) {
        self.0.entry(base.clone()).or_default().push(Group {
            members,
            parent,
            on_line,
        });
    }
}

/// The groups a union forms over its finished body, and the vertex
/// bases it leaves to the fold's own groups instead.
///
/// A union's end pass names every vertex, but a row the fold tied at a
/// member corner keeps the fold's name, fragment qualifier and all, so
/// its group is the one the fold step formed. `emit_union`'s
/// `cite_member_edges` lists those rows' bases in `by_fold`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Rederived {
    /// The groups whose counts are on record.
    pub(crate) groups: GroupRecord,
    /// The vertex bases the fold's entity-followed groups answer for.
    pub(crate) by_fold: BTreeSet<StableName>,
}

/// One group's count, as the rung reads it: the distinct entities of
/// the node's output descended from its parent within it.
type Count = usize;

/// The groups a node's emission formed, as the diagnosis ladder reads
/// them (module docs). A consumer holds one on every node value and
/// can make an empty one; what it records reaches a consumer through
/// `Diagnosis::GroupResized`'s two counts, not through this type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FragmentGroups(Read);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Read {
    /// One emission's counts, by base.
    Minted(BTreeMap<StableName, Vec<Count>>),
    /// A union's counts: each fold step's, by FOLD-SPACE base, the
    /// descent already followed to the published body; the groups the
    /// union re-derives over the finished body, by published base; and
    /// the vertex bases left to the fold ([`Rederived`]).
    Folded {
        node: RecipeNodeId,
        steps: Vec<BTreeMap<StableName, Vec<Count>>>,
        published: BTreeMap<StableName, Vec<Count>>,
        by_fold: BTreeSet<StableName>,
    },
}

/// Each group's count in `record`: its distinct members.
fn counts(record: &GroupRecord) -> BTreeMap<StableName, Vec<Count>> {
    record
        .0
        .iter()
        .map(|(b, gs)| {
            let sets = gs.iter().map(|g| g.members.iter().copied().collect());
            (b.clone(), line_counts(gs, sets))
        })
        .collect()
}

/// The counts of the groups under one base, each group's `set` its
/// distinct entities: one count per group, but every share of a line's
/// group ([`Group::on_line`]) counted together, as one count of their
/// union, ahead of the others.
fn line_counts(
    groups: &[Group],
    sets: impl IntoIterator<Item = BTreeSet<EntityRef>>,
) -> Vec<Count> {
    let mut line: Option<BTreeSet<EntityRef>> = None;
    let mut rest = Vec::new();
    for (g, set) in groups.iter().zip(sets) {
        if g.on_line {
            line.get_or_insert_with(BTreeSet::new).extend(set);
        } else {
            rest.push(set.len());
        }
    }
    line.map(|set| set.len()).into_iter().chain(rest).collect()
}

/// Whether a union forms every group under `base` over its finished
/// body rather than by fold step: a face or a seam edge
/// (`emit_union`'s `name_by_parents`), or a vertex (its
/// `cite_member_edges`, which names every vertex of the finished body).
fn rederived(base: &StableName) -> bool {
    use super::role::{EntityKind, RoleSeg};
    match base.kind {
        EntityKind::Face | EntityKind::Vertex => true,
        EntityKind::Edge => matches!(base.path.first(), Some(RoleSeg::Seam { .. })),
        EntityKind::Body => false,
    }
}

impl Default for FragmentGroups {
    fn default() -> Self {
        Self(Read::Minted(BTreeMap::new()))
    }
}

impl FragmentGroups {
    /// No groups: a node whose op forms none.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// One emission's record, read in place: each group counts its own
    /// distinct members.
    pub(crate) fn minted(record: &GroupRecord) -> Self {
        Self(Read::Minted(counts(record)))
    }

    /// A record of one group per `(base, size)`, each of one parent, as
    /// an emitter that formed exactly those groups would leave it: for
    /// the resolve suites, which build a run by hand.
    #[cfg(any(test, feature = "test-support"))]
    #[must_use]
    pub fn from_sizes(groups: impl IntoIterator<Item = (StableName, usize)>) -> Self {
        let mut m: BTreeMap<StableName, Vec<Count>> = BTreeMap::new();
        for (base, size) in groups {
            m.entry(base).or_default().push(size);
        }
        Self(Read::Minted(m))
    }

    /// **A union's record**, from the groups each fold step formed, in
    /// fold order, under the union's id.
    ///
    /// The descent is followed HERE, by entity: step `k + 1`'s A operand
    /// is step `k`'s output, so a group of step `k + 1` whose parent is
    /// [`Parent::AOperand`]`(e)` holds what step `k + 1` made of step
    /// `k`'s entity `e`. Walking back from the last step, whose output is
    /// the published body, each entity of each step gets the SET of
    /// published entities it descends to, and each group counts the
    /// distinct union of its members' sets — so a later step that
    /// swallows a member drops it, one that divides a member counts each
    /// piece, and a later merged face holding two pieces of one group
    /// counts once. Tied parents are distinct entities, so each tied
    /// group counts its own.
    ///
    /// The descent is the emitter's own: an entity counts as carried
    /// exactly where the next step's emitter named a group after it.
    /// One it re-mints under a seam name of its own is not carried.
    ///
    /// Every step's rows are minted under the union's id in the fold's
    /// space, and the published table is their collapse
    /// (`emit_union`), so the counts stay keyed by fold-space base and
    /// [`FragmentGroups::sizes`] makes the collapse when it is asked: a
    /// name the collapse refuses is the ladder's to decline over, never
    /// the evaluation's to fail on.
    ///
    /// A union groups its faces, seam edges and vertices over the
    /// finished body, not by fold step: a face group per parent, a seam
    /// group per pair of parents, a vertex group per member corner, tied
    /// parents' and corners' too. `published` is the record of those
    /// groups, and it answers for every face, seam or vertex base
    /// except the vertex bases it leaves to the fold
    /// ([`FragmentGroups::sizes`]).
    pub(crate) fn folded(
        node: RecipeNodeId,
        steps: &[Arc<GroupRecord>],
        published: &Rederived,
    ) -> Self {
        let mut counted: Vec<BTreeMap<StableName, Vec<Count>>> = vec![BTreeMap::new(); steps.len()];
        // Each entity of the step after the current one → the published
        // entities it descends to.
        let mut reach: BTreeMap<EntityRef, BTreeSet<EntityRef>> = BTreeMap::new();
        for (k, step) in steps.iter().enumerate().rev() {
            let last = k + 1 == steps.len();
            let published = |m: &EntityRef, reach: &BTreeMap<EntityRef, BTreeSet<EntityRef>>| {
                if last {
                    BTreeSet::from([*m])
                } else {
                    reach.get(m).cloned().unwrap_or_default()
                }
            };
            // What this step made of each entity of the step before it.
            let mut carried: BTreeMap<EntityRef, BTreeSet<EntityRef>> = BTreeMap::new();
            for (b, groups) in &step.0 {
                let mut sets = Vec::with_capacity(groups.len());
                for g in groups {
                    let set: BTreeSet<EntityRef> = g
                        .members
                        .iter()
                        .flat_map(|m| published(m, &reach))
                        .collect();
                    if let Parent::AOperand(e) = g.parent {
                        carried.entry(e).or_default().extend(set.iter().copied());
                    }
                    sets.push(set);
                }
                counted[k].insert(b.clone(), line_counts(groups, sets));
            }
            reach = carried;
        }
        Self(Read::Folded {
            node,
            steps: counted,
            published: counts(&published.groups),
            by_fold: published.by_fold.clone(),
        })
    }

    /// Whether this is a union's record, read through its fold
    /// ([`FragmentGroups::folded`]): its names are in the union's
    /// member space, where a seam's sides are in name order.
    pub(crate) fn is_folded(&self) -> bool {
        matches!(self.0, Read::Folded { .. })
    }

    /// Each group's count under `base`, one per group (two tied parents
    /// give two), empty when no group is. `None` when no one parent's
    /// count is on record: a union's base the collapse refuses (a
    /// junction whose lines it collides), or two of one fold step's
    /// bases that publish as one.
    ///
    /// A union's base can be spelled by more than one step: a step that
    /// leaves an entity whole spells a group of one, and a later step
    /// that divides it spells the group of its pieces under the same
    /// collapsed base. Both count the same published entities, so the
    /// latest step that spells the base answers. A face, seam or vertex
    /// base is answered by the groups re-derived over the finished body,
    /// except the vertex bases the union leaves to the fold
    /// ([`Rederived::by_fold`]).
    pub(crate) fn sizes(&self, base: &StableName) -> Option<Vec<usize>> {
        let counts = match &self.0 {
            Read::Minted(m) => m.get(base).cloned().unwrap_or_default(),
            Read::Folded {
                published, by_fold, ..
            } if rederived(base) && !by_fold.contains(base) => {
                published.get(base).cloned().unwrap_or_default()
            }
            Read::Folded { node, steps, .. } => {
                let mut found = None;
                for step in steps.iter().rev() {
                    let mut hit: Option<&Vec<Count>> = None;
                    for (b, counts) in step {
                        if super::emit_union::collapse_name(*node, b).ok()? != *base {
                            continue;
                        }
                        if hit.is_some() {
                            return None;
                        }
                        hit = Some(counts);
                    }
                    if let Some(c) = hit {
                        found = Some(c.clone());
                        break;
                    }
                }
                found.unwrap_or_default()
            }
        };
        Some(counts)
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::names::role::{CapEnd, EntityKind, RoleSeg};
    use crate::names::table::{EntityKey, EntityRef};

    /// Member 1's vertex, as a union `node` keys it.
    fn corner(union: RecipeNodeId) -> StableName {
        StableName {
            kind: EntityKind::Vertex,
            node: union,
            path: vec![RoleSeg::From {
                read: crate::VarId::new(1, 1),
                of: StableName {
                    kind: EntityKind::Vertex,
                    node: RecipeNodeId::new(0, 1),
                    path: vec![RoleSeg::Cap(CapEnd::Start)],
                }
                .into(),
            }],
        }
    }

    fn vertex(k: u64) -> EntityRef {
        EntityRef {
            body: 0,
            key: EntityKey::Vertex(topo::VertexKey::from(slotmap::KeyData::from_ffi(k))),
        }
    }

    /// A union's vertex base reads the group its end pass formed, so one
    /// member order whose fold step spelled the base and one whose step
    /// did not read alike; a base the end pass leaves to the fold reads
    /// the fold's group.
    #[test]
    fn a_unions_vertex_base_reads_the_end_pass_in_every_fold() {
        let union = RecipeNodeId::new(0, 9);
        let base = corner(union);
        let mut published = Rederived::default();
        published
            .groups
            .record(&base, vec![vertex(1)], Parent::Elsewhere);
        let mut spelled = GroupRecord::new();
        spelled.record(&base, vec![vertex(1), vertex(2)], Parent::Elsewhere);
        let spelled = [Arc::new(spelled)];
        let unspelled = [Arc::new(GroupRecord::new())];
        for (what, steps) in [("spelled", &spelled), ("unspelled", &unspelled)] {
            let read = FragmentGroups::folded(union, steps, &published);
            assert_eq!(read.sizes(&base), Some(vec![1]), "{what}");
        }
        published.by_fold.insert(base.clone());
        let read = FragmentGroups::folded(union, &spelled, &published);
        assert_eq!(read.sizes(&base), Some(vec![2]), "left to the fold");
    }
}
