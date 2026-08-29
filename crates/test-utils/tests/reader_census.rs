//! **The census of source-text guards, and the reason it is a test.**
//!
//! A guard that reads `.rs` text needs a Rust reader, and for most of
//! this tree's history each one wrote its own: five in Rust, at
//! whatever competence its author needed that day, plus an awk lexer
//! under `scripts/gates/`. The count went 7 → 9 → 11 → 12 in a single
//! session, each step a differently-*shaped* sweep rather than a
//! deeper one, and then moved a fourth time by a lane LANDING a new
//! reader while the finding was in review.
//!
//! So the population is not a list, and a fix that converts today's
//! members closes the smaller half. What closes the larger half is
//! this: **every site that reads Rust source as text is enumerated
//! here, and a new one reds this row until someone writes its line.**
//! The next reader cannot arrive silently, because arriving is what
//! this row detects.
//!
//! # What a red here means
//!
//! Not *"you did something wrong"*. It means a site that reads source
//! text arrived or moved, and the ledger owes it a disposition. Three
//! honest ones:
//!
//! - it calls [`test_utils::source`] — add the line, say which view;
//! - it reads a language that is not Rust (a STEP file, a manifest, a
//!   `.cad` document) — add the line, say which;
//! - it is a new hand-rolled Rust reader — do not add the line. Use
//!   the shared lexer. That is the whole point of the row.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::{Path, PathBuf};

use test_utils::source::{code_and_literals, rust_sources};

/// The Rust trees this census covers. `crates/`, `demos/`, `tools/`
/// and `benches/` are the whole of the workspace; `interval-
/// transcendentals/` is a separate workspace root the kernel
/// path-depends on and is included because a guard there would be
/// exactly as invisible as one anywhere else.
///
/// **`scripts/` is deliberately absent and is not a gap in the class**:
/// the gates there read Rust source too, through `scripts/gates/`
/// `lib.sh`'s `gate_rust_code`, which is their own shared awk lexer
/// with its own selftests. Two homes in two languages is a stated
/// state of the tree, not an accident this row can catch; what it
/// could catch — a gate that stops using `gate_rust_code` — is a
/// shell-side row on the track that owns `scripts/gates/`.
const TREES: [&str; 5] = [
    "crates",
    "demos",
    "tools",
    "benches",
    "interval-transcendentals",
];

/// A site that reads Rust source as text, and what reader it uses.
struct Entry {
    /// Path relative to the repository root, `/`-separated.
    path: &'static str,
    /// Why it is not a new hand-rolled Rust reader.
    disposition: Disposition,
}

