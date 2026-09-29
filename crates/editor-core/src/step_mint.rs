//! **Where a profile step's id comes from** (`names/README.md`, "N1, the
//! profile pieces", "The id."): the document's mint chain and mint log.
//!
//! The chain is a SHA-256 digest the document carries. A minting edit
//! extends it by that edit's canonical bytes ([`MintingEdit`]), and each
//! step the edit mints extends it once more; the step's id is the first
//! eight bytes of the chain at that point, big-endian. An edit that mints
//! no step leaves the chain alone. So an id is a function of the minting
//! edits that led to it: one sequence mints one set of ids (D9), and two
//! sequences that part from one value mint different ids from there on.
//!
//! The log holds every id the document has minted, dropped steps'
//! included. A mint whose id the log already holds is refused
//! ([`StepIdFault::Collides`]), and the doors that write a name, and the
//! load door, refuse a step the log does not hold
//! ([`StepIdFault::NotMinted`]).

use std::collections::BTreeSet;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest, Sha256};

use crate::node::{RecipeNodeId, StepId};
use crate::program::{LoopProgram, StepIdFault};

/// The document's step mint: the chain the next minting edit extends
/// and the log of every step id minted so far.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StepMint {
    /// The chain digest, as 64 lowercase hex digits on the wire.
    #[serde(with = "chain_hex")]
    chain: [u8; 32],
    /// Every id minted, as an ascending list on the wire; a repeated
    /// entry refuses at load.
    #[serde(with = "log_list")]
    log: BTreeSet<StepId>,
}

/// **A minting edit**, as the mint reads it: what the edit states about
/// the steps it authors. `InsertNode` states the node it becomes and the
/// profile's plane and loops (a program entering the document carries no
/// ids); `SetProgram` states its node, the new loops and, per step, the
/// id it keeps or `None` for one to mint.
///
/// Its canonical bytes are the serde form of that statement with every
/// literal's display unit read as its dimension's canonical one: the
/// display unit is never part of an expression's identity (DESIGN.md
/// D6), so two edits `bit_eq` cannot tell apart mint the same ids.
pub(crate) enum MintingEdit<'a> {
    /// A profile inserted as `node`.
    InsertNode {
        /// The id the insert gives the node.
        node: RecipeNodeId,
        /// The profile's plane.
        plane: RecipeNodeId,
        /// The profile's loops.
        loops: &'a [LoopProgram],
    },
    /// A profile's program replaced.
    SetProgram {
        /// The profile.
        node: RecipeNodeId,
        /// The new loops.
        loops: &'a [LoopProgram],
        /// The kept ids.
        ids: &'a [Vec<Option<StepId>>],
    },
}

/// [`MintingEdit`]'s canonical form, the one the chain hashes.
#[derive(Serialize)]
enum Preimage<'a> {
    InsertNode {
        node: RecipeNodeId,
        plane: RecipeNodeId,
        loops: Vec<LoopProgram>,
    },
    SetProgram {
        node: RecipeNodeId,
        loops: Vec<LoopProgram>,
        ids: &'a [Vec<Option<StepId>>],
    },
}

impl MintingEdit<'_> {
    fn preimage(&self) -> Preimage<'_> {
        let unit_blind = |loops: &[LoopProgram]| {
            let mut loops = loops.to_vec();
            for expr in loops.iter_mut().flat_map(LoopProgram::exprs_mut) {
                expr.erase_display_units();
            }
            loops
        };
        match *self {
            Self::InsertNode { node, plane, loops } => Preimage::InsertNode {
                node,
                plane,
                loops: unit_blind(loops),
            },
            Self::SetProgram { node, loops, ids } => Preimage::SetProgram {
                node,
                loops: unit_blind(loops),
                ids,
            },
        }
    }
}

const EDIT_TAG: &[u8] = b"step-mint/edit\0";
const STEP_TAG: &[u8] = b"step-mint/step\0";

