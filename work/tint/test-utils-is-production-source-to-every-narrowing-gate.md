---
id: test-utils-is-production-source-to-every-narrowing-gate
kind: issue
title: test-utils documents itself as dev-only but is production source to every gate that narrows by test-only mounts
status: open
opened: 2026-09-15
priority: P1
cost: D
---



Found by TINT-5's style review, generalising the one gate that fired on
the unit's promotion (`scripts/gates/bit-identity-punning.sh`, CI run
`35016148583`).

## The finding

`crates/test-utils/src/lib.rs`'s own header is titled **"DEV-ONLY, by
convention"** and says *"nothing depends on this crate outside
`[dev-dependencies]`, and nothing should"*. The gates do not read that
header, and there is no mechanism by which they could.

- **Twelve of the tree's twenty-two gates take the same file set**, built
  by `gate_require_crate_sources` (`scripts/gates/lib.sh`) as
  `find crates/*/src -type f -name '*.rs'` — `crates/*/tests/**` is not in
  it: `bit-identity-consumer`, `bit-identity-punning`, `bounds-allowlist`,
  `evalscalar-allowlist`, `gated-suite-paths`, `interval-square-allowlist`,
  `loop-boundary-discards`, `no-ambient-env`, `no-extra-real-bounds`,
  `panic-free-macro-bodies`, `register-equal-allowlist`,
  `witness-not-ambient`. Every `crates/test-utils/src/*.rs` file is in
  that set.
- **Three of those narrow further** with `gate_production_sources`
  (`scripts/gates/lib.sh`) — `interval-square-allowlist`,
  `panic-free-macro-bodies`, `witness-not-ambient` — and the narrowing
  is `gate_test_only_mounts`, which drops a file only when its module is
  mounted under a test-only `cfg`. In `test-utils`, exactly one module
  is: `#[cfg(test)] mod panic_capture;`. `census`, `f6`, `fuzz`,
  `roster`, `source`, `tightness` and `vacuity` are plain `pub mod`, so
  **every file of this crate that anything outside it can reach is
  production source to those three gates.**

That is the whole crate's reason for existing, classified as the thing
it is documented not to be. The consequence is not hypothetical: a
helper written inside a test binary and then given a home in this crate
crosses into twelve gates' reach on the day it moves, and has to satisfy
rules written for kernel code — which is what happened to
`assert_f6_every_variant`'s use of `core::any::type_name` on TINT-5.

## What a fix would have to decide

Either the crate declares itself test-only in a way the gates can read
(there is no `#[cfg(test)]` mount available for a crate that exists to
be a dev-dependency of OTHER crates, so this is a change to
`gate_require_crate_sources`' file set or to a per-crate declaration),
or the "DEV-ONLY, by convention" header stops implying an exemption it
does not have and says which rules do apply. The second is cheap and
may be the whole of it: the gates' rules are mostly good rules for a
test helper too, and the only real cost so far has been a promotion
discovering them late.

## Territory

The evidence spans `crates/test-utils/**` (S-TINT's) and
`scripts/gates/**` (not S-TINT's — read-only to this unit, and
untouched). Filed on `tint` because the crate is where the claim that is
wrong is written; a fix that moves the file set belongs with whoever
owns the gates.
