# opt-level calibration history

One file per calibration, named `<epoch-seconds>-<short-sha>.json` so a
lexicographic sort is a chronological one. Written and committed by
`.github/workflows/nightly.yml`'s `opt-level calibration` job, on a hosted
runner. Same idiom, same reasons, as `docs/perf-data/rebuild-latency/`.

**Append-only.** A run adds a filename; it never edits an existing one. An
overwritten reference would launder a slow drift; an accumulating one cannot.

## The question it answers

`ci.yml`'s `build` job sets `CARGO_PROFILE_{DEV,TEST}_OPT_LEVEL = 2`, and the
note arguing for that rests on one quantity: `r`, the opt-0/opt-2 **execution**
ratio, quoted there as 6.46 (default lane) and 7.08 (interval). Those figures
came from a developer's box. A 2026-08-22 census measured the same ratio at
**4.95 / 4.99** on a 4-core AVX-512 guest — about 30% lower, enough to turn the
note's "~2x and ~3x margins" into 0.94x and 0.91x. That is **not** a licence to
flip: a ratio does not transfer between machines, and the census box is not
CI's 2-vCPU runner. It is a demonstration that the number CI relies on had
never been measured where CI runs.

Each sample here is that measurement, taken on the runner:

    opt-2 wins  iff  a2 + E2  <  a0 + E0

`a` is the archive/build step, `E` the suite's execution. No model, no
extrapolation. `r` is recorded as a **derived** figure — the thing to compare
against the 6.46 above — not as an input to the verdict.

## How the two arms are taken

* **Arm A (opt-2) is free.** It is read from the step durations of recent
  code-tier gate runs through the Actions jobs API — real gate data, always
  current, never re-run. `n` is the number of runs the median is over.
* **Arm B (opt-0) is measured**, once per calibration: one clean build and one
  full-suite run, in the gate's own environment with the two opt-level knobs
  set to 0.

**Cadence: weekly, plus drift.** Arm A costs nothing to read, so every nightly
asks whether `E2` has moved more than 20% since the last sample; either trigger
re-runs arm B.

## Things to know before quoting a sample

* **Reporting only, never gated** (`memories/perf-measurement-lane.md`). No CI
  row fails on a millisecond here. A **flip** — the verdict disagreeing with
  the tree's setting — is printed loudly in the job summary and is the rare
  actionable event; it changes nothing by itself.
* **Read the `environment` block before comparing two samples.** Runner, core
  count, memory, toolchain, `RUSTFLAGS`, every `CARGO_PROFILE_*`,
  debug-assertions and ε are recorded per sample, because a committed timing is
  only worth anything if you know which box produced it.
* **The two arms must have measured the same suite.** `arm_a.tests` and
  `arm_b.tests` are recorded side by side for exactly that check. Arm B is
  deliberately built **without** `--cfg nightly_suite`: with it, `E0` would
  cover the demoted tests and `E2` would not.
* **This has flipped once already.** opt-2 (#449) was itself a reversal of an
  earlier opt-0 verdict (#52/#53) whose premises expired. That is why each
  sample carries its inputs (`r`, `E2`, `a2 - a0`, the build/total split)
  beside the verdict: the next reader should be able to tell when the
  conclusion expired rather than inherit a bare `opt-level = 2`.
