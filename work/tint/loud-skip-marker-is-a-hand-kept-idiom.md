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

Cited by `fn` name, not by line: the name IS the landing spot and is
one `grep -n` away in the named file, while three of the eight line
numbers this table used to carry no longer resolved — and one of those
three had been certified as resolving five days earlier (see the
re-derivation section at the foot).

| file | row (the `fn` name) |
| --- | --- |
| `crates/viewer/src/lib.rs` | `app_lane_skipped_no_chrome_or_gpu_coverage_here` — **no `fn` of that name exists; entry unresolved, see "Entry 1" below** |
| `crates/viewer/tests/chrome_labels.rs` | `app_lane_skipped_no_chrome_coverage_here` |
| `crates/viewer/tests/error_display.rs` | `app_lane_skipped_startup_error_arms_not_checked_here` |
| `crates/viewer/tests/panel_display.rs` | `app_lane_skipped_parameter_field_units_not_checked_here` |
| `crates/sweep/tests/m5_s12_curved_ops_interval.rs` | `interval_lane_skipped_no_certified_coverage_here` |
| `crates/sweep/tests/m5_s13_pips_interval.rs` | same name |
| `crates/sweep/tests/m6_surgery_interval.rs` | same name |
| `crates/topo/tests/m6_2_fitted_at_rest.rs` | same name |

Each is a `#[cfg(not(feature = …))] #[test] fn` whose entire body is a
`println!` naming, BY HAND, the rows that did not compile in this
build. Each also carries the same paragraph saying so —
`crates/viewer/src/lib.rs:92-100` was cited as the fullest statement:
*"Nothing here goes red if the modules it names start running, stop
existing, or grow a sibling — the enumeration above is kept by hand, and
a marker that silently went stale would look exactly like this one."*

> **[CITE repair, 2026-09-11.] That quotation is not in the tree and is
> not repointed here, because there is nothing to repoint it to: the
> paragraph at that site now says the OPPOSITE, and repointing would
> invent a subject.** The sentence above is left exactly as filed so the
> reversal stays legible, and the current text is quoted under "Entry 1"
> below. Three of the other seven copies do carry a paragraph of this
> shape; four do not. Which of the eight this row is actually about is
> TINT's call, not CITE's.

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

## Moved to S-TINT (2026-09-11)

Moved by `git mv` from `work/tcost/` at S-TINT's opening. Id, title and
body are unchanged; the directory is the claim (`work/README.md`).

**Why it moved.** S-TCOST's board was re-sorted on 2026-09-11 against the
repository going public on 2026-09-03 (`work/tcost/log.md`, the
2026-09-11 seam). That sort found this row is not a cost lever in either
currency — it neither shortens the gate's critical path nor saves a
billed minute, and it was never argued on one. It reached S-TCOST by the
tracker-wide re-home of 2026-09-04, which routed rows by PATH GLOB
(`crates/*/tests/*`, `crates/test-utils/*`) rather than by question.
This program is the question it was always about: whether the suite
asserts what it claims to assert.

## All eight table entries re-derived against the tree, 2026-09-11

Every file, line, `fn` name and quotation the table above and the
sections around it point at was resolved against the repository as it
stood on 2026-09-11, and re-cited by NAME where a name exists; the table
below is that derivation, with the command that reproduces it. The
derivation was re-run and corrected in a second pass the same day, after
a review of the first.

The repair was authorised by Ev and carried out in place rather than
routed back; it came out of the CITE row
`loud-skip-marker-row-cites-a-lib-paragraph-that-was-reversed`, which is
a secondary reference only — that row's file is deleted when CITE
closes, and nothing in this section depends on it.

**Only what the row POINTS AT and QUOTES is touched.**
The claim, the count in the title, the membership of the table and the
two fix shapes are TINT's and are unchanged — including where the
re-derivation below puts them in question.

All eight were resolved at this base with one command:
`for f in <the eight files>; do grep -n "_lane_skipped_" "$f"; done`.

