---
id: a-boss-inside-a-hole-touching-along-short-arcs-breaks-a-germ-invariant
kind: issue
title: A boss inside a hole, tangent, with short arcs either side of the touch: the union refuses ClassificationInvariant
status: dispatched
opened: 2026-10-28
priority: P1
cost: M
---


Found by the review of PR 5949 (branch `splitdoor/crescent-short-arcs`).
Pinned by `crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`
`short_arcs_at_a_turned_tip_at_the_finest_band` (its `swapped` rows).

## The pose

The boss `r = 0.1` about `(0.2, 0)`, `z ∈ [0, 1]`, over the plate
`[−1, 1]² × [−0.5, 0]` holed through by `r = 0.3` about the origin, the
boss inside the hole tangent at `(0.3, 0)`, each rim cut at the tip and
either side of it (boss at azimuth `±3s`, hole at `±s`, `s = 3e-7`,
`1e-6`, `1e-5`), the tip turned to azimuth 0, 0.7 or 2. The boss and
the plate meet in no volume; they touch along the run of short arcs.

`A ∪ B` and `B ∪ A` refuse at the default ε and at 1e-12 (`s = 3e-7`):

    ClassificationInvariant { what: "germ record without a clean In/Out transition after rewrite" }

raised at `crates/topo/src/boolean/recl.rs:632`. The intersections are
empty and the differences build `SOUND`. The refusal is the same with
`crescent`'s short-arc skip turned off, so it predates PR 5949. Without
the short arcs (one vertex at azimuth 0 or none at the tip) the same
union builds at its closed form with tier 3′ failing (`OK BAD
t3p=false`; which findings it fails on is unmeasured), the reading
`a_boss_over_a_hole_by_pose` pins as `Reads::Undecided` for its
`the boss inside the hole, tangent` pose.

An invariant breach is a bug report, not a typed refusal of an
unsupported shape: the union is two solids touching along a coincident
run, which tier 3′ represents.

## Slate triage (2026-10-28, SPLITDOOR)

- Banded P1, priced M: a legal union refuses with a kernel invariant (`ClassificationInvariant`) rather than a typed frontier, on verb breadth beyond the everyday shapes; `work/README.md` Priority.
