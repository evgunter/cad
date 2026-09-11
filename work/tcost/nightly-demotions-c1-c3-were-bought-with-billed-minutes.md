---
id: nightly-demotions-c1-c3-were-bought-with-billed-minutes
kind: issue
title: TCOST-C1/C2/C3 moved three checks off the PR gate for billed minutes alone: re-cost them or revert
status: open
opened: 2026-09-11
---


Filed 2026-09-11 out of a re-sort of this program's whole slate against
the fact that `evgunter/cad` went public on **2026-09-03**. Three of
this program's LANDED units bought their saving in billed minutes and in
nothing else, and each paid for it with detection on the pull-request
gate. The currency is gone; the price is still being paid on every run.

## The three, with what each actually bought

| unit | PR | what left the PR gate | measured saving | wall-clock saving |
|---|---|---|---|---|
| **TCOST-C1** | 1650 | `corrupt input (release profile)` — the whole job, including `review_d18`'s two `cfg(not(debug_assertions))` rows, which run in no other lane | **−2 billed minutes** on every code-tier run whose closure holds `topo` (89 of the last 128 merges) | none claimed, and none available: the job is **93 s** cold (`.github/workflows/nightly.yml`, the `release-corruption` header) on a runner where the critical path is the build→test chain |
| **TCOST-C2** | 1654 | the rustdoc gate's six excluded roots and its third pass; pass 1 narrowed from the workspace to the change closure | `fmt` job **222 s / 4 billed → 179 s / 3**, warm against warm | **43 s off a 222 s parallel job**. `fmt` `needs: filter` only and has never been the run's longest pole |
| **TCOST-C3** | 1655 | the `pncad-py` suite, on PRs whose seeds miss {pncad, pncad-py, editor-core} | **−2 billed minutes** per code-tier run whose seeds miss | **never measured** — the unit priced itself in billed minutes only |

## Why this is a cost with nothing on the other side of it

`ci.yml`'s job graph has exactly two edges: `test` needs `build`, and
`test-interval` needs `build-interval`. Every other job — `fmt`,
`clippy`, `k-lint`, `python-suite`, `renders`, and the job C1 removed —
hangs off `filter` and runs in parallel. So a run's wall clock is the
**maximum** over those chains, and the maximum is the build→test chain:
`build + archive (interval)` at **388 s median** plus a test leg at
**34–74 s**, against a whole-run median of **442 s** (created → last job
end; 482 s at tier `all`). Those figures are
`work/ciw/f3-recosting-on-a-public-repo` §M2 and this file's own
re-measure sibling, both taken on the public 4-vCPU runner.

Shortening a 93 s or a 222 s parallel job therefore returns **zero** to
the contributor waiting on the gate. It returned billed minutes, which
is what all three units said it returned, and billed minutes stopped
being a currency on 2026-09-03 — the same day all three of them merged,
and the fact did not reach them.

## What each demotion costs, now that it buys nothing

The demotions are sound under the persistence rule
(`docs/CI-MINUTES-2026-08.md` §*What is NOT sampled*: a run may skip a
detector whose subject PERSISTS in the tree). That rule says a demotion
is PERMITTED, never that it is free. What it costs is **attribution**,
and C1's own header says so: *"a break lands on the night's merges
rather than on the PR that caused it"*. The same program has since
measured what that costs when it happens — 42 red runs on 20 branches
and two agents diagnosing one line in the same hour
(`work/ciw/inherited-red-is-not-attributed-to-its-merge`).

Two of the three carry a second cost beyond attribution:

- **C1** moved the ONLY lane that runs `review_d18`'s two
  `cfg(not(debug_assertions))` rows. Nothing else in the tree compiles
  them.
- **C2** narrowed pass 1 from the workspace to the dependent closure, so
  a prose-only intra-doc break in a member outside the closure is now
  the nightly's to find, not the PR's.

## What this issue asks for

Not a revert on sight — a **re-cost**, which is what
`work/ciw/f3-recosting-on-a-public-repo` §*What is therefore open*
item 2 named on 2026-09-04 and which has never been taken. That unit
re-costed F3 and the configuration draw (both re-opened: the draw was
un-sampled on 2026-09-04, PR 1823); it explicitly left these three for
this program, and this program has not looked at them since.

Per row, and each on its own evidence:

1. **Take a hosted wall reading** for each of the three jobs as they
   would run on the PR gate on the 4-vCPU runner — C3's especially,
   which has never had one, and which is a kernel compile with the
   `python` feature and so is the one of the three that could plausibly
   approach the pole. A job that measurably lengthens the critical path
   stays demoted on a latency argument, stated as one.
2. **Restore the rest.** A job that costs nothing a contributor waits
   for has no argument left for sitting in the nightly, and it is
   holding back attribution on every run.
3. **Say it once in the workflow comments.** All three demotion notes
   in `ci.yml` and `nightly.yml` argue from billed minutes in the
   present tense; whichever way each row lands, the comment is rewritten
   to the currency that decided it. `docs/CI-MINUTES-2026-08.md` is the
   document those comments cite, and its opening premise ("the Actions
   allowance was being consumed faster than the work justified") is
   dead — no figure from it may be quoted forward
   (`work/ciw/plan.md` §The 2026-09-04 re-read).

## Keep-out

The persistence-vs-absence rule is not re-opened; it is what licensed
the demotions and it is orthogonal to price. Neither is F3 — that is
Ev's ruling of 2026-09-07 and is settled. This row is about three jobs
whose ONLY stated benefit was a billed minute.

## Not in this class

Read for the same defect and clean: **TCOST-1** and **TCOST-9** (the
per-file gate) skip suites on the test legs, which are on the critical
path, so gating still buys wall and also rests on a ratified rule
independent of price (`memories/test-suite-cost.md`: a fuzzer that is
not gated is a defect in the fuzzer). **TCOST-B3** (the cache primer) is
latency by mechanism — a cold ~300-unit compile on the longest job of
every first build. **TCOST-K1/K2/K3** and **B1** cut the build and the
slowest rows themselves, and K1/K2 speed the shipped kernel besides.
