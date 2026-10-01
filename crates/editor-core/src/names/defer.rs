//! **The N2 tie deferral** — the one shape every writer that carries
//! an operand's names forward inserts through: every emitter that
//! reads an operand's table, and the product gather, which carries
//! several finished tables onto one aggregate ([`CarriedRows`], this
//! module's only door out of `names`).
//!
//! A tie cannot be inserted one member at a time: `NameTable::insert`
//! refuses a second row under a name it already carries
//! (`DuplicateName`, whose contract is "the no-silent-aliasing bug"),
//! and the members of an `Entry::Tied` row all carry the SAME name. So
//! rows are written by one of three routes, decided by how the row's
//! name relates to its operand's:
//!
//! - **A name the op mints strict**, wrapping an operand name that is
//!   not tied: straight through `insert`, so a genuine aliasing bug is a
//!   typed `Duplicate`.
//! - **A name the op mints under a tie** — wrapping a tied operand name
//!   (B1), or several equally admissible candidates of its own (A2,
//!   [`mint_candidates`]): DEFERRED, and flushed at a stage boundary
//!   through [`mint_into`], which numbers the candidates afresh as the
//!   name is fresh.
//! - **The operand's own name, carried VERBATIM** — a split's intact
//!   pass-through, every row of the product gather: DEFERRED with the
//!   [`Candidate`] the operand row gave the entity (N4, "A tie's
//!   candidates keep their identity"), under [`TieRows::carry`]'s one
//!   rule, a (name, candidate) pair carried at most once, and flushed
//!   through [`narrow_into`], which keeps each number.
//!
//! Every flush inserts into the same table as the strict route, so a
//! deferred name colliding with a strict one is a typed `Duplicate` too.
//!
//! **One implementation, not a shape to copy.** This module exists
//! because `emit_fillet` was written without the deferral and #708
//! recorded the consequence: a legitimate upstream tie, both members
//! blended, would hand two minted entities one upstream name and the
//! second insertion would report an aliasing bug that is not one.
//! Emitters share this code rather than each carrying a translation of
//! it, so there is no site where the next one can be forgotten. The
//! gather shares it for the same reason one level out: it carries
//! several FINISHED tables onto one aggregate, so a tie the document
//! separated across two source bodies arrives in two pieces, and
//! narrowing a piece on its own is the reading the pieces do not
//! support.

use std::collections::BTreeMap;

use super::emit::NamingError;
use super::role::NameRef;
use super::table::{Candidate, DuplicateName, EntityKey, EntityRef, Entry, NameTable};
use crate::node::RecipeNodeId;

/// An entity's upstream name, how its upstream row descends from an N2
/// tie, and which of the name's candidates the entity is.
///
/// B1 (ratified, #512): a tie PROPAGATES — naming a tie is fine (N2);
/// only *referencing* one is `Ambiguous`. This mirrors the three
/// emitters that already do it (`name_pattern`, `name_in_part`,
/// `graft_names`), so a tie anywhere in an operand table no longer
/// refuses the whole downstream op.
#[derive(Clone)]
pub(super) struct Upstream {
    /// The operand-table name (identical for every tied candidate).
    ///
    /// The table's OWN handle, so a downstream segment that embeds it
    /// shares the operand's name rather than copying its whole descent.
    pub(super) name: NameRef,
    /// True iff that name's entry is `Entry::Tied`.
    pub(super) tied: bool,
    /// The entity's candidate under that name. Only a VERBATIM carry
    /// reads it ([`TieRows::carry`]): a wrapping emitter names a new
    /// row, and its candidates are numbered afresh.
    pub(super) candidate: Candidate,
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
    e: EntityRef,
) -> Result<Upstream, NamingError> {
    // The operand table is finished by the time an emitter reads it,
    // so this is where its key order is cached onto its names (the
    // flag makes every call after the first free).
    table.seal_order();
    let name = table
        .name_ref_of(&e)
        .ok_or(NamingError::MissingUpstream { node })?;
    let disagree = || NamingError::Emission {
        what: "an operand name table's forward and reverse directions disagree",
    };
    let tied = match table.entry_of(name) {
        Some(Entry::Unique(_)) => false,
        Some(Entry::Tied(_)) => true,
        None => return Err(disagree()),
    };
    let candidate = table.candidate_of(name, &e).ok_or_else(disagree)?;
    Ok(Upstream {
        name: name.clone(),
        tied,
        candidate,
    })
}

