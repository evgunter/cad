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
//! Rust source as text reaches the shared lexer, or is listed here as
//! an exception.** The next hand-rolled reader cannot arrive silently,
//! because a site that reads source and reaches no shared door is what
//! this row detects. A site that does reach one needs no line: that is
//! read off its own code.
//!
//! # What a red here means
//!
//! A site reads Rust source as text and reaches no door of
//! [`test_utils::source`]. Two honest answers:
//!
//! - it reads a language that is not Rust (a STEP file, a manifest, a
//!   `.cad` document) — add the exception, say which;
//! - it is a new hand-rolled Rust reader — do not add a line. Use the
//!   shared lexer. That is the whole point of the row.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::Path;

use test_utils::source::{balanced_end, code_and_literals, code_only, repo_root, rust_sources};

/// **The aggregation row macro's invocation, spelled ONCE.**
///
/// Two needles in this file are built from it — shape (2) of
/// [`reads_rust_source`] and [`SHARED_LEXER_DOORS`] — and a `macro_rules!`
/// is what makes them one spelling rather than two hand-kept ones, since
/// `concat!` takes literals and a `const` is not one. Collapsing fifteen
/// copies of a row onto one macro and then hand-keeping two copies of its
/// name is the class that collapse closes, one level down.
///
/// **No delimiter.** `every_suite_file_is_aggregated! { }` and `! [ ]`
/// are the same invocation to `rustc` as `! ( )`, so a needle ending in
/// `(` would stop seeing a file whose row was re-delimited. The
/// trailing `!` is what keeps a prose mention of the row's `fn` NAME
/// (`crates/bvh/tests/aggregator_headers.rs` holds one, inside an
/// `assert!` message) from answering as an invocation.
///
/// [`the_aggregation_row_needle_names_a_macro_that_exists`] is what
/// holds this to the macro's real name.
macro_rules! aggregation_row_macro {
    () => {
        "every_suite_file_is_aggregated!"
    };
}

