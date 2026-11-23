---
id: cert10-strict-gap-floor-gates-on-a-varying-seed
kind: issue
title: nurbs_cert_fuzz's cert10 strict-gap floor gates hosted CI over a per-run varying seed and fails at some seeds
status: closed
opened: 2026-09-05
priority: P3
cost: E
closed: 2026-10-28
---


## What

`crates/mesh/src/nurbs_cert_fuzz.rs:247-253` asserts `strict > trials`
over a randomly seeded sweep whose seed varies per run (`:87`, `:106`).
On PR 1919's head `38dd4692` — a diff that does not touch `crates/mesh`
— the `k-lint (gate, dev-budget)` row drew seed `0x3f9f92d266ab512c`
and got **59 strict of 300 comparisons**, one short of the floor
(`trials = 60`). The TOPO lane reproduced it locally at that seed with
its own changes reverted, so the draw is the whole cause: the floor is
a distribution claim gated on one sample, and some seeds fall under it.

The shape question `memories/test-suite-cost.md` asks — which shape is
this row, and should a seed vary under a gate — is this program's to
answer: pin the seed and keep the floor, keep the varying seed and
report rather than gate, or derive the floor from the draw. What
should not stand is a hosted gate that reds unrelated PRs at a rate
nobody has measured.

Reported by the `topo/d261-reader-collapse` lane in its PR body
(run 33945662993); filed by the TOPO orchestrator, 2026-09-05.

## Re-homed at S-MESH's exit (2026-09-16)

Moved from `work/mesh/` to S-TINT (test-suite integrity: a seed-varying hosted gate and grep-only sentinels) when S-MESH closed (`docs/S-MESH-EXIT-WALK.md`); the item's content, id and history are unchanged.

## Closed (2026-10-28)

**The witness no longer reds.** At `CAD_FUZZ_SEED=0x3f9f92d266ab512c` today's
main draws 160 strict of 300, against the floor of 60. The red was the
discard drift the 2026-09-11 generator fix removed: degree-1 directions
had been drawn knotted, and `compared` was not asserted. Over 1000 varying
seeds at effort 1, the strict count ran 112..=197 (mean 153, sd 14), with
no red. So today the floor sits about 6.5 sd below the mean.

**Shape, per discipline §8: the test mixed shape 1 and shape 3, so it
is split.** The test asserted three things:

- the inequality `fold <= whole` per channel, which is a counterexample
  search;
- `compared == trials`, a per-draw property that every drawn net is
  bounded;
- `strict > trials` and `saw_high_mult`, which are coverage witnesses
  nobody can write down.

The first two stay in `cert10_the_fold_never_exceeds_the_whole_net_hull`,
on a varying seed with the effort dial. A refused draw is now a panic at
its trial rather than a `continue` plus a count. The coverage floors move
to `cert10_a_pinned_draw_makes_the_gap_strict_at_high_multiplicity`, on
`fuzz::pinned` with a fixed 60 trials (173 strict, multiplicity >= 2
drawn). Both share one `cert10_sweep` body. The pinned row costs about
0.1 s.

**Sibling in the same file, left as is:**
`r1_random_rational_soundness_sweep`'s `worst > 0.5` is the same form,
but it is met per trial rather than per sweep. At seed 1001, all 60
trials had a truth/bound ratio above 0.5 (minimum 0.88), and 37 of 37
varying seeds reported `worst` = 1.000000. A red would need all 60 trials
below 0.5 at once. Splitting it would double a slow-set row (about 30 s in
debug) to guard against that, so it stays.
