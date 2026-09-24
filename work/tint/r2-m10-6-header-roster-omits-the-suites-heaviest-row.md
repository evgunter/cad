---
id: r2-m10-6-header-roster-omits-the-suites-heaviest-row
kind: unit
title: A test file's own roster lists five of seven rows, omits the heaviest, and names one in the opposite sense
status: closed
opened: 2026-09-12
branch: tint/4-roster-weld
pr: 2687
closed: 2026-09-15
---


Filed 2026-09-12 by the S-TCOST orchestrator out of the style review of
S-TCOST PR 2437, then **re-counted from the file** — the review said
eight rows; there are seven. Pre-existing; PR 2437 did not touch this
file, it quoted its doc comment as evidence and never opened the header.

## The roster

`crates/editor-core/tests/r2_m10_6_probes_interval.rs` opens with
*"What each row is for:"* and enumerates **five**. The file carries
**seven** `#[test]` rows:

| row | in the roster |
|---|---|
| `the_certifying_filter_changes_a_pre_m10_6_documents_drive` | yes |
| `min_separation_brackets_a_curved_pair_at_every_budget` | yes |
| `min_clearance_between_two_separated_bodies_reads_zero` | yes |
| `report_key_tells_the_dials_that_move_a_report_apart` | **named wrongly** |
| `the_mc_stream_is_re_derived_bit_for_bit` | yes |
| `the_typed_absence_names_its_verb_scalar_and_door` | **absent** |
| `a_tolerance_study_end_to_end_through_the_public_doors` | **absent** |

## Two defects, and the second is the one with teeth

**1. The roster's entry 4 names a test that does not exist, in the
opposite sense to the one that does.** It reads
`report_key_is_blind_to_the_dials_that_move_a_report` and describes the
key as blind — *"two MC reports that differ share one key"*. The actual
row is `report_key_tells_the_dials_that_move_a_report_apart`, which
asserts the opposite: the key **distinguishes** them. A reader trusting
the header comes away believing the suite pins a cache collision it in
fact pins the absence of.

Which of the two is the intended behaviour is not this row's call — it is
M10's design. What is certain is that the file's header and its code
disagree, and that only one of them is executable.

**2. The absent row is the most expensive test in the repository.**
`a_tolerance_study_end_to_end_through_the_public_doors` is, on S-TCOST's
hosted measurement of 2026-09-12, **346-660 s on its own — 85-96 % of
the CI leg that finishes last on every run read**, at every shard count.
It is the whole critical path of the interval ε = 1e-12 lane, and the
file that is supposed to say what each row is for does not mention it.

That is why this is S-TINT's and not a tidy-up: the roster is the only
document of what this suite covers, it is hand-kept, nothing reads it,
and the thing it omits is the single most consequential row in the
tree. `work/tint/plan.md` names the family — *censuses, markers and
hand-kept enumerations that pass on a substring, a term that matches
nothing, or a list nobody updated*.

## What this asks for

Complete the roster, fix entry 4's name, and say which sense is
intended (an M10 question if it is not obvious from the assertion).
Then the standing question for the class: **a roster in a doc comment
is a list nobody updated by construction.** Either it earns a keeper —
a row asserting the header names every `#[test]` in the file, which is
a dozen lines — or it should not be a roster at all.

## The class

Every `//! What each row is for:` header, and every doc-comment
enumeration of a file's own tests. Nothing in the tree checks one
against its file. The census has not been run; the pattern to run it
with is a grep for such headers followed by a count of `#[test]` in the
same file, which is mechanical enough to be a gate if the class turns
out to be large.

## Re-derived (2026-09-15, lane C)

**VERDICT: REPRODUCES** — both defects are live and untouched, and the
class census the row asks for has now been run.

**The file.** `crates/editor-core/tests/r2_m10_6_probes_interval.rs`
still opens *"What each row is for:"* with a numbered list of **five**,
and still carries **seven** `#[test]` rows. Re-derived by name (all seven
`fn` names read out of the file, the roster's five read out of the
header):

| row | in the roster |
| --- | --- |
| `the_certifying_filter_changes_a_pre_m10_6_documents_drive` | yes (1) |
| `min_separation_brackets_a_curved_pair_at_every_budget` | yes (2) |
| `min_clearance_between_two_separated_bodies_reads_zero` | yes (3) |
| `report_key_tells_the_dials_that_move_a_report_apart` | **named wrongly** (4) |
| `the_mc_stream_is_re_derived_bit_for_bit` | yes (5) |
| `the_typed_absence_names_its_verb_scalar_and_door` | **absent** |
| `a_tolerance_study_end_to_end_through_the_public_doors` | **absent** |

**Defect 1 is verbatim as filed.** Entry 4 still reads
`report_key_is_blind_to_the_dials_that_move_a_report` and still describes
the key as blind — *"two MC reports that differ share one key"* — while
the executable row is `report_key_tells_the_dials_that_move_a_report_apart`.
`grep -rl "fn report_key_is_blind_to_the_dials_that_move_a_report"` over
the tree returns **nothing**: the name the header gives exists nowhere, in
this file or any other, so it is not a cross-reference that drifted.