| the table said | at this base | disposition |
| --- | --- | --- |
| `viewer/src/lib.rs:103`, `app_lane_skipped_no_chrome_or_gpu_coverage_here` | no `fn` of that name anywhere in the tree; the `fn` in that file is `app_lane_skipped_no_app_feature_coverage_here` (`:117`), and `:103` is a line of the doc above it | **unresolved, not repointed** — see Entry 1 |
| `chrome_labels.rs:30` | `fn` at `:30` | resolves; line dropped, name kept |
| `error_display.rs:307` | `fn` at `:325`; `:307` is `let shown = indeterminate_wording("face", &cause);` | repaired to the name |
| `panel_display.rs:770` | `fn` at `:777`; `:770` is a bare `///` inside the doc block | repaired to the name |
| `m5_s12_curved_ops_interval.rs:27` | `fn` at `:27` | resolves; line dropped, name kept |
| `m5_s13_pips_interval.rs:19` | `fn` at `:19` | resolves; line dropped, name kept |
| `m6_surgery_interval.rs:20` | `fn` at `:20` | resolves; line dropped, name kept |
| `m6_2_fitted_at_rest.rs:188` | `fn` at `:188` | resolves; line dropped, name kept |

**Three of the eight line numbers were wrong** — `lib.rs:103`,
`error_display.rs:307` and `panel_display.rs:770` — **and one of those
three is one of the two a 2026-09-11 re-derivation had recorded as still
resolving** (`panel_display.rs:770` → the `fn` is at `:777`; it was
certified fine five days ago). That is the reason the table now carries
no line numbers at all: each name is one `grep -n` from its file.

**Seven of the eight names resolved at this base, not eight.** The
eighth, `app_lane_skipped_no_chrome_or_gpu_coverage_here`, resolves
nowhere in the tree — which is not a citation that drifted but a subject
that is gone, and it is Entry 1 below. Names are the better citation
here because the seven that have a subject all landed exactly; they are
no defence against a subject being removed, and nothing is.

The three citations in `## What is already settled` were checked too and
all three resolve exactly — `crates/test-utils/src/vacuity.rs:201-203`
(`pub fn stood_down`), `vacuity.rs:73-76` (the quoted exclusion,
verbatim), and `crates/step-import/tests/cert5_r1_import_probes.rs:252-258`
(the restatement).

### Entry 1: repaired at #1848, and what it is now

**Verified against the tree, not taken on #1848's word.** At this base
`crates/viewer/src/lib.rs`'s marker is `fn
app_lane_skipped_no_app_feature_coverage_here`, gated
`#[cfg(all(test, not(feature = "app")))]`, and the paragraph above it —
the one this row quoted as the fullest statement of the class — reads:

> **This row closes no gate and cannot fail.** It is evidence, not a
> check: its whole payload is its NAME appearing in the PASS list, so a
> reader of a default-feature run meets the absence instead of
> inferring it. It names the FEATURE and what the feature costs, and
> nothing else — the roster is the `#[cfg(feature = "app")]` block
> above, which the compiler keeps, so there is no hand-kept enumeration
> here to go stale when that block gains or loses a module. Read it as a
> sentence the log carries, and keep gating to the rows themselves.

Its `println!` keeps no hand-kept ENUMERATION, which is the load-bearing
point — but it is not row-free. Verbatim, it reads *"every module this
crate gates behind the `app` feature is absent from this build - their
unit rows, including the pipeline-creation smoke row (every
`create_render_pipeline` call in the viewport), run only where the `app`
feature is built"*: the subject is the FEATURE, and the one row it names
is named as an illustration of what the feature costs, not as a list
that has to be edited when the `#[cfg(feature = "app")]` block gains or
loses a module. So on its face this copy is no longer an
instance of the class the row's own `## Finding` defines — *"whose
entire body is a `println!` naming, BY HAND, the rows that did not
compile"* — and fix shape 2 in `## What is already settled` ("drop the
enumeration") is what was done to it.

**Whether this row has lost a MEMBER or only a CITATION is TINT's call
and is deliberately left open here.** CITE did not delete the entry,
did not repoint it at
`app_lane_skipped_no_app_feature_coverage_here` — that `fn` is a
different thing from the one the row named, and repointing would invent
a subject — and did not touch the title's "eight". What CITE asserts is
only the tree: the named `fn` does not exist, and the paragraph at the
cited site says the opposite of what the row quotes.

