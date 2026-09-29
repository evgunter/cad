---
id: kernel-wasm-row-denies-no-warnings
kind: issue
title: the nightly kernel/editor-core wasm32 check denies no warnings
status: open
opened: 2026-09-11
priority: P3
cost: E
---

Re-homed from BLIND (closed 2026-09-28) and rewritten against the
latency-cut CI.

## The row

`nightly.yml`'s `full-suite` job, step `wasm32 check (kernel +
editor-core)`: `cargo check --workspace --exclude pncad --exclude
pncad-py --exclude viewer --target wasm32-unknown-unknown`. It denies
nothing, and no comment records a decision not to deny. The other wasm32
row, `ci.yml`'s `viewer` job (`wasm32 clippy`), is a `-D warnings`
clippy.

Measured 2026-09-11 (then with `--features interval`, since deleted):
`RUSTFLAGS='-Dwarnings'` on the same command exited 0 with zero
diagnostics, so the flip was free then; re-measure before flipping.

## To settle

- **Which spelling.** The viewer row uses `cargo clippy … -- -D
  warnings`; at `--workspace` scope a second-target clippy costs more
  than one `-p viewer` graph, which matters less on the nightly.
- **The exclusions.** `pncad`'s own wasm arm is denied only when the
  per-PR `viewer` job runs (it builds `pncad` as a dependency), so a
  lint in it on a kernel-only change is read by nothing. The deny has
  to reach past `--exclude pncad` or the exclusion has to be re-argued.
