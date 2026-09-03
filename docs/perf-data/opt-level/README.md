# opt-level calibration history

One file per calibration, named `<epoch-seconds>-<short-sha>.json` so a
lexicographic sort is a chronological one. Written and committed by
`.github/workflows/nightly.yml`'s `opt-level calibration` job, on a hosted
runner. Same idiom, same reasons, as `docs/perf-data/rebuild-latency/`.

**Append-only.** A run adds a filename; it never edits an existing one. An
overwritten reference would launder a slow drift; an accumulating one cannot.

**Schemas.** `schema: 1` samples carry `arm_a` (opt-2, free) and `arm_b`
(opt-0, measured), and their `derived.verdict` is a choice between those two.
`schema: 2` adds the optional `arm_c` (opt-1, measured); `verdict` becomes an
argmin over whichever arms the sample carries, and `margin_s`/`margin_ratio`
are measured against `runner_up` rather than always opt-0-against-opt-2 — the
old orientation is kept beside them as `pair_opt0_over_opt2_ratio`.
`schema: 3` replaces the letters with `arms: {"opt-N": {opt_level, source, a,
E, …}}` plus `tree_opt_level`, because the letters were a proxy for the level
and stopped being one when the tree moved off opt-2. `_arms_of()` in
`scripts/opt-level-calibrate.py` reads every earlier schema into schema 3's
shape, and replaying both schema-1 samples through the current code reproduces
every value they already carry — checked in the selftest.

## The question it answers

> **THE TREE IS AT opt-level 1 SINCE 2026-08-25.** The paragraph below is the
> question as it stood when this lane was built, and it is kept because the
> lane's whole thesis is that a verdict expires and you can only tell by
> reading what it used to be. `ci.yml`'s OPT LEVEL note carries the flip, the
> sweep behind it and what would reverse it. **What changed here is which arm
> is free**: it is whichever level the gate runs, so it is opt-1 now and the
> measured arms are opt-0 and opt-2.

`ci.yml`'s `build` job set `CARGO_PROFILE_{DEV,TEST}_OPT_LEVEL = 2`, and the
note arguing for that rests on one quantity: `r`, the opt-0/opt-2 **execution**
ratio, quoted there as 6.46 (default lane) and 7.08 (interval). Those figures
came from a developer's box. A 2026-08-22 census measured the same ratio at
**4.95 / 4.99** on a 4-core AVX-512 guest — about 30% lower, enough to turn the
note's "~2x and ~3x margins" into 0.94x and 0.91x. That is **not** a licence to
flip: a ratio does not transfer between machines, and the census box is not
CI's 2-vCPU runner. It is a demonstration that the number CI relies on had
never been measured where CI runs.

Each sample here is that measurement, taken on the runner:

    the winner is  argmin over the measured levels of  (a + E)

`a` is the archive/build step, `E` the suite's execution. No model, no
extrapolation. `r` is recorded as a **derived** figure — the thing to compare
against the 6.46 above — not as an input to the verdict.

## How the three arms are taken

* **The arm at the tree's own level is free.** It is read from the step
  durations of recent code-tier gate runs through the Actions jobs API — real
  gate data, always current, never re-run. `n` is the number of runs the
  median is over. `tree_opt_level` records which level that was.
* **The other two are measured**, once per calibration: one clean build and
  one full-suite run each, in the gate's own environment with the two
  opt-level knobs set to that level. Each is **optional** — a sample that
  lacks one omits it rather than nulling it, and a sample with fewer than two
  arms is not written at all.

**Which arm is free follows the tree, and that is load-bearing.** Before
2026-08-25 the free arm was opt-2 and the code called it "arm A", with the
level welded into the key names (`arm_a.a2`). Moving the gate to opt-1 without
schema 3 would have left the free read filling `a2`/`E2` with opt-1 durations
while a measured arm took opt-1 *again* — one sample carrying opt-1 twice,
once mislabelled, and a verdict computed off it. Nothing would have gone red.
Arms are keyed by level now, and `nightly.yml`'s `the arms add up` step
refuses to run unless the free level and the measured levels partition
{0, 1, 2}.

**Cadence: weekly, plus drift, plus a moved tree.** The free arm costs nothing
to read, so every nightly asks whether its `E` has moved more than 20% since
the last sample. A **change in the tree's opt level recalibrates
unconditionally**: the previous sample's free arm was a different measurement,
so there is no baseline to drift against. The measured arms run together or
not at all — two arms taken on two different nights are two different trees.

## Why there is a third arm (schema 2, 2026-08-25)

