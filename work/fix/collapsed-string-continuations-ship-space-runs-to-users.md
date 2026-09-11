---
id: collapsed-string-continuations-ship-space-runs-to-users
kind: issue
title: eleven user-facing messages ship multi-space runs from collapsed line continuations, and a one-line grep finds them
status: open
opened: 2026-09-11
---


(FIX orchestrator) Found by the style review of PR 2354, which was
looking at two instances that PR authored and swept for the class.

## The defect

A multi-line Rust string literal wrapped for source width needs a `\`
at the end of each line, or the indentation of the next line becomes
**part of the string**. Drop the backslash and the message a user reads
carries a run of spaces mid-sentence.

Eleven pre-existing instances, each a user-facing message:

- `crates/topo/src/pcurves.rs:2383`
- `crates/topo/src/props.rs:1799`
- `crates/topo/src/chart_region.rs:858`
- `crates/topo/src/boolean/reduce.rs:2126`
- `crates/topo/src/review_m1_pr5_internal.rs:273`
- `crates/viewer/src/pickindex.rs:1791`
- `crates/viewer/src/session.rs:1471`
- `crates/viewer/src/gpu.rs:1620`
- `crates/editor-core/src/refactor.rs:304`
- `crates/geom-core/src/sym.rs:3104`

(Ten sites; the eleventh and twelfth were PR 2354's own two, fixed in
that PR's fix pass.)

## Why this is worth a row and not just a fix

**The pattern is mechanical and nothing runs it.**

```
"[^"]*[a-z,.;)]    +[a-z(]
```

finds them in seconds — a run of four or more spaces inside a string
literal, between two pieces of ordinary sentence. That is a gate row's
shape, not a sweep's: this class regenerates every time someone wraps a
long message, it is invisible in source (the wrap looks right) and
visible only in output, and no reviewer reads every message a crate
renders.

The tree already has a prose gate that runs on every build —
`crates/pncad-py/src/errors.rs`'s `reads_as_prose`, asserted at
`py::typed_err` — but it tests for the **field-brace fingerprint**, a
different property, and it only sees messages that reach the Python
door. This one is about every rendered message and is decidable from
source text alone.

**Whether the guard belongs here or in `scripts/gates/`** is the real
question, and it is CIW's as much as ours: a gate over string literals
in `crates/*/src` is the kind of row `gate_rust_code` exists for,
except that this needle lives *inside* string literals and
`gate_rust_code` builds the code-**only** view — the same obstacle PR
1809 hit and routed around by writing the census in Rust. Say which
before writing it.

## Fences

The ten sites span `topo` (unowned files plus CURVED's `chart_region.rs`),
`viewer` (CHROME's and VIEW's), `editor-core` (FIX's `refactor.rs` and
others'), and `geom-core` (PROPS's). A sweep, so name every fence in
the PR. None of the sites is a behaviour change: the repair is a
backslash and nothing else moves — which also makes the before/after
trivially checkable by rendering each message.
