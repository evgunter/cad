//! **The census of source-text guards, and the reason it is a test.**
//!
//! A guard that reads `.rs` text needs a Rust reader, and each one used
//! to write its own. **The population of such guards is not a list.**
//! Four sweeps of it returned four different counts, none by looking
//! harder in the same place, and the last of them moved because a lane
//! LANDED a new reader while the sweep was being reviewed — so a fix
//! that converts today's members and stops closes the smaller half.
//!
//! What closes the larger half is this row: **every site that reads
//! Rust source as text is enumerated here, and a new one reds until
//! someone writes its line.** The next reader cannot arrive silently,
//! because arriving is what this row detects.
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

/// The repository's own directories, skipped by NAME rather than by a
/// roster: a build directory, and anything hidden.
///
/// **There is no list of trees to cover.** One recursive walk from the
/// repository root sees every `.rs` file the repository tracks, so a
/// new top-level Rust tree is covered the day it lands; a
/// five-element roster with a `continue` on a missing entry narrows
/// coverage silently on a rename, which is the class this whole file
/// is about.
///
/// **`scripts/` needs no exclusion and is not a gap in the class**: it
/// holds no `.rs` file, so the walk never reaches it. Its gates read
/// Rust source through `scripts/gates/lib.sh`'s `gate_rust_code`, a
/// second shared reader written in awk with its own selftests — a
/// second home, in a second language, which this row cannot see and
/// does not claim to.
const SKIPPED_DIRS: [&str; 1] = ["target"];

/// A site that reads Rust source as text, and what reader it uses.
struct Entry {
    /// Path relative to the repository root, `/`-separated.
    path: &'static str,
    /// Why it is not a new hand-rolled Rust reader.
    disposition: Disposition,
}

