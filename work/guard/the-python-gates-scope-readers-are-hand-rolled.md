---
id: the-python-gates-scope-readers-are-hand-rolled
kind: issue
title: The Python gates' three scope readers are hand-rolled; source-scope cannot reach them yet
status: open
opened: 2026-11-09
---

Three Python gates resolve Rust names with scope readers they each wrote
by hand. These are the same kind of reader that produced the
flagged-door census's treadmill (PRs 6603, 6627 and 6640, each closing a
blind spot the one before it left):

- `scripts/gates/bare-rounding-allowlist.sh`: `use_items` (:212),
  `blocks` (:331), `scope_of` (:342), `bindings` (:348). These read use
  trees, block scopes and type-alias bindings.
- `scripts/gates/engine_leaves.py`: `tokenize` (:113),
  `refused_bindings` (:440), and `resolve` inside it (:537). This is a
  tokenizer plus inline-module `use super::*` resolution.
- `scripts/payload-rung-sweep.py`: `use_items` (:768) and a
  crate/module path resolver, `resolve` (:893).

The Rust guards now answer scope questions through `crates/source-scope`.
It is built on `syn`, and the `reader_census` scope ledger refuses a
hand-rolled Rust scope reader. The Python gates cannot call it.

**The measurement, from the onecopy census move (option C, deferred).**
- **CI ordering.** In `.github/workflows/ci.yml`'s lint job, the gates
  step (`scripts/run-gates.sh`, :293) runs before
  `dtolnay/rust-toolchain` (:296). A compiled tool that feeds these
  gates would need that job reordered, plus a tool build on every gate
  run.
- **Cost of the parse.** The same `syn` parse over every shipped `.rs`
  file costs about 18 s per pass at opt 0. With `syn` and `proc-macro2`
  at opt-level 2 (the root `Cargo.toml` profile stanzas) it costs about
  5–6 s. The lexical reader it replaced took about 2 s.

**Fix (option C).** Build a `tools/` binary on `source-scope` that emits
each file's scope facts as JSON: use leaves, fn signatures, and each
call's resolved binding. Move the three gates onto it. Reorder the lint
job so the toolchain comes before the gates, and cache the tool's build.
Until then, each gate's blind spots are listed in its header.
