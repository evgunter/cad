---
id: main-red-the-bounds-census-misses-two-sole-bracket-sites-in-reach-and-separation
kind: issue
title: Main red: geom-core bounds_census every_sole_bracket_bound_door_is_in_the_roster finds two unrostered sole-bracket sites, sweep blend/reach.rs (crossing) and topo separation.rs (misses)
status: open
opened: 2026-11-25
priority: P1
cost: E
---

Filed by INSIDE's orchestrator (2026-11-25) on the report of the
`inside/quartic-disc-rounding` lane (PR 7754), which replayed it red in a
clean `origin/main` worktree. Not re-run by the orchestrator.

`crates/geom-core/tests/bounds_census.rs`
`every_sole_bracket_bound_door_is_in_the_roster` names two new sites
missing from its roster:

- `crates/sweep/src/blend/reach.rs`, a site the census reads as
  `"crossing"`;
- `crates/topo/src/separation.rs`, a site it reads as `"misses"`.
  `separation.rs`'s recent history is ORBIT's separation work
  (`orbit/separation-turned`, 2026-11-23 to 24), the likely source;
  the `reach.rs` site's source is not traced.

The lane that added each site owes rostering it (or the census's
reason it is not a sole-bracket door). No other row held it at filing
(grep of `work/` for the test name).
