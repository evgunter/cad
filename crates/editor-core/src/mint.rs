//! **Where a node's id and a profile step's id come from**
//! (`names/README.md`, N1 and "N1, the profile pieces", "The id."): the
//! document's mint, one chain and one log for both.
//!
//! The chain is a SHA-256 digest the document carries. A minting edit
//! extends it by that edit's canonical bytes ([`MintingEdit`]); an
//! `InsertNode` then extends it once more for the node it mints, and
//! each step either edit mints extends it once more again. An id is the
//! first 64 bits of the chain at the point that minted it, big-endian.
//! An edit that mints nothing leaves the chain alone. So an id is a
//! function of the minting edits that led to it: one sequence mints one
//! set of ids (D9), and two sequences that part from one value mint
//! different ids from there on.
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
//! "which version"; this is one edit's statement with display units
//! erased (D6), and answers "which node" or "which step". An insert's
//! statement is its node as authored, which holds its inputs' and its
//! names' ids but never its own.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest, Sha256};

use crate::node::{Node, RecipeNodeId, StepId};
use crate::program::{LoopProgram, StepIdFault};

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
}

impl Minted {
    /// The id's bits, the log's order.
    #[must_use]
    pub fn bits(self) -> u64 {
        match self {
            Self::Node(id) => id.0,
            Self::Step(step) => step.0,
        }
    }
}

/// `node <tag>` or `step <tag>`: the entry as a sentence reads it.
impl core::fmt::Display for Minted {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Node(id) => write!(f, "node {id}"),
            Self::Step(step) => write!(f, "step {step}"),
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

/// **A minting edit**, as the mint reads it: the node an `InsertNode`
/// inserts, as authored; or what a `SetProgram` states about the steps
/// it authors — its node, the new loops and, per step, the id it keeps
/// or `None` for one to mint.
///
/// Its canonical bytes are the serde form of that statement with every
/// literal's display unit read as its dimension's canonical one: the
/// display unit is never part of an expression's identity (DESIGN.md
/// D6), so two edits `bit_eq` cannot tell apart mint the same ids.
#[derive(Serialize)]
#[serde(bound(serialize = "P: Serialize"))]
pub(crate) enum MintingEdit<'a, P> {
    /// A node inserted, as the edit states it.
    InsertNode {
        /// The node.
        node: Box<Node<P>>,
    },
    /// A profile's program replaced.
    SetProgram {
        /// The profile.
        node: RecipeNodeId,
        /// The new loops.
        loops: Vec<LoopProgram>,
        /// The kept ids.
        ids: &'a [Vec<Option<StepId>>],
    },
}

impl<'a, P: Serialize + Clone + crate::ProfilePayload> MintingEdit<'a, P> {
    /// The insert of `node`, display units erased.
    fn insert(node: &Node<P>) -> Self {
        let mut node = Box::new(node.clone());
        node.erase_display_units();
        Self::InsertNode { node }
    }
}

impl<'a> MintingEdit<'a, crate::program::ProfileProgram> {
    /// The `SetProgram` of `loops` on `node`, display units erased.
    fn set_program(
        node: RecipeNodeId,
        loops: &[LoopProgram],
        ids: &'a [Vec<Option<StepId>>],
    ) -> Self {
        let mut loops = loops.to_vec();
        for expr in loops.iter_mut().flat_map(LoopProgram::exprs_mut) {
            expr.erase_display_units();
        }
        Self::SetProgram { node, loops, ids }
    }
}

impl<P: Serialize> MintingEdit<'_, P> {
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
            Minted::Node(_) => None,
        })
    }

    /// Every node id the document has minted, ascending.
    #[cfg(test)]
    pub(crate) fn nodes(&self) -> impl Iterator<Item = RecipeNodeId> + '_ {
        self.log.iter().filter_map(|entry| match *entry {
            Minted::Node(id) => Some(id),
            Minted::Step(_) => None,
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
    fn extended<P: Serialize>(&self, edit: &MintingEdit<'_, P>) -> [u8; 32] {
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
    pub(crate) fn insert<P: Serialize + Clone + crate::ProfilePayload>(
        &mut self,
        node: &Node<P>,
    ) -> Result<RecipeNodeId, NodeIdCollides> {
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

    use super::{Mint, Minted, NodeIdCollides};
    use crate::expr::{Dimension, Expr};
    use crate::node::{Node, RecipeNodeId, StepId};
    use crate::program::{LoopProgram, ProfileProgram, StepIdFault};

    fn extrude(profile: u64, distance: f64) -> Node<ProfileProgram> {
        Node::Extrude {
            profile: RecipeNodeId(profile),
            distance: Expr::literal(distance, Dimension::Length).unwrap(),
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

    #[test]
    fn the_display_unit_is_not_part_of_what_an_insert_hashes() {
        let written = Expr::length_in(2.0, quantity::MM).unwrap();
        let canonical = Expr::literal(written.literal_value().unwrap(), Dimension::Length).unwrap();
        let mm = Node::<ProfileProgram>::Extrude {
            profile: RecipeNodeId(1),
            distance: written,
        };
        let m = Node::<ProfileProgram>::Extrude {
            profile: RecipeNodeId(1),
            distance: canonical,
        };
        assert!(mm.bit_eq(&m), "bit_eq cannot tell the two apart");
        assert_eq!(
            Mint::empty().insert(&mm).unwrap(),
            Mint::empty().insert(&m).unwrap(),
            "so they mint one id (D6)"
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

    const PIN_NODE: u64 = 14_781_035_785_231_637_513;
    const PIN_FIRST: u64 = 14_986_585_060_459_383_076;
    const PIN_LAST: u64 = 1_356_137_351_626_931_182;
    const PIN_CHAIN: &str = "12d1f7bc765cbbee68bd2a7bb046ab2b1a0027e350a1c8d5d66eb8bf72e7d5ba";

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
