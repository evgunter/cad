---
id: hand-minted-indeterminate-ratchet-gate
kind: unit
title: no gate stops a new hand-minted Indeterminate before the seal: pin each file's production count so it only falls
status: open
opened: 2026-10-10
priority: P3
cost: E
parent: topo-mints-indeterminates-outside-the-funnel
refs: [the-indeterminate-seal]
---

Unit 7 of `topo-mints-indeterminates-outside-the-funnel`'s re-scope
(2026-10-10, main `98a3817a1d`).

## What

The seal (`the-indeterminate-seal`) is parked on rows in six programs
and on D10 stage 4. Until it lands, nothing stops a new production
`Indeterminate` from being built by hand. Three have landed since the
first census (2026-10-03):
- `vtxfac.rs:549` (`bool_sector_coplanar`, PR 3513);
- `boolean/mod.rs:1238` (`unglued_coincidence`, INTENT stage 4 E);
- `chart_region.rs`'s `definite_diag` callers.

## Shape

A gate in `scripts/gates/`, beside `reporting-margin-door.sh` and
reusing `lib.sh`'s test-only resolver. It pins a per-file count of
production `terminal_sliver:` literals (every `Indeterminate` struct
literal sets that field) and of `invalid_margin::invalid(` calls. A count
may fall but not rise.

- **Start the counts** from the files the re-scope's census names: 68
  arms in `topo` and 1 in `sweep`. The literal count is lower than the
  arm count, because one helper or closure can serve several arms.
  Re-measure on the branch.
- **Name what it cannot see:** a `use … as` alias, a `..diag` relabel,
  and a mint in a crate it does not scan.
- **Each unit that retires a site** lowers its file's count in the same
  PR.
