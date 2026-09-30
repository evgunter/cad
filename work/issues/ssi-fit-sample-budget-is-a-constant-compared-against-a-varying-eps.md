---
id: ssi-fit-sample-budget-is-a-constant-compared-against-a-varying-eps
kind: issue
title: SSI_MAX_FIT_SAMPLES is a fixed cap on a sample count the marcher grows as epsilon shrinks, so a well-posed intersection refuses at fine eps
status: open
opened: 2026-09-15
refs: [D70]
priority: P1
cost: D
---


(S-TINT orchestrator, 2026-09-15) Filed from S-TINT's `D70`, whose
thirteen silent test stand-downs turned out to be two kernel defects
wearing one test-suite costume. Twelve are the plane × NURBS
certificate's, filed on TRIM as
`work/trim/plane-nurbs-certificate-bound-does-not-refine-with-eps`. This
is the thirteenth, and it is a different constant in a different file.

**Filed in `work/issues/` because the owner is genuinely undecided, not
as a waiting room.** No open program lists `crates/geom-brep/src/ssi.rs`
in its `paths` (`work.py territory` names none). TRIM owns
`edge_nurbs.rs`, `pcurve_cache.rs` and `nurbs_iso.rs` and already
carries one `plane_nurbs_ssi` row, so it is the likeliest claimant — but
that row is about `edge_nurbs.rs`'s tube pad, not about this file, and a
lane does not get to enlarge another program's territory by filing into
it. Claiming this is a `git mv` and a header edit.

## The defect

`crates/geom-brep/src/ssi.rs`, `fit_branch`:

```rust
if points.len() > SSI_MAX_FIT_SAMPLES {
    return Err(SsiError::FitSampleBudget { samples: points.len(), budget: SSI_MAX_FIT_SAMPLES });
}
```

`SSI_MAX_FIT_SAMPLES` is a `const`. The sample count it caps is not: the
marcher chooses its step so that a cubic through the samples is within ε
of the locus between them (`SSI_STEP_DEVIATION`, stated in `fit_branch`'s
own comment). **So the demand grows as ε shrinks and the cap does not
move.** A well-posed intersection therefore refuses at fine ε for a
reason that is a property of the budget rather than of the geometry.

Observed through `crates/topo/tests/m5_pr7_split_meter.rs`'s
`ssi_branch_or_budget`, whose fixture is an offset cylinder (radius
0.08, axis offset 0.03) threading the unit sphere: it returns `None` at
the fine ε row, and two `#[test]`s stand down on it.

## How this differs from the TRIM row, and why it may not be a defect at all

The TRIM row is a clean defect: a bound that does not refine, compared
against ε, on a carrier that is exact — nothing about the geometry
justifies the refusal. **This one is arguable, and a taker should decide
which it is before writing any code:**

- Read as a **resource cap**, refusing is honest. Meeting a tighter ε
  genuinely needs more samples; a fixed ceiling on work is a legitimate
  thing for a kernel to have, and `FitSampleBudget` carries both numbers
  in its payload so the refusal explains itself. On this reading the
  defect is only that the cap is stated as an absolute count rather than
  as a function of what was asked for.
- Read as a **class boundary**, it is the TRIM shape again: the set of
  intersections this lane can fit shrinks as ε tightens, for reasons no
  caller can predict from the geometry, and *"this kernel fits SSI
  branches only at tolerances coarse enough for N samples"* is a
  statement belonging in the documentation rather than in a constant.

The distinguishing measurement is cheap: does `points.len()` at the fine
ε row exceed `SSI_MAX_FIT_SAMPLES` by a little or by a lot? A little
says the cap is merely mis-set; a lot says the lane's reach really does
fall off with ε and the contract should say so.

## What is NOT claimed here

That the cap should be raised. Raising it trades refusals for fit time
and carrier size on every SSI call in the tree, which is a cost question
and S-TCOST's if it becomes one. This row asserts only that a constant
is being compared against a quantity that varies by three decades across
the CI matrix, and that nothing in the tree says which of the two
readings above is intended.
