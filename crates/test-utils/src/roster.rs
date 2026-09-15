//! **A file's header roster, welded to the rows it names.**
//! [`crate::roster!`] is the whole mechanism: a block of idents that
//! replaces a hand-kept `//!` enumeration of a file's own `#[test]`
//! rows, and a generated row that compares it against libtest's own
//! `--list`.
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
//! **Nothing requires a file to HAVE a roster.** A suite that grows a
//! header enumeration and never invokes the macro is silent, exactly as
//! it was before this module existed.
//!
//! **A roster covers its module's DIRECT rows only.** A `#[test]` in a
//! module nested inside the file lists as `<file>::<inner>::<row>` and
//! is neither demanded nor accepted, because a roster entry is an ident
//! and cannot name a path. The file this was written for has no nested
//! test module; one added later would be unrostered and unremarked.
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
/// prefix that matches nothing) are stated as listings in this module's
/// own rows rather than depending on what binary is running.
#[must_use]
fn violations_against(
    listed: &[String],
    module_path: &str,
    guard_row: &str,
    rostered: &[&str],
) -> Vec<String> {
    let prefix = row_prefix(module_path);
    let mine: Vec<&str> = listed
        .iter()
        .filter_map(|row| row.strip_prefix(&prefix))
        // Direct rows of this module only: a nested module's rows
        // list as `<inner>::<row>` and cannot be named by an ident.
        .filter(|row| !row.contains("::") && *row != guard_row)
        .collect();

    let mut violations = Vec::new();

    // The vacuity floor. A prefix that matches nothing is a guard that
    // cannot see its subject, and without this it would be the same
    // green as a roster that agrees.
    if mine.is_empty() {
        violations.push(format!(
            "no row of the {} rows in this binary is named `{prefix}<row>`, so this roster \
             compared itself against nothing. The module prefix is derived from \
             module_path!() = {module_path:?}.",
            listed.len()
        ));
        return violations;
    }

    let unrostered: Vec<&&str> = mine.iter().filter(|row| !rostered.contains(row)).collect();
    if !unrostered.is_empty() {
        violations.push(format!(
            "these #[test] rows are in this module and NOT in its roster!{{}} block: \
             {unrostered:?}. Add each one with a sentence saying what it is for."
        ));
    }

    let absent: Vec<&&str> = rostered
        .iter()
        .filter(|name| !mine.contains(name))
        .collect();
    if !absent.is_empty() {
        violations.push(format!(
            "the roster names {absent:?}, which compile as `fn()` in this module but are \
             not #[test] rows libtest lists under `{prefix}`. A roster names rows, not \
             helpers."
        ));
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
/// sense to what the row asserts, and nothing here will say so. See
/// this module's docs.
///
/// **Nothing requires a file to have a roster**, so a header
/// enumeration that never adopts this macro is silent; and a roster
/// covers the invoking module's DIRECT rows only, never those of a
/// module nested inside the file.
///
/// # What the generated row costs
///
/// One `--list` re-exec of the test binary. Measured at 5.2-5.8 ms on
/// `editor-core`'s aggregated `all` binary with `--features interval`
/// (450 MB, 1629 rows), which is the largest in the tree.
///
/// # Where the row's name comes from
///
/// The generated `fn` is named once, in the first rule below, and the
/// string the comparison excludes itself by is `stringify!` of that
/// same token — the copy this macro exists to end is not one this macro
/// makes.
#[macro_export]
macro_rules! roster {
    ($($row:ident : $what_it_is_for:literal),+ $(,)?) => {
        $crate::roster!(@weld the_header_roster_names_every_row_in_this_file, $($row),+);
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
    use super::{listed_rows, row_prefix, violations_against};

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
    /// whose name extends it, not a nested module's rows, not the
    /// generated guard.
    #[test]
    fn the_compared_set_excludes_the_prefix_sibling_the_nested_mod_and_the_guard() {
        assert!(against(&["alpha", "beta"]).is_empty());
        for intruder in ["gamma", "nested", "guard", "unrelated"] {
            let v = against(&["alpha", "beta", intruder]);
            assert!(v.contains(intruder), "rostering {intruder} must red: {v}");
        }
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

    /// The floor. Without it a derivation that matched nothing would be
    /// the same green as a roster that agreed.
    #[test]
    fn a_prefix_that_matches_nothing_is_a_violation_not_a_green() {
        let v = violations_against(&listing(), "all::no_such_module", "guard", &["anything"])
            .join("\n");
        assert!(v.contains("compared itself against nothing"), "{v}");
        assert!(v.contains("no_such_module"), "{v}");
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
        // The `: <kind>` suffix libtest prints is stripped, and only
        // `test` kinds survive.
        assert!(!rows.iter().any(|r| r.contains(": ")), "{rows:?}");
    }
}
