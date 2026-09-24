//! **The `CertifiedEnclosure` impls in the tree, counted against the
//! door values' wiring rows** — the which-scalars half of the census
//! the deleted lane traits' identity tests carried.
//!
//! The type system carries the which-body half: `QuadLane::certified`
//! and `RegionLane::certified` are each their value's one constructor
//! and their `impl` blocks are bounded on `CertifiedBounds`, so a
//! scalar without the right cannot hold a value. It does not carry the
//! which-scalars half: each `wiring_rows` module is a hand-written
//! list of the scalars whose door is pinned by pointer identity, and a
//! new `impl CertifiedEnclosure` would form both doors and owe a row
//! in each without anything going red. This census is that red. It
//! reads the impls off the code view of every `crates/*/src` file and
//! the instantiations of each roster's needle
//! (`holds_the_certified_quadrature::<…>()` off `props.rs`,
//! `holds_the_certified_region_doors::<…>()` off `chart_region.rs`),
//! and every roster must agree exactly with the impls after the ones
//! that are not door scalars — each with its reason, below — are
//! taken out. Both directions are checked: an impl with no row is a
//! missing pin, and a row for a type with no impl is a stale roster.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use test_utils::source::{code_only, repo_root, rust_sources};

/// The rosters: where each door's wiring rows live, and the needle
/// its pointer-identity helper is instantiated through.
const ROSTERS: [(&str, &str); 2] = [
    (
        "crates/topo/src/props.rs",
        "holds_the_certified_quadrature::<",
    ),
    (
        "crates/topo/src/chart_region.rs",
        "holds_the_certified_region_doors::<",
    ),
];

/// The impls that are not door scalars, each with the reason no wiring
/// row is owed. An entry here is an exemption and has to earn it; every
/// `CertifiedEnclosure` implementor in the tree is a door scalar today.
const NOT_A_DOOR_SCALAR: [(&str, &str); 0] = [];

/// The head of a type name: the path stripped and the generic arguments
/// dropped, so `geom_core::Sym<f64>`, `Sym<T>` and `Sym` are one type.
fn head(ty: &str) -> String {
    let ty = ty.trim();
    let ty = ty.split('<').next().unwrap_or(ty);
    ty.rsplit("::").next().unwrap_or(ty).trim().to_string()
}

/// The type an impl header names after `for `: everything up to the
/// body's `{` or a `where` clause, `where` read as a token rather than
/// as a letter — a type whose path holds a `w` (`wrapper::Sym<T>`) is
/// one type, not a truncated one.
fn impl_target(rest: &str) -> &str {
    let rest = rest.split('{').next().unwrap_or(rest);
    let is_boundary = |c: Option<char>| c.is_none_or(|c| !(c.is_alphanumeric() || c == '_'));
    rest.match_indices("where")
        .find(|(i, m)| {
            is_boundary(rest[..*i].chars().next_back())
                && is_boundary(rest[i + m.len()..].chars().next())
        })
        .map_or(rest, |(i, _)| &rest[..i])
}

/// Every `impl … CertifiedEnclosure for X` in the tree's crate sources,
/// as the head of `X`.
fn impls() -> BTreeSet<String> {
    let root = repo_root(env!("CARGO_MANIFEST_DIR"));
    let mut out = BTreeSet::new();
    let crates = std::fs::read_dir(root.join("crates")).expect("crates/ lists");
    for entry in crates {
        let src = entry.expect("a crate entry").path().join("src");
        if !src.is_dir() {
            continue;
        }
        for file in rust_sources(&src) {
            let text = std::fs::read_to_string(&file).expect("a crate source reads");
            for line in code_only(&text).lines() {
                let Some(rest) = line.split("CertifiedEnclosure for ").nth(1) else {
                    continue;
                };
                assert!(
                    line.trim_start().starts_with("impl"),
                    "{}: `CertifiedEnclosure for` outside an impl header: {line}",
                    file.display()
                );
                out.insert(head(impl_target(rest)));
            }
        }
    }
    out
}

/// Every scalar the roster at `wiring` instantiates its pointer-identity
/// check at, read through `needle`.
fn wired(wiring: &str, needle: &str) -> BTreeSet<String> {
    let path = repo_root(env!("CARGO_MANIFEST_DIR")).join(wiring);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{wiring} reads: {e}"));
    let code = code_only(&text);
    let mut out = BTreeSet::new();
    for (i, _) in code.match_indices(needle) {
        let rest = &code[i + needle.len()..];
        let ty = rest.split(">()").next().expect("a turbofish closes");
        out.insert(head(ty));
    }
    assert!(
        !out.is_empty(),
        "{wiring} instantiates no `{needle}…>()` — the roster this census reads is gone"
    );
    out
}

/// The header reader stops at the body or at a `where` TOKEN, and at
/// nothing else: a `w` inside the type's path is part of the type.
#[test]
fn the_impl_target_ends_at_the_body_or_a_where_token() {
    assert_eq!(impl_target("f64 {").trim(), "f64");
    assert_eq!(
        impl_target("wrapper::Sym<T> where T: CertifiedEnclosure {").trim(),
        "wrapper::Sym<T>"
    );
    assert_eq!(
        impl_target("somewhere::Wide<T>").trim(),
        "somewhere::Wide<T>"
    );
}

#[test]
fn every_certifying_scalar_has_a_wiring_row_and_every_row_a_scalar() {
    let impls = impls();
    let exempt: BTreeSet<String> = NOT_A_DOOR_SCALAR
        .iter()
        .map(|(ty, _)| (*ty).to_string())
        .collect();
    for ty in &exempt {
        assert!(
            impls.contains(ty),
            "{ty} is exempted here as not a door scalar but implements `CertifiedEnclosure` \
             nowhere in the tree — the exemption is stale"
        );
    }
    let door_scalars: BTreeSet<String> = impls.difference(&exempt).cloned().collect();
    for (wiring, needle) in ROSTERS {
        let wired = wired(wiring, needle);
        assert_eq!(
            door_scalars, wired,
            "the scalars implementing `CertifiedEnclosure` (less the exemptions {exempt:?}) and \
             the scalars {wiring}'s `wiring_rows` pins its door at must be the same set: an \
             impl with no row forms the door's `certified()` unpinned, and a row with no impl \
             is a stale roster. Add the wiring row in {wiring}, or the exemption with its \
             reason here"
        );
    }
}