/// A deferred row: the entity, and the candidate it CARRIES, or `None`
/// for a row whose name this op minted, which the flush numbers.
type Deferred = (Option<Candidate>, EntityRef);

/// Rows deferred until a stage boundary: rows whose name descends from
/// an N2 tie (B1), rows the op itself mints as several
/// equally-admissible candidates under one name (A2, every such site
/// through [`mint_candidates`]), and rows carried VERBATIM with their
/// candidate ([`TieRows::carry`]).
///
/// Upstream candidates that were equally admissible stay equally
/// admissible downstream, so their same-named descendants MERGE into
/// one entry at flush: `Tied` when ≥ 2 survive, narrowed back to
/// `Unique` when exactly one does (the `graft_names` shape). Rows whose
/// operand entry is not `Tied` and that an emitter wraps keep going
/// through `NameTable::insert` directly, so a genuine aliasing bug is
/// still a typed `Duplicate` — and so is a tie-descended name colliding
/// with a strict one, since the flush inserts into the same table.
///
/// Narrowing means a WRAPPED name can come out `Unique` here while the
/// upstream name it wraps stays `Tied` (review NOTE-2). That is the
/// ratified `graft_names` semantics, not laundering: the op genuinely
/// separated the candidates, and the upstream table is untouched.
#[derive(Default)]
pub(super) struct TieRows(BTreeMap<NameRef, Vec<Deferred>>);

impl TieRows {
    /// Defers one row of a name this op mints.
    pub(super) fn push(&mut self, name: impl Into<NameRef>, e: EntityRef) {
        self.0.entry(name.into()).or_default().push((None, e));
    }

    /// **Carries one row VERBATIM**: under its operand's own name, as
    /// the candidate its operand row gave it (N1's pass-through; N4, "A
    /// tie's candidates keep their identity").
    ///
    /// # Errors
    ///
    /// [`DuplicateName`] when the pair is already carried — the same
    /// candidate twice, or a strict name ([`Candidate::Only`], its own
    /// only candidate) beside any other row under it — or when the op
    /// also mints a row under the name.
    pub(super) fn carry(&mut self, up: Upstream, e: EntityRef) -> Result<(), DuplicateName> {
        self.carry_as(up.name, up.candidate, e)
    }

    /// [`TieRows::carry`] with the name and candidate given apart —
    /// the product gather's form, which reads them off a whole row.
    fn carry_as(
        &mut self,
        name: NameRef,
        candidate: Candidate,
        e: EntityRef,
    ) -> Result<(), DuplicateName> {
        let rows = self.0.entry(name.clone()).or_default();
        let distinct = |(held, _): &Deferred| match (held, candidate) {
            (Some(Candidate::Of(a)), Candidate::Of(b)) => *a != b,
            _ => false,
        };
        if !rows.iter().all(distinct) {
            return Err(DuplicateName {
                name: Box::new((*name).clone()),
            });
        }
        rows.push((Some(candidate), e));
        Ok(())
    }

    /// Drains the deferred rows into the table. Called at each stage
    /// boundary, because later stages read the names earlier stages
    /// wrote (the boolean vertex pass reads its incident EDGE names).
    ///
    /// # Errors
    ///
    /// [`DuplicateName`]: the insert doors' own, and a name holding
    /// both carried and minted rows.
    pub(super) fn flush(&mut self, t: &mut NameTable) -> Result<(), DuplicateName> {
        for (name, rows) in core::mem::take(&mut self.0) {
            let carried: Option<Vec<(Candidate, EntityRef)>> =
                rows.iter().map(|&(c, e)| c.map(|c| (c, e))).collect();
            match carried {
                Some(pairs) => narrow_into(t, name, pairs)?,
                None if rows.iter().all(|(c, _)| c.is_none()) => {
                    mint_into(t, name, rows.into_iter().map(|(_, e)| e).collect())?;
                }
                None => {
                    return Err(DuplicateName {
                        name: Box::new((*name).clone()),
                    });
                }
            }
        }
        Ok(())
    }
}

