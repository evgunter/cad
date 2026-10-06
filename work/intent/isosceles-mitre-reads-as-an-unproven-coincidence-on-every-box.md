---
id: isosceles-mitre-reads-as-an-unproven-coincidence-on-every-box
kind: issue
title: D10: a blend's isosceles-mitre verdict on an extruded box is decided from values, and D10's structural test, which compares carriers, cannot prove the two equal angles one construction
status: open
opened: 2026-10-06
priority: P0
cost: M
---


Filed by BAND at Ev's request on `[ev]` PR #4085 (decision 3: "can you
file this on `intent`? i want to work with that agent to decide the
behavior we want here"). The behaviour is Ev's to decide with this
program; nothing here is ruled.

## The situation

PR 4085 (BAND, the plane–plane blend run-out fork; fork-log row 72)
builds a MITRE where two requested edges of a trivalent vertex turn and
the third, L, is unrequested: the two bands meet along their
intersection (`crates/sweep/README.md`, "Where a straight band ends, and
where it turns"). When the trihedron is isosceles about L — equal
dihedrals at the two requested edges, the same fact as equal face angles
at the vertex — the mitre lands on L and splits it (one curve, a
valence-4 vertex); otherwise one band overruns past the mitre (two
curves). Which holds is a margined verdict: Zero, definite, or the sliver
band refusing.

On every extruded box and every extruded cap rim the verdict decides
Zero, because both side walls are perpendicular to the cap — one
construction. But the verdict is decided from values and makes pieces of
one result touch (the two band ends and L meet in one vertex), so under
D10 ("every coincidence the kernel infers from values … is recorded at
the one door where structure is decided") it is recorded for the
`unproven-coincidence` lint. D10's structural test compares CARRIERS in
canonical form (a plane modulo in-plane motion, an axis modulo slide and
spin); it says nothing that proves two ANGLES between carriers equal. As
written, every box mitre would report as unproven — noise on the
commonest blend there is.

## Options the BAND designers and orchestrator saw (not an option set)

- (a) The structural door learns the relations a construction fixes
  between its outputs — e.g. an extrude's side walls are each
  perpendicular to its cap; a revolve's walls are coaxial — so the
  isosceles verdict on a box is proven structurally.
- (b) Record it and accept the noise until a later rung (the symbolic
  tier's identities) proves more.
- (c) Do not record this verdict at all — which contradicts D10's
  every-inferred-coincidence rule as written.

BAND's orchestrator recommended (a), landing the mitre meanwhile under
(b). The same question will recur wherever a kernel decision depends on
an equality of angles rather than of carriers (a fillet's equal-radius
cylinder pair is a sibling: `cylinder_cylinder_section`'s doc wants the
radius equality structural or declared).

## Where it bites

Not yet built: the mitre is step 4 of PR 4085's build order. BAND will
land the cut-off steps first; the mitre waits for this row's answer or
lands under (b).

## Answer (Ev, 2026-10-06)

**Now: (b).** BAND lands the mitre and records the verdict as a value-decided coincidence. Nothing reads that record until stage 4 builds the `unproven-coincidence` lint.

**Stage 4: output definitions plus rung 3, not node theorems.** D10 already makes a node an operation that defines its outputs (stage 2). With a face's carrier defined as a formula over the node's inputs, any relation between outputs follows from those definitions, so no node has to state it.

Take an extrude in direction `d` over a profile edge with tangent `t`. The cap normal is `d`, and the wall normal is `cross(d, t)` normalised. The cosine of the dihedral is then `dot(d, cross(d, t))`, which is identically 0. Comparing canonical forms (rung 2) does not see that identity, but polynomial-identity reduction (rung 3, the symbolic tier) proves it. This is the first case that needs the "extend to (3) later if necessary" Ev allowed when ruling D10.

Stage 4 measures this on box mitres. The expressions are small, and only coincidences decided from values are checked. Per-node theorems, meaning an operation listing the identities it guarantees, are kept only as a possible cache. One is added only where the algebra is measured too slow or undecidable, because a second hand-written description of an operation can drift from its code.

The fillet sibling is already structural under rung 2: two cylinder radii that read one variable compare equal.
