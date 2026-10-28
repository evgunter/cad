---
id: random-integer-rays-search-trips-at-eps-1e-6-on-one-run
kind: issue
title: review_gui1_r1::random_integer_rays_match_the_exact_oracle failed once at eps = 1e-6 on a commit that passed it before and after
status: closed
opened: 2026-09-09
priority: P3
cost: C
rides_with: pick-oracle-sweep-anti-vacuity-fails-one-seed-in-1300
closed: 2026-10-28
---


Filed by LIB (2026-09-09) from LIB-LOOPS's CI (`lib/loops`, PR #2268,
commit `ca64828d9`): the varying-seed counterexample search at
`crates/editor-core/tests/review_gui1_r1.rs:420` failed in the
`test (eps = 1e-6, 2/2)` shard on the second of two full runs of the
same commit — green on the first run, and green six times locally at
`CAD_TOLERANCE_EPS=1e-6`. The subject is ray picking on an extruded
cube; the PR's diff is four role-name builder signatures and their
call sites, which that test does not reach. Filed here rather than
in LIB's fence because `work/docm/pick-face-fuzz-anti-vacuity-guard-trips-at-effort-1.md`
already carries a sibling flake in the same file, so the seed
discipline of that search looks like one question. LIB did not
re-run or touch the test.

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/tint/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): a probabilistic or once-flaky test guard is S-TINT's (test-suite integrity). Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

## Re-derived (2026-09-15, lane B)

**VERDICT: UNVERIFIABLE-WITHOUT-A-RUN**

The subject still exists and is unchanged, but this row never recorded the
failure MESSAGE, and that is the one fact needed to settle it. No test was
run by this lane.

### What is established by reading

`crates/editor-core/tests/review_gui1_r1.rs`,
`random_integer_rays_match_the_exact_oracle` — the row cites `:420`, and
that is still the `fn` line today, so the file is close to what LIB saw.
The test contains exactly one assertion that a seed can flip on its own:
`assert!(hits_seen > 0, "no draw hit the cube — generator shape broke; …")`.
Every other assertion in it is an oracle differential — the service's face
against an exact rational brute force — which does not depend on the draw
for its TRUTH, only for its coverage.

So there are exactly two possibilities, and they have opposite weight:

1. **It was the `hits_seen > 0` guard.** Then this row is a third instance
   of `work/tint/pick-face-fuzz-anti-vacuity-guard-trips-at-effort-1.md`
   (which already carries two, one of them in the same
   `test (eps = 1e-6, 2/2)` shard), the two rows are one row, and closing
   that one closes this.
2. **It was an oracle disagreement.** Then it is not a test-integrity
   defect at all — it is a live picking bug that shows up on some rays at
   ε = 1e-6, and it belongs on the crate's program, not on S-TINT.

Nothing in the tree distinguishes them.

### Exactly what would settle it

In order of cost:

1. **Read the archived job log.** PR #2268, commit `ca64828d9`, the SECOND
   full run of that commit, job `test (eps = 1e-6, 2/2)`. The panic
   message and file:line in that log decide between (1) and (2) with no
   compute at all. The row does not name the run id, so it has to be found
   from the PR's check history; GitHub's log retention may already have
   expired it, which is the reason this is not simply a reading job.
2. **If the log is gone**: a seed sweep at the failing point —
   `CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p editor-core -E
   'test(random_integer_rays_match_the_exact_oracle)'` under a range of
   `CAD_FUZZ_SEED` values at `CAD_FUZZ_EFFORT=1`. **A red whose message is
   "no draw hit the cube" means outcome (1)**; a red naming a face
   mismatch or a tie-set violation means outcome (2) and is a kernel
   finding. The two known falsifying seeds from the sibling row
   (`0x2870e278e5a1ef24`, `0x1a9e0f26198e881b`) are the cheapest starting
   points — if either reds at ε = 1e-6 with the vacuity message, (1) is
   established as at least sufficient to explain a red in that shard,
   though it does not prove it was THIS red.

Note that neither instrument can retroactively prove what happened on
`ca64828d9`; the strongest achievable verdict is "this row is explained by
the sibling", which is why the archived log is worth trying first.

### What did NOT change

The subject is intact — same `fuzz::scaled(120)` sweep, same integer-ray
generator, same single probabilistic assertion. LIB's framing ("the seed
discipline of that search looks like one question" as the sibling row)
still holds, and merging the two rows on a reading alone would discard the
distinction above, which is the only reason this row is separately useful.

**Recommendation:** do not close and do not merge into the sibling. Try the
archived log; if it is gone, keep the row open as the record that a red was
observed in this shard whose cause is unidentified, and re-examine it if
the sibling's fix does not end the reds.

**A seed that reproduces it** (PATHS 5a fix pass, 2026-09-30): the local
1e-6 run drew `CAD_FUZZ_SEED=0x47471a5e48e72b4c CAD_FUZZ_EFFORT=1`, and
the row fails "no draw hit the cube — generator shape broke" at that
seed at ε 1e-6 AND at the default ε, deterministically. So it is the
anti-vacuity floor bolted onto a varying seed (the shape-1/shape-3 mix
`test_utils::fuzz` warns against), not an ε effect.

## Re-banded P1 → P3, and made a passenger (2026-10-28, GREEN's cut)

Rides with `pick-oracle-sweep-anti-vacuity-fails-one-seed-in-1300`,
whose fix plants a hit so the `hits_seen > 0` guard can no longer fire
on a draw. The lane that lands that fix owes this row its verdict: with
the guard structural, the only seed-flippable failure left is an oracle
disagreement, which would be a live picking bug (P0, on the crate's
program). Three CI reds of this test on record name the guard
(`pick-face-fuzz-anti-vacuity-guard-trips-at-effort-1`, two instances,
one in the same `test (eps = 1e-6, 2/2)` shard) and none names the
oracle, so P3 until a red says otherwise.

## Closed (2026-10-28): it was the guard

**Verdict: the 2026-09-09 red was the anti-vacuity guard, not the
oracle.** The archived log was still retained, and it settles the
question. The run is PR 2268's CI run `34385213731` (the second run of
`ca64828d9`), job `test (eps = 1e-6, 2/2)`, job id `102581338874`:

```
panicked at crates/editor-core/tests/review_gui1_r1.rs:498:5:
    no draw hit the cube — generator shape broke; reproduce with CAD_FUZZ_SEED=0xe8f689ecc59918bc CAD_FUZZ_EFFORT=1
```

So this row is a fifth instance of the guard and closes with
`pick-oracle-sweep-anti-vacuity-fails-one-seed-in-1300`, which planted a
hit in case 0. That seed passes after the fix at `CAD_TOLERANCE_EPS=1e-6`.

**The oracle half was checked too, for today's tree.** The generator's
whole input space is 7³ · 5³ = 42 875 integer `(o, d)` pairs. Eight seeds
at `CAD_FUZZ_EFFORT=1000` and ε = 1e-6 drew 960 000 rays, and every one
agreed with the exact oracle. The chance that any single pair went undrawn
is (1 − 1/42875)^960000 ≈ e^-22, so the whole space was covered. Four
seeds at effort 1000 at ε = 1e-12, and four more at the default ε,
were green as well. **Blind spot:** this covers today's picking code. It
cannot prove what the oracle comparison would have said on `ca64828d9`'s
tree, but the archived message makes that question moot for this red.