/// **The one narrowing rule for a tie's survivors** (the ratified
/// `graft_names` semantics), for rows that KEEP their candidates: the
/// candidates that survive a verbatim carry — a projection onto one
/// output body, a split's pass-through, the product gather — are
/// written as `Unique` when exactly one does and as `Tied` when several
/// do, each keeping its [`Candidate`]. [`TieRows::flush`] writes a
/// carried name through it, and [`NameTable::project`] writes each row
/// of the selected body through it, so no two of those doors can narrow
/// differently.
///
/// A tie can straddle two output bodies: a pass-through entity keeps
/// its upstream name with no side tag, and a split that separates two
/// tied candidates without cutting either lands one in each half. The
/// projection onto one half then genuinely separates them, exactly as
/// an op does, and the survivor is `Unique` there — as candidate `k`
/// of that tie — while the split's own table stays `Tied`.
///
/// # Errors
///
/// [`DuplicateName`] — the carrying insert door's own
/// (`NameTable::insert_candidates_ref`): the name already held, an
/// entity already named, a kind disagreement, or a candidate carried
/// twice.
pub(super) fn narrow_into(
    t: &mut NameTable,
    name: NameRef,
    pairs: Vec<(Candidate, EntityRef)>,
) -> Result<(), DuplicateName> {
    t.insert_candidates_ref(name, pairs)
}

/// **The narrowing rule for a name this op MINTS**: [`narrow_into`]'s
/// counterpart for a wrapped name, whose candidates are numbered afresh
/// as its name is fresh — one survivor is strict, several are a tie the
/// insert door numbers. The flush writes a minted name through it, and
/// so does `emit::name_placed_union` for the prototype candidates the
/// fuse kept.
///
/// # Errors
///
/// [`DuplicateName`] — the insert doors' own: the name already held, an
/// entity already named, a kind disagreement, or a tie under two
/// candidates.
pub(super) fn mint_into(
    t: &mut NameTable,
    name: NameRef,
    ents: Vec<EntityRef>,
) -> Result<(), DuplicateName> {
    match ents.as_slice() {
        [one] => t.insert_ref(name, *one),
        _ => t.insert_tied_ref(name, ents),
    }
}

/// **The one minting rule for candidates an op makes under ONE name**
/// — the minting counterpart of [`mint_into`]: a lone candidate is
/// written as any row is ([`put`]: strict, or deferred when its name
/// descends from a tie), and several are deferred together as one N2
/// tie (A2: equally admissible, nothing covariant to tell them apart),
/// which the flush then writes through [`mint_into`] as `Tied`.
/// Every emitter that ends a candidate list in a tie ends it here, so
/// "one ⇒ strict, several ⇒ tied" has one spelling.
///
/// What this does NOT cover, on purpose: a lone member that its
/// discriminator would answer trivially keeps the base name at the
/// call site, before any discriminator runs — an `OrderAlong { 0 of 1 }`
/// says nothing. That is a decision about
/// the call site's own discriminator, not a minting rule. Where the
/// several-member branch ends in a tie, it ends here.
///
/// An EMPTY list mints nothing and is not an error — an op with no
/// candidate under a name has no row to write. [`mint_into`] differs
/// on purpose: its list is a tie's SURVIVORS, the insert door refuses
/// an empty one, and a caller whose tie can lose every candidate skips
/// the call itself.
///
/// # Errors
///
/// [`DuplicateName`] — a lone strict candidate's insert refused (the
/// insert doors' own).
pub(super) fn mint_candidates(
    t: &mut NameTable,
    tie: &mut TieRows,
    from_tie: bool,
    name: impl Into<NameRef>,
    ents: Vec<EntityRef>,
) -> Result<(), DuplicateName> {
    let name = name.into();
    match ents.as_slice() {
        [one] => put(t, tie, from_tie, name, *one),
        _ => {
            for e in ents {
                tie.push(name.clone(), e);
            }
            Ok(())
        }
    }
}

/// Inserts a downstream row: strict when its upstream name was unique,
/// deferred into the tie lane when it descends from a tie (B1).
pub(super) fn put(
    t: &mut NameTable,
    tie: &mut TieRows,
    from_tie: bool,
    name: impl Into<NameRef>,
    e: EntityRef,
) -> Result<(), DuplicateName> {
    let name = name.into();
    if from_tie {
        tie.push(name, e);
        Ok(())
    } else {
        t.insert_ref(name, e)
    }
}

