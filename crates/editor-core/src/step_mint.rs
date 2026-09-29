//! **Where a profile step's id comes from** (`names/README.md`, "N1, the
//! profile pieces", "The id."): the document's mint chain and mint log.
//!
//! The chain is a SHA-256 digest the document carries. A minting edit
//! extends it by that edit's canonical bytes ([`MintingEdit`]), and each
//! step the edit mints extends it once more; the step's id is the first
//! 64 bits of the chain at that point, big-endian. An edit that mints no
//! step leaves the chain alone. So an id is a function of the minting
//! edits that led to it: one sequence mints one set of ids (D9), and two
//! sequences that part from one value mint different ids from there on.
//!
//! The log holds every id the document has minted, dropped steps'
//! included, ascending. A mint whose id the log already holds is refused
//! ([`StepIdFault::Collides`]); the doors that write a name, and the load
//! door, refuse a step the log does not hold ([`StepIdFault::NotMinted`]);
//! and the load door refuses a log that is not strictly ascending
//! (`SnapshotError::MintLogOrder`).
//!
//! The preimage is not [`crate::persist::canonical_bytes`]: that is a
//! whole document's serde form, display units included, and answers
//! "which version"; this is one edit's statement about its steps with
//! display units erased (D6), and answers "which step".

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
    /// Every id minted, strictly ascending. Read as written, so the
    /// load door can refuse a log out of order rather than repair it.
    log: Vec<StepId>,
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
    /// The bytes the chain is extended by: [`Self::preimage`]'s serde
    /// form.
    fn canonical_bytes(&self) -> Vec<u8> {
        // Every field is a derived-`Serialize` struct, enum, integer,
        // finite float or string, and no map is keyed by anything but a
        // string, so serde_json has no error to return here.
        serde_json::to_vec(&self.preimage())
            .unwrap_or_else(|e| unreachable!("a mint preimage always encodes: {e}"))
    }

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
            log: Vec::new(),
        }
    }

    /// The chain digest the next minting edit extends.
    #[must_use]
    pub fn chain(&self) -> [u8; 32] {
        self.chain
    }

    /// Every step id the document has minted, ascending.
    #[must_use]
    pub fn log(&self) -> &[StepId] {
        &self.log
    }

    /// Whether the document minted `step`.
    #[must_use]
    pub fn has_minted(&self, step: StepId) -> bool {
        self.log.binary_search(&step).is_ok()
    }

    /// The first log entry that is not greater than the one before it —
    /// a repeat or a step down — `None` for a strictly ascending log.
    pub(crate) fn out_of_order(&self) -> Option<StepId> {
        self.log
            .windows(2)
            .find(|pair| pair[0] >= pair[1])
            .map(|pair| pair[1])
    }

    /// **The ids for `edit`'s steps**: `shape` is one list per loop, one
    /// entry per authored step, the id a step keeps or `None` for one to
    /// mint. Each `None` is minted, in loop then step order, extending
    /// the chain and the log; with no `None` nothing moves. On a refusal
    /// `self` is untouched.
    ///
    /// # Errors
    ///
    /// [`StepIdFault::Collides`] where an id is already in the log (or
    /// minted twice by this edit).
    pub(crate) fn mint(
        &mut self,
        edit: &MintingEdit<'_>,
        shape: &[Vec<Option<StepId>>],
    ) -> Result<Vec<Vec<StepId>>, StepIdFault> {
        let count = shape.iter().flatten().filter(|kept| kept.is_none()).count();
        let mut fresh = self.draw(edit, count)?.into_iter();
        Ok(shape
            .iter()
            .map(|lp| {
                lp.iter()
                    .map(|kept| {
                        kept.unwrap_or_else(|| {
                            fresh.next().unwrap_or_else(|| {
                                unreachable!("the mint draws one id per step to mint")
                            })
                        })
                    })
                    .collect()
            })
            .collect())
    }

    /// `count` fresh ids for `edit`, extending the chain and the log.
    fn draw(&mut self, edit: &MintingEdit<'_>, count: usize) -> Result<Vec<StepId>, StepIdFault> {
        if count == 0 {
            return Ok(Vec::new());
        }
        let mut chain: [u8; 32] = Sha256::new()
            .chain_update(EDIT_TAG)
            .chain_update(self.chain)
            .chain_update(edit.canonical_bytes())
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
            match log.binary_search(&step) {
                Ok(_) => return Err(StepIdFault::Collides { step }),
                Err(at) => log.insert(at, step),
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
        let refuse = || {
            D::Error::custom(format!(
                "a step mint chain is exactly 64 lowercase hex digits, got {text:?}"
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

    use super::{MintingEdit, StepMint};
    use crate::node::{RecipeNodeId, StepId};
    use crate::program::{LoopProgram, StepIdFault};

    fn insert(node: u64) -> MintingEdit<'static> {
        MintingEdit::InsertNode {
            node: RecipeNodeId(node),
            plane: RecipeNodeId(0),
            loops: &[],
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
        let mut a = StepMint::empty();
        let mut b = StepMint::empty();
        let first_a = a.mint(&insert(1), &new(3)).unwrap();
        let first_b = b.mint(&insert(1), &new(3)).unwrap();
        assert_eq!(
            first_a, first_b,
            "one edit from one mint mints one set of ids"
        );
        assert_eq!(a, b, "and leaves one mint");
        let second_a = a.mint(&insert(2), &new(1)).unwrap();
        let second_b = b.mint(&insert(3), &new(1)).unwrap();
        assert_ne!(
            second_a, second_b,
            "two edits from one mint mint different ids"
        );
    }

    #[test]
    fn kept_ids_pass_through_and_only_new_steps_mint() {
        let mut m = StepMint::empty();
        let kept = StepId(7);
        let shape = vec![vec![Some(kept), None], vec![None]];
        let ids = m.mint(&insert(1), &shape).unwrap();
        assert_eq!(ids[0][0], kept, "a kept id is handed back where it stood");
        assert_eq!(
            m.log().len(),
            2,
            "the two new steps, and only they, are logged"
        );
        assert!(m.has_minted(ids[0][1]) && m.has_minted(ids[1][0]));
    }

    #[test]
    fn an_edit_that_mints_nothing_leaves_the_chain() {
        let mut m = StepMint::empty();
        assert_eq!(flat(m.mint(&insert(1), &new(0)).unwrap()), vec![]);
        assert_eq!(m, StepMint::empty());
    }

    #[test]
    fn a_mint_the_log_holds_refuses_and_moves_nothing() {
        let mut fresh = StepMint::empty();
        let ids = flat(fresh.clone().mint(&insert(1), &new(2)).unwrap());
        // A log that already holds the second id the edit would mint.
        fresh.log.push(ids[1]);
        let before = fresh.clone();
        assert_eq!(
            fresh.mint(&insert(1), &new(2)),
            Err(StepIdFault::Collides { step: ids[1] })
        );
        assert_eq!(fresh, before, "a refused mint moves neither chain nor log");
    }

    /// **The mint's bytes are frozen**: one fixture edit — a unit square
    /// inserted as node 1 on plane 0 — mints these first and last of its
    /// five ids and leaves this chain (the same document the
    /// `OLDER_SHAPED` exemplars in `tests/` spell). A change to the preimage's shape, its
    /// serde form, the tags or the id width re-mints every id every
    /// saved log replays to, and goes red here first.
    #[test]
    fn the_mint_of_one_fixture_edit_is_pinned() {
        let square =
            LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]).unwrap();
        let loops = [square];
        let edit = MintingEdit::InsertNode {
            node: RecipeNodeId(1),
            plane: RecipeNodeId(0),
            loops: &loops,
        };
        let mut m = StepMint::empty();
        let ids = flat(m.mint(&edit, &new(loops[0].authored_steps())).unwrap());
        let hex: String = m.chain().iter().map(|b| format!("{b:02x}")).collect();
        assert_eq!(ids.len(), 5, "at, three corners, the close");
        assert_eq!((ids[0].0, ids[4].0), (PIN_FIRST, PIN_LAST));
        assert_eq!(hex, PIN_CHAIN);
    }

    const PIN_FIRST: u64 = 6_747_313_831_402_317_760;
    const PIN_LAST: u64 = 10_985_843_265_047_041_854;
    const PIN_CHAIN: &str = "98758e3e7b3f173efd085e7b81a83f61319b60646c238a909cac90c0c3dd4bfd";

    #[test]
    fn the_wire_round_trips_and_refuses_a_short_chain() {
        let mut m = StepMint::empty();
        m.mint(&insert(1), &new(2)).unwrap();
        let text = serde_json::to_string(&m).unwrap();
        assert_eq!(
            serde_json::from_str::<StepMint>(&text).unwrap(),
            m,
            "round trip"
        );
        let short = "{\"chain\":\"00\",\"log\":[]}";
        let err = serde_json::from_str::<StepMint>(short).unwrap_err();
        assert!(err.to_string().contains("64 lowercase hex"), "{err}");
        let upper = format!("{{\"chain\":\"{}\",\"log\":[]}}", "A".repeat(64));
        assert!(
            serde_json::from_str::<StepMint>(&upper).is_err(),
            "lowercase only"
        );
    }

    #[test]
    fn a_log_out_of_order_is_found() {
        let mut m = StepMint::empty();
        m.mint(&insert(1), &new(2)).unwrap();
        assert_eq!(m.out_of_order(), None, "a mint keeps the log ascending");
        let (a, b) = (m.log[0], m.log[1]);
        m.log = vec![b, a];
        assert_eq!(m.out_of_order(), Some(a), "a step down");
        m.log = vec![a, a];
        assert_eq!(m.out_of_order(), Some(a), "a repeat");
    }
}
