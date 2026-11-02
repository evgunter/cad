//! **Where a name read in one variable went, in a table that carries
//! that variable in** (N1, N3): the successor query a takeover rebinds
//! its references by (`crate::take_over`).
//!
//! An operation carries an entity of an input in as
//! `From { read, of }`, or, where it merges faces or edges, as one
//! member of a `Merged` set. A name `of` of the variable `read` has a
//! **successor** in a table when exactly one row of it is that carry,
//! alone or as a member of its set, and that row is unique. A fragment
//! of it, a borders-qualified piece, a tie, or two rows that each hold
//! it, is no successor: which entity the name now means is a choice,
//! and the reference stays stale for the person to make it (N5's
//! `Rebind`).
//!
//! **Why this is not `merged.rs`'s walk.** That walk reads a set
//! through its descent wrappers (`From` of any read, and `Inner`) to
//! list the set's constituents; a successor is read through neither.
//! An `Inner` is a shell's twin of a face, another face. A `From` of
//! another read is a carry through some other operation: the entity
//! there is that operation's, and it is the body's only where that
//! operation carried it whole and once, which is a successor question
//! about that operation's own table, not this one's. This query answers
//! for the carries of the one read it is asked about, and nothing
//! deeper. What the two share is only the one-level shape of a carry
//! and of a set.
//!
//! So a result answers here only for what it carries of the taken read
//! directly. Most takeovers' results read the taken body directly (a
//! feature of it, a union or a combine over it). A fused pattern's
//! union reads it through its family, carrying each face once per
//! member, so its takeover has no successor for any face (issue
//! `work/doors/a-fused-patterns-takeover-rebinds-nothing.md`). A copy
//! of a copy carries its source's names as they are, with no carry at
//! all; the door follows those placement chains itself
//! (`crate::take_over`).

use std::collections::BTreeMap;

use super::role::{RoleSeg, StableName};
use super::table::{Entry, NameTable};
use crate::VarId;

