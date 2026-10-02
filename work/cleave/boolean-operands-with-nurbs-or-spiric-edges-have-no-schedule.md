---
id: boolean-operands-with-nurbs-or-spiric-edges-have-no-schedule
kind: issue
title: No row schedules boolean operands whose edges are NURBS or spiric; gate_operand_edges refuses them and three demo joins wait on it
status: open
opened: 2026-10-02
priority: P1
cost: H
design: true
---

Filed by the SHOW orchestrator (2026-10-02) from the demo audit that
opened SHOW.

## Finding

`crates/topo/src/boolean/reduce.rs`'s `gate_operand_edges` refuses
any operand edge whose carrier is `Nurbs` or `Spiric`
(`BooleanError::CurvedEdgeUnsupported`). Those are the carriers the
curved zip MINTS (rung-3 edges), so any body that came out of a loft,
a sweep, or a curved boolean cannot go back in as an operand. A grep
of `work/` for the variant finds it only cited as a symptom
(`pipe/S5`, `reach/split-section-boundary-curved-arm-untested-past-the-edge-gate`,
the `pin` teapot paragraph row); no row schedules lifting it.

## Where it shows

Three live tour probes pin it, each panicking when it retires:

- `teapot.rs` wall 3 — the lofted spout ∪ the pot
  (`CurvedEdgeUnsupported { operand: B, .. }`, the canal's seams);
- `lily.rs` wall 8 — a leaf's sheath grafted onto its blade at a
  declared rectangle;
- every "set beside rather than welded" lofted or swept piece in
  those scenes.

## Why it is a design row

Consuming a NURBS edge means intersecting a NURBS curve against the
other operand's faces with a certificate the crossing layer does not
have yet; whether that is a new root lane per face kind, a re-entry
through the germ-chord lanes (DESIGN frontier (d)), or something else
is the question. Weigh it before a lane builds it.

## Home

CLEAVE: `reduce.rs` is in its paths (shared with HONE), and the gate
is a boolean operand gate. Re-home if another program's charter fits
better.

## The split's twin is reach-scoped (REACH, 2026-10-02)

`splitting/classify.rs` `gate_operand` no longer refuses a `Spiric` or
`Nurbs` operand edge wholesale: it refuses one only when the plane may
meet it, and the edge clears behind its own reach box or the box of
either face it bounds (a spline edge has no sound box of its own,
`EdgeBoxRule::NoSoundBox`, but it lies on both its faces). A plane over
`sweep::test_support::loft_prism` now returns the loft whole. The
boolean's `gate_operand_edges` stays body-scoped by its own stated
design; whether it should follow the split, pair-scoped the way the
face gate is, is part of this row's design question.
