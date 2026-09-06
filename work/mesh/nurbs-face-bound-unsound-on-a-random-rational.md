---
id: nurbs-face-bound-unsound-on-a-random-rational
kind: issue
title: nurbs_face_bound is UNSOUND on a random rational surface: r1_random_rational_soundness_sweep failed hosted at seed 0xdae51dbd4e1b79fd
status: open
opened: 2026-09-04
refs: [1850]
---


## The failure

Hosted run [33904520538](https://github.com/evgunter/cad/actions/runs/33904520538),
job `k-lint (gate, dev-budget)`, step *mesh budget meter + certificate
falsifier (feature = budget)*, on branch `ciw/unsample-klint` at `3c3723a8`:

```
thread 'nurbs_cert_fuzz::r1_random_rational_soundness_sweep' (2674) panicked
  at crates/mesh/src/nurbs_cert_fuzz.rs:132:9:
UNSOUND at trial 8: (2.660e0,7.953e0,6.996e-1) vs (2.660e0,7.953e0,7.116e-1)
  — reproduce with CAD_FUZZ_SEED=0xdae51dbd4e1b79fd CAD_FUZZ_EFFORT=1
test result: FAILED. 117 passed; 1 failed; 2 ignored
```

The failing assertion is `crates/mesh/src/nurbs_cert_fuzz.rs:132` —
`wuv <= b.muv && wuu <= b.muu && wvv <= b.mvv`, the claim that
`nurbs_face_bound`'s second-derivative bound DOMINATES the sampled truth.
The two `wuu` / `wuv` figures match the bound to four digits; the failure is
on the third component: **`wvv` sampled `7.116e-1` against a bound of
`6.996e-1`** — the truth exceeds the bound by about 1.7 %, on trial 8 of 60.

## Why this is a real finding and not a flake

`nurbs_face_bound` is a CERTIFICATE. The test is a soundness sweep: the bound
is asserted to be an upper bound, and a case where the sampled worst exceeds
it is a counterexample to the certificate, not a tolerance wobble. The margin
is 1.7 %, which is far outside anything a 61x61 sampling grid's discretization
could explain in the safe direction — sampling can only ever UNDERSTATE the
true worst, so a sample above the bound is decisive.

It is *seed-dependent*: `fuzz::start` varies the seed per run and the trial
count rides `CAD_FUZZ_EFFORT`, so it does not fire on every run. That makes
it intermittent, not spurious. The seed is printed and is above.

## How it surfaced, which is the part worth recording

**This step is `dev-budget`'s, and `dev-budget` was drawn 1-in-5 until
2026-09-04.** PR 1850 un-samples the k-lint row, so the falsifier now runs on
every code-tier run instead of one in five. It found this on the fourth run
of the new shape. Under the draw the same defect would have needed, in
expectation, five times as many merges to surface — and MIN-1's ratification
named this falsifier as unconditional, which it had not been since
2026-08-22.

Filed by the CIW lane that made the change, not by mesh, because the branch
that caught it is not mesh's. `crates/mesh/src/nurbs_cert_fuzz.rs` is
untouched by PR 1850, whose only `crates/mesh/` hunk is a `//!` comment in
`tests/probe_review.rs`.

## What is owed

1. Reproduce locally: `CAD_FUZZ_SEED=0xdae51dbd4e1b79fd CAD_FUZZ_EFFORT=1
   cargo test -p mesh --features budget r1_random_rational_soundness_sweep`.
2. Decide whether the bound is wrong or the sampler is measuring something
   the bound does not claim to cover — the assertion compares
   `sample_worst(&s, 60)` against `nurbs_face_bound`, and the disagreement is
   confined to the `vv` term with `uu` and `uv` matching to four digits,
   which points at the v-direction second-derivative term specifically.
3. If the bound is wrong, every consumer of `NurbsFaceBound`'s `mvv` inherits
   the unsoundness — the split-step selection and the tessellation budget both
   read it.
