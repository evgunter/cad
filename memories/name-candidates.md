---
name: name-candidates
description: Q9 project-name candidates — Evan's shortlist (Intension, Noumenon/Noumena, Selvage) with justifications + the full rejected/backup slate and availability as of 2026-07-23
metadata:
  type: project
---

Q9 (project name) is still open; Evan reviewed candidates 2026-07-23
and shortlisted three to revisit later. Naming brief he gave: a subtle
nod to what makes this kernel different — the FUNCTIONAL/INTENSIONAL
core (the object *is* its definition; everything else derived from
it), everything explicit, no assumptions/silent fudging.

## Shortlist (Evan-liked, all crates.io-AVAILABLE 2026-07-23)

- **Intension** — the philosopher's term for exactly the architecture:
  the definitional content of a thing vs its *extension* (the point
  set); the kernel keeps intensions and derives extensions on demand.
  Already ratified vocabulary (D2 "intensional EdgeGeometry"). Triple
  pun (Evan): reads as near-miss "intention" (apt — F2's explicit-
  intent condition), and "in tension" is apt for mechanical design.
  GitHub: alandipert/intension (★166 Clojure lib) is the only notable
  neighbor.
- **Noumenon / Noumena** — Kant's thing-in-itself vs phenomena: the
  B-rep is the noumenon, every render/mesh is phenomenal. Louder
  version of the same metaphysical joke. Both singular and plural free
  on crates.io; only tiny GitHub repos.
- **Selvage** — the woven edge that cannot fray → watertight
  boundaries by construction. Evan likes it a lot but judges it
  slightly tangential to the intensional core. crates.io free; tiny
  GitHub collisions only.

## Liked but namespace too crowded (Evan)

- **Witness** — witness midpoints, GQ1 authoritative-branch witnesses,
  certified bounds; internally the most resonant word, externally
  generic/unsearchable.
- **Genus** — double pun: genus–differentia definition theory +
  topological genus (load-bearing in the kernel). Common-word
  collisions.

## Other candidates (for possible revisit)

- **Scruple** — archaic small unit of measure + refusal to cut
  corners; both senses load-bearing (exact measurement, no fudged
  verdicts). crates.io AVAILABLE, GitHub clean. Was the first round's
  top recommendation.
- **Quiddity** — scholastic "whatness"; the kernel stores what the
  object IS. Distinctive, same archaic register as Scruple.
  (Availability unchecked.)
- **Definiens** — the defining clause of a definition; names precisely
  the artifact the kernel persists. Very obscure. (Unchecked.)
- **Constructive-logic vein** (unmined): the ethos is constructive in
  the mathematician's sense — existence needs explicit witnesses;
  trilean predicates are a working rejection of excluded middle. If
  the nod should aim at the logic rather than the metaphysics, mine
  here.

## Ruled out

- **Carvel** (watertight hull planking — VMware's Carvel k8s toolchain
  collision), **Eidos** (Platonic form — Eidos Interactive trademark),
  **Manifold** (known CSG library + half-wrong: 3′ results are
  deliberately pseudomanifold-tolerant), **Noumenon-adjacent
  "Ansatz"** (connotes assumption — the opposite ethos).
- crates.io TAKEN: dyad, dyadic, trit, plumb, ambit.

Before ratifying any pick into Q9: re-run the crates.io/GitHub sweep
(fresh — the 2026-07-23 checks go stale) and a trademark sanity look;
then DESIGN.md Q9 closes + crate-prefix rename as a mechanical PR.
