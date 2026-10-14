---
id: the-sweep-doors-check-their-results-only-in-debug-builds
kind: issue
title: extrude, revolve, loft, sweep, fillet and chamfer check their results only by a debug-build tier-2 assert, so nothing stops one returning a body tier 3 refuses
status: open
opened: 2026-10-14
priority: P3
cost: M
---


Found by CLEAVE's sweep for `a-split-along-a-seam-at-a-vanishing-tilt-returns-an-invalid-body`
(branch `cleave/seam-split-invalid-ok`). That unit found the split
returning sides that `validate_geometric` refuses, because its result
gate was tier 2 alone (`validate_closed`), and gave the split the
Boolean's at-rest gate (`AtRestPolicy::gate_at_rest`,
`SplitFinishError::ResultNotAtRest`).

The sweep doors go further: none gates its result at run time. Each ends
at a `#[cfg(debug_assertions)] debug_assert_eq!(topo::validate_closed(..), Ok(()))`,
so a release build checks nothing, a debug build panics rather than
refusing, and neither reads tier 3. Each returns a plain `Body`:

- `sweep::extrude` (`crates/sweep/src/extrude.rs`);
- `sweep::revolve`, full and partial, and the tubes over it
  (`crates/sweep/src/revolve/full.rs`, `revolve/partial.rs`, `revolve/tube.rs`);
- `loft_body`, `loft_profiles` and `sweep_body` (`crates/sweep/src/loft.rs`'s
  shared assembly);
- `sweep::fillet_edges` and `sweep::chamfer_edges` (`crates/sweep/src/blend/surgery.rs`).

The Boolean (`topo::boolean::ops::gate`), `topo::shell` and STEP import
gate at tier 3. The topo offset doors and `replace_face_offset` are
ratified construction state, tier 2 in and tier 2 out
(`crates/topo/README.md`, shell and offset surgery row). The sweep doors'
result bar is stated nowhere.

**Unmeasured:** whether any input reaches a sweep door's `Ok` with a
body tier 3 refuses. The suites mostly finish these results through
`AtRestBody::validate` (`sweep::test_support::finished`), which panics
on a refusal, so the corpus they cover does not. The editor holds every
node's body unverdicted and gates it only where a later node consumes
it (`editor-core` `finished_operand`), so a leaf node would show such a
body without a refusal.

**What closes it:** either gate each door's result at the scalar's
at-rest gate, typed as the Boolean and the split type it (a debug assert
is not a gate: D4's fail-loud refusal is a typed `Err`), or state in
`crates/sweep/README.md` that these doors return construction state and
make the editor gate a leaf node's body.
