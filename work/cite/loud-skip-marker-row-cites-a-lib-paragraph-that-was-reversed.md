---
id: loud-skip-marker-row-cites-a-lib-paragraph-that-was-reversed
kind: issue
title: work/tcost/loud-skip-marker-is-a-hand-kept-idiom cites three stale rows and one paragraph VIEW reversed, and this is the second time it has been reported with nothing filed
status: open
opened: 2026-09-06
refs: [2089, 1848]
---



Filed from #2089's fix pass. It sits here rather than on VIEW's or
TCOST's slate because the finding spans both: the stale text is in
`work/tcost/`, the code it cites is VIEW's territory, and the defect
this row exists to record is that neither program's process produced a
file the last two times someone noticed.

## The row

`work/tcost/loud-skip-marker-is-a-hand-kept-idiom` is open and lists
eight copies of the loud-skip idiom in a `file:line` table, plus a
quoted paragraph it calls the fullest statement of the class. Four of
its `crates/viewer` entries do not resolve.

**1. `crates/viewer/src/lib.rs:103`, row `app_lane_skipped_no_chrome_or_gpu_coverage_here`.**
No function of that name exists anywhere in the tree. The row in that
file is `app_lane_skipped_no_app_feature_coverage_here`, at
`crates/viewer/src/lib.rs:117`. Line `:103` is
`/// `cargo nextest run -p viewer --features app` (same file) and` —
a line of the doc comment above it.

**2. `crates/viewer/src/lib.rs:92-100`, and this one is the interesting
failure.** The item quotes that range as *"Nothing here goes red if the
modules it names start running, stop existing, or grow a sibling — the
enumeration above is kept by hand, and a marker that silently went
stale would look exactly like this one."* That sentence is not in the
file, and the paragraph now at `lib.rs:106-114` says the **opposite**:
*"the roster is the `#[cfg(feature = "app")]` block above, which the
compiler keeps, so there is no hand-kept enumeration here to go stale
when that block gains or loses a module."* VIEW fixed this copy at
#1848; the marker at `lib.rs` is no longer hand-kept, so the row's own
lead evidence has been REVERSED, not merely renumbered. A reader
working the row from its quotation would go looking for a defect that
was repaired two days ago.

**3. `crates/viewer/tests/error_display.rs:307`.** The cited row,
`app_lane_skipped_startup_error_arms_not_checked_here`, is at `:322`.
Line `:307` is `prose(&shown, "TargetFailed");`.

> **Both of those numbers have since drifted, and the drift is the
> point.** Re-derived at the CITE cut (2026-09-11): the `fn` is at
> `error_display.rs:325`, and `:307` is now
> `let shown = indeterminate_wording("face", &cause);`. Five days took
> three lines. Entry 1's reading of `lib.rs:103` still holds. Nothing
> about the finding changed — what changed is that the re-derivation
> recorded here, written to repair stale numbers, went stale itself
> before anyone took the row. **A taker re-derives at its own base and
> does not trust this paragraph either**, and that is the case for
> landing the row in cite-by-name form rather than with fresher
> numbers.

**Two of the eight entries do still resolve** and are recorded so a
taker knows the row is not wholly rotten:
`crates/viewer/tests/chrome_labels.rs:30` and
`crates/viewer/tests/panel_display.rs:770` both land on their named
`fn`. The four rows outside `crates/viewer` (`crates/sweep` ×3,
`crates/topo` ×1) were not re-derived here.

## The actual finding: §6 reported this twice and filed nothing

`docs/prompts/implementer-discipline.md` §6 says a defect found outside
your fence *"goes in your report and your PR description — not into
another program's tracker directory"*, and that reporting IS the filing
act because the orchestrator has the whole board. That has now been
exercised twice on this exact row, by this exact program, with no
durable artifact either time:

- **#1848** (`work/view/loud-skip-marker-says-two-modules-and-there-are-six`,
  closed 2026-09-04) ends: *"That issue's own table now cites a row
  that has been renamed (`:21`) and a moved line (`:23`); reported
  rather than edited, since the issue is homed outside this program's
  fence and the orchestrator is taking the tracker edit."* The tracker
  edit was not taken.
- **#2089** re-found it independently, reported it in the PR body and
  the lane report under the same clause, and would have left the same
  nothing behind.

