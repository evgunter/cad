---
id: torus-rim-mint-abandons-a-half-applied-split
kind: issue
title: The torus rim mint bails with Ok(()) after `split_at_midpoint` has already mutated the solid
status: open
opened: 2026-09-11
---


Filed by S414's sweep (`crates/step-import/src/geometry.rs`, the conic
arm's finiteness ordering). The pattern swept for was "an early return
that precedes a validity check the later path performs"; this is the
only real hit in `crates/step-import/src/`, and it is a neighbouring
class rather than S414's — an early return that abandons a *mutation
already applied*, reporting success.

## The finding

`crates/step-import/src/normalize.rs:1251` halves the torus meridian:

```rust
let (first_m, second_m, mid_v) = split_at_midpoint(solid, meridian.edge, mint);
```

`split_at_midpoint` (`:205`) takes `&mut SolidSpec` and has already
split the edge and minted the antipodal vertex when it returns. The two
bailouts below it then leave that mutation in place and report success:

- `:1262` — `if !(radius.is_finite() && radius > 0.0) { return Ok(()); }`
- `:1272` — `let Ok((t0, t1)) = crate::geometry::endpoint_params(rim_id,
  &carrier, v1, v1, true) else { return Ok(()); };`

Either path returns a solid whose meridian is split and whose second rim
was never minted — a half-applied normalization the caller cannot
distinguish from "this shape needed no normalization". The other
normalizers in the file (`:868`, `:918`, `:981`, `:1167`) bail the same
way but all of them bail *before* touching `solid`, which is what makes
these two different.

`:1272` also discards a typed `StepImportError`. It is unreachable
today — `:1262` has just established `radius > 0.0` and finite, so
`spoke.dot(u_ref) == radius > 0` and the eccentric anomaly is finite —
so this half is a silent discard of a refusal that cannot fire, not a
live swallow. S414's change (the conic angle now refuses where it is
derived) does not make it reachable, for the same reason.

## Why not S414's unit

Different file, different class, and the fix is a call S414 was not
dispatched to make: either pre-compute `radius` and the carrier *before*
`split_at_midpoint` and bail with the solid untouched, or turn both
bailouts into typed refusals. That is a design question about what a
normalizer owes when its precondition fails mid-mint, which is above
this row's class.

## Fence

`crates/step-import/src/*` is EXCH's (Track U).
