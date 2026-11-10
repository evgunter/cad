//! **A torus verdict's standing is decided in one home**
//! (`geom_brep::TorusRefusal::standing`, in
//! `crates/geom-brep/src/torus_convention.rs`). Every reader of a torus
//! refusal reads that standing, never its `verdict` itself. This row walks
//! every crate's production code and refuses a `TorusVerdict` variant
//! named outside the home, except where it builds a refusal: a struct
//! field `verdict: TorusVerdict::Refused(x)`, `x` a binding. An undecided
//! verdict is built only by the convention's own decision
//! (`TorusRefusal::require`), so `TorusVerdict::Undecided` is not named
//! outside the home at all.
//!
//! Test code is not read: a test may name the variant it pins. That means
//! `tests/`, `#[cfg(test)]` items, and files mounted as `#[cfg(test)] mod`.
#![allow(clippy::unwrap_used)]

use std::path::{Path, PathBuf};

use test_utils::source::{attributed_extent, code_only, repo_root, rust_sources};

/// The convention's home, where its verdict is matched.
const HOME: &str = "crates/geom-brep/src/torus_convention.rs";

/// `code` with every `#[cfg(test)]` item blanked, and the names of the
/// modules it mounts under `#[cfg(test)]` (`mod name;`).
fn production(code: &str) -> (String, Vec<String>) {
    let mut out = code.to_owned();
    let mut mounted = Vec::new();
    for (at, _) in code.match_indices("#[cfg(test)]") {
        let Some(extent) = attributed_extent(code, at) else {
            continue;
        };
        let item = code[extent.clone()]
            .trim_start_matches("#[cfg(test)]")
            .trim();
        let item = item.strip_prefix("pub(crate) ").unwrap_or(item);
        if let Some(rest) = item.strip_prefix("mod ") {
            let name: String = rest
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            if rest[name.len()..].trim_start().starts_with(';') {
                mounted.push(name);
            }
        }
        let blank: String = code[extent.clone()]
            .chars()
            .map(|c| if c == '\n' { '\n' } else { ' ' })
            .collect();
        out.replace_range(extent, &blank);
    }
    (out, mounted)
}

/// Whether `file` is a module its parent mounts under `#[cfg(test)]`: the
/// parent's `mod {stem};` sits inside a `#[cfg(test)]` item.
fn test_mounted(file: &Path) -> bool {
    let stem = file.file_stem().unwrap().to_string_lossy().into_owned();
    let dir = file.parent().unwrap();
    let (name, parents) = if stem == "mod" {
        let name = dir.file_name().unwrap().to_string_lossy().into_owned();
        let up = dir.parent().unwrap();
        (
            name.clone(),
            vec![
                up.join(format!("{name}.rs")),
                up.join("mod.rs"),
                up.join("lib.rs"),
            ],
        )
    } else {
        let dir_name = dir.file_name().unwrap().to_string_lossy().into_owned();
        let up = dir.parent().unwrap();
        (
            stem,
            vec![
                dir.join("mod.rs"),
                dir.join("lib.rs"),
                dir.join("main.rs"),
                up.join(format!("{dir_name}.rs")),
            ],
        )
    };
    parents.iter().filter(|p| p.is_file()).any(|parent| {
        let (_, mounted) = production(&code_only(&std::fs::read_to_string(parent).unwrap()));
        mounted.contains(&name)
    })
}

/// Whether the `TorusVerdict::Refused(` at `at` on `line` builds a refusal:
/// the value of a `verdict:` field, its argument a binding (`x` or `*x`).
fn builds(line: &str, at: usize) -> bool {
    let before = line[..at].trim_end();
    let before = before
        .strip_suffix("geom_brep::")
        .or_else(|| before.strip_suffix("crate::torus_convention::"))
        .or_else(|| before.strip_suffix("crate::"))
        .unwrap_or(before)
        .trim_end();
    let after = &line[at + "TorusVerdict::Refused(".len()..];
    let arg: String = after.chars().take_while(|&c| c != ')').collect();
    let arg = arg.trim().trim_start_matches('*');
    before.ends_with("verdict:")
        && !arg.is_empty()
        && arg != "_"
        && arg.chars().all(|c| c.is_alphanumeric() || c == '_')
        && !line.contains("=>")
}

#[test]
fn a_torus_verdict_is_matched_only_in_its_home() {
    let root = repo_root(env!("CARGO_MANIFEST_DIR"));
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in std::fs::read_dir(root.join("crates")).unwrap() {
        let src = entry.unwrap().path().join("src");
        if src.is_dir() {
            files.extend(rust_sources(&src));
        }
    }
    // Only a file whose raw text names the type is lexed: the code-only
    // view can only lose a needle the raw text holds, never add one. Each
    // is read unless its parent module mounts it under `#[cfg(test)]`.
    let mut named = Vec::new();
    let mut read = 0usize;
    for path in &files {
        let rel = path
            .strip_prefix(&root)
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let text = std::fs::read_to_string(path).unwrap();
        if rel == HOME || !text.contains("TorusVerdict") || test_mounted(path) {
            continue;
        }
        read += 1;
        let (code, _) = production(&code_only(&text));
        for (n, line) in code.lines().enumerate() {
            if line.contains("TorusVerdict::Undecided") {
                named.push(format!("{rel}:{}: {}", n + 1, line.trim()));
            }
            for (at, _) in line.match_indices("TorusVerdict::Refused(") {
                if !builds(line, at) {
                    named.push(format!("{rel}:{}: {}", n + 1, line.trim()));
                }
            }
        }
    }
    assert!(read >= 3, "the files that name the type were read: {read}");
    assert!(
        named.is_empty(),
        "a torus verdict named outside its home ({HOME}), other than to build a \
         refusal: read its standing (`TorusRefusal::standing`) instead:\n{}",
        named.join("\n")
    );
}

/// The rule's two sides, on planted lines: a struct field building a
/// refusal passes; a match arm, a wildcard pattern and an undecided
/// verdict do not.
#[test]
fn the_rule_reads_a_build_apart_from_a_match() {
    let field = "    verdict: geom_brep::TorusVerdict::Refused(verdict),";
    let deref = "    verdict: geom_brep::TorusVerdict::Refused(*verdict),";
    let arm = "    geom_brep::TorusVerdict::Refused(verdict) => FrameError::DegenerateTorus {";
    let wild = "    verdict: geom_brep::TorusVerdict::Refused(_),";
    let at = |l: &str| l.find("TorusVerdict::Refused(").unwrap();
    assert!(builds(field, at(field)) && builds(deref, at(deref)));
    assert!(!builds(arm, at(arm)) && !builds(wild, at(wild)));
}