If TINT rules the member dead, note that #1848 also makes the row's
strongest evidence its own worked example: one of the eight was
converted by fix shape 2 and the conversion held.

### Two more things for TINT, found by the sweep and not acted on

**1. The idiom has ten copies at this base, not eight.** Two are absent
from the table and have never been in it:

- `crates/sweep/tests/review_fillet_e3_probes.rs`,
  `fn interval_lane_skipped_no_certified_coverage_here`
- `crates/sweep/tests/blend_margin_payload_interval.rs`,
  `fn interval_lane_skipped_no_certified_coverage_here`

Both are the exact shape the table's last four are: `#[cfg(not(feature =
"interval"))] #[test] fn` whose entire body is a `println!`. Found by
`grep -rn "lane_skipped" --include=*.rs crates/ demos/ tools/ benches/`,
which returns fourteen lines: exactly ten `fn` declarations — the eight
in the table plus these two — and four hits that are not declarations
at all (`crates/test-utils/src/vacuity.rs:74` and
`crates/step-import/tests/cert5_r1_import_probes.rs:255` are the two
prose exclusions `## What is already settled` already quotes, and
`crates/editor-core/tests/m10_5_r1_probes_interval.rs:477` and `:480`
are `println!` strings in a RUNTIME diagnostic, not a `#[cfg]` marker
row). Corroborated by `grep -rln "Loud skip" --include=*.rs .`, which
returns the same ten files.

**That second sweep agrees only because it is case-sensitive, and
nothing enforces the capital L.** `grep -rlin "loud skip" --include=*.rs .`
returns thirteen; the three extra are exactly the over-matches `## Home`
already warns a taker off (`crates/step-import/tests/freecad.rs`,
`crates/sweep/tests/m5_pr9_cosurface_merge.rs`,
`crates/topo/src/merge_faces.rs`), so the count survives, but a sweep
resting on how a sentence was capitalised is not evidence anyone should
lean on. A third sweep on a different shape agrees:
`grep -rln 'SKIPPED (' --include=*.rs .` returns eleven — the same ten
plus `crates/test-utils/src/vacuity.rs`, which is the `stood_down` door
`## What is already settled` above deliberately sets outside this idiom.
Ten is the number all three sweeps reach.

CITE does not add them: the table is the row's membership and membership
is TINT's.

**2. The `## Finding`'s "every copy admits in its own rustdoc that it
goes stale silently" holds for three of the eight, not eight.**
`chrome_labels.rs`, `error_display.rs` and `panel_display.rs` each carry
a *"**This row closes no gate and cannot fail**"* paragraph that says
the list is kept by hand. `lib.rs` carries the reversal quoted above.
The four `interval_lane_skipped_…` copies each carry a *"**Loud
skip.**"* paragraph that argues only why the announcement exists and
admits nothing about staleness — but they do not carry it in the same
words, so the wording is attributed here only where it matches
(`grep -n -A3 "Loud skip" <file>`, run on each of the four):

- `m5_s13_pips_interval.rs` and `m6_surgery_interval.rs`, verbatim and
  identically: *"announce the skip so a lane that silently lost its
  certified rows stays visible in the battery log"*.
- `m5_s12_curved_ops_interval.rs`: *"Announce the skip instead, so a
  lane that silently lost its certified rows is visible in the log."*
- `m6_2_fitted_at_rest.rs`: *"a lane that silently lost its interval
  rows must stay visible in the battery log"*.

Of the two unlisted copies, `blend_margin_payload_interval.rs` carries
the first wording word for word; `review_fillet_e3_probes.rs` carries
one sentence and no argument at all — *"**Loud skip.** Without
`--features interval` this file is empty."* — which likewise admits
nothing about staleness. The underlying defect is unaffected (those
`println!`s do name rows by hand, and `m5_s12`'s names four of them),
but the sentence claiming every copy SAYS so is wider than the tree.

**Out of fence, reported not fixed:** `crates/test-utils/src/vacuity.rs`
says *"The four whole-binary `interval_lane_skipped_…` rows"* and there
are six. That is a code comment, and this repair changes no code.
