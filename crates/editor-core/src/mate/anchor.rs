//! **Which geometry a pin can anchor on without reading a variable**:
//! [`pin_at`], [`place_at`] and [`stand`] read a copy's pose off its own
//! geometry against numbers taken from one evaluation, so an anchor
//! whose definition reaches a document variable is pinned where that
//! variable's nominal put it, and a variable that moves it moves the
//! copy back. An anchor whose definition reaches none stands where the
//! copy's variable-free geometry stands at every value: the copy
//! follows its variables, and under a parameter box the map is the
//! identity, which the snap places by no map. A root extrude's or
//! loft's start cap lies on its first profile's plane and reads nothing.
//!
//! A typed number with no tolerance is a document variable too (Q1,
//! VR6): the revalue instrument moves it, so an anchor reading one is
//! not fixed. Only the constant `0` reads nothing.
//!
//! Decided from the recipe, never by measuring: what a name's geometry
//! reads is the node that minted it, read as narrowly as its role
//! allows (an extrude's start cap reads nothing), and every operation
//! between that node and the copied body has to carry the entity's
//! carrier and points through unmoved.
//!
//! [`pin_at`]: super::pin::pin_at
//! [`place_at`]: super::pin::place_at
//! [`stand`]: super::pin::stand

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

use crate::{CapEnd, RoleSeg, StableName};
use crate::{Node, ProfileDoc, RecipeNodeId, VarId};

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
        // A piece of an entity lies on the entity's carrier.
        let whole = &name.path[..crate::names::role::fragment_tail_start(&name.path)];
        match whole {
            // An operation that carries the entity unmoved names it by
            // the name it had: a member's, a split target's, or each of
            // the coplanar faces it merged.
            [RoleSeg::From { of, .. }] | [RoleSeg::SplitFragment { parent: of }] => self.fixed(of),
            [RoleSeg::Merged(merged)] => merged.iter().any(|n| self.fixed(n)),
            // A copy's entity stands where its placement moved it, and a
            // pattern's or an instance's where its rule did.
            [RoleSeg::Placed { .. } | RoleSeg::Instance { .. } | RoleSeg::InPart { .. }, ..] => {
                false
            }
            _ => self.carried_unmoved(name.node) && !self.minted_reads(name),
        }
    }

    /// **The nodes the copied body is built through that every later
    /// operation carries unmoved** ([`Self::carried_unmoved`]), in id
    /// order: where a construction's own pose (a revolve's axis, a
    /// ring's carrier axis) is still the copied body's.
    pub(crate) fn unmoved(&self) -> impl Iterator<Item = RecipeNodeId> + '_ {
        self.built_through
            .iter()
            .copied()
            .filter(|&at| self.carried_unmoved(at))
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
            .filter(|&&at| at != minted && self.carries(at, minted))
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

    /// Whether `at` is built through `minted`'s geometry: a placement
    /// through its shapes (a mate's target is read, never carried), any
    /// other node through everything upstream of it.
    fn carries(&self, at: RecipeNodeId, minted: RecipeNodeId) -> bool {
        match self.doc.node(at) {
            Some(Node::Place { shapes, .. }) => shapes.iter().any(|shape| {
                self.doc.operation_of(shape.read).is_some_and(|of| {
                    of == minted || self.upstream_of(of).contains(&minted)
                })
            }),
            _ => self.upstream_of(at).contains(&minted),
        }
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
    /// it. An extrude's and a loft's start cap read nothing; an
    /// extrude's other names read the profile's own numbers, and those
    /// its path names the end of read its distance. Any other node's
    /// names read everything the node reads.
    fn minted_reads(&self, name: &StableName) -> bool {
        let Some(node) = self.doc.node(name.node) else {
            return true;
        };
        // A loft's start cap lies on its first section's plane, its own
        // `xy`, whatever its sections and stations read.
        if matches!(node, Node::Loft { .. })
            && matches!(name.path.as_slice(), [RoleSeg::Cap(CapEnd::Start)])
        {
            return false;
        }
        let Node::Extrude {
            profile, distance, ..
        } = node
        else {
            return self.node_reads(name.node);
        };
        let Some(sketch) = self.doc.operation_of(*profile) else {
            return true;
        };
        if !matches!(self.doc.node(sketch), Some(Node::Profile(_))) {
            return self.node_reads(name.node);
        }
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
        shape || depth
    }

    /// Whether `node` reads a document variable, through its slots or
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
    /// variable.
    fn var_reads(&self, var: VarId) -> bool {
        if let Some(&known) = self.vars.borrow().get(&var) {
            return known;
        }
        self.vars.borrow_mut().insert(var, false);
        let reads = if self.doc.var(var).is_none() {
            true
        } else if self.doc.free(var).is_some() {
            true
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
