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
