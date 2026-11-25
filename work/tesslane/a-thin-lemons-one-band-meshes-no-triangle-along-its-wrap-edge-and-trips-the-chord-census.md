---
id: a-thin-lemons-one-band-meshes-no-triangle-along-its-wrap-edge-and-trips-the-chord-census
kind: issue
title: A thin lemon's one band, narrower than the mesh δ, emits no triangle along its wrap edge and trips the chord census's debug assert
status: open
opened: 2026-11-25
priority: P1
cost: M
---


Found by the PR 7414 dual review (LATHE one-band lane), reproduced by
the lane on `lathe/full-revolve-one-band` at b5a35b27e7.

## What

A lemon (the spindle sheet `R < 0`, `r = 1`) revolved full about its
axis is one band whose wrap meridian runs apex to apex. At
`R = −0.99999` its half-width is about `√(2·10⁻⁵) ≈ 0.0045`, under the
mesh δ `1e-2`. `mesh::tessellate` at δ `1e-2` then emits no triangle
along the wrap edge's chord, and in a debug build the chord census's
assert fires (`crates/mesh/src/tessellate.rs:1001`, issue 897):

```
chord segment Some((0, 2)) is an edge of 0 face triangles, not two per
chord carrying it. ... (issue 897)
```

That is a panic, not a typed refusal; a release build skips the assert
and returns whatever the census lets through. On base, with the lemon
two half-bands, the same body refused typed (`OverusedSide`). At
`R = −0.999` and `−0.9999` the one band meshes (one patch).

## Fixture

In `crates/sweep/tests/the_football_revolves.rs`, whose `football`,
`y_axis` and `validated` build it:

```rust
let tol = Tol::witness();
let built = revolve(&validated(vec![football(-0.99999, 1.0)]), y_axis(), Revolution::Full, tol)
    .unwrap();
mesh::tessellate(&built.body, 1e-2, tol) // panics in debug
```

## Ground

`crates/mesh/src/tessellate.rs` (the chord census) and the curved
patch's sampling of a face narrower than δ: tesslane. Fail loud means
typed: a face that emits no triangle along an edge should refuse by
name, at least as `OverusedSide` did.

## Re-banded P1

- 2026-11-25 — re-banded P1 by the LATHE orchestrator on the PR 7414
  re-review (R2): the census that catches this is `#[cfg(debug_assertions)]`
  (#884's ruling for a row-5 state, S65), so with the root
  `Cargo.toml`'s release `debug-assertions = true` stanza removed at
  publish a release build would return this open mesh as `Ok`. Today
  every profile panics. Witness, profile-independent:
  `sweep` `the_football_revolves::a_thin_lemon_never_meshes_as_ok` (red
  on any build that returns the mesh as `Ok`). Whether the census should
  refuse typed in every profile for this state is a change to #884's
  ruling and waits on Ev; the ruling's own text ("if that floor is ever
  falsified, the state moves and so does the mechanism",
  `crates/mesh/src/curved.rs` header) is the question: a face narrower
  than δ is arguably row 2 (valid but unbuilt), not row 5.
- 2026-11-25 — the LATHE orchestrator's ruling: the mechanism stays
  under #884 (no typed refusal); this row is the fix, meshing the thin
  band. If the root `Cargo.toml`'s `[profile.release] debug-assertions = true`
  stanza is ever dropped, this case must be revisited under #884's
  "if that floor is ever falsified" clause, because a release build
  would then return the open mesh as `Ok`.
