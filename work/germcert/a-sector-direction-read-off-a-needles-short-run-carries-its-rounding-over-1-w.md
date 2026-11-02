---
id: a-sector-direction-read-off-a-needles-short-run-carries-its-rounding-over-1-w
kind: issue
title: A needle channel turned off the axes refuses the reduction's sector question at ε 1e-9, its direction 4.7e-9 rad off its edge, from a source not yet located
status: open
opened: 2026-10-25
priority: P2
cost: M
---


Found by FLUSH's needle-cavity lane (`flush/needle-cavity-section-loop`,
2026-10-25) while pinning
`a-needle-cavity-refuses-at-the-joins-section-loop-before-any-read`.

**What happens.** The needle channel of
`shell_role_reader::a_needle_cavity_builds_at_every_definite_width_and_holds_it_as_a_void`:
the brick `(−1,2)³ ∖ slab (0,1)×(0.5,0.8)×(0.5,0.5+w)`, then `∪` the
fill `(0,1)×(0.5+w,0.8)×(0.5,0.5+w)`, and the bar alone (`slab ∖ fill`,
`slab ∩ (brick ∖ fill)`). Turned about `(1, 2, 3)` by 1.1 rad, at
ε 1e-9, every one of the three refuses `Escalated` on
`Coincidence(Sectors, _)` from w = 12ε up (12, 15, 21, 30ε). The
predicate is `bool_sector_within` (`boolean/sectors.rs`
`within_signs`), and `bool_sector_coplanar` at 12ε. The same
operands build axis-aligned at every width, and turned at ε 1e-6.
The union also refuses the same way at 10.5–21ε when its axis-aligned
operands are moved by `transform_rigid` rather than built with their
corners turned (it builds at 30ε). So fitting the planes from corners
is not the cause.

**Measured** (15ε, turned, the `within_signs` call that refuses).
The sector is the fill's floor at its corner: `start` is −x, `end` is
+y and `normal` is −z, all turned and exact to rounding, with
`arm = 0.3 − w`. The direction read against it lies within
4.7e-9 rad of −y rather than within rounding of it: `c2 = 4.7e-9`,
levered to 1.41e-9 at the arm. Across the widths the margins are 1.4,
1.7, −1.0 and −1.4e-9 (moved pose, 10.5–21ε). They do not follow `1/w`
across the widths. They are absent where the coordinates are exact,
and they fall away at ε 1e-6, where w is 1000× wider. Their size,
about `u·|p|/w ≈ 7e-9` rad held at a 0.3 arm, is that of a direction
read off a run w long, but that is a hypothesis. This
lane did not locate where that direction is minted (`insert::germ_dir`'s
`n_a × n_b` reads two face normals, which are exact here; a pierce or
section direction off two points w apart is the likelier source).

**Why it is owed.** The pair this reads is not in band: the direction
is along the fill's own edge, and the axis-aligned pose builds. If the
direction is read off a run w long, [verdicts] makes it a defect: "a
formula whose error is amplified by a vanishing quantity … is a
defect, re-posed or certified, never covered by K".

**What is owed.** Find the direction's source. Re-pose it, either read
off the long run it lies along or levered at the reach it was read
over. Then flip the turned ε 1e-9 rows of the pin above from refusing
to building.

## Moved by PR 6053 (2026-10-30)

PR 6053 reads a direction's side of a bound at the shorter of the two's
own reaches rather than at the sector's arm. On its head,
`crates/topo/tests/shell_role_reader.rs`
`a_needle_cavity_builds_at_every_definite_width_and_holds_it_as_a_void`
at ε 1e-9 builds every run but the turned 12ε channel's five, which
still refuse `bool_sector_coplanar`. On main, the turned channel from
12ε up and the moved channel below 30ε refused `bool_sector_within` or
`bool_sector_coplanar`. The turned leaning channel
(`a_needle_channel_whose_fill_face_leans_reads_its_side_off_its_seam`)
builds at ε 1e-9 too. Every run that builds is held to the rows'
checks (tier 3′, volume, corners).