That is precisely the case §6 warns about in its own second reason —
*"you cannot tell whether the item already exists"* — landing on the
opposite side from the one it anticipates. §6 is written against a
lane filing a DUPLICATE it cannot see; what happened here is a lane
filing NOTHING, twice, and the second lane could not see the first
because a merged PR body is not a slate and #1848's disclosure is
inside a closed item's prose, which `work/README.md` says the
re-homing sweep cannot see either.

**So the residue is a file, and this is it.** Whoever takes the row
should re-derive all eight entries at their own base — the four
`crates/viewer` ones drift because `lib.rs` and the test files are
edited by three programs — and decide whether entry 2 is a stale
citation or a row that has lost one of its eight members.

## Confidence

`sure` on the four non-resolving citations and on the reversal: each is
one `grep` and one `sed -n` away. `sure` that #1848 and #2089 both
reported it through §6 and that no file resulted. `likely` that the
§6-produced-nothing observation is worth its own sentence in
`implementer-discipline` §6, which is Ev's call and not a lane's.

## Re-homed to CITE (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

CITE collects the rows about the project's own text and harness rather
than its kernel: citations that rot, numbers that were reissued, and the
paperwork a lane runs on. This row is one of them.

Its class at the cut was **M** — re-derive eight citations across three
programs' files; decide if a reversed member is dead. The class is a
dispatch estimate made by reading the row against the tree on
2026-09-11, not a verdict on the finding, and a lane that finds it wrong
says so in its PR. The id, the `track:` letter where the row carries
one, and the body above are unchanged by the move.

## Repaired 2026-09-11, in TINT's file, on Ev's authorisation

Ev authorised CITE to repair this row's citations directly in
`work/tint/loud-skip-marker-is-a-hand-kept-idiom.md` rather than route
them, in one PR, with no routing issue filed and no `work/README.md`
change. The authorisation covers this repair and the two beside it and
nothing wider.

**What was done.** All eight entries were resolved at the repair's own
base with `grep -n "_lane_skipped_"` per file. The table now carries the
`fn` name and the file and **no line numbers**, entry 1 is marked
unresolved in place rather than repointed, and the reversed quotation is
left as filed with the current `lib.rs` text quoted beneath it and the
member/citation question handed to TINT.

**Where the re-derivation recorded ABOVE was wrong, five days on.**
The block quote in `## The row` says two of the eight *"do still
resolve"*: `chrome_labels.rs:30` and `panel_display.rs:770`.
`chrome_labels.rs:30` still does. **`panel_display.rs:770` does not** —
the `fn` is at `:777` and `:770` is a bare `///` inside the doc block.
`error_display.rs`'s `fn` is at `:325`, which the block quote had right.
So of the eight numbers the original table carried, **three** were wrong
at the repair's base — `lib.rs:103`, `error_display.rs:307` and
`panel_display.rs:770` — and the last of those is one of the two this row
had certified as fine, gone stale inside the five days between the
certification and the repair. The paragraph warning a taker not to trust it was correct about
itself.

**The four nobody had re-derived all resolve.** `m5_s12:27`,
`m5_s13:19`, `m6_surgery:20` and `m6_2_fitted_at_rest:188` each land on
their `fn` exactly. Line numbers were dropped from them anyway: a number
that happens to be right is not a citation that will stay right, which is
the whole of `work/cite/S176.md`.

**Two findings handed to TINT, not acted on** (recorded in TINT's file in
a marked section):

- **The idiom has ten copies, not eight.**
  `crates/sweep/tests/review_fillet_e3_probes.rs` and
  `crates/sweep/tests/blend_margin_payload_interval.rs` each carry the
  same `#[cfg(not(feature = "interval"))] #[test] fn
  interval_lane_skipped_no_certified_coverage_here`, and neither has ever
  been in the table. Found by
  `grep -rn "lane_skipped" --include=*.rs crates/ demos/ tools/ benches/`,
  corroborated by `grep -rln "Loud skip" --include=*.rs .` returning
  exactly those ten files. Membership is TINT's, so CITE recorded them
  rather than adding them.
- **"every copy admits in its own rustdoc that it goes stale silently"
  holds for three of the eight.** Three viewer test copies carry the
  admission, `lib.rs` carries #1848's reversal of it, and the four
  `interval_lane_skipped_…` copies say only why the announcement exists.

**Out of fence, reported not fixed:** `crates/test-utils/src/vacuity.rs`
excludes *"The four whole-binary `interval_lane_skipped_…` rows"* and
there are six. A code comment; this repair changed no code.

The `## The actual finding` half of this row — that §6 produced no
durable artifact twice — is untouched and stands: this file is that
artifact, and the repair is what it was filed to get.
