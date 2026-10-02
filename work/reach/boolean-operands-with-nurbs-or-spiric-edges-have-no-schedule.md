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

## The crossing layer's half, behind the gate (2026-10-02, REACH)

Lifting the gate reaches a second wall: `reduce::curved_face_arm`
sends every carrier other than a line, a circle or an ellipse to
`CurvedPierceUnsupported` before any clearance test, so a spiric or
NURBS edge would refuse against a cylinder, sphere or torus face
whenever its box meets the face's, however far it runs from it. The
ellipse lane (`work/reach/non-circle-conic-edge-refuses-against-every-curved-face.md`)
generalized the circle's clearance to a conic; what a general carrier
needs is in the same shape:

The conic rung's sampled enclosure
(`geom_brep::conic_arc_residual_range`: the residual's sample hull
widened by the chord-dip charge `f2·h²/8`) carries over to any carrier
with a certified bound `f2` on `|F″|` along the span. With
`|C′| ≤ s₁` and `|C″| ≤ s₂` over the span:

- against a sphere or a cylinder the residual is a quadric
  `(|⊥(p − o)|² − r²)/2r`, so `|F″| ≤ (s₁² + |⊥(C − o)|max·s₂)/r`;
- against a torus, `torus_curvature_bound` is already stated in terms
  of `|w′|, |w″|` and the axial harmonic; the conic reads it at
  `s₁ = s₂ = a` (the semi-major axis), and a general carrier needs
  `|h′| ≤ s₁`, `|h″| ≤ s₂` in place of the first-harmonic amplitude.

What is missing is `s₁`, `s₂` per carrier: for a spiric, from its
`|dP/dv|` bound (`Curve3::Spiric`'s docs give `r(R − r)/√((R − r)² −
offset²)`) and a second-derivative bound; for a NURBS, from the
hodograph's control hull (rational weights need the quotient rule's
bound). Neither has a root lane either (a spiric's residual against a
sphere is not a trigonometric polynomial in `v`), so a definite
crossing would still refuse — but a clear pair would clear.
