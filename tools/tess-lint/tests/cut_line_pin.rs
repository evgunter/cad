//! **The cut line's two halves, pinned in one direction.**
//!
//! The provenance line above a sweep is written and validated by
//! `scripts/tess_budget_cut.sh` and read by this crate, in two
//! languages across two roots. The script said so itself: *"This is
//! the shell's reading of the format `tess_lint::split_cut` parses,
//! and the two are pinned by nothing: there is no cross-language gate
//! here."* This file is that gate, built entirely on this side —
//! `scripts/` belongs to another program's fence, so nothing here
//! edits it and everything here reads it.
//!
//! Two claims, and they are different claims:
//!
//! * **The spelling.** Every place that script spells the prefix is
//!   [`tess_lint::CUT_PREFIX`] — its validator's anchor, its writer's
//!   `echo`, its strip, and its prose. The `include_str!` makes the
//!   PATH load-bearing; the assertions make the STRING load-bearing.
//! * **The shape.** The script's `CUT_RE` constrains what follows the
//!   prefix; this crate's parse does too, and the two constraints are
//!   held against each other over a table of candidate lines, with
//!   the script's own regex — extracted from its text, run by `grep
//!   -E` — as the oracle for its half.
//!
//! **The direction that matters, and why the table is two-sided
//! anyway.** A line this crate reads as a cut but the script does not
//! recognise as a stamp is the dangerous one: the script's first arm
//! would then re-stamp a file that already carries a cut, walking the
//! record forward past the rows it describes — the exact inversion
//! that arm exists to prevent. So that containment is what the reader
//! was tightened to satisfy. The other direction is pinned too, as a
//! truth table rather than an implication, because a disagreement
//! introduced from either side should be visible on the side that can
//! see both — and one such disagreement exists today: see
//! [`TRAILING_TEXT`].
//!
//! **Which clause routes this.** `tools/README.md`'s `CC1` puts the
//! check at the reading boundary, and `split_cut` is that boundary
//! for the provenance line — `parse` and `cut` both reach it and
//! nothing else parses the line. `CC5` decides that a check is owed
//! and in which voice: an admission refuses what the producer could
//! not have written, and every bound tightened here (lowercase hex,
//! at most a whole object name, one space) is a spelling
//! `tess_budget_cut.sh` cannot emit. The voice is the harness voice,
//! because what a malformed cut breaks is the reading of the file
//! rather than a measurement.
//!
//! **`CC1`–`CC5` are stated over CROSS-COLUMN readings and this is
//! not one.** The provenance line is not a column. What transfers is
//! `CC5`'s test — which defers, in its own words, to the one
//! `tess_lint::Report` states — and that test is general enough to
//! decide this. The clause ids are cited for the test and the site
//! they actually give, not as a claim that a cut line is a column.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::io::Write;
use std::process::{Command, Stdio};

use tess_lint::{CUT_PREFIX, Cut, cut};

/// The script that writes, validates and strips the line this crate
/// reads. Read as text and never edited: it is `work/ciw`'s file.
const SCRIPT: &str = include_str!("../../../scripts/tess_budget_cut.sh");

/// The concept word, without the punctuation [`CUT_PREFIX`] wraps it
/// in. Every occurrence of THIS in the script must be an occurrence
/// of that.
const STEM: &str = "tess-budget-cut";

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
/// regex is an edit to `scripts/`. Filed as
/// `work/meter/cut-regex-unanchored-admits-a-line-the-lint-refuses`.
/// When that lands, this row's expectation flips to `false` and this
/// doc comment goes.
const TRAILING_TEXT: &str = "# tess-budget-cut: 1a2b3c4 2026-08-30 extra";

/// `(line, this crate reads it as a cut, the script recognises it as
/// a stamp, what the case is)`.
///
/// Written out rather than generated: each row is a boundary someone
/// chose, and a generator would have to re-spell the very shapes
/// being pinned.
const TABLE: &[(&str, bool, bool, &str)] = &[
    (
        "# tess-budget-cut: 1a2b3c4 2026-08-30",
        true,
        true,
        "the shortest abbreviation either half admits",
    ),
    (
        "# tess-budget-cut: aba2625f8f84 2026-09-04T03:16:35-07:00",
        true,
        true,
        "the shape the committed baseline carries",
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
    (
        TRAILING_TEXT,
        false,
        true,
        "a well-formed cut with junk after it",
    ),
];

/// The physical line `byte` falls on, 1-based, for a failure message
/// a reader can open the script at.
fn line_of(byte: usize) -> usize {
    SCRIPT[..byte].lines().count().max(1)
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

/// Every place the script spells the prefix spells it the way this
/// crate does.
///
/// The three sites the finding named are located individually rather
/// than only counted, so each reds on its own: a validator whose
/// anchor drifts, a writer whose `echo` drifts, and a strip whose
/// pattern drifts are three different breakages and produce three
/// different failures. The sweep over [`STEM`] is what covers the
/// fourth site nobody has written yet.
#[test]
fn every_spelling_of_the_cut_prefix_in_the_sweep_script_is_this_crates() {
    let re = cut_re();
    assert!(
        re.starts_with(&format!("^{CUT_PREFIX} ")),
        "the script's validator anchors on a different prefix: {re}"
    );
    assert_eq!(
        SCRIPT.matches(&format!("echo \"{CUT_PREFIX} ")).count(),
        1,
        "one `echo` writes the cut line, spelled as this crate reads it"
    );
    assert!(
        SCRIPT.contains(&format!("grep -v '^{CUT_PREFIX}'")),
        "the re-stamp strip does not match this crate's prefix"
    );

    let mut seen = 0;
    for (at, _) in SCRIPT.match_indices(STEM) {
        let from = at.checked_sub(2).expect("a prefix has room before it");
        let span = SCRIPT
            .get(from..at + STEM.len() + 1)
            .expect("a prefix has room after it");
        assert_eq!(
            span,
            CUT_PREFIX,
            "scripts/tess_budget_cut.sh:{}: `{span}` is not this crate's prefix",
            line_of(at)
        );
        seen += 1;
    }
    assert!(seen >= 4, "the script names the cut line {seen} times");
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
        assert_eq!(reads_as_cut(line), reads, "this crate on {case}: {line}");
        assert_eq!(
            script_recognises(&re, line),
            recognises,
            "the script's CUT_RE on {case}: {line}"
        );
        assert!(
            !(reads && !recognises),
            "{case}: this crate reads a cut the script would re-stamp over: {line}"
        );
    }
    assert!(
        TABLE.iter().any(|r| r.1) && TABLE.iter().any(|r| !r.1),
        "the table exercises both answers"
    );
}
