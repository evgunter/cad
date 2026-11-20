---
id: three-step-round-trip-helpers-share-one-hand-written-volume-budget
kind: issue
title: Three STEP round-trip helpers state one volume budget three times, each with a hand-written 1e-12 slop
status: open
opened: 2026-11-17
priority: P4
cost: M
---

Found in review of EXPORT's `export/void-shells` (PR 7077, finding S5).
Three helpers each take a body through STEP and back and hold the
volume to the source's, and each states the budget on its own:

- `crates/step-import/tests/hollow_bodies_round_trip.rs`, `round_trips`:
  `|v_back − v_built| ≤ pad_back + pad_built + 1e-12·|v_built|`.
- `demos/tour/src/lib.rs`, `step_round_trip`: the same overlap of two
  certified enclosures plus `1e-12·|V|`, written again.
- `crates/step-import/tests/verbs_chamfer_roundtrip.rs` (~:112):
  `|got − want| ≤ 1e-12·want`, with no pad at all, so it is only sound
  while both enclosures are closed-form (pad 0).

The `1e-12·|V|` slop is a fold-order allowance the three sites assert
without deriving. `roundtrip.rs`'s committed-corpus row derives the
same allowance properly, from `topo::test_support::fold_order_bound`
over each side's `Σ|tᵢ|`. Either the three should read that bound, or
one shared helper should hold the budget once with its derivation. The
tour cannot depend on test-only crates, so a shared helper would have
to be a public measurement on the import or `topo` side, which is why
this is filed rather than fixed in the PR that found it.

Moved from TINT to CIW on 2026-11-20, by `git mv`, when TINT merged into CIW: TINT held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one CIW carries.
