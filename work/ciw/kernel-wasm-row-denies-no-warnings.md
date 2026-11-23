---
id: kernel-wasm-row-denies-no-warnings
kind: issue
title: the nightly kernel/editor-core wasm32 check denies no warnings
status: closed
opened: 2026-09-11
priority: P3
cost: E
closed: 2026-11-22
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

## Closed

2026-11-22, CIW triage (not worth its latency). The flip would catch nothing and adds a red-nightly source. No crate the check covers carries wasm-specific cfg code: `target_arch = "wasm32"` / `target_family = "wasm"` match nothing in `crates/` outside viewer and pncad. The per-PR `cargo clippy --workspace --all-targets --all-features -- -D warnings` (`ci.yml:316`) already denies every warning these crates can raise. The row names no incident.
