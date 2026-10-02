# SYM-16 — the ignored receipt rows drifted red: attribute the drift, re-take, and give them a schedule (spec)

**Program:** SYM (`work/sym/plan.md`). **Item:**
`work/sym/ignored-sym-receipt-rows-drifted-red-on-main-unattributed.md`
(P1). **Unit:** `work/sym/SYM-16.md`. **Branch:** `sym/16-receipt-drift`,
cut from `main`. **Implementer:** Opus. **Review tier:** single FULL
review (§Review).

**Read first, in full:**
- `docs/prompts/implementer-discipline.md`;
- the item, whole;
- `crates/editor-core/tests/sym11_exact_channel_rows.rs`:
  `PAST_THE_CEILING`, the row
  `sym11_the_exact_channel_never_contradicts_past_the_ceiling`, and its
  doc's "WHEN IT IS RE-TAKEN";
- `crates/editor-core/tests/m10_9_pins_interval.rs`:
  `m10_9_the_pad_at_both_rule_f_dials`, which DECIDE-9 (#3807)
  re-baselined, and the gating rows beside it;
- `.config/nextest.toml`, the slow set and its header, and
  `work/ciw/latency-cut.md` for how a row joins it.

## The claim

Two `#[ignore]`d evidence rows pin the pad's receipts exactly, and they
drifted red with no one seeing it:
- `sym11_the_exact_channel_never_contradicts_past_the_ceiling`;
- the pad's rule-F differential, now
  `m10_9_the_pad_at_both_rule_f_dials`.

The drift, from the item's table, is unattributed:
- on the pad, `numeric` +8 and `frozen` 2750 → 2722;
- on the link, `numeric` +4;
- on the bracket, `numeric` +2.

Its window opens at SYM-11 Phase 1 (`03ac24d8ba`, 2026-09-21), which
stored the values. It closes no later than `8ee3daf171` (2026-09-26),
where ENCL measured the drift.

Since then the stored values have been adjusted several times, each by
one PR's own delta only:
- ENCL's must-carry gate;
- PR 3266's run-out carrier;
- DECIDE-9's zero-product gate.

So the rows' current redness is the drift plus whatever has landed
since. Neither is attributed.

## Phase 1 — attribute

1. **Re-take at `main`.** Run both rows at the current `main`, release,
   at ε = 1e-9, 1e-6 and 1e-12. Give stored against measured, per
   document and dial.
2. **Bisect the window.** Find the commit that moved each drifted
   number.
   - The history is not linear: SYM's units landed on `props/sign-hull`
     until #2468 merged it into `main` on 2026-10-01. Decide which line
     the rows lived on through the window, and bisect along it
     (`--first-parent` where a merge carried the move). Say which.
   - The sym11 row costs about two minutes per step. Build the bisect
     script once, keep it in your scratch, and put it in the PR body.
3. **For each move, say whether the decision is right.**
   - Name the predicate(s) whose decisions changed class (use the
     per-predicate split where it exists). Say what the commit
     intended, and whether the move follows from it.
   - The sym11 row's own message calls a moved receipt "a finding, not
     a table to refresh". A move that is wrong is a finding, filed on
     its owner's slate, not re-baselined.
4. **Everything since the window.** For anything that moved the rows
   after `8ee3daf171` beyond the deltas already credited, attribute it
   the same way. Bisect only what is unattributed.

**Stop rule.** If a move is a defect in code SYM does not own:
- file it on the owner's slate with the bisected commit and the
  decisions;
- leave that row's number at the measured value, with a comment naming
  the filed item;
- report before Phase 2.

## Phase 2 — re-take and schedule

- **Re-take both rows' tables at `main`.** Annotate each value whose
  move Phase 1 attributed with its commit or PR, in the rows' existing
  style. Ev's standing words: "never skip out on a change that would
  make the code better because it would require rebaselining".
- **Give the rows a schedule.** The item's remedy: `#[ignore]` plus "by
  hand at each SYM unit's close" let the drift land unseen.
  - Move both rows out of `#[ignore]` and into the slow set in
    `.config/nextest.toml`, following `work/ciw/latency-cut.md`'s
    convention. Give the row's measured cost in the PR body.
  - Or, if the slow set's rule forbids it, say why and propose the
    schedule.
  - Make the rows' docs say what now runs them, and remove "re-taken
    by hand".
- **Rows:** none new beyond the schedule, unless Phase 1 found a shape
  worth a minimal row.

## Scope

- **Files:**
  - the two test files;
  - `.config/nextest.toml`;
  - the item and the unit;
  - any item filed on another slate.
- **Territory:** `.config/nextest.toml` is claimed by no program; CIW's
  `work/ciw/latency-cut.md` governs the slow set, so announce the change
  to CIW in the PR body. Run
  `python3 scripts/work.py territory --base origin/main` and announce
  every seam it names.
- **Not in scope:** fixing a move another program owns (Phase 1 files
  it), and any change to what the rows measure.

## Review

**Single FULL review**, recorded in `work/sym/log.md` at spec time.
This is a measurement, attribution and schedule change, settled by
re-running the rows and the bisect. Claims to falsify:
1. each bisected commit, by re-running its parent and itself;
2. the right-or-wrong call on each move;
3. the re-taken tables at three ε;
4. the schedule (does the row run on the gate, at what cost?);
5. plus `docs/prompts/reviewer-style-lane.md` in full.

## Landing

- When the PR opens, the unit's status becomes `review`.
- At merge the orchestrator closes the unit, deletes this spec (with a
  note under `docs/doc-ledger/`), and closes the item.
