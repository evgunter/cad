---
id: torus-tangency-shell-floor-does-not-scale-with-k
kind: issue
title: the_clamp_floor_clears_the_torus_tangency_shell asserts a fixed floor against a shell that grows as K^(1/3) — red at CAD_AMBIGUITY_K=30 on main
status: open
opened: 2026-09-11
priority: P3
cost: E
---


(FIX orchestrator) From the `literal-k-where-the-runs-k-belongs` lane,
PR 2346. Reported by the lane and placed here rather than repaired:
the lane's diff cannot reach it, and this program's question — whether
the suite asserts what it claims to assert — is exactly what it is.

## The defect

`crates/sweep/tests/bool3_torus_doors.rs:683`,
`the_clamp_floor_clears_the_torus_tangency_shell`, fails at
`CAD_AMBIGUITY_K=30` on `main`'s own tree (reproduced with the lane's
branch stashed):

```
the probe offset 0.001 no longer clears the tangency shell
5.147018158714216e-4 by 2×
```

`away()` is a **fixed** floor; the shell grows as K^⅓. So the row
asserts a clearance that holds at `DEFAULT_K` and at no sufficiently
large K — and **the assertion's own message predicts this**, which is
the part worth noticing: the row already knows what would break it and
still pins a constant.

## Why it is latent rather than red

No CI row runs the suite at a non-default K — measured, no workflow
mentions `AMBIGUITY_K` at all. That hole is filed separately as CIW's
`work/ciw/no-ci-row-runs-the-suite-at-a-non-default-k.md`. Until it
closes, this row cannot go red in the gate, which is why it has
survived: it is not a regression waiting to happen, it is a claim that
is already false about configurations the gate never draws.

## What it needs

The floor derived from the shell rather than pinned beside it, or the
row's claim narrowed to the K it actually holds for and said so at the
site. Both are small; the choice is about what the row is *for*, which
is this program's question rather than a passing lane's.

## Re-derived (2026-09-15, lane B)

**VERDICT: REPRODUCES** — extending the orchestrator's re-derivation
rather than repeating it. No test was run.

### The extension asked for: does any CI row now run at a non-default K?

**No. Re-derived today, and the hole is wider than "no workflow mentions
it".** `grep -rn "AMBIGUITY_K" .github/ local-scripts/ scripts/` returns
**zero hits** — not in `ci.yml`, not in `nightly.yml`, not in
`render.yml`, not in `work-status.yml`, not in `local-scripts/ci-local.sh`
(including its `--nightly` row), and not in any script the gate runs. The
same grep over the whole repo finds the symbol only in source and in
`review/r2-bool11/`, i.e. only in code that READS the knob, never in
anything that SETS it for a run.

So the row's "why it is latent" paragraph holds unchanged, and the CIW row
it points at — `work/ciw/no-ci-row-runs-the-suite-at-a-non-default-k.md`
— is still open on that slate. Nothing has closed the hole and nothing has
narrowed it.

### The defect itself, re-derived by name

`crates/sweep/tests/bool3_torus_doors.rs`. `away()` is still a body-less
forward to `probe_offset()` — a fixed, ε-clamped floor with no K term —
and the clearance assertion is intact:

```
        away() > shell * 2.0,
        "the probe offset {} no longer clears the tangency shell {shell:e} by 2× — …
```

The orchestrator's point that "the assertion's own message predicts this"
is if anything understated: **`away()`'s own doc comment predicts it in
full**, and names the fix:

> *"It scales as `K^⅓` too, so raising `CAD_AMBIGUITY_K` three decades
> puts the shell past this floor; the guard row goes red saying so, and
> the fix is to raise the FLOOR."*

The doc also records the margin that makes `K=30` enough: at default ε the
clearance is **2.7×** against a 2× floor (the table's middle row, `away()`
1e-3 vs shell 3.66e-4), where the 1e-12 and 1e-6 rows clear by ~27×. So
the default ε row is the only one the K-scaling can reach at small K, and
that is the row the lane's `CAD_AMBIGUITY_K=30` reproduction hit.

### One thing the row does not say, worth recording

The same doc says the clamp *"saturates at every shipped ε row (floor at
1e-9 and 1e-12, ceiling at 1e-6)"* — so `away()` is not merely
K-independent, it is ε-independent at all three ε rows CI draws. A guard
whose floor is constant across the entire drawn matrix while the quantity
it guards moves by two decades across that matrix is the same defect in
the ε axis; it is masked only because the clamp happens to sit on the safe
side at each drawn point. That strengthens "narrow the claim to the
configurations it holds for" over "raise the constant".

### Blind spot

Whether `CAD_AMBIGUITY_K=30` still reds on today's tree was NOT
re-measured — that is a run, and this lane ran none. What is established
by reading is that the mechanism is unchanged (fixed floor, K^⅓ shell,
2× margin at default ε, 2.7× measured headroom) and that nothing in CI
draws a non-default K, so the row cannot have started reddening the gate
since it was filed.

**Recommendation:** do not close. The choice the row frames — derive the
floor from the shell, or narrow the claim and say so at the site — is
unchanged, and the ε observation above suggests the second.

## `away()` is ε-invariant as well as K-invariant (S-TINT orchestrator, 2026-09-15)

Lane B, extending the reproduction: `away()`'s own doc says the clamp
*"saturates at every shipped ε row"*, so the floor is constant across
the whole drawn ε matrix while the shell it guards moves two decades
across it. The K axis is where the row was filed and the ε axis is the
same defect in a dimension CI **does** draw — masked only because the
clamp happens to land on the safe side at each of the three drawn
points.

**This argues for the disposition rather than against it.** Raising the
constant fixes neither axis; it buys headroom until the next K or the
next ε. What the row should take is the narrower claim — state the
clearance as a function of the shell it is clearing, so the assertion
moves with what it guards — which is the same repair the two kernel
rows `D70` closed onto are asking for one level down
(`work/trim/plane-nurbs-certificate-bound-does-not-refine-with-eps`).
A fixed number compared against a varying one is one defect with three
carriers on this slate, and this row is the test-side instance.