#[derive(PartialEq, Eq)]
enum Disposition {
    /// Reads Rust source through [`test_utils::source`]. The
    /// destination for every member of this class.
    Shared,
    /// The home itself.
    Home,
    /// A hand-rolled Rust reader that has not been converted yet, with
    /// the track that owns its file. **Every one of these is a defect
    /// with an owner**, and the list only ever shrinks: a lane that
    /// converts one deletes its line, and no lane may add one.
    Unconverted(&'static str),
    /// Reads a language that is not Rust, so the Rust lexer is not what
    /// it wants. The named language is the claim.
    NotRust(&'static str),
}

use Disposition::{Home, NotRust, Shared, Unconverted};

/// **The ledger.** One line per site that reads source text.
///
/// Sorted by path. Adding a line is a deliberate act and the enum above
/// says which acts are honest.
const LEDGER: &[Entry] = &[];

/// The repository root, for both ways the suite runs: a plain
/// `cargo test` against the baked manifest dir, and a nextest ARCHIVE
/// replayed with the per-test cwd remapped to the crate root.
fn repo_root() -> PathBuf {
    let baked = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    if baked.join("Cargo.toml").is_file() {
        return baked;
    }
    let cwd = std::env::current_dir().expect("a working directory");
    let up = cwd.join("../..");
    assert!(
        up.join("Cargo.toml").is_file(),
        "neither {baked:?} nor {up:?} is the repository root"
    );
    up
}

/// `code` with every `#[path = "…"]` module mount blanked.
///
/// A mount names a `.rs` file and reads nothing; leaving it in makes
/// every `tests/all.rs` in the tree a hit and drowns the signal.
fn without_module_mounts(code: &str) -> String {
    let mut out = String::with_capacity(code.len());
    let mut rest = code;
    while let Some(at) = rest.find("#[path") {
        out.push_str(&rest[..at]);
        rest = &rest[at..];
        let end = rest.find(']').map_or(rest.len(), |e| e + 1);
        rest = &rest[end..];
    }
    out.push_str(rest);
    out
}

/// Whether `code` (a comments-blanked view) reads Rust source as text.
///
/// **Three independent shapes, because a single-shaped sweep is what
/// under-reported this class three times** — the count moved at every
/// re-sweep, and never by looking harder in the same place. A site can
/// name a `.rs` file without spelling a comment delimiter; it can walk
/// a source tree without naming any file; and it can lex Rust comments
/// over text it obtained some third way.
fn reads_rust_source(code: &str) -> bool {
    // (1) Names a `.rs` file for something other than a module mount.
    let names_a_source_file = without_module_mounts(code).contains(".rs\"");
    // (2) Walks a source tree.
    let walks_a_source_tree = ["rust_sources(", "crate_sources(", "src_root("]
        .iter()
        .any(|n| code.contains(n))
        || (code.contains("read_dir(") && code.contains("\"rs\""));
    // (3) Spells a Rust comment delimiter as a literal — the tell of a
    // hand-rolled lexer, wherever its text came from.
    let lexes_rust_comments = ["\"//\"", "\"///\"", "\"//!\"", "\"/*\"", "\"*/\"", "b'/'"]
        .iter()
        .any(|tok| code.contains(tok));
    names_a_source_file || walks_a_source_tree || lexes_rust_comments
}

#[test]
fn every_site_that_reads_rust_source_is_in_the_ledger() {
    let root = repo_root();
    let mut found: Vec<String> = Vec::new();
    for tree in TREES {
        let dir = root.join(tree);
        if !dir.is_dir() {
            continue;
        }
        for path in rust_sources(&dir) {
            if path.components().any(|c| c.as_os_str() == "target") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("a readable source file");
            if reads_rust_source(&code_and_literals(&text)) {
                let rel = path
                    .strip_prefix(&root)
                    .expect("a walked file lies under the root")
                    .to_string_lossy()
                    .replace('\\', "/");
                found.push(rel);
            }
        }
    }
    found.sort();
    let ledger: Vec<&str> = LEDGER.iter().map(|e| e.path).collect();
    let unlisted: Vec<&String> = found.iter().filter(|f| !ledger.contains(&f.as_str())).collect();
    let stale: Vec<&&str> = ledger.iter().filter(|l| !found.iter().any(|f| f == *l)).collect();
    assert!(
        unlisted.is_empty() && stale.is_empty(),
        "the source-reader ledger no longer matches the tree.\n  \
         arrived or moved (owe a ledger line): {unlisted:#?}\n  \
         listed but no longer reading source (delete the line): {stale:#?}"
    );
}

#[test]
fn the_unconverted_list_only_shrinks() {
    let unconverted = LEDGER
        .iter()
        .filter(|e| matches!(e.disposition, Unconverted(_)))
        .count();
    assert!(
        unconverted <= UNCONVERTED_CEILING,
        "{unconverted} unconverted hand-rolled Rust readers, ceiling {UNCONVERTED_CEILING}. \
         This number is a debt, not a budget: a lane that converts one lowers it, and \
         nothing may raise it."
    );
    let _ = (Home, Shared, NotRust(""));
}

/// The number of hand-rolled Rust readers still outside the shared
/// lexer. **A ceiling, never a target** — see the row above.
const UNCONVERTED_CEILING: usize = 0;
