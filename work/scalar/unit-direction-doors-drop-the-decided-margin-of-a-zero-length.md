---
id: unit-direction-doors-drop-the-decided-margin-of-a-zero-length
kind: issue
title: The unit-direction doors drop the decided margin of a zero length, so levered sites hand-spell decide-then-normalize
status: open
opened: 2026-10-06
priority: P3
cost: M
---


## What

`crates/geom-core/src/linalg/unit_vec.rs` `decide_direction_by`
(behind `UnitVec3::new` and `UnitVec3::levered`) maps a decided
`Zero` or `Negative` length to `UnitVec3Error::Degenerate`, which
carries no payload. A site where a decided-zero length is reachable on
honest input, and has to refuse with its decided margin and the
tolerance that margin offers (`k_stats::decide_positive`'s rejection),
cannot use the door. Minting an `Indeterminate` for the `Degenerate`
arm by hand would be the outside-the-funnel shape filed at
`work/cleave/topo-mints-indeterminates-outside-the-funnel.md`. So
each such site hand-spells "decide the levered length, then normalize",
and returns a bare `Vec3` rather than a `UnitVec3`:

- `crates/topo/src/boolean/recl.rs` `flank_rep` — `bool_flank_offset`
  (`decide_positive` of `Margin::levered(off.norm(), reach)`, then
  `off.normalize()`);
- `crates/topo/src/boolean/vtxfac.rs` `pierce_germ_dir` —
  `bool_germ_line` (`decide` of `Margin::levered(int.norm(), span)`,
  then `int.normalize()`);
- `crates/topo/src/boolean/sectors.rs` `pair_search` —
  `bool_faces_parallel` (`decide_magnitude` of
  `Margin::levered(int.norm(), arm)`, then `int.normalize()`).

Two filed rows prescribe `UnitVec3::levered` and will meet the same
gap once their decided zero must refuse with a margin:
`work/cleave/sector-face-cylinder-normal-normalizes-an-undecided-radial.md`
and `work/shelf/replace-face-cylinder-radial-normalizes-an-undecided-rejection.md`.

## Shape

A door that keeps the decided margin on refusal — e.g. a
`Degenerate(Indeterminate)` arm, or a gated variant whose non-positive
rejection is the funnel's (`decide_positive`'s), minted where every
other escalation is — then a later unit moves the sites above onto it.
Found by the `cleave/recl-flanker` review (PR #4177).

## Re-homed from FLUX to SCALAR (2026-10-10)

(FLUX orchestrator) FLUX measured 125.5 budget points against 30 and was cut on its priority seam: FLUX kept the curved closed-form arms. SCALAR collects the scalar doors and the readers over them: the certified-door family, the margin and recourse readers, the unit-direction doors, and the box driver's readings. The id and the body above are unchanged; the move may have set `priority`, `cost` or `status` in the header.