impl StepMint {
    /// A document's mint before any step is authored: the zero chain and
    /// an empty log.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            chain: [0; 32],
            log: BTreeSet::new(),
        }
    }

    /// The chain digest the next minting edit extends.
    #[must_use]
    pub fn chain(&self) -> [u8; 32] {
        self.chain
    }

    /// Every step id the document has minted.
    #[must_use]
    pub fn log(&self) -> &BTreeSet<StepId> {
        &self.log
    }

    /// Whether the document minted `step`.
    #[must_use]
    pub fn has_minted(&self, step: StepId) -> bool {
        self.log.contains(&step)
    }

    /// **Mints `count` ids for `edit`**, in the order its steps are
    /// authored, extending the chain and the log. With `count == 0`
    /// nothing moves. On a refusal `self` is untouched.
    ///
    /// # Errors
    ///
    /// [`StepIdFault::Collides`] where an id is already in the log (or
    /// minted twice by this edit); [`StepIdFault::Unencodable`] where the
    /// edit does not serialize.
    pub(crate) fn mint(
        &mut self,
        edit: &MintingEdit<'_>,
        count: usize,
    ) -> Result<Vec<StepId>, StepIdFault> {
        if count == 0 {
            return Ok(Vec::new());
        }
        let bytes = serde_json::to_vec(&edit.preimage()).map_err(|_| StepIdFault::Unencodable)?;
        let mut chain: [u8; 32] = Sha256::new()
            .chain_update(EDIT_TAG)
            .chain_update(self.chain)
            .chain_update(&bytes)
            .finalize()
            .into();
        let mut log = self.log.clone();
        let mut ids = Vec::with_capacity(count);
        for _ in 0..count {
            chain = Sha256::new()
                .chain_update(STEP_TAG)
                .chain_update(chain)
                .finalize()
                .into();
            let mut head = [0u8; 8];
            head.copy_from_slice(&chain[..8]);
            let step = StepId(u64::from_be_bytes(head));
            if !log.insert(step) {
                return Err(StepIdFault::Collides { step });
            }
            ids.push(step);
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
        crate::ident::ContentPin::parse_hex(&text)
            .map(|pin| pin.0)
            .ok_or_else(|| {
                D::Error::custom(format!(
                    "a step mint chain is exactly 64 lowercase hex digits, got {text:?}"
                ))
            })
    }
}

mod log_list {
    use super::{BTreeSet, Deserialize, Deserializer, Serialize, Serializer, StepId};
    use serde::de::Error as _;

    pub(super) fn serialize<S: Serializer>(
        log: &BTreeSet<StepId>,
        ser: S,
    ) -> Result<S::Ok, S::Error> {
        log.serialize(ser)
    }

    pub(super) fn deserialize<'de, D: Deserializer<'de>>(
        de: D,
    ) -> Result<BTreeSet<StepId>, D::Error> {
        let list = Vec::<StepId>::deserialize(de)?;
        let mut log = BTreeSet::new();
        for step in list {
            if !log.insert(step) {
                return Err(D::Error::custom(format!(
                    "duplicate step mint log entry {} — refused, one mint per id",
                    step.0
                )));
            }
        }
        Ok(log)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::{MintingEdit, StepMint};
    use crate::node::RecipeNodeId;
    use crate::program::StepIdFault;

    fn insert(node: u64) -> MintingEdit<'static> {
        MintingEdit::InsertNode {
            node: RecipeNodeId(node),
            plane: RecipeNodeId(0),
            loops: &[],
        }
    }

    #[test]
    fn one_sequence_mints_one_set_of_ids_and_two_sequences_part() {
        let mut a = StepMint::empty();
        let mut b = StepMint::empty();
        let first_a = a.mint(&insert(1), 3).unwrap();
        let first_b = b.mint(&insert(1), 3).unwrap();
        assert_eq!(
            first_a, first_b,
            "one edit from one mint mints one set of ids"
        );
        assert_eq!(a, b, "and leaves one mint");
        let second_a = a.mint(&insert(2), 1).unwrap();
        let second_b = b.mint(&insert(3), 1).unwrap();
        assert_ne!(
            second_a, second_b,
            "two edits from one mint mint different ids"
        );
        assert_eq!(a.log().len(), 4, "the log holds every id minted");
    }

    #[test]
    fn an_edit_that_mints_nothing_leaves_the_chain() {
        let mut m = StepMint::empty();
        assert_eq!(m.mint(&insert(1), 0).unwrap(), vec![]);
        assert_eq!(m, StepMint::empty());
    }

    #[test]
    fn a_mint_the_log_holds_refuses_and_moves_nothing() {
        let mut fresh = StepMint::empty();
        let ids = fresh.clone().mint(&insert(1), 2).unwrap();
        // A log that already holds the second id the edit would mint.
        fresh.log.insert(ids[1]);
        let before = fresh.clone();
        assert_eq!(
            fresh.mint(&insert(1), 2),
            Err(StepIdFault::Collides { step: ids[1] })
        );
        assert_eq!(fresh, before, "a refused mint moves neither chain nor log");
    }

    #[test]
    fn the_wire_refuses_a_repeated_log_entry_and_a_short_chain() {
        let mut m = StepMint::empty();
        m.mint(&insert(1), 2).unwrap();
        let text = serde_json::to_string(&m).unwrap();
        assert_eq!(
            serde_json::from_str::<StepMint>(&text).unwrap(),
            m,
            "round trip"
        );
        let id = m.log().iter().next().unwrap().0;
        let repeated = format!("{{\"chain\":\"{}\",\"log\":[{id},{id}]}}", "0".repeat(64));
        let err = serde_json::from_str::<StepMint>(&repeated).unwrap_err();
        assert!(
            err.to_string().contains("duplicate step mint log entry"),
            "{err}"
        );
        let short = "{\"chain\":\"00\",\"log\":[]}";
        let err = serde_json::from_str::<StepMint>(short).unwrap_err();
        assert!(err.to_string().contains("64 lowercase hex"), "{err}");
    }
}
