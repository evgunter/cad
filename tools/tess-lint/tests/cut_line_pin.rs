//! **The cut line's two halves, pinned in one direction.**
//!
//! The provenance line above a sweep is written and validated by
//! `scripts/tess_budget_cut.sh` and read by this crate, in two
//! languages across two roots, and until this file there was no gate
//! between them. This file is that gate, built entirely on this side:
//! `scripts/` belongs to another program's fence, so nothing here
//! edits it and everything here reads it.
//!
//! Two claims, and they are different claims:
//!
//! * **The spelling.** Every place that script EXECUTES the prefix is
//!   [`tess_lint::CUT_PREFIX`] — its validator's anchor, its writer's
//!   `echo`, its strip, and its selftest's fixture. The `include_str!`
//!   makes the PATH load-bearing; the assertions make the STRING
//!   load-bearing. Prose about the record is deliberately outside the
//!   sweep: see [`a_comment_about_the_cut_record_is_not_a_spelling`].
//! * **The shape.** The script's `CUT_RE` constrains what follows the
//!   prefix; this crate's parse does too, and the two constraints are
//!   held against each other over a table of candidate lines, with
//!   the script's own regex — extracted from its text, run by `grep
//!   -E` — as the oracle for its half.
//!
//! **The direction that matters, and why the table is two-sided
//! anyway.** A line this crate reads as a cut but the script does not
//! recognise as a stamp is the dangerous one: the script's
//! already-stamped test (`:71`) then misses, and the file falls into
//! the BACKFILL arm (`:82`), which re-stamps it from the commit that
//! last wrote it — by then the commit that wrote the stamp, a whole
//! commit newer than the rows. The record walks forward past the data
//! it describes, which is the exact inversion the refusal arm exists
//! to prevent. So that containment is what the reader was tightened to
//! satisfy, and it is asserted on the COMPUTED answers before either
//! is compared to the table. The other direction is pinned too, as a
//! truth table rather than an implication, because a disagreement
//! introduced from either side should be visible on the side that can
//! see both — and one such disagreement exists today: see
//! [`TRAILING_TEXT`].
//!
//! **Why a check is owed here, and in which voice.** The test is the
//! one [`tess_lint::Report`] states — an observation is a FINDING
//! where the gate would otherwise assert something false, and a
//! reading nobody could have produced is not a measurement the
//! instrument exists to report. Every bound tightened here (lowercase
//! hex, at most a whole object name, one space) is a spelling
//! `tess_budget_cut.sh` cannot emit, so refusing it costs the report
//! nothing it is for. The voice is the harness voice, because what a
//! malformed cut breaks is the reading of the file rather than a
//! measurement in it.
//!
//! `tools/README.md`'s `CC1`–`CC5` are the same rule one level over,
//! and are NOT cited as routing this: they are stated over
//! cross-COLUMN admissions, `CC1` names `tess_lint::parse` and
//! `k_lint::lint_csv` as the boundary rather than the private
//! `split_cut`, and `CC5` explicitly hands the general test back to
//! `Report`. The provenance line is not a column and this file does
//! not claim it is.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::io::Write;
use std::process::{Command, Stdio};

use tess_lint::{CUT_PREFIX, Cut, cut};

/// The script that writes, validates and strips the line this crate
/// reads. Read as text and never edited: it is `work/ciw`'s file.
const SCRIPT: &str = include_str!("../../../scripts/tess_budget_cut.sh");

/// The concept word, without the punctuation [`CUT_PREFIX`] wraps it
/// in. Every occurrence of THIS on a line of shell must be an
/// occurrence of that; see [`executable_spellings`] for what that
/// excludes and why.
const STEM: &str = "tess-budget-cut";

/// The committed baseline, read so that the row of [`TABLE`] standing
/// for "the shape a real sweep carries" is the real line rather than a
/// transcription of it.
const BASELINE: &str = include_str!("../../../docs/tess-budget-data/tess-budget-baseline.csv");

/// A plausible abbreviated object name and committer date, for
/// rendering the writer's own `echo`.
const COMMIT: &str = "1a2b3c4d5e6f";
const DATE: &str = "2026-08-30T12:00:00+00:00";

/// The one line in the table on which the two halves disagree.
///
/// `CUT_RE` is anchored at the start and not at the end, so it
/// matches a line with a well-formed cut and junk after it; this
/// crate refuses that line, because a third field is not something
/// the writer could have emitted. The disagreement is harmless in the
/// safe direction and it is not this lane's to close — anchoring the
/// regex is an edit to `scripts/`, which is `work/ciw`'s. Filed as
/// `work/ciw/cut-regex-unanchored-admits-a-line-the-lint-refuses`,
/// and [`TRAILING_CASE`] carries that id into the failure message so
/// that whoever anchors the regex is told by the alarm itself what it
/// is telling them.
const TRAILING_TEXT: &str = "# tess-budget-cut: 1a2b3c4 2026-08-30 extra";

