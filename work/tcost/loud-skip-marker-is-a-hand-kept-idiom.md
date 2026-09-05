---
id: loud-skip-marker-is-a-hand-kept-idiom
kind: issue
title: The loud-skip marker is a hand-kept idiom in eight files, each copy naming its rows by hand and admitting it goes stale silently
status: open
opened: 2026-09-04
refs: [1776]
---


## Finding

**The `#[cfg]`-gated "this lane is not built here" marker row is now an
eight-copy hand-written idiom**, and every copy admits in its own
rustdoc that it goes stale silently.

The copies (PR 1776 added the last of them):

| file:line | row |
| --- | --- |
| `crates/viewer/src/lib.rs:103` | `app_lane_skipped_no_chrome_or_gpu_coverage_here` |
| `crates/viewer/tests/chrome_labels.rs:30` | `app_lane_skipped_no_chrome_coverage_here` |
| `crates/viewer/tests/error_display.rs:307` | `app_lane_skipped_startup_error_arms_not_checked_here` |
| `crates/viewer/tests/panel_display.rs:770` | `app_lane_skipped_parameter_field_units_not_checked_here` |
| `crates/sweep/tests/m5_s12_curved_ops_interval.rs:27` | `interval_lane_skipped_no_certified_coverage_here` |
| `crates/sweep/tests/m5_s13_pips_interval.rs:19` | same name |
| `crates/sweep/tests/m6_surgery_interval.rs:20` | same name |
| `crates/topo/tests/m6_2_fitted_at_rest.rs:188` | same name |

Each is a `#[cfg(not(feature = …))] #[test] fn` whose entire body is a
`println!` naming, BY HAND, the rows that did not compile in this
build. Each also carries the same paragraph saying so —
`crates/viewer/src/lib.rs:92-100` is the fullest statement: *"Nothing
here goes red if the modules it names start running, stop existing, or
grow a sibling — the enumeration above is kept by hand, and a marker
that silently went stale would look exactly like this one."*

That is a defect with a known cost: a ninth `app`-gated row added to
`panel_display.rs` leaves its marker quietly wrong, and nothing
anywhere notices.

## What is already settled, so a taker does not redo it

The tree HAS a stand-down door — `test_utils::vacuity::stood_down`
(`crates/test-utils/src/vacuity.rs:201-203`) — and its module docs
**deliberately exclude this idiom**: *"The four whole-binary
`interval_lane_skipped_no_certified_coverage_here` rows are a different
idiom and deliberately not converted: their entire body is the
announcement"* (`vacuity.rs:73-76`; the exclusion is restated at
`crates/step-import/tests/cert5_r1_import_probes.rs:252-258`). The
distinction is real: `stood_down` announces from INSIDE a row that ran
and could not enter its mode; these rows exist only because their
siblings did not compile, so the condition is a `#[cfg]` and there is
no running row to announce from.

So the question is not "route these through `stood_down`". It is
whether the enumeration can stop being hand-kept. Two shapes worth
weighing, neither adjudicated:

1. **A macro** that takes the feature and the row names and emits both
   the marker and — under the feature — nothing, so the list has one
   spelling per file. Does not fix staleness; does fix the copying.
2. **Drop the enumeration.** The rows' names are recoverable from the
   source by anything that can read a `#[cfg]`, and the marker's stated
   payload is "a reader of a default-feature run meets the absence
   instead of inferring it" — which a marker naming only the FEATURE
   and the file delivers, with nothing left to go stale. This is
   probably the answer, and it is a decision about what the marker is
   FOR rather than a refactor.

Whichever is taken, `memories/test-suite-cost.md` names the interval
rows and would need the same edit.

## Home

`work/issues/` — the copies span `crates/viewer`, `crates/sweep` and
`crates/topo`, so no one program's `paths` cover them; the shared door,
if there is to be one, lands in `crates/test-utils`.

Found by the style review of PR 1776 (CHROME unit 8). The reviewer
counted seven; the eighth is `crates/sweep/tests/m5_s13_pips_interval.rs`.
This file was first filed under an id naming the wrong count, which is
the idiom's own defect one level up — an id is a hand-kept enumeration
too. It is named for the shape now rather than for a number.

**A grep for the idiom over-matches**, so a taker does not chase the
same three: `crates/step-import/tests/freecad.rs` (a RUNTIME skip on
the ambient epsilon, printed from a row that ran),
`crates/sweep/tests/m5_pr9_cosurface_merge.rs` and
`crates/topo/src/merge_faces.rs` (the merge driver's own "loud skip"
record, a domain term for a refused cosurface run) all carry the words
and none is this shape. The eight in the table are the
`#[cfg(not(feature = …))] #[test]` marker rows.

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/tcost/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.
