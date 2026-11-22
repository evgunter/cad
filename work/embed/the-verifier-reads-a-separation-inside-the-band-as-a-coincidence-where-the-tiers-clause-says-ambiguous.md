---
id: the-verifier-reads-a-separation-inside-the-band-as-a-coincidence-where-the-tiers-clause-says-ambiguous
kind: issue
title: The embedding verifier answers Crosses for two boundary points a separation inside the band apart, where [topology.tiers] says a reading inside the band refuses as ambiguous
status: open
opened: 2026-11-22
priority: P2
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
