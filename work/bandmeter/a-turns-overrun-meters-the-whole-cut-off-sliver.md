---
id: a-turns-overrun-meters-the-whole-cut-off-sliver
kind: issue
title: blend: a turn overrun's arm (c) meters the far band's whole cut-off sliver, refusing features the overrun clears and adding no refusal the reach does not make
status: open
opened: 2026-10-13
priority: P3
cost: M
---


At a turn's overrun, `turn_plan` (`crates/sweep/src/blend/open/planar.rs`)
plans the far band `k` cut off at the near band's face of L, `Fⱼ`,
through `cut_off_plan`, and hands that cut-off's whole sliver to the ring
pass (`crates/sweep/src/blend/surgery.rs` `blend_surgery`, the `turns`
chain into `ring_clearance_pass`, arm (c)). The README's turn clause
(`crates/sweep/README.md`, "metered as band `k`'s cut-off sliver
(arm (c))") is what that implements. Two reviews of PR 4926 measured
what it buys.

**It refuses features clear of everything the overrun takes.** The
whole sliver's bound `Ω` on `Fⱼ` is, at the bracket's `y = 0` corner
(`operands::leaning_turn(−1)`, `d = 0.1`, the wall frame `(along L,
across)` from the vertex), the triangle `(0, 0)`, `(√2·d, 0)`, `(0, d)`,
far larger than the overrun `(d, 0)`, `(√2·d, 0)`, `q = (d, (1 − 1/√2)·d)`.
A round boss of radius `0.004` at `(1.15·d, 0.4·d)` stands `0.13·d` past
the overrun chord's line and `0.11·d` past band `j`'s trimline, so
nothing the chamfer takes reaches it, yet it refuses `RingClearance`
(`bounded: true`) at the wall. Pinned by
`band_turn_overrun_oracles::a_boss_clear_of_the_overrun_inside_the_far_bands_sliver_refuses`.
The far band requested alone (a plain cut-off at the same wall)
refuses the same boss the same way, so the conservatism is the chord
sliver's bound; the overrun inherits it over a region it does not cut.

**It carries no refusal the reach does not already make.** Mutant: the
turn's sliver dropped from the ring pass (`.filter(|_| false)` on the
`turns` chain). With it:
- the clear boss above builds, at the feature-free `ΔV`;
- a boss in the overrun, a hole in it, a boss and a hole straddling the
  overrun chord all still refuse, as a definite `FaceClearance` from
  predicate 2's reach (`blend/reach.rs`), whose cap stays at `Fⱼ`;
- a void touching no face over the overrun refuses `WhollyInside` from
  the reach either way;
- features clear of the overrun still build exactly.

No row of the suite, nor either review's probes, found a feature only
arm (c) refuses; a flush or imprinted ring only arm (c) would see is
not ruled out.

**The fillet** (both reviews of PR 4962) hands arm (c) its own round
sliver, and the same mutant leaves every fillet feature row passing
(`band_turn_overrun_fillet::features_in_and_around_the_overrun_refuse_or_build_as_their_regions_say`,
`a_void_in_the_overruns_volume_refuses`,
`band_turn_overrun_fillet_model::features_about_the_arc_build_clear_and_refuse_where_they_straddle`):
straddling pins refuse through the reach (`FaceClearance` or
`FaceClearanceUncertified`) instead of `RingClearance`. So for the
fillet too arm (c) carries no refusal of its own. Its over-refusal did
NOT reproduce for the fillet at the bracket's `y = 0` corner: bosses and
holes inside the far band's circle clear of the arc, below `q` and
right of the circle build at the feature-free `ΔV` (`|Δ| ≤ 4.4e-16`),
because the round sliver beyond the overrun lies in band `j`'s strip
there. The bound's cost is the chord sliver's.

**Options.** Meter only the overrun's own triangle (`p_j`, `p_k`, `q`),
as a region of its own
or arm (d) widened past band `j`'s strip (the step-2 fork 3(b) in
`a-non-isosceles-turn-overruns-past-the-mitre.md`), which keeps a meter
on `Fⱼ` without the bound's cost; or drop the turn's sliver from the
ring pass and let the reach meter the overrun alone. Either changes
what the README's turn clause says arm (c) meters, so it is a revision
for Ev. The same choice settles
`an-overruns-sliver-refuses-a-short-near-edge`, whose refusal is this
sliver's foot on the near edge.
