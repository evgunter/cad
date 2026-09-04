---
id: certify-locally-valid-range-instead-of-sampling
kind: issue
title: The locally-valid-range probe samples and bisects; the interval lane could CERTIFY the interval instead
status: open
opened: 2026-08-29
github: 1183
refs: [1143]
---

## From GitHub issue 1183

Opened 2026-08-29; 0 comments.

Banked from the 2026-08-29 GUI tweak batch (`docs/GUI-LOG.md` tail, branch `claude/gui-display-editing-tweaks-w1b8j3`), Ev's own framing in the conversation that produced it.

`viewer::bounds` answers "how far can this field move before something new fails" by **sampling**: step outward from the current value doubling the stride, then bisect the first bracket that goes bad, at most 44 document evaluations over both directions. Its module docs state the three limits that method has, and they are real ones rather than polish items:

- validity is not monotone, so what is found is the nearest boundary the *sampling* could see — a valid island beyond a bound is not reported, and an invalid sliver narrower than the stride is stepped over;
- each side reports a bracket (furthest value found valid, nearest found invalid), not a number;
- a side with no failure in reach reports how far it looked, never "unbounded".

**The certified answer this repo can already almost express.** The kernel is generic over its evaluation scalar and `evaluate::<Interval>` runs a whole `Doc` today (`m4_pr2_eval_interval.rs`, `review_m5_pr1_e2e_interval.rs`, `m5_pr8_bvh_diff.rs`). `Interval::from_bounds` is documented as *the subdivision driver's constructor* — the door for materializing a parameter sub-box for interval replay. Replaying the document with one field's value widened to `[a, b]` and reading the verdict would say something categorically stronger than sampling: not "these samples worked" but "no value in this box fails", and where it cannot say that, subdivide. Branch-and-bound over the box gives the largest certified locally-valid interval, and the sampling probe becomes at most a seed for it.

**Why this is kernel tooling and not a viewer change.** Three doors are missing, and the second is the substantive one.

1. **`evaluate` derives its own environment.** `Doc::param_env::<T>` builds every binding through `T::from_f64`, so every document parameter enters as a degenerate point enclosure and there is no door to hand `evaluate` an environment with one binding widened.
2. **A node SLOT has no name to widen at all.** A slot's value is an `Expr::literal` holding one `f64`, pinned bit-exact by D7, and rightly so — the widening is a property of the *query*, not of the document, so the answer is not an interval-valued literal in the recipe but a driver-side override that says "replay this document with slot S taking this enclosure". That vocabulary does not exist. It is also the piece that decides whether this stays a one-field query or generalizes to a box over several fields, which is the same machinery a solver would want.
3. **The verdict contract.** "The failing-node set did not grow" is what the sampling probe compares, and it is well-defined at `f64` because every classification lands. At `Interval` a decision can come back **indeterminate** — the `Decide` band declining to classify a straddling enclosure — which is not the same thing as a failure and must not be read as one: indeterminate means *subdivide*, failure means *the boundary is inside this box*. Deciding what a certified enclosure lane owes here is adjacent to the contract question already open as issue 1143 (poison absorbs vs widens), and the two should probably be settled together.

Cost is the other open question: an interval replay is more expensive per evaluation than an `f64` one, and branch-and-bound multiplies evaluations, so a certified range is plausibly not something to run behind a button press on the interaction path. The probe is already a resumable state machine (`BoundsProbe` asks for values and takes verdicts back, evaluating nothing itself), so a certified driver can replace the oracle without touching the panel — but the pacing question (inline, behind the eval seam, or a deliberate long-running query) belongs to whoever picks this up.

Nothing here is a defect in what shipped; the sampling probe says plainly in its type, its docs and its rendered wording that it is a probe. This is the design conversation about replacing a guess with a proof, and the tooling that would make it clean.

## Home

The three missing doors are the E6 interval subdivision driver's own — parameter-box replay, a slot-widening override and the indeterminate-verdict contract — which is M10's charter, and the issue names 1143 (M10's) as the question to settle alongside.
