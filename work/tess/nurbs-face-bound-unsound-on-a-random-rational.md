---
id: nurbs-face-bound-unsound-on-a-random-rational
kind: issue
title: nurbs_face_bound is UNSOUND on a random rational surface: r1_random_rational_soundness_sweep failed hosted at seed 0xdae51dbd4e1b79fd
status: open
opened: 2026-09-04
refs: [1850, rational-cells-hull-the-f64-refined-net-so-the-described-patch-escapes]
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

## Second instance, 2026-09-16 — and it is NOT the same defect

Hosted run [35050942262](https://github.com/evgunter/cad/actions/runs/35050942262),
same job and same step, on branch `dup/topo-brick-copies` at `7dd81c6f`
— a PR whose diff is `crates/topo/tests/*.rs` and `work/*.md` only
(`git diff --name-only origin/main...HEAD -- crates/mesh` is empty, so
`crates/mesh` on that branch is byte-identical to main):

```
UNSOUND at trial 29: (1.249e0,6.659e0,4.294e0) vs (1.249e0,6.659e0,4.294e0)
  — reproduce with CAD_FUZZ_SEED=0x5ca58da03160d407 CAD_FUZZ_EFFORT=1
```

Reproduced locally from the seed, deterministically. **The three
components print equal at `{:.3e}`, which is the first thing to record:
the assertion's own message cannot show the margin it failed on.** At
`{:.17e}`:

```
sampled  (1.24859234123372476, 6.65945472815095485, 4.29417353797470724)
bound    (1.24859234123372431, 6.65945472815121153, 4.29417353797474810)
```

So `wuu` exceeds `muu` by **4.44e-16 — two ULPs, 3.6e-16 relative** —
while `uv` and `vv` pass with 2.6e-13 and 4.1e-14 of room.

**This is a different cause from the 2026-09-04 instance above**, and the
distinction matters for what is owed:

- 2026-09-04 was `wvv` over `mvv` by 1.7 %. That is a bound that is
  genuinely wrong, as the section above argues.
- 2026-09-16 is `wuu` over `muu` by two ULPs. No sampling-versus-bound
  argument reaches two ULPs; this is the certificate and the sampler
  rounding differently on a surface where the bound is tight, against an
  assertion written as a bare `<=`.

So **fixing the `vv` bound will not stop this row reddening.** A
certificate that claims domination has to claim it with a rounding
margin, or the comparison has to be made in a way that cannot lose to
the last bit — and the assertion message has to print enough digits for
the next reader to tell which of the two failures they are looking at.

**Why this keeps landing on other programs' PRs.** `fuzz::start` draws a
fresh seed per run, so the row is a tree-wide flake generator: any PR
can draw the seed that exposes either defect, and the lane that draws it
has no way to tell a real regression from this row without reproducing
by hand. Related: `work/mesh/cert10-strict-gap-floor-gates-on-a-varying-seed.md`
and `work/mesh/mesh-cert10-fold-fuzz-row-flakes-on-a-fresh-seed.md`.

Evidence added by the `dup-brick` lane (S-DUP), which drew the seed.

## Re-homed at S-MESH's exit (2026-09-16)

Moved from `work/mesh/` to TESS (opened at this exit as S-MESH's successor for the tessellation kernel) when S-MESH closed (`docs/S-MESH-EXIT-WALK.md`); the item's content, id and history are unchanged.

## CORRECTION, 2026-09-18 — one defect, not two, and it is not in `mesh`

Measured by TESS's diagnostic lane (`tess/nurbs-bound-diag` at
`db4cb45a8`); the orchestrator checked the message order and the
refinement site against the tree.

**"The failure" above reads the assertion's tuples backwards.** The
message prints `(sampled) vs (bound)`, so on 2026-09-04 `6.996e-1` was
the sampled `wvv` and `7.116e-1` the bound: `vv` PASSED with 1.7 % of
room. The red component was `uu`, by two ULPs (sampled
2.66033199807363996, bound 2.66033199807363907) — the same defect as
the 2026-09-16 instance. "Why this is a real finding" §, "What is
owed" item 2's `vv` pointer and item 3, and "it is NOT the same defect"
are all wrong as written; they are left in place as the record and
this section supersedes them. The 2026-09-04 seed no longer reproduces:
`47437f2b4` changed the generator's `mk`, so the seed draws other
surfaces now (it reproduces with the old `mk` restored).

**The cause** is `geom_brep::patch_bound::rational_cells` hulling the
`f64`-refined net — PROPS' file, filed there with the exact-arithmetic
measurements:
`work/props/rational-cells-hull-the-f64-refined-net-so-the-described-patch-escapes.md`.
The certificate IS the defect (the described surface's true `‖S_uu‖`
exceeds `muu` by 3e-16 relative, in exact rational arithmetic), not the
test's bare `<=`.

**What stays TESS's on this row:**

1. The assertion messages that made the misreading possible —
   `r1_random_rational_soundness_sweep` prints `{:.3e}` and
   `assert_dominates` `{:.6e}`, neither labels which tuple is which.
   Print `{:.17e}`, labelled, and name the red component.
2. `nurbs_cert.rs`'s header sentence "interval (ring) arithmetic end to
   end", false on the rational arm today; re-worded or kept according
   to the close PROPS lands.
3. This row closes when the sweep is green over a bilinear-stratified
   census on PROPS' fix — the general sweep draws bilinear patches 1/9
   of the time, which is why two hosted hits took 4,385 runs.
