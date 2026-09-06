---
id: skip-eps-battery-by-observing-oncelock
kind: issue
title: Skip the eps battery's provably-invariant repeats by OBSERVING the tolerance OnceLock, not by declaring
status: parked
opened: 2026-08-13
github: 470
blocked_on: [449]
refs: [452, 467, 469]
---

## From GitHub issue 470

Opened 2026-08-13; 1 comment.

## The finding

The ε battery runs the whole suite at three ambient ε values. **251 tests are statically ε-free** — no `Tolerance::get()` is reachable from them at all — so two thirds of their cost is exact re-execution. Whole crates are in this set: `geom-curves` contains zero `Tolerance::get`/`Band::linear` in `src` (the two grep hits are comments), and so do `mesh::nurbs_cert`, `mesh::chords`, `geom-core::ring_interval_fuzz` and `geom-brep::review_m6_surgery_rider`.

Measured at the 2026-08-13 audit basis that was **541 cpu-s per run**. At today's post-#449 pricing it is roughly 90.

## Why the obvious fixes are wrong

The battery's ratified virtue (Ev, 2026-07-30) is stated in `tolerance.rs:8` — the global is once-initialized, *"which is what lets the test suite run at several ε values (the multi-ε CI matrix) **with zero test-code cooperation**"*. That property is not incidental; it is why the battery caught #146 (a new SSI suite hardcoding ε=1e-9 assumptions) and the M7-4 gap. Anything that lets a test *declare* itself ε-invariant adds an opt-out surface that can lie, and is a change in kind.

A read-counter plus an `eps_untouched(…)` wrapper was considered and rejected on exactly that ground. The self-proving idiom already in the tree (`stl/tests/export.rs:84` re-execs its own payload under three ε values in child processes) is the right shape but saves nothing — it pays three payload runs inside one row, which is what three rows already cost.

## The trick: observe, don't declare

`tolerance.rs:214` is `static GLOBAL: OnceLock<Global>`, populated by `get_or_init` in `fn global()` at `:223-224`. **The observation already exists as state**: `GLOBAL.get().is_some()` is exactly "did this process ever consult ε".

So:

1. put an env-gated one-liner **inside the existing `get_or_init` closure** — it runs at most once per process, so it costs nothing on the read path and nothing at all with the var unset;
2. run the default-ε leg (which runs everything anyway) with that var set, and record which tests emitted it — nextest is process-per-test, so that is per-test granularity for free;
3. run the 1e-6 and 1e-12 legs over only the tests that read it.

Derived, never declared — the same machinery as #452's interval-leg selection, pointed at a different axis. No test declares anything, so nothing can lie, and the ratified zero-cooperation property is preserved exactly.

## Why it is sound, not a heuristic

If a process never called `Tolerance::get()`, no ε value entered the computation, so no branch it took could have depended on ε. Its execution is ε-invariant by construction.

The obvious objection — *"it might read ε at 1e-12 but not at 1e-9"* — self-refutes: taking a different path at a different ε would itself require an ε-dependent branch, which requires a read.

**And it fails in the safe direction.** A test not observed reading ε is treated as sensitive and keeps running everywhere. The mechanism's failure mode is "ran too much", never "skipped something that mattered".

## Caveats for whoever builds it

- **Randomized sweeps break the argument.** Once seeds vary per run, a sweep may touch the tolerance on some seeds and not others, and a single observation does not generalise. Exclude them — they are gated separately anyway.
- The observation pass and the filtered legs must execute the **same binaries** (one archive). That coupling needs saying in the code or it will get broken.
- Timing-based inference is NOT a substitute for the static/observed answer. The audit measured a ~1.5× leg-speed skew between legs of one run, which manufactures apparent ε-sensitivity; and `review_r1_rational_probes::probe_extreme_weight_square` has flat runtime across ε while genuinely *reaching* `Tolerance::get()` — it is flat because it refuses at every ε, not because it is ε-free.

## Why not now

The whole prize is ~90 cpu-s at current pricing. It only becomes interesting if the opt-level-2 revert lands and the ε rows get expensive again — that measurement is pending. Revisit after it.

Two cases are already provably invariant with no new machinery, if a cheap partial is ever wanted: `stl`'s `eps_rows_export_identical_bytes` sets `CAD_TOLERANCE_EPS` itself in child processes so its verdict cannot vary with the outer matrix, and `m5_pr7b_tensor_compose.rs:23` documents its rows as ε-independent by design.

## Context

- #452 — interval run legs execute only the tests that feature adds (the derive-don't-declare precedent, and the tooling to copy)
- #467 — test-suite cost policy
- #469 — CI should report test cost

## Comments

**2026-08-19** — comment:

**Deferred (Ev, 2026-08-19).** Raised while settling S22's ε row; the mechanism is agreed to be the right one — derive by observing the `OnceLock`, never by declaring, so nothing can lie and it fails safe — but the saving does not justify a lane today.

The number is the reason: the 251-invariant-tests figure was measured at the **541 cpu-s** basis of the 2026-08-13 audit, and #449 has since cut that to roughly **90 cpu-s**. This issue's own `Why not now` section already gates it on the opt-level-2 revert landing and the ε rows becoming expensive again, and that remains the right trigger.

Recorded so it is not re-litigated: the ruling is *defer*, not *reject*. The two caveats stay attached for whoever picks it up — randomized sweeps break the soundness argument and must be excluded, and the observation pass and the filtered legs must execute the same binaries from one archive.

Context: this came up alongside the S22 ruling to keep the ambient `OnceLock` (no threading, no session object, no mixed-ε assemblies), which leaves the multi-ε matrix exactly as it is — so nothing about the battery's shape has changed under it.

## Home

S-TCOST: cutting the ε battery's exact re-execution is a test-suite-cost lever over `crates/*/tests/*`, this program's territory; S-QA parked it and is closed. Parked on the opt-level-2 story of #449, the trigger the issue and its deferral ruling both name.
