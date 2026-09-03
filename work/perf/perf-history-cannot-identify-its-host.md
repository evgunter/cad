---
id: perf-history-cannot-identify-its-host
kind: issue
title: perf histories cannot identify the box that produced a sample - the environment block records nproc/mem/toolchain and nothing that distinguishes two ubuntu-latest hosts
status: open
opened: 2026-09-03
---


Found by scanning the nightly lane's 11 runs (2026-08-23 … 2026-09-02, all
green). Nothing is broken; the defect is in what the histories can be read to
say afterwards.

## The evidence

`docs/perf-data/criterion/1788257654-9432ac5.json` (nightly run 10,
2026-09-01) is faster than every neighbour on **all six rows at once**, by a
factor that barely varies between them. Per-row median against the 2026-08-31
sample:

| sample | mean ratio | cv | min | max |
|---|---|---|---|---|
| 1787938290-0f7266d (08-28) | 1.016 | 0.043 | 0.956 | 1.086 |
| 1788003863-e49796f (08-29) | 1.016 | 0.051 | 0.971 | 1.117 |
| 1788086018-edb81dd (08-30) | 1.001 | 0.026 | 0.980 | 1.053 |
| 1788176534-e77c041 (08-31) | 1.000 | — | — | — |
| **1788257654-9432ac5 (09-01)** | **0.784** | **0.026** | **0.762** | **0.812** |
| 1788342256-6e5ce3d (09-02) | 1.022 | 0.042 | 0.961 | 1.078 |

A 21.6% whole-suite move that reverts the next night, spread across CDT
insertion, Euler-op surgery, the boolean commit path, flux quadrature and the
tier-2/3 validation ladder alike. No merge does that to five unrelated cost
centers by the same factor and then undoes it. It is a box, not the tree.

**And the sample cannot say so.** The `environment` block of that entry is
identical to its neighbours' in every field that could carry the difference:

    nproc=2  mem_total_kb=8128872  runner="Linux/X64 ubuntu-latest"
    rustup_toolchain=1.97.0-x86_64-unknown-linux-gnu  rustflags=""
    cargo_profile_overrides=[]  debug_assertions=false

`scripts/criterion-emit.py:126` `environment()` is the whole block, and
`nproc`/`mem_total_kb`/`platform.machine()` are constant across GitHub's
`ubuntu-latest` pool while the host CPU generation is not.

## Why this matters more than one odd row

Every reader-facing instruction in these lanes routes through that block, and
this sample defeats each of them:

* `docs/perf-data/criterion/README.md` — "**Read the `environment` block
  first** … a committed timing is worth nothing if you cannot say which box
  produced it". Here it is read, and it says nothing.
* the same README — "Treat a move under ~10% as noise unless consecutive
  entries agree." 21.6% is over the band, so the rule classifies a host swap
  as signal.
* `docs/perf-data/opt-level/README.md` §"Things to know before quoting a
  sample" and `docs/perf-data/rebuild-latency/README.md` carry the same
  instruction over the same field set.
* `memories/perf-measurement-lane.md`: "A committed timing is only worth
  anything if you know which box produced it."

And it is the exact failure the rebuild-latency history was built to end:
"three developer-workstation refreshes disagreed by 90–98% on every row with
contention ruled out, leaving a build/environment hypothesis nobody captured
side by side" (`docs/perf-data/rebuild-latency/README.md`). Going hosted
narrowed the spread; it did not make the box identifiable. A future 20% move
in these histories is not attributable, in either direction — which is what an
append-only history exists to prevent.

## Where it also bites: the opt-level verdict

`opt-level calibration` compares one **free** arm — the median of five gate
runs, so five hosts, noise partly averaged — against two **measured** arms
taken on one host on one night. A host swap of the size above moves both
measured arms together and only ever toward the status quo, because the free
arm is by construction the level the tree already runs
(`.github/workflows/nightly.yml`, `CI_TREE_OPT_LEVEL`).

The verdict has agreed with the tree's own level in **7 of 7 samples** — the
free arm has never lost. The latest margin over the runner-up is
`margin_ratio` 1.23 (205 s), i.e. within the size of the unrecorded between-
host excursion documented above. That is not evidence the verdict is wrong;
opt-1's win has been much larger on other nights (690 s on 2026-09-01). It is
that the history cannot presently distinguish "opt-1 wins" from "the nightly
drew a slow box", and the README's advice to read the environment block is
what would have distinguished them.

## The fix, and its size

Record the host's identity in `environment()`, the same shape in all three
lanes so the blocks stay comparable:

* CPU model — `/proc/cpuinfo` `model name` (first occurrence), plus the flag
  subset that actually moves these rows (`avx2`, `avx512f`); the 2026-08-22
  census already found the opt-0/opt-2 ratio 30% apart between an AVX-512
  guest and CI, so the flags are the discriminator that matters here.
* a cheap fixed-work calibration tick (a fixed loop timed on the box) if a
  model string turns out not to separate the pool.

Emitters: `scripts/criterion-emit.py:126`, and the equivalent block in the
rebuild-latency and opt-level emitters — `scripts/opt-level-calibrate.py`'s
sample carries the same fields.

Append-only means the samples already written stay unattributable; the fix
only makes the ones after it readable. That is an argument for doing it soon
rather than for not doing it.

## What is NOT wrong

* All 11 nightly runs are green; `watertight` has never failed.
* Shard handling in the free arm is correct — `sample_run` sums the shards
  (`scripts/opt-level-calibrate.py:227`) and the selftest covers a cancelled
  one.
* The measured arms are not disadvantaged by cold builds: each has its own
  persistent `Swatinem/rust-cache` key and workspace mapping, matching the
  gate's warm archive step.
