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
`Indeterminate` from being built by hand. Four have landed since the
first census (2026-10-03):
- `vtxfac.rs:549` (`bool_sector_coplanar`, PR 3513);
- `boolean/mod.rs:1238` (`unglued_coincidence`, INTENT stage 4 E);
- `chart_region.rs`'s `definite_diag` callers;
- `boolean/join.rs:2969` (`pt_germ_frame`,
  `bool_join_torus_section_touching_under_a_germ`, the plane×torus
  germ frame): the section table, levered, routes a pose to its general
  rung while the section certificate, unlevered, reads it touching or
  empty. Neither reading hands back the margin it decided on, so the
  `straddle` door has no bounds to take, and the escalation goes
  through `invalid_margin::invalid`. It landed while unit 4 (PR 4613)
  retired that helper's census caller, so the helper stays.

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

## Residue: a hand-built verdict handed to a gate door (unit 4, PR 4613)

The two passes above cannot see a `Decided` built by hand and handed to
`k_stats::gate_rejection`, which mints on the log with a margin no
classifier reached. One production site:

- `crates/topo/src/boolean/recl.rs:1503` (`resolve_bisector_graze`):
  the bisector's `On` rides a `PairRecord` code, which keeps no margin
  (a coplanar pair's codes are `On` with no bound read), so the Zero is
  quoted as `±zero`. Carrying a margin means a margin per code on
  `PairRecord` (96 uses). The `vtxfac` caller carries its bisector's
  own reading. Pinned by `recl::tests::a_grazing_bisector_between_one_sided_keys_refuses`.

The gate should count `Decided {` literals that reach `gate_rejection(`
or `straddle(` in production code, beside the two counts above.

## Slate triage (2026-10-26, CLEAVE)

- Moved to CLEAVETAIL (`work/cleavetail/`) by `git mv`, id and body unchanged: the track opened for CLEAVE's P3 and P4 tail: refusal stories, guards that cannot go red, latent soundness and hygiene. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
