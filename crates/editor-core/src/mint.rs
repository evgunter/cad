//! **Where a node's id, a profile step's id and a variable's id come
//! from** (`names/README.md`, N1 and "N1, the profile pieces", "The
//! id."; VARIABLES-DESIGN VR1): the document's mint, one chain and one
//! log for all three.
//!
//! The chain is a SHA-256 digest the document carries. A minting edit
//! extends it by that edit's canonical bytes ([`MintingEdit`]); an
//! `InsertNode` then extends it once more for the node it mints, and
//! each step either edit mints extends it once more again. An id is the
//! first 64 bits of the chain at the point that minted it, big-endian.
//! An edit that mints nothing leaves the chain alone. So an id is a
//! function of the minting edits that led to it: one sequence mints one
//! set of ids (D9), and two sequences that part from one value mint
//! different ids from there on. A `DeclareVar` extends the chain by the
//! variable's kind and then once for the variable it mints; its name is
//! not in the preimage (VR2), so two declares of one kind mint two ids
//! only because the chain moved between them. Each anonymous variable an
//! edit's lowering mints extends it by its kind and what it holds
//! ([`MintingEdit::DeclareAnonymous`], [`Held`]: a value by its bits, a
//! definition with its literals canonical), so two sibling inserts that
//! differ only in a value they hold mint two nodes; a display unit is in
//! neither preimage (D6).
//!
//! The log holds every id the document has minted, deleted nodes' and
//! dropped steps' included, each tagged with what it names ([`Minted`]),
//! ascending by id. A mint whose id the log already holds, under either
//! tag, is refused; the doors that write a name, and the load door,
//! refuse a node or a step the log does not hold as that; and the load
//! door refuses a log that is not strictly ascending
//! (`SnapshotError::MintLogOrder`).
//!
//! The preimage is not [`crate::persist::canonical_bytes`]: that is a
//! whole document's serde form, display units included, and answers
//! "which version"; this is one edit's statement, and answers "which
//! node" or "which step". An insert's statement is its node as the door
//! stores it, which holds its inputs', its names' and its slot
//! variables' ids but never its own, and so no float and no display
//! unit (D6).

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest, Sha256};

use crate::node::{Node, RecipeNodeId, StepId};
use crate::program::{LoopProgram, StepIdFault};
use crate::var::{VarId, VarKind};

/// The document's mint: the chain the next minting edit extends and
/// the log of every id minted so far.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mint {
    /// The chain digest, as 64 lowercase hex digits on the wire.
    #[serde(with = "chain_hex")]
    chain: [u8; 32],
    /// Every id minted, strictly ascending by id. Read as written, so
    /// the load door can refuse a log out of order rather than repair
    /// it.
    log: Vec<Minted>,
}

/// **One entry of the mint log**: an id, and whether it was minted for
/// a node or for a profile step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Minted {
    /// A node's id, minted by `InsertNode`.
    Node(RecipeNodeId),
    /// A profile step's id, minted by `InsertNode` or `SetProgram`.
    Step(StepId),
    /// A variable's id, minted by `DeclareVar`.
    Var(VarId),
}

impl Minted {
    /// The id's bits, the log's order.
    #[must_use]
    pub fn bits(self) -> u64 {
        match self {
            Self::Node(id) => id.0,
            Self::Step(step) => step.0,
            Self::Var(var) => var.0,
        }
    }
}

/// `node <tag>`, `step <tag>` or `variable <tag>`: the entry as a
/// sentence reads it.
impl core::fmt::Display for Minted {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Node(id) => write!(f, "node {id}"),
            Self::Step(step) => write!(f, "step {step}"),
            Self::Var(var) => write!(f, "variable {var}"),
        }
    }
}

/// Why the mint refused a node id; the insert door reports it as
/// [`crate::EditError::NodeIdCollides`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NodeIdCollides {
    /// The id the insert drew, which the log already holds.
    pub(crate) id: RecipeNodeId,
}

/// Why the mint refused a variable id; the declare door reports it as
/// [`crate::EditError::VarIdCollides`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct VarIdCollides {
    /// The id the declare drew, which the log already holds.
    pub(crate) id: VarId,
}