Because the pair was the wrong shape of question. Nothing in this repository
had ever measured, proposed or rejected `opt-level = 1` — #52/#53, #449, the
census, `ci.yml`'s OPT LEVEL note and this history's first two samples all
compare 0 against 2 and stop. But `a + E` is being minimised over a knob with
four settings, the two arms sit at opposite extremes of **both** terms, and in
the 2026-08-25 sample the build penalty opt-2 swallows to buy its execution
win (`a2 - a0` = 499 s) is more than twice the margin it wins by (220 s). An
interior point had room, and nobody had looked.

A three-arm sweep on a 4-core AVX-512 guest (2026-08-25, 3489 tests, all three
arms green) found opt-0 at 143 + 289 = 432 s, **opt-1 at 307 + 60 = 367 s**,
opt-2 at 427 + 58 = 485 s: opt-1 within 3% of opt-2's execution for 58% of its
build penalty, winning outright on a box where opt-2 *loses* to opt-0.

**That box is not this lane's box**, and the sweep is not evidence about CI.
It is the same 4-core class the census used, and
`scripts/check-ci-mirror-parity.py` declares this lane hosted-only precisely
because "a developer box … its own ratio is the measurement this lane exists
to distrust". The sweep is why arm C is wired up. Only arm C's own samples,
taken on the runner, can say what the runner does.

Two derived figures exist to make the opt-1 row readable at a glance:
`execution_kept_vs_opt2` (~1.0 means opt-1 runs as fast as opt-2) and
`build_penalty_kept_vs_opt2` (<1.0 means it pays less to get there).

**The tree was then moved to opt-1 on that evidence** (Ev, 2026-08-25),
before any runner sample existed — deliberately, because the fastest way to
get runner data on opt-1 is to run the gate on opt-1. Every PR now produces a
real opt-1 archive step and a real opt-1 test row, and this lane reads exactly
those durations for free. The measured arms became opt-0 and opt-2 so that
reverting stays a measured decision. Treat the first samples after the flip as
the least trustworthy: the knob change rotates `ci.yml`'s rust-cache key and
the opt-2 arm builds against a brand-new key of its own, so both buy one cold
rebuild.

## Things to know before quoting a sample

* **Reporting only, never gated** (`memories/perf-measurement-lane.md`). No CI
  row fails on a millisecond here. A **flip** — the verdict disagreeing with
  the tree's setting — is printed loudly in the job summary and is the rare
  actionable event; it changes nothing by itself.
* **Read the `environment` block before comparing two samples.** Runner, core
  count, memory, toolchain, `RUSTFLAGS`, every `CARGO_PROFILE_*`,
  debug-assertions and ε are recorded per sample, because a committed timing is
  only worth anything if you know which box produced it.
* **The arms must have measured the same suite.** Each arm's `tests` count is
  recorded for exactly that check. The measured arms are deliberately built
  **without** `--cfg nightly_suite`: with it, `E0`/`E1` would cover the
  demoted tests and `E2` would not.

  **This check did not work before schema 2, and every earlier sample says so
  if you look**: both schema-1 samples carry `"tests": "unknown"`. nextest
  colourises on a hosted runner even into a pipe, wrapping the count in SGR
  escapes (`\e[1m3489\e[0m tests run`), and the extraction that reads
  `\([0-9]*\) tests run` then matched the empty string and fell back to
  `unknown` — silently, because a missing count is not an error. The measured
  arms now pass `--color never`. **Arm A still reports `n/a`** and that is not
  a regression: the jobs API gives step durations, not test counts, so what
  the cross-check can actually compare is arm B against arm C.
* **A verdict is a FLIP only when it disagrees with the tree**, and a flip
  changes nothing by itself. This bullet was written when the tree was at
  opt-2 and a verdict of `opt-1` would have been that disagreement; the tree
  moved to opt-1 on 2026-08-25, so `opt-1` is now the lane AGREEING with
  `ci.yml`'s setting and the flip to watch for is `opt-0` or `opt-2`. Either
  way `ci.yml`'s OPT LEVEL note is the argument of record and moving the knob
  is a separate decision, made against samples from this history rather than
  against the sweep that motivated the arm.
* **This has flipped once already.** opt-2 (#449) was itself a reversal of an
  earlier opt-0 verdict (#52/#53) whose premises expired. That is why each
  sample carries its inputs (`r`, `E2`, `a2 - a0`, the build/total split)
  beside the verdict: the next reader should be able to tell when the
  conclusion expired rather than inherit a bare `opt-level = 2`.