/// **The successor in `table` of each name of `read` it carries in**,
/// keyed by that name.
///
/// A row counts when its whole path is `[From { read, of }]`, or
/// `[Merged(set)]` with that carry among the set; a row with more
/// segments (a fragment, a borders qualifier) carries a piece of `of`
/// and does not count. A name two rows count for, or whose one row
/// is tied, has no successor and is left out.
pub fn successors(table: &NameTable, read: VarId) -> BTreeMap<StableName, StableName> {
    let mut found: BTreeMap<&StableName, Vec<(&StableName, bool)>> = BTreeMap::new();
    for (row, entry) in table.iter() {
        let unique = matches!(entry, Entry::Unique(_));
        let carried: Vec<&StableName> = match row.path.as_slice() {
            [RoleSeg::From { read: r, of }] if *r == read => vec![of.name()],
            [RoleSeg::Merged(set)] => set
                .iter()
                .filter_map(|member| match member.path.as_slice() {
                    [RoleSeg::From { read: r, of }] if *r == read => Some(of.name()),
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        };
        for of in carried {
            found.entry(of).or_default().push((row, unique));
        }
    }
    found
        .into_iter()
        .filter_map(|(of, rows)| match rows.as_slice() {
            [(row, true)] => Some((of.clone(), (*row).clone())),
            _ => None,
        })
        .collect()
}

/// **`name` with every entity it carries in from one of `moved`'s
/// reads carried on**: each `From { read, of }` segment, at any depth,
/// whose `read` is a key of `moved` and whose `of` that read's map
/// holds, carries the mapped name instead. `None` where nothing it
/// carries moved.
pub(crate) fn recarried<M: core::borrow::Borrow<BTreeMap<StableName, StableName>>>(
    name: &StableName,
    moved: &BTreeMap<VarId, M>,
) -> Option<StableName> {
    let Ok(now) = name.clone().rewrite_path(&mut Recarry(moved));
    (now != *name).then_some(now)
}

/// [`recarried`]'s walk: a carry is answered from the pair it is, its
/// read and its name ([`super::SegRewrite::carried`]); every other carried
/// name is descended, for the carries inside it.
struct Recarry<'m, M>(&'m BTreeMap<VarId, M>);

impl<M: core::borrow::Borrow<BTreeMap<StableName, StableName>>> super::SegRewrite
    for Recarry<'_, M>
{
    type Error = core::convert::Infallible;

    fn carried(&mut self, read: VarId, of: &StableName) -> Result<super::Carry, Self::Error> {
        Ok(
            match self.0.get(&read).and_then(|map| map.borrow().get(of)) {
                Some(to) => super::Carry::Replace(to.clone()),
                None => super::Carry::Descend,
            },
        )
    }

    fn name(&mut self, _: &StableName) -> Result<super::Carry, Self::Error> {
        Ok(super::Carry::Descend)
    }

    fn descended(
        &mut self,
        n: &StableName,
        walked: StableName,
    ) -> Result<Option<StableName>, Self::Error> {
        Ok((walked != *n).then_some(walked))
    }

    fn index(&mut self, index: VarId) -> Result<VarId, Self::Error> {
        Ok(index)
    }
}

#[cfg(test)]
mod a_successor_is_one_whole_unique_row {
    #![allow(clippy::expect_used)]
    use super::{recarried, successors};
    use crate::VarId;
    use crate::names::role::{EntityKind, NameRef, Qualifier, RoleSeg, StableName};
    use crate::names::table::{EntityKey, EntityRef, NameTable};
    use crate::node::RecipeNodeId;
    use std::collections::BTreeMap;

    const READ: VarId = VarId::new(0, 7);
    const OTHER: VarId = VarId::new(0, 8);

    fn name(node: u64, path: Vec<RoleSeg>) -> StableName {
        StableName {
            kind: EntityKind::Body,
            node: RecipeNodeId::new(0, node),
            path,
        }
    }

    /// The copied body's own entity `i`.
    fn own(i: u32) -> StableName {
        name(100 + u64::from(i), vec![RoleSeg::OutputBody])
    }

    fn from(read: VarId, of: StableName) -> StableName {
        name(
            2,
            vec![RoleSeg::From {
                read,
                of: NameRef::new(of),
            }],
        )
    }

    fn ent(i: u32) -> EntityRef {
        EntityRef {
            body: i,
            key: EntityKey::Body,
        }
    }

    /// Each row shape against what it carries: a whole `From` row and a
    /// set holding the carry count; a fragment, a tie, a carry of
    /// another read and a name two rows hold do not.
    #[test]
    fn each_row_shape_against_its_carry() {
        let mut table = NameTable::new();
        let whole = from(READ, own(0));
        let set = name(
            2,
            vec![RoleSeg::Merged(vec![
                from(READ, own(1)),
                from(OTHER, own(9)),
            ])],
        );
        let piece = name(
            2,
            vec![
                RoleSeg::From {
                    read: READ,
                    of: NameRef::new(own(2)),
                },
                RoleSeg::Fragment(Qualifier::Borders(vec![own(9)])),
            ],
        );
        let elsewhere = from(OTHER, own(4));
        let twice = [
            name(
                3,
                vec![RoleSeg::Merged(vec![
                    from(READ, own(5)),
                    from(OTHER, own(6)),
                ])],
            ),
            name(
                3,
                vec![RoleSeg::Merged(vec![
                    from(READ, own(5)),
                    from(OTHER, own(7)),
                ])],
            ),
        ];
        for (i, row) in [&whole, &set, &piece, &elsewhere, &twice[0], &twice[1]]
            .into_iter()
            .enumerate()
        {
            table
                .insert(row.clone(), ent(u32::try_from(i).expect("a few rows")))
                .expect("distinct rows");
        }
        table
            .insert_tied(from(READ, own(3)), vec![ent(10), ent(11)])
            .expect("a tie");
        let found = successors(&table, READ);
        assert_eq!(
            found,
            BTreeMap::from([(own(0), whole.clone()), (own(1), set.clone())]),
            "the whole row and the set; not the piece, the tie, the other read or the name two rows hold"
        );
    }

    /// A carry of a moved read is replaced at any depth; one of an
    /// unmoved read with the same inner name is left alone.
    #[test]
    fn a_carry_is_recarried_through_its_read_alone() {
        let to = name(5, vec![RoleSeg::OutputBody]);
        let moved = BTreeMap::from([(READ, BTreeMap::from([(own(0), to.clone())]))]);
        let deep = from(OTHER, from(READ, own(0)));
        assert_eq!(
            recarried(&deep, &moved),
            Some(from(OTHER, from(READ, to))),
            "two levels down"
        );
        assert_eq!(
            recarried(&from(OTHER, own(0)), &moved),
            None,
            "another read's"
        );
    }

    /// A carry followed, in the same path, by a set of names that are
    /// not carries: the carry is recarried on its own read, and the
    /// names after it are not read as carried in by that read.
    #[test]
    fn a_carry_beside_a_borders_set_is_recarried_on_its_own_read() {
        let to = name(5, vec![RoleSeg::OutputBody]);
        let moved = BTreeMap::from([(
            READ,
            BTreeMap::from([
                (own(0), to.clone()),
                (own(9), name(6, vec![RoleSeg::OutputBody])),
            ]),
        )]);
        let piece = |of: StableName| {
            name(
                2,
                vec![
                    RoleSeg::From {
                        read: READ,
                        of: NameRef::new(of),
                    },
                    RoleSeg::Fragment(Qualifier::Borders(vec![own(9)])),
                ],
            )
        };
        assert_eq!(
            recarried(&piece(own(0)), &moved),
            Some(piece(to)),
            "the carry moves; the wall it borders, read in no `From`, does not"
        );
    }
}
