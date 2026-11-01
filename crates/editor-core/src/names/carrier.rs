//! **Where a name read in one variable went, in a table that carries
//! that variable in** (N1, N3): the carrier query a takeover re-seats
//! its references by (`refactor::take_over`).
//!
//! An operation carries an entity of an input in as
//! `From { read, of }`, or, where it merges faces or edges, as one
//! member of a `Merged` set. A name `of` of the variable `read` has a
//! **unique carrier** in a table when exactly one row of it is that
//! carry, alone or as a member of its set, and that row is unique.
//! A fragment of it, a borders-qualified piece, a tie, or two rows
//! that each hold it, is no carrier: which entity the name now means
//! is a choice, and the reference stays stale for the person to make it
//! (N5's `Rebind`).

use std::collections::BTreeMap;

use super::role::{RoleSeg, StableName};
use super::table::{Entry, NameTable};
use crate::VarId;

/// **The unique carrier in `table` of each name of `read` it carries
/// in**, keyed by that name.
///
/// A row counts when its whole path is `[From { read, of }]`, or
/// `[Merged(set)]` with that carry among the set; a row with more
/// segments (a fragment, a borders qualifier) carries a piece of `of`
/// and does not count. A name two rows count for, or whose one row
/// is tied, has no unique carrier and is left out.
pub fn carriers(table: &NameTable, read: VarId) -> BTreeMap<StableName, StableName> {
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
/// reads re-seated**: each `From { read, of }` segment, at any depth,
/// whose `read` is a key of `moved` and whose `of` that read's map
/// holds, carries the mapped name instead. `None` where nothing it
/// carries moved.
pub(crate) fn reseated(
    name: &StableName,
    moved: &BTreeMap<VarId, BTreeMap<StableName, StableName>>,
) -> Option<StableName> {
    let mut walk = Reseat { moved, read: None };
    let Ok(now) = name.clone().rewrite_path(&mut walk);
    (now != *name).then_some(now)
}

/// [`reseated`]'s walk. The walk visits a `From`'s read and then its
/// carried name, in that order and with nothing between, so the read
/// last visited is the one the carried name is read in.
struct Reseat<'m> {
    moved: &'m BTreeMap<VarId, BTreeMap<StableName, StableName>>,
    read: Option<VarId>,
}

impl super::SegRewrite for Reseat<'_> {
    type Error = core::convert::Infallible;

    fn read(&mut self, r: VarId) -> Result<VarId, Self::Error> {
        self.read = Some(r);
        Ok(r)
    }

    fn name(&mut self, n: &StableName) -> Result<super::Carry, Self::Error> {
        let to = self
            .read
            .take()
            .and_then(|read| self.moved.get(&read))
            .and_then(|map| map.get(n));
        Ok(to.map_or(super::Carry::Descend, |to| super::Carry::Replace(to.clone())))
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