/// **A minting edit**, as the mint reads it: the node an `InsertNode`
/// inserts, as stored; what a `SetProgram` states about the steps it
/// authors — its node, the new loops and, per step, the id it keeps or
/// `None` for one to mint; or the kind a variable is minted at.
///
/// Its canonical bytes are the serde form of that statement.
#[derive(Serialize)]
#[serde(bound(serialize = "P: Serialize, Node<P, S>: Serialize"))]
pub(crate) enum MintingEdit<'a, P, S: crate::Slot = crate::VarId> {
    /// A node inserted, as the edit states it.
    InsertNode {
        /// The node.
        node: Box<Node<P, S>>,
    },
    /// A profile's program replaced.
    SetProgram {
        /// The profile.
        node: RecipeNodeId,
        /// The new loops.
        loops: Vec<LoopProgram<S>>,
        /// The kept ids.
        ids: &'a [Vec<Option<StepId>>],
    },
    /// A variable declared: its KIND, and nothing else. Not its name
    /// (VR2: a name is a label, in neither an id's mint nor a content
    /// key), and not its definition: a nominal or an annotation in the
    /// preimage would make every id minted after the declare, and every
    /// verdict keyed by one, move with a value — and a distribution
    /// enters no evaluation, no content key and no predicate. Two
    /// declares of one kind still mint two ids, because the chain
    /// extends.
    DeclareVar {
        /// The kind.
        kind: VarKind,
    },
    /// An anonymous variable minted by the edit door for a value or a
    /// formula written at a slot, or for a fresh-table entry: its kind
    /// and what it holds. Unlike a declare's, the value is in the
    /// preimage: an anonymous variable IS the value a slot was written
    /// as, so two inserts applied to one base that write different
    /// values mint two variables, and so two nodes
    /// (`work/emit/sibling-branches-mint-one-node-id-for-different-nodes.md`),
    /// as they did when the value sat in the node's own bytes. Not its
    /// distribution or its notation, which enter no evaluation. A value
    /// edit afterwards mints nothing: the identity drawn here stays.
    DeclareAnonymous {
        /// The kind.
        kind: VarKind,
        /// What it holds.
        held: Held,
    },
}

/// **What an anonymous variable holds, as its mint reads it**
/// ([`MintingEdit::DeclareAnonymous`]): a continuous value's bits, a
/// count, or a definition. A display unit is never in it (D6): a
/// value's carries none, and a definition holds no float, its written
/// quantities being variables minted before it.
#[derive(Serialize)]
pub(crate) enum Held {
    /// A continuous value, by its bits.
    Value(u64),
    /// A count.
    Count(i64),
    /// A definition.
    Defined(crate::Expr),
}

impl Held {
    /// What `def` holds.
    pub(crate) fn of(def: &crate::var::VarDef) -> Self {
        match def {
            crate::var::VarDef::Free(crate::doc::FreeVar::Continuous { value, .. }) => {
                Self::Value(value.to_bits())
            }
            crate::var::VarDef::Free(crate::doc::FreeVar::Count { value }) => Self::Count(*value),
            crate::var::VarDef::Defined(expr) => Self::Defined(expr.clone()),
        }
    }
}

impl<'a, P: Serialize + Clone + crate::program::SlotPayload<S>, S: crate::Slot>
    MintingEdit<'a, P, S>
{
    /// The insert of `node`.
    fn insert(node: &Node<P, S>) -> Self {
        Self::InsertNode {
            node: Box::new(node.clone()),
        }
    }
}

impl<'a> MintingEdit<'a, crate::program::ProfileProgram> {
    /// The `SetProgram` of `loops` on `node`.
    fn set_program(
        node: RecipeNodeId,
        loops: &[LoopProgram],
        ids: &'a [Vec<Option<StepId>>],
    ) -> Self {
        Self::SetProgram {
            node,
            loops: loops.to_vec(),
            ids,
        }
    }
}

impl<P: Serialize, S: crate::Slot> MintingEdit<'_, P, S>
where
    Node<P, S>: Serialize,
{
    /// The bytes the chain is extended by: this statement's serde form.
    fn canonical_bytes(&self) -> Vec<u8> {
        // Every field is a derived-`Serialize` struct, enum, integer,
        // finite float or string, and no map is keyed by anything but a
        // string, so serde_json has no error to return here. Written in
        // the names' writing door, so a name nested any depth costs the
        // stack one level.
        crate::names::write_door(|| serde_json::to_vec(self))
            .unwrap_or_else(|e| unreachable!("a mint preimage always encodes: {e}"))
    }
}

