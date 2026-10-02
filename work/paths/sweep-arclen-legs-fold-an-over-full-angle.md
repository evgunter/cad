---
id: sweep-arclen-legs-fold-an-over-full-angle
kind: issue
title: the tangent arc legs Sweep and ArcLen gate the angle positive only — θ ≥ 2π is admitted and FOLDED (a zero-chord vertex at 2π; tan(3π/4) = −1, a CW semicircle, at 3π)
status: closed
opened: 2026-09-16
refs: [2135]
priority: P0
cost: H
closed: 2026-10-02
pr: 3774
---

Found by BOOL-10's second review (PR 2135, R2 MINOR-5) on the split-form
head and confirmed by the first (R1 executed θ = 2π): `Sweep{angle}` and
`ArcLen` refuse only a non-positive angle, so an author can write a sweep
of 2π or more and the leg folds it through `tan(θ/4)` — at exactly 2π the
emitted vertex has a zero chord ("turn margin −0 m on a 0 m arm"), at 3π
the bulge is −1, a clockwise semicircle the author did not ask for.
Pre-existing (the leg predates BOOL-10); out of BOOL-10's fence, filed by
the S-BOOL orchestrator. The fix is a typed refusal for |θ| ≥ 2π at the
leg (D2: a full turn distinguishes nothing an author can mean by a sweep;
`circle`/`circle_split` are the closed-carrier spellings) with red-first
rows at 2π − ε, 2π, 3π on both scalar lanes. Difficulty S.

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to PATHS (opened at this exit as S-BOOL's successor for the profile lattice) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.

## Closed (2026-10-02, #3774)

`Sweep` and `ArcLen` refuse an angle not definitely short of 2π, typed
`PathError::ArcSweepPastFullTurn` (Python tag `arc_sweep_past_full_turn`),
decided by `path_arc_sweep_full` on the arc length `r(2π − θ)`, so the
band is the run's linear one. The rows are in
`crates/profile/tests/arc_leg_full_turn.rs`:
- 2π − 0.01 builds;
- 2π − ε/(2r), 2π and 3π refuse.

They run for both modes, at `f64` and `Interval`, and are red with the gate
removed. The stored sweep is `4·atan(tan(θ/4))` of the authored angle, which is θ
over the reals now that θ < 2π.
