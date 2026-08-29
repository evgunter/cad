//! **The N2 tie deferral** — the one shape every emitter that reads an
//! operand's names inserts through.
//!
//! A tie cannot be inserted one member at a time: `NameTable::insert`
//! refuses a second row under a name it already carries
//! (`DuplicateName`, whose contract is "the no-silent-aliasing bug"),
//! and the members of an `Entry::Tied` row all carry the SAME name. So
//! a row whose name descends from a tie is DEFERRED here and flushed
//! as a set at a stage boundary, where `insert_tied` can take the
//! whole candidate list at once.
//!
//! Rows that do not descend from a tie keep going through `insert`
//! directly, so a genuine aliasing bug is still a typed `Duplicate` —
//! and so is a tie-descended name colliding with a strict one, since
//! the flush inserts into the same table.
//!
//! **One implementation, not a shape to copy.** This module exists
//! because `emit_fillet` was written without the deferral and #708
//! recorded the consequence: a legitimate upstream tie, both members
//! blended, would hand two minted entities one upstream name and the
//! second insertion would report an aliasing bug that is not one.
//! Emitters share this code rather than each carrying a translation of
//! it, so there is no site where the next one can be forgotten.

use std::collections::BTreeMap;

use super::emit::NamingError;
use super::role::StableName;
use super::table::{Entry, NameTable};
use crate::node::RecipeNodeId;

/// An entity's upstream name, plus whether the upstream entry is an N2
/// TIE.
///
/// B1 (ratified, #512): a tie PROPAGATES — naming a tie is fine (N2);
/// only *referencing* one is `Ambiguous`. This mirrors the three
/// emitters that already do it (`name_pattern`, `name_in_part`,
/// `graft_names`), so a tie anywhere in an operand table no longer
/// refuses the whole downstream op.
#[derive(Clone)]
pub(super) struct Upstream {
    /// The operand-table name (identical for every tied candidate).
    pub(super) name: StableName,
    /// True iff that name's entry is `Entry::Tied`.
    pub(super) tied: bool,
}

/// The upstream name of an entity. A MISSING row is still loud (the
/// upstream tables are total by this same machinery), and so is a
/// table whose two directions disagree — after B1, that is the ONE
/// remaining condition genuinely needing a unique upstream, because no
/// candidate list exists to propagate.
///
/// It has no executable test row ON PURPOSE, and the reason is a
/// property rather than an omission: the condition is unconstructible
/// through `NameTable`'s public API — `insert`/`insert_tied` write both
/// directions together and there is no removal door, so no caller can
/// reach a state where `name_of` answers and `lookup` does not. The
/// LIB-G14 review confirmed this independently (MINOR-1) and recorded
/// the prose as the faithful reading. The arm stays because the
/// invariant is the emitter's to assert, not to assume.
pub(super) fn upstream_name(
    table: &NameTable,
    node: RecipeNodeId,
    e: super::table::EntityRef,
) -> Result<Upstream, NamingError> {
    let name = table
        .name_of(&e)
        .ok_or(NamingError::MissingUpstream { node })?;
    let tied = match table.lookup(name) {
        Some(Entry::Unique(_)) => false,
        Some(Entry::Tied(_)) => true,
        None => {
            return Err(NamingError::Emission {
                what: "an operand name table's forward and reverse directions disagree",
            });
        }
    };
    Ok(Upstream {
        name: name.clone(),
        tied,
    })
}

/// Rows deferred because their name descends from an N2 tie (B1) — or,
/// for `SectionEdge`, because the op itself mints one (A2).
///
/// Upstream candidates that were equally admissible stay equally
/// admissible downstream, so their same-named descendants MERGE into
/// one entry at flush: `Tied` when ≥ 2 survive, narrowed back to
/// `Unique` when exactly one does (the `graft_names` shape). Rows that
/// do NOT descend from a tie keep going through `NameTable::insert`
/// directly, so a genuine aliasing bug is still a typed `Duplicate` —
/// and so is a tie-descended name colliding with a strict one, since
/// the flush inserts into the same table.
///
/// Narrowing means a WRAPPED name can come out `Unique` here while the
/// upstream name it wraps stays `Tied` (review NOTE-2). That is the
/// ratified `graft_names` semantics, not laundering: the op genuinely
/// separated the candidates, and the upstream table is untouched.
#[derive(Default)]
pub(super) struct TieRows(BTreeMap<StableName, Vec<super::table::EntityRef>>);

impl TieRows {
    /// Defers one row.
    pub(super) fn push(&mut self, name: StableName, e: super::table::EntityRef) {
        self.0.entry(name).or_default().push(e);
    }

    /// Drains the deferred rows into the table. Called at each stage
    /// boundary, because later stages read the names earlier stages
    /// wrote (the boolean vertex pass reads its incident EDGE names).
    pub(super) fn flush(&mut self, t: &mut NameTable) -> Result<(), NamingError> {
        for (name, ents) in core::mem::take(&mut self.0) {
            match ents.as_slice() {
                [one] => t.insert(name, *one)?,
                _ => t.insert_tied(name, ents)?,
            }
        }
        Ok(())
    }
}

/// Inserts a downstream row: strict when its upstream name was unique,
/// deferred into the tie lane when it descends from a tie (B1).
pub(super) fn put(
    t: &mut NameTable,
    tie: &mut TieRows,
    from_tie: bool,
    name: StableName,
    e: super::table::EntityRef,
) -> Result<(), NamingError> {
    if from_tie {
        tie.push(name, e);
        Ok(())
    } else {
        Ok(t.insert(name, e)?)
    }
}

