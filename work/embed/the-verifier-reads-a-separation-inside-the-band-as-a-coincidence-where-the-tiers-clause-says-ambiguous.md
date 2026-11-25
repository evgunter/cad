---
id: the-verifier-reads-a-separation-inside-the-band-as-a-coincidence-where-the-tiers-clause-says-ambiguous
kind: issue
title: The embedding verifier answers Crosses for two boundary points a separation inside the band apart, where [topology.tiers] says a reading inside the band refuses as ambiguous
status: open
opened: 2026-11-22
priority: P1
cost: M
---


Filed by the EMBED orchestrator, 2026-11-22, from `embed/strip`'s trace
of one verdict that moved (PR 7334).

## What was seen

`contact9_side_codes::a_sector_parallel_at_a_short_arm_is_coplanar_only_if_its_bounds_read_on`,
the `∩` result: a five-face sliver wedge. Its top is a piece of the
block's `z = 0` face (`9v3`), its bottom the needle-wedge's tilted face
(`8v7`, tilted 5e-8 rad). They share the 1 mm tip edge `17v1`.

Under PR 7334 the embedding ask on that solid reads `Crosses`, with the
witness `(5.009576, 5.001016, −4.78e-10)` on `8v7` and the point above
it on `9v3`: 9.6 mm from the shared edge, 4.78e-10 m apart, which is
the wedge's true thickness there (0.0096 m × 5e-8 rad). The thickness
is positive everywhere off the tip edge and below ε (1e-9) for about
20 mm from it. Before PR 7334 the same ask refused `Budget(Pairs)`; the
strip presentation lets the walk reach the pair.

So the solid does not pass either way, and the engine did what its doc
says (`topo::clearance::embedded`: "decided at the strict site, where a
separation in the band is a coincidence"; `Embedding::Crosses`: "two
points of the boundary that are not one point of it, which the verifier
decided coincide").

## The question

`docs/DESIGN.md` `[topology.tiers]`: "Pieces of a solid's boundary that
do not meet (no shared edge, vertex or recorded contact) are more than
K·ε apart. Where faces do meet, or a face is read against itself, the
local reading … passes by more than the band. … A reading inside the
band refuses as ambiguous, never as Holds or Violated."

These two faces do meet (they share an edge), and 9.6 mm from it they
are a separation inside the band apart. Is that:

- a coincidence, so the body is told it meets itself there (what the
  verifier answers today, and what "a separation in the band is a
  coincidence" says); or
- a reading inside the band, so the body is told the question is
  ambiguous there (what the clause's last sentence says of a reading
  inside the band)?

Both are rejections, and Ev's answer on PR 7319 (a body that does not
pass may be told a different rejection, as long as it is never
accepted) covers the difference in kind. What is open is which text a
user reads for a wall thinner than ε, and whether the clause's
"never as … Violated" is about the local reading only or about every
reading the question makes.

## Not this row

Nothing here is PR 7334's defect: the verifier is untouched by it. The
census holds the same shape among its crossings (a seam between faces a
small dip apart), so the population is larger than this one test; it is
not counted here.

## 2026-11-22, from PR 7334's review: main's row asserts this body valid

The reviewer reproduced the three bodies through
`topo::clearance::embedded`: `∩` reads `Crosses` (faces `8v7` and
`9v3`, 4.78e-10 apart about 1 cm from a 1 mm seam, dihedral 5e-8 rad);
`−` refuses `Uncertified / Departure` at an edge (`1v1`, `7v3`); `∪` is
`Embedded`.

**The contact9 row asserts the opposite of the door about `∩`**: its
`answers` requires `∩` to pass tier 3 with exactly those corners, and
its doc says the seam "rests as their `Intersection`". So a body main's
suite asserts valid at tier 3 is rejected by the embedding question.
The reviewer: "a conflict between tier 3 and the door, uncovered by
this PR and not caused by it. It needs a ruling before the gate lands;
I would not merge the gate with it open."

**What `[topology.tiers]` already says of it.** The finished-body
clause does not admit this body: where faces meet, "the local reading
(how the faces leave their shared edge or vertex …) passes by more
than the band", and that reading is the edge's wedge levered at the
faces' reach. Two faces leaving a 1 mm seam at 5e-8 rad are within ε of
each other out to about 2 cm. So the body is not a finished body under
the clause as it stands; what tier 3 passes today is short of the
clause (its check 4 reads the wedge at stations; `embed/remeter`'s
filed RESTFRONT row is that difference), and the test's assertion is
about tier 3 as it is, not about the clause.

So two things, neither a new decision:
- **For the gate's landing** (`embed/gate`): this test's `∩` is a body
  the gate rejects. It is pinned as what the gate reads, with this row
  named, not left as a failing test.
- **The open question stays the text**: `Crosses` against "ambiguous",
  as above.

Band raised to P1 because the gate's landing meets it.

## The witness body moved, 2026-11-22 (EMBED, lane strip)

Two readings at two heads of `embed/strip` (PR 7334), through the gate:
`Crosses` at `5953a2a38` (the first head), at a station pair 4.78e-10
apart; `Embedded` at `a54717c96` (the end reading, the strip arm
removed). On PR 7334's fix pass, the ∩ of
`contact9_side_codes::a_sector_parallel_at_a_short_arm_is_coplanar_only_if_its_bounds_read_on`
reads `Embedded` through the gate (`embed/strip-measure`). Its two faces
share a 1 mm edge and part at 5e-8 rad. Its `Crosses` came from the
distance test at a station pair 4.78e-10 apart, 9.6 mm from that edge.
The end reading now presents the cells near the edge. A departure
certifies them, with the faces' parting multiplied by the cell's
reach, the margin `embed/remeter` (PR 7329) retires. So this body no
longer shows a `Crosses` at a separation inside the band at the strip
head. What it reads under the re-metered margin is the remeter lane's
to say and pin. The verdict is not pinned in PR 7334.
