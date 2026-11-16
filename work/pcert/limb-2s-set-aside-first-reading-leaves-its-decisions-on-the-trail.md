---
id: limb-2s-set-aside-first-reading-leaves-its-decisions-on-the-trail
kind: issue
title: Limb 2's first reading that clears and is then set aside leaves its decisions on the trail, which the rounds take again
status: open
opened: 2026-11-12
priority: P4
cost: E
---

Filed by the lane that closed
`check-4s-projected-door-reads-every-span-of-a-chord-in-eight-parts`
(PR 6864, its review's S1).

## What

Two readers take a first reading of a net at `Parts::ByTurn` and fall
back to eight parts a span. They treat the first reading's decisions in
opposite ways.

- **Check 4** (`projected::run_projected_checks`) runs its first reading
  under `k_stats::detached` and `sym::aside`. It keeps the decisions only
  when that reading passes; otherwise it discards them, so the trail is
  the deciding reading's alone.
- **Limb 2** (`pcurve_cache::limb2_offset_sup`, its `cleared` closure)
  takes `offset_cleared`'s decisions on the trail directly (the window's
  ends, a cone's or torus's lever per part). It does so only once the
  `f64` bound clears, but a reading whose lever then refuses is set
  aside for the rounds, which take those decisions again. Its own
  comment says the trail then shows them twice.

The k_stats escalation log has a reader that acts on it: the
subdivision driver (`k_stats::splice_superseded`'s docs). An escalation
that a set-aside reading left there names a cause the limb did not end
on.

## Fix

Run limb 2's first reading the way check 4 runs its own: under
`k_stats::detached` and `sym::aside`, splicing only a reading that is
the limb's. A row would hold limb 2's trail to the deciding reading's
on a first reading that clears in `f64` and then refuses its lever.