/// What [`TRAILING_TEXT`]'s row prints when it reds, which is the
/// only place a reader who has just anchored `CUT_RE` will look.
///
/// A row that goes red because someone FIXED the defect it pins reads
/// as a regression unless it says otherwise, so it says otherwise
/// here rather than only in a doc comment in a cargo root outside the
/// workspace.
const TRAILING_CASE: &str = "a well-formed cut with junk after it — the one row where the two \
     halves disagree, pinning the defect and not the fix. If this just went red because \
     `CUT_RE` grew an end anchor, that is the fix: close \
     `work/ciw/cut-regex-unanchored-admits-a-line-the-lint-refuses`, set this row's \
     `recognises` to false and delete TRAILING_CASE";

/// `(line, this crate reads it as a cut, the script recognises it as
/// a stamp, what the case is)`.
///
/// Written out rather than generated: each row is a boundary someone
/// chose, and a generator would have to re-spell the very shapes
/// being pinned. The one row that is not a chosen boundary but a
/// datum — the committed baseline's own line — is held to the file by
/// [`the_committed_baselines_own_cut_line_is_a_row_of_the_table`].
///
/// **Five rows are also cases of `tess_lint`'s
/// `a_malformed_cut_line_is_harness_breakage_not_an_absent_cut`, and
/// the two assert different things about them.** There: that this
/// crate refuses them in the harness voice at line 1, which is a
/// property of this crate alone. Here: that
/// `scripts/tess_budget_cut.sh`'s own regex refuses them too, which
/// is the only claim about the two languages agreeing. Deleting
/// either leaves the other's question unasked.
const TABLE: &[(&str, bool, bool, &str)] = &[
    (
        "# tess-budget-cut: 1a2b3c4 2026-08-30",
        true,
        true,
        "the shortest abbreviation either half admits",
    ),
    (
        "# tess-budget-cut: 3f55f361b22e 2026-09-08T04:03:35+00:00",
        true,
        true,
        "the shape the committed baseline carries, read from it below",
    ),
    (
        "# tess-budget-cut: 1a2b3c4d5e6f-dirty 2026-08-30T12:00:00+00:00",
        true,
        true,
        "the dirty marker, which a hosted sweep always carries",
    ),
    (
        "# tess-budget-cut: 0123456789abcdef0123456789abcdef01234567 2026-08-30",
        true,
        true,
        "a whole object name, the longest either half admits",
    ),
    (
        "# tess-budget-cut: 0123456789abcdef0123456789abcdef012345678 2026-08-30",
        false,
        false,
        "one character longer than an object name",
    ),
    (
        "# tess-budget-cut: 1A2B3C4 2026-08-30",
        false,
        false,
        "uppercase hex, which git never abbreviates to",
    ),
    (
        "# tess-budget-cut: 1a2b3c 2026-08-30",
        false,
        false,
        "one character shorter than the floor",
    ),
    (
        "# tess-budget-cut: not-hex 2026-08-30",
        false,
        false,
        "not an object name at all",
    ),
    ("# tess-budget-cut: 1a2b3c4d5e6f", false, false, "no date"),
    (
        "# tess-budget-cut: 1a2b3c4d5e6f yesterday",
        false,
        false,
        "a date that is not a calendar day",
    ),
    (
        "# tess-budget-cut:1a2b3c4 2026-08-30",
        false,
        false,
        "no space after the prefix",
    ),
    (
        "# tess-budget-cut:  1a2b3c4 2026-08-30",
        false,
        false,
        "two spaces where the writer emits one",
    ),
    (
        "# swept at 1a2b3c4 2026-08-30",
        false,
        false,
        "a comment line that is not a cut record",
    ),
    (TRAILING_TEXT, false, true, TRAILING_CASE),
];

/// The physical line `byte` falls on, 1-based, for a failure message
/// a reader can open the script at.
fn line_of(byte: usize) -> usize {
    SCRIPT[..byte].lines().count().max(1)
}

/// What a `scripts/` reader is told when this file reds, appended to
/// every message the sweep can produce.
///
/// The suite lives in a cargo root OUTSIDE the workspace, so a
/// `scripts/`-only change does not run it locally and meets it first
/// as a CI failure. A pin whose alarm names neither the rule nor the
/// remedy is a pin that gets deleted by whoever it ambushes.
const RULE: &str = "\n  tools/tess-lint reads this line: every EXECUTABLE mention of the cut \
     record in scripts/tess_budget_cut.sh must be spelled exactly as \
     tess_lint::CUT_PREFIX. Prose about the record is not swept and never reds. \
     To change the prefix itself, change the constant and this script together; \
     to run this suite: cargo test --manifest-path tools/tess-lint/Cargo.toml";

