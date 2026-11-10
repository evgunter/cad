//! **Every predicate name is minted once, tree-wide.**
//!
//! A predicate name keys its reader's `k_stats` stream and its ledger
//! row (`docs/predicate-dimension-audit.md`), so two readers sharing one
//! would merge their samples under it. Names are minted two ways:
//!
//! - **as a row bundle's rows**, held in the minting crate's row registry
//!   (`geom::rows`, `geom_brep::rows`, `topo::rows`, `sweep::rows`), whose own `const`
//!   assertion keeps a crate's names distinct;
//! - **as a plain site**, a literal at a `k_stats` door.
//!
//! This is the one place every minting crate is seen at once, so it is
//! where the names of different crates are checked against each other.
//! It reads the registries' rosters today; the plain-site census
//! (`work/onecopy/plain-predicate-sites-have-no-tree-wide-census.md`)
//! joins this same test rather than a second one.

#![allow(clippy::unwrap_used)]

/// Every registry's roster, by crate.
fn rosters() -> Vec<(&'static str, &'static [&'static [&'static str]])> {
    vec![
        ("geom", geom::rows::ROSTER),
        ("geom-brep", geom_brep::rows::ROSTER),
        ("topo", topo::rows::ROSTER),
        ("sweep", sweep::rows::ROSTER),
    ]
}

#[test]
fn every_predicate_name_is_minted_once_tree_wide() {
    let mut seen: std::collections::BTreeMap<&str, &str> = std::collections::BTreeMap::new();
    let mut shared = Vec::new();
    for (krate, roster) in rosters() {
        assert!(
            !roster.is_empty() && roster.iter().all(|bundle| !bundle.is_empty()),
            "{krate}'s roster was read, every bundle naming its rows"
        );
        for bundle in roster {
            for &name in *bundle {
                if let Some(first) = seen.insert(name, krate) {
                    shared.push(format!("{name}: {first} and {krate}"));
                }
            }
        }
    }
    assert!(shared.is_empty(), "names minted twice: {shared:#?}");
}

/// **Only a crate's row registry mints a two-row turn bundle**: a call of
/// `TurnRows::registered` in any crate's `src` lies in a `rows.rs`, where
/// the registry's own bundle type wraps it. Anywhere else it would mint
/// two names outside every roster.
#[test]
fn only_a_registry_mints_a_two_row_turn_bundle() {
    let crates = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut stray = Vec::new();
    let mut stack: Vec<std::path::PathBuf> = std::fs::read_dir(&crates)
        .unwrap()
        .map(|e| e.unwrap().path().join("src"))
        .filter(|p| p.is_dir())
        .collect();
    let mut read = 0;
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|x| x == "rs") {
                read += 1;
                let text = std::fs::read_to_string(&path).unwrap();
                let defines = path.ends_with("geom-core/src/period.rs");
                let calls = text
                    .lines()
                    .filter(|l| !l.trim_start().starts_with("//"))
                    .any(|l| l.contains("TurnRows::registered("));
                if calls && !defines && !path.ends_with("rows.rs") {
                    stray.push(path.display().to_string());
                }
            }
        }
    }
    assert!(read > 100, "the crates' sources were read: {read} files");
    assert!(
        stray.is_empty(),
        "two-row turn bundles minted outside a registry: {stray:#?}"
    );
}
