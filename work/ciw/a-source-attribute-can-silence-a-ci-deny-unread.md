---
id: a-source-attribute-can-silence-a-ci-deny-unread
kind: issue
title: a source-level allow can return a denying CI row to what it was, and nothing reads for it
status: open
opened: 2026-09-11
---


Filed from PR 2326, which turned `ci.yml`'s viewer wasm32 row into a
`-D warnings` clippy row. The row is real and it reds — but one line of
Rust returns it to what it was, and no gate in either half reads for
that line.

## Measured

At `crates/viewer/src/lib.rs`, with `CARGO_TARGET_DIR` outside the
worktree and the row's own command
(`RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo clippy -p viewer
--features app --target wasm32-unknown-unknown -- -D warnings`):

| tree | row |
|---|---|
| a `#[cfg(target_family = "wasm")] fn …() {}` with no caller | **exit 101**, `function … is never used` |
| the same, plus `#![allow(dead_code)]` at the crate root | **exit 0**, zero diagnostics |

So the deny is one inner attribute deep, and the attribute needs no
justification, no expiry and no reviewer who knows what it costs.

## Why nothing catches it

`scripts/gates/probe-suite-census.sh`'s `CFG_LINT_SILENCED_RE` is the
only reader in the tree for a lint silenced at the site, and its subject
is `unexpected_cfgs` — it is there so a misspelt feature gate cannot go
quiet. Nothing reads for `dead_code`, for `clippy::*`, or for a
`[lints]` table in a `Cargo.toml`.

This is the blind spot PR 2326's own sweep disclosed and could not
close: that sweep partitions CI **argv**, so every source-level spelling
of a deny — or of an allow — is outside it by construction. An argv
partition cannot see a `#![allow]`, and a gate that reads source cannot
be spelled as one.

## The class, not the instance

The row this was measured on is only the newest member. Every `-D
warnings` row in `ci.yml` and `local-scripts/ci-local.sh` is silenceable
the same way, in the crate it lints; the wasm row is merely the one
whose whole subject is code no human reads on a normal day, which is
where an unexplained `allow` is least likely to be noticed in review.

## Not obviously CIW's to fix

The reader would live in `scripts/gates/`, which is GATES' program, so
widening `CFG_LINT_SILENCED_RE`'s subject — or adding a sibling — is
announced there rather than taken here (this program's `keep_out`).
CIW's half is whatever workflow wiring such a reader needs. Two shapes
worth pricing before either is built:

1. **A census of blanket `allow`s at crate roots**, with a required
   justification comment and an expiry, in the shape the repo already
   uses for exemption tables.
2. **Nothing, deliberately** — on the argument that a blanket `allow`
   in a diff is visible to a reviewer in a way a dropped CI flag is not,
   which is the argument `work/ciw/` accepted for committed conflict
   markers (Ev, 2026-09-04: a self-limiting defect is a poor subject for
   an absence detector). That argument is weaker here, because an
   `allow` is not self-limiting — it silences every future warning in
   its scope, not one.
