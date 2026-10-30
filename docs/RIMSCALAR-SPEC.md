# RIMSCALAR: the injectivity check certifies at Interval what it certifies at f64

Unit: `work/clear/injectivity-refuses-at-interval-what-it-certifies-at-f64.md`
(P1). Program: CLEAR. Branch: `clear/rimscalar`. Review: a single
FULL review, with mutation. The fix should make the `Interval` reading
agree with the reviewed `f64` reading, not add an arm. The reviewer
still has to show that nothing is loosened.

## Context

SHELL-4's lane stopped on a finding. On SHELL-4's branch at
`623daf636b`, the WIP test file `shell_cavity_embedded` holds the
evidence. The test: the self-intersection sweep on the operand vessel
`vessel_at(1.0, 2.0)` (`crates/sweep/tests/common/shell_operands.rs`),
at the strict bound, over all faces, with `same_body = true`.

| body | verdict |
|---|---|
| `Body<f64>` | Holds: 9 candidates, 50 discharged, 14 certified, `cornered = 0` |
| `Body<Interval>` | Refused `Uncertified { a: 2v1 (disc), b: 5v1 (cylinder band), shared: Edge(9v1), arm: Departure }`; refused 1, `cornered = 4` |

- Edge 9 is a half-rim where the disc meets the cylinder. It has the
  same carrier at both scalars.
- At `Interval` its witness point carries ulp-wide enclosures (for
  example `x ∈ [−1.8e-16, 4.8e-16]`).
- At `f64` the carrier arm certifies it, using the disc's plane implicit
  as ψ against the wall's iso rectangle.
- At `Interval` the corner presentation is taken instead, and departure
  fails.

**The class.** `crates/sweep/tests/self_intersection_proves_injectivity.rs`
builds every row as `Body<f64>`. No row runs the certificate on a
`Body<Interval>`, which is the certifying scalar SHELL-4's gate and the
document door use. So the divergence was invisible.

## What is true when this lands

1. **The cause is found and stated.** Name where the `Interval` body's
   reading departs from the `f64` body's. Possible places:
   - a classification (iso / straight / closed / curved) decided by an
     exact comparison on an ulp-wide enclosure;
   - an extent, or a vertex match, decided the same way.

   Report the site, the values at both scalars, and why `f64` takes the
   carrier arm and `Interval` does not.
2. **The fix is sound.** The fix makes the `Interval` reading reach the
   arm the `f64` reading reaches, by deciding the classification
   soundly on the enclosure: at a funnelled site with an audit row, or
   structurally from topology or the stored carrier kind.
   - It must not make any reading laxer at either scalar.
   - If the honest answer at `Interval` is a refusal, stop and report
     why. Do not force it to agree.
3. **The class is closed.** Every row in
   `self_intersection_proves_injectivity.rs`, and the CORNER and
   CURVEDEP sweep rows, also runs on a `Body<Interval>` built the same
   way, by the generic builders.
   - Each row's `Interval` verdict class matches its `f64` verdict
     class: Holds, Violated, or the same refusal arm.
   - Receipts may differ, and each difference is listed.
   - Any mismatch is fixed in this unit, if it shares the cause. If it
     does not, it is filed and pinned as a known divergence, typed.
   - The vessel at `Interval` holds.
4. **Mutation.** Show that the new `Interval` rows go red when the fix is
   reverted, and when the decision you add is made lax.
5. **No `f64` receipt moves.** A moved receipt is a finding.

## Ground

- `crates/topo/src/injective.rs` and `crates/topo/src/clearance.rs`.
- The sweep tests.
- `docs/predicate-dimension-audit.md`.
- No other CLEAR unit is in flight in these files. SHELL-4 is parked on
  this one, and its WIP branch touches only `shell.rs` and its tests.
- Read `docs/prompts/implementer-discipline.md` in full.
- Never arm a waiter or a background chain for a build or a test.
  Anything that outlives 600 s runs `setsid`-detached and is polled in
  the foreground.
- CI is the verification of record. Allow 25 min.

## Report

≤120 lines, covering:
- the cause;
- the fix and its soundness argument;
- the f64/Interval verdict table over every row;
- the mutants;
- the PR number and its CI result.
