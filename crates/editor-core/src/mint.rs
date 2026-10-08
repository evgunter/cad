//! **Where a node's id, a profile step's id and a variable's id come
//! from** (`names/README.md`, N1 and "N1, the profile pieces", "The
//! id."; VARIABLES-DESIGN VR1): the document's mint, one chain and one
//! log for all three.
//!
//! The chain is a SHA-256 digest the document carries. A minting edit
//! extends it by that edit's canonical bytes ([`MintingEdit`]); an
//! `InsertNode` then extends it once more for the node it mints, and
//! each step either edit mints extends it once more again. An id is a
//! [`MintId`]: its mint ordinal, the log's length when it was drawn
//! plus one, and the first 64 bits of the chain at the point that drew
//! it, big-endian. An edit that mints nothing leaves the chain alone.
//! So an id is a function of the minting edits that led to it: one
//! sequence mints one set of ids (D9), and two sequences that part from
//! one value mint different ids from there on — the same ordinals, and
//! different digests. A `DeclareVar` extends the chain by the
//! variable's kind and then once for the variable it mints; its name is
//! not in the preimage (VR2), so two declares of one kind mint two ids
//! only because the chain moved between them. Each anonymous variable an
//! edit's lowering mints extends it by its kind and what it holds
//! ([`MintingEdit::DeclareAnonymous`], [`Held`]: a value by its bits, a
//! definition with its literals canonical), so two sibling inserts that
//! differ only in a value they hold mint two nodes; a display unit is in
//! neither preimage (D6).
//!
//! Ids order by ordinal first, so they order as they were minted. The
//! log holds every id the document has minted, deleted nodes' and
//! dropped steps' included, each tagged with what it names
//! ([`Minted`]), in mint order: the entry at index `i` has ordinal
//! `i + 1`. So no two ids one document mints are equal. The doors that
//! write a name, and the load door, refuse a node or a step the log
//! does not hold as that; and the load door refuses a log whose
//! ordinals do not count up from one (`SnapshotError::MintLogOrder`),
//! the only log a mint writes.
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
use crate::program::LoopProgram;
use crate::var::{VarId, VarKind};

/// The document's mint: the chain the next minting edit extends and
/// the log of every id minted so far.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mint {
    /// The chain digest, as 64 lowercase hex digits on the wire.
    #[serde(with = "chain_hex")]
    chain: [u8; 32],
    /// Every id minted, in mint order. Read as written, so the load
    /// door can refuse a log out of order rather than repair it.
    log: Vec<Minted>,
}

/// **An id the mint draws** (`names/README.md`, N1, "The id."): the
/// pair of its mint ordinal — the mint log's length when it was
/// drawn, plus one — and the first 64 bits of the chain digest that
/// drew it. Ordered ordinal first, so of two ids one document minted
/// the lesser is the one minted first; the digest is what tells apart
/// two documents' ids at one ordinal.
///
/// [`crate::RecipeNodeId`], [`crate::StepId`] and [`crate::VarId`] each
/// wrap one. On the wire it is a string, its [`crate::FullId`]
/// spelling (`3:3fa9c1d2a0b1c3d4`), so a map keyed by ids is a JSON
/// object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MintId {
    ordinal: u32,
    digest: u64,
}

impl MintId {
    /// The id with mint ordinal `ordinal` and digest head `digest`.
    #[must_use]
    pub const fn new(ordinal: u32, digest: u64) -> Self {
        Self { ordinal, digest }
    }

    /// Its mint ordinal: one more than the number of ids its document
    /// had minted before it. No mint draws ordinal 0.
    #[must_use]
    pub const fn ordinal(self) -> u32 {
        self.ordinal
    }

    /// The first 64 bits of the chain digest that drew it.
    #[must_use]
    pub const fn digest(self) -> u64 {
        self.digest
    }

