# M5 PR 8 spec (binding): the BVH crate + planar boolean-sweep wiring

Status: BINDING. Deviations reported, never improvised. Authority:
CURVED-DESIGN C10 (the ratified commitments, restated from
PERF-PLAN §2.1/§4.4/§5); M5-PLAN PR 8. Deps: PR 3 (merged; NURBS
control hulls exist). SSI-cell duty (C3 subdivision cells carrying
C9 enclosures) is PR 7's WIRING, not this PR — but the tree's API
must not preclude it (cells with payloads; document the seam).

## What this PR is

One new workspace crate `crates/bvh` (unprefixed name per
convention): a deterministic AABB tree with a conservative-
superset contract, plus its FIRST live consumer — the boolean
edge×face all-pairs sweep in `topo::boolean::reduce` (whose
quadratic is documented in its module docs as awaiting exactly
this), under the idealized/realized differential-suite discipline
(the pattern is only permitted WITH its suite — PERF-PLAN §4.4,
C10 verbatim).

## The tree (C10 commitments, binding)

- Deterministic: ARENA-ORDER build (input order is the iteration
  order; no hash iteration anywhere), a FIXED split rule (named
  and documented — recommend: median split on the longest axis of
  the centroid bounds, ties broken by lower axis index then lower
  arena key; whatever ships, the rule and its total tie-breaks
  are documented at the definition and D9-cited), no parallel
  build in v1, fixed leaf-size constant.
- **Conservative-superset contract** (the load-bearing clause): a
  pair query may only PRUNE pairs whose boxes definitely do not
  interact under the padded box test; every pair the exact
  predicates would accept MUST survive. Box overlap tests use
  outward-safe comparisons (f64 comparisons on box bounds are
  fine — boxes are conservative by construction; no Q1 predicates
  inside the tree, it decides nothing semantic — say so in module
  docs, citing D9's "results a function of exact tests only").
- Boxes are CERTIFIED-CONSERVATIVE caches with a containment
  contract, not tolerance objects (C10): constructors provided in
  this PR for — planar-face and Line-edge boxes from vertex
  extents (+ certified-residual padding ε where vertices sit on
  carriers only up to certification); Circle-edge boxes
  closed-form from the carrier (center/axis/radius over the
  certified span — the M2 span machinery); NURBS curve/surface
  boxes from control hulls (convexity — PR 3's positive-weights
  invariant is the precondition, cite it). The curved
  constructors land now (cheap, C10 names them) even though only
  planar ones get consumed in this PR.

## The consumer wiring (topo::boolean::reduce)

- The edge×face sweep's candidate generation goes through the
  tree; the EXACT per-pair classification path is untouched (the
  tree prunes, predicates decide — bit-identical results by
  construction, and pinned by the suite below). Same for the
  edge×edge sweep if its structure makes it a one-line join;
  otherwise report it as deferred consumer #2 (do not force it).
- The reduce module-doc's documented-quadratic note updates to
  name the tree and the suite.

## The differential suite (day one, CI-riding)

- Idealized = brute-force all-pairs (the existing code path,
  preserved behind a test-only entry or reimplemented in the
  suite); realized = BVH-pruned. Pins, over the full Band 4
  corpus + the boolean demo bodies: (i) realized candidate set ⊇
  idealized ACCEPTED set (supersets allowed, misses fatal); (ii)
  final boolean results BIT-EQUAL between a tree-backed run and a
  brute-force run (the D9 pin — same bodies, same bytes); (iii) a
  planted-degradation test: perturb one box to be too small and
  verify the suite CATCHES the lost pair (the suite must be able
  to fail).
- Rides existing CI lanes (the corpus rows); no new hosted row
  expected — confirm and report.

## Battery

Full workspace 3ε default + interval; clippy both ways; doc; fmt;
the corpus/persistence/latency rows locally (the latency lane is
reporting-only but a big sweep speedup should SHOW — report the
before/after die/corpus rebuild numbers honestly, no gate).

## Out of scope

SSI seeding/subdivision wiring (PR 7); viewport picking (Band 1);
any parallel build; any change to predicate/classification code;
tuning beyond the fixed named constants (PERF-PLAN stays
advisory).

## A/B note

Row 15, difficulty M (logged pre-draw), arm = fable (block-6
draw byte 190 → fable, opus).
