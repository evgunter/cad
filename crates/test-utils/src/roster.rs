//! **A file's header roster, welded to the rows it names.**
//! [`crate::roster!`] is the whole mechanism: a block of idents that
//! replaces a hand-kept `//!` enumeration of a file's own `#[test]`
//! rows, and a generated row that compares it against libtest's own
//! `--list`.
//!
//! "Roster" names this macro's block here and nothing else. The word
//! has two other referents in this tree — `probe-suite-census.sh`'s
//! "rostered as executed" (`:535`), and the row enumerations in `work/`
//! item headers — and neither is this. The macro is not renamed away
//! from the collision because 192 files under `work/` use the word and
//! the item this module closes is one of them.
//!
//! # Why a roster needs a weld at all
//!
//! A doc comment that enumerates its file's rows is a list nobody
//! updated, by construction: nothing reads it, so a row added, removed
//! or renamed leaves it green and wrong. The shape this module exists
//! for had all three failures at once in one file — five of seven rows
//! listed, the most expensive row in the tree absent, and one entry
//! naming a `fn` that exists nowhere in the tree.
//!
//! # The ground truth is the harness, not the source text
//!
//! [`listed_rows`] re-execs the running test binary with
//! `--list --format=terse` and reads what libtest says it contains.
//! **No Rust is parsed.** That matters: a guard that scans source text
//! for `#[test] fn` has to answer *is this text code, a comment or a
//! literal* and gets a different answer than rustc does — a real row
//! commented out leaves its text in the file. libtest's own list is the
//! set that will actually run, including `#[ignore]`d rows and rows
//! nested in modules, and it is derived from the compiled binary.
//!
//! The re-exec is a listing, not a run: the child prints and exits. The
//! tree already re-execs its own test binary at eighteen sites to *run*
//! a child row, which is strictly more than this.
//!
//! **That argument is not novel here, and this module is not the first
//! site to make it.** `scripts/gates/probe-suite-census.sh` already
//! parses `cargo test --test all -- --list` for module names (`:657`) —
//! the mirror of `row_prefix`'s split — and its own `--selftest`
//! refuses to pass on an empty listing (`:1067`-`:1072`), which is this
//! module's vacuity floor written independently in bash.
//! `.github/workflows/ci.yml` (`:4837`) is what produces that listing
//! for it, and `scripts/check-ci-mirror-parity.py` (`:1584`) reads the
//! `listing=$(cargo test … --list)` assignment as a live row rather
//! than dropping it — the two neighbouring readers of the same output.
//!
//! Nothing is shared between those three and this module, and nothing
//! can be. They are shell and Python reading a listing produced by a
//! `cargo test` invocation they spell themselves; this is Rust inside
//! the binary being listed, which is the only seat from which
//! `module_path!()` answers for the invoking module. The shell/Rust
//! boundary is the whole reason for the second implementation. What
//! travels is the argument, so each end now names the other.
//!
//! # What it welds, and what it will never weld
//!
//! **NAMES, and never PROSE.** Each entry carries a hand-written
//! sentence saying what the row is for, and nothing checks it against
//! the row. The defect that motivated this module had both halves: one
//! entry named a `fn` that does not exist, and *described the row in
//! the opposite sense to what it asserts* — the header called a cache
//! key blind to the dials that move a report, where the row asserts the
//! key tells them apart. The first half is now a compile error. **The
//! second half is caught by nothing here and never will be**; a
//! sentence is not checkable against an assertion by any mechanism in
//! this crate. Read the row's own doc comment, which sits on the row
//! and moves with it.
//!
//! The prose column stays anyway, because nothing computes with it: it
//! is the human-readable half of the roster, it is what a reader lost
//! when the `//!` enumeration went away, and being wrong in it is
//! exactly as bad as it was before. What changed is that the *names*
//! beside it can no longer be wrong.
//!
//! It has exactly one consumer, and that consumer constrains its TYPE
//! and not its content: the macro binds every sentence into a
//! `const _: &[&str]`, so `the_row: 42` is a compile error rather than
//! a silently accepted column. Nothing reads the sentence, and no
//! addition to this crate will make a sentence checkable against an
//! assertion.
//!
//! **Nothing requires a file to HAVE a roster.** A suite that grows a
//! header enumeration and never invokes the macro is silent, exactly as
//! it was before this module existed.
//!
//! **A roster names its module's DIRECT rows, and a nested `#[test]`
//! is a violation rather than a blind spot.** A `#[test]` in a module
//! nested inside the file lists as `<file>::<inner>::<row>`, and a
//! roster entry is an ident that cannot name a path. Passing over one
//! silently would leave a row in the PASS list that the roster does
//! not name, under a guard whose name says it names every row in the
//! file — so the guard names it instead and says to hoist it to the
//! file's own level.
//!
//! The cost is real and deliberate: a file whose rows live in nested
//! `mod`s cannot adopt this macro until they are hoisted, and the
//! guard says so in the message rather than leaving the author to
//! infer it. The alternative — narrowing the guard's NAME to what a
//! path-blind comparison can see — was refused, because the name is
//! what reaches the PASS list, `--filter` expressions and every future
//! citation of this row.
//!
//! # Why this is a function plus a thin macro
//!
//! [`crate::every_suite_file_is_aggregated!`] has to be a macro in
//! full, because `env!` and `include_str!` answer for the crate and the
//! file whose text holds them — written here they would all answer for
//! `test-utils`. **`current_exe()` has no such problem**: it is a
//! runtime question about the running process, and the process is the
//! invoking crate's test binary wherever the code asking lives. So the
//! walk, the comparison and the argument for each live here, once, and
//! the macro carries only what must expand at the call site —
//! `module_path!()`, and the caller's idents.