enum Disposition {
    /// Reads Rust source through [`test_utils::source`]. The
    /// destination for every member of this class.
    Shared,
    /// The home itself.
    Home,
    /// Reads Rust source through something other than
    /// [`test_utils::source`] — a hand-rolled reader, or one of
    /// `topo`'s two crate-private blankers. The payload names the
    /// track that owes the conversion, **or `unowned` where the
    /// partition has no track for the file**; an unowned entry is not
    /// an exemption, it is a second finding stacked on the first, and
    /// it says so in its own text.
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
const LEDGER: &[Entry] = &[
    Entry {
        path: "crates/bvh/tests/aggregator_headers.rs",
        disposition: Shared, // prose + literal views
    },
    Entry {
        path: "crates/bvh/tests/all.rs",
        disposition: Shared, // mount guard, literal view
    },
    Entry {
        path: "crates/editor-core/tests/all.rs",
        disposition: Shared, // mount guard, literal view
    },
    Entry {
        path: "crates/editor-core/tests/gui1_pick_r2.rs",
        disposition: Shared, // public-surface scan, code view
    },
    Entry {
        path: "crates/editor-core/tests/m10_3_r2_probes_interval.rs",
        disposition: Shared, // unreachable-variant scan, code view
    },
    Entry {
        path: "crates/editor-core/tests/schema_ledger.rs",
        disposition: Shared, // doc-comment ledger, prose view
    },
    Entry {
        path: "crates/geom-brep/tests/all.rs",
        disposition: Shared, // mount guard, literal view
    },
    Entry {
        path: "crates/geom-brep/tests/pcurve_conic.rs",
        disposition: Shared, // wildcard-arm scan, code view
    },
    Entry {
        path: "crates/geom-core/tests/all.rs",
        disposition: Shared, // mount guard, literal view
    },
    Entry {
        path: "crates/geom-core/tests/flagged_census.rs",
        disposition: Shared, // call census, code view + offsets
    },
    Entry {
        path: "crates/geom/tests/all.rs",
        disposition: Shared, // mount guard, literal view
    },
    Entry {
        path: "crates/mesh/tests/all.rs",
        disposition: Shared, // eps inventory, code view
    },
    Entry {
        path: "crates/pncad/tests/all.rs",
        disposition: Unconverted("Track E, issue #763 — `code_without_comments`, line-based"),
    },
    Entry {
        path: "crates/profile/tests/all.rs",
        disposition: Shared, // mount guard, literal view
    },
    Entry {
        path: "crates/profile/tests/seal.rs",
        disposition: Shared, // serde-free seal, code view
    },
    Entry {
        path: "crates/step-export/tests/all.rs",
        disposition: Shared, // mount guard, literal view
    },
    Entry {
        path: "crates/step-import/src/parse.rs",
        disposition: NotRust("STEP Part 21"),
    },
    Entry {
        path: "crates/step-import/tests/all.rs",
        disposition: Shared, // mount guard, literal view
    },
    Entry {
        path: "crates/step-import/tests/tier_gate.rs",
        disposition: Shared, // validator call sites, code view
    },
    Entry {
        path: "crates/stl/tests/all.rs",
        disposition: Shared, // mount guard, literal view
    },
    Entry {
        path: "crates/sweep/src/fillet/admit.rs",
        disposition: Unconverted("Track T — raw `include_str!`, no reader at all"),
    },
    Entry {
        path: "crates/sweep/tests/all.rs",
        disposition: Shared, // mount guard, literal view
    },
    Entry {
        path: "crates/test-utils/src/source.rs",
        disposition: Home,
    },
    Entry {
        path: "crates/test-utils/tests/reader_census.rs",
        disposition: Shared, // this census, literal view
    },
    Entry {
        path: "crates/topo/src/boolean/boxes.rs",
        disposition: Unconverted("Track Q — reads through topo's private `source_walk::CodeOnly`"),
    },
    Entry {
        path: "crates/topo/src/chord_join.rs",
        disposition: Unconverted("Track Q — whitespace-stripped raw text, no reader"),
    },
    Entry {
        path: "crates/topo/src/face_normal.rs",
        disposition: Unconverted("Track Q — raw text, plus topo's private `fixtures::code_only`"),
    },
    Entry {
        path: "crates/topo/src/fixtures.rs",
        disposition: Unconverted("unowned — `code_only`, the second topo blanker"),
    },
    Entry {
        path: "crates/topo/src/review_d18.rs",
        disposition: Unconverted("Track P — raw text and a `\n    }\n` body carve"),
    },
    Entry {
        path: "crates/topo/src/review_d18_probes.rs",
        disposition: Unconverted("Track P — line-leading `//` only"),
    },
    Entry {
        path: "crates/topo/src/sector_shape.rs",
        disposition: Unconverted("Track Q — raw text, no reader at all"),
    },
    Entry {
        path: "crates/topo/src/source_walk.rs",
        disposition: Unconverted("unowned — `CodeOnly`, the other topo blanker"),
    },
    Entry {
        path: "crates/topo/tests/all.rs",
        disposition: Shared, // mount guard, literal view
    },
    Entry {
        path: "crates/viewer/tests/all.rs",
        disposition: Shared, // mount guard, literal view
    },
    Entry {
        path: "tools/tess-meter/tests/derivations.rs",
        disposition: Unconverted("Track K — its own string-continuation lexer"),
    },
];

/// The repository root: this crate's directory, two levels up.
///
/// The "both ways the suite runs" resolution is
/// [`test_utils::source::crate_dir`]'s, shared — three copies of that
/// six-line fallback and its paragraph existed in this tree, which is
/// the same defect one level up from the one this file guards.
fn repo_root() -> PathBuf {
    let root = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        // Canonical, so `..` is not a path COMPONENT: the skip below
        // reads components, and a relative one matched every file in
        // the tree at once — which looked exactly like a clean walk.
        .canonicalize()
        .expect("the repository root resolves");
    assert!(
        root.join("Cargo.toml").is_file(),
        "{} is not the repository root",
        root.display()
    );
    root
}

/// Whether `code` (a comments-blanked view) reads Rust source as text.
///
/// **Three independent shapes, because a single-shaped sweep is what
/// under-reported this class three times** — the count moved at every
/// re-sweep, and never by looking harder in the same place. A site can
/// name a `.rs` file without spelling a comment delimiter; it can walk
/// a source tree without naming any file; and it can lex Rust comments
/// over text it obtained some third way.
///
/// **Shape (1) is arithmetic over the whole file, and that is how a
/// module mount is told from a read.** `#[path = "x.rs"]` names a
/// `.rs` file and reads nothing, so every `tests/all.rs` in the tree
/// would otherwise be a hit — but every mount contributes **exactly
/// one** `.rs"` literal, so a file holding more of those than it holds
/// mounts is naming a source file for some other reason. Two shapes
/// were tried and are wrong, each in its own direction:
///
/// - **slicing the `#[path … ]` attributes out first** is an ad-hoc
///   source slicer inside the file whose whole rule is *do not write
///   one*, and it carries that shape's failure mode — an attribute
///   with no `]` blinds the rest of the file, which is reachable
///   through a string literal spelling `#[path` (`aggregator_headers.rs`
///   holds one);
/// - **requiring the name and the read on ONE LINE** is narrower than
///   either: `rustfmt` puts a long `include_str!` path on its own
///   line, and a named constant (`const TARGET: &str = "src/lib.rs";`
///   … `read_to_string(p.join(TARGET))`) never has them on one line at
///   all. Both are ordinary spellings, and both went undetected.
///
/// The counting needle is written with its quote ESCAPED where it is
/// itself a literal, so this file does not match itself.
fn reads_rust_source(code: &str) -> bool {
    // (1) Names more `.rs` files than it mounts as modules.
    let named = code.matches(".rs\"").count();
    let mounted = code.matches("#[path = \"").count();
    // (2) Walks a source tree.
    let walks_a_source_tree = [
        "rust_sources(",
        "crate_sources(",
        "src_root(",
        "suite_files(",
    ]
    .iter()
    .any(|n| code.contains(n))
        || (code.contains("read_dir(") && code.contains("\"rs\""));
    // (3) Spells a Rust comment delimiter as a literal — the tell of a
    // hand-rolled lexer, wherever its text came from.
    let lexes_rust_comments = ["\"//\"", "\"///\"", "\"//!\"", "\"/*\"", "\"*/\"", "b'/'"]
        .iter()
        .any(|tok| code.contains(tok));
    named > mounted || walks_a_source_tree || lexes_rust_comments
}

/// Every path under the repository root that reads Rust source as text.
fn sites_reading_rust_source(root: &Path) -> Vec<String> {
    rust_sources(root)
        .iter()
        .filter(|path| {
            !path.components().any(|c| {
                let c = c.as_os_str().to_string_lossy();
                SKIPPED_DIRS.contains(&c.as_ref()) || c.starts_with('.')
            }) && path.starts_with(root)
        })
        .filter_map(|path| {
            let text = std::fs::read_to_string(path).expect("a readable source file");
            reads_rust_source(&code_and_literals(&text)).then(|| {
                path.strip_prefix(root)
                    .expect("a walked file lies under the root")
                    .to_string_lossy()
                    .replace('\\', "/")
            })
        })
        .collect()
}

/// **The ledger is the tree's set, not a subset of it.**
///
/// A WALK THAT MATCHED NOTHING IS NOT A PASS, and the equality is what
/// says so: an empty or broken traversal reports all 34 entries as
/// stale and reds, so this row needs no separate count floor (an
/// earlier one asserted `found.len() >= 20`, which set equality had
/// already subsumed and which could not fail for the reason it
/// stated). [`test_utils::source::rust_sources`] panics on an empty
/// directory underneath it as well.
#[test]
fn every_site_that_reads_rust_source_is_in_the_ledger() {
    let root = repo_root();
    let mut found = sites_reading_rust_source(&root);
    found.sort();
    let ledger: Vec<&str> = LEDGER.iter().map(|e| e.path).collect();
    let unlisted: Vec<&String> = found
        .iter()
        .filter(|f| !ledger.contains(&f.as_str()))
        .collect();
    let stale: Vec<&&str> = ledger
        .iter()
        .filter(|l| !found.iter().any(|f| f == *l))
        .collect();
    assert!(
        unlisted.is_empty() && stale.is_empty(),
        "the source-reader ledger no longer matches the tree.\n  \
         arrived or moved (owe a ledger line): {unlisted:#?}\n  \
         listed but no longer reading source (delete the line): {stale:#?}"
    );
}

/// **A `Shared` line is a CLAIM, and this is what checks it.**
///
/// Without this row the ledger's own silent direction is the one it
/// exists to close: a converted site that reverts to a hand-rolled
/// reader keeps its `Shared` line, keeps tripping the detector, and the
/// census stays green over exactly the change it was built to catch.
#[test]
fn every_shared_entry_actually_reaches_the_shared_lexer() {
    let root = repo_root();
    let liars: Vec<&str> = LEDGER
        .iter()
        .filter(|e| matches!(e.disposition, Shared))
        .filter(|e| {
            let text = std::fs::read_to_string(root.join(e.path))
                .unwrap_or_else(|err| panic!("reading {}: {err}", e.path));
            // The code view: a mention in prose is not a call.
            !test_utils::source::code_only(&text).contains("test_utils::source")
        })
        .map(|e| e.path)
        .collect();
    assert!(
        liars.is_empty(),
        "these entries are dispositioned Shared but no longer call \
         `test_utils::source` — either they reverted to a hand-rolled reader, or \
         the line is stale: {liars:#?}"
    );
}

/// **The debt, stated as an equality and hand-synced loudly.**
///
/// Every [`Unconverted`] entry is a site reading Rust source through
/// something other than the shared lexer, with the track that owes its
/// conversion. Converting one means deleting its line AND lowering
/// [`UNCONVERTED_TODAY`] — two edits, on purpose.
///
/// **What equality catches and what it does not.** It catches a reader
/// added without one being converted, which a `<=` ceiling does not: a
/// lane that converts one and adds one nets zero and slides under a
/// ceiling silently. It does NOT catch that same swap here either —
/// nothing a single number can do will — so the swap is caught one
/// row up instead: `every_site_that_reads_rust_source_is_in_the_ledger`
/// forces the new reader to arrive as a NAMED PATH in this file, and
/// `every_shared_entry_actually_reaches_the_shared_lexer` stops it
/// hiding behind a `Shared` line. **The number is the tripwire; the
/// paths are the guard.** The failure below prints the paths for that
/// reason.
#[test]
fn the_unconverted_readers_are_the_ones_this_tree_still_owes() {
    let outstanding: Vec<String> = LEDGER
        .iter()
        .filter_map(|e| match e.disposition {
            Unconverted(owner) => Some(format!("{} — {owner}", e.path)),
            Home | Shared | NotRust(_) => None,
        })
        .collect();
    assert_eq!(
        outstanding.len(),
        UNCONVERTED_TODAY,
        "the ledger holds {} readers outside the shared lexer and \
         UNCONVERTED_TODAY says {UNCONVERTED_TODAY}. That constant is HAND-SYNCED: \
         converting one lowers it, and it may not be raised without the row that \
         licenses a new reader. Outstanding: {outstanding:#?}",
        outstanding.len()
    );
}

/// The number of sites still reading Rust source through something
/// other than [`test_utils::source`]. **Hand-synced with the ledger
/// above, and it goes one way.**
const UNCONVERTED_TODAY: usize = 11;

/// The languages other than Rust that a guard in this tree reads. **A
/// `NotRust` line must name one of these**, because free text is what
/// lets the ledger's one escape hatch be spelled `NotRust("x")` — the
/// hatch has to cost a claim about a real language, and adding one
/// here is that claim.
const OTHER_LANGUAGES: [&str; 1] = ["STEP Part 21"];

/// **The two dispositions that carry a reason must carry a REAL one.**
///
/// `NotRust` and `Unconverted` are the ledger's only ways to leave a
/// site unconverted, so each is checked for structure and not merely
/// for non-emptiness: the language must be one this tree actually
/// holds, and the owner must name a track or say `unowned` — which is
/// itself a second finding, not an exemption.
#[test]
fn every_disposition_that_carries_a_reason_states_a_real_one() {
    for entry in LEDGER {
        match entry.disposition {
            NotRust(language) => assert!(
                OTHER_LANGUAGES.contains(&language),
                "{} is dispositioned NotRust({language:?}), which is not a language this \
                 tree reads. Add it to OTHER_LANGUAGES if it is one; the alternative is \
                 that the file is a Rust reader wearing an escape hatch.",
                entry.path
            ),
            Unconverted(owner) => assert!(
                owner.starts_with("Track ") || owner.starts_with("unowned"),
                "{} is dispositioned Unconverted({owner:?}), which names no owner. Every \
                 one is a defect that belongs to a track, or is `unowned` — and unowned \
                 is a second finding on top of the first, not a way out of it.",
                entry.path
            ),
            Home | Shared => {}
        }
    }
}
