---
id: loud-skip-marker-is-a-hand-kept-idiom
kind: issue
title: The loud-skip marker is a hand-kept idiom in eight files, each copy naming its rows by hand and admitting it goes stale silently
status: closed
opened: 2026-09-04
refs: [1776]
closed: 2026-09-15
pr: 2656
parent: loud-stand-down-announcements-are-discarded-by-the-gate
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


## The `crates/viewer` citation is not stale — it is contradicted (VIEW, 2026-09-12)

Checked while placing a citation sweep from a VIEW lane, which had
proposed shifting `crates/viewer/src/lib.rs:92-100` to `:93-101` as an
ordinary line move. It is not one.

**The sentence this row quotes as "the fullest statement" no longer
exists.** `grep -n "Nothing here goes red"` over
`crates/viewer/src/lib.rs` returns **nothing**. The loud-skip paragraph
now begins at `crates/viewer/src/lib.rs:96` and says the opposite of
what is quoted here:

> **This row closes no gate and cannot fail.** It is evidence, not a
> check … It names the FEATURE and what the feature costs, and nothing
> else — **the roster is the `#[cfg(feature = "app")]` block above,
> which the compiler keeps**, so there is no hand-kept …

So at this site the *hand-kept enumeration* the row is about was
replaced by a compiler-kept one, and the marker was narrowed to name
the feature rather than the modules. Whether that closes this row
depends on the other sites it cites, which VIEW has not checked —
`crates/viewer` is the only one on VIEW's ground.

**Recorded rather than acted on**: re-pointing the line numbers would
have preserved a quotation the source no longer contains, which is the
`stale-file-citations` failure in its worse form — a citation that
still resolves and now misdescribes. The VIEW lane's sweep read the
shift as a true move because the surrounding lines still matched; the
quoted sentence is what moved out from under it.

## Re-derived (2026-09-15, lane C)

**VERDICT: REPRODUCES**, and the row's own predicted cost has already
been paid once, silently, in `crates/viewer/tests/error_display.rs`. The
two questions the 2026-09-11 section left open are settled below: Entry 1
is a **member that is dead, not a citation that drifted**, and the
"every copy admits it" sentence is **wider than the tree** — three of ten
copies say it.

### The population today

`grep -rn "lane_skipped" --include=*.rs crates/ demos/ tools/ benches/`
returns fourteen lines, the same split CITE derived: **ten `#[cfg]`-gated
marker `fn` declarations**, two prose exclusions
(`crates/test-utils/src/vacuity.rs`, `crates/step-import/tests/cert5_r1_import_probes.rs`)
and two `println!` strings in a RUNTIME diagnostic
(`crates/editor-core/tests/m10_5_r1_probes_interval.rs`, twice, not
`#[cfg]` markers). `grep -rln "Loud skip"` returns the same ten files;
`grep -rln 'SKIPPED ('` returns those ten plus `vacuity.rs`. **Ten is
confirmed at this base**, so the title's "eight" is two short and the two
CITE named (`crates/sweep/tests/review_fillet_e3_probes.rs`,
`crates/sweep/tests/blend_margin_payload_interval.rs`) are members by
shape: both are `#[cfg(not(feature = "interval"))] #[test] fn
interval_lane_skipped_no_certified_coverage_here` whose whole body is a
`println!`.

### What each copy's `println!` actually enumerates

This is the defect, so it is derived per copy rather than counted:

| copy (`fn` name where it differs) | names rows by hand? |
| --- | --- |
| `crates/viewer/src/lib.rs` `app_lane_skipped_no_app_feature_coverage_here` | **no** — names the FEATURE and one row as an illustration |
| `crates/sweep/tests/review_fillet_e3_probes.rs` | **no** — *"contributes NO certified coverage in this run."* and nothing else |
| `crates/viewer/tests/chrome_labels.rs` | yes — two subjects, and the gated `mod chrome` carries exactly two rows |
| `crates/viewer/tests/error_display.rs` | yes — **ONE row named, TWO gated. See below.** |
| `crates/viewer/tests/panel_display.rs` | yes — one row named by its full `fn` name, one gated |
| `crates/sweep/tests/m5_s12_curved_ops_interval.rs` | yes — four rows named; all four resolve |
| `crates/sweep/tests/m5_s13_pips_interval.rs` | yes — two named, two gated |
| `crates/sweep/tests/m6_surgery_interval.rs` | yes — one named, one gated |
| `crates/topo/tests/m6_2_fitted_at_rest.rs` | yes — one named, one gated |
| `crates/sweep/tests/blend_margin_payload_interval.rs` | yes — names *"the enclosure arm"*, singular; three rows are gated |

So **eight of ten still carry a hand-kept enumeration**, and fix shape 2
has been applied to exactly one (`viewer/src/lib.rs`); the tenth
(`review_fillet_e3_probes.rs`) never had one.

### The predicted failure has fired — `crates/viewer/tests/error_display.rs`

Its marker's own rustdoc says, verbatim:

> **This row closes no gate and cannot fail** — its payload is its NAME
> in the PASS list. It names ONE row, so a second `app`-gated row added
> to this file leaves the marker quietly incomplete; nothing mechanical
> says so.

That second row exists. The file carries **two** `#[cfg(feature =
"app")] #[test]` rows — `indeterminate_wording_forwards_the_causes_own_words`
and `startup_error_forwards_every_payload_arm` — and the marker
`app_lane_skipped_startup_error_arms_not_checked_here` names only the
second. A default-feature run is told that `StartupError`'s forwarding is
unchecked and is told nothing about `indeterminate_wording`. This is the
row's `## Finding` cost sentence — *"a ninth `app`-gated row added to
`panel_display.rs` leaves its marker quietly wrong, and nothing anywhere
notices"* — realised in a sibling file. It is the strongest evidence the
row has and it did not exist when the row was filed.

