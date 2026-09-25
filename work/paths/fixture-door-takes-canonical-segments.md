---
id: fixture-door-takes-canonical-segments
kind: unit
title: The dev-only RawLoop fixture door takes canonical segments; ProfileVertex retires; fixtures migrate through a helper forwarding to arc_to(Bulge)
status: closed
opened: 2026-09-25
priority: P1
cost: D
parent: lower-profiles-to-carrier-and-interval-not-vertex-and-bulge
branch: claude/clever-bardeen-4itqb3
closed: 2026-09-25
pr: 3231
---


Split out of unit 1 at spec time, so that unit 1's byte-identity claim
stays reviewable apart from the mechanical churn: about 1355
`ProfileVertex::new` sites in about 290 test files. What this unit does
(Ev, #3218 q4):
- `RawLoop` (behind `raw_door!`, compiled only under test/test-support)
  takes canonical segments, so a fixture can build a one-segment
  circle;
- `ProfileVertex` retires;
- fixtures written with a bulge go through a test-support helper that
  forwards to the algebra's `arc_to(Bulge)` lowering and has no
  arithmetic of its own.

Afterwards the only bulge surface left in the kernel is `arc_to(Bulge)`.
Move one crate per commit, and keep the rewrite scriptable.

Unit 1 gave `ProfileVertex::new` an exact-zero test built from
`is_poison` (`Real` has no equality), so it can decide Line vs Arc from
a raw bulge. It retires with `ProfileVertex` in this unit.

## Spec (PATHS orchestrator, 2026-09-25)

**Goal.** No vertex+bulge record exists outside the algebra. Only the
first two points are this unit's; units 2 and 5 finish the goal.
1. **`RawLoop::new` takes the canonical form:** verbatim vertices and
   one `Segment` per edge. So does its polygon helper. This lets a
   fixture build a one-segment circle, which validate still refuses
   until unit 3. Keep the `raw_door!` gate exactly as it is: compiled
   only under `test` / `test-support`.
2. **`ProfileVertex` retires as a type.**
   - Emission lowers straight from (point, bulge) through
     `lower_to` / `lower_arc`, with no intermediate record.
   - `is_exact_zero` goes with it, unless the one lowering rule still
     needs it. In that case it stays private to that rule, and your
     report says so.
   - `ProfileLoop`'s `input_chain` and `bulges()` stay until unit 5.
3. **Fixtures that were written with a bulge** move to one test-support
   helper, say `profile::test_support::bulge_loop(&[(Point2, b)])`.
   It builds the loop with the SAME lowering the algebra's
   `arc_to(Bulge)` uses (`lower_arc`) and has no arithmetic of its own.
   So every migrated fixture lowers bit-identically to today.
   - The rewrite is mechanical: `ProfileVertex::new(p, b)` chains
     become `bulge_loop(&[(p, b), …])`.
   - Keep the script in your report, not in the tree.
   - One crate per commit.
4. **Public surface.** `pncad` re-exports `ProfileVertex` in its
   `profile` module and prelude, and that façade is LIB's. Remove the
   export. Add a short announced-seam entry to `work/lib/log.md`, signed
   `(PATHS orchestrator)`, saying so. `bulge_from_center` /
   `bulge_from_via` are NOT this unit's; they are unit 6's.

**Byte identity: nothing moves.** No golden, K CSV, census, digest or
render may change. If one does, stop and report.

**Review tier: single FULL review.** The change is mechanical, but its
byte-identity claim over about 1355 sites deserves falsification.

**Claims:**
- (C1) Every migrated fixture lowers to the same stored segments and
  bulges, bit for bit.
- (C2) No `ProfileVertex` remains anywhere: src, tests, demos, benches,
  tools or docs.
- (C3) The raw door's gate is unchanged.
- (C4) The helper has no arithmetic of its own.

## Closed (2026-09-25, #3231)

Merged bit-identical: the reviewer executed the rewrite script over all
250 files and diffed the result. `ProfileVertex` is gone; `RawLoop::new`
takes canonical segments; `bulge_loop` forwards to `ProfileLoop::lower`.
Residue went onto `store-constructed-carriers` (canonical-door loops
drift under re-lowering: last bits, and a NaN centre for a one-segment
circle) and `one-segment-loop-through-builders` (a prerequisite).
