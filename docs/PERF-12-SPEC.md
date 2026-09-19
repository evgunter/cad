# PERF-12 — the at-rest census takes the BVH as its pre-filter

**Status: ratified at dispatch (PERF orchestrator, 2026-09-13).** Binds
the implementer of unit `PERF-12`; deleted at merge per
`docs/DOC-LEDGER.md`. Read `docs/prompts/implementer-discipline.md` in
full first. The item is `work/perf/assemble-aggregate-census-is-quadratic-in-solids.md`.

## 0. The finding this executes

`assemble` (`crates/editor-core/src/product.rs`) runs the tier-3′
at-rest census over the aggregate body (`topo::validate_pseudomanifold
_certificate_certified` → `crate::census::census_and_certify`,
`crates/topo/src/census.rs`). The census's own header says its
sweep shape: quadratic all-pairs sweeps in arena order (vertex×vertex,
vertex×edge, vertex×face, edge×face, edge×edge) plus the conformal
arm and the cross-solid backstop — "correctness first, the BVH filter
is PERF-PLAN's later 10×". Measured on the corpus heat sink driven to
N fins (release): 11 solids 11 ms, 41 solids 85 ms, 161 solids
**1.3 s**, n^1.96; the gather is 23 ms at 161 solids and `run_checks`
answers the linear part. `crates/bvh` is already a dependency of
`topo` (the boolean edge×face sweep, the separation certificate) and
the census already reaches it.

## 1. What this unit delivers

**The five all-pairs sweeps behind the BVH pre-filter**, under D9's
conservative-superset contract (`work/perf/plan.md` §2.1): the filter
may prune only pairs the exact predicate would reject, so the census's
result stays a function of exact tests only; every candidate pair the
exact sweep would have examined AND decided non-trivially must still
be examined. Concretely:

- measure FIRST which pairs each sweep examines on the fin bodies and
  the corpus (how many pairs, how many reach a predicate, how many
  decide anything) — the numbers that justify the filter and size it;
- one `Bvh` per entity class over the aggregate's boxes, built in
  arena order (the crate's determinism contract), the sweeps
  iterating `bvh.pairs`/`overlaps` candidates in **ascending arena
  order** (the crate's query contract: candidates a subsequence of
  the arena order, independent of tree shape) so every sweep's
  decision order — and therefore its verdict order, its K-funnel
  recording and its error vector — is exactly today's; where a sweep's
  order today is not arena-ascending, say so and keep today's order;
- the box of each entity is the one the boolean's `face_box` /
  `edge_box` already prove sound for every carrier kind (`boolean/
  boxes.rs`: NURBS via the control-net hull; a null carrier poisons
  and is never pruned); reuse those doors, do not write new boxes;
  vertex boxes are points widened by the band's ε (state the
  widening and why it makes the filter conservative under the
  census's own tolerance);
- the conformal-patch arm and the cross-solid backstop: read what
  they sweep and whether the filter applies; if their pairs are
  keyed (same-key opposed-sense pairs), they are not quadratic and
  stay as they are — say so.

If the measurement shows the pairs that reach a predicate are already
few and the time is elsewhere (the predicates themselves, the
declared-contact certification), the filter is not the fix: report
where the n² is and stop — a finding, not a unit.

## 2. The pin

- **Verdict identity across the corpus and the fin bodies**: every
  `census_and_certify` error vector (kinds, entities, order) and every
  K-funnel recording byte-identical to main's — a golden cut on the
  merge base over the corpus's multi-solid documents, the STEP
  fixtures that reach the census, and the heat sink at 10/40/160
  fins; `assemble`'s verdicts and `run_checks`'s unchanged.
- **The superset contract, adversarially**: a differential row that is
  NOT built from axis-aligned bricks (the gap that let the NURBS hole
  live for three milestones — plan §2.1): curved solids touching at a
  curved seat, a NURBS-walled body beside a planar one, tori resting
  on cylinders, an edge grazing a face's box but not the face — the
  filtered sweep's examined-pair SET must equal the exact sweep's set
  of pairs that reach a predicate, on every row (keep the exact sweep
  as the test oracle, test-only, not a production twin).
- **Poison never pruned**: a null-carrier entity is examined against
  everything, as today.

## 3. Measurement to report

Release, 4 vCPU under the slot, medians of 3: `assemble` and the
census on the heat sink at 10/40/160/640 fins before and after (the
exponent, not just the time); the corpus's multi-solid documents;
pairs examined before/after per sweep. Expect ~n log n with the
predicate count unchanged.

## 4. Out of fence

The predicates and the declared-contact certification; the census's
decision rules and vocabulary; `crates/bvh`'s build and query (a new
pairwise query door there is in scope only if the crate lacks one —
then its differential suite gains it under the same contract);
`run_checks`. DOCM/LIB territory in `product.rs` is untouched;
TOPO territory in `census.rs`, announced in `work/perf/log.md`;
PERF-11 runs beside you on `props.rs` — merge `origin/main` before
opening the PR.

## 5. Report

≤120 lines: the pair census before/after per sweep, the boxes reused,
the order argument, the adversarial rows, the goldens, the
measurements of §3, deviations, findings outside the fence.
