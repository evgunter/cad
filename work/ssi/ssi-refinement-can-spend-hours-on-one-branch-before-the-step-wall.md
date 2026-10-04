---
id: ssi-refinement-can-spend-hours-on-one-branch-before-the-step-wall
kind: issue
title: ssi: refinement's rounds are bounded only by the step wall, about 10^4 rounds of O(n) each, so one branch could take hours before it refuses; and the wall's 20 000 has no stated envelope
status: open
opened: 2026-10-04
priority: P3
cost: M
design: true
---


(Filed 2026-10-04 by the `ssi/retire-fit-budget` lane, PR 4028, from
the PR's review, findings M1 and S7.)

## What

`refine::refine_by_certificate` (`crates/geom-brep/src/ssi/refine.rs`)
stops on two terms: the band rule, which is formal (a 2.2 m loop at a
band of 1e-11 m admits about 10^11 samples), and the branch's step
budget, `SSI_MAX_STEPS = 20 000` (`crates/geom-brep/src/ssi.rs`). Every
round that does not stop adds at least one sample, and at least two
where every selected midpoint settles, so a refinement from `n₀`
marched samples takes at most `(SSI_MAX_STEPS − n₀)/2 + 1` rounds:
about 10^4. Each round refits and re-certifies the whole carrier, which
is O(n) since the banded solve (PR 4023), at about 1–2 s a round in
release near n = 10^4 (the dome at ε 1e-13 and 1e-14). A refusal that
creeps one gap a round across a long branch could therefore spend hours
on one branch before the wall refuses it.

Nothing public has reached that. The worst case measured is 37 rounds
(the dome's tilt cut at ε 1e-14, 100 s, release); the creeping shape is
pinned only on a stand-in
(`refine::tests::a_creeping_refusal_is_bounded_by_the_neighbours_it_halves`,
under a 64-step context).

## The wall has no envelope (S7)

The 20 000 is agent-written (PR 7) and documented as a named resource
wall, not a derivation. Deriving it needs a stated envelope: the
largest model, the highest curvature and the finest ε the kernel
supports. The count then is `n ≈ L·κ^{3/4}·(c/ε)^{1/4}`. The one data
point measured: the dome's 2.2 m level loop (3-D curvature 1.4–4.7/m)
takes 5 787 samples at 1e-12, 10 291 at 1e-13 and 18 299 at 1e-14.
Choosing the envelope is Ev's.

## Options seen, unranked, no ruling

1. A time or work budget per branch beside the step wall (rounds × n),
   named and typed like the others.
2. A cap on rounds alone, derived from the halving argument (a gap's
   between-sample error falls by 16 per halving, so a refusal still
   creeping after `k` halvings of the same gap is not between-sample
   error). The fork weighed a progress stop and withdrew it: margins
   are non-monotone and switch limbs (`work/ssi/log.md`, 2026-10-04).
3. Leave it: the wall bounds memory and work is bounded in principle.
   Record the worst case in the wall's doc.
