# VERBS-PIERCE — the curved pierce/split substrate (one PR)

The unit CYLCYL PR-B's opening measurement created (adjudicated
2026-08-26, addendum in `docs/VERBS-CYLCYL-SPEC.md`): the
cylinder-union refusals come from the CROSSING layer, not the
join — so the germ arms (CYLCYL's parallel-axis + Steinmetz,
SPHSPH's circle, and every later exact rung) all wait on the same
two unwired doors. Shared substrate, one PR, branch
`verbs/pierce`. Difficulty logged pre-dispatch: **M**. #347's
union half is the consumer of record; the arms themselves are the
NEXT unit, not this one.

## The two doors (as measured on #1044's opening table)

1. **`PointSplitCarrierUnsupported` — Circle-edge splitting.**
   Splitting an edge at an event point is wired only for `Line`
   carriers (exact point parameter). The Circle arm is the
   deliverable: an exact split parameter on the f64 lane, and on
   the interval lane whatever honest form the carrier supports —
   the PR-B lesson binds: `atan2`/branch-cut forms that need
   ordering are not available on `Real`; a bracket/subdivision
   derivation (or a typed interval-lane refusal with the reason
   at the site) beats a lane fork. Both halves of the split
   inherit descriptions/pcurves per the existing Line-split
   shape; census + validity pinned both lanes.
2. **`CurvedPierceUnsupported` — the pierce event path.** A rim
   circle definitely piercing a partner wall must produce the
   split-and-ring-insert the Line pierce already performs. Reuse
   the existing pierce machinery with the Circle split from door
   1; mint nothing new geometrically (the event point comes from
   the existing section/route answers — this unit routes and
   splits, it does not intersect).

## Fences

- **No join arms.** After this unit the four measured CYLCYL
  cases must reach the JOIN layer and refuse THERE (typed, naming
  the absent arm) — the doors move one layer down, honestly. The
  arms unit then flips them green.
- No sphere/cone work beyond what the split door shares by kind;
  no Steinmetz, no `SectionConic` widening.
- The D10 extent-certificate posture (PR-A) is not touched.
- STOPS for adjudication if the ring-insert half needs machinery
  beyond the existing Line-pierce precedents.

## Acceptance

- The coaxial-boss case (PR-B's table row 3) passes the split and
  refuses at the join, typed.
- Parallel-equal-r and coaxial-equal-r (rows 1–2) likewise reach
  the join layer.
- Planted reds: a Circle split at a poisoned/degenerate parameter
  refuses typed; the interval lane's posture pinned (build or
  honest refusal, both directions per the two-arm pattern where a
  measured constant gates).
- Existing boolean suites bit-identical; the #1044 conservatism
  rows untouched.

## Lane obligations

`docs/prompts/implementer-discipline.md` binds. No Co-Authored-By
trailer (blinding). Lane-private PR draft. Merge origin/main
before opening; confirm CI runs STARTED (a CONFLICTING PR gets no
run, silently — verify one fires); note the drawn point; watch to
completion; cancel detached timers before the final report; do
not merge.
