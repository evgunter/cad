---
id: ci-local-topo-release-guard-cannot-pass
kind: issue
title: ci-local.sh's topo_release guard greps for a row that cannot exist locally: the local gate row is structurally red
status: open
opened: 2026-09-12
---


Filed 2026-09-12 by the S-TCOST orchestrator, out of the correctness
review of S-TCOST PR 2434 and then verified independently from the
source. `local-scripts/*` is CIW's by `work/ciw/program.md`.

## The defect

`local-scripts/ci-local.sh`'s `topo_release()` runs

```
cargo test --release -p topo --lib -- \
  review_m1_pr2::release_corruption \
  review_m1_pr4::kill_ops_survive_torn_bodies_without_panicking
```

and then guards the result with, among others:

```
grep -q 'garbage_in_garbage_out_release \.\.\. ok' "$log" ||
  { echo "ERROR: $t did not run"; rc=1; }
```

**That row cannot exist in a local `--release` build.**
`foreign_parent_loop_garbage_in_garbage_out_release`
(`crates/topo/src/review_m1_pr2/release_corruption.rs`) is
`#[cfg(not(debug_assertions))]`, and the root `Cargo.toml` carries

```
[profile.release]
debug-assertions = true
```

so a local `cargo test --release` compiles the **debug** arms. The
function is not in the binary, cannot appear in the log, and the guard
reds. The hosted job gets the row only because `ci.yml` sets
`CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS: "false"` on its own step —
which `ci-local.sh` does not set.

Checked for the obvious escape: there is exactly one function with that
name in the file, and it is the `cfg`-gated one. There is no debug-arm
counterpart whose name the grep could match instead.

## Why this has survived

Three reasons worth recording, because each is a separate lesson:

1. **`--selftest` cannot see it.** The guard is a `grep` over a log that
   only exists after a real `cargo test --release`, so no selftest arm
   drives it.
2. **The row is behind `RUN_TOPO_RELEASE`**, so a local gate run over a
   diff that does not reach the `topo` closure never calls it.
3. **`local-scripts/` is deleted by every hosted job** (*"every workflow
   job does `rm -rf local-scripts`"*), so no hosted run can red on it
   either. The file is outside every gate in the project by
   construction.

So this is a guard that cannot pass, in a script no gate runs, guarding
a row that cannot exist — which is why it reads as working.

## What makes it worse than a broken local script

The comments in three source files asserted, until PR 2434 corrected
them, that `ci-local.sh` *"runs the same rows on every local gate"*. It
does not: it omits `review_d18` from the selection entirely, and the two
`cfg(not(debug_assertions))` rows are absent from its build. A developer
reading those headers would have believed the local gate covered rows it
has never once compiled.

## What this asks for

Decide what the local row is FOR, then make it that:

- **If it should mirror the hosted job**, set
  `CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=false` in `topo_release()` and
  add `review_d18` to the selection. The guard then passes and the
  mirror claim becomes true.
- **If it should not** — a local gate may reasonably decline to
  re-compile `topo` with a different profile — then delete the guard
  lines for rows that cannot exist locally and say in the function what
  the local row does and does not cover.

Either is fine; the current state is the one that is not, because it
reports a failure that says `did not run` when the truth is `cannot
exist`.

## The class

**A guard whose subject is excluded by the very configuration the guard
runs under.** `ci-local.sh` carries roughly fifty-five `HOSTED MIRROR:`
markers pairing a local function with a hosted job, matched by
`scripts/check-ci-mirror-parity.py` **on NAME only** — nothing compares
the bodies, so any pair may have drifted the same way. This one is the
instance that was found; the census has not been run.

A related finding routed separately: the hosted half's
`CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS` sits outside
`check-ci-mirror-parity.py`'s `SEMANTIC_ENV` allowlist
(`RUSTFLAGS`, `RUSTDOCFLAGS`, plus a `CAD_` prefix), so the parity gate
is blind to the most semantics-bearing variable in the pair — it decides
whether two test rows compile at all.
