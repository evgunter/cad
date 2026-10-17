---
id: blend-reach-displaces-is-a-sole-bracket-door-missing-from-the-bounds-roster
kind: issue
title: blend/reach.rs's displaces is a sole-bracket bound door missing from the bounds roster, so main's bounds census is red
status: closed
opened: 2026-10-13
priority: P0
cost: E
closed: 2026-10-13
pr: 4959
---


Found by the FLUX lanes (`flux/iso-parse-incidence`, PR 4932), whose
`test` job went red on the merge ref with a failure their diff does
not touch. It reproduces on `origin/main` alone (`95d344176`):

```
cargo nextest run -p geom-core -E 'test(every_sole_bracket_bound_door_is_in_the_roster)'
the sole-bracket-bound roster no longer matches the tree.
  arrived or moved (owe a roster line WITH its disposition):
    ("crates/sweep/src/blend/reach.rs", "displaces")
```

`displaces` arrived with `15b75526e` ("band: a feature wholly inside a
ruled cut-off's sliver goes with it (bore ruling, step 3)", PR 4896).
`crates/geom-core/tests/bounds_census.rs` wants a roster line for it
with its disposition, as `blend-worse-is-a-sole-bracket-door-missing-from-the-bounds-roster`
gave `worse`.
