---
id: blend-reach-crossing-is-a-sole-bracket-door-missing-from-the-bounds-roster
kind: issue
title: blend/reach.rs's crossing is a sole-bracket bound door missing from the bounds roster, so main's bounds census is red
status: open
opened: 2026-11-18
priority: P1
cost: E
---

Found by lane `embed/conic-vertex` (EMBED, 2026-11-18), running
geom-core's census rows; the bounds census is in the slow set and the
PR gate does not select it.

`geom-core::all bounds_census::every_sole_bracket_bound_door_is_in_the_roster`
fails on a checkout at main `763d6fe0`:

```text
arrived or moved (owe a roster line WITH its disposition):
  ("crates/sweep/src/blend/reach.rs", "crossing")
listed but no longer present (delete the line): []
```

The same shape as `blend-reach-displaces-is-a-sole-bracket-door-missing-from-the-bounds-roster`
(closed by PR 4959): a sole-bracket door in `blend/reach.rs` landed
without its roster line. Not traced to the PR that added it.

**What closes it:** `crossing`'s roster line with its disposition, or
the door routed through a named bound.

**Traced (EXCH triage, 2026-11-20).** `crossing` arrived with PR 7078
(`band/mixed-corner-reach`, merge `36c7cb2b66`). The bounds census
passes on `36c7cb2b66^1` and reports `("crates/sweep/src/blend/reach.rs",
"crossing")` on `36c7cb2b66`. Still red on main `bd8b2c9bb5`, together
with `sure_below` (DOCTAIL's
`main-red-bounds-census-misses-reach-rs-crossing-and-sure-below`, PR
7114). Reproduce:
`cargo nextest run -p geom-core --ignore-default-filter -E 'test(every_sole_bracket_bound_door)'`.