    /// The id as [`crate::FullId`] spells it, read back: the ordinal in
    /// decimal with no leading zero, a colon, and the digest as 16
    /// lowercase hex digits. `None` for any other text.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        let (ordinal, digest) = text.split_once(':')?;
        let decimal = !ordinal.is_empty()
            && ordinal.bytes().all(|b| b.is_ascii_digit())
            && (ordinal == "0" || !ordinal.starts_with('0'));
        let hex = digest.len() == 16
            && digest
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
        if !(decimal && hex) {
            return None;
        }
        Some(Self {
            ordinal: ordinal.parse().ok()?,
            digest: u64::from_str_radix(digest, 16).ok()?,
        })
    }
}

/// The id whole, as the wire spells it: the ordinal in decimal, a
/// colon, and the digest as 16 lowercase hex digits
/// (`3:3fa9c1d2a0b1c3d4`), [`crate::FullId`]'s spelling.
impl core::fmt::Display for MintId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:{:016x}", self.ordinal, self.digest)
    }
}

impl Serialize for MintId {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for MintId {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        struct Spelled;
        impl serde::de::Visitor<'_> for Spelled {
            type Value = MintId;
            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(
                    "an id: its mint ordinal in decimal, a colon, and its digest as 16 \
                     lowercase hex digits (`3:3fa9c1d2a0b1c3d4`)",
                )
            }
            fn visit_str<E: serde::de::Error>(self, text: &str) -> Result<MintId, E> {
                MintId::parse(text)
                    .ok_or_else(|| E::invalid_value(serde::de::Unexpected::Str(text), &self))
            }
        }
        de.deserialize_str(Spelled)
    }
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
    /// A variable's id, minted by `DeclareVar`, by an edit's lowering
    /// for an anonymous one, or by `InsertNode` for its node's outputs.
    Var(VarId),
}