**Defect 2 is verbatim as filed.** `a_tolerance_study_end_to_end_through_the_public_doors`
is still in the file and still not in the roster. Its cost was not
re-measured here — this lane runs nothing — so the 346-660 s figure stands
as S-TCOST's 2026-09-12 measurement and is the one claim in this row that
a re-read cannot confirm or refute.

### The class census, run (the row says it had not been)

Pattern: over every `.rs` file in `crates/`, `tools/`, `demos/` and
`benches/` with a `//!` header and two or more `#[test]` rows, collect
every backticked snake_case identifier in the header, intersect with the
file's own `fn` names, and report the files where the header names three
or more of its own rows. That is the "roster" shape; 22 files match.

**The false-name defect is a singleton.** Exactly one of the 22 carries a
header backtick that names no `fn` in the file AND no `fn` anywhere in
the tree: this one. Two other files carry header names with no `fn`
behind them, and both are archaeology rather than rosters —
`crates/step-export/tests/m5_pr13_curved.rs` (*"`CENSUS` succeeds
`no_body_at_rest_carries_a_nurbs_carrier_or_face`, …"*, three retired
names) and `crates/topo/tests/review_mate4a_r2_probes.rs`
(`pm_census_ee_parallel`, named as a source of escalations). Neither
misdescribes what the file asserts; both are the
`docs/prompts/implementer-discipline.md` §4 comment-history shape, and
are reported to the orchestrator rather than added here.

**The incompleteness defect is NOT a singleton, but this is the only
file where a roster CLAIMS completeness.** Eleven of the 22 name every
`#[test]` in their file; ten name a strict subset (worst:
`crates/geom-brep/tests/m5_pr7_ssi.rs`, 3 of 27, and
`crates/editor-core/tests/m10_3_driver_interval.rs`, 3 of 23). Those ten
headers do not open with *"What each row is for:"* or any equivalent
universal, so they are illustrations rather than rosters and the §5
scope-sentence failure does not apply to them. **The numbered
"What each row is for:" shape has three files in the tree** —
`crates/topo/src/review_d18_probes.rs`,
`crates/sweep/tests/verbs_tubewall_r2_probes.rs` and this one — and only
this one enumerates rows numerically. `verbs_tubewall_r2_probes.rs`
explicitly declines to (*"What each row is for is written on the row"*),
which is the cheap honest answer this row's `## What this asks for`
proposes.

**Blind spot of that census**: it keys on backticked identifiers, so a
roster that names its rows in prose without backticks, or by a
human-readable paraphrase (which is exactly what
`crates/viewer/tests/chrome_labels.rs`'s skip marker does), is invisible
to it; and it requires three or more named rows, so a two-row roster with
one wrong name would not surface.

**Recommendation (orchestrator's call).** Keep open. The class is small
enough that the "does it earn a keeper" question can be answered no: one
file has the defect, and the fix is to complete the roster, correct entry
4's name and sense (an M10 question), and add
`a_tolerance_study_end_to_end_through_the_public_doors` with its cost
stated. A mechanical keeper over three files is not worth its own row.

## Closed by TINT-4 (PR #2687, `2101cb36a` on main, 2026-09-15)

Both defects the title names are gone, and the mechanism that removed
them is `test_utils::roster!` — the header enumeration is now a block of
the file's own row **idents**, each feeding `let _: fn() = $row;` (a
retired name is `error[E0425]`), `stringify!($row)` (the compared string
cannot be mistyped) and the text a human reads, compared against
libtest's own `--list --format=terse` through a `current_exe()` re-exec.
No Rust is parsed, so it is not an instance of
`source-scanning-censuses-are-a-tripwire-on-ordinary-rust`.

**The omitted row is in, and the reversed one was reversed the right
way round.** The review verified the sense independently: the row
asserts `assert_ne!(key(&mc_a), key(&mc_b), …)` and
`crates/editor-core/src/report.rs`'s own "why the dials are in it" block
agrees, so the lane corrected a wrong header rather than reversing a
kernel claim.

**What it does NOT enforce, which is the half this row should be
remembered for.** The weld holds NAMES and never PROSE. Each entry
carries a sentence, and `$what_it_is_for:literal` is matched and never
expanded — nothing computes with it. The fix pass's class check over
those seven sentences found **four wrong or misplaced on arrival**: a
citation to a deviation that did not put the dials in (D11 does), an
entry naming a `Violated` the row never asserts, one overclaiming, and
one correctly restating figures whose own item says they are stated
nowhere else — making this file a fifth site of a measurement kept
deliberately to one. The column is now constrained to be a string
(`const _: &[&str]`, so `row: 42` is `E0308`) and labelled as reading
nothing, but **a wrong sentence beside a right name still passes.**

Also not enforced: nothing requires a file to HAVE a roster, and the
weld says nothing about other files' rows, `#[bench]` or no-harness
targets. A `#[test]` under a nested `mod` is a **violation** rather than
a silent exemption — the fix pass took that shape so the guard's name,
which is what CI prints, would be true.
