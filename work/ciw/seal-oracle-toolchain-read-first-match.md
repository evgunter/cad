---
id: seal-oracle-toolchain-read-first-match
kind: issue
title: seal-oracle.sh reads the toolchain with a first-match sed against Cargo.toml, not the toolchain file
status: open
opened: 2026-09-06
refs: [ruff-pin-read-shares-the-first-match-shape, nightly-pin-reading-idiom-four-copies]
---

`local-scripts/seal-oracle.sh:33`:

    TOOLCHAIN=${TOOLCHAIN:-$(sed -n 's/^rust-version[^"]*"\([^"]*\)".*/\1/p' "$CAD/Cargo.toml" | head -1)}

Found by the whole-tree arm of the pin sweep that closed
`ruff-pin-read-shares-the-first-match-shape`, and not fixed by it: that unit's
subject was `ci.yml`'s `env:` block, and this reads a different file for a
different value.

**Two things are worth a look, and they are separable.**

1. *The shape.* `head -1` again: the first `rust-version` at column 0 in
   `Cargo.toml` wins, and which TOML table it belongs to is not checked. Today
   there is exactly one (`Cargo.toml:50`, `rust-version = "1.97.0"`), so the
   read is correct by coincidence of there being one candidate — which is
   precisely the state the five `nightly.yml` sites were in before they were
   given an anchored reader. A `rust-version` added under any other table
   changes this script's toolchain silently. `scripts/ci-pin.py` does not
   answer this question (it is anchored to a workflow's `env:` block, and
   deliberately narrow about it), so a fix is either a second anchored reader
   or a widening of that one, and which is right is the item.

2. *The source.* `ci.yml` states that the pinned compiler has a single source
   of truth and that it is `rust-toolchain.toml`; `Cargo.toml`'s `rust-version`
   is the workspace's MSRV declaration, which is a different claim that happens
   to carry the same string today. A script that pins its oracle's toolchain
   arguably wants the toolchain file. Whether the two are meant to track each
   other at all is the question to settle first — if they are, nothing
   reconciles them either, and that is the same class as
   `local-half-restates-ci-pins-as-literals` one file over.

**Not urgent.** `seal-oracle.sh` is an investigation script, run by hand; a
wrong toolchain there produces a probe built on the wrong compiler, which is
visible in its own output (`echo "toolchain: +$TOOLCHAIN"`) rather than
silently gating anything.