const EDIT_TAG: &[u8] = b"mint/edit\0";
const NODE_TAG: &[u8] = b"mint/node\0";
const STEP_TAG: &[u8] = b"mint/step\0";
const VAR_TAG: &[u8] = b"mint/var\0";

impl Mint {
    /// A document's mint before anything is inserted: the zero chain
    /// and an empty log.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            chain: [0; 32],
            log: Vec::new(),
        }
    }

    /// The chain digest the next minting edit extends.
    #[cfg(test)]
    #[must_use]
    pub(crate) fn chain(&self) -> [u8; 32] {
        self.chain
    }

    /// Every id the document has minted, ascending by id.
    #[cfg(test)]
    #[must_use]
    pub(crate) fn log(&self) -> &[Minted] {
        &self.log
    }

    /// Every step id the document has minted, ascending.
    pub fn steps(&self) -> impl Iterator<Item = StepId> + '_ {
        self.log.iter().filter_map(|entry| match *entry {
            Minted::Step(step) => Some(step),
            Minted::Node(_) | Minted::Var(_) => None,
        })
    }

    /// Every node id the document has minted, ascending.
    #[cfg(test)]
    pub(crate) fn nodes(&self) -> impl Iterator<Item = RecipeNodeId> + '_ {
        self.log.iter().filter_map(|entry| match *entry {
            Minted::Node(id) => Some(id),
            Minted::Step(_) | Minted::Var(_) => None,
        })
    }

    fn holds(&self, entry: Minted) -> bool {
        self.log
            .binary_search_by_key(&entry.bits(), |e| e.bits())
            .is_ok_and(|at| self.log[at] == entry)
    }

    /// Whether the document minted `id` as a node's id: what
    /// [`crate::Doc::has_minted`] answers.
    #[must_use]
    pub(crate) fn has_node(&self, id: RecipeNodeId) -> bool {
        self.holds(Minted::Node(id))
    }

    /// Whether the document minted `step` as a step's id.
    #[must_use]
    pub fn has_step(&self, step: StepId) -> bool {
        self.holds(Minted::Step(step))
    }

    /// Whether the document minted `var` as a variable's id.
    #[must_use]
    pub fn has_var(&self, var: VarId) -> bool {
        self.holds(Minted::Var(var))
    }

    /// Every variable id the document has minted, ascending.
    pub fn vars(&self) -> impl Iterator<Item = VarId> + '_ {
        self.log.iter().filter_map(|entry| match *entry {
            Minted::Var(var) => Some(var),
            Minted::Node(_) | Minted::Step(_) => None,
        })
    }

    /// **The id a declare of a `kind` variable draws here**, and the
    /// chain it leaves: the chain extended by the declare's canonical
    /// bytes (its kind alone, [`MintingEdit::DeclareVar`]), then once
    /// for the variable. Reads; mints nothing.
    fn draw_var(&self, kind: VarKind) -> ([u8; 32], VarId) {
        self.draw_var_of(&MintingEdit::<()>::DeclareVar { kind })
    }

    /// The id the minting statement `edit` draws for its variable here,
    /// and the chain it leaves.
    fn draw_var_of(&self, edit: &MintingEdit<'_, ()>) -> ([u8; 32], VarId) {
        let (chain, bits) = Self::draw(VAR_TAG, self.extended(edit));
        (chain, VarId(bits))
    }

    /// **The id an anonymous variable holding `def` draws here**: the
    /// chain extended by its kind and what it holds
    /// ([`MintingEdit::DeclareAnonymous`]), then once for the variable.
    fn draw_anonymous(&self, def: &crate::var::VarDef) -> ([u8; 32], VarId) {
        self.draw_var_of(&MintingEdit::DeclareAnonymous {
            kind: def.kind(),
            held: Held::of(def),
        })
    }

    /// The id an anonymous variable holding `def` would mint here —
    /// what a refusal of it speaks. Reads; mints nothing.
    #[must_use]
    pub(crate) fn would_declare_anonymous(&self, def: &crate::var::VarDef) -> VarId {
        self.draw_anonymous(def).1
    }

    /// **Mint the id of an anonymous variable holding `def`**
    /// ([`Self::draw_anonymous`]). On a refusal `self` is untouched.
    ///
    /// # Errors
    ///
    /// [`VarIdCollides`] where the log already holds the id.
    pub(crate) fn declare_anonymous(
        &mut self,
        def: &crate::var::VarDef,
    ) -> Result<VarId, VarIdCollides> {
        let (chain, id) = self.draw_anonymous(def);
        let mut log = self.log.clone();
        Self::log_new(&mut log, Minted::Var(id)).map_err(|_| VarIdCollides { id })?;
        self.chain = chain;
        self.log = log;
        Ok(id)
    }

    /// The id a declare of a `kind` variable would mint here — what a
    /// refused declare's refusal speaks. Reads; mints nothing.
    #[must_use]
    pub(crate) fn would_declare(&self, kind: VarKind) -> VarId {
        self.draw_var(kind).1
    }

    /// **Mint the id of a declared `kind` variable** ([`Self::draw_var`]).
    /// On a refusal `self` is untouched.
    ///
    /// # Errors
    ///
    /// [`VarIdCollides`] where the log already holds the id.
    pub(crate) fn declare(&mut self, kind: VarKind) -> Result<VarId, VarIdCollides> {
        let (chain, id) = self.draw_var(kind);
        let mut log = self.log.clone();
        Self::log_new(&mut log, Minted::Var(id)).map_err(|_| VarIdCollides { id })?;
        self.chain = chain;
        self.log = log;
        Ok(id)
    }

    /// This mint with `entries` logged too, where a test pushes nodes
    /// by hand rather than through the insert door.
    #[cfg(test)]
    pub(crate) fn logged(mut self, entries: impl IntoIterator<Item = Minted>) -> Self {
        for entry in entries {
            // A test's hand-chosen id the log holds already stays once.
            let _ = Self::log_new(&mut self.log, entry);
        }
        self
    }

    /// The first log entry whose id is not greater than the one before
    /// it — a repeat or a step down — `None` for a strictly ascending
    /// log.
    pub(crate) fn out_of_order(&self) -> Option<Minted> {
        self.log
            .windows(2)
            .find(|pair| pair[0].bits() >= pair[1].bits())
            .map(|pair| pair[1])
    }

    /// The chain extended by `edit`'s canonical bytes.
    fn extended<P: Serialize, S: crate::Slot>(&self, edit: &MintingEdit<'_, P, S>) -> [u8; 32]
    where
        Node<P, S>: Serialize,
    {
        Sha256::new()
            .chain_update(EDIT_TAG)
            .chain_update(self.chain)
            .chain_update(edit.canonical_bytes())
            .finalize()
            .into()
    }

    /// The next digest along `chain` under `tag`, and the id it gives.
    fn draw(tag: &[u8], chain: [u8; 32]) -> ([u8; 32], u64) {
        let chain: [u8; 32] = Sha256::new()
            .chain_update(tag)
            .chain_update(chain)
            .finalize()
            .into();
        let mut head = [0u8; 8];
        head.copy_from_slice(&chain[..8]);
        (chain, u64::from_be_bytes(head))
    }

    /// Logs `entry`, or hands it back where the log holds its id
    /// already.
    fn log_new(log: &mut Vec<Minted>, entry: Minted) -> Result<(), Minted> {
        match log.binary_search_by_key(&entry.bits(), |e| e.bits()) {
            Ok(_) => Err(entry),
            Err(at) => {
                log.insert(at, entry);
                Ok(())
            }
        }
    }

    /// **The id an insert of `node` mints**: the chain extended by the
    /// node's canonical bytes, then once for the node. The steps the
    /// node's profile authors, if any, are minted next, by
    /// [`Self::steps_of_insert`]. On a refusal `self` is untouched.
    ///
    /// # Errors
    ///
    /// [`NodeIdCollides`] where the log already holds the id.
    pub(crate) fn insert<P: Serialize + Clone + crate::program::SlotPayload<S>, S: crate::Slot>(
        &mut self,
        node: &Node<P, S>,
    ) -> Result<RecipeNodeId, NodeIdCollides>
    where
        Node<P, S>: Serialize,
    {
        let (chain, bits) = Self::draw(NODE_TAG, self.extended(&MintingEdit::insert(node)));
        let id = RecipeNodeId(bits);
        let mut log = self.log.clone();
        Self::log_new(&mut log, Minted::Node(id)).map_err(|_| NodeIdCollides { id })?;
        self.chain = chain;
        self.log = log;
        Ok(id)
    }

    /// **The ids for the steps of the profile [`Self::insert`] just
    /// minted a node for**: one per authored step, `count` in all, in
    /// loop then step order, each extending the chain once. On a
    /// refusal `self` is untouched.
    ///
    /// # Errors
    ///
    /// [`StepIdFault::Collides`] where an id is already in the log.
    pub(crate) fn steps_of_insert(
        &mut self,
        shape: &[Vec<Option<StepId>>],
    ) -> Result<Vec<Vec<StepId>>, StepIdFault> {
        self.fill(self.chain, shape)
    }

    /// **The ids for a `SetProgram`'s steps**: `shape` is one list per
    /// loop, one entry per authored step, the id a step keeps or `None`
    /// for one to mint. The chain is extended by `edit`'s bytes, and
    /// each `None` is minted from it in loop then step order; with no
    /// `None` nothing moves. On a refusal `self` is untouched.
    ///
    /// # Errors
    ///
    /// [`StepIdFault::Collides`] where an id is already in the log (or
    /// minted twice by this edit).
    pub(crate) fn set_program(
        &mut self,
        node: RecipeNodeId,
        loops: &[LoopProgram],
        shape: &[Vec<Option<StepId>>],
    ) -> Result<Vec<Vec<StepId>>, StepIdFault> {
        if shape.iter().flatten().all(Option::is_some) {
            return Ok(shape
                .iter()
                .map(|lp| lp.iter().flatten().copied().collect())
                .collect());
        }
        self.fill(
            self.extended(&MintingEdit::set_program(node, loops, shape)),
            shape,
        )
    }

    /// Mints each `None` of `shape` along `chain`, extending the log.
    fn fill(
        &mut self,
        mut chain: [u8; 32],
        shape: &[Vec<Option<StepId>>],
    ) -> Result<Vec<Vec<StepId>>, StepIdFault> {
        let mut log = self.log.clone();
        let mut ids = Vec::with_capacity(shape.len());
        for lp in shape {
            let mut out = Vec::with_capacity(lp.len());
            for kept in lp {
                let step = match *kept {
                    Some(step) => step,
                    None => {
                        let (next, bits) = Self::draw(STEP_TAG, chain);
                        chain = next;
                        let step = StepId(bits);
                        Self::log_new(&mut log, Minted::Step(step))
                            .map_err(|_| StepIdFault::Collides { step })?;
                        step
                    }
                };
                out.push(step);
            }
            ids.push(out);
        }
        self.chain = chain;
        self.log = log;
        Ok(ids)
    }
}

