---
id: expr-non-finite-asks-value-times-zero
kind: issue
title: refuse_non_finite asks value·0, which reads [0,0] for an unbounded Interval enclosure, so an expression whose value runs to infinity passes the final check
status: closed
opened: 2026-10-13
priority: P2
cost: E
closed: 2026-10-16
---


`refuse_non_finite` (`crates/editor-core/src/expr.rs:1903-1911`) decides
an expression's finiteness as `value · 0` through
`k_stats::check_unlogged` and admits on `Sign::Zero`. At a point scalar
that is right: `∞·0` is NaN, which is poison. At `Interval`, an unbounded
enclosure such as `[MAX, ∞]`, `[0, ∞]` or `[−∞, ∞]` times zero is the
finite `[0, 0]`. A scratch row on SYM-17's branch showed this, and none
of the three is poison. So an expression whose enclosure runs to infinity
passes the final check as finite.

This is the same flaw SYM-17's delta review found at
`RigidMotion::check_rigid`'s translation probe (`0·tᵢ`; MINOR-1 on
`sym/17-review-delta`). PR #4197 fixed it there by asking
`Real::is_bounded` (`crates/geom-core/src/real.rs:385`), a required
method that decides both endpoints at an enclosure.

The fix is likely `value.is_bounded()` in place of the `value·0`
decision. The doc above the function says no `Margin` door fits, because
`value · 0` carries `value`'s dimension. `is_bounded` is not a band
decision, so that objection lapses with it. Moving off the K sample
`expr_non_finite` (F18) changes what that sample measures; say what moved.

Found by SYM-17's final pass, in the sweep for `x − x` / `0·x`
finiteness spellings.

## 2026-10-16 — closed (INTENT, `intent/j-census-reading-and-finite`)

**The fix.** `refuse_non_finite` asks `value.is_bounded()`. A row pins
it:
- `[MAX, ∞]`, `[0, ∞]` and `[−∞, ∞]` refuse;
- a finite enclosure and a finite point pass;
- NaN and ±∞ at a point refuse.

**What moved.** The check no longer decides, so the `expr_non_finite`
K sample (ledger row F18, retired in `docs/predicate-dimension-audit.md`)
is gone. Its theorems leave every `symbolic` count, and numeric, frozen,
registered and every verdict hold.
- Re-pinned: `m10_bulge_interval` (three tables), `m10_8_pins_interval`,
  `m10_9_pins_interval`, `sym_9_retry_interval`, `sym11_exact_channel_rows`,
  `m10_derived_frame_tilted_interval`, and `m10_sym_profile_interval`'s
  ledgers (calls out of `Plain/Decision`, a few forms now built under
  `Plain/Assertion`).
- `m10_bulge_renders.txt` is a historical dump no test reads, already
  stale before this, and is left as is.

**Sweep, for the `value · 0` / `x − x` finiteness shape.**
- `real.rs` `is_finite_length` (`x − x`): waves an overflowed norm
  through on purpose, as `Real::is_bounded`'s doc says. Not this unit.
- `real.rs` `is_exact_zero` (`x·0` not poison): the guard is the flawed
  shape, but the second clause `x·(1/x)` is poison only at `[0, 0]`, so
  no unbounded enclosure reads as zero. Correct, so not changed.
- `geom` `banded.rs` (`0·v`): reproduces the dense solve's NaN
  propagation bit for bit. Not a finiteness test.
- `topo` `offset_axial.rs` (`0·angle`): builds a typed zero angle. Not a
  test.
- `check_rigid`: asks `is_bounded` already (#4197).
