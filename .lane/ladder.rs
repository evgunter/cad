/// The mid-evaluation N5 refusal ladder, shared by every door that
/// resolves an AUTHORED name against the tables the run has built so
/// far ([`super::resolve_selection`], [`super::resolve_declarations`]).
///
/// Mid-evaluation there is no prior run and no whole-evaluation
/// index, so [`crate::resolve`]'s full ladder does not apply: what is
/// left is three rungs, in this order.
///
/// 1. [`live`] — the minting node must still be in the document.
///    Ids are never reused, so an id below the mint counter was
///    DELETED and one at/above it was never this document's
///    (`ForeignNode`). This rung outranks every later refusal,
///    including a door's own; the [`Live`] token is the only way to
///    reach the rungs below, so no door can invert the order.
/// 2. [`Landing::Tied`] → `Ambiguous`. The tie row IS the ambiguity
///    (N5), so the tied set expressed in names is the name itself,
///    and the witness carries the multiplicity and the minting site.
/// 3. [`Landing::Absent`] → `Vanished`, with the honest single-run
///    fallback diagnosis: no prior run is consultable mid-evaluation,
///    so `NodeChanged` names the minting node as the disagreement
///    SITE, not a claim that an edit happened, and `last_good` is
///    `None` because nothing was banked.
///
/// A door supplies [`Landing`]s — one per table it resolves through —
/// and keeps its own arity: which table to consult, what a
/// multi-table hit means, and what kind of entity it will accept are
/// the door's business. Which typed refusal comes out is this
/// module's, and has one home.
mod ladder {
    use crate::names::{Entry, EntityRef, NameTable, StableName};
    use crate::program::ProfileProgram;
    use crate::resolve::{Diagnosis, RecipeEditRef, ResolveError, TieWitness};

    /// Where a name landed in ONE table (rungs 2 and 3, as data).
    pub(super) enum Landing {
        /// Exactly one entity carries the name.
        Unique(EntityRef),
        /// The name is a tie row of this width.
        Tied(u32),
        /// This table does not carry the name.
        Absent,
    }

    /// Proof that rung 1 passed: `name`'s minting node is live.
    /// Constructible only by [`live`], so [`resolve`] cannot be
    /// reached before the `NodeGone` check that outranks it.
    pub(super) struct Live<'n>(&'n StableName);

    /// Reads one table (N4: resolution IS this read).
    pub(super) fn landing(table: &NameTable, name: &StableName) -> Landing {
        match table.lookup(name) {
            Some(Entry::Unique(ent)) => Landing::Unique(*ent),
            Some(Entry::Tied(ents)) => Landing::Tied(ents.len() as u32),
            None => Landing::Absent,
        }
    }

    /// Rung 1: `NodeGone` with the deleted-vs-foreign split.
    pub(super) fn live<'n>(
        name: &'n StableName,
        doc: &crate::doc::Doc<ProfileProgram>,
    ) -> Result<Live<'n>, ResolveError> {
        if doc.node(name.node).is_some() {
            return Ok(Live(name));
        }
        Err(ResolveError::NodeGone {
            name: name.clone(),
            edit: if name.node.0 < doc.next_id {
                RecipeEditRef::NodeDeleted { node: name.node }
            } else {
                RecipeEditRef::ForeignNode { node: name.node }
            },
        })
    }

    /// Rungs 2 and 3: the entity, or the refusal its landing earns.
    pub(super) fn resolve(live: Live<'_>, landing: Landing) -> Result<EntityRef, ResolveError> {
        let name = live.0;
        match landing {
            Landing::Unique(ent) => Ok(ent),
            Landing::Tied(width) => Err(ResolveError::Ambiguous {
                name: name.clone(),
                candidates: vec![name.clone()],
                tie: TieWitness {
                    node: name.node,
                    at: name.clone(),
                    width,
                },
            }),
            Landing::Absent => Err(ResolveError::Vanished {
                name: name.clone(),
                diagnosis: Diagnosis::RecipeEdit {
                    edit: RecipeEditRef::NodeChanged { node: name.node },
                },
                last_good: None,
            }),
        }
    }
}
