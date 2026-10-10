---
id: interference-at-rest-is-a-finding
kind: issue
title: D10 stage 5 PR B: interference between copies is its own finding, quiet only under a one-sided Gap assertion whose own faces bound the overlap
status: parked
opened: 2026-10-08
priority: P0
cost: H
blocked_on: [select-defines-face-and-edge-variables]
---

INTENT stage 5, PR B. Ev approved the design in PR 4319 (fork log row
98, FORK-S5Q); `docs/INTENT-STAGE5-SPEC.md` §3 predates it, and where
they disagree this row governs.

D10: at rest an overlap of two copies' material is an interference
finding, and nothing at rest refuses. Today the census's containment arm
(`crates/topo/src/census.rs:5634`) pushes
`ValidationError::InstanceInterference`, and A5's gate attributes it
`Unattributed` (`crates/editor-core/src/assembly.rs:1791`) and refuses
`AssemblyError::AtRest`. A transverse pierce between two copies refuses
as `UndeclaredContact { EdgeFacePierce }`.

B partitions those verdicts out of the gate's refusal into an
`InterferenceFinding` per site. **The site** is a connected overlap of
the two copies' material, named by the faces bounding it; regions of one
carrier pair are one site.

**What quiets it.** B lands the quieting rule's one home
(`checks/at_rest.rs`) with its interference half. A holding assertion
quiets an interference when it reads a `Gap`'s output directly, over an
opposed pair of faces of the two copies, admits only negative values
(`≤ b` or `= b` with `b` negative), its two faces bound the overlap, and
every face bounding the overlap lies between the carriers of an asserted
pair. So a second lug bore on the same carrier needs its own assertion,
and every other new overlap is loud by default. `Distance = 0` quiets
nothing. An overlap the kernel cannot intersect is a loud interference
that nothing quiets, not a could-not-look finding. A witness point is
not a slot on `Assert`; it waits for a future pointed local `Gap` arm.

The kernel is unchanged except for one typed `SameSide` field. ASSEMBLY
A5 *Interference.*, topo C6's invariant and MATE-4B's "EdgeFacePierce
stays categorical" are re-worded at the at-rest door.

It needs no stage-3 or stage-4 work. It needs stage 2's copies (C),
single-primitive measures (D) and selections (E) to state a site.

The contact half of the rule (stage 5 C's) needs stage 4 to record a
face-on-face planar contact as the face pair: the census records planar
contact at vertex and edge level, and no `Vertex` selection exists, so
without that lift a planar contact could never be quieted.

Ev, 2026-10-08, on a pattern's outputs: "we should have a map higher
order function to allow declaring over all the outputs of the
pattern." A `map` over a pattern's `Bodies` covers member against
another copy: one assertion written once, one per member, each reading
the one bound variable, as many as the pattern's `Count`. Overlap
between members (neighbours in a ring) needs a map over pairs of
members, adjacent or all. D10's Assertions paragraph states it; the
maps are `a-map-over-a-patterns-bodies-asserts-once-per-member`. This
unit's quieting rule is per finding, so a mapped assertion quiets
exactly as the one it expands to would.