mod chain_hex {
    use super::{Deserialize, Deserializer, Serializer};
    use serde::de::Error as _;

    pub(super) fn serialize<S: Serializer>(chain: &[u8; 32], ser: S) -> Result<S::Ok, S::Error> {
        crate::persist::hexbytes::serialize(chain, ser)
    }

    pub(super) fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<[u8; 32], D::Error> {
        let text = String::deserialize(de)?;
        let refuse = || {
            D::Error::custom(format!(
                "a mint chain is exactly 64 lowercase hex digits, got {text:?}"
            ))
        };
        let digit = |b: u8| match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            _ => None,
        };
        if text.len() != 64 {
            return Err(refuse());
        }
        let mut chain = [0u8; 32];
        for (byte, pair) in chain.iter_mut().zip(text.as_bytes().chunks_exact(2)) {
            let (Some(hi), Some(lo)) = (digit(pair[0]), digit(pair[1])) else {
                return Err(refuse());
            };
            *byte = (hi << 4) | lo;
        }
        Ok(chain)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::{Mint, Minted, NodeIdCollides, VarIdCollides};
    use crate::node::{Node, RecipeNodeId, StepId};
    use crate::program::{LoopProgram, ProfileProgram, StepIdFault};
    use crate::var::{VarId, VarKind};

