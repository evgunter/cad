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
/// `(` would stop seeing a file whose row was re-delimited — which reds
/// fifteen `Shared` lines over a change that altered nothing. The
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
    /// Reads Rust source through [`test_utils::source`]. The
    /// destination for every member of this class.
    Shared,
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
        path: "crates/editor-core/src/eval/mod.rs",
        disposition: Shared, // node-kind vocabulary census, code view
    },
    Entry {
        path: "crates/editor-core/tests/all.rs",
        disposition: Shared, // mount guard, literal view
    },
    Entry {
        path: "crates/editor-core/tests/docm1_face_frame.rs",
        disposition: Shared, // the two read-door module docs, prose view
    },
    Entry {
        path: "crates/editor-core/tests/docm4_evaluation_identity.rs",
        disposition: Shared, // Evaluation-literal census, code view
    },
    Entry {
        path: "crates/editor-core/tests/docm5_subject.rs",
        disposition: Shared, // the landing's gather call sites and the no-sharing
                             // needles, code view
    },
    Entry {
        path: "crates/editor-core/tests/fix_loop_polygon_expr.rs",
        disposition: Shared, // polygon-close uniqueness census, code view
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
        path: "crates/editor-core/tests/msolve7_member_residue.rs",
        disposition: Shared, // one-environment-per-solve build count over
                             // mate/member.rs and mate/solve.rs, code view
    },
    Entry {
        path: "crates/editor-core/tests/node_standing.rs",
        disposition: Shared, // node-result reader census over every src tree, code view
    },
    Entry {
        path: "crates/editor-core/tests/product_finishes_once.rs",
        disposition: Shared, // product.rs finishes its aggregate once and censuses at
                             // one site, code view (`blanked`)
    },
    Entry {
        path: "crates/editor-core/tests/refusal_concision_chains.rs",
        disposition: Shared, // blend raise-site details over sweep/src/blend,
                             // code and code-and-literals views
    },
    Entry {
        path: "crates/editor-core/tests/wire_operand_door.rs",
        disposition: Shared, // operand-door and expected-phrase census over
                             // eval/wire.rs and verbs/split.rs, code and
                             // code-and-literals views
    },
    Entry {
        path: "crates/geom-brep/src/curve_face/rows.rs",
        disposition: Shared, // door-name prefix census (`predicate_census`), code and
                             // literal views
    },
    Entry {
        path: "crates/geom-brep/src/sector_shape.rs",
        disposition: Shared, // the rung names spelled in one module, code-and-literals view
    },
    Entry {
        path: "crates/geom-brep/tests/all.rs",
        disposition: Shared, // mount guard, literal view
    },
    Entry {
        path: "crates/geom-brep/tests/kstats_escalation_channel.rs",
        disposition: Shared, // op-minted `Indeterminate` scan, code view
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
        path: "crates/geom-core/tests/bounds_census.rs",
        disposition: Shared, // sole-bracket-bound roster, code view
    },
    Entry {
        path: "crates/geom-core/tests/certified_endpoint_census.rs",
        disposition: Shared, // certification endpoint-read census, code view + literal view
    },
    Entry {
        path: "crates/geom-core/tests/flagged_census.rs",
        disposition: Shared, // call census, code view + offsets
    },
    Entry {
        path: "crates/geom-core/tests/sym_rule_f_rows.rs",
        disposition: Shared, // `copysign` mint-site register over crates/*/src,
                             // code view, each file cut at its test module
    },
    Entry {
        path: "crates/geom/src/curves/on_carrier/rows.rs",
        disposition: Shared, // on-carrier seed-read census over crates/*/src, code view
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
        path: "crates/pncad-py/src/prose_census.rs",
        disposition: Shared, // every `impl Display` in the tree: code view for the
                             // structure, literal view for the format string
    },
    Entry {
        path: "crates/pncad-py/src/surface_census.rs",
        disposition: Shared, // every `*Options` type CONSTRUCTED under src/py/, so the
                             // options rosters' membership is derived rather than hand-kept:
                             // code view, which is what keeps a doc comment naming a struct
                             // from reading as a door building one
    },
    Entry {
        path: "crates/pncad-py/src/tests.rs",
        disposition: Shared, // the tag table in src/tags.rs: code view to locate, literal to
                             // read; the kind words in src/node_kind.rs: literal view alone;
                             // the literal census of src/errors.rs: code, literal and comment
                             // views together, so a literal adjoining a comment is told apart;
                             // the roster's and the blind-spot list's test names, re-derived
                             // against this file's own source: code view, which is what keeps
                             // a name written in prose from answering yes; and the instance
                             // attributes pncad.pyi declares, which is a Python stub and no
                             // Rust source at all — it is read by line prefix and triple-quote
                             // parity, with tests/test_stubs.py's `ast` walk the second reader
                             // of that one convention (work/census/one-stub-convention-…)
    },
    Entry {
        path: "crates/pncad/tests/all.rs",
        disposition: Shared, // the facade boundary guards, code and literal views; and the
                             // box-document fixture twin in crates/pncad-py/src/tests.rs,
                             // raw text between sentinels, compared with its own copy
    },
    Entry {
        path: "crates/profile/tests/all.rs",
        disposition: Shared, // mount guard, literal view
    },
    Entry {
        path: "crates/profile/tests/gate_kinds_followed.rs",
        disposition: Shared, // fillet_recourse.rs's gate rows against its prose,
                             // code-and-literals and prose views
    },
    Entry {
        path: "crates/profile/tests/r2_bool9_review_probes.rs",
        disposition: Shared, // the value-equal row's ceiling is asserted to
                             // EXIST in lift_census.rs, code+literal view
    },
    Entry {
        path: "crates/profile/tests/raw_door_census.rs",
        disposition: Shared, // production-writer census + the raw door's own
                             // gate, code+literal view
    },
    Entry {
        path: "crates/profile/tests/recourse_roster.rs",
        disposition: Shared, // the dispatch-order row reads path.rs's own match
                             // patterns, code+literal view. The decide-site walk
                             // it also runs is source::predicate_census, so that
                             // half reads nothing here
    },
    Entry {
        path: "crates/profile/tests/seal.rs",
        disposition: Shared, // serde-free seal, code view
    },
    Entry {
        path: "crates/source-scope/tests/all.rs",
        disposition: Shared, // mount guard, literal view
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
        path: "crates/step-import/tests/per_part_gate_policy.rs",
        disposition: Shared, // lib.rs's per-part policy consultation, code view
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
        path: "crates/sweep/src/blend/admit.rs",
        disposition: Shared, // token construction sites, code view
    },
    Entry {
        path: "crates/sweep/src/lib.rs",
        disposition: Shared, // caller's re-description door call census, code view
    },
    Entry {
        path: "crates/sweep/tests/all.rs",
        disposition: Shared, // mount guard, literal view
    },
    Entry {
        path: "crates/sweep/tests/review_blend5_r5_probes.rs",
        disposition: Shared, // the doc citations and the rows they name,
                             // prose and code views
    },
    Entry {
        path: "crates/sweep/tests/review_fillet_split_r2_probes.rs",
        disposition: Shared, // the seam's and open bands' visibility census, code view
    },
    Entry {
        path: "crates/sweep/tests/review_fillet_t_r1_probes.rs",
        disposition: Shared, // the blend surgery's one `kef` door, code view
    },
    Entry {
        path: "crates/test-utils/src/source.rs",
        disposition: Home,
    },
    Entry {
        path: "crates/test-utils/tests/all.rs",
        disposition: Shared, // mount guard, literal view
    },
    Entry {
        path: "crates/test-utils/tests/deny_unknown_fields_census.rs",
        disposition: Shared, // `deny_unknown_fields` sites and the declaration
                             // each heads, code view
    },
    Entry {
        path: "crates/test-utils/tests/hand_written_impl_census.rs",
        disposition: Shared, // hand-written `Debug`/`PartialEq` impls and the bodies
                             // they walk, code view
    },
    Entry {
        path: "crates/test-utils/tests/reader_census.rs",
        disposition: Shared, // this census, literal view
    },
    Entry {
        path: "crates/topo/src/boolean/boxes.rs",
        disposition: Unconverted(
            "Track Q — reaches the shared lexer only through `source_walk::CodeOnly`, \
             topo's handle on it; the direct call is Track Q's to make",
        ),
    },
    Entry {
        path: "crates/topo/src/boolean/join.rs",
        disposition: Shared, // the join's one face-to-surface walk, code view
    },
    Entry {
        path: "crates/topo/src/boolean/offer_rows.rs",
        disposition: Shared, // the Boolean's decision sites, code view
    },
    Entry {
        path: "crates/topo/src/boolean/ops.rs",
        disposition: Shared, // the smooth seam arm's route through the rule, code view
    },
    Entry {
        path: "crates/topo/src/census.rs",
        disposition: Shared, // the backstop's no-inline-`what` guard, literal view
                             // carved by `balanced_end`
    },
    Entry {
        path: "crates/topo/src/chord_join.rs",
        disposition: Unconverted("Track Q — whitespace-stripped raw text, no reader"),
    },
    Entry {
        path: "crates/topo/src/euler.rs",
        disposition: Shared, // the removal census over validate.rs's tier-1 pass and
                             // the kill helpers' bodies, code view
    },
    Entry {
        path: "crates/topo/src/face_normal.rs",
        disposition: Unconverted("Track Q — raw text"),
    },
    Entry {
        path: "crates/topo/src/link.rs",
        disposition: Shared, // the one-door guard over link.rs and census.rs's touch
                             // section, code view carved by `balanced_end`
    },
    Entry {
        path: "crates/topo/src/live.rs",
        disposition: Shared, // the `Live` door guard, code view carved by `balanced_end`
    },
    Entry {
        path: "crates/topo/src/policy_lane.rs",
        disposition: Shared, // the edge-certification origin census, code view carved
                             // by `attributed_extent`
    },
    Entry {
        path: "crates/topo/src/review_d18.rs",
        disposition: Shared, // the announcing body, code view carved by `balanced_end`
    },
    Entry {
        path: "crates/topo/src/review_d18_probes.rs",
        disposition: Shared, // `unreachable!` message texts, literal view
    },
    Entry {
        path: "crates/topo/src/sector_shape.rs",
        disposition: Unconverted("Track Q — raw text, no reader at all"),
    },
    Entry {
        path: "crates/topo/src/source_walk.rs",
        disposition: Shared, // the mutation-door walk, code view
    },
    Entry {
        path: "crates/topo/src/surgery.rs",
        disposition: Shared, // `Body`'s surgery-depth field declaration, code view
    },
    Entry {
        path: "crates/topo/src/tier3_tests.rs",
        disposition: Shared, // the one-walk guard over validate.rs and rim_wedge.rs, code view
    },
    Entry {
        path: "crates/topo/src/validate.rs",
        disposition: Shared, // the at-rest door roster, prose view and code view
    },
    Entry {
        path: "crates/topo/tests/all.rs",
        disposition: Shared, // mount guard, literal view
    },
    Entry {
        path: "crates/topo/tests/r1_lane1_bracket_read_census.rs",
        disposition: Shared, // validate.rs's bracket reads, code view
    },
    Entry {
        path: "crates/topo/tests/certified_enclosure_impl_census.rs",
        disposition: Shared, // door values and CertifiedEnclosure impls vs wiring rows, code view
    },
    Entry {
        path: "crates/topo/tests/readback_sense_kind.rs",
        disposition: Shared, // the query seat's body, code view
    },
    Entry {
        path: "crates/topo/tests/shell_tolerance_chain.rs",
        disposition: Shared, // the shell offset chain's signatures, code view
    },
    Entry {
        path: "crates/verbs/tests/all.rs",
        disposition: Shared, // mount guard, literal view
    },
    Entry {
        path: "crates/verbs/tests/layer_guard.rs",
        disposition: Shared, // the crate's src/ directory walk, for the file-list pin
    },
    Entry {
        path: "crates/viewer/src/gpu.rs",
        disposition: Shared, // pipeline census over its own source, code view
    },
    Entry {
        path: "crates/viewer/src/widgets.rs",
        disposition: Shared, // the helper roster and the message roster, code view
    },
    Entry {
        path: "crates/viewer/tests/all.rs",
        disposition: Shared, // mount guard, literal view
    },
    Entry {
        path: "crates/viewer/tests/frame_policy.rs",
        disposition: Shared, // the README's badge-door and store-read counts, code view
    },
    Entry {
        path: "crates/viewer/tests/gesture_table.rs",
        disposition: Shared, // the cancel doors' chrome reader, code view
    },
    Entry {
        path: "crates/viewer/tests/landing_gathers.rs",
        disposition: Shared, // the gather counter's three gated sites, code view
    },
    Entry {
        path: "crates/viewer/tests/tree_badges.rs",
        disposition: Shared, // the standing doors' as-drawn census, code view
    },
    Entry {
        path: "demos/tour/tests/all.rs",
        disposition: Shared, // mount guard, literal view
    },
    Entry {
        path: "tools/k-lint/tests/predicate_roster.rs",
        disposition: Shared, // roster pinned to the kernel's mint: code view to locate, literal view to read
    },
    Entry {
        path: "tools/tess-meter/tests/derivations.rs",
        disposition: Shared, // cross-root const pins: code view to locate, literal view to read
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

/// `code` with `//`-to-end-of-line comments removed — a marker is an ITEM and
/// never a comment, and without the cut the macro's own documentation (which
/// quotes a call) subtracts paths nothing declares. A line cut rather than a
/// lexer, the same trade `scripts/ci-filter.py` makes and for the same reason:
/// a `//` inside a string literal truncates the line early, which can only
/// LOSE an invocation and never invent one.
fn strip_line_comments(code: &str) -> String {
    code.lines()
        .map(|l| match l.find("//") {
            Some(i) => &l[..i],
            None => l,
        })
        .collect::<Vec<_>>()
        .join("\n")
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

/// **The ledger is the tree's set, not a subset of it.**
///
/// A WALK THAT MATCHED NOTHING IS NOT A PASS, and the equality is what
/// says so: an empty or broken traversal reports every entry in the
/// ledger as stale and reds, so this row needs no separate count floor
/// (an earlier one asserted `found.len() >= 20`, which set equality had
/// already subsumed and which could not fail for the reason it
/// stated). [`test_utils::source::rust_sources`] panics on an empty
/// directory underneath it as well.
#[test]
fn every_site_that_reads_rust_source_is_in_the_ledger() {
    let root = repo_root(env!("CARGO_MANIFEST_DIR"));
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

/// The ways a site reaches the shared lexer, as they are SPELLED.
///
/// Two, because one of them hides the other: an aggregating
/// `tests/all.rs` reaches `test_utils::source::aggregation_violations`
/// through the aggregation row macro, and the module path it reaches is
/// in the macro's EXPANSION rather than in the file. A door added to the
/// façade that no site can be seen to use does not belong here; a door
/// that fifteen files use does.
///
/// **What the second entry costs, stated because it is a real trade.**
/// The first entry is checked against its subject on every `Shared`
/// row: the file names `test_utils::source` or it does not. The second
/// is a claim about a macro body that is NOT in the file being read, and
/// `crates/test-utils/src/source.rs` — where that body lives — is
/// dispositioned [`Home`], so it is filtered out of
/// [`every_shared_entry_actually_reaches_the_shared_lexer`] before that
/// row looks at anything. Before the collapse, fifteen `all.rs` files
/// each carried their own copy and each was checked on its own text; a
/// reversion in ONE of them red'd that file. Now one body answers for
/// fifteen call sites.
/// [`the_aggregation_row_macro_reaches_the_shared_lexer`] is what checks
/// it, and it is ONE check where there were fifteen — the loss of
/// redundancy is what the collapse costs, and it is not nothing.
const SHARED_LEXER_DOORS: [&str; 2] = [
    "test_utils::source",
    concat!("test_utils::", aggregation_row_macro!()),
];

/// **A `Shared` line is a CLAIM, and this is what checks it.**
///
/// Without this row the ledger's own silent direction is the one it
/// exists to close: a converted site that reverts to a hand-rolled
/// reader keeps its `Shared` line, keeps tripping the detector, and the
/// census stays green over exactly the change it was built to catch.
#[test]
fn every_shared_entry_actually_reaches_the_shared_lexer() {
    let root = repo_root(env!("CARGO_MANIFEST_DIR"));
    let liars: Vec<&str> = LEDGER
        .iter()
        .filter(|e| matches!(e.disposition, Shared))
        .filter(|e| {
            let text = std::fs::read_to_string(root.join(e.path))
                .unwrap_or_else(|err| panic!("reading {}: {err}", e.path));
            // The code view: a mention in prose is not a call.
            let code = test_utils::source::code_only(&text);
            !SHARED_LEXER_DOORS.iter().any(|door| code.contains(door))
        })
        .map(|e| e.path)
        .collect();
    assert!(
        liars.is_empty(),
        "these entries are dispositioned Shared but no longer call \
         `test_utils::source`, by any of {SHARED_LEXER_DOORS:?} — either they reverted \
         to a hand-rolled reader, or the line is stale: {liars:#?}"
    );
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
/// matches nothing, fifteen `Shared` lines red as stale, and the cause is
/// a needle nobody edited rather than a reader anybody reverted. This row
/// is the edge a rename trips on instead, and it names the cause.
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
/// a body in `crates/test-utils/src/source.rs`, which is dispositioned
/// [`Home`] and therefore never looked at by
/// [`every_shared_entry_actually_reaches_the_shared_lexer`] — so
/// rewriting the macro to walk `tests/` by hand leaves all fifteen
/// `Shared` lines green and every other row in this file passing.
/// Measured, not supposed: that reversion reds this row and nothing else.
///
/// Before the collapse each of the fifteen carried its own copy and each
/// was checked against its own file. This is the one that survives, and
/// it is stated as one check replacing fifteen rather than as their
/// equal.
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
const UNCONVERTED_TODAY: usize = 4;

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
struct ScopeEntry {
    /// Path relative to the repository root, `/`-separated.
    path: &'static str,
    disposition: ScopeDisposition,
}

enum ScopeDisposition {
    /// `crates/source-scope` itself.
    ScopeHome,
    /// Answers its scope questions through `source_scope`. Checked:
    /// the file must name it.
    Parsed,
    /// Spells a binding keyword for a reason that is not a scope
    /// question. The payload says what the spelling is for.
    NotScope(&'static str),
    /// A hand-rolled scope reader that predates the home. The payload
    /// names the work row that owes its conversion. **A new hand-rolled
    /// scope reader does not get this line**: use `source_scope`.
    HandRolled(&'static str),
}

use ScopeDisposition::{HandRolled, NotScope, Parsed, ScopeHome};

/// The work row every pre-existing hand-rolled scope reader is owed by.
const SCOPE_CONVERSION_ROW: &str =
    "work/onecopy/four-hand-rolled-scope-readers-predate-source-scope.md";

/// **The scope ledger.** One line per site [`asks_a_scope_question`]
/// finds. Sorted by path.
const SCOPE_LEDGER: &[ScopeEntry] = &[
    ScopeEntry {
        path: "crates/geom-core/tests/flagged_census.rs",
        disposition: Parsed, // the flagged doors' call sites and forwarding wrappers
    },
    ScopeEntry {
        path: "crates/pncad-py/src/prose_census.rs",
        disposition: HandRolled(SCOPE_CONVERSION_ROW), // `use_bindings` / `normalize`
    },
    ScopeEntry {
        path: "crates/pncad-py/src/tests.rs",
        disposition: HandRolled(SCOPE_CONVERSION_ROW), // a fn's `use … as R;` shorthands
    },
    ScopeEntry {
        path: "crates/pncad/tests/all.rs",
        disposition: HandRolled(SCOPE_CONVERSION_ROW), // `use_statement_roots`
    },
    ScopeEntry {
        path: "crates/source-scope/src/lib.rs",
        disposition: ScopeHome,
    },
    ScopeEntry {
        path: "crates/source-scope/tests/scopes.rs",
        disposition: Parsed, // the home's own resolution rows
    },
    ScopeEntry {
        path: "crates/step-import/tests/inst_review_probes.rs",
        disposition: NotScope("an `.expect(\"use\")` message"),
    },
    ScopeEntry {
        path: "crates/sweep/tests/review_fillet_split_r2_probes.rs",
        disposition: NotScope("a `pub(super)` visibility, read as a word"),
    },
    ScopeEntry {
        path: "crates/test-utils/src/source.rs",
        disposition: NotScope("the lexer's keyword tables: where an item head starts"),
    },
    ScopeEntry {
        path: "crates/test-utils/tests/hand_written_impl_census.rs",
        disposition: HandRolled(SCOPE_CONVERSION_ROW), // `self_aliases`, one `let` deep
    },
    ScopeEntry {
        path: "crates/test-utils/tests/reader_census.rs",
        disposition: NotScope("this census's own needles"),
    },
    ScopeEntry {
        path: "crates/topo/src/boolean/offer_rows.rs",
        disposition: NotScope("skips a `use` line, so an import is not a passing site"),
    },
    ScopeEntry {
        path: "crates/topo/src/source_walk.rs",
        disposition: NotScope("blanks `use` declarations: a declaration reaches nothing"),
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

/// Whether `code` (a comments-blanked, literals-kept view) asks a scope
/// question: it spells a binding keyword as a literal, or it names the
/// home.
fn asks_a_scope_question(code: &str) -> bool {
    SCOPE_KEYWORD_LITERALS.iter().any(|n| code.contains(n)) || code.contains(SCOPE_HOME_DOOR)
}

/// **The scope ledger is the tree's set**, the way [`LEDGER`] is: a new
/// scope reader reds here until it has a line, and the only line a new
/// hand-rolled one can honestly carry is none.
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
            (asks_a_scope_question(&text) && asks_a_scope_question(&code_and_literals(&text)))
                .then(|| relative.to_string_lossy().replace('\\', "/"))
        })
        .collect();
    found.sort();
    let ledger: Vec<&str> = SCOPE_LEDGER.iter().map(|e| e.path).collect();
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

/// **A `Parsed` line is a claim**, checked the way a `Shared` one is: a
/// site that reverts to a hand-rolled scope reader keeps tripping the
/// needle, and without this row it would keep its line too.
#[test]
fn every_parsed_entry_actually_resolves_through_source_scope() {
    let root = repo_root(env!("CARGO_MANIFEST_DIR"));
    let liars: Vec<&str> = SCOPE_LEDGER
        .iter()
        .filter(|e| matches!(e.disposition, Parsed))
        .filter(|e| {
            let text = std::fs::read_to_string(root.join(e.path))
                .unwrap_or_else(|err| panic!("reading {}: {err}", e.path));
            !code_only(&text).contains(SCOPE_HOME_DOOR)
        })
        .map(|e| e.path)
        .collect();
    assert!(
        liars.is_empty(),
        "these entries are dispositioned Parsed but no longer name `{SCOPE_HOME_DOOR}`: {liars:#?}"
    );
}

/// The number of hand-rolled scope readers that predate the home.
/// **Hand-synced, and it goes one way**, like [`UNCONVERTED_TODAY`].
const HAND_ROLLED_SCOPE_READERS_TODAY: usize = 4;

/// **A hand-rolled scope reader is refused**: the count goes one way,
/// and each pre-existing one names the row that owes its conversion,
/// which must exist.
#[test]
fn the_hand_rolled_scope_readers_are_the_ones_this_tree_still_owes() {
    let root = repo_root(env!("CARGO_MANIFEST_DIR"));
    let owed: Vec<&ScopeEntry> = SCOPE_LEDGER
        .iter()
        .filter(|e| matches!(e.disposition, HandRolled(_)))
        .collect();
    assert_eq!(
        owed.len(),
        HAND_ROLLED_SCOPE_READERS_TODAY,
        "the scope ledger holds {} hand-rolled scope readers and \
         HAND_ROLLED_SCOPE_READERS_TODAY says {HAND_ROLLED_SCOPE_READERS_TODAY}. It goes \
         down as one converts onto `source_scope`; it is not raised for a new one: \
         {:#?}",
        owed.len(),
        owed.iter().map(|e| e.path).collect::<Vec<_>>()
    );
    for e in &owed {
        let HandRolled(row) = e.disposition else {
            unreachable!()
        };
        assert!(
            root.join(row).is_file(),
            "{} owes its conversion to {row}, which does not exist",
            e.path
        );
    }
    for e in SCOPE_LEDGER {
        if let NotScope(why) = e.disposition {
            assert!(
                !why.trim().is_empty(),
                "{}: a NotScope line says what it is for",
                e.path
            );
        }
        if matches!(e.disposition, ScopeHome) {
            assert!(
                e.path.starts_with("crates/source-scope/"),
                "{}: the home is source-scope",
                e.path
            );
        }
    }
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
}
