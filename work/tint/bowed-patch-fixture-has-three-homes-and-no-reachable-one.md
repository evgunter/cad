---
id: bowed-patch-fixture-has-three-homes-and-no-reachable-one
kind: issue
title: The bowed-patch fixture has three homes and no crate all three can reach
status: open
opened: 2026-09-21
---


## What

One `0.15 u(1−u) + 0.1 v²` patch over `[0,1]²`, in three crates:

- `crates/topo/src/fixtures.rs`, `bowed_patch` — amplitude `1.5e-2`
  since LANE-0's fix pass, because it goes through the `Tol` door and
  has to certify at the `1e-12` eps row;
- `crates/sweep/tests/verbs_offc_consumer.rs`, `bowed` — amplitude
  `0.15`, minted through the `_at` instrument at a fixed target, so a
  seven-round fit;
- `crates/geom-brep/tests/approx_surface.rs`, `bowed` — amplitude
  `0.15`, likewise at a fixed target.

Same shape, three amplitudes, three copies of the control-point loop.

## Why it is not fixed where it was found

Found by LANE-0's second review (PR 2981, R2's style Q1) and left by
its fix pass, because no crate all three can reach holds it:

- `topo::fixtures` is `#[cfg(test)]` and private, so it does not exist
  in the `test-support` build the other crates would import; moving
  the fixture into `topo::test_support_fixtures` would reach sweep but
  not `geom-brep`, which sits BELOW `topo` and cannot depend on it;
- `test-utils` is the leaf every crate can take, and it has zero
  dependencies on purpose (its `Cargo.toml` says so, and that is what
  keeps it below `interval-transcendentals` too) — it cannot name
  `geom::NurbsSurface`;
- a `test-support`-gated fixture in `geom-brep`'s own `src` would
  reach all three, at the cost of putting a test fixture on a
  production crate's public surface for two outside consumers.

The third option is the only one that folds all three, and it is a
decision about `geom-brep`'s surface rather than a tidy-up.

## The shape of the fix

Either take the third option — `geom_brep::test_support::bowed_patch(bow)`
behind the same gate `topo::test_support` uses, the three call sites
passing their own amplitude — or decide that a nine-line control-point
loop is cheaper repeated than exported, and say so once where the
copies are.