// This module PANICS and `expect`s, deliberately, for `source`'s reason
// one file over: a guard that cannot ask the harness what it contains
// must go RED rather than quietly compare a roster against an empty
// set. The workspace's no-panic rule is about production code, and
// nothing on a shipped build path can reach here — no production
// manifest names this crate at all (see the crate docs).
#![allow(clippy::panic, clippy::expect_used)]

/// The rows libtest reports for the RUNNING test binary, named as
/// libtest names them (`module::row`, crate root stripped).
///
/// Rows whose listed kind is not `test` — a `#[bench]` reports
/// `benchmark` — are dropped, so a roster is about `#[test]` rows only.
///
/// # Panics
///
/// If the running executable has no path, cannot be re-executed, exits
/// non-zero when asked to list, or prints something that is not UTF-8.
/// Each is a broken harness rather than a test outcome, and each is
/// loud.
#[must_use]
pub fn listed_rows() -> Vec<String> {
    let exe = std::env::current_exe().expect("a running test binary has a path");
    let out = std::process::Command::new(&exe)
        .args(["--list", "--format=terse"])
        .output()
        .unwrap_or_else(|e| panic!("re-exec of {} --list failed: {e}", exe.display()));
    assert!(
        out.status.success(),
        "{} --list exited {}: {}",
        exe.display(),
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8(out.stdout).expect("libtest's --list output is UTF-8");
    rows_of_listing(&stdout)
}

/// The `--list --format=terse` parse, over text supplied by the caller
/// — separated from [`listed_rows`] so that the thing it has to get
/// right (every line whose kind is not `test` is dropped, and so is the
/// trailing summary) is stated as a listing in this module's own rows.
/// Read from the running binary it cannot be: no binary in this tree
/// carries a `#[bench]`, so the kind filter would be exercised by
/// nothing and a row asserting over it could not go red.
#[must_use]
fn rows_of_listing(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .filter_map(|line| line.strip_suffix(": test"))
        .map(str::to_owned)
        .collect()
}

/// The prefix libtest puts in front of every row of the module
/// `module_path` names.
///
/// libtest strips the crate root from a test's path, so
/// `all::r2_m10_6_probes_interval` lists its rows as
/// `r2_m10_6_probes_interval::<row>`. A file that IS the crate root of
/// its own `[[test]]` target has no `::` at all, and its rows list
/// bare — an empty prefix, which is the right answer for a binary that
/// holds nothing else.
///
/// The trailing `::` is what stops a naive filter matching a sibling
/// module whose name extends this one.
#[must_use]
fn row_prefix(module_path: &str) -> String {
    match module_path.split_once("::") {
        Some((_crate_root, rest)) => format!("{rest}::"),
        None => String::new(),
    }
}

/// Compare a roster against what libtest says the invoking module
/// contains. Returns one sentence per violation; empty means the two
/// agree.
///
/// `module_path` is the invoking module's `module_path!()`, `guard_row`
/// is the name of the generated row doing the asking (it is the
/// mechanism, not a subject of the roster, so it is excluded from both
/// sides), and `rostered` is the roster's names.
///
/// # Panics
///
/// Through [`listed_rows`], on a harness that cannot list itself.
#[must_use]
pub fn roster_violations(module_path: &str, guard_row: &str, rostered: &[&str]) -> Vec<String> {
    violations_against(&listed_rows(), module_path, guard_row, rostered)
}

/// The comparison itself, over a listing supplied by the caller —
/// separated from [`listed_rows`] so that the cases it has to get right
/// (a sibling module whose name EXTENDS this one, a nested module, a
/// module with no direct rows left) are stated as listings in this
/// module's own rows rather than depending on what binary is running.
#[must_use]
fn violations_against(
    listed: &[String],
    module_path: &str,
    guard_row: &str,
    rostered: &[&str],
) -> Vec<String> {
    let prefix = row_prefix(module_path);
    let under_prefix: Vec<&str> = listed
        .iter()
        .filter_map(|row| row.strip_prefix(&prefix))
        .collect();
    // A nested module's rows list as `<inner>::<row>`, which no ident
    // can name. They are a violation, not a blind spot: see `nested`.
    let (nested, direct): (Vec<&str>, Vec<&str>) =
        under_prefix.iter().partition(|row| row.contains("::"));
    let mine: Vec<&str> = direct.into_iter().filter(|row| *row != guard_row).collect();

    let mut violations = Vec::new();

    // The vacuity floor. The comparison below is vacuous on an empty
    // `mine`, and without this it would be the same green as a roster
    // that agrees. The reachable cause is NOT a mis-derived prefix —
    // in a real build both sides of it come from the compiler — it is
    // that this module has no direct rows left. So the message leads
    // with that, and the comparison below still runs and names the
    // rostered rows that are gone.
    if mine.is_empty() {
        violations.push(format!(
            "this roster compared itself against nothing: of the {} rows in this binary, \
             none other than this guard is a #[test] listed directly under `{prefix}`. \
             Either every row this module had is gone — deleted, or moved into a \
             module nested inside it — or `{module_path}` is not the path libtest lists \
             this module's rows under. The names still in the roster are reported below.",
            listed.len()
        ));
    }

    if !nested.is_empty() {
        violations.push(format!(
            "these #[test] rows are in modules NESTED inside this one, where a roster \
             entry — an ident — cannot name them: {nested:?}. Hoist each one to this \
             file's own level so the roster can name it, or drop the roster!{{}} from \
             this file. This guard's name says it names every row in the file; a nested \
             row passed over silently is what would make that name false."
        ));
    }

    // The two set directions are ONE comparison, and this crate holds
    // exactly one of those (`crate::census::set_difference`). Spelling
    // the filters again here would be a second copy of a comparator
    // kept in step by hand, in the module whose subject is a second
    // list kept in step by hand. The two violations above are NOT set
    // directions — they are pushed before any comparison and say why
    // the comparison cannot answer — so they stay their own entries.
    if let Some(report) = crate::census::set_difference(
        rostered,
        &mine,
        &format!(
            "this roster!{{}} block and the #[test] rows libtest lists under `{prefix}` disagree"
        ),
        "these are #[test] rows in this module and NOT in its roster!{} block — add each \
         one with a sentence saying what it is for",
        "these compile as `fn()` in this module but are not #[test] rows libtest lists \
         under that prefix. A roster names rows, not helpers",
    ) {
        violations.push(report);
    }

    violations
}

/// **A file's roster of its own `#[test]` rows, welded to them.**
/// Replaces a hand-kept `//!` enumeration:
///
/// ```ignore
/// test_utils::roster! {
///     the_certifying_filter_changes_a_pre_m10_6_documents_drive:
///         "the unit's zero-impact claim, exhibited on a document it does not hold",
///     a_tolerance_study_end_to_end_through_the_public_doors:
///         "the whole consumer walk; the suite's critical path",
/// }
/// ```
///
/// One invocation per file, at the top, right under the `//!` header —
/// which stops enumerating and points here.
///
/// # One token, three consumers
///
/// Each entry is an **ident**, not a string, and it feeds all three:
///
/// 1. `let _: fn() = $row;` — a retired or misspelt name is a **compile
///    error** (`error[E0425]`), on the branch that retired it.
/// 2. `stringify!($row)` — the name compared against libtest's list is
///    the same token, so the compared string cannot be mistyped
///    independently of the one rustc checked.
/// 3. The block itself is what a human reads, with the sentence beside
///    each name.
///
/// `module_path!()` supplies the module prefix, so a `git mv` or a
/// renamed `mod` line re-spells it. Nothing about the file is typed
/// twice.
///
/// # What it does NOT enforce
///
/// **Names, never prose.** The sentence beside each name is
/// hand-written and unchecked — it can describe a row in the opposite
/// sense to what the row asserts, and nothing here will say so. The
/// generated row's one use of it is `const _: &[&str]`, which makes a
/// non-string column (`row: 42`) a compile error and says nothing
/// whatever about what the string means. See this module's docs.
///
/// **Nothing requires a file to have a roster**, so a header
/// enumeration that never adopts this macro is silent.
///
/// **Only what libtest LISTS is in reach.** A `#[bench]` in the module
/// lists with kind `benchmark`, is dropped by the parse, and is
/// therefore neither demanded nor accepted — unrostered and unremarked.
/// No `#[bench]` exists in this tree today. The same holds for a target
/// that runs no libtest harness at all: `benches/Cargo.toml` sets
/// `harness = false`, and that root is outside the workspace and holds
/// no roster.
///
/// **A row in another file of the same binary is not this roster's**,
/// which is the prefix doing its job rather than a hole: an aggregated
/// `all` binary holds every suite of its crate, and each file answers
/// for its own module path.
///
/// # What the generated row costs
///
/// One `--list` re-exec of the test binary, in single-digit
/// milliseconds. Measured at 6.6-7.9 ms over eight runs on
/// `editor-core`'s aggregated `all` binary under `--features interval`
/// — the largest in the tree, ~450 MB and 1630 listed rows, of which
/// this file contributes eight.
///
/// Nothing goes red when those figures stop being true, and they get
/// neither a guard nor a scheduled re-measure. A guard would have to
/// re-take the measurement on every run, which is to pay the cost in
/// order to assert it is small; the size and the row count move with
/// every commit that adds a test, and the millisecond figure moves with
/// the machine. What the number is for is the decision a reader is
/// asked to accept — that one re-exec per adopting file is affordable —
/// and the order of magnitude is what that decision turns on, which is
/// why an exact byte count is not written down here. This is the third
/// case in `work/guard/measurements-have-no-mechanical-guard.md`:
/// unguardable, with the reason written down.
///
/// # Where the row's name comes from
///
/// The generated `fn` is named once, in the first rule below, and the
/// string the comparison excludes itself by is `stringify!` of that
/// same token — the copy this macro exists to end is not one this macro
/// makes.
///
/// `macro_rules!` has no private rules, so the `@weld` rule below is
/// reachable from any crate and will name the guard whatever it is
/// given; the one-name property holds for the door above it, not
/// against a caller who walks around it.
#[macro_export]
macro_rules! roster {
    ($($row:ident : $what_it_is_for:literal),+ $(,)?) => {
        $crate::roster!(@weld the_header_roster_names_every_row_in_this_file, $($row),+);
        // The prose column's ONLY consumer. It constrains the TYPE —
        // a column that is not a string literal is a compile error —
        // and reads nothing: the sentence stays unchecked against the
        // row, and no line here will ever check it.
        const _: &[&str] = &[$($what_it_is_for),+];
    };
    (@weld $guard:ident, $($row:ident),+) => {
        /// This file's `roster!{}` block against libtest's own `--list`
        /// of this binary. Its one home — the re-exec, the comparison
        /// and what it does and does not weld — is
        /// `test_utils::roster`.
        #[test]
        fn $guard() {
            let rostered: &[&str] = &[$({
                let _: fn() = $row;
                ::core::stringify!($row)
            }),+];
            let violations = $crate::roster::roster_violations(
                ::core::module_path!(),
                ::core::stringify!($guard),
                rostered,
            );
            assert!(violations.is_empty(), "{}", violations.join("\n"));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::{listed_rows, row_prefix, rows_of_listing, violations_against};

    /// The listing a suite of this shape would produce: one module with
    /// two rows, a SIBLING WHOSE NAME EXTENDS IT, and a nested module.
    fn listing() -> Vec<String> {
        [
            "other::unrelated",
            "probes::alpha",
            "probes::beta",
            "probes::guard",
            "probes::inner::nested",
            "probes_extended::gamma",
        ]
        .iter()
        .map(|s| (*s).to_owned())
        .collect()
    }

    fn against(rostered: &[&str]) -> String {
        violations_against(&listing(), "all::probes", "guard", rostered).join("\n")
    }

    /// The prefix keeps the `::`, which is what stops
    /// `probes_extended::gamma` being read as a row of `probes`.
    #[test]
    fn the_row_prefix_strips_the_crate_root_and_keeps_the_separator() {
        assert_eq!(row_prefix("all::r2_m10_6_probes"), "r2_m10_6_probes::");
        assert_eq!(row_prefix("all::outer::inner"), "outer::inner::");
        // A file that is its own `[[test]]` crate root: no prefix, and
        // every row of that binary is its own.
        assert_eq!(row_prefix("standalone_suite"), "");
    }

    /// The set compared is the module's DIRECT rows: not the neighbour
    /// whose name extends it, not the generated guard, and not a row
    /// that belongs to some other module of the same binary.
    #[test]
    fn the_compared_set_excludes_the_prefix_sibling_and_the_guard() {
        // `inner::nested` is under this prefix, so it reds on its own
        // account — the flat listing is what isolates the exclusions.
        let flat = [
            "probes::alpha",
            "probes::beta",
            "probes::guard",
            "other::unrelated",
        ]
        .iter()
        .map(|s| (*s).to_owned())
        .collect::<Vec<_>>();
        let against_flat = |rostered: &[&str]| {
            violations_against(&flat, "all::probes", "guard", rostered).join("\n")
        };
        assert!(against_flat(&["alpha", "beta"]).is_empty());
        for intruder in ["gamma", "guard", "unrelated"] {
            let v = against_flat(&["alpha", "beta", intruder]);
            assert!(v.contains(intruder), "rostering {intruder} must red: {v}");
        }
    }

    /// A `#[test]` under a nested `mod` is a VIOLATION, naming the row
    /// and saying what to do — the guard's name claims every row in the
    /// file and this is what makes that claim true.
    #[test]
    fn a_row_under_a_nested_mod_is_a_violation_no_roster_can_answer() {
        let v = against(&["alpha", "beta"]);
        assert!(v.contains("inner::nested"), "{v}");
        assert!(v.contains("NESTED"), "{v}");
        assert!(v.contains("Hoist"), "{v}");
        // And rostering it does not help: an ident cannot name a path,
        // so the entry lands in `absent` while the row still reds.
        let v = against(&["alpha", "beta", "nested"]);
        assert!(v.contains("inner::nested"), "{v}");
        assert!(v.contains("A roster names rows, not"), "{v}");
    }

    #[test]
    fn an_unrostered_row_is_named() {
        let v = against(&["alpha"]);
        assert!(v.contains("\"beta\""), "{v}");
        assert!(v.contains("NOT in its roster"), "{v}");
    }

    #[test]
    fn a_rostered_name_that_is_no_row_of_this_module_is_named() {
        let v = against(&["alpha", "beta", "a_helper_that_happens_to_be_fn"]);
        assert!(v.contains("a_helper_that_happens_to_be_fn"), "{v}");
        assert!(v.contains("A roster names rows, not"), "{v}");
    }

    /// The floor, at its REACHABLE trigger: the module's rows are all
    /// gone. In a real build both sides of the prefix come from the
    /// compiler, so a mis-derived prefix is near-unreachable; what
    /// happens is that the rows are deleted, or moved into a `mod`
    /// nested inside the file, and neither is left. The floor
    /// fires AND the rostered names that are gone are still named,
    /// which is the report the author needs.
    #[test]
    fn a_module_with_no_direct_rows_left_is_a_violation_not_a_green() {
        let emptied = [
            "other::unrelated",
            "probes::guard",
            "probes_extended::gamma",
        ]
        .iter()
        .map(|s| (*s).to_owned())
        .collect::<Vec<_>>();
        let v = violations_against(&emptied, "all::probes", "guard", &["alpha", "beta"]).join("\n");
        assert!(v.contains("compared itself against nothing"), "{v}");
        assert!(v.contains("every row this module had is gone"), "{v}");
        // And the rostered names that are gone: the floor reports
        // them rather than being the only thing reported.
        assert!(v.contains("\"alpha\""), "{v}");
        assert!(v.contains("\"beta\""), "{v}");
        assert!(v.contains("A roster names rows, not"), "{v}");
    }

    /// The other, near-unreachable half: the prefix itself is wrong.
    /// Same floor, and the message names the path it derived.
    #[test]
    fn a_prefix_that_matches_nothing_names_the_module_path_it_derived() {
        let v = violations_against(&listing(), "all::no_such_module", "guard", &["anything"])
            .join("\n");
        assert!(v.contains("compared itself against nothing"), "{v}");
        assert!(v.contains("all::no_such_module"), "{v}");
    }

    /// The re-exec answers about the RUNNING binary — the half the
    /// synthetic listings above cannot check.
    #[test]
    fn the_binary_lists_this_very_row() {
        let rows = listed_rows();
        assert!(
            rows.iter()
                .any(|r| r.ends_with("roster::tests::the_binary_lists_this_very_row")),
            "the listing must contain the row asking for it: {rows:?}"
        );
    }

    /// The parse: every line whose kind is not `test` is dropped, and
    /// so is the trailing summary. The running binary cannot check
    /// this — no `#[bench]` exists in this tree, so the filter would be
    /// exercised by nothing — and a supplied listing can.
    #[test]
    fn the_listing_parse_keeps_test_rows_and_drops_every_other_kind() {
        let stdout = "probes::alpha: test\n\
                      probes::measure_it: benchmark\n\
                      probes::beta: test\n\
                      \n\
                      2 tests, 1 benchmark\n";
        assert_eq!(rows_of_listing(stdout), ["probes::alpha", "probes::beta"]);
    }
}
