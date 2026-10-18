//! **Which geometry a pin can anchor on without reading a variable**
//! (stage 3 C, the orchestrator's anchor rule): [`pin_at`] and
//! [`place_at`] read a copy's pose off its own geometry against numbers
//! taken from one evaluation, so an anchor whose definition reaches a
//! document variable is pinned where that variable's nominal put it,
//! and a variable that moves it moves the copy back. An anchor whose
//! definition reaches none stands where the copy's variable-free
//! geometry stands at every value: the copy follows its variables as it
//! did before it was pinned, and under a parameter box the map is the
//! identity, which the snap places by no map.
//!
//! [`Anchor::Fixed`] is relative to the variables an analysis moves:
//! a free variable with a tolerance ([`crate::analysis::is_axis`]). A
//! typed number with none is a free variable too (Q1), but a constant
//! of every analysis (`analyzed_box`), so geometry that reads only
//! such numbers stands still under every box. The revalue instrument
//! moves untoleranced variables as well, so a `Fixed` anchor can still
//! move under it. The gap is the interim's alone: until D a
//! construction sits on a datum written in such numbers, and after D a
//! root extrude's start cap reads nothing at all.
//!
//! Decided from the recipe, never by measuring: what a name's geometry
//! reads is the node that minted it, read as narrowly as its role
//! allows (an extrude's start cap reads its profile's frame alone),
//! and every operation between that node and the copied body has to
//! carry the entity's carrier and points through unmoved.
//!
//! [`pin_at`]: super::pin::pin_at
//! [`place_at`]: super::pin::place_at

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

use crate::{CapEnd, RoleSeg, StableName};
use crate::{Node, ProfileDoc, ProfilePayload, RecipeNodeId, VarId};

/// **What a pin's anchor reads.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    /// Geometry whose definition reaches no document variable: the
    /// copy is pinned where it stands at every value.
    Fixed,
    /// Every anchor the copy offers reads a variable: the pin holds the
    /// nominal's numbers, and a variable that moves the anchor moves
    /// the copy back toward them
    /// (`work/intent/a-placed-copy-rests-at-the-identity-only-at-the-nominal.md`).
    Varying,
}

/// **The recipe's answer to which names read no variable**, over one
/// document and the node whose body is copied, memoized per variable
/// and per node.
pub(crate) struct Anchors<'a> {
    doc: &'a ProfileDoc,
    /// The nodes the copied body is built through, the copied node
    /// included.
    built_through: BTreeSet<RecipeNodeId>,
    vars: RefCell<BTreeMap<VarId, bool>>,
    nodes: RefCell<BTreeMap<RecipeNodeId, bool>>,
}

impl<'a> Anchors<'a> {
    /// The answers for names on the body `copied` defines.
    pub(crate) fn of(doc: &'a ProfileDoc, copied: RecipeNodeId) -> Self {
        let mut built_through = BTreeSet::from([copied]);
        let mut frontier = vec![copied];
        while let Some(at) = frontier.pop() {
            for up in doc.upstream(at) {
                if built_through.insert(up) {
                    frontier.push(up);
                }
            }
        }
        Self {
            doc,
            built_through,
            vars: RefCell::new(BTreeMap::new()),
            nodes: RefCell::new(BTreeMap::new()),
        }
    }

    /// **Whether `name`'s geometry reads no document variable**: the
    /// minting node's own reads, as narrowly as its role allows, and
    /// every operation between it and the copied body one that carries
    /// a surviving entity's carrier and points through unmoved.
    pub(crate) fn fixed(&self, name: &StableName) -> bool {
        self.carried_unmoved(name.node) && !self.minted_reads(name)
    }