/// Every occurrence of [`STEM`] in `text` that is on a line of shell,
/// as `(byte offset, the two-character span before it and one after)`.
///
/// **Comment lines are excluded, deliberately.**
/// `scripts/tess_budget_cut.sh` is `work/ciw`'s file, and a lane there
/// writing an ordinary sentence about the record — *"the
/// tess-budget-cut record is described above"* — must not red a suite
/// in another cargo root over a spelling nothing executes. What the
/// sweep is for is the spellings that DECIDE something: the
/// validator's anchor, the writer's `echo`, the re-stamp strip, and
/// the selftest's malformed fixture.
///
/// **Its blind spot, stated:** a comment is recognised by the line's
/// first non-blank character, so a trailing comment on a line of shell
/// (`foo # the tess-budget-cut record`) is still swept. Recognising
/// that case needs a shell lexer over quoting, which is more machinery
/// than the hazard is worth; [`RULE`] is what covers it, by telling
/// that reader what happened.
fn executable_spellings(text: &str) -> Vec<(usize, &str)> {
    let mut out = Vec::new();
    for (at, _) in text.match_indices(STEM) {
        let line_start = text[..at].rfind('\n').map_or(0, |n| n + 1);
        if text[line_start..at].trim_start().starts_with('#') {
            continue;
        }
        let from = at.checked_sub(2).expect("a prefix has room before it");
        let span = text
            .get(from..at + STEM.len() + 1)
            .expect("a prefix has room after it");
        out.push((at, span));
    }
    out
}

/// The script's validator, extracted from its own text so that the
/// regex is what gets run rather than a copy of it.
fn cut_re() -> String {
    let key = "\nCUT_RE='";
    assert_eq!(
        SCRIPT.matches(key).count(),
        1,
        "one `CUT_RE=` assignment in scripts/tess_budget_cut.sh"
    );
    let at = SCRIPT.find(key).unwrap() + key.len();
    let rest = &SCRIPT[at..];
    let end = rest
        .find('\'')
        .expect("the CUT_RE assignment closes its quote");
    rest[..end].to_string()
}

