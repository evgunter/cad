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

/// **Every row bundle is minted in its crate's registry, and nowhere
/// else**, read once for every crate here, over the shared Rust reader
/// (`test_utils::source`, comments and literals blanked):
///
/// - a call of `TurnRows::registered`, the one door to a two-row turn
///   bundle outside geom-core, lies in a `rows.rs`, where that crate's
///   registry wraps it in a bundle type of its own;
/// - a `rows.rs` declares no `const` of one of its bundle types outside
///   its `row_registry!` blocks, where it would escape the derived
///   roster. Its bundle types are read off the consts inside the blocks,
///   so a new type is covered the day its first const lands.
#[test]
fn every_row_bundle_is_minted_in_its_registry() {
    use test_utils::source::{balanced_end, code_only, repo_root, rust_sources};
    let crates = repo_root(env!("CARGO_MANIFEST_DIR")).join("crates");
    let mut minted_outside = Vec::new();
    let mut stray = Vec::new();
    let mut registries = 0;
    for entry in std::fs::read_dir(&crates).unwrap() {
        let src = entry.unwrap().path().join("src");
        if !src.is_dir() {
            continue;
        }
        for path in rust_sources(&src) {
            let code = code_only(&std::fs::read_to_string(&path).unwrap());
            let is_registry = path.ends_with("rows.rs");
            if !is_registry
                && !path.ends_with("geom-core/src/period.rs")
                && code.contains("TurnRows::registered(")
            {
                minted_outside.push(path.display().to_string());
            }
            if !is_registry || !code.contains("row_registry!") {
                continue;
            }
            registries += 1;
            let mut blocks = Vec::new();
            let mut at = 0;
            while let Some(hit) = code[at..].find("row_registry!") {
                let open = at + hit + code[at + hit..].find('{').unwrap();
                let end = balanced_end(&code, open).unwrap();
                blocks.push(open..end);
                at = end;
            }
            // `const NAME: TYPE`, at each `const ` that starts a declaration.
            let decls: Vec<(usize, &str)> = code
                .match_indices("const ")
                .filter(|(k, _)| *k == 0 || !code.as_bytes()[k - 1].is_ascii_alphanumeric())
                .filter_map(|(k, _)| {
                    let rest = &code[k + "const ".len()..];
                    let (name, after) = rest.split_once(':')?;
                    if !name.trim().chars().all(|c| c.is_alphanumeric() || c == '_') {
                        return None;
                    }
                    let ty = after.split(['=', ';']).next()?.trim();
                    Some((k, ty))
                })
                .collect();
            let inside = |k: usize| blocks.iter().any(|b| b.contains(&k));
            let types: std::collections::BTreeSet<&str> = decls
                .iter()
                .filter(|(k, _)| inside(*k))
                .map(|&(_, ty)| ty)
                .collect();
            for &(k, ty) in &decls {
                if !inside(k) && types.contains(ty) {
                    stray.push(format!("{}: const of {ty} at byte {k}", path.display()));
                }
            }
        }
    }
    assert!(registries >= 3, "the registries were read: {registries}");
    assert!(
        minted_outside.is_empty(),
        "two-row turn bundles minted outside a registry: {minted_outside:#?}"
    );
    assert!(
        stray.is_empty(),
        "bundle consts outside their registry's block: {stray:#?}"
    );
}
