---
id: editor-core-keeps-computed-values-in-t
kind: issue
title: The mate fold's two step arms take the lever arm as a constant and do not cancel the levered sine's arm
status: open
opened: 2026-10-10
priority: P2
cost: E
refs: [the-from-f64-gate-reads-every-generic-crate]
---

`editor-core/src/mate/coset.rs`'s `Arm::of(parts, datum)` sums two
inputs: the copy's reach, and the farthest a mate's poses sit from
their origins (`mate/solve.rs`'s `bundle_arm`). Both are upper bounds
read out of resolved enclosures, in `f64`.

**Re-read against stage 3 C (#5267, `683f44c3c0`), 2026-10-18.** C
moved four of the six sites this item named onto
`Real::from_computed`, so at `Sym` each is its own unknown and no
identity is proved over a constant arm:
- `mate.rs`'s `Lever::margin` and the chart's antipode chord
  (`mate.rs:908`, `:1176`);
- `coset.rs`'s `parallel` and `perpendicular` (`coset.rs:607`, `:634`).

The gate's row narrowed with it: `scripts/gates/from-f64-sites.tsv:13`
lists the two left.

**What is owed.** Two step arms in `coset.rs`'s anchor step
(`coset.rs:1615`, the two-plane arm, and `coset.rs:1655`, the two-line
arm) still re-enter the arm through `T::from_f64(arm.get())`, a
literal. They divide by the `sine` that `parallel` minted. That sine
holds its own arm, a `from_computed` opaque. So `arm / sine` cancels the
arm only to rounding: the step's literal and the verdict's opaque are
two different nodes. The same `Arm` reaches both (`intersect` passes one
arm to `table` and to the step, `coset.rs:1237-1260`), so the fix is
local to `coset.rs`:
- mint the arm in `T` once per `intersect`, through `from_computed`;
- thread that one value through `table`, `parallel`, `perpendicular`
  and the step;
- keep the `f64` `Arm` for `decides_over` and the refusals.

Pin it with a row at `Sym` in which a step's `arm / sine` leaves
no arm node standing; the corpus reaches no levered mate.
`from-f64-sites.tsv`'s row then goes.

Keeping the arm in `T` from the start (rather than as a bound read out
to `f64`) is not owed. The arm is a lever length, an upper bound by
construction, and an upper bound read off an enclosure is
`from_computed`'s own case (`geom-core/src/real.rs:324-330`).