/// [`aggregation_row_macro`]'s text, for the rows that reason about it.
const AGGREGATION_ROW_MACRO: &str = aggregation_row_macro!();

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
    /// The home itself.
    Home,
    /// Reads Rust source through something other than
    /// [`test_utils::source`] — a hand-rolled reader, or raw text with
    /// no reader at all. The payload names the
    /// track that owes the conversion, **or `unowned` where the
    /// partition has no track for the file**; an unowned entry is not
    /// an exemption, it is a second finding stacked on the first, and
    /// it says so in its own text.
    Unconverted(&'static str),
    /// Reads a language that is not Rust, so the Rust lexer is not what
    /// it wants. The named language is the claim.
    NotRust(&'static str),
}

use Disposition::{Home, NotRust, Unconverted};

/// **The exceptions.** One line per site that reads Rust source as text
/// and does not reach the shared lexer: the home itself, a reader of
/// another language, and the readers still owed a conversion.
///
/// A site that reaches the shared lexer needs no line — that is read
/// off its own code ([`reaches_the_shared_lexer`]). Sorted by path.
/// Adding a line is a deliberate act and the enum above says which
/// acts are honest. A line whose file was converted or deleted is not a
/// red: it excuses nothing, and it goes stale silently.
const LEDGER: &[Entry] = &[
    Entry {
        path: "crates/step-import/src/parse.rs",
        disposition: NotRust("STEP Part 21"),
    },
    Entry {
        path: "crates/test-utils/src/source.rs",
        disposition: Home,
    },
    Entry {
        path: "crates/topo/src/chord_join.rs",
        disposition: Unconverted("Track Q — whitespace-stripped raw text, no reader"),
    },
    Entry {
        path: "crates/topo/src/face_normal.rs",
        disposition: Unconverted("Track Q — raw text"),
    },
    Entry {
        path: "crates/topo/src/pcurves/lift_agreement_rows.rs",
        disposition: Unconverted(
            "unowned by a code-quality track (the file is CHARTTAIL's) — the maker \
             census counts `JointElement(` over raw lines with `//` skipped by prefix, \
             beside two reads that do go through the shared lexer \
             (work/charttail/the-ring-promotion-posture-row-reads-euler-kill-rs-by-hand.md)",
        ),
    },
    Entry {
        path: "crates/topo/src/pcurves/staleness_posture.rs",
        disposition: Unconverted(
            "unowned by a code-quality track (the file is CHARTTAIL's) — the \
             ring-promotion posture row reads `euler_kill.rs` as raw lines and skips \
             `//` by prefix, no reader \
             (work/charttail/the-ring-promotion-posture-row-reads-euler-kill-rs-by-hand.md)",
        ),
    },
    Entry {
        path: "crates/topo/src/sector_shape.rs",
        disposition: Unconverted("Track Q — raw text, no reader at all"),
    },
];

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
/// mounts is naming a source file for some other reason. An
/// aggregator's own margin is now zero: the `include_str!("all.rs")`
/// that used to carry it lives in
/// `test_utils::every_suite_file_is_aggregated!`'s expansion, and shape
/// (2) below is what sees those files. Shape (1) still decides every
/// other named-path site, and still keeps a mount from reading as a
/// read. Two shapes
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
/// **A GATED-SUITE MARKER IS THE SECOND WAY TO NAME A SOURCE FILE
/// WITHOUT READING ONE**, and it is subtracted for the same reason a
/// mount is. `test_utils::gated_to!["crates/x/src/y.rs", …]` declares
/// the paths a suite is specific to so the change filter can skip it;
/// the suite never opens them, and `scripts/ci-filter.py` is what
/// reads that text. Unsubtracted, every marked suite in the tree reads
/// as a source reader and owes a ledger line it cannot honestly carry.
///
/// A marker's paths are NOT one-per-invocation the way a mount's is —
/// `rustfmt` collapses a short list onto one line — so this one is
/// counted by scanning each invocation's own brackets rather than by a
/// second whole-file needle. That is a BOUNDED slice, not the ad-hoc
/// source slicer rejected above, and its failure direction is the safe
/// one: an invocation whose brackets do not close subtracts NOTHING, so
/// the file stays a hit and someone has to look at it.
///
/// The counting needle is written with its quote ESCAPED where it is
/// itself a literal, so this file does not match itself.
/// `.rs"` literals inside `gated_to!` INVOCATIONS — the paths a gated suite
/// DECLARES rather than reads.
///
/// **The strict spelling, and over-subtraction is why.** This number is
/// SUBTRACTED from the count of named `.rs` files, so anything it over-counts
/// hides a real source reader from the ledger — the silent direction. So an
/// invocation is `gated_to!` with its bracket IMMEDIATELY after (whitespace
/// and nothing else between), read over the CODE VIEW with comments stripped,
/// which is exactly what `scripts/ci-filter.py` requires before it treats a
/// file as marked. A prose mention followed later in the file by an unrelated
/// bracketed `.rs"` literal is not an invocation and subtracts nothing.
///
/// Zero when an invocation's brackets do not close, which leaves the file
/// counted as a reader — the safe direction again.
fn gated_to_names(code: &str) -> usize {
    let code = strip_line_comments(code);
    let mut total = 0;
    let mut rest = code.as_str();
    while let Some(at) = rest.find("gated_to!") {
        rest = &rest[at + "gated_to!".len()..];
        // IMMEDIATELY after, modulo whitespace. Skipping ahead to the next
        // bracket ANYWHERE in the file is how a mention of the macro starts
        // subtracting some unrelated reader's literals.
        let after = rest.trim_start();
        let Some(opener) = after.chars().next().filter(|c| "[({".contains(*c)) else {
            continue;
        };
        let open = rest.len() - after.len();
        let closer = match opener {
            '[' => ']',
            '(' => ')',
            _ => '}',
        };
        let mut depth = 0usize;
        let mut end = None;
        for (i, c) in rest[open..].char_indices() {
            if c == opener {
                depth += 1;
            } else if c == closer {
                depth -= 1;
                if depth == 0 {
                    end = Some(open + i);
                    break;
                }
            }
        }
        let Some(end) = end else { break };
        total += rest[open..end].matches(".rs\"").count();
        rest = &rest[end..];
    }
    total
}

/// `code` with its comments blanked by the shared lexer — a marker is an
/// ITEM and never a comment, and without the cut the macro's own
/// documentation (which quotes a call) subtracts paths nothing declares.
/// Literals are kept: the marker's paths are literals.
fn strip_line_comments(code: &str) -> String {
    code_and_literals(code)
}

/// **The subtraction's own guard.** `gated_to_names` is the one number in this
/// file that makes the detector LESS sensitive, so its near misses are what
/// need planting: only a real invocation may subtract, and a mention must not.
///
/// EVERY CASE IS ASSEMBLED, never spelled — `call` joins the macro name to its
/// bracket at run time, so this file's own SOURCE carries no invocation and
/// neither this census nor `scripts/gates/gated-suite-paths.sh` reads a
/// fixture as a marker. Same self-non-matching trick as `reads_rust_source`'s
/// escaped needle below, for the same reason.
#[test]
fn only_a_real_marker_invocation_subtracts_its_paths() {
    fn call(tail: &str) -> String {
        format!("{}{}", "gated_to", tail)
    }
    let cases: [(&str, usize, String); 7] = [
        (
            "a one-line invocation",
            2,
            call(r#"!["a/b.rs", "c/d.rs"];"#),
        ),
        (
            "a wrapped invocation",
            2,
            call("![\n    \"a/b.rs\",\n    \"c/d.rs\",\n];"),
        ),
        (
            "a directory entry is not a .rs name",
            0,
            call(r#"!["a/b/"];"#),
        ),
        (
            "a mention in a doc comment",
            0,
            format!("/// `test_utils::{}` declares.", call(r#"!["a/b.rs"]"#)),
        ),
        (
            "the macro NAMED in a string, with an unrelated bracket after it",
            0,
            format!("let n = \"{}!\";\nlet v = [\"src/lib.rs\"];", "gated_to"),
        ),
        (
            "the macro's own definition",
            0,
            format!(
                "macro_rules! {} {{ ($($p:literal),+ $(,)?) => {{}}; }}",
                "gated_to"
            ),
        ),
        ("brackets that never close", 0, call(r#"!["a/b.rs","#)),
    ];
    for (what, want, code) in cases {
        assert_eq!(gated_to_names(&code), want, "{what}: {code:?}");
    }
}

fn reads_rust_source(code: &str) -> bool {
    // (1) Names more `.rs` files than it mounts as modules or declares
    //     in a gated-suite marker.
    let named = code.matches(".rs\"").count();
    let mounted = code.matches("#[path = \"").count() + gated_to_names(code);
    // (2) Walks a source tree — directly, or by invoking a macro whose
    //     expansion does. The aggregation row walks the invoking crate's
    //     `tests/` and reads every suite file in it; the tokens land in
    //     the invoking file, so the walk is that file's, and the needle
    //     is the INVOCATION because the expansion is not in the text
    //     this reads. `aggregation_row_macro!` is the one spelling.
    let walks_a_source_tree = [
        "rust_sources(",
        "crate_sources(",
        "src_root(",
        "suite_files(",
        aggregation_row_macro!(),
    ]
    .iter()
    .any(|n| code.contains(n))
        || (code.contains("read_dir(") && code.contains("\"rs\""));
    // (3) Spells a Rust comment delimiter as a literal — the tell of a
    // hand-rolled lexer, wherever its text came from.
    let lexes_rust_comments = COMMENT_DELIMITERS.iter().any(|tok| code.contains(tok));
    named > mounted || walks_a_source_tree || lexes_rust_comments
}

/// Shape (3)'s needles: a Rust comment delimiter spelled as a literal.
const COMMENT_DELIMITERS: [&str; 6] = ["\"//\"", "\"///\"", "\"//!\"", "\"/*\"", "\"*/\"", "b'/'"];

/// Every path under the repository root that reads Rust source as text.
fn sites_reading_rust_source(root: &Path) -> Vec<String> {
    rust_sources(root)
        .iter()
        .filter(|path| {
            // Skip on the path RELATIVE TO ROOT, never on the absolute
            // one. The components above the repository are not the
            // repository's business, and one of them being hidden is
            // not a fact about any file here: a checkout under a
            // hidden directory — `~/.mngr/worktrees/<name>` is the
            // one every agent lane in this project works in — matched
            // `.mngr` on EVERY path and filtered the whole tree away.
            // The row survived that as a red rather than a false
            // green, which is the doc comment below working exactly
            // as it claims; what it cost was the ability to run this
            // test anywhere but CI.
            path.strip_prefix(root).is_ok_and(|relative| {
                !relative.components().any(|c| {
                    let c = c.as_os_str().to_string_lossy();
                    SKIPPED_DIRS.contains(&c.as_ref()) || c.starts_with('.')
                })
            })
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

/// **Every site that reads Rust source reaches the shared lexer, or is
/// an exception.**
///
/// A WALK THAT MATCHED NOTHING IS NOT A PASS: the walk must still find
/// the [`Home`], the one reader no conversion moves, or it has gone
/// blind and every site it cannot see passes unread.
#[test]
fn every_site_that_reads_rust_source_is_in_the_ledger() {
    let root = repo_root(env!("CARGO_MANIFEST_DIR"));
    let mut found = sites_reading_rust_source(&root);
    found.sort();
    for e in LEDGER.iter().filter(|e| matches!(e.disposition, Home)) {
        assert!(
            found.iter().any(|f| f == e.path),
            "the walk no longer finds {}, the shared lexer's home: it has gone blind. \
             Found: {found:#?}",
            e.path
        );
    }
    let unlisted: Vec<&String> = found
        .iter()
        .filter(|f| !LEDGER.iter().any(|e| e.path == f.as_str()))
        .filter(|f| {
            let text = std::fs::read_to_string(root.join(f.as_str()))
                .unwrap_or_else(|err| panic!("reading {f}: {err}"));
            !reaches_the_shared_lexer(&text)
        })
        .collect();
    let doors = shared_lexer_fns();
    let unlexed: Vec<String> = found
        .iter()
        .filter(|f| !LEDGER.iter().any(|e| e.path == f.as_str()))
        .flat_map(|f| {
            let text = std::fs::read_to_string(root.join(f.as_str()))
                .unwrap_or_else(|err| panic!("reading {f}: {err}"));
            fns_spelling_a_delimiter_unlexed(&text, &doors)
                .into_iter()
                .map(move |name| format!("{f}: fn {name}"))
        })
        .collect();
    assert!(
        unlexed.is_empty(),
        "these fns spell a Rust comment delimiter as a literal and call no fn of the \
         shared lexer: a hand-rolled reader beside a file's shared reads. Read through \
         `test_utils::source`: {unlexed:#?}"
    );
    assert!(
        unlisted.is_empty(),
        "these sites read Rust source as text and reach no door of the shared lexer \
         ({SHARED_LEXER_DOORS:?}). Read through `test_utils::source`; a site reading a \
         language that is not Rust takes a `NotRust` line in LEDGER: {unlisted:#?}"
    );
}

/// The ways a site reaches the shared lexer, as they are SPELLED.
///
/// Two, because one of them hides the other: an aggregating
/// `tests/all.rs` reaches `test_utils::source::aggregation_violations`
/// through the aggregation row macro, and the module path it reaches is
/// in the macro's EXPANSION rather than in the file. One body answers
/// for every invocation, so
/// [`the_aggregation_row_macro_reaches_the_shared_lexer`] is what checks
/// that the second entry still reaches the lexer.
const SHARED_LEXER_DOORS: [&str; 2] = [
    "test_utils::source",
    concat!("test_utils::", aggregation_row_macro!()),
];

/// Whether a file's code (not its prose: a mention is not a call)
/// names a door of the shared lexer.
///
/// A file can reach the lexer for one read and hand-roll another; the
/// per-`fn` check ([`fns_spelling_a_delimiter_unlexed`]) catches the
/// hand-rolled one when it spells a comment delimiter. One that reads
/// raw text with no delimiter, in a file that reaches the lexer
/// elsewhere, is not seen.
fn reaches_the_shared_lexer(text: &str) -> bool {
    let code = code_only(text);
    SHARED_LEXER_DOORS.iter().any(|door| code.contains(door))
}

/// The `pub fn`s of the shared lexer, read off its home: the names a
/// call to it is spelled with once imported.
fn shared_lexer_fns() -> Vec<String> {
    let home = repo_root(env!("CARGO_MANIFEST_DIR")).join("crates/test-utils/src/source.rs");
    let code = code_only(&std::fs::read_to_string(home).expect("the lexer's home reads"));
    let mut out: Vec<String> = code
        .match_indices("pub fn ")
        .map(|(at, _)| test_utils::source::ident(&code, at + "pub fn ".len()).to_string())
        .filter(|name| !name.is_empty())
        .collect();
    out.sort();
    out.dedup();
    assert!(
        out.iter().any(|n| n == "code_only"),
        "the lexer's home names no `code_only`: this read of it has gone blind"
    );
    out
}

/// The innermost `fn` items of `text` that spell a comment delimiter as a
/// literal and whose body calls no fn of the shared lexer (by path or by
/// an imported name).
fn fns_spelling_a_delimiter_unlexed(text: &str, doors: &[String]) -> Vec<String> {
    let bare = code_only(text);
    let lexed = code_and_literals(text);
    let Ok(items) = test_utils::source::fn_items(&bare) else {
        return Vec::new();
    };
    let calls_a_door = |body: &str| {
        body.contains("test_utils::source::")
            || doors.iter().any(|door| {
                body.match_indices(&format!("{door}(")).any(|(at, _)| {
                    body[..at]
                        .chars()
                        .next_back()
                        .is_none_or(|c| !(c.is_alphanumeric() || c == '_'))
                })
            })
    };
    let mut out = Vec::new();
    for tok in COMMENT_DELIMITERS {
        for (at, _) in lexed.match_indices(tok) {
            let Some((name, body)) = items
                .iter()
                .filter(|(_, body)| body.contains(&at))
                .min_by_key(|(_, body)| body.len())
            else {
                continue;
            };
            if !calls_a_door(&bare[body.clone()]) && !out.contains(name) {
                out.push(name.clone());
            }
        }
    }
    out
}

/// The file holding the aggregation row macro's body — the subject of
/// the two rows below.
fn aggregation_row_macro_home() -> std::path::PathBuf {
    repo_root(env!("CARGO_MANIFEST_DIR")).join("crates/test-utils/src/source.rs")
}

/// The macro's bare name, from the one spelling of its invocation.
fn aggregation_row_macro_name() -> &'static str {
    AGGREGATION_ROW_MACRO
        .strip_suffix('!')
        .expect("the needle is an invocation, so it ends in `!`")
}

/// **The needle names a macro that EXISTS, under that name.**
///
/// [`AGGREGATION_ROW_MACRO`] is a string. Renaming the macro re-spells
/// fifteen invocations for free — the compiler does it — and leaves the
/// string untouched, at which point shape (2) of [`reads_rust_source`]
/// matches nothing and every aggregating `tests/all.rs` stops reading as
/// a source reader, silently. This row is the edge a rename trips on
/// instead, and it names the cause.
#[test]
fn the_aggregation_row_needle_names_a_macro_that_exists() {
    let name = aggregation_row_macro_name();
    let text = std::fs::read_to_string(aggregation_row_macro_home()).expect("a readable source");
    let declaration = format!("macro_rules! {name} {{");
    assert!(
        code_only(&text).contains(&declaration),
        "`{declaration}` is not declared in crates/test-utils/src/source.rs. The needle \
         `{AGGREGATION_ROW_MACRO}` names no macro, so every aggregating tests/all.rs has \
         stopped reading as a source reader for a reason that is in THIS file, not in theirs."
    );
}

/// **The one check where there were fifteen.**
///
/// [`SHARED_LEXER_DOORS`]' second entry says that invoking the
/// aggregation row macro reaches the shared lexer. That is a claim about
/// a body in `crates/test-utils/src/source.rs`, not in the invoking
/// file — so rewriting the macro to walk `tests/` by hand leaves every
/// invoking file reading as one that reaches the lexer, and every other
/// row in this file passing. That reversion reds this row and nothing
/// else.
#[test]
fn the_aggregation_row_macro_reaches_the_shared_lexer() {
    let name = aggregation_row_macro_name();
    let text = std::fs::read_to_string(aggregation_row_macro_home()).expect("a readable source");
    let code = code_only(&text);
    let at = code
        .find(&format!("macro_rules! {name} {{"))
        .expect("the row above proves the declaration is here");
    let open = at + code[at..].find('{').expect("the declaration opens a brace");
    let end = balanced_end(&code, open).expect("the macro's braces close");
    let body = &code[open..=end];
    for door in ["source::crate_dir(", "source::aggregation_violations("] {
        assert!(
            body.contains(door),
            "the `{name}!` expansion no longer calls `{door}` — it has stopped reaching the \
             shared lexer, and no other row in this file can see that, because the fifteen \
             call sites carry the invocation and this one body carries the reader."
        );
    }
}

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
            Home => {}
        }
    }
}

// ---------------------------------------------------------------------
// The second half: scope questions.
// ---------------------------------------------------------------------

/// **A site that asks what a NAME MEANS at a place in Rust source, and
/// what answers it.**
///
/// Lexing is one question and scope is another. [`test_utils::source`]
/// answers the first — which bytes are code, which are prose, which are
/// literals — and it is the right home for it because a lexer needs no
/// dependency. *Which binding a name reaches* (a `let` earlier in the
/// block, a closure or match-arm pattern, an fn parameter, a `use`, an
/// item, a `use super::*` in an inline module) is a question about the
/// syntax tree, and every hand-rolled answer to it in this tree has been
/// a blind spot waiting for its next shape: the flagged-door census
/// minted one per review (`if let`, `for`, a tuple `let`, a parameter
/// shadowing an import) before it moved onto a parse.
///
/// **`crates/source-scope` is the one home for scope questions.** It
/// parses with `syn` and resolves every bare name through a scope stack;
/// it is a dev-dependency only, so `test-utils` stays zero-dependency
/// and the lexer stays where it is.
///
/// A site is found by the KEYWORDS A SCOPE READER HAS TO SPELL: to find
/// a binding by text, a reader looks for `let`, `use` or `super` as a
/// literal, and a site that resolves through the home names it. The
/// needle is loud on purpose — a literal `"use"` in a panic message is a
/// hit and owes a [`NotScope`](ScopeDisposition::NotScope) line saying
/// so — because a quiet needle is how the first half of this file
/// under-reported its class four times.
///
/// **What the needle cannot see**, and why the ledger takes a second
/// kind of line ([`Seen::Anchor`]):
/// - a resolver by SPELLING, which decides what a name means by
///   comparing its text with the paths it may be written as (`"Bounds"`,
///   `"crate::Bounds"`) and spells no binding keyword;
/// - a reader that compares identifiers it collected some other way (a
///   pattern's names read off brackets, an `impl` head's type), with no
///   keyword literal;
/// - a reader in another language (the Python gates, whose
///   blind spots each gate's header lists).
///
/// A site of those shapes found by review is listed with an anchor, the
/// item that does the reading, and held to it.
struct ScopeEntry {
    /// Path relative to the repository root, `/`-separated.
    path: &'static str,
    /// How the entry is found.
    seen: Seen,
    disposition: ScopeDisposition,
    /// The `fn` items of the file this line answers for. For a
    /// [`HandRolled`] or a [`NotScope`] line, every needle hit in the file
    /// lies in one of its lines' items, and each item holds one: a reader
    /// added to a file beside a listed one is a hit no item answers for.
    items: &'static [&'static str],
}

/// How a scope-ledger entry is held to the tree.
enum Seen {
    /// By [`asks_a_scope_question`]: the walk finds the file, and the
    /// ledger is that set.
    Needle,
    /// By review, for a shape the needle cannot see: the file's code
    /// still holds the line's items (the walk must NOT find the file, or
    /// the line belongs under [`Seen::Needle`]).
    Anchor,
}

enum ScopeDisposition {
    /// `crates/source-scope` itself.
    ScopeHome,
    /// Spells a binding keyword for a reason that is not a scope
    /// question: one of a closed set of reasons, each confined to the
    /// files it was written for and to the needles they hold. The text
    /// says where.
    NotScope(NotScopeWhy, &'static str),
    /// A hand-rolled scope reader that predates the home. The payload
    /// names the work row that owes its conversion, and each of the
    /// line's items must be in [`HAND_ROLLED_AT_THE_HOMES_ARRIVAL`]. **A
    /// new hand-rolled scope reader does not get this line**: use
    /// `source_scope`.
    HandRolled(&'static str),
}

/// **Why a site spells a binding keyword without asking a scope
/// question.** Closed: a new reason, or a new file for one, is an edit
/// here reviewed as one, not free text a hand reader can wear.
#[derive(Clone, Copy, Debug)]
enum NotScopeWhy {
    /// The keyword is the text of a message (an `expect`).
    Message,
    /// A `pub(super)` visibility, read as a word.
    Visibility,
    /// The shared lexer's keyword tables, and its call census's skip of
    /// the `let` that declares a name: where an item head or a binding
    /// starts, resolving nothing.
    LexerTables,
    /// This census's own needles.
    OwnNeedles,
    /// A `use` declaration located to be skipped or blanked: a
    /// declaration reaches nothing, and nothing it binds is resolved.
    SkipsUseDeclarations,
    /// A call census's skip of the `let` that declares the name it
    /// counts (`let f = …` is no call of `f`), resolving nothing.
    SkipsADeclaringLet,
}

impl NotScopeWhy {
    /// The needles this reason answers for. Which files and items may
    /// claim it is [`NOT_SCOPE_AT_THE_HOMES_ARRIVAL`]'s to say.
    fn admits(self) -> &'static [&'static str] {
        match self {
            NotScopeWhy::Message => &["\"use\""],
            NotScopeWhy::Visibility => &["\"super\""],
            NotScopeWhy::LexerTables => &["\"let\"", "\"use\""],
            NotScopeWhy::OwnNeedles => &SCOPE_KEYWORD_LITERALS,
            NotScopeWhy::SkipsUseDeclarations => &["\"use\"", "\"use \""],
            NotScopeWhy::SkipsADeclaringLet => &["\"let\""],
        }
    }
}

use ScopeDisposition::{HandRolled, NotScope, ScopeHome};

/// The work row every pre-existing hand-rolled scope reader is owed by.
const SCOPE_CONVERSION_ROW: &str = "work/onecopy/hand-rolled-scope-readers-predate-source-scope.md";

/// **The hand-rolled scope readers as they stood when the home landed,
/// by item (`path::fn`): a record that only shrinks.** Every
/// [`HandRolled`] line's items are in it, and every entry in it is a
/// [`HandRolled`] item that still reads: a converted reader leaves its
/// line and its entry together
/// ([`the_hand_rolled_scope_readers_are_the_ones_this_tree_still_owes`]).
/// `scripts/gates/scope-readers-only-shrink.sh` refuses an entry the
/// merge base does not hold, which is what keeps a new reader out of it.
const HAND_ROLLED_AT_THE_HOMES_ARRIVAL: [&str; 6] = [
    "crates/geom-core/tests/bounds_census.rs::is_sole_bracket",
    "crates/pncad-py/src/prose_census.rs::normalize",
    "crates/pncad-py/src/prose_census.rs::use_bindings",
    "crates/pncad-py/src/tests.rs::parse_tag_body",
    "crates/pncad/tests/all.rs::use_statement_roots",
    "crates/test-utils/tests/hand_written_impl_census.rs::self_aliases",
];

/// **The items that spell a binding keyword for a reason that is not a
/// scope question, as they stood when the home landed: a record that
/// only shrinks**, keyed `Reason path::item`. Every [`NotScope`] line's
/// items are in it under the line's reason, and every entry is such an
/// item that still spells one. The shrink gate holds it against the
/// merge base with [`HAND_ROLLED_AT_THE_HOMES_ARRIVAL`], so a new reason,
/// a new file for one, or a new item, is an edit the base refuses.
const NOT_SCOPE_AT_THE_HOMES_ARRIVAL: [&str; 11] = [
    "LexerTables crates/test-utils/src/source.rs::DECLARERS",
    "LexerTables crates/test-utils/src/source.rs::ITEM_KEYWORDS",
    "LexerTables crates/test-utils/src/source.rs::funnel_sites",
    "Message crates/step-import/tests/inst_review_probes.rs::solid_clouds",
    "OwnNeedles crates/test-utils/tests/reader_census.rs::the_scope_needle_finds_a_binding_keyword_spelled_to_be_searched_for",
    "SkipsADeclaringLet crates/pncad/tests/all.rs::first_arg_literals",
    "SkipsUseDeclarations crates/pncad-py/src/tests.rs::matches",
    "SkipsUseDeclarations crates/pncad-py/src/tests.rs::top_form",
    "SkipsUseDeclarations crates/topo/src/boolean/offer_rows.rs::every_site_names_the_decision_it_raises",
    "SkipsUseDeclarations crates/topo/src/source_walk.rs::uses_blanked",
    "Visibility crates/sweep/tests/review_fillet_split_r2_probes.rs::the_seam_and_the_open_bands_export_one_public_item_and_widen_nothing_else",
];

/// **The scope ledger.** One line per reason a site [`asks_a_scope_question`]
/// without calling the home's reader (a file may hold more than one), and
/// one per site of a shape it cannot see ([`Seen::Anchor`]). A site that
/// calls the home ([`calls_the_scope_home`]) needs no line. Sorted by
/// path.
const SCOPE_LEDGER: &[ScopeEntry] = &[
    ScopeEntry {
        path: "crates/geom-core/tests/bounds_census.rs",
        seen: Seen::Anchor,
        disposition: HandRolled(SCOPE_CONVERSION_ROW), // a bound's path matched by spelling
        items: &["is_sole_bracket"],
    },
    ScopeEntry {
        path: "crates/pncad-py/src/prose_census.rs",
        seen: Seen::Needle,
        disposition: HandRolled(SCOPE_CONVERSION_ROW),
        items: &["normalize", "use_bindings"],
    },
    ScopeEntry {
        path: "crates/pncad-py/src/tests.rs",
        seen: Seen::Needle,
        disposition: HandRolled(SCOPE_CONVERSION_ROW), // a fn's `use … as R;` shorthands
        items: &["parse_tag_body"],
    },
    ScopeEntry {
        path: "crates/pncad-py/src/tests.rs",
        seen: Seen::Needle,
        disposition: NotScope(
            NotScopeWhy::SkipsUseDeclarations,
            "the tag table's top-level form classifier",
        ),
        items: &["matches", "top_form"],
    },
    ScopeEntry {
        path: "crates/pncad/tests/all.rs",
        seen: Seen::Needle,
        disposition: HandRolled(SCOPE_CONVERSION_ROW),
        items: &["use_statement_roots"],
    },
    ScopeEntry {
        path: "crates/pncad/tests/all.rs",
        seen: Seen::Needle,
        disposition: NotScope(
            NotScopeWhy::SkipsADeclaringLet,
            "the struct-literal census skips a `let` declaring the name",
        ),
        items: &["first_arg_literals"],
    },
    ScopeEntry {
        path: "crates/source-scope/src/lib.rs",
        seen: Seen::Needle,
        disposition: ScopeHome,
        items: &[],
    },
    ScopeEntry {
        path: "crates/step-import/tests/inst_review_probes.rs",
        seen: Seen::Needle,
        disposition: NotScope(NotScopeWhy::Message, "an `.expect(\"use\")`"),
        items: &["solid_clouds"],
    },
    ScopeEntry {
        path: "crates/sweep/tests/review_fillet_split_r2_probes.rs",
        seen: Seen::Needle,
        disposition: NotScope(NotScopeWhy::Visibility, "the open bands' `pub(super)`"),
        items: &["the_seam_and_the_open_bands_export_one_public_item_and_widen_nothing_else"],
    },
    ScopeEntry {
        path: "crates/test-utils/src/source.rs",
        seen: Seen::Needle,
        disposition: NotScope(
            NotScopeWhy::LexerTables,
            "the item-head keyword tables, and `funnel_sites`' skip of a `let` declaring the name",
        ),
        items: &["DECLARERS", "ITEM_KEYWORDS", "funnel_sites"],
    },
    ScopeEntry {
        path: "crates/test-utils/tests/hand_written_impl_census.rs",
        seen: Seen::Needle,
        disposition: HandRolled(SCOPE_CONVERSION_ROW), // one `let` deep
        items: &["self_aliases"],
    },
    ScopeEntry {
        path: "crates/test-utils/tests/reader_census.rs",
        seen: Seen::Needle,
        disposition: NotScope(NotScopeWhy::OwnNeedles, "SCOPE_KEYWORD_LITERALS"),
        items: &["the_scope_needle_finds_a_binding_keyword_spelled_to_be_searched_for"],
    },
    ScopeEntry {
        path: "crates/topo/src/boolean/offer_rows.rs",
        seen: Seen::Needle,
        disposition: NotScope(
            NotScopeWhy::SkipsUseDeclarations,
            "a `use` line is no passing site",
        ),
        items: &["every_site_names_the_decision_it_raises"],
    },
    ScopeEntry {
        path: "crates/topo/src/source_walk.rs",
        seen: Seen::Needle,
        disposition: NotScope(NotScopeWhy::SkipsUseDeclarations, "blanks `use` items"),
        items: &["uses_blanked"],
    },
];

/// The needles [`asks_a_scope_question`] looks for, each a binding
/// keyword spelled as a literal, quote included.
const SCOPE_KEYWORD_LITERALS: [&str; 6] = [
    "\"let\"",
    "\"let \"",
    "\"use\"",
    "\"use \"",
    "\"super\"",
    "\"super::\"",
];

/// The spelling a site that resolves through the home carries.
const SCOPE_HOME_DOOR: &str = "source_scope";

/// The home's reader: a site that calls it answers its scope questions
/// through the home.
const SCOPE_HOME_READ: &str = "read";

/// Whether `code` (a comments-blanked, literals-kept view) asks a scope
/// question: it spells a binding keyword as a literal, or it names the
/// home.
fn asks_a_scope_question(code: &str) -> bool {
    SCOPE_KEYWORD_LITERALS.iter().any(|n| code.contains(n)) || code.contains(SCOPE_HOME_DOOR)
}

/// Each keyword needle `code` holds, with its offset.
fn scope_needle_hits(code: &str) -> Vec<(usize, &'static str)> {
    let mut out: Vec<(usize, &'static str)> = SCOPE_KEYWORD_LITERALS
        .iter()
        .flat_map(|n| code.match_indices(n).map(move |(at, _)| (at, *n)))
        .collect();
    out.sort_unstable();
    out
}

/// Every `fn` item of `code` (a code-only view) with its body's range,
/// and every `const` and `static` item with its whole extent: the items
/// a scope-ledger line names.
fn fn_bodies(code: &str) -> Vec<(String, std::ops::Range<usize>)> {
    let mut out = test_utils::source::fn_items(code)
        .unwrap_or_else(|why| panic!("the walk cannot place a function: {why}"));
    for keyword in ["const ", "static "] {
        for (at, _) in code.match_indices(keyword) {
            if !test_utils::source::boundary_before(code, at) {
                continue;
            }
            let name_at = test_utils::source::skip_ws(code, at + keyword.len());
            let name = test_utils::source::ident(code, name_at);
            if name.is_empty() || name == "fn" || name == "mut" {
                continue;
            }
            // A `const`/`static` runs to its `;`, through any brace in
            // its initializer.
            if let Some(end) = code[at..].find(";\n") {
                out.push((name.to_string(), at..at + end + 1));
            }
        }
    }
    out
}

/// The innermost item whose extent holds offset `at`.
fn innermost_fn(bodies: &[(String, std::ops::Range<usize>)], at: usize) -> Option<&str> {
    bodies
        .iter()
        .filter(|(_, r)| r.contains(&at))
        .min_by_key(|(_, r)| r.len())
        .map(|(n, _)| n.as_str())
}

/// Whether `code` (a code-only view) calls the home's reader: by its
/// path (`source_scope::read(`), or by the name a `use source_scope::…`
/// import binds (`read(`). A mention of the crate is not a call.
fn calls_the_scope_home(code: &str) -> bool {
    let by_path = format!("{SCOPE_HOME_DOOR}::{SCOPE_HOME_READ}(");
    if code.contains(&by_path) {
        return true;
    }
    let call = format!("{SCOPE_HOME_READ}(");
    let imported = code
        .match_indices(&format!("use {SCOPE_HOME_DOOR}::"))
        .any(|(at, _)| {
            let item = &code[at..code[at..].find(';').map_or(code.len(), |e| at + e)];
            item.split(|c: char| !(c.is_alphanumeric() || c == '_'))
                .any(|w| w == SCOPE_HOME_READ)
        });
    imported
        && code.match_indices(&call).any(|(at, _)| {
            code[..at]
                .chars()
                .next_back()
                .is_none_or(|c| !(c.is_alphanumeric() || c == '_' || c == '.'))
        })
}

/// **The scope ledger is the tree's set of scope readers that do not call
/// the home**: a new one reds here, and the only line a new hand-rolled
/// one can honestly carry is none.
#[test]
fn every_site_that_asks_a_scope_question_is_in_the_scope_ledger() {
    let root = repo_root(env!("CARGO_MANIFEST_DIR"));
    let mut found: Vec<String> = rust_sources(&root)
        .iter()
        .filter_map(|path| {
            let relative = path.strip_prefix(&root).ok()?;
            if relative.components().any(|c| {
                let c = c.as_os_str().to_string_lossy();
                SKIPPED_DIRS.contains(&c.as_ref()) || c.starts_with('.')
            }) {
                return None;
            }
            let text = std::fs::read_to_string(path).expect("a readable source file");
            // The lexed view only blanks comments, so a needle it holds is
            // in the raw text too: screen on the raw text, lex the hits.
            (asks_a_scope_question(&text)
                && asks_a_scope_question(&code_and_literals(&text))
                && !calls_the_scope_home(&code_only(&text)))
            .then(|| relative.to_string_lossy().replace('\\', "/"))
        })
        .collect();
    found.sort();
    let mut ledger: Vec<&str> = SCOPE_LEDGER
        .iter()
        .filter(|e| matches!(e.seen, Seen::Needle))
        .map(|e| e.path)
        .collect();
    ledger.dedup();
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
        "the scope ledger no longer matches the tree.\n  \
         arrived or moved (owe a line; a scope reader resolves through \
         `source_scope`, it is not hand-rolled): {unlisted:#?}\n  \
         listed but no longer asking (delete the line): {stale:#?}"
    );
}

/// **Each line's claim, checked against its file.**
///
/// - [`HandRolled`] and [`NotScope`] name their items, and in a file with
///   either, every needle hit lies in an item some line lists: a reader
///   added beside the listed ones is a hit no line answers for. Each
///   listed item still holds a hit (a converted one holds none: remove
///   it, and its record entry).
/// - [`NotScope`]: each hit in the line's items is a needle its reason
///   admits, and each item is recorded under that reason in
///   [`NOT_SCOPE_AT_THE_HOMES_ARRIVAL`].
/// - [`Seen::Anchor`]: the line's items are still in the file, and the
///   needle does not find the file.
#[test]
fn every_scope_line_holds_of_its_file() {
    let root = repo_root(env!("CARGO_MANIFEST_DIR"));
    let mut wrong: Vec<String> = Vec::new();
    let mut held: Vec<String> = Vec::new();
    let mut paths: Vec<&str> = SCOPE_LEDGER.iter().map(|e| e.path).collect();
    paths.dedup();
    for path in paths {
        let text = std::fs::read_to_string(root.join(path))
            .unwrap_or_else(|err| panic!("reading {path}: {err}"));
        let code = code_only(&text);
        let lexed = code_and_literals(&text);
        let bodies = fn_bodies(&code);
        let lines: Vec<&ScopeEntry> = SCOPE_LEDGER.iter().filter(|e| e.path == path).collect();
        let hits = scope_needle_hits(&lexed);
        let in_items = |items: &[&str], at: usize| {
            innermost_fn(&bodies, at).is_some_and(|f| items.contains(&f))
        };
        let itemised = lines
            .iter()
            .any(|e| matches!(e.disposition, HandRolled(_) | NotScope(..)));
        for e in &lines {
            match e.disposition {
                HandRolled(_) | NotScope(..) if e.items.is_empty() => wrong.push(format!(
                    "{path}: a HandRolled or NotScope line names the items it answers for"
                )),
                NotScope(why, where_) => {
                    if where_.trim().is_empty() {
                        wrong.push(format!("{path}: NotScope({why:?}) says where in the file"));
                    }
                    for (at, n) in &hits {
                        if in_items(e.items, *at) && !why.admits().contains(n) {
                            wrong.push(format!(
                                "{path}: NotScope({why:?}) does not answer for {n} (byte {at})"
                            ));
                        }
                    }
                    for item in e.items {
                        let key = format!("{why:?} {path}::{item}");
                        if !NOT_SCOPE_AT_THE_HOMES_ARRIVAL.contains(&key.as_str()) {
                            wrong.push(format!(
                                "{key} is no item recorded under its reason in \
                                 NOT_SCOPE_AT_THE_HOMES_ARRIVAL"
                            ));
                        }
                        held.push(key);
                    }
                }
                _ => {}
            }
            for item in e.items {
                let named: Vec<&std::ops::Range<usize>> = bodies
                    .iter()
                    .filter(|(n, _)| n == item)
                    .map(|(_, r)| r)
                    .collect();
                if named.is_empty() {
                    wrong.push(format!("{path}: its item `{item}` is gone"));
                    continue;
                }
                let holds = hits
                    .iter()
                    .any(|(at, _)| named.iter().any(|r| r.contains(at)));
                if matches!(e.seen, Seen::Needle) && !holds {
                    wrong.push(format!(
                        "{path}: `{item}` holds no binding keyword now: converted, so remove it \
                         from the line and from its record"
                    ));
                }
            }
            if matches!(e.seen, Seen::Anchor) && asks_a_scope_question(&lexed) {
                wrong.push(format!(
                    "{path}: the needle finds it now, so its line is Seen::Needle"
                ));
            }
        }
        if itemised {
            let listed: Vec<&str> = lines.iter().flat_map(|e| e.items.iter().copied()).collect();
            for (at, n) in &hits {
                if !in_items(&listed, *at) {
                    wrong.push(format!(
                        "{path}: {n} at byte {at} (in {}) is in no item a line lists: a new \
                         scope reader beside the listed ones resolves through `{SCOPE_HOME_DOOR}`",
                        innermost_fn(&bodies, *at)
                            .map_or("no item".to_string(), |f| format!("`{f}`"))
                    ));
                }
            }
        }
    }
    for entry in NOT_SCOPE_AT_THE_HOMES_ARRIVAL {
        if !held.iter().any(|h| h == entry) {
            wrong.push(format!(
                "{entry} is no NotScope item now: remove it from NOT_SCOPE_AT_THE_HOMES_ARRIVAL"
            ));
        }
    }
    assert!(wrong.is_empty(), "scope lines that do not hold: {wrong:#?}");
}

/// **A hand-rolled scope reader is refused**: each [`HandRolled`] item
/// is in the record of the home's arrival, each record entry is a
/// [`HandRolled`] item (a converted one leaves both), and the row that
/// owes the conversion exists. The home is source-scope alone.
#[test]
fn the_hand_rolled_scope_readers_are_the_ones_this_tree_still_owes() {
    let root = repo_root(env!("CARGO_MANIFEST_DIR"));
    let mut held: Vec<String> = Vec::new();
    for e in SCOPE_LEDGER {
        match e.disposition {
            HandRolled(row) => {
                assert!(
                    !e.items.is_empty(),
                    "{}: a HandRolled line names its reader's items",
                    e.path
                );
                for item in e.items {
                    let key = format!("{}::{item}", e.path);
                    assert!(
                        HAND_ROLLED_AT_THE_HOMES_ARRIVAL.contains(&key.as_str()),
                        "{key} is dispositioned HandRolled, and it was no hand-rolled scope \
                         reader when the home landed: a new scope reader resolves through \
                         `{SCOPE_HOME_DOOR}`"
                    );
                    held.push(key);
                }
                assert!(
                    root.join(row).is_file(),
                    "{} owes its conversion to {row}, which does not exist",
                    e.path
                );
            }
            ScopeHome => assert!(
                e.path.starts_with("crates/source-scope/"),
                "{}: the home is source-scope",
                e.path
            ),
            NotScope(..) => {}
        }
    }
    let converted: Vec<&&str> = HAND_ROLLED_AT_THE_HOMES_ARRIVAL
        .iter()
        .filter(|r| !held.iter().any(|h| h == *r))
        .collect();
    assert!(
        converted.is_empty(),
        "these record entries are no HandRolled item now: converted, so remove them from \
         HAND_ROLLED_AT_THE_HOMES_ARRIVAL: {converted:#?}"
    );
}

/// **The scope needle's own plants.** Each is assembled at run time, so
/// this file's source spells none of them as code: a hand scope reader
/// looking for a binding keyword, and one resolving through the home,
/// are hits; the keywords used as Rust (a `let` statement, a `use`
/// item) and in prose are not.
#[test]
fn the_scope_needle_finds_a_binding_keyword_spelled_to_be_searched_for() {
    let q = |s: &str| format!("\"{s}\"");
    let cases: [(&str, bool, String); 6] = [
        (
            "a `let` search",
            true,
            format!("let at = body.find({});", q("let")),
        ),
        (
            "a `use ` prefix test",
            true,
            format!("if line.starts_with({}) {{}}", q("use ")),
        ),
        (
            "a `super` path segment",
            true,
            format!("match seg {{ {} => up(), _ => {{}} }}", q("super")),
        ),
        (
            "the home named",
            true,
            format!("let src = {}::read(text);", SCOPE_HOME_DOOR),
        ),
        (
            "the keywords as Rust",
            false,
            "use std::fmt;\nfn f() { let x = super::g(); }".to_owned(),
        ),
        (
            "the keywords in prose, comments blanked",
            false,
            code_and_literals("// a `let` binds; `use super::*` imports\nfn f() {}"),
        ),
    ];
    for (what, want, code) in cases {
        assert_eq!(asks_a_scope_question(&code), want, "{what}: {code:?}");
    }
    let calls: [(&str, bool, String); 4] = [
        (
            "by path",
            true,
            format!("let s = {SCOPE_HOME_DOOR}::{SCOPE_HOME_READ}(t);"),
        ),
        (
            "by an import",
            true,
            format!(
                "use {SCOPE_HOME_DOOR}::{{{SCOPE_HOME_READ}, Bound}};\nlet s = {SCOPE_HOME_READ}(t);"
            ),
        ),
        (
            "a mention only",
            false,
            format!("use {SCOPE_HOME_DOOR}::Bound;\nfn f(b: Bound) {{}}"),
        ),
        (
            "an import, and a method of the same name",
            false,
            format!("use {SCOPE_HOME_DOOR}::{SCOPE_HOME_READ};\nlet s = f.{SCOPE_HOME_READ}(t);"),
        ),
    ];
    for (what, want, code) in calls {
        assert_eq!(calls_the_scope_home(&code), want, "{what}: {code:?}");
    }
}