    fn extrude(profile: u64, distance: f64) -> Node<ProfileProgram> {
        Node::Extrude {
            profile: RecipeNodeId(profile),
            distance: VarId(distance.to_bits()),
            side: crate::ExtrudeSide::Along,
        }
    }

    fn new(count: usize) -> Vec<Vec<Option<StepId>>> {
        vec![vec![None; count]]
    }

    fn flat(ids: Vec<Vec<StepId>>) -> Vec<StepId> {
        ids.into_iter().flatten().collect()
    }

    #[test]
    fn one_sequence_mints_one_set_of_ids_and_two_sequences_part() {
        let mut a = Mint::empty();
        let mut b = Mint::empty();
        let first_a = a.insert(&extrude(1, 2.0)).unwrap();
        let first_b = b.insert(&extrude(1, 2.0)).unwrap();
        assert_eq!(first_a, first_b, "one edit from one mint mints one id");
        assert_eq!(a, b, "and leaves one mint");
        let second_a = a.insert(&extrude(1, 3.0)).unwrap();
        let second_b = b.insert(&extrude(1, 4.0)).unwrap();
        assert_ne!(
            second_a, second_b,
            "two edits from one mint mint different ids"
        );
        let again = a.insert(&extrude(1, 3.0)).unwrap();
        assert_ne!(
            again, second_a,
            "one edit twice mints two ids: the chain moved"
        );
    }

