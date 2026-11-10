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
//! roster's carriers (`test_utils::source::declared_carriers`), so a
//! fixture that outlives a rename is red here. A deliberate negative — a
//! hand-built cause, a name asserted absent — says so at the site with
//! `// UNDECIDED-NAME: <why>` on the line above, not in a list. A name
//! only a listed crate spells is held by that crate's entry: its roster
//! is what will read it, and once it runs one the name has to resolve.
//!
//! **When it runs.** It walks the whole tree, so it is in the slow set:
//! a PR runs it only when the PR names it, and the nightly every night.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};
use test_utils::source::{
    PredicateCensus, declared_carriers, predicate_census, repo_root, spelled_literals,
    unresolved_references,
};

/// The crates that decide a name and run no roster yet, each with the
/// row that gives it one.
const KNOWN_MISSING: &[(&str, &str)] = &[
    (
        "editor-core",
        "work/onecopy/editor-core-runs-no-predicate-name-roster.md",
    ),
    ("geom", "work/onecopy/geom-runs-no-predicate-name-roster.md"),
    (
        "geom-core",
        "work/onecopy/geom-core-runs-no-predicate-name-roster.md",
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
        let roster = std::fs::read_to_string(dir.join("tests/recourse_roster.rs")).ok();
        let runs_roster = roster
            .as_deref()
            .is_some_and(|text| text.contains("roster_violations("));
        let carriers = roster.as_deref().map(declared_carriers).unwrap_or_default();
        let name = dir.file_name().unwrap().to_string_lossy().into_owned();
        out.insert(name, Crate {
            census: predicate_census(&src, &carriers),
            runs_roster,
        });
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
    let listed: BTreeMap<&str, &str> = KNOWN_MISSING.iter().copied().collect();
    assert_eq!(listed.len(), KNOWN_MISSING.len(), "a crate is listed twice");
    let mut out = Vec::new();
    for (name, row) in KNOWN_MISSING {
        if !root.join(row).is_file() {
            out.push(format!("`{name}` cites `{row}`, which is not a file"));
        }
        match crates.get(*name) {
            None => out.push(format!("`{name}` is listed but is no crate with a src")),
            Some(c) if c.runs_roster => out.push(format!(
                "`{name}` runs a roster now — remove it from KNOWN_MISSING"
            )),
            Some(c) if !has_sites(&c.census) => out.push(format!(
                "`{name}` decides nothing now — remove it from KNOWN_MISSING"
            )),
            Some(_) => {}
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
    let held: BTreeSet<String> = KNOWN_MISSING
        .iter()
        .flat_map(|(name, _)| {
            spelled_literals(&repo_root(env!("CARGO_MANIFEST_DIR")).join("crates").join(name).join("src"))
        })
        .collect();
    for (name, c) in &crates {
        for site in unresolved_references(&c.census, &union) {
            let referenced = site.rsplit(": ").next().unwrap_or(&site);
            if !held.contains(referenced) {
                out.push(format!(
                    "crates/{name}/src/{site} — a predicate literal no crate decides: \
                     spell it as the door does, route it through the door's const, or mark \
                     a deliberate negative `// UNDECIDED-NAME: <why>` on the line above"
                ));
            }
        }
    }
    assert!(out.is_empty(), "{}", out.join("\n"));
}
