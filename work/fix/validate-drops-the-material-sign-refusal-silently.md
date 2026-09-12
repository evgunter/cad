---
id: validate-drops-the-material-sign-refusal-silently
kind: issue
title: tier 3's curved check 6 EXEMPTS a refused material-sign derivation, and the code could not be read to say so
status: closed
opened: 2026-09-11
branch: fix/validate-material-sign-refusal
pr: 2365
closed: 2026-09-11
---


(FIX orchestrator) Found by the
`census-flattens-the-typed-chart-region-declines` lane's sweep
(PR 2354), reported rather than taken — it is a different class from
that unit's.

## The defect

`crates/topo/src/validate.rs:4099`:

```rust
Ok(MaterialSign::Unencoded) | Err(_) => {}
```

`boundary_material_sign`'s refusal is folded into the same arm as a
successful answer of "unencoded", and **nothing is raised**. The
validator examined something, got a typed refusal, and discarded it.

## Why it is not the parent unit's class, and is arguably worse

The parent unit's class is a **flatten**: a typed refusal reaching the
user as a less specific one. Here nothing reaches the user at all. A
flattened refusal is a bad sentence about a real event; a dropped one
leaves no record that the event occurred, and the validator reports a
clean pass over a question it could not answer.

That is a **not-examined** masquerading as an examined-and-fine, which
is the shape A5's letter exists to forbid. Worth checking against A5
directly rather than against this row's summary of it.

## What it needs

Establish first whether `Err(_)` here is reachable at all, and with
what — a refusal that cannot occur wants the arm deleted and a
sentence saying why, not a raise. If it is reachable, the question is
whether the validator's own finding vocabulary can carry it, or
whether this is another cause-carrying widening like PR 2354's.

`crates/topo/src/validate.rs` is **TOPO's** territory glob. Homed on
FIX's slate because it was found here and is one-PR shaped; if TOPO
wants it, the row moves (`work/README.md`'s claim rule) — TOPO's open
PR #2131 does not touch `validate.rs` (checked 2026-09-11).