    /// The display unit a slot is written in enters neither the
    /// anonymous variable's preimage (its kind and what it holds, a
    /// value by its bits and a definition with its literals canonical:
    /// [`Held`]) nor the node's (its variables' ids). So one point
    /// written in millimetres and in metres mints one id, and so does
    /// one written `w + 125 mm` and `w + 0.125 m` (D6).
    #[test]
    fn the_display_unit_is_not_part_of_what_an_insert_hashes() {
        let base = || {
            crate::edit::apply(
                &crate::ProfileDoc::empty_derived("mint_units", geom_core::Tol::witness()),
                &crate::DocEdit::DeclareVar {
                    name: crate::VarName::from_static("w"),
                    def: crate::VarDecl::Free(crate::FreeVar::continuous(
                        crate::Dimension::Length,
                        1.0,
                    )),
                },
                geom_core::Tol::witness(),
                &crate::RefusingReach,
            )
            .unwrap()
            .doc
        };
        let point = |position: [crate::Formula; 3]| -> RecipeNodeId {
            let doc = base();
            let applied = crate::edit::apply(
                &doc,
                &crate::DocEdit::InsertNode {
                    node: Box::new(Node::Datum(crate::node::Datum::Point { position })),
                    fresh: Vec::new(),
                },
                geom_core::Tol::witness(),
                &crate::RefusingReach,
            )
            .unwrap();
            applied.record.minted.unwrap()
        };
        let mm = [2.0, 3.0, 4.0].map(|v| crate::Formula::length_in(v, quantity::MM).unwrap());
        let m = [2.0, 3.0, 4.0].map(|v| crate::test_support::len(v / 1000.0));
        assert_eq!(
            point(mm),
            point(m),
            "one point in two units mints one id (D6)"
        );
        let w =
            || crate::Formula::named(crate::VarName::from_static("w"), crate::Dimension::Length);
        let plus = |offset: crate::Formula| {
            let x = crate::Formula::add(w(), offset).unwrap();
            [
                x,
                crate::test_support::len(0.0),
                crate::test_support::len(0.0),
            ]
        };
        assert_eq!(
            point(plus(
                crate::Formula::length_in(125.0, quantity::MM).unwrap()
            )),
            point(plus(crate::test_support::len(0.125))),
            "one definition in two units mints one id (D6)"
        );
    }

    #[test]
    fn a_profile_insert_mints_its_node_then_its_steps_on_one_chain() {
        let mut m = Mint::empty();
        let node = m.insert(&extrude(1, 2.0)).unwrap();
        let ids = flat(m.steps_of_insert(&new(2)).unwrap());
        assert_eq!(m.log().len(), 3, "the node and its two steps");
        assert!(m.has_node(node) && !m.has_step(StepId(node.0)));
        assert!(
            ids.iter()
                .all(|s| m.has_step(*s) && !m.has_node(RecipeNodeId(s.0)))
        );
        assert_eq!(m.nodes().collect::<Vec<_>>(), vec![node]);
    }

