# TIGHTEN: min_separation decides its cells, under an unlogged scope

One P1 row. Program: CLEAR. Branch: `clear/tighten`. Review: DUAL.
The unit changes what a shipped bracket certifies, so its soundness
needs two independent readers.
- `work/clear/min-separation-tightening-crosses-the-drive.md`. Read its
  `## Decided (CLEAR orchestrator, 2026-10-28)` section in full: it is
  this unit's design. `docs/DESIGN-FORK-LOG.md` row 206 holds the
  argument and what was rejected. Do not re-open the design. If the
  code shows that a decided point cannot hold, stop and report it.

## What is true when this lands

1. **A scoped unlogged door.** `geom_core::k_stats` has a scoped form
   of `check_unlogged`: a closure that runs under its own frame.
   - Its verdicts and escalations are discarded. Its K samples keep
     their predicate names.
   - `check_unlogged` becomes the one-margin case of the scoped door,
     so it is not a second spelling.
   - A new ledger row in `docs/predicate-dimension-audit.md`, numbered
     after the last F row, states the argument: every outcome of a
     reading inside the scope leaves a true bracket, so the readings
     choose which cells feed the bracket and decide nothing a census
     must see.
2. **The engine's door opens that scope around its region readings.**
   These are `window_of` with a band, `cell_region` and
   `station_region`, all inside `min_separation`.
   - The receipt may count what the scope decided, as a receipt line.
     It is not a census row.
   - **No node's census gains a row.** `m10_6_certifying_keys.txt` and
     every census golden stay bit-identical. If one moves, that is a
     finding: stop and report.
3. **`min_separation` decides cells, as the sweep does.**
   - Definite `Outside` cells drop out of `lo`. `lo` stays the least
     `separation_lo` over every pair still standing, `Undecided`
     included.
   - `hi` reads only `Inside`–`Inside` cell pairs, or station pairs that
     the point doors place `In`, `OnEdge` or `OnVertex`. So `[lo, hi]`
     encloses the FACES' separation.
   - An absent `hi` reads `Unevaluated`, and the drive bisects under its
     existing rule.
4. **What retires:**
   - `Certified::LowerBoundOnly` and `UnevaluatedReason::WindowSuperset`;
   - `WINDOW_TIGHTENING`;
   - `MinSeparation::window_hi`'s caveat;
   - `topo::clearance`'s header paragraph saying that the sweep's
     windows and the measure's differ.

   Their consumers follow: `pncad`'s `analysis.rs`, the viewer's
   `features.rs`, `pncad-py`'s binding census test, and editor-core's
   `measure.rs`, `eval/measure.rs` and `lib.rs`. Re-check whether
   `Certified::Neither` still earns its place, and report the answer
   with its reason, whether you retire it or keep it.
5. **Pins:**
   - the M10-6 notch fixture's bracket contains the true face
     separation, 0.269, at more than one budget;
   - `m10_6_r1_probes_interval::the_notch_bracket_is_the_windows_not_the_faces`
     flips. Rename it to say what is now true;
   - the seven M10-6 rows that the earlier tightening broke stay green.
     Find them by name from the row's history (TRIM-3 PR-2) and list
     them in the PR body.

## First probe

Run this before you write the change. It is a two-run differential:
`band: None` against `Some(band)` under the scope, on the notch
fixture and on one `min_clearance` document through the drive. It
measures:
- that no census row appears;
- whether in-band escalations would bisect every leaf. The design
  marks that unverified and likely. If they do, report the cost and
  propose, but do not build, a remedy.

## Ground

- `crates/topo/src/clearance.rs` (`min_separation`'s `Bracket`
  question; since LOOPS (#5976), a `Question` impl on the one walk).
- `crates/geom-core/src/k_stats.rs`.
- editor-core's `measure.rs` and `eval/measure.rs`, and the
  `Certified` consumers named above.
- `docs/predicate-dimension-audit.md` (one ledger row).
- Parallel units:
  - CURVEDEP edits `crates/topo/src/injective.rs`. Keep out of it.
  - CLEARTEXT (#5977) edits the refusal types and their renderings in
    both `clearance.rs` files. Keep your diff out of the refusal types.
    If it merges first, merge main.
- Run `python3 scripts/work.py territory --base origin/main`.
- Read `docs/prompts/implementer-discipline.md` in full.
- Never arm a waiter or a background chain for a build or a test.
  Anything that outlives 600 s runs `setsid`-detached and is polled in
  the foreground.
- CI is the verification of record. The diff seeds `geom-core`, so
  allow 30 min.

## Report

≤120 lines, covering:
- the scoped door's shape and its ledger row;
- the first probe's numbers;
- the bracket's new readings on the notch, with budgets;
- what retired, and the `Certified::Neither` answer;
- every golden or pin that moved, with why;
- the PR number and its CI result.
