//! **Every crate that decides a predicate name runs a roster, or is
//! listed here with the row that gives it one; and every name a
//! predicate literal references is one some crate decides.**
//!
//! A roster (`crates/<crate>/tests/recourse_roster.rs`, on the shared
//! harness `test_utils::source::roster_violations`) holds that nothing
//! its crate decides is invisible to the census. This row is what makes
//! a crate fall under one without anyone remembering to add it: it walks
//! every crate's `src`, and **a crate with a decide site that runs no
//! roster is red** unless [`KNOWN_MISSING`] lists it with its row.
//!
//! The list is a **two-way ratchet**. A listed crate that now runs a
//! roster, or has no site left, is red too ("remove it from the list"),
//! so the list only shrinks, and an entry cites a row that exists.
//!
//! **References.** A literal a `predicate` field or comparison names
//! (`predicate: Some("…")`, `d.predicate == Some("…")`) is a second
//! spelling of a name some door decides. It has to resolve against the
//! union of every crate's census, each roster crate's built with its own
//! roster's carriers (`test_utils::source::crate_roster`), so a
//! fixture that outlives a rename is red here. A deliberate negative — a
//! hand-built cause, a name asserted absent — says so at the site with
//! `// UNDECIDED-NAME: <why>` on the line above, not in a list. A name
//! only a listed crate spells is held by that crate's entry: its roster
//! is what will read it, and once it runs one the name has to resolve.
//! A crate that runs a roster but spells its names in a closed type's
//! table the census does not read (profile's `Gate`) is an entry too,
//! [`Missing::ClosedTable`], red once it holds no reference.
//!
//! **When it runs.** It walks the whole tree, so it is in the slow set:
//! a PR runs it only when the PR names it, and the nightly every night.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};
use test_utils::source::{
    PredicateCensus, crate_roster, predicate_census, repo_root, spelled_literals,
    unresolved_references,
};

/// Why a crate's decided names are not all in the census union yet.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Missing {
    /// It decides names and runs no roster.
    Roster,
    /// It runs a roster, but its names are spelled in a closed type's
    /// table, which the census does not read.
    ClosedTable,
}

/// The crates whose decided names the union does not yet hold, each
/// with why and the row that closes it. A reference only such a crate
/// spells is held by its entry.
const KNOWN_MISSING: &[(&str, Missing, &str)] = &[
    (
        "editor-core",
        Missing::Roster,
        "work/onecopy/editor-core-runs-no-predicate-name-roster.md",
    ),
    (
        "profile",
        Missing::ClosedTable,
        "work/onecopy/the-census-reads-no-closed-type-name-table.md",
    ),
];

/// A crate as this row reads it.
struct Crate {
    census: PredicateCensus,
    runs_roster: bool,
}

/// Every crate under `crates/` with a `src`, read once.
fn crates() -> BTreeMap<String, Crate> {
    let root = repo_root(env!("CARGO_MANIFEST_DIR")).join("crates");
    let mut out = BTreeMap::new();
    for entry in std::fs::read_dir(&root).expect("a readable crates directory") {
        let dir = entry.expect("a readable entry").path();
        let src = dir.join("src");
        if !src.is_dir() {
            continue;
        }
        let roster = crate_roster(&dir);
        let runs_roster = roster.is_some();
        let carriers = roster.unwrap_or_default();
        let name = dir.file_name().unwrap().to_string_lossy().into_owned();
        out.insert(
            name,
            Crate {
                census: predicate_census(&src, &carriers),
                runs_roster,
            },
        );
    }
    out
}

/// Whether a census found any site: a name, or one it could not read.
fn has_sites(census: &PredicateCensus) -> bool {
    !census.names.is_empty()
        || !census.indirect.is_empty()
        || !census.consts.is_empty()
        || !census.unreadable.is_empty()
}

/// **Every crate with a site runs a roster or is listed, the list holds
/// only crates that need it, and every referenced name resolves.** One
/// walk of the tree for both halves.
#[test]
fn every_deciding_crate_runs_a_roster_and_every_referenced_name_is_decided() {
    let crates = crates();
    let root = repo_root(env!("CARGO_MANIFEST_DIR"));
    let listed: BTreeMap<&str, Missing> = KNOWN_MISSING
        .iter()
        .map(|(name, why, _)| (*name, *why))
        .collect();
    assert_eq!(listed.len(), KNOWN_MISSING.len(), "a crate is listed twice");
    let mut out = Vec::new();
    for (name, why, row) in KNOWN_MISSING {
        if !root.join(row).is_file() {
            out.push(format!("`{name}` cites `{row}`, which is not a file"));
        }
        match (crates.get(*name), why) {
            (None, _) => out.push(format!("`{name}` is listed but is no crate with a src")),
            (Some(c), Missing::Roster) if c.runs_roster => out.push(format!(
                "`{name}` runs a roster now — remove it from KNOWN_MISSING"
            )),
            (Some(c), Missing::Roster) if !has_sites(&c.census) => out.push(format!(
                "`{name}` decides nothing now — remove it from KNOWN_MISSING"
            )),
            (Some(c), Missing::ClosedTable) if !c.runs_roster => out.push(format!(
                "`{name}` runs no roster: list it as `Missing::Roster`"
            )),
            _ => {}
        }
    }
    for (name, c) in &crates {
        if has_sites(&c.census) && !c.runs_roster && !listed.contains_key(name.as_str()) {
            out.push(format!(
                "`{name}` decides predicate names and runs no roster — give it \
                 `tests/recourse_roster.rs` on `test_utils::source::roster_violations`, or \
                 list it in KNOWN_MISSING with the row that will"
            ));
        }
    }
    let union: BTreeSet<String> = crates
        .values()
        .flat_map(|c| c.census.names.iter().cloned())
        .collect();
    assert!(!union.is_empty(), "the census read no name anywhere");
    let crates_dir = root.join("crates");
    let spelled: BTreeMap<&str, BTreeSet<String>> = KNOWN_MISSING
        .iter()
        .map(|(name, _, _)| (*name, spelled_literals(&crates_dir.join(name).join("src"))))
        .collect();
    let mut holding: BTreeSet<&str> = BTreeSet::new();
    for (name, c) in &crates {
        for site in unresolved_references(&c.census, &union) {
            let referenced = site.rsplit(": ").next().unwrap_or(&site);
            let holders: Vec<&str> = spelled
                .iter()
                .filter(|(_, literals)| literals.contains(referenced))
                .map(|(holder, _)| *holder)
                .collect();
            if holders.is_empty() {
                out.push(format!(
                    "crates/{name}/src/{site} — a predicate literal no crate decides: \
                     spell it as the door does, route it through the door's const, or mark \
                     a deliberate negative `// UNDECIDED-NAME: <why>` on the line above"
                ));
            }
            holding.extend(holders);
        }
    }
    for (name, why, _) in KNOWN_MISSING {
        if *why == Missing::ClosedTable && !holding.contains(name) {
            out.push(format!(
                "`{name}`'s closed table holds no unresolved reference now — remove it from \
                 KNOWN_MISSING"
            ));
        }
    }
    assert!(out.is_empty(), "{}", out.join("\n"));
}