    #[test]
    fn kept_ids_pass_through_and_only_new_steps_mint() {
        let mut m = Mint::empty();
        let kept = StepId(7);
        let shape = vec![vec![Some(kept), None], vec![None]];
        let ids = m.set_program(RecipeNodeId(1), &[], &shape).unwrap();
        assert_eq!(ids[0][0], kept, "a kept id is handed back where it stood");
        assert_eq!(
            m.log().len(),
            2,
            "the two new steps, and only they, are logged"
        );
        assert!(m.has_step(ids[0][1]) && m.has_step(ids[1][0]));
    }

    #[test]
    fn a_set_program_that_mints_nothing_leaves_the_chain() {
        let mut m = Mint::empty();
        let kept = vec![vec![Some(StepId(3))]];
        assert_eq!(
            flat(m.set_program(RecipeNodeId(1), &[], &kept).unwrap()),
            vec![StepId(3)]
        );
        assert_eq!(m, Mint::empty());
    }

    #[test]
    fn a_mint_the_log_holds_refuses_and_moves_nothing() {
        let node = Mint::empty().insert(&extrude(1, 2.0)).unwrap();
        // A log that already holds the id the insert would mint, as a
        // step's: one log, so the tag does not let it through.
        let taken = Mint::empty().logged([Minted::Step(StepId(node.0))]);
        let mut m = taken.clone();
        assert_eq!(m.insert(&extrude(1, 2.0)), Err(NodeIdCollides { id: node }));
        assert_eq!(m, taken, "a refused mint moves neither chain nor log");

        let mut probe = Mint::empty();
        let steps = flat(probe.set_program(RecipeNodeId(1), &[], &new(2)).unwrap());
        let taken = Mint::empty().logged([Minted::Node(RecipeNodeId(steps[1].0))]);
        let mut m = taken.clone();
        assert_eq!(
            m.set_program(RecipeNodeId(1), &[], &new(2)),
            Err(StepIdFault::Collides { step: steps[1] })
        );
        assert_eq!(m, taken, "a refused mint moves neither chain nor log");
    }

