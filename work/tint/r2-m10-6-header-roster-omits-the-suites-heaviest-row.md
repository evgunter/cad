---
id: r2-m10-6-header-roster-omits-the-suites-heaviest-row
kind: issue
title: A test file's own roster lists five of seven rows, omits the heaviest, and names one in the opposite sense
status: open
opened: 2026-09-12
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
