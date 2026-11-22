//! **A torus verdict is named only in its home**
//! (`crates/geom-brep/src/torus_convention.rs`).
//!
//! The type holds the rule: `TorusRefusal`'s verdict is a private field,
//! and `TorusVerdict` is private to the home's module, so no other module
//! of any crate can build, match or import one. A reader reads a refusal's
//! standing (`TorusRefusal::standing`), and a door builds a refusal
//! through `TorusRefusal::of`, `require` or `decided`.
//!
//! This row covers what the type cannot: the type being widened again (a
//! `pub`, a re-export) and then named. It refuses the identifier
//! `TorusVerdict` anywhere in the code of a `.rs` file under `crates/`
//! other than the home: a pattern or an expression (`TorusVerdict::…`,
//! across lines or in a `let … else`), a `use` of it, under an alias or
//! as a glob. It reads code only (comments and string literals are
//! blanked), and test code too, since a test outside the home cannot name
//! the type either. It does not see a glob of the home's module
//! (`torus_convention::*`) that would bring the type in unnamed; it
//! refuses that glob too.
#![allow(clippy::unwrap_used)]

use std::path::PathBuf;

use test_utils::source::{code_only, repo_root, rust_sources};

/// The convention's home, where its verdict is named.
const HOME: &str = "crates/geom-brep/src/torus_convention.rs";

/// The lines of `text`'s code that name the verdict, or glob the home's
/// module, as `(line number, line)`.
fn names_the_verdict(text: &str) -> Vec<(usize, String)> {
    let code = code_only(text);
    code.lines()
        .enumerate()
        .filter(|(_, line)| {
            let word = line.match_indices("TorusVerdict").any(|(at, needle)| {
                let before = line[..at].chars().next_back();
                let after = line[at + needle.len()..].chars().next();
                let ident = |c: Option<char>| c.is_some_and(|c| c.is_alphanumeric() || c == '_');
                !ident(before) && !ident(after)
            });
            let glob = line.contains("torus_convention::*")
                || line.contains("torus_convention::{self")
                || line.contains("torus_convention::{*");
            word || glob
        })
        .map(|(n, line)| (n + 1, line.trim().to_owned()))
        .collect()
}

#[test]
fn a_torus_verdict_is_named_only_in_its_home() {
    let root = repo_root(env!("CARGO_MANIFEST_DIR"));
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in std::fs::read_dir(root.join("crates")).unwrap() {
        files.extend(rust_sources(&entry.unwrap().path()));
    }
    let home = std::fs::read_to_string(root.join(HOME)).unwrap();
    assert!(
        !names_the_verdict(&home).is_empty(),
        "the home names the verdict, so the needle is the type's name"
    );
    let mut named = Vec::new();
    for path in &files {
        let rel = path
            .strip_prefix(&root)
            .unwrap()
            .to_string_lossy()
            .into_owned();
        if rel == HOME {
            continue;
        }
        let text = std::fs::read_to_string(path).unwrap();
        for (n, line) in names_the_verdict(&text) {
            named.push(format!("{rel}:{n}: {line}"));
        }
    }
    assert!(
        files.len() > 100,
        "the crates' sources were read: {}",
        files.len()
    );
    assert!(
        named.is_empty(),
        "a torus verdict named outside its home ({HOME}): read the refusal's standing \
         (`TorusRefusal::standing`), or build it through `TorusRefusal::of`, `require` or \
         `decided`:\n{}",
        named.join("\n")
    );
}

/// The row's reach, on planted sources: (a) a multi-line struct pattern,
/// (b) the same pattern in a `let … else`, (c) a `use` under an alias, (d)
/// a glob of the type's variants, (e) a glob of the home's module, and a
/// build (no exemption) all fire; a comment, a string and an unrelated
/// identifier that contains the name do not.
#[test]
fn the_row_reads_every_naming_and_no_mention() {
    let fires = [
        (
            "(a) a multi-line pattern",
            "match r {\n    TorusRefusal {\n        verdict: TorusVerdict::Refused(v),\n        ..\n    } => v,\n}\n",
        ),
        (
            "(b) a let-else",
            "let TorusRefusal {\n    verdict: crate::torus_convention::TorusVerdict::Refused(v), ..\n} = r else {\n    return;\n};\n",
        ),
        (
            "(c) an aliased use",
            "use crate::torus_convention::TorusVerdict as V;\nfn f(v: V) {}\n",
        ),
        (
            "(d) a glob of its variants",
            "use geom_brep::TorusVerdict::*;\nfn f() {}\n",
        ),
        (
            "(e) a glob of the home's module",
            "use crate::torus_convention::*;\n",
        ),
        (
            "a build",
            "let r = TorusRefusal { convention, verdict: geom_brep::TorusVerdict::Refused(x) };\n",
        ),
    ];
    for (what, source) in fires {
        assert!(!names_the_verdict(source).is_empty(), "{what} is not seen");
    }
    let quiet = "// TorusVerdict in a comment\nlet s = \"TorusVerdict\";\nlet t = MyTorusVerdicts;\nlet u = TorusVerdictish;\n";
    assert!(
        names_the_verdict(quiet).is_empty(),
        "a mention fired: {:?}",
        names_the_verdict(quiet)
    );
}