/// Whether the script's own validator matches `line`.
fn script_recognises(re: &str, line: &str) -> bool {
    let mut child = Command::new("grep")
        .arg("-Eq")
        .arg(re)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("grep is what the script itself validates with");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(format!("{line}\n").as_bytes())
        .unwrap();
    let out = child.wait_with_output().unwrap();
    match out.status.code() {
        Some(0) => true,
        Some(1) => false,
        other => panic!(
            "grep -E {re:?} answered {other:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        ),
    }
}

/// Whether this crate reads `line` as a recorded cut.
fn reads_as_cut(line: &str) -> bool {
    matches!(cut(&format!("{line}\nscene,face\n")), Ok(Some(_)))
}

/// The count of executable spellings in the script today.
///
/// Three of them are located by name below, so each drifts into its
/// own failure; the fourth — the selftest's malformed fixture — is
/// covered by the sweep alone, and a floor equal to the live count is
/// what stops that one going missing unnoticed. It is a FLOOR and not
/// an equality: a fifth executable spelling is swept like the others
/// and does not need this number moved.
const EXECUTABLE_SPELLINGS: usize = 4;

/// Every place the script EXECUTES the prefix spells it the way this
/// crate does.
///
/// The three sites the finding named are located individually rather
/// than only counted, so each reds on its own: a validator whose
/// anchor drifts, a writer whose `echo` drifts, and a strip whose
/// pattern drifts are three different breakages and produce three
/// different failures. The sweep over [`STEM`] is what covers the
/// selftest's fixture and any fifth site nobody has written yet —
/// over shell only, for the reason [`executable_spellings`] gives.
#[test]
fn every_spelling_of_the_cut_prefix_in_the_sweep_script_is_this_crates() {
    let re = cut_re();
    assert!(
        re.starts_with(&format!("^{CUT_PREFIX} ")),
        "the script's validator anchors on a different prefix: {re}{RULE}"
    );
    assert_eq!(
        SCRIPT.matches(&format!("echo \"{CUT_PREFIX} ")).count(),
        1,
        "one `echo` writes the cut line, spelled as this crate reads it{RULE}"
    );
    assert!(
        SCRIPT.contains(&format!("grep -v '^{CUT_PREFIX}'")),
        "the re-stamp strip does not match this crate's prefix{RULE}"
    );

    let sites = executable_spellings(SCRIPT);
    for &(at, span) in &sites {
        assert_eq!(
            span,
            CUT_PREFIX,
            "scripts/tess_budget_cut.sh:{}: `{span}` is not `{CUT_PREFIX}`{RULE}",
            line_of(at)
        );
    }
    assert!(
        sites.len() >= EXECUTABLE_SPELLINGS,
        "the script executes the cut record {} times, and {EXECUTABLE_SPELLINGS} of those \
         spellings are what this pin is over — one has gone missing{RULE}",
        sites.len()
    );
}

/// A sentence ABOUT the cut record is not a spelling of it.
///
/// The decoy is the script's own text plus the comment a `work/ciw`
/// lane would plausibly write, and the fixture proves the sweep reads
/// shell rather than prose about it: the mention is really there, and
/// the sweep really declines it. Without this row the pin is hostage
/// to another program's comments — an ordinary sentence in
/// `scripts/tess_budget_cut.sh` would red a suite in a cargo root
/// outside the workspace, naming neither the rule nor the fix.
#[test]
fn a_comment_about_the_cut_record_is_not_a_spelling() {
    let decoyed = format!("{SCRIPT}\n# NOTE: the {STEM} record is described above.\n");
    assert_eq!(
        decoyed.matches(STEM).count(),
        SCRIPT.matches(STEM).count() + 1,
        "the decoy does not add a mention, so it proves nothing"
    );
    assert_eq!(
        executable_spellings(&decoyed).len(),
        executable_spellings(SCRIPT).len(),
        "prose about the record was swept as a spelling of it"
    );
}

/// The line the script's `echo` actually writes is one this crate
/// reads, with the fields in the order it expects.
///
/// Rendered from the script's own text: the `echo`'s field list is
/// what is under test, so a fourth field, a reordering or a changed
/// separator reds here even though every spelling of the prefix would
/// still be right.
#[test]
fn the_line_the_cut_script_writes_is_one_this_crate_reads() {
    let key = format!("echo \"{CUT_PREFIX}");
    // `find` takes the FIRST match, so "first" has to be made to mean
    // "only" here rather than in another `#[test]` that this one
    // cannot see the result of.
    assert_eq!(
        SCRIPT.matches(&key).count(),
        1,
        "one `echo` writes the cut line{RULE}"
    );
    let at = SCRIPT.find(&key).expect("the writer's echo") + "echo \"".len();
    let rest = &SCRIPT[at..];
    let end = rest.find('"').expect("the echo closes its quote");
    let written = &rest[..end];
    assert!(
        written.contains("$commit") && written.contains("$date"),
        "the writer no longer interpolates a commit and a date: {written}"
    );
    let line = written.replace("$commit", COMMIT).replace("$date", DATE);
    assert_eq!(
        cut(&format!("{line}\nscene,face\n")).unwrap(),
        Some(Cut {
            commit: COMMIT.into(),
            date: DATE.into()
        }),
        "this crate does not read the line the script writes: {line}"
    );
}

/// This crate's reading of a cut line and the script's validator
/// agree on every case in [`TABLE`], one documented disagreement
/// aside.
///
/// The containment that matters is `reads` implies `recognises`: the
/// other way round the script merely declines to refuse a re-stamp,
/// while this way round it re-stamps a file that already carries a
/// cut and dates the record after the rows.
#[test]
fn the_two_halves_agree_about_what_a_cut_line_is() {
    let re = cut_re();
    for &(line, reads, recognises, case) in TABLE {
        let reads_now = reads_as_cut(line);
        let recognises_now = script_recognises(&re, line);
        // The containment is over the COMPUTED answers and is asserted
        // FIRST, so that a real inversion — introduced from either
        // side — reds with what it costs rather than with a table
        // mismatch. Reading it off the table instead would make it a
        // statement about this file's own source text.
        assert!(
            !(reads_now && !recognises_now),
            "this crate reads a cut the script would re-stamp over, walking the record \
             past the rows it describes — {case}: {line}"
        );
        assert_eq!(reads_now, reads, "this crate on {case}: {line}");
        assert_eq!(
            recognises_now, recognises,
            "the script's CUT_RE on {case}: {line}"
        );
    }
}

/// The row standing for "the shape a real sweep carries" carries it.
///
/// [`TABLE`]'s second row is the committed baseline's own cut line, and
/// a transcription of it goes stale in silence at the next re-cut. It
/// is read from the file instead, which `baseline_census.rs` already
/// does one directory over.
#[test]
fn the_committed_baselines_own_cut_line_is_a_row_of_the_table() {
    let first = BASELINE.lines().next().expect("the baseline is not empty");
    assert!(
        TABLE
            .iter()
            .any(|&(l, reads, recognises, _)| { l == first && reads && recognises }),
        "the committed baseline's first line is not a row both halves admit: {first}"
    );
}
