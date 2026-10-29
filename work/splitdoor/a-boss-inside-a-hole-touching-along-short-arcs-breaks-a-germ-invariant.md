---
id: a-boss-inside-a-hole-touching-along-short-arcs-breaks-a-germ-invariant
kind: issue
title: A boss inside a hole, tangent, with short arcs either side of the touch: the union refuses ClassificationInvariant
status: closed
opened: 2026-10-28
priority: P1
cost: M
closed: 2026-10-29
pr: 6005
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

## Closed (2026-10-29, PR 6005)

**The cause** sits at the vertex beside the touch, not at the tip. The hole arc's tangent lies 6e-7 rad outside the boss floor sector. `sectors::pair_search` decided the hit with `within` at the sector's 1.8e-7 short arm, where the gap reads Zero, but the side codes are read at each bound's reach, where they are definite Out. That left a record `mark_germ` could not rewrite.

**The fix:** `pair_search` records no pair when either sector's two side codes are the same definite side (`sectors::one_side`, shared with `recl::cancel_uniform`). A convex sector, under a half-turn, that lies wholly on one side of a plane meets it only at the vertex.

**Result:** the unions build as two shells at 1.890044257 at every ε, with tier 3 and the certificate passing. Tier 3′ fails only on `CensusUndecidable`.

**FULL review:** APPROVE. The reviewer ran topo and sweep with the slow set on both heads, plus a 2205-row pose grid. Only the 140 shared-vertex unions moved.

**Filed:**
- `a-held-germ-can-land-on-a-record-the-lump-cancelled`;
- `sector-decisions-read-at-the-arm-disagree-with-codes-read-at-the-reach` (the class).
