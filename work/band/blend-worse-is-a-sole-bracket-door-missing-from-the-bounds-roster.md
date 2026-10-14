---
id: blend-worse-is-a-sole-bracket-door-missing-from-the-bounds-roster
kind: issue
title: blend/surgery.rs's worse<T: Bounds> landed without a bounds_census roster line, so every_sole_bracket_bound_door_is_in_the_roster is red on main
status: closed
opened: 2026-10-06
priority: P0
cost: E
pr: 4153
closed: 2026-10-07
---


Found by the `shell/planar-gate-misses` lane (PR 4115), whose CI run
37458992415 (head `474ce863`, tested as its merge with main) went red on
it. **Measured**. Reproduced on `7c6ad8a4`, which is PR 4115 plus
`origin/main` `20f054a3`; PR 4115 touches neither file:

`CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p geom-core every_sole_bracket_bound_door_is_in_the_roster`

    the sole-bracket-bound roster no longer matches the tree.
      arrived or moved (owe a roster line WITH its disposition): [
        ("crates/sweep/src/blend/surgery.rs", "worse"),
      ]

`fn worse<T: Bounds>` (`crates/sweep/src/blend/surgery.rs`, "The worse
of two margins of one cycle") arrived in `653b93be` ("sweep: one
per-piece home for ring reading; each cycle refuses at its least
margin"). It reads `b.lo() < a.lo()` to choose which margin's `bounded`
flag rides with `a.min(b)`. That is a bracket read selecting a payload,
so `crates/geom-core/tests/bounds_census.rs` owes it a `Site` row with
its DL5(b) disposition, beside `surgery.rs`'s other rows
(`CircleFrame`, `piece_along`, `piece_distance`, `boxed_reach`).

Writing that disposition is the blend surgery's own argument, so the
lane that found it did not guess it. Every PR whose CI runs `geom-core`'s
census is red on this row until the line lands.

## Closed

The roster line landed in PR 4153 (`6014eda8ab`), and was ported again in
PR 4128 (`723fbdeb0a`). `crates/geom-core/tests/bounds_census.rs` carries
`subject: "worse"`, with its `Selection` disposition: the bracket read
only selects which `bounded` flag rides with the `min`, and both inputs go
to `ring_clearance`, whose DL5(b) disposition it inherits. Every PR since,
#4143, #4092 and #4209 included, ran the census green.