impl Minted {
    /// The id, whatever it names.
    #[must_use]
    pub fn id(self) -> MintId {
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
    pub(crate) fn of(def: &crate::var::WrittenDef) -> Self {
        use crate::var::WrittenDef;
        match def {
            WrittenDef::Free(crate::doc::FreeVar::Continuous { value, .. }) => {
                Self::Value(value.to_bits())
            }
            WrittenDef::Free(crate::doc::FreeVar::Count { value }) => Self::Count(*value),
            WrittenDef::Defined(expr) => Self::Defined(expr.clone()),
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
const OUTPUT_TAG: &[u8] = b"mint/output\0";

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

    /// Every id the document has minted, in mint order.
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

    /// Whether the log holds `entry`: the entry at its ordinal's index
    /// is it.
    fn holds(&self, entry: Minted) -> bool {
        let at = (entry.id().ordinal() as usize).checked_sub(1);
        at.and_then(|at| self.log.get(at)) == Some(&entry)
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
        let (chain, id) = Self::draw(VAR_TAG, self.extended(edit), &self.log);
        (chain, VarId(id))
    }

    /// **The id an anonymous variable holding `def` draws here**: the
    /// chain extended by its kind and what it holds
    /// ([`MintingEdit::DeclareAnonymous`]), then once for the variable.
    fn draw_anonymous(&self, def: &crate::var::WrittenDef) -> ([u8; 32], VarId) {
        self.draw_var_of(&MintingEdit::DeclareAnonymous {
            kind: def.kind(),
            held: Held::of(def),
        })
    }

    /// The id an anonymous variable holding `def` would mint here —
    /// what a refusal of it speaks. Reads; mints nothing.
    #[must_use]
    pub(crate) fn would_declare_anonymous(&self, def: &crate::var::WrittenDef) -> VarId {
        self.draw_anonymous(def).1
    }

    /// **Mint the id of an anonymous variable holding `def`**
    /// ([`Self::draw_anonymous`]).
    pub(crate) fn declare_anonymous(&mut self, def: &crate::var::WrittenDef) -> VarId {
        let (chain, id) = self.draw_anonymous(def);
        self.chain = chain;
        self.log.push(Minted::Var(id));
        id
    }

    /// The id a declare of a `kind` variable would mint here — what a
    /// refused declare's refusal speaks. Reads; mints nothing.
    #[must_use]
    pub(crate) fn would_declare(&self, kind: VarKind) -> VarId {
        self.draw_var(kind).1
    }

    /// **Mint the id of a declared `kind` variable** ([`Self::draw_var`]).
    pub(crate) fn declare(&mut self, kind: VarKind) -> VarId {
        let (chain, id) = self.draw_var(kind);
        self.chain = chain;
        self.log.push(Minted::Var(id));
        id
    }

    /// This mint with `entries` logged too, in order, where a test
    /// pushes nodes by hand rather than through the insert door: each
    /// entry's ordinal must be the next one.
    #[cfg(test)]
    pub(crate) fn logged(mut self, entries: impl IntoIterator<Item = Minted>) -> Self {
        self.log.extend(entries);
        assert_eq!(
            self.out_of_order(),
            None,
            "a hand-built log counts up from one"
        );
        self
    }

    /// The first log entry whose ordinal is not its place in the log —
    /// a repeat, a gap or a step down — `None` for a log whose ordinals
    /// count up from one, the only log a mint writes.
    pub(crate) fn out_of_order(&self) -> Option<Minted> {
        self.log
            .iter()
            .zip(1u64..)
            .find(|&(entry, ordinal)| u64::from(entry.id().ordinal()) != ordinal)
            .map(|(&entry, _)| entry)
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

    /// The next digest along `chain` under `tag`, and the id it gives
    /// as the next entry of `log`.
    fn draw(tag: &[u8], chain: [u8; 32], log: &[Minted]) -> ([u8; 32], MintId) {
        let chain: [u8; 32] = Sha256::new()
            .chain_update(tag)
            .chain_update(chain)
            .finalize()
            .into();
        let mut head = [0u8; 8];
        head.copy_from_slice(&chain[..8]);
        let Some(ordinal) = u32::try_from(log.len())
            .ok()
            .and_then(|minted| minted.checked_add(1))
        else {
            // One mint per edit's node, step or variable: a document
            // reaches 2^32 of them only by an edit log no door replays
            // in this process's lifetime.
            unreachable!(
                "a document has minted {} ids, the most a mint ordinal counts",
                log.len()
            )
        };
        (chain, MintId::new(ordinal, u64::from_be_bytes(head)))
    }

    /// **The id an insert of `node` mints**: the chain extended by the
    /// node's canonical bytes, then once for the node. The steps the
    /// node's profile authors, if any, are minted next, by
    /// [`Self::steps_of_insert`].
    pub(crate) fn insert<P: Serialize + Clone + crate::program::SlotPayload<S>, S: crate::Slot>(
        &mut self,
        node: &Node<P, S>,
    ) -> RecipeNodeId
    where
        Node<P, S>: Serialize,
    {
        let edit = self.extended(&MintingEdit::insert(node));
        let (chain, id) = Self::draw(NODE_TAG, edit, &self.log);
        let id = RecipeNodeId(id);
        self.chain = chain;
        self.log.push(Minted::Node(id));
        id
    }

    /// **The ids for the steps of the profile [`Self::insert`] just
    /// minted a node for**: one per authored step, `count` in all, in
    /// loop then step order, each extending the chain once.
    pub(crate) fn steps_of_insert(&mut self, shape: &[Vec<Option<StepId>>]) -> Vec<Vec<StepId>> {
        self.fill(self.chain, shape)
    }

    /// **The ids of the outputs of the node [`Self::insert`] just
    /// minted** (its steps', if any, minted before them): one per port
    /// of its signature, `ports` in all, in port order, each extending
    /// the chain once under the output tag and its port.
    pub(crate) fn outputs_of_insert(&mut self, ports: u8) -> Vec<VarId> {
        (0..ports)
            .map(|port| {
                let indexed: [u8; 32] = Sha256::new()
                    .chain_update(OUTPUT_TAG)
                    .chain_update([port])
                    .chain_update(self.chain)
                    .finalize()
                    .into();
                let (chain, id) = Self::draw(OUTPUT_TAG, indexed, &self.log);
                let var = VarId(id);
                self.chain = chain;
                self.log.push(Minted::Var(var));
                var
            })
            .collect()
    }

    /// **The ids for a `SetProgram`'s steps**: `shape` is one list per
    /// loop, one entry per authored step, the id a step keeps or `None`
    /// for one to mint. The chain is extended by `edit`'s bytes, and
    /// each `None` is minted from it in loop then step order; with no
    /// `None` nothing moves.
    pub(crate) fn set_program(
        &mut self,
        node: RecipeNodeId,
        loops: &[LoopProgram],
        shape: &[Vec<Option<StepId>>],
    ) -> Vec<Vec<StepId>> {
        if shape.iter().flatten().all(Option::is_some) {
            return shape
                .iter()
                .map(|lp| lp.iter().flatten().copied().collect())
                .collect();
        }
        self.fill(
            self.extended(&MintingEdit::set_program(node, loops, shape)),
            shape,
        )
    }

    /// Mints each `None` of `shape` along `chain`, extending the log.
    fn fill(&mut self, mut chain: [u8; 32], shape: &[Vec<Option<StepId>>]) -> Vec<Vec<StepId>> {
        let mut ids = Vec::with_capacity(shape.len());
        for lp in shape {
            let mut out = Vec::with_capacity(lp.len());
            for kept in lp {
                let step = match *kept {
                    Some(step) => step,
                    None => {
                        let (next, id) = Self::draw(STEP_TAG, chain, &self.log);
                        chain = next;
                        let step = StepId(id);
                        self.log.push(Minted::Step(step));
                        step
                    }
                };
                out.push(step);
            }
            ids.push(out);
        }
        self.chain = chain;
        ids
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

    use super::{Mint, MintId, Minted};
    use crate::node::{Node, RecipeNodeId, StepId};
    use crate::program::{LoopProgram, ProfileProgram};
    use crate::var::{VarId, VarKind};

    fn extrude(profile: u64, distance: f64) -> Node<ProfileProgram> {
        Node::Extrude {
            profile: RecipeNodeId::new(0, profile),
            distance: VarId::new(0, distance.to_bits()),
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
        let first_a = a.insert(&extrude(1, 2.0));
        let first_b = b.insert(&extrude(1, 2.0));
        assert_eq!(first_a, first_b, "one edit from one mint mints one id");
        assert_eq!(a, b, "and leaves one mint");
        let second_a = a.insert(&extrude(1, 3.0));
        let second_b = b.insert(&extrude(1, 4.0));
        assert_eq!(
            second_a.0.ordinal(),
            second_b.0.ordinal(),
            "two edits from one mint draw one ordinal"
        );
        assert_ne!(second_a, second_b, "and different ids: the digests part");
        let again = a.insert(&extrude(1, 3.0));
        assert_ne!(
            again.0.digest(),
            second_a.0.digest(),
            "one edit twice draws two digests: the chain moved"
        );
    }

    /// **Ids order as they were minted**, whatever their digests: every
    /// draw, node, step or variable, takes the next ordinal, so the log
    /// reads in mint order and each id is greater than every id drawn
    /// before it.
    #[test]
    fn ids_order_as_minted_across_every_tag() {
        let mut m = Mint::empty();
        let node = m.insert(&extrude(1, 2.0));
        let steps = flat(m.steps_of_insert(&new(2)));
        let var = m.declare(VarKind::Length);
        let later = m.insert(&extrude(1, 3.0));
        let drawn = [node.0, steps[0].0, steps[1].0, var.0, later.0];
        assert_eq!(
            drawn.map(MintId::ordinal),
            [1, 2, 3, 4, 5],
            "each draw takes the log's length plus one"
        );
        assert!(
            drawn.windows(2).all(|pair| pair[0] < pair[1]),
            "so each id is greater than the one drawn before it: {drawn:?}"
        );
        assert_eq!(m.log().iter().map(|e| e.id()).collect::<Vec<_>>(), drawn);
        assert_eq!(m.out_of_order(), None);
        assert!(
            RecipeNodeId::new(1, u64::MAX) < RecipeNodeId::new(2, 0),
            "the ordinal decides before the digest"
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
        let node = m.insert(&extrude(1, 2.0));
        let ids = flat(m.steps_of_insert(&new(2)));
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
        let kept = StepId::new(0, 7);
        let shape = vec![vec![Some(kept), None], vec![None]];
        let ids = m.set_program(RecipeNodeId::new(0, 1), &[], &shape);
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
        let kept = vec![vec![Some(StepId::new(0, 3))]];
        assert_eq!(
            flat(m.set_program(RecipeNodeId::new(0, 1), &[], &kept)),
            vec![StepId::new(0, 3)]
        );
        assert_eq!(m, Mint::empty());
    }

    /// **The mint's bytes are frozen**: one fixture edit — a unit
    /// square on plane 0, inserted into the empty document — mints this
    /// node id and these first and last of its five step ids, and leaves
    /// this chain. A change to the preimage's shape, its serde form,
    /// the tags or the id's form re-mints every id every saved log
    /// replays to, and goes red here first.
    #[test]
    fn the_mint_of_one_fixture_edit_is_pinned() {
        let square =
            LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]).unwrap();
        let steps = square.authored_steps();
        let node = Node::Profile(ProfileProgram {
            plane: RecipeNodeId::new(0, 0),
            loops: vec![square],
            ids: Vec::new(),
        });
        let mut m = Mint::empty();
        let id = m.insert(&node);
        let ids = flat(m.steps_of_insert(&new(steps)));
        let hex: String = m.chain().iter().map(|b| format!("{b:02x}")).collect();
        assert_eq!(ids.len(), 5, "at, three corners, the close");
        assert_eq!(
            (
                crate::FullId(id.0).to_string(),
                crate::FullId(ids[0].0).to_string(),
                crate::FullId(ids[4].0).to_string(),
                hex.as_str()
            ),
            (
                PIN_NODE.to_owned(),
                PIN_FIRST.to_owned(),
                PIN_LAST.to_owned(),
                PIN_CHAIN
            )
        );
    }

    const PIN_NODE: &str = "1:90bea142d63f9c8e";
    const PIN_FIRST: &str = "2:68976136f9f05b41";
    const PIN_LAST: &str = "6:f9596cc085049b34";
    const PIN_CHAIN: &str = "f9596cc085049b34ef2b287aceec347cfcbceabc6afadef8f5fe9c9f3bedfc6b";

    const LEN: VarKind = VarKind::Length;

    /// INTENT-VARS-1 §4 row 4 and ruling Q5: the chain extends, so two
    /// declares of one kind mint two ids, both logged as variables' —
    /// equal values are not one variable.
    #[test]
    fn two_declares_of_one_kind_mint_two_ids() {
        let mut m = Mint::empty();
        let a = m.declare(LEN);
        let b = m.declare(LEN);
        assert_ne!(
            a.0.digest(),
            b.0.digest(),
            "the second declare extends a different chain"
        );
        assert!(a < b, "and is the later id");
        assert_eq!(m.vars().collect::<Vec<_>>(), vec![a, b]);
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
        let id = Mint::empty().declare(LEN);
        for other in [VarKind::Angle, VarKind::Scalar, VarKind::Count] {
            assert_ne!(Mint::empty().declare(other), id, "{other:?}");
        }
        let m = Mint::empty();
        assert_eq!(m.would_declare(LEN), id);
        assert_eq!(m, Mint::empty(), "a read mints nothing");
    }

    /// **The declare's bytes are frozen**, as the insert's are: one
    /// fixture declare — a length, into the empty document — mints this
    /// id and leaves this chain.
    #[test]
    fn the_mint_of_one_fixture_declare_is_pinned() {
        let mut m = Mint::empty();
        let id = m.declare(LEN);
        let hex: String = m.chain().iter().map(|b| format!("{b:02x}")).collect();
        assert_eq!((id, hex.as_str()), (VarId::new(1, PIN_VAR), PIN_VAR_CHAIN));
    }

    const PIN_VAR: u64 = 16_300_829_493_895_992_422;
    const PIN_VAR_CHAIN: &str = "e2382e0f27e19466322b51bf83cd48ccace92c81be7625e6fe836050c144b25f";

    #[test]
    fn the_wire_round_trips_and_refuses_a_short_chain() {
        let mut m = Mint::empty();
        m.insert(&extrude(1, 2.0));
        m.steps_of_insert(&new(2));
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

    /// **An id's wire spelling is its whole id**, and only the one
    /// spelling reads back: a leading zero, an uppercase digit, a short
    /// digest or a missing half is refused rather than read.
    #[test]
    fn an_id_reads_back_only_from_its_one_spelling() {
        let id = MintId::new(12, 0x3fa9_c1d2_a0b1_c3d4);
        let text = serde_json::to_string(&id).unwrap();
        assert_eq!(text, "\"12:3fa9c1d2a0b1c3d4\"");
        assert_eq!(serde_json::from_str::<MintId>(&text).unwrap(), id);
        assert_eq!(MintId::parse("0:0000000000000000"), Some(MintId::new(0, 0)));
        for bad in [
            "012:3fa9c1d2a0b1c3d4",
            "12:3FA9C1D2A0B1C3D4",
            "12:3fa9c1d2a0b1c3d",
            "12:3fa9c1d2a0b1c3d4f",
            ":3fa9c1d2a0b1c3d4",
            "12",
            "4294967296:3fa9c1d2a0b1c3d4",
            "-1:3fa9c1d2a0b1c3d4",
        ] {
            assert_eq!(MintId::parse(bad), None, "{bad}");
            assert!(
                serde_json::from_str::<MintId>(&format!("\"{bad}\"")).is_err(),
                "{bad}"
            );
        }
        assert!(
            serde_json::from_str::<MintId>("12").is_err(),
            "a bare integer is no id"
        );
    }

    #[test]
    fn a_log_out_of_order_is_found() {
        let mut m = Mint::empty();
        m.insert(&extrude(1, 2.0));
        m.steps_of_insert(&new(1));
        assert_eq!(m.out_of_order(), None, "a mint counts the log up from one");
        let (a, b) = (m.log[0], m.log[1]);
        m.log = vec![b, a];
        assert_eq!(m.out_of_order(), Some(b), "a step down");
        m.log = vec![a, a];
        assert_eq!(m.out_of_order(), Some(a), "one id twice");
        m.log = vec![b];
        assert_eq!(m.out_of_order(), Some(b), "a gap");
        let twin = Minted::Step(StepId(a.id()));
        m.log = vec![a, twin];
        assert_eq!(
            m.out_of_order(),
            Some(twin),
            "one ordinal twice, under either tag"
        );
    }
}
#[cfg(test)]
mod zz_ord_sizes {
    use crate::*;
    #[test]
    fn zz_ord_sizes() {
        use core::mem::size_of as s;
        eprintln!(
            "ORDSIZES edit={} persist={} split={} snapshot={} inline={} mate={}",
            s::<EditError>(),
            s::<PersistError>(),
            s::<SplitError>(),
            s::<SnapshotError>(),
            s::<InlineError>(),
            s::<MateFault>()
        );
    }
}
