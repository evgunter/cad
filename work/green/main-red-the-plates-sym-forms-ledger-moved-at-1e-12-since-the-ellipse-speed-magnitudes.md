---
id: main-red-the-plates-sym-forms-ledger-moved-at-1e-12-since-the-ellipse-speed-magnitudes
kind: issue
title: Main red (ε = 1e-12): editor-core m10_sym_profile_interval::the_forms_the_plates_walks_build_are_pinned reads a moved ledger since PR 7183 (germ/ellipse-speed-magnitudes)
status: closed
opened: 2026-11-21
priority: P1
cost: M
closed: 2026-11-22
pr: 7389
branch: green/editor-1e-12
---


Found by ONEROUND's lane (2026-11-21) on main `43080b1f`: `CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p editor-core --all-features --ignore-default-filter -E 'test(the_forms_the_plates_walks_build_are_pinned)'`.

## What

    plate at eps 1e-12: the walks built different forms (counts SymCounts { symbolic: 979, …, checks_expanded: 1047, checks_frozen: 0, numeric: 1047, retried: 0, frozen: 54 })
      left:  Plain/Decision calls 979 forms 39705 frozen 54 digest 3c078f72…   Plain/Assertion calls 1047 forms 14594 …   Early/Assertion calls 1047 forms 15164 …   Door/Decision calls 550 forms 15364 …
      right: Plain/Decision calls 979 forms 39703 frozen 54 digest 7efda4b6…   Plain/Assertion calls 1047 forms 14592 …   Early/Assertion calls 1047 forms 15162 …   Door/Decision calls 550 forms 15356 …

The call counts and the frozen counts are the pin's; the form counts are up by 2, 2, 2 and 8 and four of the five digests moved (`Early/Decision` is unchanged).

## The merge

`6d58cebd`, PR 7183 (`germ/ellipse-speed-magnitudes`): the row passes on its first parent `48d98d55` and fails on the merge, and with this same ledger on `d3012f9f`, `ca6dd709`, `0ad2e0b5`, `02804869` and `43080b1f`. It passes on `1360b9eb`, `46e89240` and `f2c38057`. So neither PR 7146 (`ca6dd709`) nor PR 7143 (`02804869`) moved it: the ledger is identical on each side of both.

## Owed (GERM, the PR's program)

Whether the plate's walks owe the extra forms (then the 1e-12 pin moves, with the reason) or the PR builds forms it should not. Not read here.

## Measured (GREEN U3, `green/editor-1e-12`, 2026-11-21)

The scheduled nightly 44870651337 (main `f5ef8f2b`) has this row red at every ε, not only 1e-12; at the default ε it is SCALAR's `main-red-the-plates-sym-walk-ledger-reads-2-to-8-more-forms` (P2), with the same five lines this row quotes. One pin serves every ε (`the_forms_the_plates_walks_build_are_pinned` reads no ε), so the two rows are one move. PR 7131 (`onehome/carrier-frames`, merged `9bc9d651` 11-21) re-pinned `PLATE_LEDGER` after that nightly, writing its move from 39703 (the old pin), not from the 39705 main then read.

### Read on hosted runners (2026-11-22)

Throwaway probe branches (`green/editor-1e-12-bisect-*`) ran the row at the default ε, 1e-6 and 1e-12, with an ignored probe row printing the profile's per-op tally (`SymProfile::ops`: built, frozen, terms out).

- **One move at every ε.** On PR 7183's first parent `45119994` the plate reads the old pin at all three ε; on its merge `6d58cebd` it reads `39705 / 14594 / 28232 / 15164 / 15364` at all three. On main `1aa4a931` it reads `33088 / 12224 / 22928 / 12674 / 12366` at all three, against the pin `33086 / 12222 / 22928 / 12672 / 12358`.
- **Which forms.** Across PR 7183 the op tally moves in two ops only: `Abs` 1535 → 1537 and `Max` 3332 → 3344, 14 forms, the four walks' +2, +2, +2, +8. Those are the ops PR 7183's conic speed readers add (`Circle::speed_lo` = `radius.max(0)`, `speed_hi` and `speed_at` = `radius.abs()`; `Ellipse::speed_lo` floored at zero, `speed_hi` = `larger.max(-smaller)`, in `crates/geom/src/curves/conic.rs`).
- **The revert probe.** On main with those five readers read as before PR 7183 (`radius`, `semi_axes_by_size().1` and `.0`), every count returns to the pin, and so do four of the five digests. `Plain/Decision` keeps its count and reads digest `36e6ee47…` against the pin's `40789891…`. That digest move is not PR 7183's; it is PR 7193's (below).
- **Verdict on the 14 forms: owed.** The readers are the PR's ratified design (GERM's log, the fork's part 1: `speed_lo` floored at 0 and `speed_hi` Interval-sound), so a bound stays sound over an enclosure of a size that straddles zero. Over the plate's symbolic radii each builds one more `Abs` or `Max` form; no call, no frozen count and no decision moves. Re-pinned at main's reading.

### The `Plain/Decision` digest is PR 7193's (PR 7389's fix pass, 2026-11-22)

PR 7389's review found that the re-pin also blesses a `Plain/Decision` digest move that is not PR 7183's. Bisected on hosted runners, the plate row at the default ε with PR 7183's speed readers reverted wherever they exist:

- **On PR 7131's branch** (`onehome/carrier-frames`, where the old pin was set). The old digest `40789891…` holds on `6811bc86`, `ec874955` (PR 7183 merged in), `c67c03e2`, `021fbc03` and `41bde16f`. `36e6ee47…` first appears on `5e5906b5`, a merge of main, and holds on `b65500a5`, and on 7131's own merge into main (`9bc9d651`).
- **On main, over the window that merge brought in** (`2bbfcdff` to `e9e4b731`). The pre-7183 ledger `39703 … digest 7efda4b6…` holds on `2bbfcdff`, and `aee23d66…` appears at PR 7193's merge `fb5d4552` (`sect/certificate-by-support`). Its first parent differs from `2bbfcdff` in no file under `crates/`. Same calls, forms and frozen count on both sides.
- **What moved, and why it is right.** A probe added an order-free sum of the walk's form digests beside the chain (`WalkProfile`, throwaway). Across PR 7193, `Plain/Decision`'s sum is the same (`8a713bd5…`), and so are its calls, forms and frozen count; only the chain moves. So the walk builds the same forms, each to the coefficient and term order, in another order. PR 7193 is where the closed-form limbs began deciding operand by operand in the order asked (its body: a refusal now names the plane, read first, where it named the cylinder), and the plate's holes are plane–cylinder pairs. A build order the PR changed on purpose, with nothing it builds changed, is a right reading of a digest that chains in build order.

`PLATE_LEDGER`'s move history is cut from the code (PR 7389's review, S3). It is recoverable above the constant at `60c1d63e88`, and each move's cause is on the row that re-pinned it.
