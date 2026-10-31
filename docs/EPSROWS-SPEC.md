# EPSROWS: the two self-intersection rows hold at every nightly ε

Unit: `work/clear/main-red-two-injectivity-rows-fail-at-eps-1e-12.md`
(P1/M). Program: CLEAR. Branch: `clear/epsrows` (the row is its first
commit). Review: a single FULL review with mutation. The helix half
touches a producer (an iso-line mint), so believing it takes more than
reading it.

Read the row in full first.

## What is true when this lands

1. **The tee row holds at default, 1e-6 and 1e-12.**
   - Pin what the row is about: the injectivity check refuses typed at a
     vertex the junction shares alone. Pin the Interval union's failure
     by the claim that does not depend on ε (a typed refusal of the
     union), not by one exact arm, unless you show that arm is the
     honest answer at every ε.
   - Check its f64 half at 1e-12 too, once the panic is out of the way.
     Report its verdict at each ε.
   - The row must still go red if the union starts answering Ok, or
     answers with an untyped failure.
2. **The helix row holds at 1e-12, or its build refuses honestly.**
   - Find why the helical rim's residual (~2e-12) exceeds ε at the
     `pcurve_iso_line_rim` mint (`crates/topo/src/pcurves.rs:1480`), on
     half-edge 18v1, margin `[0, 1.86e-12]`.
   - Fix at the producer: if the rim is an iso line by construction,
     decide it structurally (from how the sweep built it), or make the
     residual honest. Do not widen a band to make the row pass.
   - If the honest answer at 1e-12 is a refusal, stop and report. Then
     the row pins that ε's refusal typed instead, and a filed row says
     why the helix cannot be built there.
3. **No new ε without a verdict that needs it.** Revert any concession
   alone and diff the verdicts.
4. **Nothing else moves.** The self-intersection and pipe rows, and the
   sweep's helical rows, keep their verdict class at all three ε. Every
   f64 receipt at default ε is bit-identical; list any that move.
5. **Mutation.** Each change goes red on a row when it is reverted, and
   when the decision it adds is made lax.
6. **The class.** The row states it: a row pinning an exact refusal arm
   under the ε matrix. Grep the sweep tests for other rows that pin an
   exact refusal arm and run them at 1e-12. Fix any red one that shares
   the cause; file any other.

## Ground

- `crates/sweep/tests/self_intersection_proves_injectivity.rs`,
  `crates/topo/src/pcurves.rs`, and the sweep builders.
- `docs/predicate-dimension-audit.md` if a site moves.
- The nightly's ε loop is in `.github/workflows/nightly.yml` (`every eps
  row`). Reproduce with `CAD_TOLERANCE_EPS=1e-12 cargo nextest run ...`.
- FRUSTUM (`clear/frustum`, #6114) touches `clearance.rs` and the same
  test file. Merge main once it lands.
- Read `docs/prompts/implementer-discipline.md` in full.
- Never arm a waiter or a background chain for a build or a test.
  Anything that outlives 600 s runs `setsid`-detached and is polled in
  the foreground.
- CI is the verification of record. Allow 25 min. CI runs default ε
  only, so paste the 1e-12 and 1e-6 local runs.

## Report

≤80 lines, covering:
- the cause of each failure;
- each fix and why it is sound;
- the verdict table at three ε;
- the mutants;
- the class grep;
- the PR number and its CI result.
