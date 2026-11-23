---
id: an-adjacent-pairs-shared-vertex-is-recomputed-as-a-root
kind: issue
title: validation's pair pass recomputes an adjacent pair's shared vertex as a root of the carriers' intersection, then asks which root it got: six numeric decisions on a parameter bulge stand on that choice
status: open
opened: 2026-09-26
priority: P2
cost: M
refs: [the-apothems-sign-is-a-value-read]
---


## What

`Profile::validate`'s pair pass (`crates/profile/src/validate.rs`,
`judge_pair`) runs `seg::pair_contacts` on every segment pair, adjacent
pairs included.
- It builds `shared`, the pair's legal shared vertices, and does not
  pass it to `pair_contacts`.
- It only discounts afterwards a Touch that `contact_at_shared_vertex`
  finds within the band of a shared vertex.

So for an adjacent pair, `pair_contacts` solves the carriers'
intersection from scratch:
- For a line and an arc, `seg::line_arc` spells the two candidates
  `tc ∓ sqrt(r² − h²)`, with `tc` the foot of the centre's perpendicular
  on the line. It asks `line_span` and `arc_span` at each.
- `arc_arc`'s secant arm is the same shape, across the radical line.

One of the two candidates IS the shared vertex, and which one it is
depends on a sign. On R2's D-tab, where the lines adjacent to the arc are
perpendicular to its chord, that sign is the apothem's,
`sign(1 − b²)·σ`. To prove "this candidate is the shared vertex", the
symbolic tier must show `|apothem| = ±apothem`. So on a parameter bulge
these decisions land `numeric`:
- on the `0.5` parameter control, `line_span`, `arc_span` and
  `contact_at_shared_vertex`, two each;
- on the `0.4` parameter D-tab, `line_span`, two, which are frozen on
  `fl(0.4)`'s coefficient ring.

The early forms are in `crates/editor-core/tests/m10_bulge_renders.txt`
(on `props/sign-hull`). The attribution is
`work/decide/the-apothems-sign-is-a-value-read.md`.

## The fix

`pair_contacts` takes the pair's shared vertices.
- **The shared vertex is a contact by construction:** an endpoint of both
  segments, so a Touch at a shared vertex. Nothing is asked about it.
- **The other intersection is spelled from it, with no square root:**
  - line/circle: `2·foot − v`, from the sum of the quadratic's roots;
  - two circles: `v` reflected across the line through the centres.
- **Unchanged:**
  - the carrier predicates still decide tangent against secant, and a
    tangent adjacent joint has no second candidate;
  - `line_span` and `arc_span` are asked at the second candidate only;
  - non-adjacent pairs.

## What to measure first

- **What stops being asked:**
  - the six on the `0.5` parameter control;
  - the same joints' decisions on both `0.4` D-tabs, including the
    literal D-tab's six frozen on `fl(0.4)`'s ring;
  - the boss's two `sign_gated` `line_span`;
  - the bracket's fillet-joint `line_span` and
    `contact_at_shared_vertex`.
- **Whether `contact_at_shared_vertex` keeps a caller.** A Touch at a
  definite-secant pair's second candidate may always be far from the
  shared vertex. `SHARED_CLAUSE_ONLY`'s roster row holds it either way.
- **The second candidate's value bits,** which move. Re-baseline what
  moves.
- **Whether it moves the adjacent `line × arc` instances of
  `arc-arc-shallow-corner-legs-escalate-arc-span`** (P0): the same pair
  pass at a near-tangent joint. The arc × arc witness there is
  NON-adjacent (segments 0 and 2), which this fix does not touch.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILE (`work/profile/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam; PROFILE holds the goal (`docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.

## The line × line arm takes the shared ends (2026-11-20)

`pair_contacts` now carries an adjacent pair's shared ends as
`seg::SharedEnds` (which end of each is the stored vertex), and
`line_line` reads neither side nor span there; its contact is
`CKind::SharedVertex`, which `judge_pair` discounts unread. This row's
fix extends the same value to `line_arc` and `arc_arc`. Their
`end_touches` also reads the shared end against the other segment
(`touches` → `line_span` / `arc_span`), the same re-reading.

## Raise to P1 (2026-11-21, PR 7264's review)

PR 7264 left the curved arms reading the shared vertex. That leaves two
recognisers in one pass, and a user-facing audit can now see it:
- `line_line` takes the shared vertex by construction (`seg::SharedEnds`,
  `CKind::SharedVertex`), its collinear arm included;
- `line_arc`, `arc_arc` and `end_touches` still read it, and
  `judge_pair` still discounts their Touches through
  `contact_at_shared_vertex`.

`editor-core`'s `m4_pr6_eps_diff` golden now reports a `chord_side`
Zero → Positive shift that no reading made. The fixture's straight
branch no longer reads its shared vertices, while its arc branch's
line × arc pairs still read theirs as Zero sides, and the residual
pairing matches the difference. The golden's comment names this row as
the one whose fix removes the shift.

This lane did not take the curved arms. Their second candidate is a root
of the carriers' intersection, so recognising the shared end there is
this row's own restructure (the second candidate spelled from the
vertex, the bit moves measured), not a contained change. **The
orchestrator is asked to raise this row to P1**: it is the remaining
half of one recogniser.