    /// Every operation the copied body is built through after `minted`
    /// keeps a surviving entity where `minted` put it: a boolean, a
    /// blend, a split or a shell trims and adds, and moves nothing it
    /// keeps. A vertex such an operation trims (a blend's corner) is a
    /// new entity under the operation's own name, minted by it, and so
    /// is asked of that operation's reads, never of `minted`'s.
    fn carried_unmoved(&self, minted: RecipeNodeId) -> bool {
        if !self.built_through.contains(&minted) {
            return false;
        }
        self.built_through
            .iter()
            .filter(|&&at| at != minted && self.upstream_of(at).contains(&minted))
            .all(|&at| {
                matches!(
                    self.doc.node(at),
                    Some(
                        Node::Union { .. }
                            | Node::Intersect { .. }
                            | Node::Subtract { .. }
                            | Node::Fillet { .. }
                            | Node::Chamfer { .. }
                            | Node::Split { .. }
                            | Node::Shell { .. }
                    )
                )
            })
    }

    /// The nodes `at` is built through, `at` excluded.
    fn upstream_of(&self, at: RecipeNodeId) -> BTreeSet<RecipeNodeId> {
        let mut seen = BTreeSet::new();
        let mut frontier = vec![at];
        while let Some(node) = frontier.pop() {
            for up in self.doc.upstream(node) {
                if seen.insert(up) {
                    frontier.push(up);
                }
            }
        }
        seen
    }

    /// Whether the node that minted `name` reads a variable that moves
    /// it. An extrude's names read its profile's frame; all but its
    /// start cap read the profile's own numbers; those its path names
    /// the end of read its distance. Any other node's names read
    /// everything the node reads.
    fn minted_reads(&self, name: &StableName) -> bool {
        let Some(node) = self.doc.node(name.node) else {
            return true;
        };
        let Node::Extrude {
            profile, distance, ..
        } = node
        else {
            return self.node_reads(name.node);
        };
        let Some(sketch) = self.doc.operation_of(*profile) else {
            return true;
        };
        let Some(Node::Profile(program)) = self.doc.node(sketch) else {
            return self.node_reads(name.node);
        };
        let frame = program.frame_read().is_some_and(|f| self.var_reads(f));
        let segs = name.path.as_slice();
        let start_cap = matches!(segs, [RoleSeg::Cap(CapEnd::Start)]);
        let shape = !start_cap
            && self
                .doc
                .node(sketch)
                .is_some_and(|n| n.exprs().into_iter().any(|&v| self.var_reads(v)));
        let names_end = segs.iter().any(|seg| {
            matches!(
                seg,
                RoleSeg::Cap(CapEnd::End)
                    | RoleSeg::RimEdge(CapEnd::End, _)
                    | RoleSeg::CapVertex(CapEnd::End, _)
            )
        });
        let depth = names_end && self.var_reads(*distance);
        frame || shape || depth
    }

    /// Whether `node` reads a toleranced variable, through its slots or
    /// its operands, transitively.
    fn node_reads(&self, node: RecipeNodeId) -> bool {
        if let Some(&known) = self.nodes.borrow().get(&node) {
            return known;
        }
        // A cycle reads nothing new: answered `false` while it is open.
        self.nodes.borrow_mut().insert(node, false);
        let reads = self.doc.node(node).is_none_or(|n| {
            n.exprs().into_iter().any(|&v| self.var_reads(v))
                || n.operand_rows().into_iter().any(|(_, v)| self.var_reads(v))
        });
        self.nodes.borrow_mut().insert(node, reads);
        reads
    }

    /// Whether `var` is, or reaches through its definition, a free
    /// variable with a tolerance.
    fn var_reads(&self, var: VarId) -> bool {
        if let Some(&known) = self.vars.borrow().get(&var) {
            return known;
        }
        self.vars.borrow_mut().insert(var, false);
        let reads = if self.doc.var(var).is_none() {
            true
        } else if let Some(free) = self.doc.free(var) {
            crate::analysis::is_axis(free)
        } else if let Some(node) = self.doc.operation_of(var) {
            self.node_reads(node)
        } else {
            self.doc
                .definition_reads(var)
                .into_iter()
                .any(|v| self.var_reads(v))
        };
        self.vars.borrow_mut().insert(var, reads);
        reads
    }
}
