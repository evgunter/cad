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
