# criterion benchmark history

One file per measurement, named `<epoch-seconds>-<short-sha>.json` so a
lexicographic sort is a chronological one. Written and committed by
`.github/workflows/nightly.yml`'s `criterion benchmarks (reporting)` job,
on a hosted runner. Same idiom, same reasons, as
`docs/perf-data/rebuild-latency/` and `docs/perf-data/opt-level/`.

**Append-only.** A run adds a filename; it never edits an existing one. An
overwritten reference would launder a slow drift — 5% per merge over 20
merges is 165% with no single flag — and an accumulating one cannot.

## What it measures

`benches/benches/kernel.rs`, which is PERF-PLAN §5's item 1: the six rows
that document names as its five scenarios. Each is a cost center §1.3
ranks, sited where the plan says the cost is.

| row | the cost center it watches |
|---|---|
| `tessellate/washer/1e-4` | CDT insertion (finding 7b); the cheap end |
| `tessellate/washer/1e-6` | the same, where the quadratic bites — the row a `spade` bulk-load adoption (§2.1) has to move |
| `kernel/validate/tier23_washer` | the commit lane's validation ladder (findings 4, 5, 16) |
| `kernel/mass_props/washer` | per-face flux quadrature; §2.2's idiom-2 parallelism target |
| `kernel/build/extrude` | Euler-op surgery through the sweep door (finding 9) |
| `kernel/boolean/two_bricks` | the boolean commit path (findings 4, 13, 14, 15) |

The two tessellation rows are one scenario measured twice on purpose: the
finding is about the QUADRATIC, so the 1e-4 → 1e-6 ratio is the shape, and
neither number alone is.

## Before quoting a sample

* **Read the `environment` block first.** Runner, core count, memory,
  toolchain, RUSTFLAGS, every `CARGO_PROFILE_*` and the debug-assertions
  posture are recorded per sample, because a committed timing is worth
  nothing if you cannot say which box produced it
  (`memories/perf-measurement-lane.md`).
* **`median_ci_ns` is a WITHIN-run interval and it understates what a
  comparison across two entries can resolve.** Three consecutive runs on a
  quiet 4-core box (2026-08-27) spread ~3–9% against within-run intervals
  of ±2–3%; a hosted 2-vCPU runner has a fatter tail than that. Treat a
  move under ~10% as noise unless consecutive entries agree.
* **Debug assertions are OFF here, and the kernel's own `[profile.release]`
  turns them ON.** `benches/Cargo.toml` carries the argument and the
  measurement behind it: on this tree, turning them on costs **6.5×** on
  `kernel/build/extrude` and **5.2×** on `kernel/boolean/two_bricks` and
  nothing on the other four. That is PERF-PLAN §1.3's per-op debug
  full-body tier-1 (D1's ratified postcondition clause) measured for the
  first time. So these numbers are the kernel's own cost, and they are
  **not** the cost of the profile real parts meet.
* **Reporting only, never gated** (`memories/perf-measurement-lane.md`,
  PERF-PLAN Q-P4). No CI row fails on a millisecond. The one thing that
  does fail is `scripts/criterion-emit.py`'s roster pin: a renamed or
  dropped benchmark would silently start a new column and end an old one,
  which reads in the trend as a cost that went away.

## Cadence, and what it costs

Nightly, and only on a night where `main` actually moved — the workflow's
`gate` job — a few billed minutes when it runs, nothing when it does not.
No figure is quoted here on purpose: it is a build plus a run on a shared
runner, both of which move, and `docs/CI-MINUTES-2026-08.md` is where a
reading of CI's cost belongs.

PERF-PLAN's ratified Q-P4 says post-merge, never a PR gate, and named
pushes to `main`. The nightly is strictly cheaper than that and satisfies
the same requirement — the trend merely has to predate the first change it
would police. What it gives up is per-commit attribution: a regression
lands somewhere in a day's merges rather than on one commit. The workflow's
`workflow_dispatch` **ref** input is the handle that closes that gap; a
dispatch at a SHA runs the rows and **writes nothing**, so a bisection
cannot corrupt this history with measurements of an old tree.

## Running it yourself

    cd benches && cargo bench                        # the six rows
    cd benches && cargo run --release --example counts   # the δ sweep

The first is the lane's own measurement; the second walks chordal
tolerance over four decades and prints the exponent in triangle count,
which is what tells a steep constant from a bad asymptote. Both are the
right local act and neither writes anything here — deliberately. Your
milliseconds are not comparable with a runner's, which is the design and
not a limitation. `scripts/criterion-emit.py` is declared hosted-only in
`scripts/check-ci-mirror-parity.py`'s exemption table for exactly that
reason.