    /// **The mint's bytes are frozen**: one fixture edit — a unit
    /// square on plane 0, inserted into the empty document — mints this
    /// node id and these first and last of its five step ids, and leaves
    /// this chain. A change to the preimage's shape, its serde form,
    /// the tags or the id width re-mints every id every saved log
    /// replays to, and goes red here first.
    #[test]
    fn the_mint_of_one_fixture_edit_is_pinned() {
        let square =
            LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]).unwrap();
        let steps = square.authored_steps();
        let node = Node::Profile(ProfileProgram {
            plane: RecipeNodeId(0),
            loops: vec![square],
            ids: Vec::new(),
        });
        let mut m = Mint::empty();
        let id = m.insert(&node).unwrap();
        let ids = flat(m.steps_of_insert(&new(steps)).unwrap());
        let hex: String = m.chain().iter().map(|b| format!("{b:02x}")).collect();
        assert_eq!(ids.len(), 5, "at, three corners, the close");
        assert_eq!(
            (id.0, ids[0].0, ids[4].0, hex.as_str()),
            (PIN_NODE, PIN_FIRST, PIN_LAST, PIN_CHAIN)
        );
    }

    const PIN_NODE: u64 = 17_256_259_864_915_814_436;
    const PIN_FIRST: u64 = 5_791_587_197_423_663_483;
    const PIN_LAST: u64 = 6_517_970_377_860_469_354;
    const PIN_CHAIN: &str = "5a747b8664e4526a15b0cb9936bea2775190ffb8fe8cb3810aa0d5be00cc8865";

    const LEN: VarKind = VarKind::Length;

    /// INTENT-VARS-1 §4 row 4 and ruling Q5: the chain extends, so two
    /// declares of one kind mint two ids, both logged as variables' —
    /// equal values are not one variable.
    #[test]
    fn two_declares_of_one_kind_mint_two_ids() {
        let mut m = Mint::empty();
        let a = m.declare(LEN).unwrap();
        let b = m.declare(LEN).unwrap();
        assert_ne!(a, b, "the second declare extends a different chain");
        assert_eq!(m.vars().collect::<Vec<_>>(), {
            let mut both = vec![a, b];
            both.sort();
            both
        });
        assert!(m.has_var(a) && m.has_var(b));
        assert!(
            !m.has_node(RecipeNodeId(a.0)) && !m.has_step(StepId(a.0)),
            "logged under the variable tag, not another"
        );
    }

    /// The declare's preimage is the kind (the orchestrator's ruling on
    /// spec §1): kinds part, and `would_declare` reads the id `declare`
    /// mints without minting it.
    #[test]
    fn a_declare_hashes_its_kind_and_would_declare_agrees() {
        let id = Mint::empty().declare(LEN).unwrap();
        for other in [VarKind::Angle, VarKind::Scalar, VarKind::Count] {
            assert_ne!(Mint::empty().declare(other).unwrap(), id, "{other:?}");
        }
        let m = Mint::empty();
        assert_eq!(m.would_declare(LEN), id);
        assert_eq!(m, Mint::empty(), "a read mints nothing");
    }

    /// §4 row 4: a log already holding the bits a declare would draw —
    /// as a NODE's id, so the tag does not let it through — refuses
    /// `VarIdCollides` and moves neither chain nor log.
    #[test]
    fn a_var_id_the_log_holds_refuses_and_moves_nothing() {
        let drawn = Mint::empty().declare(LEN).unwrap();
        let taken = Mint::empty().logged([Minted::Node(RecipeNodeId(drawn.0))]);
        let mut m = taken.clone();
        assert_eq!(m.declare(LEN), Err(VarIdCollides { id: drawn }));
        assert_eq!(m, taken, "a refused mint moves neither chain nor log");
        assert_eq!(m.chain(), taken.chain());
        let mut free = Mint::empty();
        assert_eq!(free.declare(LEN), Ok(drawn), "and an empty log takes it");
    }

    /// **The declare's bytes are frozen**, as the insert's are: one
    /// fixture declare — a length, into the empty document — mints this
    /// id and leaves this chain.
    #[test]
    fn the_mint_of_one_fixture_declare_is_pinned() {
        let mut m = Mint::empty();
        let id = m.declare(LEN).unwrap();
        let hex: String = m.chain().iter().map(|b| format!("{b:02x}")).collect();
        assert_eq!((id, hex.as_str()), (VarId(PIN_VAR), PIN_VAR_CHAIN));
    }

    const PIN_VAR: u64 = 16_300_829_493_895_992_422;
    const PIN_VAR_CHAIN: &str = "e2382e0f27e19466322b51bf83cd48ccace92c81be7625e6fe836050c144b25f";

    #[test]
    fn the_wire_round_trips_and_refuses_a_short_chain() {
        let mut m = Mint::empty();
        m.insert(&extrude(1, 2.0)).unwrap();
        m.steps_of_insert(&new(2)).unwrap();
        let text = serde_json::to_string(&m).unwrap();
        assert_eq!(
            serde_json::from_str::<Mint>(&text).unwrap(),
            m,
            "round trip"
        );
        let short = "{\"chain\":\"00\",\"log\":[]}";
        let err = serde_json::from_str::<Mint>(short).unwrap_err();
        assert!(err.to_string().contains("64 lowercase hex"), "{err}");
        let upper = format!("{{\"chain\":\"{}\",\"log\":[]}}", "A".repeat(64));
        assert!(
            serde_json::from_str::<Mint>(&upper).is_err(),
            "lowercase only"
        );
    }

    #[test]
    fn a_log_out_of_order_is_found() {
        let mut m = Mint::empty();
        m.insert(&extrude(1, 2.0)).unwrap();
        m.steps_of_insert(&new(1)).unwrap();
        assert_eq!(m.out_of_order(), None, "a mint keeps the log ascending");
        let (a, b) = (m.log[0], m.log[1]);
        m.log = vec![b, a];
        assert_eq!(m.out_of_order(), Some(a), "a step down");
        m.log = vec![a, Minted::Step(StepId(a.bits()))];
        assert_eq!(
            m.out_of_order(),
            Some(Minted::Step(StepId(a.bits()))),
            "one id twice, under either tag"
        );
    }
}
