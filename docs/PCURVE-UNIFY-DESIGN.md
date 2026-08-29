# Pcurve unification (#427) — M9-D design conversation

STATUS: **RATIFIED (Evan, PR #514 comment 5303556411,
2026-08-15): U2 as scoped; the scaffold = MappedCurve retained as
a description SOLELY for pre-body edges (narrow, tightly fenced)
— **fence criterion CORRECTED 2026-08-27, Evan ratifying in chat
after P-1's substrate: the boundary is TRANSIENCE, not "pre-body".**
`MappedCurve` measurably reaches REST through `describe_minted_edges`
and six fillet strut sites, so "pre-body" never fenced it. Evan's
Q2 choice — narrow `MappedCurve` rather than a dedicated `Scaffold`
rung — is UNCHANGED and was not revisited ("i think i don't want to
revisit Scaffold"); only the doc's description of where the fence
falls was wrong. Legal as a description only through the scaffolding
door; tier 3 refuses it at rest;
Seam folds in as drafted. Q3 (the authority record's home) adopted
by dominant argument, its pushback window closed unexercised at
M9's ratification (#1041): per-edge KERNEL data — forced because
tier-3's prefer-intrinsic enforcement
(validate.rs's TransverseNotIntrinsic/TangentNotIntrinsic) must
read the record replacing MappedCurve's negative space, and the
naming layer is editor-core, invisible to the kernel; the same
layering argument that moved ContactClass down. Scheduling
(delegated): the migration is the PCURVE program,
`docs/PCURVE-PLAN.md`.** This is M9-D (M9-PLAN,
ratified #509): the ratification pass that must precede any code,
sequenced before M9-3's seam minting hardens new edge
descriptions. Substrate: fresh exploration 2026-08-15 (file:line
evidence below is from it). The issue's ask: grow `Pcurve` to
general curves-in-UV, make `EdgeGeometry` reference
(surface, pcurve), demote `MappedCurve` to an authority/provenance
record rather than a geometry class.

## What the substrate settled (facts, not options)

- **Persistence is a non-cost.** Nothing in geom-brep/topo
  serializes; descriptions and pcurve caches are in-memory,
  re-derived (D8/D9). The migration is a design cost only; the
  at-rest constraints are STEP adoption's bitwise reproduction
  (adopt.rs's ladder must still match native descriptions) and
  D9 bit-replay of the mint pass.
- **Each special certification class buys its whole-domain sup
  from an exactness fact a general curve does not have**:
  Harmonic — both sides in span{1, cos, sin, t}, four
  coefficients, between-samples corruption UNREPRESENTABLE;
  IsoLine — the iso IS a copied control row, one spline space,
  partition-of-unity sup with zero snap slack; IsoArc — the
  carrier re-expressed in B's own knots/weights, difference a
  convex combination, structure read as exact f64 (C6). The one
  general class (Fitted) is weaker IN KIND (OnLocusHull on
  periodic analytic charts), narrower in scalars (no Dual), and
  every mass-props/tessellation consumer refuses it today.
- **MappedCurve has a load-bearing use #427 does not mention**:
  the pre-body Euler-op SCAFFOLDING (line_between /
  self_loop_circle_at) certifies via MappedSource with NO surface
  in existence yet — a provenance-only MappedCurve leaves those
  edges without any description class.
- **Planar faces store zero pcurves by ratified design**
  (CURVED-DESIGN C4, pinned by test): a chart-anchored
  EdgeGeometry either reverses that (speculative caches) or keeps
  a derive-on-demand door.
- **C7 item 3 needs almost no description machinery** — the
  tier-3 marks (Tangent / SmoothUnderdetermined) and their
  enforcement predicates already exist; M9-4 is mark-wiring under
  any ruling that adds no new geometry class.
- **CURVED-DESIGN OQ4** decided carrier-primary with a recorded
  counterweight ("if post-M5 work goes heavily trimmed-NURBS,
  pcurve-primary would have been leaner — declined with eyes
  open"). #427 is that counterweight coming due; the ruling must
  say whether OQ4 re-opens.

## Options

**U0 — status quo, grow by variant.** Every new lane adds an
EdgeGeometry variant + a certifier + arms at ~10 dispatch sites
(the "classic bug farm" D2's own background paragraph names);
#498's diagonal/interior loci have no representational home.
Rejected as the standing answer, kept as the null.

**U1 — full unification (one general curve-in-UV subsumes the
specials).** REJECTED on the exactness argument above: it trades
unrepresentable-by-construction certificates for sampled hulls,
loses the bitwise control-row equality adoption matches on, and
strands quadrature/tessellation (Harmonic-only and refusing-on-
Fitted consumers) on every wall.

**U2 — unify the DESCRIPTION, keep the certification lanes
(PROPOSED).** `EdgeGeometry`'s conventional variants collapse to
one form: **(surface, `Pcurve`)** — while `Pcurve` KEEPS its
exact variants as certification lanes (Harmonic / IsoLine /
IsoArc / Fitted) and gains a `General` curve-in-UV arm certified
at the honest Fitted grade. `IsoCurve`, the M8-4 boundary-
Intersection form, cap rims, and (likely) `Seam` become
(surface, exact-lane pcurve) instances — the classes survive as
what they really are: exactness certificates, not taxonomy.
`MappedCurve` demotes to an AUTHORITY RECORD carried beside the
description (sketch provenance recorded, not derived), with the
tier-3 prefer-intrinsic rules reading the record instead of the
negative space. A `Scaffold` rung (or MappedCurve retained as a
description ONLY pre-body) covers the Euler-op scaffolding.
Planar faces keep zero stored pcurves: derive-on-demand becomes
THE single door rather than one of two taxonomies. OQ4 is NOT
re-opened — carrier-primary stands; this unifies descriptions,
not the primary geometry.

## Sequencing consequence (part of the ruling)

Under U2 the MIGRATION is its own post-ratification unit (not
M9-core code): M9-3 mints within today's taxonomy but chooses
emission shapes that map 1:1 onto (surface, pcurve) — the design
pass exists precisely so M9-3 doesn't harden against the target —
and **M9-4 collapses into M9-3** (mark-wiring only). Lily wall
8's `CurvedEdgeUnsupported` does NOT resolve here —
`gate_operand_edges` refuses on the edge CARRIER's kind, which the
migration never touches (PCURVE-PLAN P-3); #388 takes its option (a)
unblocked; #498 inherits `General` as its named home when the
migration lands.

## Questions for Evan

1. U2 as scoped — sign off, or push back on keeping `Seam`
   spatial (its mirror-nappe caveat is the one variant with a
   genuinely non-chart definition; the draft folds it in but it
   could stay a peer)?
2. The scaffold: a dedicated `Scaffold` description rung, or
   MappedCurve surviving as a description solely for pre-body
   edges (one variant, tightly fenced)?
3. The authority record's home: per-edge data beside the
   description, or naming/provenance-layer data? (The record is
   what makes sketch truth recorded-not-derived — its home
   decides who reads it.)
4. Migration scheduling: post-M9 kernel candidate as drafted, or
   pulled into M9 after M9-3 if the milestone has room?