/// **The gather's carry**: several source tables re-keyed onto ONE
/// aggregate table, every row carried VERBATIM with its candidate and
/// narrowed once at the end.
///
/// The product gather grafts one source BODY at a time and carries
/// that source's rows as it goes, so a name whose candidates lie in
/// two different source bodies arrives in two pieces — which is what
/// a split that separates a tie without cutting either candidate
/// hands it, one candidate per half, whether as the split root's two
/// bodies or as two `Part` roots over them. So this is [`TieRows`]
/// again, one level out: every row is carried through
/// [`TieRows::carry`]'s rule and narrowed through [`narrow_into`], the
/// door `flush` and [`NameTable::project`] also narrow through, so no
/// caller here can narrow differently.
///
/// **The one rule** (N4, "A tie's candidates keep their identity"): a
/// (name, candidate) pair reaches the product at most once. Different
/// candidates of one tie merge back into the tie. The same candidate
/// twice — one entity placed twice — refuses, and so does a strict
/// name carried twice, since a strict name is its own only candidate.
/// The refusal is read as each row is carried, so it names the root
/// whose row repeated the pair.
///
/// **Why this door knows the gather's body model.** The source-body
/// filter and the body-0 target live here rather than in the caller
/// because the alternative is a caller that pre-selects and re-keys
/// its own rows — which is where the third copy of the narrowing rule
/// lived. The door is the GATHER'S carry, named so; the price of
/// keeping the rule undivided is that it knows the shape of the thing
/// it carries for.
///
/// **[`TieRows`] is still the door for an emitter inside `names`.**
/// The two differ in lifetime, deliberately: `TieRows::flush` takes
/// `&mut self` because an emitter flushes at EVERY stage boundary and
/// keeps accumulating after one; [`CarriedRows::finish`] consumes
/// `self` because the gather narrows once, after the last source, and
/// a second flush would have nothing correct to mean.
#[derive(Default)]
pub(crate) struct CarriedRows(TieRows);

impl CarriedRows {
    /// Carries one source body's rows: `from`'s candidates whose
    /// [`EntityRef::body`] is `ix`, each re-keyed by `map` onto body 0
    /// — a gathered product is ONE body. `map` answers `None` for a row
    /// that does not carry.
    ///
    /// # Errors
    ///
    /// [`DuplicateName`]: a (name, candidate) pair an earlier source
    /// already carried ([`TieRows::carry`]).
    pub(crate) fn carry(
        &mut self,
        from: &NameTable,
        ix: u32,
        map: impl Fn(EntityKey) -> Option<EntityKey>,
    ) -> Result<(), DuplicateName> {
        for (name, row) in from.rows() {
            for (candidate, e) in row.pairs().filter(|(_, e)| e.body == ix) {
                let Some(key) = map(e.key) else { continue };
                let moved = EntityRef { body: 0, key };
                self.0.carry_as(name.clone(), candidate, moved)?;
            }
        }
        Ok(())
    }

