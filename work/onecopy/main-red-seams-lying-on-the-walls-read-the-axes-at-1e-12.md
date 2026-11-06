---
id: main-red-seams-lying-on-the-walls-read-the-axes-at-1e-12
kind: issue
title: main red at 1e-12: sweep parallel_cylinder_join::seams_lying_on_the_walls_read_the_axes_across_the_region (U ab: OK BAD … v=8.858311527)
status: closed
opened: 2026-11-06
priority: P1
cost: E
branch: onecopy/seams-1e-12
closed: 2026-11-06
pr: 6550
---


Found by the PR 6491 takeover lane (`onecopy/meet-pose`). It is red at `CAD_TOLERANCE_EPS=1e-12` on main `264f1407`, with a panic byte-identical to the head's (`U ab: OK BAD … v=8.858311527`). No work row named it. It is in the slow set, so the per-PR gate never runs it.

## What closing it takes

1. Reproduce it on current main at 1e-12.
2. Bisect to the commit that moved it, or find that it was never green at 1e-12.
3. Read the panic: which pair, and which reading.
4. Fix the cause at its home, or re-pin it with the reason.

## Found (2026-11-06)

1. **Reproduced** on main `2a1b1a3a` at 1e-12, in 101 s. The panic is byte-identical.
2. **Bisected** to `6adc5fdd` ("chorded rows read their pad against the resolution's own bound"):
   - its parent passes at 1e-12;
   - `6adc5fdd` itself fails with the same panic.

   That commit swapped the row's `outcome` (the shared `VOLUME_PAD_BUDGET`, 1e-7) for `judged_within(…, resolution_pad(tol, 10π))`, which is `1024·ε·area/2`: 1.6e-8 at 1e-12. The row has been red at 1e-12 since its ceiling arrived.
3. **The panic.** The turned pose about `x`, op `U ab`, the first run the row reads. Its volume is right (`v = want` to 10 digits), but its enclosure's half-width is `3.5e-8` against the 1.6e-8 ceiling.
   - The measurement stopped at `Stop::Wall`: the chorded walls reach the work wall before the pinned resolution, 1024·ε = 1e-9 m.
   - **All six turned-`x` runs stop at the wall at 1e-12**, with pads `3.50e-8` to `4.02e-8`, so each is over the ceiling. An earlier version of this section said only `U ab` was; it was the first to panic.
   - `resolution_pad`'s premise is that every face met the resolution, so its ceiling does not apply to a wall stop. `differential::judge` read the enclosure without asking which stop it reached.
4. **Fixed at its home**, `sweep/tests/common/differential.rs`:
   - `Budget` is the one rule, with `Budget::ceiling` and `Budget::verdict`.
   - `judged_at_resolution(r, want, tol, area, stopped_short)` holds a measurement that met the resolution to `resolution_pad`. One that stopped short is held to `stopped_short`, the row's own measured guard. Within the guard, with every check passing and the enclosure holding the closed form, it reads `Verdict::Unmet`; otherwise `BAD`. The line names the stop.
   - `Judgement` carries the stop and the ceiling applied, so a row that checks a pad by hand (the tilted rod's ∪, which tier 3′ refuses) reads the same rule.
   - The two rows that read the resolution's ceiling now read it through this:
     - `parallel_cylinder_join::seams_lying_on_the_walls_read_the_axes_across_the_region`, under `TURNED_WALL_GUARD = 1.7e-7` (four times `4.02e-8`);
     - `tang_circle_cylinder::a_tilted_rod_through_a_rim_builds_at_its_sliced_volume`, under `TILTED_ROD_WALL_GUARD = 6.0e-6` (four times `1.49e-6`).
   - Each admits `UNMET` only off the witness ε, as it admits an escalation there. At ε 1e-9 each still needs `SOUND`, under its regression guard.
   - Pinned: `tang_circle_cylinder::a_stop_short_of_the_resolution_is_held_to_its_guard`. A wall stop with a pad of 1e-3, whose enclosure holds any nearby volume, reads `BAD`.
   - `judged` and `judged_within` read as before.

**Sibling.** `tang_circle_cylinder::a_tilted_rod_through_a_rim_builds_at_its_sliced_volume` is red at 1e-12 on main for the same cause: every op stops at the wall, with `∖` and `∩` at pad `1.49e-6` and `∪` at `1.26e-7`. It was red there before `6adc5fdd` too, whose parent held it to `1024·ε` (`1.02e-9` at 1e-12). It passes here.

**Left as found.** The work wall at 1e-12 on chorded walls is a cost frontier, not a wrong number: every enclosure holds its closed form. Its home is `work/quadtail/chorded-walls-reach-the-work-wall-short-of-the-pinned-resolution-at-1e-12.md`.