### Settled: Entry 1 is a dead MEMBER

`app_lane_skipped_no_chrome_or_gpu_coverage_here` resolves nowhere in the
tree (the fourteen-line `lane_skipped` grep above is exhaustive over
`crates/`, `demos/`, `tools/`, `benches/`). `crates/viewer/src/lib.rs`
carries `fn app_lane_skipped_no_app_feature_coverage_here`
(`#[cfg(all(test, not(feature = "app")))]`) whose `println!` names the
feature and, as an illustration of what the feature costs, the
pipeline-creation smoke row — no list to edit when the
`#[cfg(feature = "app")]` block gains or loses a module. Its paragraph
(`"**This row closes no gate and cannot fail.**… the roster is the
`#[cfg(feature = "app")]` block above, which the compiler keeps, so there
is no hand-kept enumeration here to go stale"`) is the reversal CITE
recorded, and it is still there.

**Ruling proposed: the member is dead and the row lost nothing by it.**
The subject the row named is gone and what replaced it is not an instance
of the class the `## Finding` defines. The population is ten because two
were never in the table, not nine-plus-one. `viewer/src/lib.rs` should be
kept in the file as the **worked example of fix shape 2** — it is the
only copy that has been converted and the conversion held — and dropped
from the membership list.

### Settled: "every copy admits in its own rustdoc that it goes stale"

**Three of ten.** `grep -rn "This row closes no gate and cannot fail"`
returns four files; of those:

- `chrome_labels.rs` — *"the list it recites is kept by hand and a stale
  one would read exactly like a current one"* — admits it;
- `error_display.rs` — *"It names ONE row, so a second `app`-gated row
  added to this file leaves the marker quietly incomplete"* — admits it;
- `panel_display.rs` — *"It names the row by hand, so a second
  `app`-gated row added to this file leaves the marker quietly
  incomplete"* — admits it;
- `viewer/src/lib.rs` — the reversal; admits nothing because there is
  nothing to admit.

The six `interval_lane_skipped_…` copies carry a **"Loud skip."**
paragraph that argues why the announcement exists and says nothing about
staleness. So the `## Finding`'s universal is wrong in the tree and was
wrong at 2026-09-11 too; CITE's "three of the eight" is right and the
denominator is ten. **The underlying defect is untouched by this** —
eight copies do carry hand-kept lists whatever their prose says.

### The relation to `loud-stand-down-announcements-are-discarded-by-the-gate`

Different population, and the two compound rather than overlap.
`test_utils::vacuity::stood_down` is a `println!` inside a row that RAN;
these ten are `#[cfg]`-gated rows that exist only because their siblings
did not compile, and `vacuity.rs`'s own module docs deliberately exclude
them (*"a different idiom and deliberately not converted: their entire
body is the announcement"*). Checked here: `.github/workflows/ci.yml`
carries exactly one `--success-output` anywhere, on the
`cargo nextest run -p viewer --features app` GPU-smoke job — the lane
where the `app` markers do **not** compile. Every gating
`cargo nextest run` that DOES compile these markers (the archived
default and interval matrix runs) passes no `--success-output`, whose
default is `never`.

**The consequence for this row, which is new and sharpens it.** Each
marker's stated payload is split in two and only half survives the gate:
the `fn` NAME reaches the PASS list and is read; **the `println!` body —
which is precisely the hand-kept enumeration this row is about — is
discarded by every gating run**. So the eight hand-kept lists are not
merely stale-prone: on the gate nobody can read them at all. That is an
argument for fix shape 2 (drop the enumeration, keep the name) that the
row did not have, and it means the cost of the staleness is borne only by
a human reading a local run.

### Out of fence, still wrong

`crates/test-utils/src/vacuity.rs` still says *"The four whole-binary
`interval_lane_skipped_no_certified_coverage_here` rows"* and there are
**six**. CITE reported this on 2026-09-11 and it is unrepaired. It is a
`crates/test-utils` comment — S-TCOST's glob — not this lane's to touch.

**Recommendation (orchestrator's call).** Keep open; retitle to "ten
copies" if the title is ever re-cut, take fix shape 2 (the `lib.rs`
conversion is the worked example), and repair `error_display.rs`'s
marker in the same unit since it is already incomplete.

## Closed — TINT-2, PR #2656 (2026-09-15)

Closed with its carrier, `loud-stand-down-announcements-are-discarded-by-the-gate`;
that row holds the full account.

**The population was ten, not the title's eight, and the repair is one
macro** — `test_utils::loud_skip_marker!` — carrying all nine in-fence
sites. The tenth, `crates/viewer/src/lib.rs`, is `src/` and out of
fence; it is filed as `work/view/viewer-lib-marker-claims-the-log-carries-its-sentence`
rather than edited.

**Three copies were FALSE, not the one the review first named.** The
row's predicted failure had fired more widely than anyone had counted:

| file | `#[test]` rows before the gated block | the claim it made |
|---|---|---|
| `crates/topo/tests/m6_2_fitted_at_rest.rs` | 4 | false — 3 ungated |
| `crates/viewer/tests/error_display.rs` | 16 | false — 15 ungated |
| `crates/viewer/tests/panel_display.rs` | 17 | false — 16 ungated |

Each said, in one of three spellings, that its file's rows are the gated
ones — while fifteen and sixteen of them run right there. **The macro's
wording closes the class by construction** rather than patching three
sites: it says only that *the rows this file gates behind `feature`* are
not compiled here, which is true of an empty binary and of one with
sixteen ungated rows alike.

**The hand-kept enumeration is gone in both halves.** The `println!`
bodies no longer name rows, and the two marker NAMES that were
themselves enumerations were renamed — the name being the half that
reaches the PASS list and is read.