    /// Writes every carried name onto the aggregate — once, after the
    /// last source has been carried: `Unique` for one candidate (a
    /// strict name, or a lone `Part` root's piece of a separated tie),
    /// `Tied` for several.
    ///
    /// # Errors
    ///
    /// [`DuplicateName`], the insert doors' own. Not reachable by
    /// construction: `into` is empty, [`CarriedRows::carry`] already
    /// refused every repeated pair, and the graft mints a distinct
    /// entity per source entity.
    pub(crate) fn finish(mut self, into: &mut NameTable) -> Result<(), DuplicateName> {
        self.0.flush(into)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    //! **The gather's one rule on bare rows**: a (name, candidate) pair
    //! is carried at most once, whichever sources carry it.

    use super::{Candidate, CarriedRows, narrow_into};
    use crate::names::role::{EntityKind, NameRef, RoleSeg, StableName};
    use crate::names::table::{EntityKey, EntityRef, Entry, NameTable};
    use crate::node::RecipeNodeId;

    fn name() -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(7),
            path: vec![RoleSeg::OutputBody],
        }
    }

    /// A distinct face per index; the tables here are never read
    /// against an arena.
    fn face(body: u32, i: u64) -> EntityRef {
        EntityRef {
            body,
            key: EntityKey::Face(topo::FaceKey::from(slotmap::KeyData::from_ffi(i))),
        }
    }

    /// A source table naming `face(0, i)` as candidate `c` of `name()`.
    fn source(c: Candidate, i: u64) -> NameTable {
        let mut t = NameTable::new();
        narrow_into(&mut t, NameRef::new(name()), vec![(c, face(0, i))]).expect("a fresh row");
        t
    }

    /// Carries body `ix` of each source with its keys verbatim (a graft
    /// mints a distinct aggregate entity per source entity, and the
    /// sources here already name distinct faces), then narrows. A
    /// refusal reports the index of the source that carried it.
    fn gather(sources: &[(&NameTable, u32)]) -> Result<NameTable, (usize, Box<StableName>)> {
        let mut rows = CarriedRows::default();
        for (i, (t, ix)) in sources.iter().enumerate() {
            rows.carry(t, *ix, Some).map_err(|e| (i, e.name))?;
        }
        let mut out = NameTable::new();
        rows.finish(&mut out).map_err(|e| (usize::MAX, e.name))?;
        Ok(out)
    }

    fn candidates(t: &NameTable) -> Vec<Candidate> {
        let mut ks: Vec<Candidate> = t
            .rows()
            .flat_map(|(_, r)| r.pairs())
            .map(|(c, _)| c)
            .collect();
        ks.sort();
        ks
    }

    #[test]
    fn different_candidates_of_one_tie_merge_back_into_the_tie() {
        let (a, b) = (source(Candidate::Of(0), 1), source(Candidate::Of(1), 2));
        let out = gather(&[(&a, 0), (&b, 0)]).expect("two candidates of one tie merge");
        assert!(
            matches!(out.lookup(&name()), Some(Entry::Tied(es)) if es.len() == 2),
            "the product holds the tie over both: {out:?}"
        );
        assert_eq!(candidates(&out), vec![Candidate::Of(0), Candidate::Of(1)]);
    }

    #[test]
    fn a_split_tie_gathers_the_same_whole_or_as_its_projected_halves() {
        // The split's own table: one tie straddling its two bodies.
        let mut split = NameTable::new();
        narrow_into(
            &mut split,
            NameRef::new(name()),
            vec![
                (Candidate::Of(0), face(0, 1)),
                (Candidate::Of(1), face(1, 2)),
            ],
        )
        .expect("the split's tie");
        let whole = gather(&[(&split, 0), (&split, 1)]).expect("the split root gathers");
        let (above, below) = (split.project(0).unwrap(), split.project(1).unwrap());
        assert!(
            matches!(above.lookup(&name()), Some(Entry::Unique(_))),
            "a half holding one candidate publishes the name unique"
        );
        assert_eq!(
            candidates(&above),
            vec![Candidate::Of(0)],
            "and keeps its candidate"
        );
        let halves = gather(&[(&above, 0), (&below, 0)]).expect("the halves gather");
        assert_eq!(whole, halves, "the halves merge back into the split's tie");
        assert_eq!(
            gather(&[(&above, 0), (&split, 0)]).map(|t| t.len()),
            Err((1, Box::new(name()))),
            "a half beside the split root's same body carries candidate 0 twice"
        );
    }

    #[test]
    fn a_lone_candidate_narrows_to_unique_and_keeps_its_number() {
        let a = source(Candidate::Of(1), 1);
        let out = gather(&[(&a, 0)]).expect("one candidate");
        assert!(matches!(out.lookup(&name()), Some(Entry::Unique(_))));
        assert_eq!(candidates(&out), vec![Candidate::Of(1)]);
    }

    #[test]
    fn the_same_candidate_twice_refuses() {
        let (a, b, c) = (
            source(Candidate::Of(0), 1),
            source(Candidate::Of(1), 2),
            source(Candidate::Of(0), 3),
        );
        assert_eq!(
            gather(&[(&a, 0), (&b, 0), (&c, 0)]).map(|t| t.len()),
            Err((2, Box::new(name()))),
            "the third source repeats candidate 0, and is the one refused"
        );
    }

    #[test]
    fn a_strict_name_carried_twice_refuses() {
        let (a, b) = (source(Candidate::Only, 1), source(Candidate::Only, 2));
        assert_eq!(
            gather(&[(&a, 0), (&b, 0)]).map(|t| t.len()),
            Err((1, Box::new(name())))
        );
    }

    #[test]
    fn a_strict_name_beside_a_tie_candidate_refuses() {
        for [first, second] in [
            [Candidate::Only, Candidate::Of(0)],
            [Candidate::Of(0), Candidate::Only],
        ] {
            let (a, b) = (source(first, 1), source(second, 2));
            assert_eq!(
                gather(&[(&a, 0), (&b, 0)]).map(|t| t.len()),
                Err((1, Box::new(name()))),
                "a strict name is its own only candidate: {first:?} then {second:?}"
            );
        }
    }
}
