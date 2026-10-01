---
id: conic-plane-sweep-examines-only-the-first-root
kind: issue
title: The sweep's conic × plane lane examines only the first root, so a second root inside the face is dropped unrecorded (premise S of the section certificate fails)
status: closed
opened: 2026-09-28
priority: P0
cost: E
closed: 2026-09-28
branch: germ/conic-plane-every-root
refs: [torus-face-meeting-a-partner-only-in-an-interior-loop-while-crossings-exist-elsewhere]
---

## What

`reduce.rs` `sweep_direction`, the conic × plane root lane
(`if let Some(&t) = roots.first()`, about line 841 on `2ba90bced`).
`conic_plane_crossing_roots` returns up to two interior roots in
ascending order, and the lane classifies only the FIRST against the
face. When that root lands `Out` of the face it falls through to
"No interior root: endpoint processing only" and `continue`s, so a
second root that lands `In` (or on the face's boundary) is never
examined. The edge's contact with the face there is neither recorded,
nor certified a miss, nor refused.

`wall_crossing`, the curved-face sibling, already loops over every
root ("Every root is examined, and the FIRST interior one wins").

This breaks premise S ("every box-overlapping edge × face pair is a
certified miss, a recorded contact, or a typed refusal"), which the
section certificate's lemma L1 rests on
(`docs/GERM-SECTION-CERTIFICATE-SPEC.md` §0, §1), and which the spec
asserted from `curved_face_arm`'s "never a silent fallback" without
walking the planar lane.

## Measured (GERM section-certificate lane, 2026-09-28, on `2ba90bced`)

Fixture: A = `common::germ_pair::cyl(1.0, 1.0)` spun by `s` about
`z`; B = the box `x ∈ [−1.5, 0]` (or `[−1.5, −0.3]`),
`y ∈ [0.5, 0.7]`, `z ∈ [−1.5, 1.5]`. A's rim arcs cross the plane
`y = 0.5` at `x = ±0.866`; B's `y = 0.5` face holds only `x = −0.866`.
B's vertical edges at `x = 0` pierce A's caps, so the op has crossings.

- `s ∈ {0, 0.3, 3}`: ∪, ∩ and A∖B all refuse
  `Join(UnpairedLooseEnds { count: 4 })`.
- `s ∈ {1, 2, 4, 5}`, and the mirror box `x ∈ [0, 1.5]` at every `s`:
  every op answers its closed form (∪ `6.864499`, ∩ `0.318686`,
  A∖B `5.964499`), `point_in_solid` agreeing at `(−0.75, 0.6, 0.9)`.
- Scratch change (reverted): `for &t in &roots` in place of
  `if let Some(&t) = roots.first()`. Every refusing case above then
  answers the same closed forms as its mirror.

So the dropped root is the whole cause of the refusal, and the refusal
itself is the join noticing loose ends downstream, not the sweep. Here
the result is loud. Nothing at the sweep guarantees that: a
certificate that reads "no event on this pair" as "no boundary
contact" (the section certificate's no-event decision, W3) is exactly
what a dropped root can fool.

## Fix

Loop over `roots` as `wall_crossing` does: an `Out` root continues to
the next, the first `In`/`OnEdge`/`OnVertex` root splits and re-queues
against the same face. The rows are the fixture above at `s = 0`,
answering its closed forms under ∪, ∩ and ∖.

## Closed — every root is examined (germ/conic-plane-every-root)

`sweep_direction`'s conic × plane lane loops over `roots` as
`wall_crossing` does: an `Out` root continues to the next, the first
`In`/`OnEdge`/`OnVertex` root splits and re-queues both fragments
against the same face (so any other root is found again on them), and
only when no root lands in the face does the lane fall through to
endpoint processing. The trace pushes `accepted` once, for the root
that splits, exactly as before.

Rows: `crates/sweep/tests/germ_conic_plane_roots.rs` — the fixture at
`s ∈ {0, 0.3, 3}` for both `x` windows (`[−1.5, 0]`, `[−1.5, −0.3]`)
under ∪, ∩, A∖B and B∖A, each at its closed form to 1e-9 relative
and `point_in_solid` agreeing at five witness points. All 24 refused
`Join(UnpairedLooseEnds { count: 4 })` on the base.

The one other row in the `topo` and `sweep` suites that reaches a later
root is `pcurve_p1b_r2_probes`'s tube pocket (a full-revolve tube minus
a slab), which skips a refusal: it moves from
`Join(UnpairedLooseEnds { count: 4 })` to the full-period wall's
containment escalation (`bool_wall_trim_period`), the frontier
`reach/full-period-wall-has-no-containment-verdict` already tracks.
